::bobbin_mcu::periph!( USART1, Usart1, USART1_PERIPH, UsartPeriph, USART1_OWNED, USART1_REF_COUNT, 0x40013800, 0x00, 0x15);

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB2RSTR"), field: Some("USART1RST"), description: None }
impl ::bobbin_mcu::gate::GateRst for Usart1 {
    #[inline]
    fn gate_rst(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb2rstr().usart1rst() }
    #[inline]
    fn set_gate_rst<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2rstr(|r| r.set_usart1rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB2ENR"), field: Some("USART1EN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Usart1 {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb2enr().usart1en() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2enr(|r| r.set_usart1en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB2SMENR"), field: Some("USART1SMEN"), description: None }
impl ::bobbin_mcu::gate::GateSleepEn for Usart1 {
    #[inline]
    fn gate_sleep_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb2smenr().usart1smen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2smenr(|r| r.set_usart1smen(value));
        self
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="USART Peripheral"]
pub struct UsartPeriph(pub usize); 

impl UsartPeriph {
    #[doc="Get the CR1 Register."]
    #[inline] pub fn cr1_reg(&self) -> ::bobbin_mcu::register::Register<Cr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cr1, 0x0)
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
    #[inline] pub fn cr2_reg(&self) -> ::bobbin_mcu::register::Register<Cr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cr2, 0x4)
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
    #[inline] pub fn cr3_reg(&self) -> ::bobbin_mcu::register::Register<Cr3> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cr3, 0x8)
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
    #[inline] pub fn brr_reg(&self) -> ::bobbin_mcu::register::Register<Brr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Brr, 0xc)
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
    #[inline] pub fn gtpr_reg(&self) -> ::bobbin_mcu::register::Register<Gtpr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Gtpr, 0x10)
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
    #[inline] pub fn rtor_reg(&self) -> ::bobbin_mcu::register::Register<Rtor> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rtor, 0x14)
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
    #[inline] pub fn rqr_reg(&self) -> ::bobbin_mcu::register::Register<Rqr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rqr, 0x18)
    }

    #[doc="Get the *mut pointer for the RQR register."]
    #[inline] pub fn rqr_mut(&self) -> *mut Rqr { 
        self.rqr_reg().ptr()
    }

    #[doc="Get the *const pointer for the RQR register."]
    #[inline] pub fn rqr_ptr(&self) -> *const Rqr { 
        self.rqr_reg().ptr()
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

    #[doc="Get the ISR Register."]
    #[inline] pub fn isr_reg(&self) -> ::bobbin_mcu::register::Register<Isr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Isr, 0x1c)
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
    #[inline] pub fn icr_reg(&self) -> ::bobbin_mcu::register::Register<Icr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Icr, 0x20)
    }

    #[doc="Get the *mut pointer for the ICR register."]
    #[inline] pub fn icr_mut(&self) -> *mut Icr { 
        self.icr_reg().ptr()
    }

    #[doc="Get the *const pointer for the ICR register."]
    #[inline] pub fn icr_ptr(&self) -> *const Icr { 
        self.icr_reg().ptr()
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

    #[doc="Get the RDR Register."]
    #[inline] pub fn rdr_reg(&self) -> ::bobbin_mcu::register::Register<Rdr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rdr, 0x24)
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
    #[inline] pub fn tdr_reg(&self) -> ::bobbin_mcu::register::Register<Tdr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Tdr, 0x28)
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

    #[doc="Get the PRESC Register."]
    #[inline] pub fn presc_reg(&self) -> ::bobbin_mcu::register::Register<Presc> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Presc, 0x2c)
    }

    #[doc="Get the *mut pointer for the PRESC register."]
    #[inline] pub fn presc_mut(&self) -> *mut Presc { 
        self.presc_reg().ptr()
    }

    #[doc="Get the *const pointer for the PRESC register."]
    #[inline] pub fn presc_ptr(&self) -> *const Presc { 
        self.presc_reg().ptr()
    }

    #[doc="Read the PRESC register."]
    #[inline] pub fn presc(&self) -> Presc { 
        self.presc_reg().read()
    }

    #[doc="Write the PRESC register."]
    #[inline] pub fn write_presc(&self, value: Presc) -> &Self { 
        self.presc_reg().write(value);
        self
    }

    #[doc="Set the PRESC register."]
    #[inline] pub fn set_presc<F: FnOnce(Presc) -> Presc>(&self, f: F) -> &Self {
        self.presc_reg().set(f);
        self
    }

    #[doc="Modify the PRESC register."]
    #[inline] pub fn with_presc<F: FnOnce(Presc) -> Presc>(&self, f: F) -> &Self {
        self.presc_reg().with(f);
        self
    }

}

#[doc="Control register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr1(pub u32);
impl Cr1 {
    #[doc="RXFIFO Full interrupt enable"]
    #[inline] pub fn rxffie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if RXFFIE != 0"]
    #[inline] pub fn test_rxffie(&self) -> bool {
        self.rxffie() != 0
    }

    #[doc="Sets the RXFFIE field."]
    #[inline] pub fn set_rxffie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="TXFIFO empty interrupt enable"]
    #[inline] pub fn txfeie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if TXFEIE != 0"]
    #[inline] pub fn test_txfeie(&self) -> bool {
        self.txfeie() != 0
    }

    #[doc="Sets the TXFEIE field."]
    #[inline] pub fn set_txfeie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="FIFO mode enable"]
    #[inline] pub fn fifoen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if FIFOEN != 0"]
    #[inline] pub fn test_fifoen(&self) -> bool {
        self.fifoen() != 0
    }

    #[doc="Sets the FIFOEN field."]
    #[inline] pub fn set_fifoen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Word length"]
    #[inline] pub fn m1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if M1 != 0"]
    #[inline] pub fn test_m1(&self) -> bool {
        self.m1() != 0
    }

    #[doc="Sets the M1 field."]
    #[inline] pub fn set_m1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="End of Block interrupt enable"]
    #[inline] pub fn eobie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if EOBIE != 0"]
    #[inline] pub fn test_eobie(&self) -> bool {
        self.eobie() != 0
    }

    #[doc="Sets the EOBIE field."]
    #[inline] pub fn set_eobie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Receiver timeout interrupt enable"]
    #[inline] pub fn rtoie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if RTOIE != 0"]
    #[inline] pub fn test_rtoie(&self) -> bool {
        self.rtoie() != 0
    }

    #[doc="Sets the RTOIE field."]
    #[inline] pub fn set_rtoie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Driver Enable assertion time"]
    #[inline] pub fn deat4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if DEAT4 != 0"]
    #[inline] pub fn test_deat4(&self) -> bool {
        self.deat4() != 0
    }

    #[doc="Sets the DEAT4 field."]
    #[inline] pub fn set_deat4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="DEAT3"]
    #[inline] pub fn deat3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if DEAT3 != 0"]
    #[inline] pub fn test_deat3(&self) -> bool {
        self.deat3() != 0
    }

    #[doc="Sets the DEAT3 field."]
    #[inline] pub fn set_deat3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="DEAT2"]
    #[inline] pub fn deat2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if DEAT2 != 0"]
    #[inline] pub fn test_deat2(&self) -> bool {
        self.deat2() != 0
    }

    #[doc="Sets the DEAT2 field."]
    #[inline] pub fn set_deat2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="DEAT1"]
    #[inline] pub fn deat1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if DEAT1 != 0"]
    #[inline] pub fn test_deat1(&self) -> bool {
        self.deat1() != 0
    }

    #[doc="Sets the DEAT1 field."]
    #[inline] pub fn set_deat1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="DEAT0"]
    #[inline] pub fn deat0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if DEAT0 != 0"]
    #[inline] pub fn test_deat0(&self) -> bool {
        self.deat0() != 0
    }

    #[doc="Sets the DEAT0 field."]
    #[inline] pub fn set_deat0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Driver Enable de-assertion time"]
    #[inline] pub fn dedt4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if DEDT4 != 0"]
    #[inline] pub fn test_dedt4(&self) -> bool {
        self.dedt4() != 0
    }

    #[doc="Sets the DEDT4 field."]
    #[inline] pub fn set_dedt4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="DEDT3"]
    #[inline] pub fn dedt3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if DEDT3 != 0"]
    #[inline] pub fn test_dedt3(&self) -> bool {
        self.dedt3() != 0
    }

    #[doc="Sets the DEDT3 field."]
    #[inline] pub fn set_dedt3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="DEDT2"]
    #[inline] pub fn dedt2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if DEDT2 != 0"]
    #[inline] pub fn test_dedt2(&self) -> bool {
        self.dedt2() != 0
    }

    #[doc="Sets the DEDT2 field."]
    #[inline] pub fn set_dedt2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="DEDT1"]
    #[inline] pub fn dedt1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if DEDT1 != 0"]
    #[inline] pub fn test_dedt1(&self) -> bool {
        self.dedt1() != 0
    }

    #[doc="Sets the DEDT1 field."]
    #[inline] pub fn set_dedt1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="DEDT0"]
    #[inline] pub fn dedt0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if DEDT0 != 0"]
    #[inline] pub fn test_dedt0(&self) -> bool {
        self.dedt0() != 0
    }

    #[doc="Sets the DEDT0 field."]
    #[inline] pub fn set_dedt0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Oversampling mode"]
    #[inline] pub fn over8(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if OVER8 != 0"]
    #[inline] pub fn test_over8(&self) -> bool {
        self.over8() != 0
    }

    #[doc="Sets the OVER8 field."]
    #[inline] pub fn set_over8<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Character match interrupt enable"]
    #[inline] pub fn cmie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if CMIE != 0"]
    #[inline] pub fn test_cmie(&self) -> bool {
        self.cmie() != 0
    }

    #[doc="Sets the CMIE field."]
    #[inline] pub fn set_cmie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Mute mode enable"]
    #[inline] pub fn mme(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if MME != 0"]
    #[inline] pub fn test_mme(&self) -> bool {
        self.mme() != 0
    }

    #[doc="Sets the MME field."]
    #[inline] pub fn set_mme<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Word length"]
    #[inline] pub fn m0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if M0 != 0"]
    #[inline] pub fn test_m0(&self) -> bool {
        self.m0() != 0
    }

    #[doc="Sets the M0 field."]
    #[inline] pub fn set_m0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Receiver wakeup method"]
    #[inline] pub fn wake(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if WAKE != 0"]
    #[inline] pub fn test_wake(&self) -> bool {
        self.wake() != 0
    }

    #[doc="Sets the WAKE field."]
    #[inline] pub fn set_wake<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Parity control enable"]
    #[inline] pub fn pce(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PCE != 0"]
    #[inline] pub fn test_pce(&self) -> bool {
        self.pce() != 0
    }

    #[doc="Sets the PCE field."]
    #[inline] pub fn set_pce<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Parity selection"]
    #[inline] pub fn ps(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PS != 0"]
    #[inline] pub fn test_ps(&self) -> bool {
        self.ps() != 0
    }

    #[doc="Sets the PS field."]
    #[inline] pub fn set_ps<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="PE interrupt enable"]
    #[inline] pub fn peie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PEIE != 0"]
    #[inline] pub fn test_peie(&self) -> bool {
        self.peie() != 0
    }

    #[doc="Sets the PEIE field."]
    #[inline] pub fn set_peie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="interrupt enable"]
    #[inline] pub fn txeie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXEIE != 0"]
    #[inline] pub fn test_txeie(&self) -> bool {
        self.txeie() != 0
    }

    #[doc="Sets the TXEIE field."]
    #[inline] pub fn set_txeie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Transmission complete interrupt enable"]
    #[inline] pub fn tcie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TCIE != 0"]
    #[inline] pub fn test_tcie(&self) -> bool {
        self.tcie() != 0
    }

    #[doc="Sets the TCIE field."]
    #[inline] pub fn set_tcie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="RXNE interrupt enable"]
    #[inline] pub fn rxneie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RXNEIE != 0"]
    #[inline] pub fn test_rxneie(&self) -> bool {
        self.rxneie() != 0
    }

    #[doc="Sets the RXNEIE field."]
    #[inline] pub fn set_rxneie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="IDLE interrupt enable"]
    #[inline] pub fn idleie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if IDLEIE != 0"]
    #[inline] pub fn test_idleie(&self) -> bool {
        self.idleie() != 0
    }

    #[doc="Sets the IDLEIE field."]
    #[inline] pub fn set_idleie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Transmitter enable"]
    #[inline] pub fn te(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TE != 0"]
    #[inline] pub fn test_te(&self) -> bool {
        self.te() != 0
    }

    #[doc="Sets the TE field."]
    #[inline] pub fn set_te<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Receiver enable"]
    #[inline] pub fn re(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RE != 0"]
    #[inline] pub fn test_re(&self) -> bool {
        self.re() != 0
    }

    #[doc="Sets the RE field."]
    #[inline] pub fn set_re<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="USART enable in Stop mode"]
    #[inline] pub fn uesm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if UESM != 0"]
    #[inline] pub fn test_uesm(&self) -> bool {
        self.uesm() != 0
    }

    #[doc="Sets the UESM field."]
    #[inline] pub fn set_uesm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="USART enable"]
    #[inline] pub fn ue(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if UE != 0"]
    #[inline] pub fn test_ue(&self) -> bool {
        self.ue() != 0
    }

    #[doc="Sets the UE field."]
    #[inline] pub fn set_ue<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
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
        if self.rxffie() != 0 { try!(write!(f, " rxffie"))}
        if self.txfeie() != 0 { try!(write!(f, " txfeie"))}
        if self.fifoen() != 0 { try!(write!(f, " fifoen"))}
        if self.m1() != 0 { try!(write!(f, " m1"))}
        if self.eobie() != 0 { try!(write!(f, " eobie"))}
        if self.rtoie() != 0 { try!(write!(f, " rtoie"))}
        if self.deat4() != 0 { try!(write!(f, " deat4"))}
        if self.deat3() != 0 { try!(write!(f, " deat3"))}
        if self.deat2() != 0 { try!(write!(f, " deat2"))}
        if self.deat1() != 0 { try!(write!(f, " deat1"))}
        if self.deat0() != 0 { try!(write!(f, " deat0"))}
        if self.dedt4() != 0 { try!(write!(f, " dedt4"))}
        if self.dedt3() != 0 { try!(write!(f, " dedt3"))}
        if self.dedt2() != 0 { try!(write!(f, " dedt2"))}
        if self.dedt1() != 0 { try!(write!(f, " dedt1"))}
        if self.dedt0() != 0 { try!(write!(f, " dedt0"))}
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
    #[inline] pub fn add4_7(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0xf) as u8) } // [31:28]
    }

    #[doc="Returns true if ADD4_7 != 0"]
    #[inline] pub fn test_add4_7(&self) -> bool {
        self.add4_7() != 0
    }

    #[doc="Sets the ADD4_7 field."]
    #[inline] pub fn set_add4_7<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Address of the USART node"]
    #[inline] pub fn add0_3(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if ADD0_3 != 0"]
    #[inline] pub fn test_add0_3(&self) -> bool {
        self.add0_3() != 0
    }

    #[doc="Sets the ADD0_3 field."]
    #[inline] pub fn set_add0_3<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Receiver timeout enable"]
    #[inline] pub fn rtoen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if RTOEN != 0"]
    #[inline] pub fn test_rtoen(&self) -> bool {
        self.rtoen() != 0
    }

    #[doc="Sets the RTOEN field."]
    #[inline] pub fn set_rtoen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Auto baud rate mode"]
    #[inline] pub fn abrmod1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if ABRMOD1 != 0"]
    #[inline] pub fn test_abrmod1(&self) -> bool {
        self.abrmod1() != 0
    }

    #[doc="Sets the ABRMOD1 field."]
    #[inline] pub fn set_abrmod1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="ABRMOD0"]
    #[inline] pub fn abrmod0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if ABRMOD0 != 0"]
    #[inline] pub fn test_abrmod0(&self) -> bool {
        self.abrmod0() != 0
    }

    #[doc="Sets the ABRMOD0 field."]
    #[inline] pub fn set_abrmod0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Auto baud rate enable"]
    #[inline] pub fn abren(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if ABREN != 0"]
    #[inline] pub fn test_abren(&self) -> bool {
        self.abren() != 0
    }

    #[doc="Sets the ABREN field."]
    #[inline] pub fn set_abren<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Most significant bit first"]
    #[inline] pub fn msbfirst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if MSBFIRST != 0"]
    #[inline] pub fn test_msbfirst(&self) -> bool {
        self.msbfirst() != 0
    }

    #[doc="Sets the MSBFIRST field."]
    #[inline] pub fn set_msbfirst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Binary data inversion"]
    #[inline] pub fn tainv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if TAINV != 0"]
    #[inline] pub fn test_tainv(&self) -> bool {
        self.tainv() != 0
    }

    #[doc="Sets the TAINV field."]
    #[inline] pub fn set_tainv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="TX pin active level inversion"]
    #[inline] pub fn txinv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if TXINV != 0"]
    #[inline] pub fn test_txinv(&self) -> bool {
        self.txinv() != 0
    }

    #[doc="Sets the TXINV field."]
    #[inline] pub fn set_txinv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="RX pin active level inversion"]
    #[inline] pub fn rxinv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if RXINV != 0"]
    #[inline] pub fn test_rxinv(&self) -> bool {
        self.rxinv() != 0
    }

    #[doc="Sets the RXINV field."]
    #[inline] pub fn set_rxinv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Swap TX/RX pins"]
    #[inline] pub fn swap(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if SWAP != 0"]
    #[inline] pub fn test_swap(&self) -> bool {
        self.swap() != 0
    }

    #[doc="Sets the SWAP field."]
    #[inline] pub fn set_swap<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="LIN mode enable"]
    #[inline] pub fn linen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if LINEN != 0"]
    #[inline] pub fn test_linen(&self) -> bool {
        self.linen() != 0
    }

    #[doc="Sets the LINEN field."]
    #[inline] pub fn set_linen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="STOP bits"]
    #[inline] pub fn stop(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if STOP != 0"]
    #[inline] pub fn test_stop(&self) -> bool {
        self.stop() != 0
    }

    #[doc="Sets the STOP field."]
    #[inline] pub fn set_stop<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Clock enable"]
    #[inline] pub fn clken(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CLKEN != 0"]
    #[inline] pub fn test_clken(&self) -> bool {
        self.clken() != 0
    }

    #[doc="Sets the CLKEN field."]
    #[inline] pub fn set_clken<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Clock polarity"]
    #[inline] pub fn cpol(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CPOL != 0"]
    #[inline] pub fn test_cpol(&self) -> bool {
        self.cpol() != 0
    }

    #[doc="Sets the CPOL field."]
    #[inline] pub fn set_cpol<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Clock phase"]
    #[inline] pub fn cpha(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CPHA != 0"]
    #[inline] pub fn test_cpha(&self) -> bool {
        self.cpha() != 0
    }

    #[doc="Sets the CPHA field."]
    #[inline] pub fn set_cpha<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Last bit clock pulse"]
    #[inline] pub fn lbcl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if LBCL != 0"]
    #[inline] pub fn test_lbcl(&self) -> bool {
        self.lbcl() != 0
    }

    #[doc="Sets the LBCL field."]
    #[inline] pub fn set_lbcl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="LIN break detection interrupt enable"]
    #[inline] pub fn lbdie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if LBDIE != 0"]
    #[inline] pub fn test_lbdie(&self) -> bool {
        self.lbdie() != 0
    }

    #[doc="Sets the LBDIE field."]
    #[inline] pub fn set_lbdie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="LIN break detection length"]
    #[inline] pub fn lbdl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if LBDL != 0"]
    #[inline] pub fn test_lbdl(&self) -> bool {
        self.lbdl() != 0
    }

    #[doc="Sets the LBDL field."]
    #[inline] pub fn set_lbdl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="7-bit Address Detection/4-bit Address Detection"]
    #[inline] pub fn addm7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ADDM7 != 0"]
    #[inline] pub fn test_addm7(&self) -> bool {
        self.addm7() != 0
    }

    #[doc="Sets the ADDM7 field."]
    #[inline] pub fn set_addm7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="When the DSI_NSS bit is set, the NSS pin input will be ignored"]
    #[inline] pub fn dis_nss(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DIS_NSS != 0"]
    #[inline] pub fn test_dis_nss(&self) -> bool {
        self.dis_nss() != 0
    }

    #[doc="Sets the DIS_NSS field."]
    #[inline] pub fn set_dis_nss<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Synchronous Slave mode enable"]
    #[inline] pub fn slven(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SLVEN != 0"]
    #[inline] pub fn test_slven(&self) -> bool {
        self.slven() != 0
    }

    #[doc="Sets the SLVEN field."]
    #[inline] pub fn set_slven<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
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
        if self.add4_7() != 0 { try!(write!(f, " add4_7=0x{:x}", self.add4_7()))}
        if self.add0_3() != 0 { try!(write!(f, " add0_3=0x{:x}", self.add0_3()))}
        if self.rtoen() != 0 { try!(write!(f, " rtoen"))}
        if self.abrmod1() != 0 { try!(write!(f, " abrmod1"))}
        if self.abrmod0() != 0 { try!(write!(f, " abrmod0"))}
        if self.abren() != 0 { try!(write!(f, " abren"))}
        if self.msbfirst() != 0 { try!(write!(f, " msbfirst"))}
        if self.tainv() != 0 { try!(write!(f, " tainv"))}
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
        if self.dis_nss() != 0 { try!(write!(f, " dis_nss"))}
        if self.slven() != 0 { try!(write!(f, " slven"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr3(pub u32);
impl Cr3 {
    #[doc="TXFIFO threshold configuration"]
    #[inline] pub fn txftcfg(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x7) as u8) } // [31:29]
    }

    #[doc="Returns true if TXFTCFG != 0"]
    #[inline] pub fn test_txftcfg(&self) -> bool {
        self.txftcfg() != 0
    }

    #[doc="Sets the TXFTCFG field."]
    #[inline] pub fn set_txftcfg<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="RXFIFO threshold interrupt enable"]
    #[inline] pub fn rxftie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if RXFTIE != 0"]
    #[inline] pub fn test_rxftie(&self) -> bool {
        self.rxftie() != 0
    }

    #[doc="Sets the RXFTIE field."]
    #[inline] pub fn set_rxftie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Receive FIFO threshold configuration"]
    #[inline] pub fn rxftcfg(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x7) as u8) } // [27:25]
    }

    #[doc="Returns true if RXFTCFG != 0"]
    #[inline] pub fn test_rxftcfg(&self) -> bool {
        self.rxftcfg() != 0
    }

    #[doc="Sets the RXFTCFG field."]
    #[inline] pub fn set_rxftcfg<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Tr Complete before guard time, interrupt enable"]
    #[inline] pub fn tcbgtie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if TCBGTIE != 0"]
    #[inline] pub fn test_tcbgtie(&self) -> bool {
        self.tcbgtie() != 0
    }

    #[doc="Sets the TCBGTIE field."]
    #[inline] pub fn set_tcbgtie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="threshold interrupt enable"]
    #[inline] pub fn txftie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if TXFTIE != 0"]
    #[inline] pub fn test_txftie(&self) -> bool {
        self.txftie() != 0
    }

    #[doc="Sets the TXFTIE field."]
    #[inline] pub fn set_txftie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Wakeup from Stop mode interrupt enable"]
    #[inline] pub fn wufie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if WUFIE != 0"]
    #[inline] pub fn test_wufie(&self) -> bool {
        self.wufie() != 0
    }

    #[doc="Sets the WUFIE field."]
    #[inline] pub fn set_wufie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Wakeup from Stop mode interrupt flag selection"]
    #[inline] pub fn wus(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
    }

    #[doc="Returns true if WUS != 0"]
    #[inline] pub fn test_wus(&self) -> bool {
        self.wus() != 0
    }

    #[doc="Sets the WUS field."]
    #[inline] pub fn set_wus<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Smartcard auto-retry count"]
    #[inline] pub fn scarcnt(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x7) as u8) } // [19:17]
    }

    #[doc="Returns true if SCARCNT != 0"]
    #[inline] pub fn test_scarcnt(&self) -> bool {
        self.scarcnt() != 0
    }

    #[doc="Sets the SCARCNT field."]
    #[inline] pub fn set_scarcnt<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Driver enable polarity selection"]
    #[inline] pub fn dep(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if DEP != 0"]
    #[inline] pub fn test_dep(&self) -> bool {
        self.dep() != 0
    }

    #[doc="Sets the DEP field."]
    #[inline] pub fn set_dep<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Driver enable mode"]
    #[inline] pub fn dem(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if DEM != 0"]
    #[inline] pub fn test_dem(&self) -> bool {
        self.dem() != 0
    }

    #[doc="Sets the DEM field."]
    #[inline] pub fn set_dem<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="DMA Disable on Reception Error"]
    #[inline] pub fn ddre(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if DDRE != 0"]
    #[inline] pub fn test_ddre(&self) -> bool {
        self.ddre() != 0
    }

    #[doc="Sets the DDRE field."]
    #[inline] pub fn set_ddre<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Overrun Disable"]
    #[inline] pub fn ovrdis(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if OVRDIS != 0"]
    #[inline] pub fn test_ovrdis(&self) -> bool {
        self.ovrdis() != 0
    }

    #[doc="Sets the OVRDIS field."]
    #[inline] pub fn set_ovrdis<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="One sample bit method enable"]
    #[inline] pub fn onebit(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if ONEBIT != 0"]
    #[inline] pub fn test_onebit(&self) -> bool {
        self.onebit() != 0
    }

    #[doc="Sets the ONEBIT field."]
    #[inline] pub fn set_onebit<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="CTS interrupt enable"]
    #[inline] pub fn ctsie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CTSIE != 0"]
    #[inline] pub fn test_ctsie(&self) -> bool {
        self.ctsie() != 0
    }

    #[doc="Sets the CTSIE field."]
    #[inline] pub fn set_ctsie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="CTS enable"]
    #[inline] pub fn ctse(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CTSE != 0"]
    #[inline] pub fn test_ctse(&self) -> bool {
        self.ctse() != 0
    }

    #[doc="Sets the CTSE field."]
    #[inline] pub fn set_ctse<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="RTS enable"]
    #[inline] pub fn rtse(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if RTSE != 0"]
    #[inline] pub fn test_rtse(&self) -> bool {
        self.rtse() != 0
    }

    #[doc="Sets the RTSE field."]
    #[inline] pub fn set_rtse<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DMA enable transmitter"]
    #[inline] pub fn dmat(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if DMAT != 0"]
    #[inline] pub fn test_dmat(&self) -> bool {
        self.dmat() != 0
    }

    #[doc="Sets the DMAT field."]
    #[inline] pub fn set_dmat<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="DMA enable receiver"]
    #[inline] pub fn dmar(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DMAR != 0"]
    #[inline] pub fn test_dmar(&self) -> bool {
        self.dmar() != 0
    }

    #[doc="Sets the DMAR field."]
    #[inline] pub fn set_dmar<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Smartcard mode enable"]
    #[inline] pub fn scen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if SCEN != 0"]
    #[inline] pub fn test_scen(&self) -> bool {
        self.scen() != 0
    }

    #[doc="Sets the SCEN field."]
    #[inline] pub fn set_scen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Smartcard NACK enable"]
    #[inline] pub fn nack(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NACK != 0"]
    #[inline] pub fn test_nack(&self) -> bool {
        self.nack() != 0
    }

    #[doc="Sets the NACK field."]
    #[inline] pub fn set_nack<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Half-duplex selection"]
    #[inline] pub fn hdsel(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if HDSEL != 0"]
    #[inline] pub fn test_hdsel(&self) -> bool {
        self.hdsel() != 0
    }

    #[doc="Sets the HDSEL field."]
    #[inline] pub fn set_hdsel<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Ir low-power"]
    #[inline] pub fn irlp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if IRLP != 0"]
    #[inline] pub fn test_irlp(&self) -> bool {
        self.irlp() != 0
    }

    #[doc="Sets the IRLP field."]
    #[inline] pub fn set_irlp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Ir mode enable"]
    #[inline] pub fn iren(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if IREN != 0"]
    #[inline] pub fn test_iren(&self) -> bool {
        self.iren() != 0
    }

    #[doc="Sets the IREN field."]
    #[inline] pub fn set_iren<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Error interrupt enable"]
    #[inline] pub fn eie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EIE != 0"]
    #[inline] pub fn test_eie(&self) -> bool {
        self.eie() != 0
    }

    #[doc="Sets the EIE field."]
    #[inline] pub fn set_eie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
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
        if self.txftcfg() != 0 { try!(write!(f, " txftcfg=0x{:x}", self.txftcfg()))}
        if self.rxftie() != 0 { try!(write!(f, " rxftie"))}
        if self.rxftcfg() != 0 { try!(write!(f, " rxftcfg=0x{:x}", self.rxftcfg()))}
        if self.tcbgtie() != 0 { try!(write!(f, " tcbgtie"))}
        if self.txftie() != 0 { try!(write!(f, " txftie"))}
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
    #[inline] pub fn div_mantissa(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xfff) as u16) } // [15:4]
    }

    #[doc="Returns true if DIV_Mantissa != 0"]
    #[inline] pub fn test_div_mantissa(&self) -> bool {
        self.div_mantissa() != 0
    }

    #[doc="Sets the DIV_Mantissa field."]
    #[inline] pub fn set_div_mantissa<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="fraction of USARTDIV"]
    #[inline] pub fn div_fraction(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if DIV_Fraction != 0"]
    #[inline] pub fn test_div_fraction(&self) -> bool {
        self.div_fraction() != 0
    }

    #[doc="Sets the DIV_Fraction field."]
    #[inline] pub fn set_div_fraction<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
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
    #[inline] pub fn gt(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if GT != 0"]
    #[inline] pub fn test_gt(&self) -> bool {
        self.gt() != 0
    }

    #[doc="Sets the GT field."]
    #[inline] pub fn set_gt<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Prescaler value"]
    #[inline] pub fn psc(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PSC != 0"]
    #[inline] pub fn test_psc(&self) -> bool {
        self.psc() != 0
    }

    #[doc="Sets the PSC field."]
    #[inline] pub fn set_psc<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
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
    #[inline] pub fn blen(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if BLEN != 0"]
    #[inline] pub fn test_blen(&self) -> bool {
        self.blen() != 0
    }

    #[doc="Sets the BLEN field."]
    #[inline] pub fn set_blen<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Receiver timeout value"]
    #[inline] pub fn rto(&self) -> ::bobbin_bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffff) as u32) } // [23:0]
    }

    #[doc="Returns true if RTO != 0"]
    #[inline] pub fn test_rto(&self) -> bool {
        self.rto() != 0
    }

    #[doc="Sets the RTO field."]
    #[inline] pub fn set_rto<V: Into<::bobbin_bits::U24>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U24 = value.into();
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
    #[inline] pub fn txfrq(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TXFRQ != 0"]
    #[inline] pub fn test_txfrq(&self) -> bool {
        self.txfrq() != 0
    }

    #[doc="Sets the TXFRQ field."]
    #[inline] pub fn set_txfrq<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Receive data flush request"]
    #[inline] pub fn rxfrq(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RXFRQ != 0"]
    #[inline] pub fn test_rxfrq(&self) -> bool {
        self.rxfrq() != 0
    }

    #[doc="Sets the RXFRQ field."]
    #[inline] pub fn set_rxfrq<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Mute mode request"]
    #[inline] pub fn mmrq(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if MMRQ != 0"]
    #[inline] pub fn test_mmrq(&self) -> bool {
        self.mmrq() != 0
    }

    #[doc="Sets the MMRQ field."]
    #[inline] pub fn set_mmrq<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Send break request"]
    #[inline] pub fn sbkrq(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SBKRQ != 0"]
    #[inline] pub fn test_sbkrq(&self) -> bool {
        self.sbkrq() != 0
    }

    #[doc="Sets the SBKRQ field."]
    #[inline] pub fn set_sbkrq<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Auto baud rate request"]
    #[inline] pub fn abrrq(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ABRRQ != 0"]
    #[inline] pub fn test_abrrq(&self) -> bool {
        self.abrrq() != 0
    }

    #[doc="Sets the ABRRQ field."]
    #[inline] pub fn set_abrrq<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
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
    #[doc="TXFIFO threshold flag"]
    #[inline] pub fn txft(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if TXFT != 0"]
    #[inline] pub fn test_txft(&self) -> bool {
        self.txft() != 0
    }

    #[doc="Sets the TXFT field."]
    #[inline] pub fn set_txft<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="RXFIFO threshold flag"]
    #[inline] pub fn rxft(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if RXFT != 0"]
    #[inline] pub fn test_rxft(&self) -> bool {
        self.rxft() != 0
    }

    #[doc="Sets the RXFT field."]
    #[inline] pub fn set_rxft<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Transmission complete before guard time flag"]
    #[inline] pub fn tcbgt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if TCBGT != 0"]
    #[inline] pub fn test_tcbgt(&self) -> bool {
        self.tcbgt() != 0
    }

    #[doc="Sets the TCBGT field."]
    #[inline] pub fn set_tcbgt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="RXFIFO Full"]
    #[inline] pub fn rxff(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if RXFF != 0"]
    #[inline] pub fn test_rxff(&self) -> bool {
        self.rxff() != 0
    }

    #[doc="Sets the RXFF field."]
    #[inline] pub fn set_rxff<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="TXFIFO Empty"]
    #[inline] pub fn txfe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if TXFE != 0"]
    #[inline] pub fn test_txfe(&self) -> bool {
        self.txfe() != 0
    }

    #[doc="Sets the TXFE field."]
    #[inline] pub fn set_txfe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="REACK"]
    #[inline] pub fn reack(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if REACK != 0"]
    #[inline] pub fn test_reack(&self) -> bool {
        self.reack() != 0
    }

    #[doc="Sets the REACK field."]
    #[inline] pub fn set_reack<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="TEACK"]
    #[inline] pub fn teack(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if TEACK != 0"]
    #[inline] pub fn test_teack(&self) -> bool {
        self.teack() != 0
    }

    #[doc="Sets the TEACK field."]
    #[inline] pub fn set_teack<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="WUF"]
    #[inline] pub fn wuf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if WUF != 0"]
    #[inline] pub fn test_wuf(&self) -> bool {
        self.wuf() != 0
    }

    #[doc="Sets the WUF field."]
    #[inline] pub fn set_wuf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="RWU"]
    #[inline] pub fn rwu(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if RWU != 0"]
    #[inline] pub fn test_rwu(&self) -> bool {
        self.rwu() != 0
    }

    #[doc="Sets the RWU field."]
    #[inline] pub fn set_rwu<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="SBKF"]
    #[inline] pub fn sbkf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if SBKF != 0"]
    #[inline] pub fn test_sbkf(&self) -> bool {
        self.sbkf() != 0
    }

    #[doc="Sets the SBKF field."]
    #[inline] pub fn set_sbkf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="CMF"]
    #[inline] pub fn cmf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if CMF != 0"]
    #[inline] pub fn test_cmf(&self) -> bool {
        self.cmf() != 0
    }

    #[doc="Sets the CMF field."]
    #[inline] pub fn set_cmf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="BUSY"]
    #[inline] pub fn busy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if BUSY != 0"]
    #[inline] pub fn test_busy(&self) -> bool {
        self.busy() != 0
    }

    #[doc="Sets the BUSY field."]
    #[inline] pub fn set_busy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="ABRF"]
    #[inline] pub fn abrf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if ABRF != 0"]
    #[inline] pub fn test_abrf(&self) -> bool {
        self.abrf() != 0
    }

    #[doc="Sets the ABRF field."]
    #[inline] pub fn set_abrf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="ABRE"]
    #[inline] pub fn abre(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if ABRE != 0"]
    #[inline] pub fn test_abre(&self) -> bool {
        self.abre() != 0
    }

    #[doc="Sets the ABRE field."]
    #[inline] pub fn set_abre<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="SPI slave underrun error flag"]
    #[inline] pub fn udr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if UDR != 0"]
    #[inline] pub fn test_udr(&self) -> bool {
        self.udr() != 0
    }

    #[doc="Sets the UDR field."]
    #[inline] pub fn set_udr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="EOBF"]
    #[inline] pub fn eobf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if EOBF != 0"]
    #[inline] pub fn test_eobf(&self) -> bool {
        self.eobf() != 0
    }

    #[doc="Sets the EOBF field."]
    #[inline] pub fn set_eobf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="RTOF"]
    #[inline] pub fn rtof(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if RTOF != 0"]
    #[inline] pub fn test_rtof(&self) -> bool {
        self.rtof() != 0
    }

    #[doc="Sets the RTOF field."]
    #[inline] pub fn set_rtof<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="CTS"]
    #[inline] pub fn cts(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CTS != 0"]
    #[inline] pub fn test_cts(&self) -> bool {
        self.cts() != 0
    }

    #[doc="Sets the CTS field."]
    #[inline] pub fn set_cts<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="CTSIF"]
    #[inline] pub fn ctsif(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CTSIF != 0"]
    #[inline] pub fn test_ctsif(&self) -> bool {
        self.ctsif() != 0
    }

    #[doc="Sets the CTSIF field."]
    #[inline] pub fn set_ctsif<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="LBDF"]
    #[inline] pub fn lbdf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if LBDF != 0"]
    #[inline] pub fn test_lbdf(&self) -> bool {
        self.lbdf() != 0
    }

    #[doc="Sets the LBDF field."]
    #[inline] pub fn set_lbdf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="TXE"]
    #[inline] pub fn txe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXE != 0"]
    #[inline] pub fn test_txe(&self) -> bool {
        self.txe() != 0
    }

    #[doc="Sets the TXE field."]
    #[inline] pub fn set_txe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="TC"]
    #[inline] pub fn tc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TC != 0"]
    #[inline] pub fn test_tc(&self) -> bool {
        self.tc() != 0
    }

    #[doc="Sets the TC field."]
    #[inline] pub fn set_tc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="RXNE"]
    #[inline] pub fn rxne(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RXNE != 0"]
    #[inline] pub fn test_rxne(&self) -> bool {
        self.rxne() != 0
    }

    #[doc="Sets the RXNE field."]
    #[inline] pub fn set_rxne<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="IDLE"]
    #[inline] pub fn idle(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if IDLE != 0"]
    #[inline] pub fn test_idle(&self) -> bool {
        self.idle() != 0
    }

    #[doc="Sets the IDLE field."]
    #[inline] pub fn set_idle<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ORE"]
    #[inline] pub fn ore(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ORE != 0"]
    #[inline] pub fn test_ore(&self) -> bool {
        self.ore() != 0
    }

    #[doc="Sets the ORE field."]
    #[inline] pub fn set_ore<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="NF"]
    #[inline] pub fn nf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if NF != 0"]
    #[inline] pub fn test_nf(&self) -> bool {
        self.nf() != 0
    }

    #[doc="Sets the NF field."]
    #[inline] pub fn set_nf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="FE"]
    #[inline] pub fn fe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if FE != 0"]
    #[inline] pub fn test_fe(&self) -> bool {
        self.fe() != 0
    }

    #[doc="Sets the FE field."]
    #[inline] pub fn set_fe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="PE"]
    #[inline] pub fn pe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PE != 0"]
    #[inline] pub fn test_pe(&self) -> bool {
        self.pe() != 0
    }

    #[doc="Sets the PE field."]
    #[inline] pub fn set_pe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
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
        if self.txft() != 0 { try!(write!(f, " txft"))}
        if self.rxft() != 0 { try!(write!(f, " rxft"))}
        if self.tcbgt() != 0 { try!(write!(f, " tcbgt"))}
        if self.rxff() != 0 { try!(write!(f, " rxff"))}
        if self.txfe() != 0 { try!(write!(f, " txfe"))}
        if self.reack() != 0 { try!(write!(f, " reack"))}
        if self.teack() != 0 { try!(write!(f, " teack"))}
        if self.wuf() != 0 { try!(write!(f, " wuf"))}
        if self.rwu() != 0 { try!(write!(f, " rwu"))}
        if self.sbkf() != 0 { try!(write!(f, " sbkf"))}
        if self.cmf() != 0 { try!(write!(f, " cmf"))}
        if self.busy() != 0 { try!(write!(f, " busy"))}
        if self.abrf() != 0 { try!(write!(f, " abrf"))}
        if self.abre() != 0 { try!(write!(f, " abre"))}
        if self.udr() != 0 { try!(write!(f, " udr"))}
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
    #[inline] pub fn wucf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if WUCF != 0"]
    #[inline] pub fn test_wucf(&self) -> bool {
        self.wucf() != 0
    }

    #[doc="Sets the WUCF field."]
    #[inline] pub fn set_wucf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Character match clear flag"]
    #[inline] pub fn cmcf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if CMCF != 0"]
    #[inline] pub fn test_cmcf(&self) -> bool {
        self.cmcf() != 0
    }

    #[doc="Sets the CMCF field."]
    #[inline] pub fn set_cmcf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="SPI slave underrun clear flag"]
    #[inline] pub fn udrcf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if UDRCF != 0"]
    #[inline] pub fn test_udrcf(&self) -> bool {
        self.udrcf() != 0
    }

    #[doc="Sets the UDRCF field."]
    #[inline] pub fn set_udrcf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="End of block clear flag"]
    #[inline] pub fn eobcf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if EOBCF != 0"]
    #[inline] pub fn test_eobcf(&self) -> bool {
        self.eobcf() != 0
    }

    #[doc="Sets the EOBCF field."]
    #[inline] pub fn set_eobcf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Receiver timeout clear flag"]
    #[inline] pub fn rtocf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if RTOCF != 0"]
    #[inline] pub fn test_rtocf(&self) -> bool {
        self.rtocf() != 0
    }

    #[doc="Sets the RTOCF field."]
    #[inline] pub fn set_rtocf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="CTS clear flag"]
    #[inline] pub fn ctscf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CTSCF != 0"]
    #[inline] pub fn test_ctscf(&self) -> bool {
        self.ctscf() != 0
    }

    #[doc="Sets the CTSCF field."]
    #[inline] pub fn set_ctscf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="LIN break detection clear flag"]
    #[inline] pub fn lbdcf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if LBDCF != 0"]
    #[inline] pub fn test_lbdcf(&self) -> bool {
        self.lbdcf() != 0
    }

    #[doc="Sets the LBDCF field."]
    #[inline] pub fn set_lbdcf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Transmission complete before Guard time clear flag"]
    #[inline] pub fn tcbgtcf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TCBGTCF != 0"]
    #[inline] pub fn test_tcbgtcf(&self) -> bool {
        self.tcbgtcf() != 0
    }

    #[doc="Sets the TCBGTCF field."]
    #[inline] pub fn set_tcbgtcf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Transmission complete clear flag"]
    #[inline] pub fn tccf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TCCF != 0"]
    #[inline] pub fn test_tccf(&self) -> bool {
        self.tccf() != 0
    }

    #[doc="Sets the TCCF field."]
    #[inline] pub fn set_tccf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="TXFIFO empty clear flag"]
    #[inline] pub fn txfecf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TXFECF != 0"]
    #[inline] pub fn test_txfecf(&self) -> bool {
        self.txfecf() != 0
    }

    #[doc="Sets the TXFECF field."]
    #[inline] pub fn set_txfecf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Idle line detected clear flag"]
    #[inline] pub fn idlecf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if IDLECF != 0"]
    #[inline] pub fn test_idlecf(&self) -> bool {
        self.idlecf() != 0
    }

    #[doc="Sets the IDLECF field."]
    #[inline] pub fn set_idlecf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Overrun error clear flag"]
    #[inline] pub fn orecf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ORECF != 0"]
    #[inline] pub fn test_orecf(&self) -> bool {
        self.orecf() != 0
    }

    #[doc="Sets the ORECF field."]
    #[inline] pub fn set_orecf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Noise detected clear flag"]
    #[inline] pub fn ncf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if NCF != 0"]
    #[inline] pub fn test_ncf(&self) -> bool {
        self.ncf() != 0
    }

    #[doc="Sets the NCF field."]
    #[inline] pub fn set_ncf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Framing error clear flag"]
    #[inline] pub fn fecf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if FECF != 0"]
    #[inline] pub fn test_fecf(&self) -> bool {
        self.fecf() != 0
    }

    #[doc="Sets the FECF field."]
    #[inline] pub fn set_fecf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Parity error clear flag"]
    #[inline] pub fn pecf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PECF != 0"]
    #[inline] pub fn test_pecf(&self) -> bool {
        self.pecf() != 0
    }

    #[doc="Sets the PECF field."]
    #[inline] pub fn set_pecf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
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
        if self.udrcf() != 0 { try!(write!(f, " udrcf"))}
        if self.eobcf() != 0 { try!(write!(f, " eobcf"))}
        if self.rtocf() != 0 { try!(write!(f, " rtocf"))}
        if self.ctscf() != 0 { try!(write!(f, " ctscf"))}
        if self.lbdcf() != 0 { try!(write!(f, " lbdcf"))}
        if self.tcbgtcf() != 0 { try!(write!(f, " tcbgtcf"))}
        if self.tccf() != 0 { try!(write!(f, " tccf"))}
        if self.txfecf() != 0 { try!(write!(f, " txfecf"))}
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
    #[inline] pub fn rdr(&self) -> ::bobbin_bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
    }

    #[doc="Returns true if RDR != 0"]
    #[inline] pub fn test_rdr(&self) -> bool {
        self.rdr() != 0
    }

    #[doc="Sets the RDR field."]
    #[inline] pub fn set_rdr<V: Into<::bobbin_bits::U9>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U9 = value.into();
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
    #[inline] pub fn tdr(&self) -> ::bobbin_bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
    }

    #[doc="Returns true if TDR != 0"]
    #[inline] pub fn test_tdr(&self) -> bool {
        self.tdr() != 0
    }

    #[doc="Sets the TDR field."]
    #[inline] pub fn set_tdr<V: Into<::bobbin_bits::U9>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U9 = value.into();
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

#[doc="Prescaler register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Presc(pub u32);
impl Presc {
    #[doc="Clock prescaler"]
    #[inline] pub fn prescaler(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if PRESCALER != 0"]
    #[inline] pub fn test_prescaler(&self) -> bool {
        self.prescaler() != 0
    }

    #[doc="Sets the PRESCALER field."]
    #[inline] pub fn set_prescaler<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Presc {
    #[inline]
    fn from(other: u32) -> Self {
         Presc(other)
    }
}

impl ::core::fmt::Display for Presc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Presc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.prescaler() != 0 { try!(write!(f, " prescaler=0x{:x}", self.prescaler()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

