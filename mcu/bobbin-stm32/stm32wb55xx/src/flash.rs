
::bobbin_mcu::periph!( FLASH, Flash, FLASH_PERIPH, FlashPeriph, FLASH_OWNED, FLASH_REF_COUNT, 0x58004000, 0x00, 0x00);


// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("AHB3RSTR"), field: Some("FLASHRST"), description: None }
impl ::bobbin_mcu::gate::GateRst for Flash {
    #[inline]
    fn gate_rst(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahb3rstr().flashrst() }
    #[inline]
    fn set_gate_rst<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb3rstr(|r| r.set_flashrst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("AHB3ENR"), field: Some("FLASHEN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Flash {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahb3enr().flashen() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb3enr(|r| r.set_flashen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("AHB3SMENR"), field: Some("FLASHSMEN"), description: None }
impl ::bobbin_mcu::gate::GateSleepEn for Flash {
    #[inline]
    fn gate_sleep_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahb3smenr().flashsmen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb3smenr(|r| r.set_flashsmen(value));
        self
    }
}

#[doc="Flash"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FlashPeriph(pub usize);
impl FlashPeriph {
    #[doc="Get the ACR Register."]
    #[inline] pub fn acr_reg(&self) -> ::bobbin_mcu::register::Register<Acr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Acr, 0x0)
    }

    #[doc="Get the *mut pointer for the ACR register."]
    #[inline] pub fn acr_mut(&self) -> *mut Acr { 
        self.acr_reg().ptr()
    }

    #[doc="Get the *const pointer for the ACR register."]
    #[inline] pub fn acr_ptr(&self) -> *const Acr { 
        self.acr_reg().ptr()
    }

    #[doc="Read the ACR register."]
    #[inline] pub fn acr(&self) -> Acr { 
        self.acr_reg().read()
    }

    #[doc="Write the ACR register."]
    #[inline] pub fn write_acr(&self, value: Acr) -> &Self { 
        self.acr_reg().write(value);
        self
    }

    #[doc="Set the ACR register."]
    #[inline] pub fn set_acr<F: FnOnce(Acr) -> Acr>(&self, f: F) -> &Self {
        self.acr_reg().set(f);
        self
    }

    #[doc="Modify the ACR register."]
    #[inline] pub fn with_acr<F: FnOnce(Acr) -> Acr>(&self, f: F) -> &Self {
        self.acr_reg().with(f);
        self
    }

    #[doc="Get the KEYR Register."]
    #[inline] pub fn keyr_reg(&self) -> ::bobbin_mcu::register::Register<Keyr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Keyr, 0x8)
    }

    #[doc="Get the *mut pointer for the KEYR register."]
    #[inline] pub fn keyr_mut(&self) -> *mut Keyr { 
        self.keyr_reg().ptr()
    }

    #[doc="Get the *const pointer for the KEYR register."]
    #[inline] pub fn keyr_ptr(&self) -> *const Keyr { 
        self.keyr_reg().ptr()
    }

    #[doc="Write the KEYR register."]
    #[inline] pub fn write_keyr(&self, value: Keyr) -> &Self { 
        self.keyr_reg().write(value);
        self
    }

    #[doc="Set the KEYR register."]
    #[inline] pub fn set_keyr<F: FnOnce(Keyr) -> Keyr>(&self, f: F) -> &Self {
        self.keyr_reg().set(f);
        self
    }

    #[doc="Get the OPTKEYR Register."]
    #[inline] pub fn optkeyr_reg(&self) -> ::bobbin_mcu::register::Register<Optkeyr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Optkeyr, 0xc)
    }

    #[doc="Get the *mut pointer for the OPTKEYR register."]
    #[inline] pub fn optkeyr_mut(&self) -> *mut Optkeyr { 
        self.optkeyr_reg().ptr()
    }

    #[doc="Get the *const pointer for the OPTKEYR register."]
    #[inline] pub fn optkeyr_ptr(&self) -> *const Optkeyr { 
        self.optkeyr_reg().ptr()
    }

    #[doc="Write the OPTKEYR register."]
    #[inline] pub fn write_optkeyr(&self, value: Optkeyr) -> &Self { 
        self.optkeyr_reg().write(value);
        self
    }

    #[doc="Set the OPTKEYR register."]
    #[inline] pub fn set_optkeyr<F: FnOnce(Optkeyr) -> Optkeyr>(&self, f: F) -> &Self {
        self.optkeyr_reg().set(f);
        self
    }

    #[doc="Get the SR Register."]
    #[inline] pub fn sr_reg(&self) -> ::bobbin_mcu::register::Register<Sr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sr, 0x10)
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

    #[doc="Get the CR Register."]
    #[inline] pub fn cr_reg(&self) -> ::bobbin_mcu::register::Register<Cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cr, 0x14)
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

    #[doc="Get the ECCR Register."]
    #[inline] pub fn eccr_reg(&self) -> ::bobbin_mcu::register::Register<Eccr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Eccr, 0x18)
    }

    #[doc="Get the *mut pointer for the ECCR register."]
    #[inline] pub fn eccr_mut(&self) -> *mut Eccr { 
        self.eccr_reg().ptr()
    }

    #[doc="Get the *const pointer for the ECCR register."]
    #[inline] pub fn eccr_ptr(&self) -> *const Eccr { 
        self.eccr_reg().ptr()
    }

    #[doc="Read the ECCR register."]
    #[inline] pub fn eccr(&self) -> Eccr { 
        self.eccr_reg().read()
    }

    #[doc="Write the ECCR register."]
    #[inline] pub fn write_eccr(&self, value: Eccr) -> &Self { 
        self.eccr_reg().write(value);
        self
    }

    #[doc="Set the ECCR register."]
    #[inline] pub fn set_eccr<F: FnOnce(Eccr) -> Eccr>(&self, f: F) -> &Self {
        self.eccr_reg().set(f);
        self
    }

    #[doc="Modify the ECCR register."]
    #[inline] pub fn with_eccr<F: FnOnce(Eccr) -> Eccr>(&self, f: F) -> &Self {
        self.eccr_reg().with(f);
        self
    }

    #[doc="Get the OPTR Register."]
    #[inline] pub fn optr_reg(&self) -> ::bobbin_mcu::register::Register<Optr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Optr, 0x20)
    }

    #[doc="Get the *mut pointer for the OPTR register."]
    #[inline] pub fn optr_mut(&self) -> *mut Optr { 
        self.optr_reg().ptr()
    }

    #[doc="Get the *const pointer for the OPTR register."]
    #[inline] pub fn optr_ptr(&self) -> *const Optr { 
        self.optr_reg().ptr()
    }

    #[doc="Read the OPTR register."]
    #[inline] pub fn optr(&self) -> Optr { 
        self.optr_reg().read()
    }

    #[doc="Write the OPTR register."]
    #[inline] pub fn write_optr(&self, value: Optr) -> &Self { 
        self.optr_reg().write(value);
        self
    }

    #[doc="Set the OPTR register."]
    #[inline] pub fn set_optr<F: FnOnce(Optr) -> Optr>(&self, f: F) -> &Self {
        self.optr_reg().set(f);
        self
    }

    #[doc="Modify the OPTR register."]
    #[inline] pub fn with_optr<F: FnOnce(Optr) -> Optr>(&self, f: F) -> &Self {
        self.optr_reg().with(f);
        self
    }

    #[doc="Get the PCROP1ASR Register."]
    #[inline] pub fn pcrop1asr_reg(&self) -> ::bobbin_mcu::register::Register<Pcrop1asr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pcrop1asr, 0x24)
    }

    #[doc="Get the *mut pointer for the PCROP1ASR register."]
    #[inline] pub fn pcrop1asr_mut(&self) -> *mut Pcrop1asr { 
        self.pcrop1asr_reg().ptr()
    }

    #[doc="Get the *const pointer for the PCROP1ASR register."]
    #[inline] pub fn pcrop1asr_ptr(&self) -> *const Pcrop1asr { 
        self.pcrop1asr_reg().ptr()
    }

    #[doc="Read the PCROP1ASR register."]
    #[inline] pub fn pcrop1asr(&self) -> Pcrop1asr { 
        self.pcrop1asr_reg().read()
    }

    #[doc="Write the PCROP1ASR register."]
    #[inline] pub fn write_pcrop1asr(&self, value: Pcrop1asr) -> &Self { 
        self.pcrop1asr_reg().write(value);
        self
    }

    #[doc="Set the PCROP1ASR register."]
    #[inline] pub fn set_pcrop1asr<F: FnOnce(Pcrop1asr) -> Pcrop1asr>(&self, f: F) -> &Self {
        self.pcrop1asr_reg().set(f);
        self
    }

    #[doc="Modify the PCROP1ASR register."]
    #[inline] pub fn with_pcrop1asr<F: FnOnce(Pcrop1asr) -> Pcrop1asr>(&self, f: F) -> &Self {
        self.pcrop1asr_reg().with(f);
        self
    }

    #[doc="Get the PCROP1AER Register."]
    #[inline] pub fn pcrop1aer_reg(&self) -> ::bobbin_mcu::register::Register<Pcrop1aer> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pcrop1aer, 0x28)
    }

    #[doc="Get the *mut pointer for the PCROP1AER register."]
    #[inline] pub fn pcrop1aer_mut(&self) -> *mut Pcrop1aer { 
        self.pcrop1aer_reg().ptr()
    }

    #[doc="Get the *const pointer for the PCROP1AER register."]
    #[inline] pub fn pcrop1aer_ptr(&self) -> *const Pcrop1aer { 
        self.pcrop1aer_reg().ptr()
    }

    #[doc="Read the PCROP1AER register."]
    #[inline] pub fn pcrop1aer(&self) -> Pcrop1aer { 
        self.pcrop1aer_reg().read()
    }

    #[doc="Write the PCROP1AER register."]
    #[inline] pub fn write_pcrop1aer(&self, value: Pcrop1aer) -> &Self { 
        self.pcrop1aer_reg().write(value);
        self
    }

    #[doc="Set the PCROP1AER register."]
    #[inline] pub fn set_pcrop1aer<F: FnOnce(Pcrop1aer) -> Pcrop1aer>(&self, f: F) -> &Self {
        self.pcrop1aer_reg().set(f);
        self
    }

    #[doc="Modify the PCROP1AER register."]
    #[inline] pub fn with_pcrop1aer<F: FnOnce(Pcrop1aer) -> Pcrop1aer>(&self, f: F) -> &Self {
        self.pcrop1aer_reg().with(f);
        self
    }

    #[doc="Get the WRP1AR Register."]
    #[inline] pub fn wrp1ar_reg(&self) -> ::bobbin_mcu::register::Register<Wrp1ar> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Wrp1ar, 0x2c)
    }

    #[doc="Get the *mut pointer for the WRP1AR register."]
    #[inline] pub fn wrp1ar_mut(&self) -> *mut Wrp1ar { 
        self.wrp1ar_reg().ptr()
    }

    #[doc="Get the *const pointer for the WRP1AR register."]
    #[inline] pub fn wrp1ar_ptr(&self) -> *const Wrp1ar { 
        self.wrp1ar_reg().ptr()
    }

    #[doc="Read the WRP1AR register."]
    #[inline] pub fn wrp1ar(&self) -> Wrp1ar { 
        self.wrp1ar_reg().read()
    }

    #[doc="Write the WRP1AR register."]
    #[inline] pub fn write_wrp1ar(&self, value: Wrp1ar) -> &Self { 
        self.wrp1ar_reg().write(value);
        self
    }

    #[doc="Set the WRP1AR register."]
    #[inline] pub fn set_wrp1ar<F: FnOnce(Wrp1ar) -> Wrp1ar>(&self, f: F) -> &Self {
        self.wrp1ar_reg().set(f);
        self
    }

    #[doc="Modify the WRP1AR register."]
    #[inline] pub fn with_wrp1ar<F: FnOnce(Wrp1ar) -> Wrp1ar>(&self, f: F) -> &Self {
        self.wrp1ar_reg().with(f);
        self
    }

    #[doc="Get the WRP1BR Register."]
    #[inline] pub fn wrp1br_reg(&self) -> ::bobbin_mcu::register::Register<Wrp1br> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Wrp1br, 0x30)
    }

    #[doc="Get the *mut pointer for the WRP1BR register."]
    #[inline] pub fn wrp1br_mut(&self) -> *mut Wrp1br { 
        self.wrp1br_reg().ptr()
    }

    #[doc="Get the *const pointer for the WRP1BR register."]
    #[inline] pub fn wrp1br_ptr(&self) -> *const Wrp1br { 
        self.wrp1br_reg().ptr()
    }

    #[doc="Read the WRP1BR register."]
    #[inline] pub fn wrp1br(&self) -> Wrp1br { 
        self.wrp1br_reg().read()
    }

    #[doc="Write the WRP1BR register."]
    #[inline] pub fn write_wrp1br(&self, value: Wrp1br) -> &Self { 
        self.wrp1br_reg().write(value);
        self
    }

    #[doc="Set the WRP1BR register."]
    #[inline] pub fn set_wrp1br<F: FnOnce(Wrp1br) -> Wrp1br>(&self, f: F) -> &Self {
        self.wrp1br_reg().set(f);
        self
    }

    #[doc="Modify the WRP1BR register."]
    #[inline] pub fn with_wrp1br<F: FnOnce(Wrp1br) -> Wrp1br>(&self, f: F) -> &Self {
        self.wrp1br_reg().with(f);
        self
    }

    #[doc="Get the PCROP1BSR Register."]
    #[inline] pub fn pcrop1bsr_reg(&self) -> ::bobbin_mcu::register::Register<Pcrop1bsr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pcrop1bsr, 0x34)
    }

    #[doc="Get the *mut pointer for the PCROP1BSR register."]
    #[inline] pub fn pcrop1bsr_mut(&self) -> *mut Pcrop1bsr { 
        self.pcrop1bsr_reg().ptr()
    }

    #[doc="Get the *const pointer for the PCROP1BSR register."]
    #[inline] pub fn pcrop1bsr_ptr(&self) -> *const Pcrop1bsr { 
        self.pcrop1bsr_reg().ptr()
    }

    #[doc="Read the PCROP1BSR register."]
    #[inline] pub fn pcrop1bsr(&self) -> Pcrop1bsr { 
        self.pcrop1bsr_reg().read()
    }

    #[doc="Write the PCROP1BSR register."]
    #[inline] pub fn write_pcrop1bsr(&self, value: Pcrop1bsr) -> &Self { 
        self.pcrop1bsr_reg().write(value);
        self
    }

    #[doc="Set the PCROP1BSR register."]
    #[inline] pub fn set_pcrop1bsr<F: FnOnce(Pcrop1bsr) -> Pcrop1bsr>(&self, f: F) -> &Self {
        self.pcrop1bsr_reg().set(f);
        self
    }

    #[doc="Modify the PCROP1BSR register."]
    #[inline] pub fn with_pcrop1bsr<F: FnOnce(Pcrop1bsr) -> Pcrop1bsr>(&self, f: F) -> &Self {
        self.pcrop1bsr_reg().with(f);
        self
    }

    #[doc="Get the PCROP1BER Register."]
    #[inline] pub fn pcrop1ber_reg(&self) -> ::bobbin_mcu::register::Register<Pcrop1ber> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pcrop1ber, 0x38)
    }

    #[doc="Get the *mut pointer for the PCROP1BER register."]
    #[inline] pub fn pcrop1ber_mut(&self) -> *mut Pcrop1ber { 
        self.pcrop1ber_reg().ptr()
    }

    #[doc="Get the *const pointer for the PCROP1BER register."]
    #[inline] pub fn pcrop1ber_ptr(&self) -> *const Pcrop1ber { 
        self.pcrop1ber_reg().ptr()
    }

    #[doc="Read the PCROP1BER register."]
    #[inline] pub fn pcrop1ber(&self) -> Pcrop1ber { 
        self.pcrop1ber_reg().read()
    }

    #[doc="Write the PCROP1BER register."]
    #[inline] pub fn write_pcrop1ber(&self, value: Pcrop1ber) -> &Self { 
        self.pcrop1ber_reg().write(value);
        self
    }

    #[doc="Set the PCROP1BER register."]
    #[inline] pub fn set_pcrop1ber<F: FnOnce(Pcrop1ber) -> Pcrop1ber>(&self, f: F) -> &Self {
        self.pcrop1ber_reg().set(f);
        self
    }

    #[doc="Modify the PCROP1BER register."]
    #[inline] pub fn with_pcrop1ber<F: FnOnce(Pcrop1ber) -> Pcrop1ber>(&self, f: F) -> &Self {
        self.pcrop1ber_reg().with(f);
        self
    }

    #[doc="Get the IPCCBR Register."]
    #[inline] pub fn ipccbr_reg(&self) -> ::bobbin_mcu::register::Register<Ipccbr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ipccbr, 0x3c)
    }

    #[doc="Get the *mut pointer for the IPCCBR register."]
    #[inline] pub fn ipccbr_mut(&self) -> *mut Ipccbr { 
        self.ipccbr_reg().ptr()
    }

    #[doc="Get the *const pointer for the IPCCBR register."]
    #[inline] pub fn ipccbr_ptr(&self) -> *const Ipccbr { 
        self.ipccbr_reg().ptr()
    }

    #[doc="Read the IPCCBR register."]
    #[inline] pub fn ipccbr(&self) -> Ipccbr { 
        self.ipccbr_reg().read()
    }

    #[doc="Write the IPCCBR register."]
    #[inline] pub fn write_ipccbr(&self, value: Ipccbr) -> &Self { 
        self.ipccbr_reg().write(value);
        self
    }

    #[doc="Set the IPCCBR register."]
    #[inline] pub fn set_ipccbr<F: FnOnce(Ipccbr) -> Ipccbr>(&self, f: F) -> &Self {
        self.ipccbr_reg().set(f);
        self
    }

    #[doc="Modify the IPCCBR register."]
    #[inline] pub fn with_ipccbr<F: FnOnce(Ipccbr) -> Ipccbr>(&self, f: F) -> &Self {
        self.ipccbr_reg().with(f);
        self
    }

    #[doc="Get the C2ACR Register."]
    #[inline] pub fn c2acr_reg(&self) -> ::bobbin_mcu::register::Register<C2acr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C2acr, 0x5c)
    }

    #[doc="Get the *mut pointer for the C2ACR register."]
    #[inline] pub fn c2acr_mut(&self) -> *mut C2acr { 
        self.c2acr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C2ACR register."]
    #[inline] pub fn c2acr_ptr(&self) -> *const C2acr { 
        self.c2acr_reg().ptr()
    }

    #[doc="Read the C2ACR register."]
    #[inline] pub fn c2acr(&self) -> C2acr { 
        self.c2acr_reg().read()
    }

    #[doc="Write the C2ACR register."]
    #[inline] pub fn write_c2acr(&self, value: C2acr) -> &Self { 
        self.c2acr_reg().write(value);
        self
    }

    #[doc="Set the C2ACR register."]
    #[inline] pub fn set_c2acr<F: FnOnce(C2acr) -> C2acr>(&self, f: F) -> &Self {
        self.c2acr_reg().set(f);
        self
    }

    #[doc="Modify the C2ACR register."]
    #[inline] pub fn with_c2acr<F: FnOnce(C2acr) -> C2acr>(&self, f: F) -> &Self {
        self.c2acr_reg().with(f);
        self
    }

    #[doc="Get the C2SR Register."]
    #[inline] pub fn c2sr_reg(&self) -> ::bobbin_mcu::register::Register<C2sr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C2sr, 0x60)
    }

    #[doc="Get the *mut pointer for the C2SR register."]
    #[inline] pub fn c2sr_mut(&self) -> *mut C2sr { 
        self.c2sr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C2SR register."]
    #[inline] pub fn c2sr_ptr(&self) -> *const C2sr { 
        self.c2sr_reg().ptr()
    }

    #[doc="Read the C2SR register."]
    #[inline] pub fn c2sr(&self) -> C2sr { 
        self.c2sr_reg().read()
    }

    #[doc="Write the C2SR register."]
    #[inline] pub fn write_c2sr(&self, value: C2sr) -> &Self { 
        self.c2sr_reg().write(value);
        self
    }

    #[doc="Set the C2SR register."]
    #[inline] pub fn set_c2sr<F: FnOnce(C2sr) -> C2sr>(&self, f: F) -> &Self {
        self.c2sr_reg().set(f);
        self
    }

    #[doc="Modify the C2SR register."]
    #[inline] pub fn with_c2sr<F: FnOnce(C2sr) -> C2sr>(&self, f: F) -> &Self {
        self.c2sr_reg().with(f);
        self
    }

    #[doc="Get the C2CR Register."]
    #[inline] pub fn c2cr_reg(&self) -> ::bobbin_mcu::register::Register<C2cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C2cr, 0x64)
    }

    #[doc="Get the *mut pointer for the C2CR register."]
    #[inline] pub fn c2cr_mut(&self) -> *mut C2cr { 
        self.c2cr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C2CR register."]
    #[inline] pub fn c2cr_ptr(&self) -> *const C2cr { 
        self.c2cr_reg().ptr()
    }

    #[doc="Read the C2CR register."]
    #[inline] pub fn c2cr(&self) -> C2cr { 
        self.c2cr_reg().read()
    }

    #[doc="Write the C2CR register."]
    #[inline] pub fn write_c2cr(&self, value: C2cr) -> &Self { 
        self.c2cr_reg().write(value);
        self
    }

    #[doc="Set the C2CR register."]
    #[inline] pub fn set_c2cr<F: FnOnce(C2cr) -> C2cr>(&self, f: F) -> &Self {
        self.c2cr_reg().set(f);
        self
    }

    #[doc="Modify the C2CR register."]
    #[inline] pub fn with_c2cr<F: FnOnce(C2cr) -> C2cr>(&self, f: F) -> &Self {
        self.c2cr_reg().with(f);
        self
    }

    #[doc="Get the SFR Register."]
    #[inline] pub fn sfr_reg(&self) -> ::bobbin_mcu::register::Register<Sfr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sfr, 0x80)
    }

    #[doc="Get the *mut pointer for the SFR register."]
    #[inline] pub fn sfr_mut(&self) -> *mut Sfr { 
        self.sfr_reg().ptr()
    }

    #[doc="Get the *const pointer for the SFR register."]
    #[inline] pub fn sfr_ptr(&self) -> *const Sfr { 
        self.sfr_reg().ptr()
    }

    #[doc="Read the SFR register."]
    #[inline] pub fn sfr(&self) -> Sfr { 
        self.sfr_reg().read()
    }

    #[doc="Write the SFR register."]
    #[inline] pub fn write_sfr(&self, value: Sfr) -> &Self { 
        self.sfr_reg().write(value);
        self
    }

    #[doc="Set the SFR register."]
    #[inline] pub fn set_sfr<F: FnOnce(Sfr) -> Sfr>(&self, f: F) -> &Self {
        self.sfr_reg().set(f);
        self
    }

    #[doc="Modify the SFR register."]
    #[inline] pub fn with_sfr<F: FnOnce(Sfr) -> Sfr>(&self, f: F) -> &Self {
        self.sfr_reg().with(f);
        self
    }

    #[doc="Get the SRRVR Register."]
    #[inline] pub fn srrvr_reg(&self) -> ::bobbin_mcu::register::Register<Srrvr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Srrvr, 0x84)
    }

    #[doc="Get the *mut pointer for the SRRVR register."]
    #[inline] pub fn srrvr_mut(&self) -> *mut Srrvr { 
        self.srrvr_reg().ptr()
    }

    #[doc="Get the *const pointer for the SRRVR register."]
    #[inline] pub fn srrvr_ptr(&self) -> *const Srrvr { 
        self.srrvr_reg().ptr()
    }

    #[doc="Read the SRRVR register."]
    #[inline] pub fn srrvr(&self) -> Srrvr { 
        self.srrvr_reg().read()
    }

    #[doc="Write the SRRVR register."]
    #[inline] pub fn write_srrvr(&self, value: Srrvr) -> &Self { 
        self.srrvr_reg().write(value);
        self
    }

    #[doc="Set the SRRVR register."]
    #[inline] pub fn set_srrvr<F: FnOnce(Srrvr) -> Srrvr>(&self, f: F) -> &Self {
        self.srrvr_reg().set(f);
        self
    }

    #[doc="Modify the SRRVR register."]
    #[inline] pub fn with_srrvr<F: FnOnce(Srrvr) -> Srrvr>(&self, f: F) -> &Self {
        self.srrvr_reg().with(f);
        self
    }

}

#[doc="Access control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Acr(pub u32);
impl Acr {
    #[doc="Latency"]
    #[inline] pub fn latency(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if LATENCY != 0"]
    #[inline] pub fn test_latency(&self) -> bool {
        self.latency() != 0
    }

    #[doc="Sets the LATENCY field."]
    #[inline] pub fn set_latency<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Prefetch enable"]
    #[inline] pub fn prften(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PRFTEN != 0"]
    #[inline] pub fn test_prften(&self) -> bool {
        self.prften() != 0
    }

    #[doc="Sets the PRFTEN field."]
    #[inline] pub fn set_prften<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Instruction cache enable"]
    #[inline] pub fn icen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if ICEN != 0"]
    #[inline] pub fn test_icen(&self) -> bool {
        self.icen() != 0
    }

    #[doc="Sets the ICEN field."]
    #[inline] pub fn set_icen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Data cache enable"]
    #[inline] pub fn dcen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DCEN != 0"]
    #[inline] pub fn test_dcen(&self) -> bool {
        self.dcen() != 0
    }

    #[doc="Sets the DCEN field."]
    #[inline] pub fn set_dcen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Instruction cache reset"]
    #[inline] pub fn icrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if ICRST != 0"]
    #[inline] pub fn test_icrst(&self) -> bool {
        self.icrst() != 0
    }

    #[doc="Sets the ICRST field."]
    #[inline] pub fn set_icrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Data cache reset"]
    #[inline] pub fn dcrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if DCRST != 0"]
    #[inline] pub fn test_dcrst(&self) -> bool {
        self.dcrst() != 0
    }

    #[doc="Sets the DCRST field."]
    #[inline] pub fn set_dcrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="CPU1 CortexM4 program erase suspend request"]
    #[inline] pub fn pes(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PES != 0"]
    #[inline] pub fn test_pes(&self) -> bool {
        self.pes() != 0
    }

    #[doc="Sets the PES field."]
    #[inline] pub fn set_pes<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Flash User area empty"]
    #[inline] pub fn empty(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if EMPTY != 0"]
    #[inline] pub fn test_empty(&self) -> bool {
        self.empty() != 0
    }

    #[doc="Sets the EMPTY field."]
    #[inline] pub fn set_empty<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Acr {
    #[inline]
    fn from(other: u32) -> Self {
         Acr(other)
    }
}

impl ::core::fmt::Display for Acr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Acr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.latency() != 0 { try!(write!(f, " latency=0x{:x}", self.latency()))}
        if self.prften() != 0 { try!(write!(f, " prften"))}
        if self.icen() != 0 { try!(write!(f, " icen"))}
        if self.dcen() != 0 { try!(write!(f, " dcen"))}
        if self.icrst() != 0 { try!(write!(f, " icrst"))}
        if self.dcrst() != 0 { try!(write!(f, " dcrst"))}
        if self.pes() != 0 { try!(write!(f, " pes"))}
        if self.empty() != 0 { try!(write!(f, " empty"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash key register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Keyr(pub u32);
impl Keyr {
    #[doc="KEYR"]
    #[inline] pub fn keyr(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if KEYR != 0"]
    #[inline] pub fn test_keyr(&self) -> bool {
        self.keyr() != 0
    }

    #[doc="Sets the KEYR field."]
    #[inline] pub fn set_keyr<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Keyr {
    #[inline]
    fn from(other: u32) -> Self {
         Keyr(other)
    }
}

impl ::core::fmt::Display for Keyr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Keyr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Option byte key register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Optkeyr(pub u32);
impl Optkeyr {
    #[doc="Option byte key"]
    #[inline] pub fn optkeyr(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if OPTKEYR != 0"]
    #[inline] pub fn test_optkeyr(&self) -> bool {
        self.optkeyr() != 0
    }

    #[doc="Sets the OPTKEYR field."]
    #[inline] pub fn set_optkeyr<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Optkeyr {
    #[inline]
    fn from(other: u32) -> Self {
         Optkeyr(other)
    }
}

impl ::core::fmt::Display for Optkeyr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Optkeyr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc="End of operation"]
    #[inline] pub fn eop(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EOP != 0"]
    #[inline] pub fn test_eop(&self) -> bool {
        self.eop() != 0
    }

    #[doc="Sets the EOP field."]
    #[inline] pub fn set_eop<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Operation error"]
    #[inline] pub fn operr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if OPERR != 0"]
    #[inline] pub fn test_operr(&self) -> bool {
        self.operr() != 0
    }

    #[doc="Sets the OPERR field."]
    #[inline] pub fn set_operr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Programming error"]
    #[inline] pub fn progerr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PROGERR != 0"]
    #[inline] pub fn test_progerr(&self) -> bool {
        self.progerr() != 0
    }

    #[doc="Sets the PROGERR field."]
    #[inline] pub fn set_progerr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Write protected error"]
    #[inline] pub fn wrperr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if WRPERR != 0"]
    #[inline] pub fn test_wrperr(&self) -> bool {
        self.wrperr() != 0
    }

    #[doc="Sets the WRPERR field."]
    #[inline] pub fn set_wrperr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Programming alignment error"]
    #[inline] pub fn pgaerr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PGAERR != 0"]
    #[inline] pub fn test_pgaerr(&self) -> bool {
        self.pgaerr() != 0
    }

    #[doc="Sets the PGAERR field."]
    #[inline] pub fn set_pgaerr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Size error"]
    #[inline] pub fn sizerr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if SIZERR != 0"]
    #[inline] pub fn test_sizerr(&self) -> bool {
        self.sizerr() != 0
    }

    #[doc="Sets the SIZERR field."]
    #[inline] pub fn set_sizerr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Programming sequence error"]
    #[inline] pub fn pgserr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PGSERR != 0"]
    #[inline] pub fn test_pgserr(&self) -> bool {
        self.pgserr() != 0
    }

    #[doc="Sets the PGSERR field."]
    #[inline] pub fn set_pgserr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Fast programming data miss error"]
    #[inline] pub fn miserr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if MISERR != 0"]
    #[inline] pub fn test_miserr(&self) -> bool {
        self.miserr() != 0
    }

    #[doc="Sets the MISERR field."]
    #[inline] pub fn set_miserr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Fast programming error"]
    #[inline] pub fn fasterr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FASTERR != 0"]
    #[inline] pub fn test_fasterr(&self) -> bool {
        self.fasterr() != 0
    }

    #[doc="Sets the FASTERR field."]
    #[inline] pub fn set_fasterr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="User Option OPTVAL indication"]
    #[inline] pub fn optnv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if OPTNV != 0"]
    #[inline] pub fn test_optnv(&self) -> bool {
        self.optnv() != 0
    }

    #[doc="Sets the OPTNV field."]
    #[inline] pub fn set_optnv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="PCROP read error"]
    #[inline] pub fn rderr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if RDERR != 0"]
    #[inline] pub fn test_rderr(&self) -> bool {
        self.rderr() != 0
    }

    #[doc="Sets the RDERR field."]
    #[inline] pub fn set_rderr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Option validity error"]
    #[inline] pub fn optverr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if OPTVERR != 0"]
    #[inline] pub fn test_optverr(&self) -> bool {
        self.optverr() != 0
    }

    #[doc="Sets the OPTVERR field."]
    #[inline] pub fn set_optverr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Busy"]
    #[inline] pub fn bsy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if BSY != 0"]
    #[inline] pub fn test_bsy(&self) -> bool {
        self.bsy() != 0
    }

    #[doc="Sets the BSY field."]
    #[inline] pub fn set_bsy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Programming or erase configuration busy"]
    #[inline] pub fn cfgbsy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if CFGBSY != 0"]
    #[inline] pub fn test_cfgbsy(&self) -> bool {
        self.cfgbsy() != 0
    }

    #[doc="Sets the CFGBSY field."]
    #[inline] pub fn set_cfgbsy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Programming or erase operation suspended"]
    #[inline] pub fn pesd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if PESD != 0"]
    #[inline] pub fn test_pesd(&self) -> bool {
        self.pesd() != 0
    }

    #[doc="Sets the PESD field."]
    #[inline] pub fn set_pesd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
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
        if self.eop() != 0 { try!(write!(f, " eop"))}
        if self.operr() != 0 { try!(write!(f, " operr"))}
        if self.progerr() != 0 { try!(write!(f, " progerr"))}
        if self.wrperr() != 0 { try!(write!(f, " wrperr"))}
        if self.pgaerr() != 0 { try!(write!(f, " pgaerr"))}
        if self.sizerr() != 0 { try!(write!(f, " sizerr"))}
        if self.pgserr() != 0 { try!(write!(f, " pgserr"))}
        if self.miserr() != 0 { try!(write!(f, " miserr"))}
        if self.fasterr() != 0 { try!(write!(f, " fasterr"))}
        if self.optnv() != 0 { try!(write!(f, " optnv"))}
        if self.rderr() != 0 { try!(write!(f, " rderr"))}
        if self.optverr() != 0 { try!(write!(f, " optverr"))}
        if self.bsy() != 0 { try!(write!(f, " bsy"))}
        if self.cfgbsy() != 0 { try!(write!(f, " cfgbsy"))}
        if self.pesd() != 0 { try!(write!(f, " pesd"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="Programming"]
    #[inline] pub fn pg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PG != 0"]
    #[inline] pub fn test_pg(&self) -> bool {
        self.pg() != 0
    }

    #[doc="Sets the PG field."]
    #[inline] pub fn set_pg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Page erase"]
    #[inline] pub fn per(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PER != 0"]
    #[inline] pub fn test_per(&self) -> bool {
        self.per() != 0
    }

    #[doc="Sets the PER field."]
    #[inline] pub fn set_per<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="This bit triggers the mass erase (all user pages) when set"]
    #[inline] pub fn mer(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if MER != 0"]
    #[inline] pub fn test_mer(&self) -> bool {
        self.mer() != 0
    }

    #[doc="Sets the MER field."]
    #[inline] pub fn set_mer<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Page number selection"]
    #[inline] pub fn pnb(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0xff) as u8) } // [10:3]
    }

    #[doc="Returns true if PNB != 0"]
    #[inline] pub fn test_pnb(&self) -> bool {
        self.pnb() != 0
    }

    #[doc="Sets the PNB field."]
    #[inline] pub fn set_pnb<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Start"]
    #[inline] pub fn strt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if STRT != 0"]
    #[inline] pub fn test_strt(&self) -> bool {
        self.strt() != 0
    }

    #[doc="Sets the STRT field."]
    #[inline] pub fn set_strt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Options modification start"]
    #[inline] pub fn optstrt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if OPTSTRT != 0"]
    #[inline] pub fn test_optstrt(&self) -> bool {
        self.optstrt() != 0
    }

    #[doc="Sets the OPTSTRT field."]
    #[inline] pub fn set_optstrt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Fast programming"]
    #[inline] pub fn fstpg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if FSTPG != 0"]
    #[inline] pub fn test_fstpg(&self) -> bool {
        self.fstpg() != 0
    }

    #[doc="Sets the FSTPG field."]
    #[inline] pub fn set_fstpg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="End of operation interrupt enable"]
    #[inline] pub fn eopie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if EOPIE != 0"]
    #[inline] pub fn test_eopie(&self) -> bool {
        self.eopie() != 0
    }

    #[doc="Sets the EOPIE field."]
    #[inline] pub fn set_eopie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Error interrupt enable"]
    #[inline] pub fn errie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if ERRIE != 0"]
    #[inline] pub fn test_errie(&self) -> bool {
        self.errie() != 0
    }

    #[doc="Sets the ERRIE field."]
    #[inline] pub fn set_errie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="PCROP read error interrupt enable"]
    #[inline] pub fn rderrie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if RDERRIE != 0"]
    #[inline] pub fn test_rderrie(&self) -> bool {
        self.rderrie() != 0
    }

    #[doc="Sets the RDERRIE field."]
    #[inline] pub fn set_rderrie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Force the option byte loading"]
    #[inline] pub fn obl_launch(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if OBL_LAUNCH != 0"]
    #[inline] pub fn test_obl_launch(&self) -> bool {
        self.obl_launch() != 0
    }

    #[doc="Sets the OBL_LAUNCH field."]
    #[inline] pub fn set_obl_launch<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Options Lock"]
    #[inline] pub fn optlock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if OPTLOCK != 0"]
    #[inline] pub fn test_optlock(&self) -> bool {
        self.optlock() != 0
    }

    #[doc="Sets the OPTLOCK field."]
    #[inline] pub fn set_optlock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="FLASH_CR Lock"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
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
        if self.pg() != 0 { try!(write!(f, " pg"))}
        if self.per() != 0 { try!(write!(f, " per"))}
        if self.mer() != 0 { try!(write!(f, " mer"))}
        if self.pnb() != 0 { try!(write!(f, " pnb=0x{:x}", self.pnb()))}
        if self.strt() != 0 { try!(write!(f, " strt"))}
        if self.optstrt() != 0 { try!(write!(f, " optstrt"))}
        if self.fstpg() != 0 { try!(write!(f, " fstpg"))}
        if self.eopie() != 0 { try!(write!(f, " eopie"))}
        if self.errie() != 0 { try!(write!(f, " errie"))}
        if self.rderrie() != 0 { try!(write!(f, " rderrie"))}
        if self.obl_launch() != 0 { try!(write!(f, " obl_launch"))}
        if self.optlock() != 0 { try!(write!(f, " optlock"))}
        if self.lock() != 0 { try!(write!(f, " lock"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash ECC register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Eccr(pub u32);
impl Eccr {
    #[doc="ECC fail address"]
    #[inline] pub fn addr_ecc(&self) -> ::bobbin_bits::U17 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ffff) as u32) } // [16:0]
    }

    #[doc="Returns true if ADDR_ECC != 0"]
    #[inline] pub fn test_addr_ecc(&self) -> bool {
        self.addr_ecc() != 0
    }

    #[doc="Sets the ADDR_ECC field."]
    #[inline] pub fn set_addr_ecc<V: Into<::bobbin_bits::U17>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U17 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="System Flash ECC fail"]
    #[inline] pub fn sysf_ecc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if SYSF_ECC != 0"]
    #[inline] pub fn test_sysf_ecc(&self) -> bool {
        self.sysf_ecc() != 0
    }

    #[doc="Sets the SYSF_ECC field."]
    #[inline] pub fn set_sysf_ecc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="ECC correction interrupt enable"]
    #[inline] pub fn ecccie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if ECCCIE != 0"]
    #[inline] pub fn test_ecccie(&self) -> bool {
        self.ecccie() != 0
    }

    #[doc="Sets the ECCCIE field."]
    #[inline] pub fn set_ecccie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="CPU identification"]
    #[inline] pub fn cpuid(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x7) as u8) } // [28:26]
    }

    #[doc="Returns true if CPUID != 0"]
    #[inline] pub fn test_cpuid(&self) -> bool {
        self.cpuid() != 0
    }

    #[doc="Sets the CPUID field."]
    #[inline] pub fn set_cpuid<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="ECC correction"]
    #[inline] pub fn eccc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if ECCC != 0"]
    #[inline] pub fn test_eccc(&self) -> bool {
        self.eccc() != 0
    }

    #[doc="Sets the ECCC field."]
    #[inline] pub fn set_eccc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="ECC detection"]
    #[inline] pub fn eccd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if ECCD != 0"]
    #[inline] pub fn test_eccd(&self) -> bool {
        self.eccd() != 0
    }

    #[doc="Sets the ECCD field."]
    #[inline] pub fn set_eccd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Eccr {
    #[inline]
    fn from(other: u32) -> Self {
         Eccr(other)
    }
}

impl ::core::fmt::Display for Eccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Eccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.addr_ecc() != 0 { try!(write!(f, " addr_ecc=0x{:x}", self.addr_ecc()))}
        if self.sysf_ecc() != 0 { try!(write!(f, " sysf_ecc"))}
        if self.ecccie() != 0 { try!(write!(f, " ecccie"))}
        if self.cpuid() != 0 { try!(write!(f, " cpuid=0x{:x}", self.cpuid()))}
        if self.eccc() != 0 { try!(write!(f, " eccc"))}
        if self.eccd() != 0 { try!(write!(f, " eccd"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash option register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Optr(pub u32);
impl Optr {
    #[doc="Read protection level"]
    #[inline] pub fn rdp(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if RDP != 0"]
    #[inline] pub fn test_rdp(&self) -> bool {
        self.rdp() != 0
    }

    #[doc="Sets the RDP field."]
    #[inline] pub fn set_rdp<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Security enabled"]
    #[inline] pub fn ese(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if ESE != 0"]
    #[inline] pub fn test_ese(&self) -> bool {
        self.ese() != 0
    }

    #[doc="Sets the ESE field."]
    #[inline] pub fn set_ese<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="BOR reset Level"]
    #[inline] pub fn bor_lev(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x7) as u8) } // [11:9]
    }

    #[doc="Returns true if BOR_LEV != 0"]
    #[inline] pub fn test_bor_lev(&self) -> bool {
        self.bor_lev() != 0
    }

    #[doc="Sets the BOR_LEV field."]
    #[inline] pub fn set_bor_lev<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="nRST_STOP"]
    #[inline] pub fn nrst_stop(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if NRST_STOP != 0"]
    #[inline] pub fn test_nrst_stop(&self) -> bool {
        self.nrst_stop() != 0
    }

    #[doc="Sets the NRST_STOP field."]
    #[inline] pub fn set_nrst_stop<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="nRST_STDBY"]
    #[inline] pub fn nrst_stdby(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if NRST_STDBY != 0"]
    #[inline] pub fn test_nrst_stdby(&self) -> bool {
        self.nrst_stdby() != 0
    }

    #[doc="Sets the NRST_STDBY field."]
    #[inline] pub fn set_nrst_stdby<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="nRST_SHDW"]
    #[inline] pub fn nrst_shdw(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if NRST_SHDW != 0"]
    #[inline] pub fn test_nrst_shdw(&self) -> bool {
        self.nrst_shdw() != 0
    }

    #[doc="Sets the NRST_SHDW field."]
    #[inline] pub fn set_nrst_shdw<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Independent watchdog selection"]
    #[inline] pub fn idwg_sw(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if IDWG_SW != 0"]
    #[inline] pub fn test_idwg_sw(&self) -> bool {
        self.idwg_sw() != 0
    }

    #[doc="Sets the IDWG_SW field."]
    #[inline] pub fn set_idwg_sw<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Independent watchdog counter freeze in Stop mode"]
    #[inline] pub fn iwdg_stop(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if IWDG_STOP != 0"]
    #[inline] pub fn test_iwdg_stop(&self) -> bool {
        self.iwdg_stop() != 0
    }

    #[doc="Sets the IWDG_STOP field."]
    #[inline] pub fn set_iwdg_stop<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Independent watchdog counter freeze in Standby mode"]
    #[inline] pub fn iwdg_stdby(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if IWDG_STDBY != 0"]
    #[inline] pub fn test_iwdg_stdby(&self) -> bool {
        self.iwdg_stdby() != 0
    }

    #[doc="Sets the IWDG_STDBY field."]
    #[inline] pub fn set_iwdg_stdby<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Window watchdog selection"]
    #[inline] pub fn wwdg_sw(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if WWDG_SW != 0"]
    #[inline] pub fn test_wwdg_sw(&self) -> bool {
        self.wwdg_sw() != 0
    }

    #[doc="Sets the WWDG_SW field."]
    #[inline] pub fn set_wwdg_sw<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Boot configuration"]
    #[inline] pub fn nboot1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if NBOOT1 != 0"]
    #[inline] pub fn test_nboot1(&self) -> bool {
        self.nboot1() != 0
    }

    #[doc="Sets the NBOOT1 field."]
    #[inline] pub fn set_nboot1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="SRAM2 parity check enable"]
    #[inline] pub fn sram2_pe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if SRAM2_PE != 0"]
    #[inline] pub fn test_sram2_pe(&self) -> bool {
        self.sram2_pe() != 0
    }

    #[doc="Sets the SRAM2_PE field."]
    #[inline] pub fn set_sram2_pe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="SRAM2 Erase when system reset"]
    #[inline] pub fn sram2_rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if SRAM2_RST != 0"]
    #[inline] pub fn test_sram2_rst(&self) -> bool {
        self.sram2_rst() != 0
    }

    #[doc="Sets the SRAM2_RST field."]
    #[inline] pub fn set_sram2_rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Software Boot0"]
    #[inline] pub fn nswboot0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if NSWBOOT0 != 0"]
    #[inline] pub fn test_nswboot0(&self) -> bool {
        self.nswboot0() != 0
    }

    #[doc="Sets the NSWBOOT0 field."]
    #[inline] pub fn set_nswboot0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="nBoot0 option bit"]
    #[inline] pub fn nboot0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if NBOOT0 != 0"]
    #[inline] pub fn test_nboot0(&self) -> bool {
        self.nboot0() != 0
    }

    #[doc="Sets the NBOOT0 field."]
    #[inline] pub fn set_nboot0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Radio Automatic Gain Control Trimming"]
    #[inline] pub fn agc_trim(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x7) as u8) } // [31:29]
    }

    #[doc="Returns true if AGC_TRIM != 0"]
    #[inline] pub fn test_agc_trim(&self) -> bool {
        self.agc_trim() != 0
    }

    #[doc="Sets the AGC_TRIM field."]
    #[inline] pub fn set_agc_trim<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 29);
        self.0 |= value << 29;
        self
    }

}

impl From<u32> for Optr {
    #[inline]
    fn from(other: u32) -> Self {
         Optr(other)
    }
}

impl ::core::fmt::Display for Optr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Optr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rdp() != 0 { try!(write!(f, " rdp=0x{:x}", self.rdp()))}
        if self.ese() != 0 { try!(write!(f, " ese"))}
        if self.bor_lev() != 0 { try!(write!(f, " bor_lev=0x{:x}", self.bor_lev()))}
        if self.nrst_stop() != 0 { try!(write!(f, " nrst_stop"))}
        if self.nrst_stdby() != 0 { try!(write!(f, " nrst_stdby"))}
        if self.nrst_shdw() != 0 { try!(write!(f, " nrst_shdw"))}
        if self.idwg_sw() != 0 { try!(write!(f, " idwg_sw"))}
        if self.iwdg_stop() != 0 { try!(write!(f, " iwdg_stop"))}
        if self.iwdg_stdby() != 0 { try!(write!(f, " iwdg_stdby"))}
        if self.wwdg_sw() != 0 { try!(write!(f, " wwdg_sw"))}
        if self.nboot1() != 0 { try!(write!(f, " nboot1"))}
        if self.sram2_pe() != 0 { try!(write!(f, " sram2_pe"))}
        if self.sram2_rst() != 0 { try!(write!(f, " sram2_rst"))}
        if self.nswboot0() != 0 { try!(write!(f, " nswboot0"))}
        if self.nboot0() != 0 { try!(write!(f, " nboot0"))}
        if self.agc_trim() != 0 { try!(write!(f, " agc_trim=0x{:x}", self.agc_trim()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash Bank 1 PCROP Start address zone A register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pcrop1asr(pub u32);
impl Pcrop1asr {
    #[doc="Bank 1 PCROPQ area start offset"]
    #[inline] pub fn pcrop1a_strt(&self) -> ::bobbin_bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
    }

    #[doc="Returns true if PCROP1A_STRT != 0"]
    #[inline] pub fn test_pcrop1a_strt(&self) -> bool {
        self.pcrop1a_strt() != 0
    }

    #[doc="Sets the PCROP1A_STRT field."]
    #[inline] pub fn set_pcrop1a_strt<V: Into<::bobbin_bits::U9>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U9 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pcrop1asr {
    #[inline]
    fn from(other: u32) -> Self {
         Pcrop1asr(other)
    }
}

impl ::core::fmt::Display for Pcrop1asr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pcrop1asr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pcrop1a_strt() != 0 { try!(write!(f, " pcrop1a_strt=0x{:x}", self.pcrop1a_strt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash Bank 1 PCROP End address zone A register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pcrop1aer(pub u32);
impl Pcrop1aer {
    #[doc="Bank 1 PCROP area end offset"]
    #[inline] pub fn pcrop1a_end(&self) -> ::bobbin_bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
    }

    #[doc="Returns true if PCROP1A_END != 0"]
    #[inline] pub fn test_pcrop1a_end(&self) -> bool {
        self.pcrop1a_end() != 0
    }

    #[doc="Sets the PCROP1A_END field."]
    #[inline] pub fn set_pcrop1a_end<V: Into<::bobbin_bits::U9>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U9 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="PCROP area preserved when RDP level decreased"]
    #[inline] pub fn pcrop_rdp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PCROP_RDP != 0"]
    #[inline] pub fn test_pcrop_rdp(&self) -> bool {
        self.pcrop_rdp() != 0
    }

    #[doc="Sets the PCROP_RDP field."]
    #[inline] pub fn set_pcrop_rdp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Pcrop1aer {
    #[inline]
    fn from(other: u32) -> Self {
         Pcrop1aer(other)
    }
}

impl ::core::fmt::Display for Pcrop1aer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pcrop1aer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pcrop1a_end() != 0 { try!(write!(f, " pcrop1a_end=0x{:x}", self.pcrop1a_end()))}
        if self.pcrop_rdp() != 0 { try!(write!(f, " pcrop_rdp"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash Bank 1 WRP area A address register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Wrp1ar(pub u32);
impl Wrp1ar {
    #[doc="Bank 1 WRP first area A start offset"]
    #[inline] pub fn wrp1a_strt(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if WRP1A_STRT != 0"]
    #[inline] pub fn test_wrp1a_strt(&self) -> bool {
        self.wrp1a_strt() != 0
    }

    #[doc="Sets the WRP1A_STRT field."]
    #[inline] pub fn set_wrp1a_strt<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Bank 1 WRP first area A end offset"]
    #[inline] pub fn wrp1a_end(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if WRP1A_END != 0"]
    #[inline] pub fn test_wrp1a_end(&self) -> bool {
        self.wrp1a_end() != 0
    }

    #[doc="Sets the WRP1A_END field."]
    #[inline] pub fn set_wrp1a_end<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Wrp1ar {
    #[inline]
    fn from(other: u32) -> Self {
         Wrp1ar(other)
    }
}

impl ::core::fmt::Display for Wrp1ar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Wrp1ar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wrp1a_strt() != 0 { try!(write!(f, " wrp1a_strt=0x{:x}", self.wrp1a_strt()))}
        if self.wrp1a_end() != 0 { try!(write!(f, " wrp1a_end=0x{:x}", self.wrp1a_end()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash Bank 1 WRP area B address register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Wrp1br(pub u32);
impl Wrp1br {
    #[doc="Bank 1 WRP second area B end offset"]
    #[inline] pub fn wrp1b_strt(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if WRP1B_STRT != 0"]
    #[inline] pub fn test_wrp1b_strt(&self) -> bool {
        self.wrp1b_strt() != 0
    }

    #[doc="Sets the WRP1B_STRT field."]
    #[inline] pub fn set_wrp1b_strt<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Bank 1 WRP second area B start offset"]
    #[inline] pub fn wrp1b_end(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if WRP1B_END != 0"]
    #[inline] pub fn test_wrp1b_end(&self) -> bool {
        self.wrp1b_end() != 0
    }

    #[doc="Sets the WRP1B_END field."]
    #[inline] pub fn set_wrp1b_end<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Wrp1br {
    #[inline]
    fn from(other: u32) -> Self {
         Wrp1br(other)
    }
}

impl ::core::fmt::Display for Wrp1br {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Wrp1br {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wrp1b_strt() != 0 { try!(write!(f, " wrp1b_strt=0x{:x}", self.wrp1b_strt()))}
        if self.wrp1b_end() != 0 { try!(write!(f, " wrp1b_end=0x{:x}", self.wrp1b_end()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash Bank 1 PCROP Start address area B register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pcrop1bsr(pub u32);
impl Pcrop1bsr {
    #[doc="Bank 1 PCROP area B start offset"]
    #[inline] pub fn pcrop1b_strt(&self) -> ::bobbin_bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
    }

    #[doc="Returns true if PCROP1B_STRT != 0"]
    #[inline] pub fn test_pcrop1b_strt(&self) -> bool {
        self.pcrop1b_strt() != 0
    }

    #[doc="Sets the PCROP1B_STRT field."]
    #[inline] pub fn set_pcrop1b_strt<V: Into<::bobbin_bits::U9>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U9 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pcrop1bsr {
    #[inline]
    fn from(other: u32) -> Self {
         Pcrop1bsr(other)
    }
}

impl ::core::fmt::Display for Pcrop1bsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pcrop1bsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pcrop1b_strt() != 0 { try!(write!(f, " pcrop1b_strt=0x{:x}", self.pcrop1b_strt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash Bank 1 PCROP End address area B register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pcrop1ber(pub u32);
impl Pcrop1ber {
    #[doc="Bank 1 PCROP area end area B offset"]
    #[inline] pub fn pcrop1b_end(&self) -> ::bobbin_bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
    }

    #[doc="Returns true if PCROP1B_END != 0"]
    #[inline] pub fn test_pcrop1b_end(&self) -> bool {
        self.pcrop1b_end() != 0
    }

    #[doc="Sets the PCROP1B_END field."]
    #[inline] pub fn set_pcrop1b_end<V: Into<::bobbin_bits::U9>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U9 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pcrop1ber {
    #[inline]
    fn from(other: u32) -> Self {
         Pcrop1ber(other)
    }
}

impl ::core::fmt::Display for Pcrop1ber {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pcrop1ber {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pcrop1b_end() != 0 { try!(write!(f, " pcrop1b_end=0x{:x}", self.pcrop1b_end()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="IPCC mailbox data buffer address register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ipccbr(pub u32);
impl Ipccbr {
    #[doc="PCC mailbox data buffer base address"]
    #[inline] pub fn ipccdba(&self) -> ::bobbin_bits::U14 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3fff) as u16) } // [13:0]
    }

    #[doc="Returns true if IPCCDBA != 0"]
    #[inline] pub fn test_ipccdba(&self) -> bool {
        self.ipccdba() != 0
    }

    #[doc="Sets the IPCCDBA field."]
    #[inline] pub fn set_ipccdba<V: Into<::bobbin_bits::U14>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U14 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3fff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ipccbr {
    #[inline]
    fn from(other: u32) -> Self {
         Ipccbr(other)
    }
}

impl ::core::fmt::Display for Ipccbr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ipccbr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ipccdba() != 0 { try!(write!(f, " ipccdba=0x{:x}", self.ipccdba()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CPU2 cortex M0 access control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C2acr(pub u32);
impl C2acr {
    #[doc="CPU2 cortex M0 prefetch enable"]
    #[inline] pub fn prften(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PRFTEN != 0"]
    #[inline] pub fn test_prften(&self) -> bool {
        self.prften() != 0
    }

    #[doc="Sets the PRFTEN field."]
    #[inline] pub fn set_prften<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="CPU2 cortex M0 instruction cache enable"]
    #[inline] pub fn icen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if ICEN != 0"]
    #[inline] pub fn test_icen(&self) -> bool {
        self.icen() != 0
    }

    #[doc="Sets the ICEN field."]
    #[inline] pub fn set_icen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="CPU2 cortex M0 instruction cache reset"]
    #[inline] pub fn icrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if ICRST != 0"]
    #[inline] pub fn test_icrst(&self) -> bool {
        self.icrst() != 0
    }

    #[doc="Sets the ICRST field."]
    #[inline] pub fn set_icrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="CPU2 cortex M0 program erase suspend request"]
    #[inline] pub fn pes(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PES != 0"]
    #[inline] pub fn test_pes(&self) -> bool {
        self.pes() != 0
    }

    #[doc="Sets the PES field."]
    #[inline] pub fn set_pes<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u32> for C2acr {
    #[inline]
    fn from(other: u32) -> Self {
         C2acr(other)
    }
}

impl ::core::fmt::Display for C2acr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C2acr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.prften() != 0 { try!(write!(f, " prften"))}
        if self.icen() != 0 { try!(write!(f, " icen"))}
        if self.icrst() != 0 { try!(write!(f, " icrst"))}
        if self.pes() != 0 { try!(write!(f, " pes"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CPU2 cortex M0 status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C2sr(pub u32);
impl C2sr {
    #[doc="End of operation"]
    #[inline] pub fn eop(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EOP != 0"]
    #[inline] pub fn test_eop(&self) -> bool {
        self.eop() != 0
    }

    #[doc="Sets the EOP field."]
    #[inline] pub fn set_eop<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Operation error"]
    #[inline] pub fn operr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if OPERR != 0"]
    #[inline] pub fn test_operr(&self) -> bool {
        self.operr() != 0
    }

    #[doc="Sets the OPERR field."]
    #[inline] pub fn set_operr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Programming error"]
    #[inline] pub fn progerr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PROGERR != 0"]
    #[inline] pub fn test_progerr(&self) -> bool {
        self.progerr() != 0
    }

    #[doc="Sets the PROGERR field."]
    #[inline] pub fn set_progerr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="write protection error"]
    #[inline] pub fn wrperr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if WRPERR != 0"]
    #[inline] pub fn test_wrperr(&self) -> bool {
        self.wrperr() != 0
    }

    #[doc="Sets the WRPERR field."]
    #[inline] pub fn set_wrperr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Programming alignment error"]
    #[inline] pub fn pgaerr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PGAERR != 0"]
    #[inline] pub fn test_pgaerr(&self) -> bool {
        self.pgaerr() != 0
    }

    #[doc="Sets the PGAERR field."]
    #[inline] pub fn set_pgaerr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Size error"]
    #[inline] pub fn sizerr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if SIZERR != 0"]
    #[inline] pub fn test_sizerr(&self) -> bool {
        self.sizerr() != 0
    }

    #[doc="Sets the SIZERR field."]
    #[inline] pub fn set_sizerr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Programming sequence error"]
    #[inline] pub fn pgserr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PGSERR != 0"]
    #[inline] pub fn test_pgserr(&self) -> bool {
        self.pgserr() != 0
    }

    #[doc="Sets the PGSERR field."]
    #[inline] pub fn set_pgserr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Fast programming data miss error"]
    #[inline] pub fn misserr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if MISSERR != 0"]
    #[inline] pub fn test_misserr(&self) -> bool {
        self.misserr() != 0
    }

    #[doc="Sets the MISSERR field."]
    #[inline] pub fn set_misserr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Fast programming error"]
    #[inline] pub fn fasterr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FASTERR != 0"]
    #[inline] pub fn test_fasterr(&self) -> bool {
        self.fasterr() != 0
    }

    #[doc="Sets the FASTERR field."]
    #[inline] pub fn set_fasterr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="PCROP read error"]
    #[inline] pub fn rderr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if RDERR != 0"]
    #[inline] pub fn test_rderr(&self) -> bool {
        self.rderr() != 0
    }

    #[doc="Sets the RDERR field."]
    #[inline] pub fn set_rderr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Busy"]
    #[inline] pub fn bsy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if BSY != 0"]
    #[inline] pub fn test_bsy(&self) -> bool {
        self.bsy() != 0
    }

    #[doc="Sets the BSY field."]
    #[inline] pub fn set_bsy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Programming or erase configuration busy"]
    #[inline] pub fn cfgbsy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if CFGBSY != 0"]
    #[inline] pub fn test_cfgbsy(&self) -> bool {
        self.cfgbsy() != 0
    }

    #[doc="Sets the CFGBSY field."]
    #[inline] pub fn set_cfgbsy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Programming or erase operation suspended"]
    #[inline] pub fn pesd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if PESD != 0"]
    #[inline] pub fn test_pesd(&self) -> bool {
        self.pesd() != 0
    }

    #[doc="Sets the PESD field."]
    #[inline] pub fn set_pesd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

}

impl From<u32> for C2sr {
    #[inline]
    fn from(other: u32) -> Self {
         C2sr(other)
    }
}

impl ::core::fmt::Display for C2sr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C2sr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.eop() != 0 { try!(write!(f, " eop"))}
        if self.operr() != 0 { try!(write!(f, " operr"))}
        if self.progerr() != 0 { try!(write!(f, " progerr"))}
        if self.wrperr() != 0 { try!(write!(f, " wrperr"))}
        if self.pgaerr() != 0 { try!(write!(f, " pgaerr"))}
        if self.sizerr() != 0 { try!(write!(f, " sizerr"))}
        if self.pgserr() != 0 { try!(write!(f, " pgserr"))}
        if self.misserr() != 0 { try!(write!(f, " misserr"))}
        if self.fasterr() != 0 { try!(write!(f, " fasterr"))}
        if self.rderr() != 0 { try!(write!(f, " rderr"))}
        if self.bsy() != 0 { try!(write!(f, " bsy"))}
        if self.cfgbsy() != 0 { try!(write!(f, " cfgbsy"))}
        if self.pesd() != 0 { try!(write!(f, " pesd"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CPU2 cortex M0 control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C2cr(pub u32);
impl C2cr {
    #[doc="Programming"]
    #[inline] pub fn pg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PG != 0"]
    #[inline] pub fn test_pg(&self) -> bool {
        self.pg() != 0
    }

    #[doc="Sets the PG field."]
    #[inline] pub fn set_pg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Page erase"]
    #[inline] pub fn per(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PER != 0"]
    #[inline] pub fn test_per(&self) -> bool {
        self.per() != 0
    }

    #[doc="Sets the PER field."]
    #[inline] pub fn set_per<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Masse erase"]
    #[inline] pub fn mer(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if MER != 0"]
    #[inline] pub fn test_mer(&self) -> bool {
        self.mer() != 0
    }

    #[doc="Sets the MER field."]
    #[inline] pub fn set_mer<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Page Number selection"]
    #[inline] pub fn pnb(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0xff) as u8) } // [10:3]
    }

    #[doc="Returns true if PNB != 0"]
    #[inline] pub fn test_pnb(&self) -> bool {
        self.pnb() != 0
    }

    #[doc="Sets the PNB field."]
    #[inline] pub fn set_pnb<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Start"]
    #[inline] pub fn strt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if STRT != 0"]
    #[inline] pub fn test_strt(&self) -> bool {
        self.strt() != 0
    }

    #[doc="Sets the STRT field."]
    #[inline] pub fn set_strt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Fast programming"]
    #[inline] pub fn fstpg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if FSTPG != 0"]
    #[inline] pub fn test_fstpg(&self) -> bool {
        self.fstpg() != 0
    }

    #[doc="Sets the FSTPG field."]
    #[inline] pub fn set_fstpg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="End of operation interrupt enable"]
    #[inline] pub fn eopie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if EOPIE != 0"]
    #[inline] pub fn test_eopie(&self) -> bool {
        self.eopie() != 0
    }

    #[doc="Sets the EOPIE field."]
    #[inline] pub fn set_eopie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Error interrupt enable"]
    #[inline] pub fn errie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if ERRIE != 0"]
    #[inline] pub fn test_errie(&self) -> bool {
        self.errie() != 0
    }

    #[doc="Sets the ERRIE field."]
    #[inline] pub fn set_errie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="PCROP read error interrupt enable"]
    #[inline] pub fn rderrie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if RDERRIE != 0"]
    #[inline] pub fn test_rderrie(&self) -> bool {
        self.rderrie() != 0
    }

    #[doc="Sets the RDERRIE field."]
    #[inline] pub fn set_rderrie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

}

impl From<u32> for C2cr {
    #[inline]
    fn from(other: u32) -> Self {
         C2cr(other)
    }
}

impl ::core::fmt::Display for C2cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C2cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pg() != 0 { try!(write!(f, " pg"))}
        if self.per() != 0 { try!(write!(f, " per"))}
        if self.mer() != 0 { try!(write!(f, " mer"))}
        if self.pnb() != 0 { try!(write!(f, " pnb=0x{:x}", self.pnb()))}
        if self.strt() != 0 { try!(write!(f, " strt"))}
        if self.fstpg() != 0 { try!(write!(f, " fstpg"))}
        if self.eopie() != 0 { try!(write!(f, " eopie"))}
        if self.errie() != 0 { try!(write!(f, " errie"))}
        if self.rderrie() != 0 { try!(write!(f, " rderrie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Secure flash start address register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sfr(pub u32);
impl Sfr {
    #[doc="Secure flash start address"]
    #[inline] pub fn sfsa(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if SFSA != 0"]
    #[inline] pub fn test_sfsa(&self) -> bool {
        self.sfsa() != 0
    }

    #[doc="Sets the SFSA field."]
    #[inline] pub fn set_sfsa<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Disable Cortex M0 debug access"]
    #[inline] pub fn dds(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if DDS != 0"]
    #[inline] pub fn test_dds(&self) -> bool {
        self.dds() != 0
    }

    #[doc="Sets the DDS field."]
    #[inline] pub fn set_dds<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Flash security disable"]
    #[inline] pub fn fsd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if FSD != 0"]
    #[inline] pub fn test_fsd(&self) -> bool {
        self.fsd() != 0
    }

    #[doc="Sets the FSD field."]
    #[inline] pub fn set_fsd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Sfr {
    #[inline]
    fn from(other: u32) -> Self {
         Sfr(other)
    }
}

impl ::core::fmt::Display for Sfr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sfr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sfsa() != 0 { try!(write!(f, " sfsa=0x{:x}", self.sfsa()))}
        if self.dds() != 0 { try!(write!(f, " dds"))}
        if self.fsd() != 0 { try!(write!(f, " fsd"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Secure SRAM2 start address and cortex M0 reset vector register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Srrvr(pub u32);
impl Srrvr {
    #[doc="cortex M0 access control register"]
    #[inline] pub fn sbrv(&self) -> ::bobbin_bits::U18 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ffff) as u32) } // [17:0]
    }

    #[doc="Returns true if SBRV != 0"]
    #[inline] pub fn test_sbrv(&self) -> bool {
        self.sbrv() != 0
    }

    #[doc="Sets the SBRV field."]
    #[inline] pub fn set_sbrv<V: Into<::bobbin_bits::U18>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U18 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Secure backup SRAM2a start address"]
    #[inline] pub fn sbrsa(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1f) as u8) } // [22:18]
    }

    #[doc="Returns true if SBRSA != 0"]
    #[inline] pub fn test_sbrsa(&self) -> bool {
        self.sbrsa() != 0
    }

    #[doc="Sets the SBRSA field."]
    #[inline] pub fn set_sbrsa<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="backup SRAM2a security disable"]
    #[inline] pub fn brsd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if BRSD != 0"]
    #[inline] pub fn test_brsd(&self) -> bool {
        self.brsd() != 0
    }

    #[doc="Sets the BRSD field."]
    #[inline] pub fn set_brsd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Secure non backup SRAM2a start address"]
    #[inline] pub fn snbrsa(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1f) as u8) } // [29:25]
    }

    #[doc="Returns true if SNBRSA != 0"]
    #[inline] pub fn test_snbrsa(&self) -> bool {
        self.snbrsa() != 0
    }

    #[doc="Sets the SNBRSA field."]
    #[inline] pub fn set_snbrsa<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="CPU2 cortex M0 boot reset vector memory selection"]
    #[inline] pub fn c2opt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if C2OPT != 0"]
    #[inline] pub fn test_c2opt(&self) -> bool {
        self.c2opt() != 0
    }

    #[doc="Sets the C2OPT field."]
    #[inline] pub fn set_c2opt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="non-backup SRAM2b security disable"]
    #[inline] pub fn nbrsd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if NBRSD != 0"]
    #[inline] pub fn test_nbrsd(&self) -> bool {
        self.nbrsd() != 0
    }

    #[doc="Sets the NBRSD field."]
    #[inline] pub fn set_nbrsd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

}

impl From<u32> for Srrvr {
    #[inline]
    fn from(other: u32) -> Self {
         Srrvr(other)
    }
}

impl ::core::fmt::Display for Srrvr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Srrvr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sbrv() != 0 { try!(write!(f, " sbrv=0x{:x}", self.sbrv()))}
        if self.sbrsa() != 0 { try!(write!(f, " sbrsa=0x{:x}", self.sbrsa()))}
        if self.brsd() != 0 { try!(write!(f, " brsd"))}
        if self.snbrsa() != 0 { try!(write!(f, " snbrsa=0x{:x}", self.snbrsa()))}
        if self.c2opt() != 0 { try!(write!(f, " c2opt"))}
        if self.nbrsd() != 0 { try!(write!(f, " nbrsd"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

