
::bobbin_mcu::periph!( HASH, Hash, HASH_PERIPH, HashPeriph, HASH_OWNED, HASH_REF_COUNT, 0x50060400, 0x00, 0x0f);


// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("AHB2RSTR"), field: Some("HASHRST"), description: None }
impl ::bobbin_mcu::gate::GateRst for Hash {
    #[inline]
    fn gate_rst(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahb2rstr().hashrst() }
    #[inline]
    fn set_gate_rst<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2rstr(|r| r.set_hashrst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("AHB2ENR"), field: Some("HASHEN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Hash {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahb2enr().hashen() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2enr(|r| r.set_hashen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("AHB2LPENR"), field: Some("HASHLPEN"), description: None }
impl ::bobbin_mcu::gate::GateSleepEn for Hash {
    #[inline]
    fn gate_sleep_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahb2lpenr().hashlpen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2lpenr(|r| r.set_hashlpen(value));
        self
    }
}

#[doc="Hash processor"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct HashPeriph(pub usize);
impl HashPeriph {
    #[doc="Get the CR Register."]
    #[inline] pub fn cr_reg(&self) -> ::bobbin_mcu::register::Register<Cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cr, 0x0)
    }

    #[doc="Get the *mut pointer for the CR register."]
    #[inline] pub fn cr_mut(&self) -> *mut Cr { 
        self.cr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CR register."]
    #[inline] pub fn cr_ptr(&self) -> *const Cr { 
        self.cr_reg().ptr()
    }

    #[doc="Read the CR register."]
    #[inline] pub fn cr(&self) -> Cr { 
        self.cr_reg().read()
    }

    #[doc="Write the CR register."]
    #[inline] pub fn write_cr(&self, value: Cr) -> &Self { 
        self.cr_reg().write(value);
        self
    }

    #[doc="Set the CR register."]
    #[inline] pub fn set_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        self.cr_reg().set(f);
        self
    }

    #[doc="Modify the CR register."]
    #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        self.cr_reg().with(f);
        self
    }

    #[doc="Get the DIN Register."]
    #[inline] pub fn din_reg(&self) -> ::bobbin_mcu::register::Register<Din> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Din, 0x4)
    }

    #[doc="Get the *mut pointer for the DIN register."]
    #[inline] pub fn din_mut(&self) -> *mut Din { 
        self.din_reg().ptr()
    }

    #[doc="Get the *const pointer for the DIN register."]
    #[inline] pub fn din_ptr(&self) -> *const Din { 
        self.din_reg().ptr()
    }

    #[doc="Read the DIN register."]
    #[inline] pub fn din(&self) -> Din { 
        self.din_reg().read()
    }

    #[doc="Write the DIN register."]
    #[inline] pub fn write_din(&self, value: Din) -> &Self { 
        self.din_reg().write(value);
        self
    }

    #[doc="Set the DIN register."]
    #[inline] pub fn set_din<F: FnOnce(Din) -> Din>(&self, f: F) -> &Self {
        self.din_reg().set(f);
        self
    }

    #[doc="Modify the DIN register."]
    #[inline] pub fn with_din<F: FnOnce(Din) -> Din>(&self, f: F) -> &Self {
        self.din_reg().with(f);
        self
    }

    #[doc="Get the STR Register."]
    #[inline] pub fn str_reg(&self) -> ::bobbin_mcu::register::Register<Str> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Str, 0x8)
    }

    #[doc="Get the *mut pointer for the STR register."]
    #[inline] pub fn str_mut(&self) -> *mut Str { 
        self.str_reg().ptr()
    }

    #[doc="Get the *const pointer for the STR register."]
    #[inline] pub fn str_ptr(&self) -> *const Str { 
        self.str_reg().ptr()
    }

    #[doc="Read the STR register."]
    #[inline] pub fn str(&self) -> Str { 
        self.str_reg().read()
    }

    #[doc="Write the STR register."]
    #[inline] pub fn write_str(&self, value: Str) -> &Self { 
        self.str_reg().write(value);
        self
    }

    #[doc="Set the STR register."]
    #[inline] pub fn set_str<F: FnOnce(Str) -> Str>(&self, f: F) -> &Self {
        self.str_reg().set(f);
        self
    }

    #[doc="Modify the STR register."]
    #[inline] pub fn with_str<F: FnOnce(Str) -> Str>(&self, f: F) -> &Self {
        self.str_reg().with(f);
        self
    }

    #[doc="Get the HR0 Register."]
    #[inline] pub fn hr0_reg(&self) -> ::bobbin_mcu::register::Register<Hr0> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Hr0, 0xc)
    }

    #[doc="Get the *mut pointer for the HR0 register."]
    #[inline] pub fn hr0_mut(&self) -> *mut Hr0 { 
        self.hr0_reg().ptr()
    }

    #[doc="Get the *const pointer for the HR0 register."]
    #[inline] pub fn hr0_ptr(&self) -> *const Hr0 { 
        self.hr0_reg().ptr()
    }

    #[doc="Read the HR0 register."]
    #[inline] pub fn hr0(&self) -> Hr0 { 
        self.hr0_reg().read()
    }

    #[doc="Get the HR1 Register."]
    #[inline] pub fn hr1_reg(&self) -> ::bobbin_mcu::register::Register<Hr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Hr1, 0x10)
    }

    #[doc="Get the *mut pointer for the HR1 register."]
    #[inline] pub fn hr1_mut(&self) -> *mut Hr1 { 
        self.hr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the HR1 register."]
    #[inline] pub fn hr1_ptr(&self) -> *const Hr1 { 
        self.hr1_reg().ptr()
    }

    #[doc="Read the HR1 register."]
    #[inline] pub fn hr1(&self) -> Hr1 { 
        self.hr1_reg().read()
    }

    #[doc="Get the HR2 Register."]
    #[inline] pub fn hr2_reg(&self) -> ::bobbin_mcu::register::Register<Hr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Hr2, 0x14)
    }

    #[doc="Get the *mut pointer for the HR2 register."]
    #[inline] pub fn hr2_mut(&self) -> *mut Hr2 { 
        self.hr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the HR2 register."]
    #[inline] pub fn hr2_ptr(&self) -> *const Hr2 { 
        self.hr2_reg().ptr()
    }

    #[doc="Read the HR2 register."]
    #[inline] pub fn hr2(&self) -> Hr2 { 
        self.hr2_reg().read()
    }

    #[doc="Get the HR3 Register."]
    #[inline] pub fn hr3_reg(&self) -> ::bobbin_mcu::register::Register<Hr3> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Hr3, 0x18)
    }

    #[doc="Get the *mut pointer for the HR3 register."]
    #[inline] pub fn hr3_mut(&self) -> *mut Hr3 { 
        self.hr3_reg().ptr()
    }

    #[doc="Get the *const pointer for the HR3 register."]
    #[inline] pub fn hr3_ptr(&self) -> *const Hr3 { 
        self.hr3_reg().ptr()
    }

    #[doc="Read the HR3 register."]
    #[inline] pub fn hr3(&self) -> Hr3 { 
        self.hr3_reg().read()
    }

    #[doc="Get the HR4 Register."]
    #[inline] pub fn hr4_reg(&self) -> ::bobbin_mcu::register::Register<Hr4> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Hr4, 0x1c)
    }

    #[doc="Get the *mut pointer for the HR4 register."]
    #[inline] pub fn hr4_mut(&self) -> *mut Hr4 { 
        self.hr4_reg().ptr()
    }

    #[doc="Get the *const pointer for the HR4 register."]
    #[inline] pub fn hr4_ptr(&self) -> *const Hr4 { 
        self.hr4_reg().ptr()
    }

    #[doc="Read the HR4 register."]
    #[inline] pub fn hr4(&self) -> Hr4 { 
        self.hr4_reg().read()
    }

    #[doc="Get the IMR Register."]
    #[inline] pub fn imr_reg(&self) -> ::bobbin_mcu::register::Register<Imr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Imr, 0x20)
    }

    #[doc="Get the *mut pointer for the IMR register."]
    #[inline] pub fn imr_mut(&self) -> *mut Imr { 
        self.imr_reg().ptr()
    }

    #[doc="Get the *const pointer for the IMR register."]
    #[inline] pub fn imr_ptr(&self) -> *const Imr { 
        self.imr_reg().ptr()
    }

    #[doc="Read the IMR register."]
    #[inline] pub fn imr(&self) -> Imr { 
        self.imr_reg().read()
    }

    #[doc="Write the IMR register."]
    #[inline] pub fn write_imr(&self, value: Imr) -> &Self { 
        self.imr_reg().write(value);
        self
    }

    #[doc="Set the IMR register."]
    #[inline] pub fn set_imr<F: FnOnce(Imr) -> Imr>(&self, f: F) -> &Self {
        self.imr_reg().set(f);
        self
    }

    #[doc="Modify the IMR register."]
    #[inline] pub fn with_imr<F: FnOnce(Imr) -> Imr>(&self, f: F) -> &Self {
        self.imr_reg().with(f);
        self
    }

    #[doc="Get the SR Register."]
    #[inline] pub fn sr_reg(&self) -> ::bobbin_mcu::register::Register<Sr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sr, 0x24)
    }

    #[doc="Get the *mut pointer for the SR register."]
    #[inline] pub fn sr_mut(&self) -> *mut Sr { 
        self.sr_reg().ptr()
    }

    #[doc="Get the *const pointer for the SR register."]
    #[inline] pub fn sr_ptr(&self) -> *const Sr { 
        self.sr_reg().ptr()
    }

    #[doc="Read the SR register."]
    #[inline] pub fn sr(&self) -> Sr { 
        self.sr_reg().read()
    }

    #[doc="Write the SR register."]
    #[inline] pub fn write_sr(&self, value: Sr) -> &Self { 
        self.sr_reg().write(value);
        self
    }

    #[doc="Set the SR register."]
    #[inline] pub fn set_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        self.sr_reg().set(f);
        self
    }

    #[doc="Modify the SR register."]
    #[inline] pub fn with_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        self.sr_reg().with(f);
        self
    }

    #[doc="Get the CSR0 Register."]
    #[inline] pub fn csr0_reg(&self) -> ::bobbin_mcu::register::Register<Csr0> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr0, 0xf8)
    }

    #[doc="Get the *mut pointer for the CSR0 register."]
    #[inline] pub fn csr0_mut(&self) -> *mut Csr0 { 
        self.csr0_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR0 register."]
    #[inline] pub fn csr0_ptr(&self) -> *const Csr0 { 
        self.csr0_reg().ptr()
    }

    #[doc="Read the CSR0 register."]
    #[inline] pub fn csr0(&self) -> Csr0 { 
        self.csr0_reg().read()
    }

    #[doc="Write the CSR0 register."]
    #[inline] pub fn write_csr0(&self, value: Csr0) -> &Self { 
        self.csr0_reg().write(value);
        self
    }

    #[doc="Set the CSR0 register."]
    #[inline] pub fn set_csr0<F: FnOnce(Csr0) -> Csr0>(&self, f: F) -> &Self {
        self.csr0_reg().set(f);
        self
    }

    #[doc="Modify the CSR0 register."]
    #[inline] pub fn with_csr0<F: FnOnce(Csr0) -> Csr0>(&self, f: F) -> &Self {
        self.csr0_reg().with(f);
        self
    }

    #[doc="Get the CSR1 Register."]
    #[inline] pub fn csr1_reg(&self) -> ::bobbin_mcu::register::Register<Csr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr1, 0xfc)
    }

    #[doc="Get the *mut pointer for the CSR1 register."]
    #[inline] pub fn csr1_mut(&self) -> *mut Csr1 { 
        self.csr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR1 register."]
    #[inline] pub fn csr1_ptr(&self) -> *const Csr1 { 
        self.csr1_reg().ptr()
    }

    #[doc="Read the CSR1 register."]
    #[inline] pub fn csr1(&self) -> Csr1 { 
        self.csr1_reg().read()
    }

    #[doc="Write the CSR1 register."]
    #[inline] pub fn write_csr1(&self, value: Csr1) -> &Self { 
        self.csr1_reg().write(value);
        self
    }

    #[doc="Set the CSR1 register."]
    #[inline] pub fn set_csr1<F: FnOnce(Csr1) -> Csr1>(&self, f: F) -> &Self {
        self.csr1_reg().set(f);
        self
    }

    #[doc="Modify the CSR1 register."]
    #[inline] pub fn with_csr1<F: FnOnce(Csr1) -> Csr1>(&self, f: F) -> &Self {
        self.csr1_reg().with(f);
        self
    }

    #[doc="Get the CSR2 Register."]
    #[inline] pub fn csr2_reg(&self) -> ::bobbin_mcu::register::Register<Csr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr2, 0x100)
    }

    #[doc="Get the *mut pointer for the CSR2 register."]
    #[inline] pub fn csr2_mut(&self) -> *mut Csr2 { 
        self.csr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR2 register."]
    #[inline] pub fn csr2_ptr(&self) -> *const Csr2 { 
        self.csr2_reg().ptr()
    }

    #[doc="Read the CSR2 register."]
    #[inline] pub fn csr2(&self) -> Csr2 { 
        self.csr2_reg().read()
    }

    #[doc="Write the CSR2 register."]
    #[inline] pub fn write_csr2(&self, value: Csr2) -> &Self { 
        self.csr2_reg().write(value);
        self
    }

    #[doc="Set the CSR2 register."]
    #[inline] pub fn set_csr2<F: FnOnce(Csr2) -> Csr2>(&self, f: F) -> &Self {
        self.csr2_reg().set(f);
        self
    }

    #[doc="Modify the CSR2 register."]
    #[inline] pub fn with_csr2<F: FnOnce(Csr2) -> Csr2>(&self, f: F) -> &Self {
        self.csr2_reg().with(f);
        self
    }

    #[doc="Get the CSR3 Register."]
    #[inline] pub fn csr3_reg(&self) -> ::bobbin_mcu::register::Register<Csr3> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr3, 0x104)
    }

    #[doc="Get the *mut pointer for the CSR3 register."]
    #[inline] pub fn csr3_mut(&self) -> *mut Csr3 { 
        self.csr3_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR3 register."]
    #[inline] pub fn csr3_ptr(&self) -> *const Csr3 { 
        self.csr3_reg().ptr()
    }

    #[doc="Read the CSR3 register."]
    #[inline] pub fn csr3(&self) -> Csr3 { 
        self.csr3_reg().read()
    }

    #[doc="Write the CSR3 register."]
    #[inline] pub fn write_csr3(&self, value: Csr3) -> &Self { 
        self.csr3_reg().write(value);
        self
    }

    #[doc="Set the CSR3 register."]
    #[inline] pub fn set_csr3<F: FnOnce(Csr3) -> Csr3>(&self, f: F) -> &Self {
        self.csr3_reg().set(f);
        self
    }

    #[doc="Modify the CSR3 register."]
    #[inline] pub fn with_csr3<F: FnOnce(Csr3) -> Csr3>(&self, f: F) -> &Self {
        self.csr3_reg().with(f);
        self
    }

    #[doc="Get the CSR4 Register."]
    #[inline] pub fn csr4_reg(&self) -> ::bobbin_mcu::register::Register<Csr4> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr4, 0x108)
    }

    #[doc="Get the *mut pointer for the CSR4 register."]
    #[inline] pub fn csr4_mut(&self) -> *mut Csr4 { 
        self.csr4_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR4 register."]
    #[inline] pub fn csr4_ptr(&self) -> *const Csr4 { 
        self.csr4_reg().ptr()
    }

    #[doc="Read the CSR4 register."]
    #[inline] pub fn csr4(&self) -> Csr4 { 
        self.csr4_reg().read()
    }

    #[doc="Write the CSR4 register."]
    #[inline] pub fn write_csr4(&self, value: Csr4) -> &Self { 
        self.csr4_reg().write(value);
        self
    }

    #[doc="Set the CSR4 register."]
    #[inline] pub fn set_csr4<F: FnOnce(Csr4) -> Csr4>(&self, f: F) -> &Self {
        self.csr4_reg().set(f);
        self
    }

    #[doc="Modify the CSR4 register."]
    #[inline] pub fn with_csr4<F: FnOnce(Csr4) -> Csr4>(&self, f: F) -> &Self {
        self.csr4_reg().with(f);
        self
    }

    #[doc="Get the CSR5 Register."]
    #[inline] pub fn csr5_reg(&self) -> ::bobbin_mcu::register::Register<Csr5> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr5, 0x10c)
    }

    #[doc="Get the *mut pointer for the CSR5 register."]
    #[inline] pub fn csr5_mut(&self) -> *mut Csr5 { 
        self.csr5_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR5 register."]
    #[inline] pub fn csr5_ptr(&self) -> *const Csr5 { 
        self.csr5_reg().ptr()
    }

    #[doc="Read the CSR5 register."]
    #[inline] pub fn csr5(&self) -> Csr5 { 
        self.csr5_reg().read()
    }

    #[doc="Write the CSR5 register."]
    #[inline] pub fn write_csr5(&self, value: Csr5) -> &Self { 
        self.csr5_reg().write(value);
        self
    }

    #[doc="Set the CSR5 register."]
    #[inline] pub fn set_csr5<F: FnOnce(Csr5) -> Csr5>(&self, f: F) -> &Self {
        self.csr5_reg().set(f);
        self
    }

    #[doc="Modify the CSR5 register."]
    #[inline] pub fn with_csr5<F: FnOnce(Csr5) -> Csr5>(&self, f: F) -> &Self {
        self.csr5_reg().with(f);
        self
    }

    #[doc="Get the CSR6 Register."]
    #[inline] pub fn csr6_reg(&self) -> ::bobbin_mcu::register::Register<Csr6> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr6, 0x110)
    }

    #[doc="Get the *mut pointer for the CSR6 register."]
    #[inline] pub fn csr6_mut(&self) -> *mut Csr6 { 
        self.csr6_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR6 register."]
    #[inline] pub fn csr6_ptr(&self) -> *const Csr6 { 
        self.csr6_reg().ptr()
    }

    #[doc="Read the CSR6 register."]
    #[inline] pub fn csr6(&self) -> Csr6 { 
        self.csr6_reg().read()
    }

    #[doc="Write the CSR6 register."]
    #[inline] pub fn write_csr6(&self, value: Csr6) -> &Self { 
        self.csr6_reg().write(value);
        self
    }

    #[doc="Set the CSR6 register."]
    #[inline] pub fn set_csr6<F: FnOnce(Csr6) -> Csr6>(&self, f: F) -> &Self {
        self.csr6_reg().set(f);
        self
    }

    #[doc="Modify the CSR6 register."]
    #[inline] pub fn with_csr6<F: FnOnce(Csr6) -> Csr6>(&self, f: F) -> &Self {
        self.csr6_reg().with(f);
        self
    }

    #[doc="Get the CSR7 Register."]
    #[inline] pub fn csr7_reg(&self) -> ::bobbin_mcu::register::Register<Csr7> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr7, 0x114)
    }

    #[doc="Get the *mut pointer for the CSR7 register."]
    #[inline] pub fn csr7_mut(&self) -> *mut Csr7 { 
        self.csr7_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR7 register."]
    #[inline] pub fn csr7_ptr(&self) -> *const Csr7 { 
        self.csr7_reg().ptr()
    }

    #[doc="Read the CSR7 register."]
    #[inline] pub fn csr7(&self) -> Csr7 { 
        self.csr7_reg().read()
    }

    #[doc="Write the CSR7 register."]
    #[inline] pub fn write_csr7(&self, value: Csr7) -> &Self { 
        self.csr7_reg().write(value);
        self
    }

    #[doc="Set the CSR7 register."]
    #[inline] pub fn set_csr7<F: FnOnce(Csr7) -> Csr7>(&self, f: F) -> &Self {
        self.csr7_reg().set(f);
        self
    }

    #[doc="Modify the CSR7 register."]
    #[inline] pub fn with_csr7<F: FnOnce(Csr7) -> Csr7>(&self, f: F) -> &Self {
        self.csr7_reg().with(f);
        self
    }

    #[doc="Get the CSR8 Register."]
    #[inline] pub fn csr8_reg(&self) -> ::bobbin_mcu::register::Register<Csr8> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr8, 0x118)
    }

    #[doc="Get the *mut pointer for the CSR8 register."]
    #[inline] pub fn csr8_mut(&self) -> *mut Csr8 { 
        self.csr8_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR8 register."]
    #[inline] pub fn csr8_ptr(&self) -> *const Csr8 { 
        self.csr8_reg().ptr()
    }

    #[doc="Read the CSR8 register."]
    #[inline] pub fn csr8(&self) -> Csr8 { 
        self.csr8_reg().read()
    }

    #[doc="Write the CSR8 register."]
    #[inline] pub fn write_csr8(&self, value: Csr8) -> &Self { 
        self.csr8_reg().write(value);
        self
    }

    #[doc="Set the CSR8 register."]
    #[inline] pub fn set_csr8<F: FnOnce(Csr8) -> Csr8>(&self, f: F) -> &Self {
        self.csr8_reg().set(f);
        self
    }

    #[doc="Modify the CSR8 register."]
    #[inline] pub fn with_csr8<F: FnOnce(Csr8) -> Csr8>(&self, f: F) -> &Self {
        self.csr8_reg().with(f);
        self
    }

    #[doc="Get the CSR9 Register."]
    #[inline] pub fn csr9_reg(&self) -> ::bobbin_mcu::register::Register<Csr9> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr9, 0x11c)
    }

    #[doc="Get the *mut pointer for the CSR9 register."]
    #[inline] pub fn csr9_mut(&self) -> *mut Csr9 { 
        self.csr9_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR9 register."]
    #[inline] pub fn csr9_ptr(&self) -> *const Csr9 { 
        self.csr9_reg().ptr()
    }

    #[doc="Read the CSR9 register."]
    #[inline] pub fn csr9(&self) -> Csr9 { 
        self.csr9_reg().read()
    }

    #[doc="Write the CSR9 register."]
    #[inline] pub fn write_csr9(&self, value: Csr9) -> &Self { 
        self.csr9_reg().write(value);
        self
    }

    #[doc="Set the CSR9 register."]
    #[inline] pub fn set_csr9<F: FnOnce(Csr9) -> Csr9>(&self, f: F) -> &Self {
        self.csr9_reg().set(f);
        self
    }

    #[doc="Modify the CSR9 register."]
    #[inline] pub fn with_csr9<F: FnOnce(Csr9) -> Csr9>(&self, f: F) -> &Self {
        self.csr9_reg().with(f);
        self
    }

    #[doc="Get the CSR10 Register."]
    #[inline] pub fn csr10_reg(&self) -> ::bobbin_mcu::register::Register<Csr10> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr10, 0x120)
    }

    #[doc="Get the *mut pointer for the CSR10 register."]
    #[inline] pub fn csr10_mut(&self) -> *mut Csr10 { 
        self.csr10_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR10 register."]
    #[inline] pub fn csr10_ptr(&self) -> *const Csr10 { 
        self.csr10_reg().ptr()
    }

    #[doc="Read the CSR10 register."]
    #[inline] pub fn csr10(&self) -> Csr10 { 
        self.csr10_reg().read()
    }

    #[doc="Write the CSR10 register."]
    #[inline] pub fn write_csr10(&self, value: Csr10) -> &Self { 
        self.csr10_reg().write(value);
        self
    }

    #[doc="Set the CSR10 register."]
    #[inline] pub fn set_csr10<F: FnOnce(Csr10) -> Csr10>(&self, f: F) -> &Self {
        self.csr10_reg().set(f);
        self
    }

    #[doc="Modify the CSR10 register."]
    #[inline] pub fn with_csr10<F: FnOnce(Csr10) -> Csr10>(&self, f: F) -> &Self {
        self.csr10_reg().with(f);
        self
    }

    #[doc="Get the CSR11 Register."]
    #[inline] pub fn csr11_reg(&self) -> ::bobbin_mcu::register::Register<Csr11> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr11, 0x124)
    }

    #[doc="Get the *mut pointer for the CSR11 register."]
    #[inline] pub fn csr11_mut(&self) -> *mut Csr11 { 
        self.csr11_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR11 register."]
    #[inline] pub fn csr11_ptr(&self) -> *const Csr11 { 
        self.csr11_reg().ptr()
    }

    #[doc="Read the CSR11 register."]
    #[inline] pub fn csr11(&self) -> Csr11 { 
        self.csr11_reg().read()
    }

    #[doc="Write the CSR11 register."]
    #[inline] pub fn write_csr11(&self, value: Csr11) -> &Self { 
        self.csr11_reg().write(value);
        self
    }

    #[doc="Set the CSR11 register."]
    #[inline] pub fn set_csr11<F: FnOnce(Csr11) -> Csr11>(&self, f: F) -> &Self {
        self.csr11_reg().set(f);
        self
    }

    #[doc="Modify the CSR11 register."]
    #[inline] pub fn with_csr11<F: FnOnce(Csr11) -> Csr11>(&self, f: F) -> &Self {
        self.csr11_reg().with(f);
        self
    }

    #[doc="Get the CSR12 Register."]
    #[inline] pub fn csr12_reg(&self) -> ::bobbin_mcu::register::Register<Csr12> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr12, 0x128)
    }

    #[doc="Get the *mut pointer for the CSR12 register."]
    #[inline] pub fn csr12_mut(&self) -> *mut Csr12 { 
        self.csr12_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR12 register."]
    #[inline] pub fn csr12_ptr(&self) -> *const Csr12 { 
        self.csr12_reg().ptr()
    }

    #[doc="Read the CSR12 register."]
    #[inline] pub fn csr12(&self) -> Csr12 { 
        self.csr12_reg().read()
    }

    #[doc="Write the CSR12 register."]
    #[inline] pub fn write_csr12(&self, value: Csr12) -> &Self { 
        self.csr12_reg().write(value);
        self
    }

    #[doc="Set the CSR12 register."]
    #[inline] pub fn set_csr12<F: FnOnce(Csr12) -> Csr12>(&self, f: F) -> &Self {
        self.csr12_reg().set(f);
        self
    }

    #[doc="Modify the CSR12 register."]
    #[inline] pub fn with_csr12<F: FnOnce(Csr12) -> Csr12>(&self, f: F) -> &Self {
        self.csr12_reg().with(f);
        self
    }

    #[doc="Get the CSR13 Register."]
    #[inline] pub fn csr13_reg(&self) -> ::bobbin_mcu::register::Register<Csr13> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr13, 0x12c)
    }

    #[doc="Get the *mut pointer for the CSR13 register."]
    #[inline] pub fn csr13_mut(&self) -> *mut Csr13 { 
        self.csr13_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR13 register."]
    #[inline] pub fn csr13_ptr(&self) -> *const Csr13 { 
        self.csr13_reg().ptr()
    }

    #[doc="Read the CSR13 register."]
    #[inline] pub fn csr13(&self) -> Csr13 { 
        self.csr13_reg().read()
    }

    #[doc="Write the CSR13 register."]
    #[inline] pub fn write_csr13(&self, value: Csr13) -> &Self { 
        self.csr13_reg().write(value);
        self
    }

    #[doc="Set the CSR13 register."]
    #[inline] pub fn set_csr13<F: FnOnce(Csr13) -> Csr13>(&self, f: F) -> &Self {
        self.csr13_reg().set(f);
        self
    }

    #[doc="Modify the CSR13 register."]
    #[inline] pub fn with_csr13<F: FnOnce(Csr13) -> Csr13>(&self, f: F) -> &Self {
        self.csr13_reg().with(f);
        self
    }

    #[doc="Get the CSR14 Register."]
    #[inline] pub fn csr14_reg(&self) -> ::bobbin_mcu::register::Register<Csr14> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr14, 0x130)
    }

    #[doc="Get the *mut pointer for the CSR14 register."]
    #[inline] pub fn csr14_mut(&self) -> *mut Csr14 { 
        self.csr14_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR14 register."]
    #[inline] pub fn csr14_ptr(&self) -> *const Csr14 { 
        self.csr14_reg().ptr()
    }

    #[doc="Read the CSR14 register."]
    #[inline] pub fn csr14(&self) -> Csr14 { 
        self.csr14_reg().read()
    }

    #[doc="Write the CSR14 register."]
    #[inline] pub fn write_csr14(&self, value: Csr14) -> &Self { 
        self.csr14_reg().write(value);
        self
    }

    #[doc="Set the CSR14 register."]
    #[inline] pub fn set_csr14<F: FnOnce(Csr14) -> Csr14>(&self, f: F) -> &Self {
        self.csr14_reg().set(f);
        self
    }

    #[doc="Modify the CSR14 register."]
    #[inline] pub fn with_csr14<F: FnOnce(Csr14) -> Csr14>(&self, f: F) -> &Self {
        self.csr14_reg().with(f);
        self
    }

    #[doc="Get the CSR15 Register."]
    #[inline] pub fn csr15_reg(&self) -> ::bobbin_mcu::register::Register<Csr15> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr15, 0x134)
    }

    #[doc="Get the *mut pointer for the CSR15 register."]
    #[inline] pub fn csr15_mut(&self) -> *mut Csr15 { 
        self.csr15_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR15 register."]
    #[inline] pub fn csr15_ptr(&self) -> *const Csr15 { 
        self.csr15_reg().ptr()
    }

    #[doc="Read the CSR15 register."]
    #[inline] pub fn csr15(&self) -> Csr15 { 
        self.csr15_reg().read()
    }

    #[doc="Write the CSR15 register."]
    #[inline] pub fn write_csr15(&self, value: Csr15) -> &Self { 
        self.csr15_reg().write(value);
        self
    }

    #[doc="Set the CSR15 register."]
    #[inline] pub fn set_csr15<F: FnOnce(Csr15) -> Csr15>(&self, f: F) -> &Self {
        self.csr15_reg().set(f);
        self
    }

    #[doc="Modify the CSR15 register."]
    #[inline] pub fn with_csr15<F: FnOnce(Csr15) -> Csr15>(&self, f: F) -> &Self {
        self.csr15_reg().with(f);
        self
    }

    #[doc="Get the CSR16 Register."]
    #[inline] pub fn csr16_reg(&self) -> ::bobbin_mcu::register::Register<Csr16> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr16, 0x138)
    }

    #[doc="Get the *mut pointer for the CSR16 register."]
    #[inline] pub fn csr16_mut(&self) -> *mut Csr16 { 
        self.csr16_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR16 register."]
    #[inline] pub fn csr16_ptr(&self) -> *const Csr16 { 
        self.csr16_reg().ptr()
    }

    #[doc="Read the CSR16 register."]
    #[inline] pub fn csr16(&self) -> Csr16 { 
        self.csr16_reg().read()
    }

    #[doc="Write the CSR16 register."]
    #[inline] pub fn write_csr16(&self, value: Csr16) -> &Self { 
        self.csr16_reg().write(value);
        self
    }

    #[doc="Set the CSR16 register."]
    #[inline] pub fn set_csr16<F: FnOnce(Csr16) -> Csr16>(&self, f: F) -> &Self {
        self.csr16_reg().set(f);
        self
    }

    #[doc="Modify the CSR16 register."]
    #[inline] pub fn with_csr16<F: FnOnce(Csr16) -> Csr16>(&self, f: F) -> &Self {
        self.csr16_reg().with(f);
        self
    }

    #[doc="Get the CSR17 Register."]
    #[inline] pub fn csr17_reg(&self) -> ::bobbin_mcu::register::Register<Csr17> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr17, 0x13c)
    }

    #[doc="Get the *mut pointer for the CSR17 register."]
    #[inline] pub fn csr17_mut(&self) -> *mut Csr17 { 
        self.csr17_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR17 register."]
    #[inline] pub fn csr17_ptr(&self) -> *const Csr17 { 
        self.csr17_reg().ptr()
    }

    #[doc="Read the CSR17 register."]
    #[inline] pub fn csr17(&self) -> Csr17 { 
        self.csr17_reg().read()
    }

    #[doc="Write the CSR17 register."]
    #[inline] pub fn write_csr17(&self, value: Csr17) -> &Self { 
        self.csr17_reg().write(value);
        self
    }

    #[doc="Set the CSR17 register."]
    #[inline] pub fn set_csr17<F: FnOnce(Csr17) -> Csr17>(&self, f: F) -> &Self {
        self.csr17_reg().set(f);
        self
    }

    #[doc="Modify the CSR17 register."]
    #[inline] pub fn with_csr17<F: FnOnce(Csr17) -> Csr17>(&self, f: F) -> &Self {
        self.csr17_reg().with(f);
        self
    }

    #[doc="Get the CSR18 Register."]
    #[inline] pub fn csr18_reg(&self) -> ::bobbin_mcu::register::Register<Csr18> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr18, 0x140)
    }

    #[doc="Get the *mut pointer for the CSR18 register."]
    #[inline] pub fn csr18_mut(&self) -> *mut Csr18 { 
        self.csr18_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR18 register."]
    #[inline] pub fn csr18_ptr(&self) -> *const Csr18 { 
        self.csr18_reg().ptr()
    }

    #[doc="Read the CSR18 register."]
    #[inline] pub fn csr18(&self) -> Csr18 { 
        self.csr18_reg().read()
    }

    #[doc="Write the CSR18 register."]
    #[inline] pub fn write_csr18(&self, value: Csr18) -> &Self { 
        self.csr18_reg().write(value);
        self
    }

    #[doc="Set the CSR18 register."]
    #[inline] pub fn set_csr18<F: FnOnce(Csr18) -> Csr18>(&self, f: F) -> &Self {
        self.csr18_reg().set(f);
        self
    }

    #[doc="Modify the CSR18 register."]
    #[inline] pub fn with_csr18<F: FnOnce(Csr18) -> Csr18>(&self, f: F) -> &Self {
        self.csr18_reg().with(f);
        self
    }

    #[doc="Get the CSR19 Register."]
    #[inline] pub fn csr19_reg(&self) -> ::bobbin_mcu::register::Register<Csr19> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr19, 0x144)
    }

    #[doc="Get the *mut pointer for the CSR19 register."]
    #[inline] pub fn csr19_mut(&self) -> *mut Csr19 { 
        self.csr19_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR19 register."]
    #[inline] pub fn csr19_ptr(&self) -> *const Csr19 { 
        self.csr19_reg().ptr()
    }

    #[doc="Read the CSR19 register."]
    #[inline] pub fn csr19(&self) -> Csr19 { 
        self.csr19_reg().read()
    }

    #[doc="Write the CSR19 register."]
    #[inline] pub fn write_csr19(&self, value: Csr19) -> &Self { 
        self.csr19_reg().write(value);
        self
    }

    #[doc="Set the CSR19 register."]
    #[inline] pub fn set_csr19<F: FnOnce(Csr19) -> Csr19>(&self, f: F) -> &Self {
        self.csr19_reg().set(f);
        self
    }

    #[doc="Modify the CSR19 register."]
    #[inline] pub fn with_csr19<F: FnOnce(Csr19) -> Csr19>(&self, f: F) -> &Self {
        self.csr19_reg().with(f);
        self
    }

    #[doc="Get the CSR20 Register."]
    #[inline] pub fn csr20_reg(&self) -> ::bobbin_mcu::register::Register<Csr20> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr20, 0x148)
    }

    #[doc="Get the *mut pointer for the CSR20 register."]
    #[inline] pub fn csr20_mut(&self) -> *mut Csr20 { 
        self.csr20_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR20 register."]
    #[inline] pub fn csr20_ptr(&self) -> *const Csr20 { 
        self.csr20_reg().ptr()
    }

    #[doc="Read the CSR20 register."]
    #[inline] pub fn csr20(&self) -> Csr20 { 
        self.csr20_reg().read()
    }

    #[doc="Write the CSR20 register."]
    #[inline] pub fn write_csr20(&self, value: Csr20) -> &Self { 
        self.csr20_reg().write(value);
        self
    }

    #[doc="Set the CSR20 register."]
    #[inline] pub fn set_csr20<F: FnOnce(Csr20) -> Csr20>(&self, f: F) -> &Self {
        self.csr20_reg().set(f);
        self
    }

    #[doc="Modify the CSR20 register."]
    #[inline] pub fn with_csr20<F: FnOnce(Csr20) -> Csr20>(&self, f: F) -> &Self {
        self.csr20_reg().with(f);
        self
    }

    #[doc="Get the CSR21 Register."]
    #[inline] pub fn csr21_reg(&self) -> ::bobbin_mcu::register::Register<Csr21> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr21, 0x14c)
    }

    #[doc="Get the *mut pointer for the CSR21 register."]
    #[inline] pub fn csr21_mut(&self) -> *mut Csr21 { 
        self.csr21_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR21 register."]
    #[inline] pub fn csr21_ptr(&self) -> *const Csr21 { 
        self.csr21_reg().ptr()
    }

    #[doc="Read the CSR21 register."]
    #[inline] pub fn csr21(&self) -> Csr21 { 
        self.csr21_reg().read()
    }

    #[doc="Write the CSR21 register."]
    #[inline] pub fn write_csr21(&self, value: Csr21) -> &Self { 
        self.csr21_reg().write(value);
        self
    }

    #[doc="Set the CSR21 register."]
    #[inline] pub fn set_csr21<F: FnOnce(Csr21) -> Csr21>(&self, f: F) -> &Self {
        self.csr21_reg().set(f);
        self
    }

    #[doc="Modify the CSR21 register."]
    #[inline] pub fn with_csr21<F: FnOnce(Csr21) -> Csr21>(&self, f: F) -> &Self {
        self.csr21_reg().with(f);
        self
    }

    #[doc="Get the CSR22 Register."]
    #[inline] pub fn csr22_reg(&self) -> ::bobbin_mcu::register::Register<Csr22> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr22, 0x150)
    }

    #[doc="Get the *mut pointer for the CSR22 register."]
    #[inline] pub fn csr22_mut(&self) -> *mut Csr22 { 
        self.csr22_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR22 register."]
    #[inline] pub fn csr22_ptr(&self) -> *const Csr22 { 
        self.csr22_reg().ptr()
    }

    #[doc="Read the CSR22 register."]
    #[inline] pub fn csr22(&self) -> Csr22 { 
        self.csr22_reg().read()
    }

    #[doc="Write the CSR22 register."]
    #[inline] pub fn write_csr22(&self, value: Csr22) -> &Self { 
        self.csr22_reg().write(value);
        self
    }

    #[doc="Set the CSR22 register."]
    #[inline] pub fn set_csr22<F: FnOnce(Csr22) -> Csr22>(&self, f: F) -> &Self {
        self.csr22_reg().set(f);
        self
    }

    #[doc="Modify the CSR22 register."]
    #[inline] pub fn with_csr22<F: FnOnce(Csr22) -> Csr22>(&self, f: F) -> &Self {
        self.csr22_reg().with(f);
        self
    }

    #[doc="Get the CSR23 Register."]
    #[inline] pub fn csr23_reg(&self) -> ::bobbin_mcu::register::Register<Csr23> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr23, 0x154)
    }

    #[doc="Get the *mut pointer for the CSR23 register."]
    #[inline] pub fn csr23_mut(&self) -> *mut Csr23 { 
        self.csr23_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR23 register."]
    #[inline] pub fn csr23_ptr(&self) -> *const Csr23 { 
        self.csr23_reg().ptr()
    }

    #[doc="Read the CSR23 register."]
    #[inline] pub fn csr23(&self) -> Csr23 { 
        self.csr23_reg().read()
    }

    #[doc="Write the CSR23 register."]
    #[inline] pub fn write_csr23(&self, value: Csr23) -> &Self { 
        self.csr23_reg().write(value);
        self
    }

    #[doc="Set the CSR23 register."]
    #[inline] pub fn set_csr23<F: FnOnce(Csr23) -> Csr23>(&self, f: F) -> &Self {
        self.csr23_reg().set(f);
        self
    }

    #[doc="Modify the CSR23 register."]
    #[inline] pub fn with_csr23<F: FnOnce(Csr23) -> Csr23>(&self, f: F) -> &Self {
        self.csr23_reg().with(f);
        self
    }

    #[doc="Get the CSR24 Register."]
    #[inline] pub fn csr24_reg(&self) -> ::bobbin_mcu::register::Register<Csr24> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr24, 0x158)
    }

    #[doc="Get the *mut pointer for the CSR24 register."]
    #[inline] pub fn csr24_mut(&self) -> *mut Csr24 { 
        self.csr24_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR24 register."]
    #[inline] pub fn csr24_ptr(&self) -> *const Csr24 { 
        self.csr24_reg().ptr()
    }

    #[doc="Read the CSR24 register."]
    #[inline] pub fn csr24(&self) -> Csr24 { 
        self.csr24_reg().read()
    }

    #[doc="Write the CSR24 register."]
    #[inline] pub fn write_csr24(&self, value: Csr24) -> &Self { 
        self.csr24_reg().write(value);
        self
    }

    #[doc="Set the CSR24 register."]
    #[inline] pub fn set_csr24<F: FnOnce(Csr24) -> Csr24>(&self, f: F) -> &Self {
        self.csr24_reg().set(f);
        self
    }

    #[doc="Modify the CSR24 register."]
    #[inline] pub fn with_csr24<F: FnOnce(Csr24) -> Csr24>(&self, f: F) -> &Self {
        self.csr24_reg().with(f);
        self
    }

    #[doc="Get the CSR25 Register."]
    #[inline] pub fn csr25_reg(&self) -> ::bobbin_mcu::register::Register<Csr25> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr25, 0x15c)
    }

    #[doc="Get the *mut pointer for the CSR25 register."]
    #[inline] pub fn csr25_mut(&self) -> *mut Csr25 { 
        self.csr25_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR25 register."]
    #[inline] pub fn csr25_ptr(&self) -> *const Csr25 { 
        self.csr25_reg().ptr()
    }

    #[doc="Read the CSR25 register."]
    #[inline] pub fn csr25(&self) -> Csr25 { 
        self.csr25_reg().read()
    }

    #[doc="Write the CSR25 register."]
    #[inline] pub fn write_csr25(&self, value: Csr25) -> &Self { 
        self.csr25_reg().write(value);
        self
    }

    #[doc="Set the CSR25 register."]
    #[inline] pub fn set_csr25<F: FnOnce(Csr25) -> Csr25>(&self, f: F) -> &Self {
        self.csr25_reg().set(f);
        self
    }

    #[doc="Modify the CSR25 register."]
    #[inline] pub fn with_csr25<F: FnOnce(Csr25) -> Csr25>(&self, f: F) -> &Self {
        self.csr25_reg().with(f);
        self
    }

    #[doc="Get the CSR26 Register."]
    #[inline] pub fn csr26_reg(&self) -> ::bobbin_mcu::register::Register<Csr26> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr26, 0x160)
    }

    #[doc="Get the *mut pointer for the CSR26 register."]
    #[inline] pub fn csr26_mut(&self) -> *mut Csr26 { 
        self.csr26_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR26 register."]
    #[inline] pub fn csr26_ptr(&self) -> *const Csr26 { 
        self.csr26_reg().ptr()
    }

    #[doc="Read the CSR26 register."]
    #[inline] pub fn csr26(&self) -> Csr26 { 
        self.csr26_reg().read()
    }

    #[doc="Write the CSR26 register."]
    #[inline] pub fn write_csr26(&self, value: Csr26) -> &Self { 
        self.csr26_reg().write(value);
        self
    }

    #[doc="Set the CSR26 register."]
    #[inline] pub fn set_csr26<F: FnOnce(Csr26) -> Csr26>(&self, f: F) -> &Self {
        self.csr26_reg().set(f);
        self
    }

    #[doc="Modify the CSR26 register."]
    #[inline] pub fn with_csr26<F: FnOnce(Csr26) -> Csr26>(&self, f: F) -> &Self {
        self.csr26_reg().with(f);
        self
    }

    #[doc="Get the CSR27 Register."]
    #[inline] pub fn csr27_reg(&self) -> ::bobbin_mcu::register::Register<Csr27> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr27, 0x164)
    }

    #[doc="Get the *mut pointer for the CSR27 register."]
    #[inline] pub fn csr27_mut(&self) -> *mut Csr27 { 
        self.csr27_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR27 register."]
    #[inline] pub fn csr27_ptr(&self) -> *const Csr27 { 
        self.csr27_reg().ptr()
    }

    #[doc="Read the CSR27 register."]
    #[inline] pub fn csr27(&self) -> Csr27 { 
        self.csr27_reg().read()
    }

    #[doc="Write the CSR27 register."]
    #[inline] pub fn write_csr27(&self, value: Csr27) -> &Self { 
        self.csr27_reg().write(value);
        self
    }

    #[doc="Set the CSR27 register."]
    #[inline] pub fn set_csr27<F: FnOnce(Csr27) -> Csr27>(&self, f: F) -> &Self {
        self.csr27_reg().set(f);
        self
    }

    #[doc="Modify the CSR27 register."]
    #[inline] pub fn with_csr27<F: FnOnce(Csr27) -> Csr27>(&self, f: F) -> &Self {
        self.csr27_reg().with(f);
        self
    }

    #[doc="Get the CSR28 Register."]
    #[inline] pub fn csr28_reg(&self) -> ::bobbin_mcu::register::Register<Csr28> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr28, 0x168)
    }

    #[doc="Get the *mut pointer for the CSR28 register."]
    #[inline] pub fn csr28_mut(&self) -> *mut Csr28 { 
        self.csr28_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR28 register."]
    #[inline] pub fn csr28_ptr(&self) -> *const Csr28 { 
        self.csr28_reg().ptr()
    }

    #[doc="Read the CSR28 register."]
    #[inline] pub fn csr28(&self) -> Csr28 { 
        self.csr28_reg().read()
    }

    #[doc="Write the CSR28 register."]
    #[inline] pub fn write_csr28(&self, value: Csr28) -> &Self { 
        self.csr28_reg().write(value);
        self
    }

    #[doc="Set the CSR28 register."]
    #[inline] pub fn set_csr28<F: FnOnce(Csr28) -> Csr28>(&self, f: F) -> &Self {
        self.csr28_reg().set(f);
        self
    }

    #[doc="Modify the CSR28 register."]
    #[inline] pub fn with_csr28<F: FnOnce(Csr28) -> Csr28>(&self, f: F) -> &Self {
        self.csr28_reg().with(f);
        self
    }

    #[doc="Get the CSR29 Register."]
    #[inline] pub fn csr29_reg(&self) -> ::bobbin_mcu::register::Register<Csr29> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr29, 0x16c)
    }

    #[doc="Get the *mut pointer for the CSR29 register."]
    #[inline] pub fn csr29_mut(&self) -> *mut Csr29 { 
        self.csr29_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR29 register."]
    #[inline] pub fn csr29_ptr(&self) -> *const Csr29 { 
        self.csr29_reg().ptr()
    }

    #[doc="Read the CSR29 register."]
    #[inline] pub fn csr29(&self) -> Csr29 { 
        self.csr29_reg().read()
    }

    #[doc="Write the CSR29 register."]
    #[inline] pub fn write_csr29(&self, value: Csr29) -> &Self { 
        self.csr29_reg().write(value);
        self
    }

    #[doc="Set the CSR29 register."]
    #[inline] pub fn set_csr29<F: FnOnce(Csr29) -> Csr29>(&self, f: F) -> &Self {
        self.csr29_reg().set(f);
        self
    }

    #[doc="Modify the CSR29 register."]
    #[inline] pub fn with_csr29<F: FnOnce(Csr29) -> Csr29>(&self, f: F) -> &Self {
        self.csr29_reg().with(f);
        self
    }

    #[doc="Get the CSR30 Register."]
    #[inline] pub fn csr30_reg(&self) -> ::bobbin_mcu::register::Register<Csr30> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr30, 0x170)
    }

    #[doc="Get the *mut pointer for the CSR30 register."]
    #[inline] pub fn csr30_mut(&self) -> *mut Csr30 { 
        self.csr30_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR30 register."]
    #[inline] pub fn csr30_ptr(&self) -> *const Csr30 { 
        self.csr30_reg().ptr()
    }

    #[doc="Read the CSR30 register."]
    #[inline] pub fn csr30(&self) -> Csr30 { 
        self.csr30_reg().read()
    }

    #[doc="Write the CSR30 register."]
    #[inline] pub fn write_csr30(&self, value: Csr30) -> &Self { 
        self.csr30_reg().write(value);
        self
    }

    #[doc="Set the CSR30 register."]
    #[inline] pub fn set_csr30<F: FnOnce(Csr30) -> Csr30>(&self, f: F) -> &Self {
        self.csr30_reg().set(f);
        self
    }

    #[doc="Modify the CSR30 register."]
    #[inline] pub fn with_csr30<F: FnOnce(Csr30) -> Csr30>(&self, f: F) -> &Self {
        self.csr30_reg().with(f);
        self
    }

    #[doc="Get the CSR31 Register."]
    #[inline] pub fn csr31_reg(&self) -> ::bobbin_mcu::register::Register<Csr31> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr31, 0x174)
    }

    #[doc="Get the *mut pointer for the CSR31 register."]
    #[inline] pub fn csr31_mut(&self) -> *mut Csr31 { 
        self.csr31_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR31 register."]
    #[inline] pub fn csr31_ptr(&self) -> *const Csr31 { 
        self.csr31_reg().ptr()
    }

    #[doc="Read the CSR31 register."]
    #[inline] pub fn csr31(&self) -> Csr31 { 
        self.csr31_reg().read()
    }

    #[doc="Write the CSR31 register."]
    #[inline] pub fn write_csr31(&self, value: Csr31) -> &Self { 
        self.csr31_reg().write(value);
        self
    }

    #[doc="Set the CSR31 register."]
    #[inline] pub fn set_csr31<F: FnOnce(Csr31) -> Csr31>(&self, f: F) -> &Self {
        self.csr31_reg().set(f);
        self
    }

    #[doc="Modify the CSR31 register."]
    #[inline] pub fn with_csr31<F: FnOnce(Csr31) -> Csr31>(&self, f: F) -> &Self {
        self.csr31_reg().with(f);
        self
    }

    #[doc="Get the CSR32 Register."]
    #[inline] pub fn csr32_reg(&self) -> ::bobbin_mcu::register::Register<Csr32> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr32, 0x178)
    }

    #[doc="Get the *mut pointer for the CSR32 register."]
    #[inline] pub fn csr32_mut(&self) -> *mut Csr32 { 
        self.csr32_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR32 register."]
    #[inline] pub fn csr32_ptr(&self) -> *const Csr32 { 
        self.csr32_reg().ptr()
    }

    #[doc="Read the CSR32 register."]
    #[inline] pub fn csr32(&self) -> Csr32 { 
        self.csr32_reg().read()
    }

    #[doc="Write the CSR32 register."]
    #[inline] pub fn write_csr32(&self, value: Csr32) -> &Self { 
        self.csr32_reg().write(value);
        self
    }

    #[doc="Set the CSR32 register."]
    #[inline] pub fn set_csr32<F: FnOnce(Csr32) -> Csr32>(&self, f: F) -> &Self {
        self.csr32_reg().set(f);
        self
    }

    #[doc="Modify the CSR32 register."]
    #[inline] pub fn with_csr32<F: FnOnce(Csr32) -> Csr32>(&self, f: F) -> &Self {
        self.csr32_reg().with(f);
        self
    }

    #[doc="Get the CSR33 Register."]
    #[inline] pub fn csr33_reg(&self) -> ::bobbin_mcu::register::Register<Csr33> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr33, 0x17c)
    }

    #[doc="Get the *mut pointer for the CSR33 register."]
    #[inline] pub fn csr33_mut(&self) -> *mut Csr33 { 
        self.csr33_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR33 register."]
    #[inline] pub fn csr33_ptr(&self) -> *const Csr33 { 
        self.csr33_reg().ptr()
    }

    #[doc="Read the CSR33 register."]
    #[inline] pub fn csr33(&self) -> Csr33 { 
        self.csr33_reg().read()
    }

    #[doc="Write the CSR33 register."]
    #[inline] pub fn write_csr33(&self, value: Csr33) -> &Self { 
        self.csr33_reg().write(value);
        self
    }

    #[doc="Set the CSR33 register."]
    #[inline] pub fn set_csr33<F: FnOnce(Csr33) -> Csr33>(&self, f: F) -> &Self {
        self.csr33_reg().set(f);
        self
    }

    #[doc="Modify the CSR33 register."]
    #[inline] pub fn with_csr33<F: FnOnce(Csr33) -> Csr33>(&self, f: F) -> &Self {
        self.csr33_reg().with(f);
        self
    }

    #[doc="Get the CSR34 Register."]
    #[inline] pub fn csr34_reg(&self) -> ::bobbin_mcu::register::Register<Csr34> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr34, 0x180)
    }

    #[doc="Get the *mut pointer for the CSR34 register."]
    #[inline] pub fn csr34_mut(&self) -> *mut Csr34 { 
        self.csr34_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR34 register."]
    #[inline] pub fn csr34_ptr(&self) -> *const Csr34 { 
        self.csr34_reg().ptr()
    }

    #[doc="Read the CSR34 register."]
    #[inline] pub fn csr34(&self) -> Csr34 { 
        self.csr34_reg().read()
    }

    #[doc="Write the CSR34 register."]
    #[inline] pub fn write_csr34(&self, value: Csr34) -> &Self { 
        self.csr34_reg().write(value);
        self
    }

    #[doc="Set the CSR34 register."]
    #[inline] pub fn set_csr34<F: FnOnce(Csr34) -> Csr34>(&self, f: F) -> &Self {
        self.csr34_reg().set(f);
        self
    }

    #[doc="Modify the CSR34 register."]
    #[inline] pub fn with_csr34<F: FnOnce(Csr34) -> Csr34>(&self, f: F) -> &Self {
        self.csr34_reg().with(f);
        self
    }

    #[doc="Get the CSR35 Register."]
    #[inline] pub fn csr35_reg(&self) -> ::bobbin_mcu::register::Register<Csr35> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr35, 0x184)
    }

    #[doc="Get the *mut pointer for the CSR35 register."]
    #[inline] pub fn csr35_mut(&self) -> *mut Csr35 { 
        self.csr35_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR35 register."]
    #[inline] pub fn csr35_ptr(&self) -> *const Csr35 { 
        self.csr35_reg().ptr()
    }

    #[doc="Read the CSR35 register."]
    #[inline] pub fn csr35(&self) -> Csr35 { 
        self.csr35_reg().read()
    }

    #[doc="Write the CSR35 register."]
    #[inline] pub fn write_csr35(&self, value: Csr35) -> &Self { 
        self.csr35_reg().write(value);
        self
    }

    #[doc="Set the CSR35 register."]
    #[inline] pub fn set_csr35<F: FnOnce(Csr35) -> Csr35>(&self, f: F) -> &Self {
        self.csr35_reg().set(f);
        self
    }

    #[doc="Modify the CSR35 register."]
    #[inline] pub fn with_csr35<F: FnOnce(Csr35) -> Csr35>(&self, f: F) -> &Self {
        self.csr35_reg().with(f);
        self
    }

    #[doc="Get the CSR36 Register."]
    #[inline] pub fn csr36_reg(&self) -> ::bobbin_mcu::register::Register<Csr36> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr36, 0x188)
    }

    #[doc="Get the *mut pointer for the CSR36 register."]
    #[inline] pub fn csr36_mut(&self) -> *mut Csr36 { 
        self.csr36_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR36 register."]
    #[inline] pub fn csr36_ptr(&self) -> *const Csr36 { 
        self.csr36_reg().ptr()
    }

    #[doc="Read the CSR36 register."]
    #[inline] pub fn csr36(&self) -> Csr36 { 
        self.csr36_reg().read()
    }

    #[doc="Write the CSR36 register."]
    #[inline] pub fn write_csr36(&self, value: Csr36) -> &Self { 
        self.csr36_reg().write(value);
        self
    }

    #[doc="Set the CSR36 register."]
    #[inline] pub fn set_csr36<F: FnOnce(Csr36) -> Csr36>(&self, f: F) -> &Self {
        self.csr36_reg().set(f);
        self
    }

    #[doc="Modify the CSR36 register."]
    #[inline] pub fn with_csr36<F: FnOnce(Csr36) -> Csr36>(&self, f: F) -> &Self {
        self.csr36_reg().with(f);
        self
    }

    #[doc="Get the CSR37 Register."]
    #[inline] pub fn csr37_reg(&self) -> ::bobbin_mcu::register::Register<Csr37> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr37, 0x18c)
    }

    #[doc="Get the *mut pointer for the CSR37 register."]
    #[inline] pub fn csr37_mut(&self) -> *mut Csr37 { 
        self.csr37_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR37 register."]
    #[inline] pub fn csr37_ptr(&self) -> *const Csr37 { 
        self.csr37_reg().ptr()
    }

    #[doc="Read the CSR37 register."]
    #[inline] pub fn csr37(&self) -> Csr37 { 
        self.csr37_reg().read()
    }

    #[doc="Write the CSR37 register."]
    #[inline] pub fn write_csr37(&self, value: Csr37) -> &Self { 
        self.csr37_reg().write(value);
        self
    }

    #[doc="Set the CSR37 register."]
    #[inline] pub fn set_csr37<F: FnOnce(Csr37) -> Csr37>(&self, f: F) -> &Self {
        self.csr37_reg().set(f);
        self
    }

    #[doc="Modify the CSR37 register."]
    #[inline] pub fn with_csr37<F: FnOnce(Csr37) -> Csr37>(&self, f: F) -> &Self {
        self.csr37_reg().with(f);
        self
    }

    #[doc="Get the CSR38 Register."]
    #[inline] pub fn csr38_reg(&self) -> ::bobbin_mcu::register::Register<Csr38> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr38, 0x190)
    }

    #[doc="Get the *mut pointer for the CSR38 register."]
    #[inline] pub fn csr38_mut(&self) -> *mut Csr38 { 
        self.csr38_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR38 register."]
    #[inline] pub fn csr38_ptr(&self) -> *const Csr38 { 
        self.csr38_reg().ptr()
    }

    #[doc="Read the CSR38 register."]
    #[inline] pub fn csr38(&self) -> Csr38 { 
        self.csr38_reg().read()
    }

    #[doc="Write the CSR38 register."]
    #[inline] pub fn write_csr38(&self, value: Csr38) -> &Self { 
        self.csr38_reg().write(value);
        self
    }

    #[doc="Set the CSR38 register."]
    #[inline] pub fn set_csr38<F: FnOnce(Csr38) -> Csr38>(&self, f: F) -> &Self {
        self.csr38_reg().set(f);
        self
    }

    #[doc="Modify the CSR38 register."]
    #[inline] pub fn with_csr38<F: FnOnce(Csr38) -> Csr38>(&self, f: F) -> &Self {
        self.csr38_reg().with(f);
        self
    }

    #[doc="Get the CSR39 Register."]
    #[inline] pub fn csr39_reg(&self) -> ::bobbin_mcu::register::Register<Csr39> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr39, 0x194)
    }

    #[doc="Get the *mut pointer for the CSR39 register."]
    #[inline] pub fn csr39_mut(&self) -> *mut Csr39 { 
        self.csr39_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR39 register."]
    #[inline] pub fn csr39_ptr(&self) -> *const Csr39 { 
        self.csr39_reg().ptr()
    }

    #[doc="Read the CSR39 register."]
    #[inline] pub fn csr39(&self) -> Csr39 { 
        self.csr39_reg().read()
    }

    #[doc="Write the CSR39 register."]
    #[inline] pub fn write_csr39(&self, value: Csr39) -> &Self { 
        self.csr39_reg().write(value);
        self
    }

    #[doc="Set the CSR39 register."]
    #[inline] pub fn set_csr39<F: FnOnce(Csr39) -> Csr39>(&self, f: F) -> &Self {
        self.csr39_reg().set(f);
        self
    }

    #[doc="Modify the CSR39 register."]
    #[inline] pub fn with_csr39<F: FnOnce(Csr39) -> Csr39>(&self, f: F) -> &Self {
        self.csr39_reg().with(f);
        self
    }

    #[doc="Get the CSR40 Register."]
    #[inline] pub fn csr40_reg(&self) -> ::bobbin_mcu::register::Register<Csr40> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr40, 0x198)
    }

    #[doc="Get the *mut pointer for the CSR40 register."]
    #[inline] pub fn csr40_mut(&self) -> *mut Csr40 { 
        self.csr40_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR40 register."]
    #[inline] pub fn csr40_ptr(&self) -> *const Csr40 { 
        self.csr40_reg().ptr()
    }

    #[doc="Read the CSR40 register."]
    #[inline] pub fn csr40(&self) -> Csr40 { 
        self.csr40_reg().read()
    }

    #[doc="Write the CSR40 register."]
    #[inline] pub fn write_csr40(&self, value: Csr40) -> &Self { 
        self.csr40_reg().write(value);
        self
    }

    #[doc="Set the CSR40 register."]
    #[inline] pub fn set_csr40<F: FnOnce(Csr40) -> Csr40>(&self, f: F) -> &Self {
        self.csr40_reg().set(f);
        self
    }

    #[doc="Modify the CSR40 register."]
    #[inline] pub fn with_csr40<F: FnOnce(Csr40) -> Csr40>(&self, f: F) -> &Self {
        self.csr40_reg().with(f);
        self
    }

    #[doc="Get the CSR41 Register."]
    #[inline] pub fn csr41_reg(&self) -> ::bobbin_mcu::register::Register<Csr41> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr41, 0x19c)
    }

    #[doc="Get the *mut pointer for the CSR41 register."]
    #[inline] pub fn csr41_mut(&self) -> *mut Csr41 { 
        self.csr41_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR41 register."]
    #[inline] pub fn csr41_ptr(&self) -> *const Csr41 { 
        self.csr41_reg().ptr()
    }

    #[doc="Read the CSR41 register."]
    #[inline] pub fn csr41(&self) -> Csr41 { 
        self.csr41_reg().read()
    }

    #[doc="Write the CSR41 register."]
    #[inline] pub fn write_csr41(&self, value: Csr41) -> &Self { 
        self.csr41_reg().write(value);
        self
    }

    #[doc="Set the CSR41 register."]
    #[inline] pub fn set_csr41<F: FnOnce(Csr41) -> Csr41>(&self, f: F) -> &Self {
        self.csr41_reg().set(f);
        self
    }

    #[doc="Modify the CSR41 register."]
    #[inline] pub fn with_csr41<F: FnOnce(Csr41) -> Csr41>(&self, f: F) -> &Self {
        self.csr41_reg().with(f);
        self
    }

    #[doc="Get the CSR42 Register."]
    #[inline] pub fn csr42_reg(&self) -> ::bobbin_mcu::register::Register<Csr42> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr42, 0x1a0)
    }

    #[doc="Get the *mut pointer for the CSR42 register."]
    #[inline] pub fn csr42_mut(&self) -> *mut Csr42 { 
        self.csr42_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR42 register."]
    #[inline] pub fn csr42_ptr(&self) -> *const Csr42 { 
        self.csr42_reg().ptr()
    }

    #[doc="Read the CSR42 register."]
    #[inline] pub fn csr42(&self) -> Csr42 { 
        self.csr42_reg().read()
    }

    #[doc="Write the CSR42 register."]
    #[inline] pub fn write_csr42(&self, value: Csr42) -> &Self { 
        self.csr42_reg().write(value);
        self
    }

    #[doc="Set the CSR42 register."]
    #[inline] pub fn set_csr42<F: FnOnce(Csr42) -> Csr42>(&self, f: F) -> &Self {
        self.csr42_reg().set(f);
        self
    }

    #[doc="Modify the CSR42 register."]
    #[inline] pub fn with_csr42<F: FnOnce(Csr42) -> Csr42>(&self, f: F) -> &Self {
        self.csr42_reg().with(f);
        self
    }

    #[doc="Get the CSR43 Register."]
    #[inline] pub fn csr43_reg(&self) -> ::bobbin_mcu::register::Register<Csr43> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr43, 0x1a4)
    }

    #[doc="Get the *mut pointer for the CSR43 register."]
    #[inline] pub fn csr43_mut(&self) -> *mut Csr43 { 
        self.csr43_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR43 register."]
    #[inline] pub fn csr43_ptr(&self) -> *const Csr43 { 
        self.csr43_reg().ptr()
    }

    #[doc="Read the CSR43 register."]
    #[inline] pub fn csr43(&self) -> Csr43 { 
        self.csr43_reg().read()
    }

    #[doc="Write the CSR43 register."]
    #[inline] pub fn write_csr43(&self, value: Csr43) -> &Self { 
        self.csr43_reg().write(value);
        self
    }

    #[doc="Set the CSR43 register."]
    #[inline] pub fn set_csr43<F: FnOnce(Csr43) -> Csr43>(&self, f: F) -> &Self {
        self.csr43_reg().set(f);
        self
    }

    #[doc="Modify the CSR43 register."]
    #[inline] pub fn with_csr43<F: FnOnce(Csr43) -> Csr43>(&self, f: F) -> &Self {
        self.csr43_reg().with(f);
        self
    }

    #[doc="Get the CSR44 Register."]
    #[inline] pub fn csr44_reg(&self) -> ::bobbin_mcu::register::Register<Csr44> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr44, 0x1a8)
    }

    #[doc="Get the *mut pointer for the CSR44 register."]
    #[inline] pub fn csr44_mut(&self) -> *mut Csr44 { 
        self.csr44_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR44 register."]
    #[inline] pub fn csr44_ptr(&self) -> *const Csr44 { 
        self.csr44_reg().ptr()
    }

    #[doc="Read the CSR44 register."]
    #[inline] pub fn csr44(&self) -> Csr44 { 
        self.csr44_reg().read()
    }

    #[doc="Write the CSR44 register."]
    #[inline] pub fn write_csr44(&self, value: Csr44) -> &Self { 
        self.csr44_reg().write(value);
        self
    }

    #[doc="Set the CSR44 register."]
    #[inline] pub fn set_csr44<F: FnOnce(Csr44) -> Csr44>(&self, f: F) -> &Self {
        self.csr44_reg().set(f);
        self
    }

    #[doc="Modify the CSR44 register."]
    #[inline] pub fn with_csr44<F: FnOnce(Csr44) -> Csr44>(&self, f: F) -> &Self {
        self.csr44_reg().with(f);
        self
    }

    #[doc="Get the CSR45 Register."]
    #[inline] pub fn csr45_reg(&self) -> ::bobbin_mcu::register::Register<Csr45> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr45, 0x1ac)
    }

    #[doc="Get the *mut pointer for the CSR45 register."]
    #[inline] pub fn csr45_mut(&self) -> *mut Csr45 { 
        self.csr45_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR45 register."]
    #[inline] pub fn csr45_ptr(&self) -> *const Csr45 { 
        self.csr45_reg().ptr()
    }

    #[doc="Read the CSR45 register."]
    #[inline] pub fn csr45(&self) -> Csr45 { 
        self.csr45_reg().read()
    }

    #[doc="Write the CSR45 register."]
    #[inline] pub fn write_csr45(&self, value: Csr45) -> &Self { 
        self.csr45_reg().write(value);
        self
    }

    #[doc="Set the CSR45 register."]
    #[inline] pub fn set_csr45<F: FnOnce(Csr45) -> Csr45>(&self, f: F) -> &Self {
        self.csr45_reg().set(f);
        self
    }

    #[doc="Modify the CSR45 register."]
    #[inline] pub fn with_csr45<F: FnOnce(Csr45) -> Csr45>(&self, f: F) -> &Self {
        self.csr45_reg().with(f);
        self
    }

    #[doc="Get the CSR46 Register."]
    #[inline] pub fn csr46_reg(&self) -> ::bobbin_mcu::register::Register<Csr46> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr46, 0x1b0)
    }

    #[doc="Get the *mut pointer for the CSR46 register."]
    #[inline] pub fn csr46_mut(&self) -> *mut Csr46 { 
        self.csr46_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR46 register."]
    #[inline] pub fn csr46_ptr(&self) -> *const Csr46 { 
        self.csr46_reg().ptr()
    }

    #[doc="Read the CSR46 register."]
    #[inline] pub fn csr46(&self) -> Csr46 { 
        self.csr46_reg().read()
    }

    #[doc="Write the CSR46 register."]
    #[inline] pub fn write_csr46(&self, value: Csr46) -> &Self { 
        self.csr46_reg().write(value);
        self
    }

    #[doc="Set the CSR46 register."]
    #[inline] pub fn set_csr46<F: FnOnce(Csr46) -> Csr46>(&self, f: F) -> &Self {
        self.csr46_reg().set(f);
        self
    }

    #[doc="Modify the CSR46 register."]
    #[inline] pub fn with_csr46<F: FnOnce(Csr46) -> Csr46>(&self, f: F) -> &Self {
        self.csr46_reg().with(f);
        self
    }

    #[doc="Get the CSR47 Register."]
    #[inline] pub fn csr47_reg(&self) -> ::bobbin_mcu::register::Register<Csr47> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr47, 0x1b4)
    }

    #[doc="Get the *mut pointer for the CSR47 register."]
    #[inline] pub fn csr47_mut(&self) -> *mut Csr47 { 
        self.csr47_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR47 register."]
    #[inline] pub fn csr47_ptr(&self) -> *const Csr47 { 
        self.csr47_reg().ptr()
    }

    #[doc="Read the CSR47 register."]
    #[inline] pub fn csr47(&self) -> Csr47 { 
        self.csr47_reg().read()
    }

    #[doc="Write the CSR47 register."]
    #[inline] pub fn write_csr47(&self, value: Csr47) -> &Self { 
        self.csr47_reg().write(value);
        self
    }

    #[doc="Set the CSR47 register."]
    #[inline] pub fn set_csr47<F: FnOnce(Csr47) -> Csr47>(&self, f: F) -> &Self {
        self.csr47_reg().set(f);
        self
    }

    #[doc="Modify the CSR47 register."]
    #[inline] pub fn with_csr47<F: FnOnce(Csr47) -> Csr47>(&self, f: F) -> &Self {
        self.csr47_reg().with(f);
        self
    }

    #[doc="Get the CSR48 Register."]
    #[inline] pub fn csr48_reg(&self) -> ::bobbin_mcu::register::Register<Csr48> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr48, 0x1b8)
    }

    #[doc="Get the *mut pointer for the CSR48 register."]
    #[inline] pub fn csr48_mut(&self) -> *mut Csr48 { 
        self.csr48_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR48 register."]
    #[inline] pub fn csr48_ptr(&self) -> *const Csr48 { 
        self.csr48_reg().ptr()
    }

    #[doc="Read the CSR48 register."]
    #[inline] pub fn csr48(&self) -> Csr48 { 
        self.csr48_reg().read()
    }

    #[doc="Write the CSR48 register."]
    #[inline] pub fn write_csr48(&self, value: Csr48) -> &Self { 
        self.csr48_reg().write(value);
        self
    }

    #[doc="Set the CSR48 register."]
    #[inline] pub fn set_csr48<F: FnOnce(Csr48) -> Csr48>(&self, f: F) -> &Self {
        self.csr48_reg().set(f);
        self
    }

    #[doc="Modify the CSR48 register."]
    #[inline] pub fn with_csr48<F: FnOnce(Csr48) -> Csr48>(&self, f: F) -> &Self {
        self.csr48_reg().with(f);
        self
    }

    #[doc="Get the CSR49 Register."]
    #[inline] pub fn csr49_reg(&self) -> ::bobbin_mcu::register::Register<Csr49> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr49, 0x1bc)
    }

    #[doc="Get the *mut pointer for the CSR49 register."]
    #[inline] pub fn csr49_mut(&self) -> *mut Csr49 { 
        self.csr49_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR49 register."]
    #[inline] pub fn csr49_ptr(&self) -> *const Csr49 { 
        self.csr49_reg().ptr()
    }

    #[doc="Read the CSR49 register."]
    #[inline] pub fn csr49(&self) -> Csr49 { 
        self.csr49_reg().read()
    }

    #[doc="Write the CSR49 register."]
    #[inline] pub fn write_csr49(&self, value: Csr49) -> &Self { 
        self.csr49_reg().write(value);
        self
    }

    #[doc="Set the CSR49 register."]
    #[inline] pub fn set_csr49<F: FnOnce(Csr49) -> Csr49>(&self, f: F) -> &Self {
        self.csr49_reg().set(f);
        self
    }

    #[doc="Modify the CSR49 register."]
    #[inline] pub fn with_csr49<F: FnOnce(Csr49) -> Csr49>(&self, f: F) -> &Self {
        self.csr49_reg().with(f);
        self
    }

    #[doc="Get the CSR50 Register."]
    #[inline] pub fn csr50_reg(&self) -> ::bobbin_mcu::register::Register<Csr50> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr50, 0x1c0)
    }

    #[doc="Get the *mut pointer for the CSR50 register."]
    #[inline] pub fn csr50_mut(&self) -> *mut Csr50 { 
        self.csr50_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR50 register."]
    #[inline] pub fn csr50_ptr(&self) -> *const Csr50 { 
        self.csr50_reg().ptr()
    }

    #[doc="Read the CSR50 register."]
    #[inline] pub fn csr50(&self) -> Csr50 { 
        self.csr50_reg().read()
    }

    #[doc="Write the CSR50 register."]
    #[inline] pub fn write_csr50(&self, value: Csr50) -> &Self { 
        self.csr50_reg().write(value);
        self
    }

    #[doc="Set the CSR50 register."]
    #[inline] pub fn set_csr50<F: FnOnce(Csr50) -> Csr50>(&self, f: F) -> &Self {
        self.csr50_reg().set(f);
        self
    }

    #[doc="Modify the CSR50 register."]
    #[inline] pub fn with_csr50<F: FnOnce(Csr50) -> Csr50>(&self, f: F) -> &Self {
        self.csr50_reg().with(f);
        self
    }

    #[doc="Get the CSR51 Register."]
    #[inline] pub fn csr51_reg(&self) -> ::bobbin_mcu::register::Register<Csr51> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr51, 0x1c4)
    }

    #[doc="Get the *mut pointer for the CSR51 register."]
    #[inline] pub fn csr51_mut(&self) -> *mut Csr51 { 
        self.csr51_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR51 register."]
    #[inline] pub fn csr51_ptr(&self) -> *const Csr51 { 
        self.csr51_reg().ptr()
    }

    #[doc="Read the CSR51 register."]
    #[inline] pub fn csr51(&self) -> Csr51 { 
        self.csr51_reg().read()
    }

    #[doc="Write the CSR51 register."]
    #[inline] pub fn write_csr51(&self, value: Csr51) -> &Self { 
        self.csr51_reg().write(value);
        self
    }

    #[doc="Set the CSR51 register."]
    #[inline] pub fn set_csr51<F: FnOnce(Csr51) -> Csr51>(&self, f: F) -> &Self {
        self.csr51_reg().set(f);
        self
    }

    #[doc="Modify the CSR51 register."]
    #[inline] pub fn with_csr51<F: FnOnce(Csr51) -> Csr51>(&self, f: F) -> &Self {
        self.csr51_reg().with(f);
        self
    }

    #[doc="Get the CSR52 Register."]
    #[inline] pub fn csr52_reg(&self) -> ::bobbin_mcu::register::Register<Csr52> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr52, 0x1c8)
    }

    #[doc="Get the *mut pointer for the CSR52 register."]
    #[inline] pub fn csr52_mut(&self) -> *mut Csr52 { 
        self.csr52_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR52 register."]
    #[inline] pub fn csr52_ptr(&self) -> *const Csr52 { 
        self.csr52_reg().ptr()
    }

    #[doc="Read the CSR52 register."]
    #[inline] pub fn csr52(&self) -> Csr52 { 
        self.csr52_reg().read()
    }

    #[doc="Write the CSR52 register."]
    #[inline] pub fn write_csr52(&self, value: Csr52) -> &Self { 
        self.csr52_reg().write(value);
        self
    }

    #[doc="Set the CSR52 register."]
    #[inline] pub fn set_csr52<F: FnOnce(Csr52) -> Csr52>(&self, f: F) -> &Self {
        self.csr52_reg().set(f);
        self
    }

    #[doc="Modify the CSR52 register."]
    #[inline] pub fn with_csr52<F: FnOnce(Csr52) -> Csr52>(&self, f: F) -> &Self {
        self.csr52_reg().with(f);
        self
    }

    #[doc="Get the CSR53 Register."]
    #[inline] pub fn csr53_reg(&self) -> ::bobbin_mcu::register::Register<Csr53> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr53, 0x1cc)
    }

    #[doc="Get the *mut pointer for the CSR53 register."]
    #[inline] pub fn csr53_mut(&self) -> *mut Csr53 { 
        self.csr53_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR53 register."]
    #[inline] pub fn csr53_ptr(&self) -> *const Csr53 { 
        self.csr53_reg().ptr()
    }

    #[doc="Read the CSR53 register."]
    #[inline] pub fn csr53(&self) -> Csr53 { 
        self.csr53_reg().read()
    }

    #[doc="Write the CSR53 register."]
    #[inline] pub fn write_csr53(&self, value: Csr53) -> &Self { 
        self.csr53_reg().write(value);
        self
    }

    #[doc="Set the CSR53 register."]
    #[inline] pub fn set_csr53<F: FnOnce(Csr53) -> Csr53>(&self, f: F) -> &Self {
        self.csr53_reg().set(f);
        self
    }

    #[doc="Modify the CSR53 register."]
    #[inline] pub fn with_csr53<F: FnOnce(Csr53) -> Csr53>(&self, f: F) -> &Self {
        self.csr53_reg().with(f);
        self
    }

    #[doc="Get the HASH_HR0 Register."]
    #[inline] pub fn hash_hr0_reg(&self) -> ::bobbin_mcu::register::Register<HashHr0> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut HashHr0, 0x310)
    }

    #[doc="Get the *mut pointer for the HASH_HR0 register."]
    #[inline] pub fn hash_hr0_mut(&self) -> *mut HashHr0 { 
        self.hash_hr0_reg().ptr()
    }

    #[doc="Get the *const pointer for the HASH_HR0 register."]
    #[inline] pub fn hash_hr0_ptr(&self) -> *const HashHr0 { 
        self.hash_hr0_reg().ptr()
    }

    #[doc="Read the HASH_HR0 register."]
    #[inline] pub fn hash_hr0(&self) -> HashHr0 { 
        self.hash_hr0_reg().read()
    }

    #[doc="Get the HASH_HR1 Register."]
    #[inline] pub fn hash_hr1_reg(&self) -> ::bobbin_mcu::register::Register<HashHr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut HashHr1, 0x314)
    }

    #[doc="Get the *mut pointer for the HASH_HR1 register."]
    #[inline] pub fn hash_hr1_mut(&self) -> *mut HashHr1 { 
        self.hash_hr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the HASH_HR1 register."]
    #[inline] pub fn hash_hr1_ptr(&self) -> *const HashHr1 { 
        self.hash_hr1_reg().ptr()
    }

    #[doc="Read the HASH_HR1 register."]
    #[inline] pub fn hash_hr1(&self) -> HashHr1 { 
        self.hash_hr1_reg().read()
    }

    #[doc="Get the HASH_HR2 Register."]
    #[inline] pub fn hash_hr2_reg(&self) -> ::bobbin_mcu::register::Register<HashHr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut HashHr2, 0x318)
    }

    #[doc="Get the *mut pointer for the HASH_HR2 register."]
    #[inline] pub fn hash_hr2_mut(&self) -> *mut HashHr2 { 
        self.hash_hr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the HASH_HR2 register."]
    #[inline] pub fn hash_hr2_ptr(&self) -> *const HashHr2 { 
        self.hash_hr2_reg().ptr()
    }

    #[doc="Read the HASH_HR2 register."]
    #[inline] pub fn hash_hr2(&self) -> HashHr2 { 
        self.hash_hr2_reg().read()
    }

    #[doc="Get the HASH_HR3 Register."]
    #[inline] pub fn hash_hr3_reg(&self) -> ::bobbin_mcu::register::Register<HashHr3> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut HashHr3, 0x31c)
    }

    #[doc="Get the *mut pointer for the HASH_HR3 register."]
    #[inline] pub fn hash_hr3_mut(&self) -> *mut HashHr3 { 
        self.hash_hr3_reg().ptr()
    }

    #[doc="Get the *const pointer for the HASH_HR3 register."]
    #[inline] pub fn hash_hr3_ptr(&self) -> *const HashHr3 { 
        self.hash_hr3_reg().ptr()
    }

    #[doc="Read the HASH_HR3 register."]
    #[inline] pub fn hash_hr3(&self) -> HashHr3 { 
        self.hash_hr3_reg().read()
    }

    #[doc="Get the HASH_HR4 Register."]
    #[inline] pub fn hash_hr4_reg(&self) -> ::bobbin_mcu::register::Register<HashHr4> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut HashHr4, 0x320)
    }

    #[doc="Get the *mut pointer for the HASH_HR4 register."]
    #[inline] pub fn hash_hr4_mut(&self) -> *mut HashHr4 { 
        self.hash_hr4_reg().ptr()
    }

    #[doc="Get the *const pointer for the HASH_HR4 register."]
    #[inline] pub fn hash_hr4_ptr(&self) -> *const HashHr4 { 
        self.hash_hr4_reg().ptr()
    }

    #[doc="Read the HASH_HR4 register."]
    #[inline] pub fn hash_hr4(&self) -> HashHr4 { 
        self.hash_hr4_reg().read()
    }

    #[doc="Get the HASH_HR5 Register."]
    #[inline] pub fn hash_hr5_reg(&self) -> ::bobbin_mcu::register::Register<HashHr5> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut HashHr5, 0x324)
    }

    #[doc="Get the *mut pointer for the HASH_HR5 register."]
    #[inline] pub fn hash_hr5_mut(&self) -> *mut HashHr5 { 
        self.hash_hr5_reg().ptr()
    }

    #[doc="Get the *const pointer for the HASH_HR5 register."]
    #[inline] pub fn hash_hr5_ptr(&self) -> *const HashHr5 { 
        self.hash_hr5_reg().ptr()
    }

    #[doc="Read the HASH_HR5 register."]
    #[inline] pub fn hash_hr5(&self) -> HashHr5 { 
        self.hash_hr5_reg().read()
    }

    #[doc="Get the HASH_HR6 Register."]
    #[inline] pub fn hash_hr6_reg(&self) -> ::bobbin_mcu::register::Register<HashHr6> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut HashHr6, 0x328)
    }

    #[doc="Get the *mut pointer for the HASH_HR6 register."]
    #[inline] pub fn hash_hr6_mut(&self) -> *mut HashHr6 { 
        self.hash_hr6_reg().ptr()
    }

    #[doc="Get the *const pointer for the HASH_HR6 register."]
    #[inline] pub fn hash_hr6_ptr(&self) -> *const HashHr6 { 
        self.hash_hr6_reg().ptr()
    }

    #[doc="Read the HASH_HR6 register."]
    #[inline] pub fn hash_hr6(&self) -> HashHr6 { 
        self.hash_hr6_reg().read()
    }

    #[doc="Get the HASH_HR7 Register."]
    #[inline] pub fn hash_hr7_reg(&self) -> ::bobbin_mcu::register::Register<HashHr7> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut HashHr7, 0x32c)
    }

    #[doc="Get the *mut pointer for the HASH_HR7 register."]
    #[inline] pub fn hash_hr7_mut(&self) -> *mut HashHr7 { 
        self.hash_hr7_reg().ptr()
    }

    #[doc="Get the *const pointer for the HASH_HR7 register."]
    #[inline] pub fn hash_hr7_ptr(&self) -> *const HashHr7 { 
        self.hash_hr7_reg().ptr()
    }

    #[doc="Read the HASH_HR7 register."]
    #[inline] pub fn hash_hr7(&self) -> HashHr7 { 
        self.hash_hr7_reg().read()
    }

}

#[doc="control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="Initialize message digest calculation"]
    #[inline] pub fn init(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if INIT != 0"]
    #[inline] pub fn test_init(&self) -> bool {
        self.init() != 0
    }

    #[doc="Sets the INIT field."]
    #[inline] pub fn set_init<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="DMA enable"]
    #[inline] pub fn dmae(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DMAE != 0"]
    #[inline] pub fn test_dmae(&self) -> bool {
        self.dmae() != 0
    }

    #[doc="Sets the DMAE field."]
    #[inline] pub fn set_dmae<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Data type selection"]
    #[inline] pub fn datatype(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if DATATYPE != 0"]
    #[inline] pub fn test_datatype(&self) -> bool {
        self.datatype() != 0
    }

    #[doc="Sets the DATATYPE field."]
    #[inline] pub fn set_datatype<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Mode selection"]
    #[inline] pub fn mode(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode(&self) -> bool {
        self.mode() != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Algorithm selection"]
    #[inline] pub fn algo0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ALGO0 != 0"]
    #[inline] pub fn test_algo0(&self) -> bool {
        self.algo0() != 0
    }

    #[doc="Sets the ALGO0 field."]
    #[inline] pub fn set_algo0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Number of words already pushed"]
    #[inline] pub fn nbw(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if NBW != 0"]
    #[inline] pub fn test_nbw(&self) -> bool {
        self.nbw() != 0
    }

    #[doc="Sets the NBW field."]
    #[inline] pub fn set_nbw<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DIN not empty"]
    #[inline] pub fn dinne(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if DINNE != 0"]
    #[inline] pub fn test_dinne(&self) -> bool {
        self.dinne() != 0
    }

    #[doc="Sets the DINNE field."]
    #[inline] pub fn set_dinne<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Multiple DMA Transfers"]
    #[inline] pub fn mdmat(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if MDMAT != 0"]
    #[inline] pub fn test_mdmat(&self) -> bool {
        self.mdmat() != 0
    }

    #[doc="Sets the MDMAT field."]
    #[inline] pub fn set_mdmat<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Long key selection"]
    #[inline] pub fn lkey(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if LKEY != 0"]
    #[inline] pub fn test_lkey(&self) -> bool {
        self.lkey() != 0
    }

    #[doc="Sets the LKEY field."]
    #[inline] pub fn set_lkey<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="ALGO"]
    #[inline] pub fn algo1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if ALGO1 != 0"]
    #[inline] pub fn test_algo1(&self) -> bool {
        self.algo1() != 0
    }

    #[doc="Sets the ALGO1 field."]
    #[inline] pub fn set_algo1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

}

impl From<u32> for Cr {
    #[inline]
    fn from(other: u32) -> Self {
         Cr(other)
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
        if self.init() != 0 { try!(write!(f, " init"))}
        if self.dmae() != 0 { try!(write!(f, " dmae"))}
        if self.datatype() != 0 { try!(write!(f, " datatype=0x{:x}", self.datatype()))}
        if self.mode() != 0 { try!(write!(f, " mode"))}
        if self.algo0() != 0 { try!(write!(f, " algo0"))}
        if self.nbw() != 0 { try!(write!(f, " nbw=0x{:x}", self.nbw()))}
        if self.dinne() != 0 { try!(write!(f, " dinne"))}
        if self.mdmat() != 0 { try!(write!(f, " mdmat"))}
        if self.lkey() != 0 { try!(write!(f, " lkey"))}
        if self.algo1() != 0 { try!(write!(f, " algo1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="data input register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Din(pub u32);
impl Din {
    #[doc="Data input"]
    #[inline] pub fn datain(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if DATAIN != 0"]
    #[inline] pub fn test_datain(&self) -> bool {
        self.datain() != 0
    }

    #[doc="Sets the DATAIN field."]
    #[inline] pub fn set_datain<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Din {
    #[inline]
    fn from(other: u32) -> Self {
         Din(other)
    }
}

impl ::core::fmt::Display for Din {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Din {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="start register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Str(pub u32);
impl Str {
    #[doc="Digest calculation"]
    #[inline] pub fn dcal(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if DCAL != 0"]
    #[inline] pub fn test_dcal(&self) -> bool {
        self.dcal() != 0
    }

    #[doc="Sets the DCAL field."]
    #[inline] pub fn set_dcal<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Number of valid bits in the last word of the message"]
    #[inline] pub fn nblw(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if NBLW != 0"]
    #[inline] pub fn test_nblw(&self) -> bool {
        self.nblw() != 0
    }

    #[doc="Sets the NBLW field."]
    #[inline] pub fn set_nblw<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Str {
    #[inline]
    fn from(other: u32) -> Self {
         Str(other)
    }
}

impl ::core::fmt::Display for Str {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Str {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dcal() != 0 { try!(write!(f, " dcal"))}
        if self.nblw() != 0 { try!(write!(f, " nblw=0x{:x}", self.nblw()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="digest registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hr0(pub u32);
impl Hr0 {
    #[doc="H0"]
    #[inline] pub fn h0(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if H0 != 0"]
    #[inline] pub fn test_h0(&self) -> bool {
        self.h0() != 0
    }

    #[doc="Sets the H0 field."]
    #[inline] pub fn set_h0<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Hr0 {
    #[inline]
    fn from(other: u32) -> Self {
         Hr0(other)
    }
}

impl ::core::fmt::Display for Hr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="digest registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hr1(pub u32);
impl Hr1 {
    #[doc="H1"]
    #[inline] pub fn h1(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if H1 != 0"]
    #[inline] pub fn test_h1(&self) -> bool {
        self.h1() != 0
    }

    #[doc="Sets the H1 field."]
    #[inline] pub fn set_h1<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Hr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Hr1(other)
    }
}

impl ::core::fmt::Display for Hr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="digest registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hr2(pub u32);
impl Hr2 {
    #[doc="H2"]
    #[inline] pub fn h2(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if H2 != 0"]
    #[inline] pub fn test_h2(&self) -> bool {
        self.h2() != 0
    }

    #[doc="Sets the H2 field."]
    #[inline] pub fn set_h2<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Hr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Hr2(other)
    }
}

impl ::core::fmt::Display for Hr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="digest registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hr3(pub u32);
impl Hr3 {
    #[doc="H3"]
    #[inline] pub fn h3(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if H3 != 0"]
    #[inline] pub fn test_h3(&self) -> bool {
        self.h3() != 0
    }

    #[doc="Sets the H3 field."]
    #[inline] pub fn set_h3<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Hr3 {
    #[inline]
    fn from(other: u32) -> Self {
         Hr3(other)
    }
}

impl ::core::fmt::Display for Hr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="digest registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hr4(pub u32);
impl Hr4 {
    #[doc="H4"]
    #[inline] pub fn h4(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if H4 != 0"]
    #[inline] pub fn test_h4(&self) -> bool {
        self.h4() != 0
    }

    #[doc="Sets the H4 field."]
    #[inline] pub fn set_h4<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Hr4 {
    #[inline]
    fn from(other: u32) -> Self {
         Hr4(other)
    }
}

impl ::core::fmt::Display for Hr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="interrupt enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Imr(pub u32);
impl Imr {
    #[doc="Digest calculation completion interrupt enable"]
    #[inline] pub fn dcie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DCIE != 0"]
    #[inline] pub fn test_dcie(&self) -> bool {
        self.dcie() != 0
    }

    #[doc="Sets the DCIE field."]
    #[inline] pub fn set_dcie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Data input interrupt enable"]
    #[inline] pub fn dinie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DINIE != 0"]
    #[inline] pub fn test_dinie(&self) -> bool {
        self.dinie() != 0
    }

    #[doc="Sets the DINIE field."]
    #[inline] pub fn set_dinie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Imr {
    #[inline]
    fn from(other: u32) -> Self {
         Imr(other)
    }
}

impl ::core::fmt::Display for Imr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Imr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dcie() != 0 { try!(write!(f, " dcie"))}
        if self.dinie() != 0 { try!(write!(f, " dinie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc="Busy bit"]
    #[inline] pub fn busy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if BUSY != 0"]
    #[inline] pub fn test_busy(&self) -> bool {
        self.busy() != 0
    }

    #[doc="Sets the BUSY field."]
    #[inline] pub fn set_busy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="DMA Status"]
    #[inline] pub fn dmas(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DMAS != 0"]
    #[inline] pub fn test_dmas(&self) -> bool {
        self.dmas() != 0
    }

    #[doc="Sets the DMAS field."]
    #[inline] pub fn set_dmas<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Digest calculation completion interrupt status"]
    #[inline] pub fn dcis(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DCIS != 0"]
    #[inline] pub fn test_dcis(&self) -> bool {
        self.dcis() != 0
    }

    #[doc="Sets the DCIS field."]
    #[inline] pub fn set_dcis<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Data input interrupt status"]
    #[inline] pub fn dinis(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DINIS != 0"]
    #[inline] pub fn test_dinis(&self) -> bool {
        self.dinis() != 0
    }

    #[doc="Sets the DINIS field."]
    #[inline] pub fn set_dinis<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sr {
    #[inline]
    fn from(other: u32) -> Self {
         Sr(other)
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
        if self.busy() != 0 { try!(write!(f, " busy"))}
        if self.dmas() != 0 { try!(write!(f, " dmas"))}
        if self.dcis() != 0 { try!(write!(f, " dcis"))}
        if self.dinis() != 0 { try!(write!(f, " dinis"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr0(pub u32);
impl Csr0 {
    #[doc="CSR0"]
    #[inline] pub fn csr0(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR0 != 0"]
    #[inline] pub fn test_csr0(&self) -> bool {
        self.csr0() != 0
    }

    #[doc="Sets the CSR0 field."]
    #[inline] pub fn set_csr0<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr0 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr0(other)
    }
}

impl ::core::fmt::Display for Csr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr1(pub u32);
impl Csr1 {
    #[doc="CSR1"]
    #[inline] pub fn csr1(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR1 != 0"]
    #[inline] pub fn test_csr1(&self) -> bool {
        self.csr1() != 0
    }

    #[doc="Sets the CSR1 field."]
    #[inline] pub fn set_csr1<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr1(other)
    }
}

impl ::core::fmt::Display for Csr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr2(pub u32);
impl Csr2 {
    #[doc="CSR2"]
    #[inline] pub fn csr2(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR2 != 0"]
    #[inline] pub fn test_csr2(&self) -> bool {
        self.csr2() != 0
    }

    #[doc="Sets the CSR2 field."]
    #[inline] pub fn set_csr2<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr2(other)
    }
}

impl ::core::fmt::Display for Csr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr3(pub u32);
impl Csr3 {
    #[doc="CSR3"]
    #[inline] pub fn csr3(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR3 != 0"]
    #[inline] pub fn test_csr3(&self) -> bool {
        self.csr3() != 0
    }

    #[doc="Sets the CSR3 field."]
    #[inline] pub fn set_csr3<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr3 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr3(other)
    }
}

impl ::core::fmt::Display for Csr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr4(pub u32);
impl Csr4 {
    #[doc="CSR4"]
    #[inline] pub fn csr4(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR4 != 0"]
    #[inline] pub fn test_csr4(&self) -> bool {
        self.csr4() != 0
    }

    #[doc="Sets the CSR4 field."]
    #[inline] pub fn set_csr4<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr4 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr4(other)
    }
}

impl ::core::fmt::Display for Csr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr5(pub u32);
impl Csr5 {
    #[doc="CSR5"]
    #[inline] pub fn csr5(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR5 != 0"]
    #[inline] pub fn test_csr5(&self) -> bool {
        self.csr5() != 0
    }

    #[doc="Sets the CSR5 field."]
    #[inline] pub fn set_csr5<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr5 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr5(other)
    }
}

impl ::core::fmt::Display for Csr5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr6(pub u32);
impl Csr6 {
    #[doc="CSR6"]
    #[inline] pub fn csr6(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR6 != 0"]
    #[inline] pub fn test_csr6(&self) -> bool {
        self.csr6() != 0
    }

    #[doc="Sets the CSR6 field."]
    #[inline] pub fn set_csr6<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr6 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr6(other)
    }
}

impl ::core::fmt::Display for Csr6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr7(pub u32);
impl Csr7 {
    #[doc="CSR7"]
    #[inline] pub fn csr7(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR7 != 0"]
    #[inline] pub fn test_csr7(&self) -> bool {
        self.csr7() != 0
    }

    #[doc="Sets the CSR7 field."]
    #[inline] pub fn set_csr7<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr7 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr7(other)
    }
}

impl ::core::fmt::Display for Csr7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr8(pub u32);
impl Csr8 {
    #[doc="CSR8"]
    #[inline] pub fn csr8(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR8 != 0"]
    #[inline] pub fn test_csr8(&self) -> bool {
        self.csr8() != 0
    }

    #[doc="Sets the CSR8 field."]
    #[inline] pub fn set_csr8<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr8 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr8(other)
    }
}

impl ::core::fmt::Display for Csr8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr9(pub u32);
impl Csr9 {
    #[doc="CSR9"]
    #[inline] pub fn csr9(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR9 != 0"]
    #[inline] pub fn test_csr9(&self) -> bool {
        self.csr9() != 0
    }

    #[doc="Sets the CSR9 field."]
    #[inline] pub fn set_csr9<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr9 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr9(other)
    }
}

impl ::core::fmt::Display for Csr9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr10(pub u32);
impl Csr10 {
    #[doc="CSR10"]
    #[inline] pub fn csr10(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR10 != 0"]
    #[inline] pub fn test_csr10(&self) -> bool {
        self.csr10() != 0
    }

    #[doc="Sets the CSR10 field."]
    #[inline] pub fn set_csr10<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr10 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr10(other)
    }
}

impl ::core::fmt::Display for Csr10 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr10 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr11(pub u32);
impl Csr11 {
    #[doc="CSR11"]
    #[inline] pub fn csr11(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR11 != 0"]
    #[inline] pub fn test_csr11(&self) -> bool {
        self.csr11() != 0
    }

    #[doc="Sets the CSR11 field."]
    #[inline] pub fn set_csr11<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr11 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr11(other)
    }
}

impl ::core::fmt::Display for Csr11 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr11 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr12(pub u32);
impl Csr12 {
    #[doc="CSR12"]
    #[inline] pub fn csr12(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR12 != 0"]
    #[inline] pub fn test_csr12(&self) -> bool {
        self.csr12() != 0
    }

    #[doc="Sets the CSR12 field."]
    #[inline] pub fn set_csr12<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr12 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr12(other)
    }
}

impl ::core::fmt::Display for Csr12 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr12 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr13(pub u32);
impl Csr13 {
    #[doc="CSR13"]
    #[inline] pub fn csr13(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR13 != 0"]
    #[inline] pub fn test_csr13(&self) -> bool {
        self.csr13() != 0
    }

    #[doc="Sets the CSR13 field."]
    #[inline] pub fn set_csr13<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr13 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr13(other)
    }
}

impl ::core::fmt::Display for Csr13 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr13 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr14(pub u32);
impl Csr14 {
    #[doc="CSR14"]
    #[inline] pub fn csr14(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR14 != 0"]
    #[inline] pub fn test_csr14(&self) -> bool {
        self.csr14() != 0
    }

    #[doc="Sets the CSR14 field."]
    #[inline] pub fn set_csr14<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr14 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr14(other)
    }
}

impl ::core::fmt::Display for Csr14 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr14 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr15(pub u32);
impl Csr15 {
    #[doc="CSR15"]
    #[inline] pub fn csr15(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR15 != 0"]
    #[inline] pub fn test_csr15(&self) -> bool {
        self.csr15() != 0
    }

    #[doc="Sets the CSR15 field."]
    #[inline] pub fn set_csr15<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr15 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr15(other)
    }
}

impl ::core::fmt::Display for Csr15 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr15 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr16(pub u32);
impl Csr16 {
    #[doc="CSR16"]
    #[inline] pub fn csr16(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR16 != 0"]
    #[inline] pub fn test_csr16(&self) -> bool {
        self.csr16() != 0
    }

    #[doc="Sets the CSR16 field."]
    #[inline] pub fn set_csr16<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr16 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr16(other)
    }
}

impl ::core::fmt::Display for Csr16 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr16 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr17(pub u32);
impl Csr17 {
    #[doc="CSR17"]
    #[inline] pub fn csr17(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR17 != 0"]
    #[inline] pub fn test_csr17(&self) -> bool {
        self.csr17() != 0
    }

    #[doc="Sets the CSR17 field."]
    #[inline] pub fn set_csr17<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr17 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr17(other)
    }
}

impl ::core::fmt::Display for Csr17 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr17 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr18(pub u32);
impl Csr18 {
    #[doc="CSR18"]
    #[inline] pub fn csr18(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR18 != 0"]
    #[inline] pub fn test_csr18(&self) -> bool {
        self.csr18() != 0
    }

    #[doc="Sets the CSR18 field."]
    #[inline] pub fn set_csr18<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr18 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr18(other)
    }
}

impl ::core::fmt::Display for Csr18 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr18 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr19(pub u32);
impl Csr19 {
    #[doc="CSR19"]
    #[inline] pub fn csr19(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR19 != 0"]
    #[inline] pub fn test_csr19(&self) -> bool {
        self.csr19() != 0
    }

    #[doc="Sets the CSR19 field."]
    #[inline] pub fn set_csr19<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr19 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr19(other)
    }
}

impl ::core::fmt::Display for Csr19 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr19 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr20(pub u32);
impl Csr20 {
    #[doc="CSR20"]
    #[inline] pub fn csr20(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR20 != 0"]
    #[inline] pub fn test_csr20(&self) -> bool {
        self.csr20() != 0
    }

    #[doc="Sets the CSR20 field."]
    #[inline] pub fn set_csr20<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr20 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr20(other)
    }
}

impl ::core::fmt::Display for Csr20 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr20 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr21(pub u32);
impl Csr21 {
    #[doc="CSR21"]
    #[inline] pub fn csr21(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR21 != 0"]
    #[inline] pub fn test_csr21(&self) -> bool {
        self.csr21() != 0
    }

    #[doc="Sets the CSR21 field."]
    #[inline] pub fn set_csr21<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr21 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr21(other)
    }
}

impl ::core::fmt::Display for Csr21 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr21 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr22(pub u32);
impl Csr22 {
    #[doc="CSR22"]
    #[inline] pub fn csr22(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR22 != 0"]
    #[inline] pub fn test_csr22(&self) -> bool {
        self.csr22() != 0
    }

    #[doc="Sets the CSR22 field."]
    #[inline] pub fn set_csr22<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr22 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr22(other)
    }
}

impl ::core::fmt::Display for Csr22 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr22 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr23(pub u32);
impl Csr23 {
    #[doc="CSR23"]
    #[inline] pub fn csr23(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR23 != 0"]
    #[inline] pub fn test_csr23(&self) -> bool {
        self.csr23() != 0
    }

    #[doc="Sets the CSR23 field."]
    #[inline] pub fn set_csr23<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr23 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr23(other)
    }
}

impl ::core::fmt::Display for Csr23 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr23 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr24(pub u32);
impl Csr24 {
    #[doc="CSR24"]
    #[inline] pub fn csr24(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR24 != 0"]
    #[inline] pub fn test_csr24(&self) -> bool {
        self.csr24() != 0
    }

    #[doc="Sets the CSR24 field."]
    #[inline] pub fn set_csr24<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr24 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr24(other)
    }
}

impl ::core::fmt::Display for Csr24 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr24 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr25(pub u32);
impl Csr25 {
    #[doc="CSR25"]
    #[inline] pub fn csr25(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR25 != 0"]
    #[inline] pub fn test_csr25(&self) -> bool {
        self.csr25() != 0
    }

    #[doc="Sets the CSR25 field."]
    #[inline] pub fn set_csr25<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr25 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr25(other)
    }
}

impl ::core::fmt::Display for Csr25 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr25 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr26(pub u32);
impl Csr26 {
    #[doc="CSR26"]
    #[inline] pub fn csr26(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR26 != 0"]
    #[inline] pub fn test_csr26(&self) -> bool {
        self.csr26() != 0
    }

    #[doc="Sets the CSR26 field."]
    #[inline] pub fn set_csr26<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr26 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr26(other)
    }
}

impl ::core::fmt::Display for Csr26 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr26 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr27(pub u32);
impl Csr27 {
    #[doc="CSR27"]
    #[inline] pub fn csr27(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR27 != 0"]
    #[inline] pub fn test_csr27(&self) -> bool {
        self.csr27() != 0
    }

    #[doc="Sets the CSR27 field."]
    #[inline] pub fn set_csr27<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr27 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr27(other)
    }
}

impl ::core::fmt::Display for Csr27 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr27 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr28(pub u32);
impl Csr28 {
    #[doc="CSR28"]
    #[inline] pub fn csr28(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR28 != 0"]
    #[inline] pub fn test_csr28(&self) -> bool {
        self.csr28() != 0
    }

    #[doc="Sets the CSR28 field."]
    #[inline] pub fn set_csr28<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr28 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr28(other)
    }
}

impl ::core::fmt::Display for Csr28 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr28 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr29(pub u32);
impl Csr29 {
    #[doc="CSR29"]
    #[inline] pub fn csr29(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR29 != 0"]
    #[inline] pub fn test_csr29(&self) -> bool {
        self.csr29() != 0
    }

    #[doc="Sets the CSR29 field."]
    #[inline] pub fn set_csr29<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr29 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr29(other)
    }
}

impl ::core::fmt::Display for Csr29 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr29 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr30(pub u32);
impl Csr30 {
    #[doc="CSR30"]
    #[inline] pub fn csr30(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR30 != 0"]
    #[inline] pub fn test_csr30(&self) -> bool {
        self.csr30() != 0
    }

    #[doc="Sets the CSR30 field."]
    #[inline] pub fn set_csr30<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr30 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr30(other)
    }
}

impl ::core::fmt::Display for Csr30 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr30 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr31(pub u32);
impl Csr31 {
    #[doc="CSR31"]
    #[inline] pub fn csr31(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR31 != 0"]
    #[inline] pub fn test_csr31(&self) -> bool {
        self.csr31() != 0
    }

    #[doc="Sets the CSR31 field."]
    #[inline] pub fn set_csr31<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr31 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr31(other)
    }
}

impl ::core::fmt::Display for Csr31 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr31 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr32(pub u32);
impl Csr32 {
    #[doc="CSR32"]
    #[inline] pub fn csr32(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR32 != 0"]
    #[inline] pub fn test_csr32(&self) -> bool {
        self.csr32() != 0
    }

    #[doc="Sets the CSR32 field."]
    #[inline] pub fn set_csr32<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr32 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr32(other)
    }
}

impl ::core::fmt::Display for Csr32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr33(pub u32);
impl Csr33 {
    #[doc="CSR33"]
    #[inline] pub fn csr33(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR33 != 0"]
    #[inline] pub fn test_csr33(&self) -> bool {
        self.csr33() != 0
    }

    #[doc="Sets the CSR33 field."]
    #[inline] pub fn set_csr33<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr33 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr33(other)
    }
}

impl ::core::fmt::Display for Csr33 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr33 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr34(pub u32);
impl Csr34 {
    #[doc="CSR34"]
    #[inline] pub fn csr34(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR34 != 0"]
    #[inline] pub fn test_csr34(&self) -> bool {
        self.csr34() != 0
    }

    #[doc="Sets the CSR34 field."]
    #[inline] pub fn set_csr34<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr34 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr34(other)
    }
}

impl ::core::fmt::Display for Csr34 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr34 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr35(pub u32);
impl Csr35 {
    #[doc="CSR35"]
    #[inline] pub fn csr35(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR35 != 0"]
    #[inline] pub fn test_csr35(&self) -> bool {
        self.csr35() != 0
    }

    #[doc="Sets the CSR35 field."]
    #[inline] pub fn set_csr35<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr35 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr35(other)
    }
}

impl ::core::fmt::Display for Csr35 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr35 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr36(pub u32);
impl Csr36 {
    #[doc="CSR36"]
    #[inline] pub fn csr36(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR36 != 0"]
    #[inline] pub fn test_csr36(&self) -> bool {
        self.csr36() != 0
    }

    #[doc="Sets the CSR36 field."]
    #[inline] pub fn set_csr36<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr36 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr36(other)
    }
}

impl ::core::fmt::Display for Csr36 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr36 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr37(pub u32);
impl Csr37 {
    #[doc="CSR37"]
    #[inline] pub fn csr37(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR37 != 0"]
    #[inline] pub fn test_csr37(&self) -> bool {
        self.csr37() != 0
    }

    #[doc="Sets the CSR37 field."]
    #[inline] pub fn set_csr37<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr37 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr37(other)
    }
}

impl ::core::fmt::Display for Csr37 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr37 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr38(pub u32);
impl Csr38 {
    #[doc="CSR38"]
    #[inline] pub fn csr38(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR38 != 0"]
    #[inline] pub fn test_csr38(&self) -> bool {
        self.csr38() != 0
    }

    #[doc="Sets the CSR38 field."]
    #[inline] pub fn set_csr38<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr38 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr38(other)
    }
}

impl ::core::fmt::Display for Csr38 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr38 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr39(pub u32);
impl Csr39 {
    #[doc="CSR39"]
    #[inline] pub fn csr39(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR39 != 0"]
    #[inline] pub fn test_csr39(&self) -> bool {
        self.csr39() != 0
    }

    #[doc="Sets the CSR39 field."]
    #[inline] pub fn set_csr39<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr39 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr39(other)
    }
}

impl ::core::fmt::Display for Csr39 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr39 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr40(pub u32);
impl Csr40 {
    #[doc="CSR40"]
    #[inline] pub fn csr40(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR40 != 0"]
    #[inline] pub fn test_csr40(&self) -> bool {
        self.csr40() != 0
    }

    #[doc="Sets the CSR40 field."]
    #[inline] pub fn set_csr40<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr40 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr40(other)
    }
}

impl ::core::fmt::Display for Csr40 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr40 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr41(pub u32);
impl Csr41 {
    #[doc="CSR41"]
    #[inline] pub fn csr41(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR41 != 0"]
    #[inline] pub fn test_csr41(&self) -> bool {
        self.csr41() != 0
    }

    #[doc="Sets the CSR41 field."]
    #[inline] pub fn set_csr41<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr41 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr41(other)
    }
}

impl ::core::fmt::Display for Csr41 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr41 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr42(pub u32);
impl Csr42 {
    #[doc="CSR42"]
    #[inline] pub fn csr42(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR42 != 0"]
    #[inline] pub fn test_csr42(&self) -> bool {
        self.csr42() != 0
    }

    #[doc="Sets the CSR42 field."]
    #[inline] pub fn set_csr42<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr42 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr42(other)
    }
}

impl ::core::fmt::Display for Csr42 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr42 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr43(pub u32);
impl Csr43 {
    #[doc="CSR43"]
    #[inline] pub fn csr43(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR43 != 0"]
    #[inline] pub fn test_csr43(&self) -> bool {
        self.csr43() != 0
    }

    #[doc="Sets the CSR43 field."]
    #[inline] pub fn set_csr43<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr43 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr43(other)
    }
}

impl ::core::fmt::Display for Csr43 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr43 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr44(pub u32);
impl Csr44 {
    #[doc="CSR44"]
    #[inline] pub fn csr44(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR44 != 0"]
    #[inline] pub fn test_csr44(&self) -> bool {
        self.csr44() != 0
    }

    #[doc="Sets the CSR44 field."]
    #[inline] pub fn set_csr44<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr44 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr44(other)
    }
}

impl ::core::fmt::Display for Csr44 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr44 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr45(pub u32);
impl Csr45 {
    #[doc="CSR45"]
    #[inline] pub fn csr45(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR45 != 0"]
    #[inline] pub fn test_csr45(&self) -> bool {
        self.csr45() != 0
    }

    #[doc="Sets the CSR45 field."]
    #[inline] pub fn set_csr45<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr45 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr45(other)
    }
}

impl ::core::fmt::Display for Csr45 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr45 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr46(pub u32);
impl Csr46 {
    #[doc="CSR46"]
    #[inline] pub fn csr46(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR46 != 0"]
    #[inline] pub fn test_csr46(&self) -> bool {
        self.csr46() != 0
    }

    #[doc="Sets the CSR46 field."]
    #[inline] pub fn set_csr46<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr46 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr46(other)
    }
}

impl ::core::fmt::Display for Csr46 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr46 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr47(pub u32);
impl Csr47 {
    #[doc="CSR47"]
    #[inline] pub fn csr47(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR47 != 0"]
    #[inline] pub fn test_csr47(&self) -> bool {
        self.csr47() != 0
    }

    #[doc="Sets the CSR47 field."]
    #[inline] pub fn set_csr47<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr47 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr47(other)
    }
}

impl ::core::fmt::Display for Csr47 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr47 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr48(pub u32);
impl Csr48 {
    #[doc="CSR48"]
    #[inline] pub fn csr48(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR48 != 0"]
    #[inline] pub fn test_csr48(&self) -> bool {
        self.csr48() != 0
    }

    #[doc="Sets the CSR48 field."]
    #[inline] pub fn set_csr48<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr48 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr48(other)
    }
}

impl ::core::fmt::Display for Csr48 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr48 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr49(pub u32);
impl Csr49 {
    #[doc="CSR49"]
    #[inline] pub fn csr49(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR49 != 0"]
    #[inline] pub fn test_csr49(&self) -> bool {
        self.csr49() != 0
    }

    #[doc="Sets the CSR49 field."]
    #[inline] pub fn set_csr49<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr49 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr49(other)
    }
}

impl ::core::fmt::Display for Csr49 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr49 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr50(pub u32);
impl Csr50 {
    #[doc="CSR50"]
    #[inline] pub fn csr50(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR50 != 0"]
    #[inline] pub fn test_csr50(&self) -> bool {
        self.csr50() != 0
    }

    #[doc="Sets the CSR50 field."]
    #[inline] pub fn set_csr50<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr50 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr50(other)
    }
}

impl ::core::fmt::Display for Csr50 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr50 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr51(pub u32);
impl Csr51 {
    #[doc="CSR51"]
    #[inline] pub fn csr51(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR51 != 0"]
    #[inline] pub fn test_csr51(&self) -> bool {
        self.csr51() != 0
    }

    #[doc="Sets the CSR51 field."]
    #[inline] pub fn set_csr51<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr51 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr51(other)
    }
}

impl ::core::fmt::Display for Csr51 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr51 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr52(pub u32);
impl Csr52 {
    #[doc="CSR52"]
    #[inline] pub fn csr52(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR52 != 0"]
    #[inline] pub fn test_csr52(&self) -> bool {
        self.csr52() != 0
    }

    #[doc="Sets the CSR52 field."]
    #[inline] pub fn set_csr52<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr52 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr52(other)
    }
}

impl ::core::fmt::Display for Csr52 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr52 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr53(pub u32);
impl Csr53 {
    #[doc="CSR53"]
    #[inline] pub fn csr53(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR53 != 0"]
    #[inline] pub fn test_csr53(&self) -> bool {
        self.csr53() != 0
    }

    #[doc="Sets the CSR53 field."]
    #[inline] pub fn set_csr53<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr53 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr53(other)
    }
}

impl ::core::fmt::Display for Csr53 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr53 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="HASH digest register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct HashHr0(pub u32);
impl HashHr0 {
    #[doc="H0"]
    #[inline] pub fn h0(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if H0 != 0"]
    #[inline] pub fn test_h0(&self) -> bool {
        self.h0() != 0
    }

    #[doc="Sets the H0 field."]
    #[inline] pub fn set_h0<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for HashHr0 {
    #[inline]
    fn from(other: u32) -> Self {
         HashHr0(other)
    }
}

impl ::core::fmt::Display for HashHr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for HashHr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="read-only"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct HashHr1(pub u32);
impl HashHr1 {
    #[doc="H1"]
    #[inline] pub fn h1(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if H1 != 0"]
    #[inline] pub fn test_h1(&self) -> bool {
        self.h1() != 0
    }

    #[doc="Sets the H1 field."]
    #[inline] pub fn set_h1<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for HashHr1 {
    #[inline]
    fn from(other: u32) -> Self {
         HashHr1(other)
    }
}

impl ::core::fmt::Display for HashHr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for HashHr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="read-only"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct HashHr2(pub u32);
impl HashHr2 {
    #[doc="H2"]
    #[inline] pub fn h2(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if H2 != 0"]
    #[inline] pub fn test_h2(&self) -> bool {
        self.h2() != 0
    }

    #[doc="Sets the H2 field."]
    #[inline] pub fn set_h2<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for HashHr2 {
    #[inline]
    fn from(other: u32) -> Self {
         HashHr2(other)
    }
}

impl ::core::fmt::Display for HashHr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for HashHr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="read-only"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct HashHr3(pub u32);
impl HashHr3 {
    #[doc="H3"]
    #[inline] pub fn h3(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if H3 != 0"]
    #[inline] pub fn test_h3(&self) -> bool {
        self.h3() != 0
    }

    #[doc="Sets the H3 field."]
    #[inline] pub fn set_h3<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for HashHr3 {
    #[inline]
    fn from(other: u32) -> Self {
         HashHr3(other)
    }
}

impl ::core::fmt::Display for HashHr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for HashHr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="read-only"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct HashHr4(pub u32);
impl HashHr4 {
    #[doc="H4"]
    #[inline] pub fn h4(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if H4 != 0"]
    #[inline] pub fn test_h4(&self) -> bool {
        self.h4() != 0
    }

    #[doc="Sets the H4 field."]
    #[inline] pub fn set_h4<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for HashHr4 {
    #[inline]
    fn from(other: u32) -> Self {
         HashHr4(other)
    }
}

impl ::core::fmt::Display for HashHr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for HashHr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="read-only"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct HashHr5(pub u32);
impl HashHr5 {
    #[doc="H5"]
    #[inline] pub fn h5(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if H5 != 0"]
    #[inline] pub fn test_h5(&self) -> bool {
        self.h5() != 0
    }

    #[doc="Sets the H5 field."]
    #[inline] pub fn set_h5<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for HashHr5 {
    #[inline]
    fn from(other: u32) -> Self {
         HashHr5(other)
    }
}

impl ::core::fmt::Display for HashHr5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for HashHr5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="read-only"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct HashHr6(pub u32);
impl HashHr6 {
    #[doc="H6"]
    #[inline] pub fn h6(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if H6 != 0"]
    #[inline] pub fn test_h6(&self) -> bool {
        self.h6() != 0
    }

    #[doc="Sets the H6 field."]
    #[inline] pub fn set_h6<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for HashHr6 {
    #[inline]
    fn from(other: u32) -> Self {
         HashHr6(other)
    }
}

impl ::core::fmt::Display for HashHr6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for HashHr6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="read-only"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct HashHr7(pub u32);
impl HashHr7 {
    #[doc="H7"]
    #[inline] pub fn h7(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if H7 != 0"]
    #[inline] pub fn test_h7(&self) -> bool {
        self.h7() != 0
    }

    #[doc="Sets the H7 field."]
    #[inline] pub fn set_h7<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for HashHr7 {
    #[inline]
    fn from(other: u32) -> Self {
         HashHr7(other)
    }
}

impl ::core::fmt::Display for HashHr7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for HashHr7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

