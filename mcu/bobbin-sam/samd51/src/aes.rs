::bobbin_mcu::periph!( AES, Aes, AES_PERIPH, AesPeriph, AES_OWNED, AES_REF_COUNT, 0x42002400, 0x00, 0x04);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="AES Peripheral"]
pub struct AesPeriph(pub usize); 

impl AesPeriph {
    #[doc="Get the CTRLA Register."]
    #[inline] pub fn ctrla_reg(&self) -> ::bobbin_mcu::register::Register<Ctrla> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ctrla, 0x0)
    }

    #[doc="Get the *mut pointer for the CTRLA register."]
    #[inline] pub fn ctrla_mut(&self) -> *mut Ctrla { 
        self.ctrla_reg().ptr()
    }

    #[doc="Get the *const pointer for the CTRLA register."]
    #[inline] pub fn ctrla_ptr(&self) -> *const Ctrla { 
        self.ctrla_reg().ptr()
    }

    #[doc="Read the CTRLA register."]
    #[inline] pub fn ctrla(&self) -> Ctrla { 
        self.ctrla_reg().read()
    }

    #[doc="Write the CTRLA register."]
    #[inline] pub fn write_ctrla(&self, value: Ctrla) -> &Self { 
        self.ctrla_reg().write(value);
        self
    }

    #[doc="Set the CTRLA register."]
    #[inline] pub fn set_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
        self.ctrla_reg().set(f);
        self
    }

    #[doc="Modify the CTRLA register."]
    #[inline] pub fn with_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
        self.ctrla_reg().with(f);
        self
    }

    #[doc="Get the CTRLB Register."]
    #[inline] pub fn ctrlb_reg(&self) -> ::bobbin_mcu::register::Register<Ctrlb> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ctrlb, 0x4)
    }

    #[doc="Get the *mut pointer for the CTRLB register."]
    #[inline] pub fn ctrlb_mut(&self) -> *mut Ctrlb { 
        self.ctrlb_reg().ptr()
    }

    #[doc="Get the *const pointer for the CTRLB register."]
    #[inline] pub fn ctrlb_ptr(&self) -> *const Ctrlb { 
        self.ctrlb_reg().ptr()
    }

    #[doc="Read the CTRLB register."]
    #[inline] pub fn ctrlb(&self) -> Ctrlb { 
        self.ctrlb_reg().read()
    }

    #[doc="Write the CTRLB register."]
    #[inline] pub fn write_ctrlb(&self, value: Ctrlb) -> &Self { 
        self.ctrlb_reg().write(value);
        self
    }

    #[doc="Set the CTRLB register."]
    #[inline] pub fn set_ctrlb<F: FnOnce(Ctrlb) -> Ctrlb>(&self, f: F) -> &Self {
        self.ctrlb_reg().set(f);
        self
    }

    #[doc="Modify the CTRLB register."]
    #[inline] pub fn with_ctrlb<F: FnOnce(Ctrlb) -> Ctrlb>(&self, f: F) -> &Self {
        self.ctrlb_reg().with(f);
        self
    }

    #[doc="Get the INTENCLR Register."]
    #[inline] pub fn intenclr_reg(&self) -> ::bobbin_mcu::register::Register<Intenclr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenclr, 0x5)
    }

    #[doc="Get the *mut pointer for the INTENCLR register."]
    #[inline] pub fn intenclr_mut(&self) -> *mut Intenclr { 
        self.intenclr_reg().ptr()
    }

    #[doc="Get the *const pointer for the INTENCLR register."]
    #[inline] pub fn intenclr_ptr(&self) -> *const Intenclr { 
        self.intenclr_reg().ptr()
    }

    #[doc="Read the INTENCLR register."]
    #[inline] pub fn intenclr(&self) -> Intenclr { 
        self.intenclr_reg().read()
    }

    #[doc="Write the INTENCLR register."]
    #[inline] pub fn write_intenclr(&self, value: Intenclr) -> &Self { 
        self.intenclr_reg().write(value);
        self
    }

    #[doc="Set the INTENCLR register."]
    #[inline] pub fn set_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
        self.intenclr_reg().set(f);
        self
    }

    #[doc="Modify the INTENCLR register."]
    #[inline] pub fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
        self.intenclr_reg().with(f);
        self
    }

    #[doc="Get the INTENSET Register."]
    #[inline] pub fn intenset_reg(&self) -> ::bobbin_mcu::register::Register<Intenset> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenset, 0x6)
    }

    #[doc="Get the *mut pointer for the INTENSET register."]
    #[inline] pub fn intenset_mut(&self) -> *mut Intenset { 
        self.intenset_reg().ptr()
    }

    #[doc="Get the *const pointer for the INTENSET register."]
    #[inline] pub fn intenset_ptr(&self) -> *const Intenset { 
        self.intenset_reg().ptr()
    }

    #[doc="Read the INTENSET register."]
    #[inline] pub fn intenset(&self) -> Intenset { 
        self.intenset_reg().read()
    }

    #[doc="Write the INTENSET register."]
    #[inline] pub fn write_intenset(&self, value: Intenset) -> &Self { 
        self.intenset_reg().write(value);
        self
    }

    #[doc="Set the INTENSET register."]
    #[inline] pub fn set_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
        self.intenset_reg().set(f);
        self
    }

    #[doc="Modify the INTENSET register."]
    #[inline] pub fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
        self.intenset_reg().with(f);
        self
    }

    #[doc="Get the INTFLAG Register."]
    #[inline] pub fn intflag_reg(&self) -> ::bobbin_mcu::register::Register<Intflag> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intflag, 0x7)
    }

    #[doc="Get the *mut pointer for the INTFLAG register."]
    #[inline] pub fn intflag_mut(&self) -> *mut Intflag { 
        self.intflag_reg().ptr()
    }

    #[doc="Get the *const pointer for the INTFLAG register."]
    #[inline] pub fn intflag_ptr(&self) -> *const Intflag { 
        self.intflag_reg().ptr()
    }

    #[doc="Read the INTFLAG register."]
    #[inline] pub fn intflag(&self) -> Intflag { 
        self.intflag_reg().read()
    }

    #[doc="Write the INTFLAG register."]
    #[inline] pub fn write_intflag(&self, value: Intflag) -> &Self { 
        self.intflag_reg().write(value);
        self
    }

    #[doc="Set the INTFLAG register."]
    #[inline] pub fn set_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
        self.intflag_reg().set(f);
        self
    }

    #[doc="Modify the INTFLAG register."]
    #[inline] pub fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
        self.intflag_reg().with(f);
        self
    }

    #[doc="Get the DATABUFPTR Register."]
    #[inline] pub fn databufptr_reg(&self) -> ::bobbin_mcu::register::Register<Databufptr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Databufptr, 0x8)
    }

    #[doc="Get the *mut pointer for the DATABUFPTR register."]
    #[inline] pub fn databufptr_mut(&self) -> *mut Databufptr { 
        self.databufptr_reg().ptr()
    }

    #[doc="Get the *const pointer for the DATABUFPTR register."]
    #[inline] pub fn databufptr_ptr(&self) -> *const Databufptr { 
        self.databufptr_reg().ptr()
    }

    #[doc="Read the DATABUFPTR register."]
    #[inline] pub fn databufptr(&self) -> Databufptr { 
        self.databufptr_reg().read()
    }

    #[doc="Write the DATABUFPTR register."]
    #[inline] pub fn write_databufptr(&self, value: Databufptr) -> &Self { 
        self.databufptr_reg().write(value);
        self
    }

    #[doc="Set the DATABUFPTR register."]
    #[inline] pub fn set_databufptr<F: FnOnce(Databufptr) -> Databufptr>(&self, f: F) -> &Self {
        self.databufptr_reg().set(f);
        self
    }

    #[doc="Modify the DATABUFPTR register."]
    #[inline] pub fn with_databufptr<F: FnOnce(Databufptr) -> Databufptr>(&self, f: F) -> &Self {
        self.databufptr_reg().with(f);
        self
    }

    #[doc="Get the DBGCTRL Register."]
    #[inline] pub fn dbgctrl_reg(&self) -> ::bobbin_mcu::register::Register<Dbgctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dbgctrl, 0x9)
    }

    #[doc="Get the *mut pointer for the DBGCTRL register."]
    #[inline] pub fn dbgctrl_mut(&self) -> *mut Dbgctrl { 
        self.dbgctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the DBGCTRL register."]
    #[inline] pub fn dbgctrl_ptr(&self) -> *const Dbgctrl { 
        self.dbgctrl_reg().ptr()
    }

    #[doc="Read the DBGCTRL register."]
    #[inline] pub fn dbgctrl(&self) -> Dbgctrl { 
        self.dbgctrl_reg().read()
    }

    #[doc="Write the DBGCTRL register."]
    #[inline] pub fn write_dbgctrl(&self, value: Dbgctrl) -> &Self { 
        self.dbgctrl_reg().write(value);
        self
    }

    #[doc="Set the DBGCTRL register."]
    #[inline] pub fn set_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
        self.dbgctrl_reg().set(f);
        self
    }

    #[doc="Modify the DBGCTRL register."]
    #[inline] pub fn with_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
        self.dbgctrl_reg().with(f);
        self
    }

    #[doc="Get the KEYWORD Register."]
    #[inline] pub fn keyword_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Keyword, ::bobbin_bits::R8> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Keyword, 0xc, 0x4)
    }

    #[doc="Get the *mut pointer for the KEYWORD register."]
    #[inline] pub fn keyword_mut<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *mut Keyword { 
        self.keyword_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the KEYWORD register."]
    #[inline] pub fn keyword_ptr<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *const Keyword { 
        self.keyword_reg().ptr(index.into())
    }

    #[doc="Write the KEYWORD register."]
    #[inline] pub fn write_keyword<I: Into<::bobbin_bits::R8>>(&self, index: I, value: Keyword) -> &Self {
        self.keyword_reg().write(index.into(), value);
        self
    }

    #[doc="Set the KEYWORD register."]
    #[inline] pub fn set_keyword<I: Into<::bobbin_bits::R8>, F: FnOnce(Keyword) -> Keyword>(&self, index: I, f: F) -> &Self {
        self.keyword_reg().set(index.into(), f);
        self
    }

    #[doc="Get the INDATA Register."]
    #[inline] pub fn indata_reg(&self) -> ::bobbin_mcu::register::Register<Indata> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Indata, 0x38)
    }

    #[doc="Get the *mut pointer for the INDATA register."]
    #[inline] pub fn indata_mut(&self) -> *mut Indata { 
        self.indata_reg().ptr()
    }

    #[doc="Get the *const pointer for the INDATA register."]
    #[inline] pub fn indata_ptr(&self) -> *const Indata { 
        self.indata_reg().ptr()
    }

    #[doc="Read the INDATA register."]
    #[inline] pub fn indata(&self) -> Indata { 
        self.indata_reg().read()
    }

    #[doc="Write the INDATA register."]
    #[inline] pub fn write_indata(&self, value: Indata) -> &Self { 
        self.indata_reg().write(value);
        self
    }

    #[doc="Set the INDATA register."]
    #[inline] pub fn set_indata<F: FnOnce(Indata) -> Indata>(&self, f: F) -> &Self {
        self.indata_reg().set(f);
        self
    }

    #[doc="Modify the INDATA register."]
    #[inline] pub fn with_indata<F: FnOnce(Indata) -> Indata>(&self, f: F) -> &Self {
        self.indata_reg().with(f);
        self
    }

    #[doc="Get the INTVECTV Register."]
    #[inline] pub fn intvectv_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Intvectv, ::bobbin_bits::R4> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Intvectv, 0x3c, 0x4)
    }

    #[doc="Get the *mut pointer for the INTVECTV register."]
    #[inline] pub fn intvectv_mut<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *mut Intvectv { 
        self.intvectv_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the INTVECTV register."]
    #[inline] pub fn intvectv_ptr<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *const Intvectv { 
        self.intvectv_reg().ptr(index.into())
    }

    #[doc="Write the INTVECTV register."]
    #[inline] pub fn write_intvectv<I: Into<::bobbin_bits::R4>>(&self, index: I, value: Intvectv) -> &Self {
        self.intvectv_reg().write(index.into(), value);
        self
    }

    #[doc="Set the INTVECTV register."]
    #[inline] pub fn set_intvectv<I: Into<::bobbin_bits::R4>, F: FnOnce(Intvectv) -> Intvectv>(&self, index: I, f: F) -> &Self {
        self.intvectv_reg().set(index.into(), f);
        self
    }

    #[doc="Get the HASHKEY Register."]
    #[inline] pub fn hashkey_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Hashkey, ::bobbin_bits::R4> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Hashkey, 0x5c, 0x4)
    }

    #[doc="Get the *mut pointer for the HASHKEY register."]
    #[inline] pub fn hashkey_mut<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *mut Hashkey { 
        self.hashkey_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the HASHKEY register."]
    #[inline] pub fn hashkey_ptr<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *const Hashkey { 
        self.hashkey_reg().ptr(index.into())
    }

    #[doc="Read the HASHKEY register."]
    #[inline] pub fn hashkey<I: Into<::bobbin_bits::R4>>(&self, index: I) -> Hashkey { 
        self.hashkey_reg().read(index.into())
    }

    #[doc="Write the HASHKEY register."]
    #[inline] pub fn write_hashkey<I: Into<::bobbin_bits::R4>>(&self, index: I, value: Hashkey) -> &Self {
        self.hashkey_reg().write(index.into(), value);
        self
    }

    #[doc="Set the HASHKEY register."]
    #[inline] pub fn set_hashkey<I: Into<::bobbin_bits::R4>, F: FnOnce(Hashkey) -> Hashkey>(&self, index: I, f: F) -> &Self {
        self.hashkey_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the HASHKEY register."]
    #[inline] pub fn with_hashkey<I: Into<::bobbin_bits::R4> + Copy, F: FnOnce(Hashkey) -> Hashkey>(&self, index: I, f: F) -> &Self {
        self.hashkey_reg().with(index.into(), f);
        self
    }

    #[doc="Get the GHASH Register."]
    #[inline] pub fn ghash_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Ghash, ::bobbin_bits::R4> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Ghash, 0x6c, 0x4)
    }

    #[doc="Get the *mut pointer for the GHASH register."]
    #[inline] pub fn ghash_mut<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *mut Ghash { 
        self.ghash_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the GHASH register."]
    #[inline] pub fn ghash_ptr<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *const Ghash { 
        self.ghash_reg().ptr(index.into())
    }

    #[doc="Read the GHASH register."]
    #[inline] pub fn ghash<I: Into<::bobbin_bits::R4>>(&self, index: I) -> Ghash { 
        self.ghash_reg().read(index.into())
    }

    #[doc="Write the GHASH register."]
    #[inline] pub fn write_ghash<I: Into<::bobbin_bits::R4>>(&self, index: I, value: Ghash) -> &Self {
        self.ghash_reg().write(index.into(), value);
        self
    }

    #[doc="Set the GHASH register."]
    #[inline] pub fn set_ghash<I: Into<::bobbin_bits::R4>, F: FnOnce(Ghash) -> Ghash>(&self, index: I, f: F) -> &Self {
        self.ghash_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the GHASH register."]
    #[inline] pub fn with_ghash<I: Into<::bobbin_bits::R4> + Copy, F: FnOnce(Ghash) -> Ghash>(&self, index: I, f: F) -> &Self {
        self.ghash_reg().with(index.into(), f);
        self
    }

    #[doc="Get the CIPLEN Register."]
    #[inline] pub fn ciplen_reg(&self) -> ::bobbin_mcu::register::Register<Ciplen> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ciplen, 0x80)
    }

    #[doc="Get the *mut pointer for the CIPLEN register."]
    #[inline] pub fn ciplen_mut(&self) -> *mut Ciplen { 
        self.ciplen_reg().ptr()
    }

    #[doc="Get the *const pointer for the CIPLEN register."]
    #[inline] pub fn ciplen_ptr(&self) -> *const Ciplen { 
        self.ciplen_reg().ptr()
    }

    #[doc="Read the CIPLEN register."]
    #[inline] pub fn ciplen(&self) -> Ciplen { 
        self.ciplen_reg().read()
    }

    #[doc="Write the CIPLEN register."]
    #[inline] pub fn write_ciplen(&self, value: Ciplen) -> &Self { 
        self.ciplen_reg().write(value);
        self
    }

    #[doc="Set the CIPLEN register."]
    #[inline] pub fn set_ciplen<F: FnOnce(Ciplen) -> Ciplen>(&self, f: F) -> &Self {
        self.ciplen_reg().set(f);
        self
    }

    #[doc="Modify the CIPLEN register."]
    #[inline] pub fn with_ciplen<F: FnOnce(Ciplen) -> Ciplen>(&self, f: F) -> &Self {
        self.ciplen_reg().with(f);
        self
    }

    #[doc="Get the RANDSEED Register."]
    #[inline] pub fn randseed_reg(&self) -> ::bobbin_mcu::register::Register<Randseed> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Randseed, 0x84)
    }

    #[doc="Get the *mut pointer for the RANDSEED register."]
    #[inline] pub fn randseed_mut(&self) -> *mut Randseed { 
        self.randseed_reg().ptr()
    }

    #[doc="Get the *const pointer for the RANDSEED register."]
    #[inline] pub fn randseed_ptr(&self) -> *const Randseed { 
        self.randseed_reg().ptr()
    }

    #[doc="Read the RANDSEED register."]
    #[inline] pub fn randseed(&self) -> Randseed { 
        self.randseed_reg().read()
    }

    #[doc="Write the RANDSEED register."]
    #[inline] pub fn write_randseed(&self, value: Randseed) -> &Self { 
        self.randseed_reg().write(value);
        self
    }

    #[doc="Set the RANDSEED register."]
    #[inline] pub fn set_randseed<F: FnOnce(Randseed) -> Randseed>(&self, f: F) -> &Self {
        self.randseed_reg().set(f);
        self
    }

    #[doc="Modify the RANDSEED register."]
    #[inline] pub fn with_randseed<F: FnOnce(Randseed) -> Randseed>(&self, f: F) -> &Self {
        self.randseed_reg().with(f);
        self
    }

}

#[doc="Control A"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrla(pub u32);
impl Ctrla {
    #[doc="Software Reset"]
    #[inline] pub fn swrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SWRST != 0"]
    #[inline] pub fn test_swrst(&self) -> bool {
        self.swrst() != 0
    }

    #[doc="Sets the SWRST field."]
    #[inline] pub fn set_swrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Enable"]
    #[inline] pub fn enable(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ENABLE != 0"]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable() != 0
    }

    #[doc="Sets the ENABLE field."]
    #[inline] pub fn set_enable<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="AES Modes of operation"]
    #[inline] pub fn aesmode(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x7) as u8) } // [4:2]
    }

    #[doc="Returns true if AESMODE != 0"]
    #[inline] pub fn test_aesmode(&self) -> bool {
        self.aesmode() != 0
    }

    #[doc="Sets the AESMODE field."]
    #[inline] pub fn set_aesmode<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Cipher Feedback Block Size"]
    #[inline] pub fn cfbs(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x7) as u8) } // [7:5]
    }

    #[doc="Returns true if CFBS != 0"]
    #[inline] pub fn test_cfbs(&self) -> bool {
        self.cfbs() != 0
    }

    #[doc="Sets the CFBS field."]
    #[inline] pub fn set_cfbs<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Encryption Key Size"]
    #[inline] pub fn keysize(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if KEYSIZE != 0"]
    #[inline] pub fn test_keysize(&self) -> bool {
        self.keysize() != 0
    }

    #[doc="Sets the KEYSIZE field."]
    #[inline] pub fn set_keysize<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Cipher Mode"]
    #[inline] pub fn cipher(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CIPHER != 0"]
    #[inline] pub fn test_cipher(&self) -> bool {
        self.cipher() != 0
    }

    #[doc="Sets the CIPHER field."]
    #[inline] pub fn set_cipher<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Start Mode Select"]
    #[inline] pub fn startmode(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if STARTMODE != 0"]
    #[inline] pub fn test_startmode(&self) -> bool {
        self.startmode() != 0
    }

    #[doc="Sets the STARTMODE field."]
    #[inline] pub fn set_startmode<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Last Output Data Mode"]
    #[inline] pub fn lod(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if LOD != 0"]
    #[inline] pub fn test_lod(&self) -> bool {
        self.lod() != 0
    }

    #[doc="Sets the LOD field."]
    #[inline] pub fn set_lod<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Last Key Generation"]
    #[inline] pub fn keygen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if KEYGEN != 0"]
    #[inline] pub fn test_keygen(&self) -> bool {
        self.keygen() != 0
    }

    #[doc="Sets the KEYGEN field."]
    #[inline] pub fn set_keygen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="XOR Key Operation"]
    #[inline] pub fn xorkey(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if XORKEY != 0"]
    #[inline] pub fn test_xorkey(&self) -> bool {
        self.xorkey() != 0
    }

    #[doc="Sets the XORKEY field."]
    #[inline] pub fn set_xorkey<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Counter Measure Type"]
    #[inline] pub fn ctype(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if CTYPE != 0"]
    #[inline] pub fn test_ctype(&self) -> bool {
        self.ctype() != 0
    }

    #[doc="Sets the CTYPE field."]
    #[inline] pub fn set_ctype<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Ctrla {
    #[inline]
    fn from(other: u32) -> Self {
         Ctrla(other)
    }
}

impl ::core::fmt::Display for Ctrla {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrla {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.swrst() != 0 { try!(write!(f, " swrst"))}
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.aesmode() != 0 { try!(write!(f, " aesmode=0x{:x}", self.aesmode()))}
        if self.cfbs() != 0 { try!(write!(f, " cfbs=0x{:x}", self.cfbs()))}
        if self.keysize() != 0 { try!(write!(f, " keysize=0x{:x}", self.keysize()))}
        if self.cipher() != 0 { try!(write!(f, " cipher"))}
        if self.startmode() != 0 { try!(write!(f, " startmode"))}
        if self.lod() != 0 { try!(write!(f, " lod"))}
        if self.keygen() != 0 { try!(write!(f, " keygen"))}
        if self.xorkey() != 0 { try!(write!(f, " xorkey"))}
        if self.ctype() != 0 { try!(write!(f, " ctype=0x{:x}", self.ctype()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control B"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrlb(pub u8);
impl Ctrlb {
    #[doc="Start Encryption/Decryption"]
    #[inline] pub fn start(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if START != 0"]
    #[inline] pub fn test_start(&self) -> bool {
        self.start() != 0
    }

    #[doc="Sets the START field."]
    #[inline] pub fn set_start<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="New message"]
    #[inline] pub fn newmsg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if NEWMSG != 0"]
    #[inline] pub fn test_newmsg(&self) -> bool {
        self.newmsg() != 0
    }

    #[doc="Sets the NEWMSG field."]
    #[inline] pub fn set_newmsg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="End of message"]
    #[inline] pub fn eom(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if EOM != 0"]
    #[inline] pub fn test_eom(&self) -> bool {
        self.eom() != 0
    }

    #[doc="Sets the EOM field."]
    #[inline] pub fn set_eom<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="GF Multiplication"]
    #[inline] pub fn gfmul(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if GFMUL != 0"]
    #[inline] pub fn test_gfmul(&self) -> bool {
        self.gfmul() != 0
    }

    #[doc="Sets the GFMUL field."]
    #[inline] pub fn set_gfmul<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u8> for Ctrlb {
    #[inline]
    fn from(other: u8) -> Self {
         Ctrlb(other)
    }
}

impl ::core::fmt::Display for Ctrlb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrlb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.start() != 0 { try!(write!(f, " start"))}
        if self.newmsg() != 0 { try!(write!(f, " newmsg"))}
        if self.eom() != 0 { try!(write!(f, " eom"))}
        if self.gfmul() != 0 { try!(write!(f, " gfmul"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
    #[doc="Encryption Complete Interrupt Enable"]
    #[inline] pub fn enccmp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ENCCMP != 0"]
    #[inline] pub fn test_enccmp(&self) -> bool {
        self.enccmp() != 0
    }

    #[doc="Sets the ENCCMP field."]
    #[inline] pub fn set_enccmp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="GF Multiplication Complete Interrupt Enable"]
    #[inline] pub fn gfmcmp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if GFMCMP != 0"]
    #[inline] pub fn test_gfmcmp(&self) -> bool {
        self.gfmcmp() != 0
    }

    #[doc="Sets the GFMCMP field."]
    #[inline] pub fn set_gfmcmp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u8> for Intenclr {
    #[inline]
    fn from(other: u8) -> Self {
         Intenclr(other)
    }
}

impl ::core::fmt::Display for Intenclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intenclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.enccmp() != 0 { try!(write!(f, " enccmp"))}
        if self.gfmcmp() != 0 { try!(write!(f, " gfmcmp"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
    #[doc="Encryption Complete Interrupt Enable"]
    #[inline] pub fn enccmp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ENCCMP != 0"]
    #[inline] pub fn test_enccmp(&self) -> bool {
        self.enccmp() != 0
    }

    #[doc="Sets the ENCCMP field."]
    #[inline] pub fn set_enccmp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="GF Multiplication Complete Interrupt Enable"]
    #[inline] pub fn gfmcmp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if GFMCMP != 0"]
    #[inline] pub fn test_gfmcmp(&self) -> bool {
        self.gfmcmp() != 0
    }

    #[doc="Sets the GFMCMP field."]
    #[inline] pub fn set_gfmcmp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u8> for Intenset {
    #[inline]
    fn from(other: u8) -> Self {
         Intenset(other)
    }
}

impl ::core::fmt::Display for Intenset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intenset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.enccmp() != 0 { try!(write!(f, " enccmp"))}
        if self.gfmcmp() != 0 { try!(write!(f, " gfmcmp"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Flag Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
    #[doc="Encryption Complete"]
    #[inline] pub fn enccmp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ENCCMP != 0"]
    #[inline] pub fn test_enccmp(&self) -> bool {
        self.enccmp() != 0
    }

    #[doc="Sets the ENCCMP field."]
    #[inline] pub fn set_enccmp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="GF Multiplication Complete"]
    #[inline] pub fn gfmcmp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if GFMCMP != 0"]
    #[inline] pub fn test_gfmcmp(&self) -> bool {
        self.gfmcmp() != 0
    }

    #[doc="Sets the GFMCMP field."]
    #[inline] pub fn set_gfmcmp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u8> for Intflag {
    #[inline]
    fn from(other: u8) -> Self {
         Intflag(other)
    }
}

impl ::core::fmt::Display for Intflag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intflag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.enccmp() != 0 { try!(write!(f, " enccmp"))}
        if self.gfmcmp() != 0 { try!(write!(f, " gfmcmp"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Data buffer pointer"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Databufptr(pub u8);
impl Databufptr {
    #[doc="Input Data Pointer"]
    #[inline] pub fn indataptr(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if INDATAPTR != 0"]
    #[inline] pub fn test_indataptr(&self) -> bool {
        self.indataptr() != 0
    }

    #[doc="Sets the INDATAPTR field."]
    #[inline] pub fn set_indataptr<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Databufptr {
    #[inline]
    fn from(other: u8) -> Self {
         Databufptr(other)
    }
}

impl ::core::fmt::Display for Databufptr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Databufptr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.indataptr() != 0 { try!(write!(f, " indataptr=0x{:x}", self.indataptr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Debug control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
    #[doc="Debug Run"]
    #[inline] pub fn dbgrun(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DBGRUN != 0"]
    #[inline] pub fn test_dbgrun(&self) -> bool {
        self.dbgrun() != 0
    }

    #[doc="Sets the DBGRUN field."]
    #[inline] pub fn set_dbgrun<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Dbgctrl {
    #[inline]
    fn from(other: u8) -> Self {
         Dbgctrl(other)
    }
}

impl ::core::fmt::Display for Dbgctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dbgctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dbgrun() != 0 { try!(write!(f, " dbgrun"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Keyword n"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Keyword(pub u32);
impl Keyword {
}

impl From<u32> for Keyword {
    #[inline]
    fn from(other: u32) -> Self {
         Keyword(other)
    }
}

impl ::core::fmt::Display for Keyword {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Keyword {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Indata"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Indata(pub u32);
impl Indata {
}

impl From<u32> for Indata {
    #[inline]
    fn from(other: u32) -> Self {
         Indata(other)
    }
}

impl ::core::fmt::Display for Indata {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Indata {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Initialisation Vector n"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intvectv(pub u32);
impl Intvectv {
}

impl From<u32> for Intvectv {
    #[inline]
    fn from(other: u32) -> Self {
         Intvectv(other)
    }
}

impl ::core::fmt::Display for Intvectv {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intvectv {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Hash key n"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hashkey(pub u32);
impl Hashkey {
}

impl From<u32> for Hashkey {
    #[inline]
    fn from(other: u32) -> Self {
         Hashkey(other)
    }
}

impl ::core::fmt::Display for Hashkey {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hashkey {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Galois Hash n"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ghash(pub u32);
impl Ghash {
}

impl From<u32> for Ghash {
    #[inline]
    fn from(other: u32) -> Self {
         Ghash(other)
    }
}

impl ::core::fmt::Display for Ghash {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ghash {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Cipher Length"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ciplen(pub u32);
impl Ciplen {
}

impl From<u32> for Ciplen {
    #[inline]
    fn from(other: u32) -> Self {
         Ciplen(other)
    }
}

impl ::core::fmt::Display for Ciplen {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ciplen {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Random Seed"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Randseed(pub u32);
impl Randseed {
}

impl From<u32> for Randseed {
    #[inline]
    fn from(other: u32) -> Self {
         Randseed(other)
    }
}

impl ::core::fmt::Display for Randseed {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Randseed {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

