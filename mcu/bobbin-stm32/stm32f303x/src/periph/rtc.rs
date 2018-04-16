#[allow(unused_imports)] use ::bobbin_common::*;

#[doc="Real-time clock"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct RtcPeriph(pub usize);
impl RtcPeriph {
    #[doc="Get the TR Register."]
    #[inline] pub fn tr_reg(&self) -> Register<Tr> { 
        Register::new(self.0 as *mut Tr, 0x0)
    }

    #[doc="Get the *mut pointer for the TR register."]
    #[inline] pub fn tr_mut(&self) -> *mut Tr { 
        self.tr_reg().ptr()
    }

    #[doc="Get the *const pointer for the TR register."]
    #[inline] pub fn tr_ptr(&self) -> *const Tr { 
        self.tr_reg().ptr()
    }

    #[doc="Read the TR register."]
    #[inline] pub fn tr(&self) -> Tr { 
        self.tr_reg().read()
    }

    #[doc="Write the TR register."]
    #[inline] pub fn write_tr(&self, value: Tr) -> &Self { 
        self.tr_reg().write(value);
        self
    }

    #[doc="Set the TR register."]
    #[inline] pub fn set_tr<F: FnOnce(Tr) -> Tr>(&self, f: F) -> &Self {
        self.tr_reg().set(f);
        self
    }

    #[doc="Modify the TR register."]
    #[inline] pub fn with_tr<F: FnOnce(Tr) -> Tr>(&self, f: F) -> &Self {
        self.tr_reg().with(f);
        self
    }

    #[doc="Get the DR Register."]
    #[inline] pub fn dr_reg(&self) -> Register<Dr> { 
        Register::new(self.0 as *mut Dr, 0x4)
    }

    #[doc="Get the *mut pointer for the DR register."]
    #[inline] pub fn dr_mut(&self) -> *mut Dr { 
        self.dr_reg().ptr()
    }

    #[doc="Get the *const pointer for the DR register."]
    #[inline] pub fn dr_ptr(&self) -> *const Dr { 
        self.dr_reg().ptr()
    }

    #[doc="Read the DR register."]
    #[inline] pub fn dr(&self) -> Dr { 
        self.dr_reg().read()
    }

    #[doc="Write the DR register."]
    #[inline] pub fn write_dr(&self, value: Dr) -> &Self { 
        self.dr_reg().write(value);
        self
    }

    #[doc="Set the DR register."]
    #[inline] pub fn set_dr<F: FnOnce(Dr) -> Dr>(&self, f: F) -> &Self {
        self.dr_reg().set(f);
        self
    }

    #[doc="Modify the DR register."]
    #[inline] pub fn with_dr<F: FnOnce(Dr) -> Dr>(&self, f: F) -> &Self {
        self.dr_reg().with(f);
        self
    }

    #[doc="Get the CR Register."]
    #[inline] pub fn cr_reg(&self) -> Register<Cr> { 
        Register::new(self.0 as *mut Cr, 0x8)
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

    #[doc="Get the ISR Register."]
    #[inline] pub fn isr_reg(&self) -> Register<Isr> { 
        Register::new(self.0 as *mut Isr, 0xc)
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

    #[doc="Write the ISR register."]
    #[inline] pub fn write_isr(&self, value: Isr) -> &Self { 
        self.isr_reg().write(value);
        self
    }

    #[doc="Set the ISR register."]
    #[inline] pub fn set_isr<F: FnOnce(Isr) -> Isr>(&self, f: F) -> &Self {
        self.isr_reg().set(f);
        self
    }

    #[doc="Modify the ISR register."]
    #[inline] pub fn with_isr<F: FnOnce(Isr) -> Isr>(&self, f: F) -> &Self {
        self.isr_reg().with(f);
        self
    }

    #[doc="Get the PRER Register."]
    #[inline] pub fn prer_reg(&self) -> Register<Prer> { 
        Register::new(self.0 as *mut Prer, 0x10)
    }

    #[doc="Get the *mut pointer for the PRER register."]
    #[inline] pub fn prer_mut(&self) -> *mut Prer { 
        self.prer_reg().ptr()
    }

    #[doc="Get the *const pointer for the PRER register."]
    #[inline] pub fn prer_ptr(&self) -> *const Prer { 
        self.prer_reg().ptr()
    }

    #[doc="Read the PRER register."]
    #[inline] pub fn prer(&self) -> Prer { 
        self.prer_reg().read()
    }

    #[doc="Write the PRER register."]
    #[inline] pub fn write_prer(&self, value: Prer) -> &Self { 
        self.prer_reg().write(value);
        self
    }

    #[doc="Set the PRER register."]
    #[inline] pub fn set_prer<F: FnOnce(Prer) -> Prer>(&self, f: F) -> &Self {
        self.prer_reg().set(f);
        self
    }

    #[doc="Modify the PRER register."]
    #[inline] pub fn with_prer<F: FnOnce(Prer) -> Prer>(&self, f: F) -> &Self {
        self.prer_reg().with(f);
        self
    }

    #[doc="Get the WUTR Register."]
    #[inline] pub fn wutr_reg(&self) -> Register<Wutr> { 
        Register::new(self.0 as *mut Wutr, 0x14)
    }

    #[doc="Get the *mut pointer for the WUTR register."]
    #[inline] pub fn wutr_mut(&self) -> *mut Wutr { 
        self.wutr_reg().ptr()
    }

    #[doc="Get the *const pointer for the WUTR register."]
    #[inline] pub fn wutr_ptr(&self) -> *const Wutr { 
        self.wutr_reg().ptr()
    }

    #[doc="Read the WUTR register."]
    #[inline] pub fn wutr(&self) -> Wutr { 
        self.wutr_reg().read()
    }

    #[doc="Write the WUTR register."]
    #[inline] pub fn write_wutr(&self, value: Wutr) -> &Self { 
        self.wutr_reg().write(value);
        self
    }

    #[doc="Set the WUTR register."]
    #[inline] pub fn set_wutr<F: FnOnce(Wutr) -> Wutr>(&self, f: F) -> &Self {
        self.wutr_reg().set(f);
        self
    }

    #[doc="Modify the WUTR register."]
    #[inline] pub fn with_wutr<F: FnOnce(Wutr) -> Wutr>(&self, f: F) -> &Self {
        self.wutr_reg().with(f);
        self
    }

    #[doc="Get the ALRMAR Register."]
    #[inline] pub fn alrmar_reg(&self) -> Register<Alrmar> { 
        Register::new(self.0 as *mut Alrmar, 0x1c)
    }

    #[doc="Get the *mut pointer for the ALRMAR register."]
    #[inline] pub fn alrmar_mut(&self) -> *mut Alrmar { 
        self.alrmar_reg().ptr()
    }

    #[doc="Get the *const pointer for the ALRMAR register."]
    #[inline] pub fn alrmar_ptr(&self) -> *const Alrmar { 
        self.alrmar_reg().ptr()
    }

    #[doc="Read the ALRMAR register."]
    #[inline] pub fn alrmar(&self) -> Alrmar { 
        self.alrmar_reg().read()
    }

    #[doc="Write the ALRMAR register."]
    #[inline] pub fn write_alrmar(&self, value: Alrmar) -> &Self { 
        self.alrmar_reg().write(value);
        self
    }

    #[doc="Set the ALRMAR register."]
    #[inline] pub fn set_alrmar<F: FnOnce(Alrmar) -> Alrmar>(&self, f: F) -> &Self {
        self.alrmar_reg().set(f);
        self
    }

    #[doc="Modify the ALRMAR register."]
    #[inline] pub fn with_alrmar<F: FnOnce(Alrmar) -> Alrmar>(&self, f: F) -> &Self {
        self.alrmar_reg().with(f);
        self
    }

    #[doc="Get the ALRMBR Register."]
    #[inline] pub fn alrmbr_reg(&self) -> Register<Alrmbr> { 
        Register::new(self.0 as *mut Alrmbr, 0x20)
    }

    #[doc="Get the *mut pointer for the ALRMBR register."]
    #[inline] pub fn alrmbr_mut(&self) -> *mut Alrmbr { 
        self.alrmbr_reg().ptr()
    }

    #[doc="Get the *const pointer for the ALRMBR register."]
    #[inline] pub fn alrmbr_ptr(&self) -> *const Alrmbr { 
        self.alrmbr_reg().ptr()
    }

    #[doc="Read the ALRMBR register."]
    #[inline] pub fn alrmbr(&self) -> Alrmbr { 
        self.alrmbr_reg().read()
    }

    #[doc="Write the ALRMBR register."]
    #[inline] pub fn write_alrmbr(&self, value: Alrmbr) -> &Self { 
        self.alrmbr_reg().write(value);
        self
    }

    #[doc="Set the ALRMBR register."]
    #[inline] pub fn set_alrmbr<F: FnOnce(Alrmbr) -> Alrmbr>(&self, f: F) -> &Self {
        self.alrmbr_reg().set(f);
        self
    }

    #[doc="Modify the ALRMBR register."]
    #[inline] pub fn with_alrmbr<F: FnOnce(Alrmbr) -> Alrmbr>(&self, f: F) -> &Self {
        self.alrmbr_reg().with(f);
        self
    }

    #[doc="Get the WPR Register."]
    #[inline] pub fn wpr_reg(&self) -> Register<Wpr> { 
        Register::new(self.0 as *mut Wpr, 0x24)
    }

    #[doc="Get the *mut pointer for the WPR register."]
    #[inline] pub fn wpr_mut(&self) -> *mut Wpr { 
        self.wpr_reg().ptr()
    }

    #[doc="Get the *const pointer for the WPR register."]
    #[inline] pub fn wpr_ptr(&self) -> *const Wpr { 
        self.wpr_reg().ptr()
    }

    #[doc="Write the WPR register."]
    #[inline] pub fn write_wpr(&self, value: Wpr) -> &Self { 
        self.wpr_reg().write(value);
        self
    }

    #[doc="Set the WPR register."]
    #[inline] pub fn set_wpr<F: FnOnce(Wpr) -> Wpr>(&self, f: F) -> &Self {
        self.wpr_reg().set(f);
        self
    }

    #[doc="Get the SSR Register."]
    #[inline] pub fn ssr_reg(&self) -> Register<Ssr> { 
        Register::new(self.0 as *mut Ssr, 0x28)
    }

    #[doc="Get the *mut pointer for the SSR register."]
    #[inline] pub fn ssr_mut(&self) -> *mut Ssr { 
        self.ssr_reg().ptr()
    }

    #[doc="Get the *const pointer for the SSR register."]
    #[inline] pub fn ssr_ptr(&self) -> *const Ssr { 
        self.ssr_reg().ptr()
    }

    #[doc="Read the SSR register."]
    #[inline] pub fn ssr(&self) -> Ssr { 
        self.ssr_reg().read()
    }

    #[doc="Get the SHIFTR Register."]
    #[inline] pub fn shiftr_reg(&self) -> Register<Shiftr> { 
        Register::new(self.0 as *mut Shiftr, 0x2c)
    }

    #[doc="Get the *mut pointer for the SHIFTR register."]
    #[inline] pub fn shiftr_mut(&self) -> *mut Shiftr { 
        self.shiftr_reg().ptr()
    }

    #[doc="Get the *const pointer for the SHIFTR register."]
    #[inline] pub fn shiftr_ptr(&self) -> *const Shiftr { 
        self.shiftr_reg().ptr()
    }

    #[doc="Write the SHIFTR register."]
    #[inline] pub fn write_shiftr(&self, value: Shiftr) -> &Self { 
        self.shiftr_reg().write(value);
        self
    }

    #[doc="Set the SHIFTR register."]
    #[inline] pub fn set_shiftr<F: FnOnce(Shiftr) -> Shiftr>(&self, f: F) -> &Self {
        self.shiftr_reg().set(f);
        self
    }

    #[doc="Get the TSTR Register."]
    #[inline] pub fn tstr_reg(&self) -> Register<Tstr> { 
        Register::new(self.0 as *mut Tstr, 0x30)
    }

    #[doc="Get the *mut pointer for the TSTR register."]
    #[inline] pub fn tstr_mut(&self) -> *mut Tstr { 
        self.tstr_reg().ptr()
    }

    #[doc="Get the *const pointer for the TSTR register."]
    #[inline] pub fn tstr_ptr(&self) -> *const Tstr { 
        self.tstr_reg().ptr()
    }

    #[doc="Read the TSTR register."]
    #[inline] pub fn tstr(&self) -> Tstr { 
        self.tstr_reg().read()
    }

    #[doc="Get the TSDR Register."]
    #[inline] pub fn tsdr_reg(&self) -> Register<Tsdr> { 
        Register::new(self.0 as *mut Tsdr, 0x34)
    }

    #[doc="Get the *mut pointer for the TSDR register."]
    #[inline] pub fn tsdr_mut(&self) -> *mut Tsdr { 
        self.tsdr_reg().ptr()
    }

    #[doc="Get the *const pointer for the TSDR register."]
    #[inline] pub fn tsdr_ptr(&self) -> *const Tsdr { 
        self.tsdr_reg().ptr()
    }

    #[doc="Read the TSDR register."]
    #[inline] pub fn tsdr(&self) -> Tsdr { 
        self.tsdr_reg().read()
    }

    #[doc="Get the TSSSR Register."]
    #[inline] pub fn tsssr_reg(&self) -> Register<Tsssr> { 
        Register::new(self.0 as *mut Tsssr, 0x38)
    }

    #[doc="Get the *mut pointer for the TSSSR register."]
    #[inline] pub fn tsssr_mut(&self) -> *mut Tsssr { 
        self.tsssr_reg().ptr()
    }

    #[doc="Get the *const pointer for the TSSSR register."]
    #[inline] pub fn tsssr_ptr(&self) -> *const Tsssr { 
        self.tsssr_reg().ptr()
    }

    #[doc="Read the TSSSR register."]
    #[inline] pub fn tsssr(&self) -> Tsssr { 
        self.tsssr_reg().read()
    }

    #[doc="Get the CALR Register."]
    #[inline] pub fn calr_reg(&self) -> Register<Calr> { 
        Register::new(self.0 as *mut Calr, 0x3c)
    }

    #[doc="Get the *mut pointer for the CALR register."]
    #[inline] pub fn calr_mut(&self) -> *mut Calr { 
        self.calr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CALR register."]
    #[inline] pub fn calr_ptr(&self) -> *const Calr { 
        self.calr_reg().ptr()
    }

    #[doc="Read the CALR register."]
    #[inline] pub fn calr(&self) -> Calr { 
        self.calr_reg().read()
    }

    #[doc="Write the CALR register."]
    #[inline] pub fn write_calr(&self, value: Calr) -> &Self { 
        self.calr_reg().write(value);
        self
    }

    #[doc="Set the CALR register."]
    #[inline] pub fn set_calr<F: FnOnce(Calr) -> Calr>(&self, f: F) -> &Self {
        self.calr_reg().set(f);
        self
    }

    #[doc="Modify the CALR register."]
    #[inline] pub fn with_calr<F: FnOnce(Calr) -> Calr>(&self, f: F) -> &Self {
        self.calr_reg().with(f);
        self
    }

    #[doc="Get the TAFCR Register."]
    #[inline] pub fn tafcr_reg(&self) -> Register<Tafcr> { 
        Register::new(self.0 as *mut Tafcr, 0x40)
    }

    #[doc="Get the *mut pointer for the TAFCR register."]
    #[inline] pub fn tafcr_mut(&self) -> *mut Tafcr { 
        self.tafcr_reg().ptr()
    }

    #[doc="Get the *const pointer for the TAFCR register."]
    #[inline] pub fn tafcr_ptr(&self) -> *const Tafcr { 
        self.tafcr_reg().ptr()
    }

    #[doc="Read the TAFCR register."]
    #[inline] pub fn tafcr(&self) -> Tafcr { 
        self.tafcr_reg().read()
    }

    #[doc="Write the TAFCR register."]
    #[inline] pub fn write_tafcr(&self, value: Tafcr) -> &Self { 
        self.tafcr_reg().write(value);
        self
    }

    #[doc="Set the TAFCR register."]
    #[inline] pub fn set_tafcr<F: FnOnce(Tafcr) -> Tafcr>(&self, f: F) -> &Self {
        self.tafcr_reg().set(f);
        self
    }

    #[doc="Modify the TAFCR register."]
    #[inline] pub fn with_tafcr<F: FnOnce(Tafcr) -> Tafcr>(&self, f: F) -> &Self {
        self.tafcr_reg().with(f);
        self
    }

    #[doc="Get the ALRMASSR Register."]
    #[inline] pub fn alrmassr_reg(&self) -> Register<Alrmassr> { 
        Register::new(self.0 as *mut Alrmassr, 0x44)
    }

    #[doc="Get the *mut pointer for the ALRMASSR register."]
    #[inline] pub fn alrmassr_mut(&self) -> *mut Alrmassr { 
        self.alrmassr_reg().ptr()
    }

    #[doc="Get the *const pointer for the ALRMASSR register."]
    #[inline] pub fn alrmassr_ptr(&self) -> *const Alrmassr { 
        self.alrmassr_reg().ptr()
    }

    #[doc="Read the ALRMASSR register."]
    #[inline] pub fn alrmassr(&self) -> Alrmassr { 
        self.alrmassr_reg().read()
    }

    #[doc="Write the ALRMASSR register."]
    #[inline] pub fn write_alrmassr(&self, value: Alrmassr) -> &Self { 
        self.alrmassr_reg().write(value);
        self
    }

    #[doc="Set the ALRMASSR register."]
    #[inline] pub fn set_alrmassr<F: FnOnce(Alrmassr) -> Alrmassr>(&self, f: F) -> &Self {
        self.alrmassr_reg().set(f);
        self
    }

    #[doc="Modify the ALRMASSR register."]
    #[inline] pub fn with_alrmassr<F: FnOnce(Alrmassr) -> Alrmassr>(&self, f: F) -> &Self {
        self.alrmassr_reg().with(f);
        self
    }

    #[doc="Get the ALRMBSSR Register."]
    #[inline] pub fn alrmbssr_reg(&self) -> Register<Alrmbssr> { 
        Register::new(self.0 as *mut Alrmbssr, 0x48)
    }

    #[doc="Get the *mut pointer for the ALRMBSSR register."]
    #[inline] pub fn alrmbssr_mut(&self) -> *mut Alrmbssr { 
        self.alrmbssr_reg().ptr()
    }

    #[doc="Get the *const pointer for the ALRMBSSR register."]
    #[inline] pub fn alrmbssr_ptr(&self) -> *const Alrmbssr { 
        self.alrmbssr_reg().ptr()
    }

    #[doc="Read the ALRMBSSR register."]
    #[inline] pub fn alrmbssr(&self) -> Alrmbssr { 
        self.alrmbssr_reg().read()
    }

    #[doc="Write the ALRMBSSR register."]
    #[inline] pub fn write_alrmbssr(&self, value: Alrmbssr) -> &Self { 
        self.alrmbssr_reg().write(value);
        self
    }

    #[doc="Set the ALRMBSSR register."]
    #[inline] pub fn set_alrmbssr<F: FnOnce(Alrmbssr) -> Alrmbssr>(&self, f: F) -> &Self {
        self.alrmbssr_reg().set(f);
        self
    }

    #[doc="Modify the ALRMBSSR register."]
    #[inline] pub fn with_alrmbssr<F: FnOnce(Alrmbssr) -> Alrmbssr>(&self, f: F) -> &Self {
        self.alrmbssr_reg().with(f);
        self
    }

    #[doc="Get the BKPR Register."]
    #[inline] pub fn bkpr_reg(&self) -> RegisterArray<Bkpr, bits::R32> { 
        RegisterArray::new(self.0 as *mut Bkpr, 0x50, 0x4)
    }

    #[doc="Get the *mut pointer for the BKPR register."]
    #[inline] pub fn bkpr_mut<I: Into<bits::R32>>(&self, index: I) -> *mut Bkpr { 
        self.bkpr_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the BKPR register."]
    #[inline] pub fn bkpr_ptr<I: Into<bits::R32>>(&self, index: I) -> *const Bkpr { 
        self.bkpr_reg().ptr(index.into())
    }

    #[doc="Read the BKPR register."]
    #[inline] pub fn bkpr<I: Into<bits::R32>>(&self, index: I) -> Bkpr { 
        self.bkpr_reg().read(index.into())
    }

    #[doc="Write the BKPR register."]
    #[inline] pub fn write_bkpr<I: Into<bits::R32>>(&self, index: I, value: Bkpr) -> &Self {
        self.bkpr_reg().write(index.into(), value);
        self
    }

    #[doc="Set the BKPR register."]
    #[inline] pub fn set_bkpr<I: Into<bits::R32>, F: FnOnce(Bkpr) -> Bkpr>(&self, index: I, f: F) -> &Self {
        self.bkpr_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the BKPR register."]
    #[inline] pub fn with_bkpr<I: Into<bits::R32> + Copy, F: FnOnce(Bkpr) -> Bkpr>(&self, index: I, f: F) -> &Self {
        self.bkpr_reg().with(index.into(), f);
        self
    }

}

#[doc="time register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tr(pub u32);
impl Tr {
    #[doc="AM/PM notation"]
    #[inline] pub fn pm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if PM != 0"]
    #[inline] pub fn test_pm(&self) -> bool {
        self.pm() != 0
    }

    #[doc="Sets the PM field."]
    #[inline] pub fn set_pm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Hour tens in BCD format"]
    #[inline] pub fn ht(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
    }

    #[doc="Returns true if HT != 0"]
    #[inline] pub fn test_ht(&self) -> bool {
        self.ht() != 0
    }

    #[doc="Sets the HT field."]
    #[inline] pub fn set_ht<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Hour units in BCD format"]
    #[inline] pub fn hu(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if HU != 0"]
    #[inline] pub fn test_hu(&self) -> bool {
        self.hu() != 0
    }

    #[doc="Sets the HU field."]
    #[inline] pub fn set_hu<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Minute tens in BCD format"]
    #[inline] pub fn mnt(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
    }

    #[doc="Returns true if MNT != 0"]
    #[inline] pub fn test_mnt(&self) -> bool {
        self.mnt() != 0
    }

    #[doc="Sets the MNT field."]
    #[inline] pub fn set_mnt<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Minute units in BCD format"]
    #[inline] pub fn mnu(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if MNU != 0"]
    #[inline] pub fn test_mnu(&self) -> bool {
        self.mnu() != 0
    }

    #[doc="Sets the MNU field."]
    #[inline] pub fn set_mnu<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Second tens in BCD format"]
    #[inline] pub fn st(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if ST != 0"]
    #[inline] pub fn test_st(&self) -> bool {
        self.st() != 0
    }

    #[doc="Sets the ST field."]
    #[inline] pub fn set_st<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Second units in BCD format"]
    #[inline] pub fn su(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if SU != 0"]
    #[inline] pub fn test_su(&self) -> bool {
        self.su() != 0
    }

    #[doc="Sets the SU field."]
    #[inline] pub fn set_su<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tr {
    #[inline]
    fn from(other: u32) -> Self {
         Tr(other)
    }
}

impl ::core::fmt::Display for Tr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pm() != 0 { try!(write!(f, " pm"))}
        if self.ht() != 0 { try!(write!(f, " ht=0x{:x}", self.ht()))}
        if self.hu() != 0 { try!(write!(f, " hu=0x{:x}", self.hu()))}
        if self.mnt() != 0 { try!(write!(f, " mnt=0x{:x}", self.mnt()))}
        if self.mnu() != 0 { try!(write!(f, " mnu=0x{:x}", self.mnu()))}
        if self.st() != 0 { try!(write!(f, " st=0x{:x}", self.st()))}
        if self.su() != 0 { try!(write!(f, " su=0x{:x}", self.su()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="date register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr(pub u32);
impl Dr {
    #[doc="Year tens in BCD format"]
    #[inline] pub fn yt(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
    }

    #[doc="Returns true if YT != 0"]
    #[inline] pub fn test_yt(&self) -> bool {
        self.yt() != 0
    }

    #[doc="Sets the YT field."]
    #[inline] pub fn set_yt<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Year units in BCD format"]
    #[inline] pub fn yu(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if YU != 0"]
    #[inline] pub fn test_yu(&self) -> bool {
        self.yu() != 0
    }

    #[doc="Sets the YU field."]
    #[inline] pub fn set_yu<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Week day units"]
    #[inline] pub fn wdu(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x7) as u8) } // [15:13]
    }

    #[doc="Returns true if WDU != 0"]
    #[inline] pub fn test_wdu(&self) -> bool {
        self.wdu() != 0
    }

    #[doc="Sets the WDU field."]
    #[inline] pub fn set_wdu<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Month tens in BCD format"]
    #[inline] pub fn mt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if MT != 0"]
    #[inline] pub fn test_mt(&self) -> bool {
        self.mt() != 0
    }

    #[doc="Sets the MT field."]
    #[inline] pub fn set_mt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Month units in BCD format"]
    #[inline] pub fn mu(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if MU != 0"]
    #[inline] pub fn test_mu(&self) -> bool {
        self.mu() != 0
    }

    #[doc="Sets the MU field."]
    #[inline] pub fn set_mu<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Date tens in BCD format"]
    #[inline] pub fn dt(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if DT != 0"]
    #[inline] pub fn test_dt(&self) -> bool {
        self.dt() != 0
    }

    #[doc="Sets the DT field."]
    #[inline] pub fn set_dt<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Date units in BCD format"]
    #[inline] pub fn du(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if DU != 0"]
    #[inline] pub fn test_du(&self) -> bool {
        self.du() != 0
    }

    #[doc="Sets the DU field."]
    #[inline] pub fn set_du<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dr {
    #[inline]
    fn from(other: u32) -> Self {
         Dr(other)
    }
}

impl ::core::fmt::Display for Dr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.yt() != 0 { try!(write!(f, " yt=0x{:x}", self.yt()))}
        if self.yu() != 0 { try!(write!(f, " yu=0x{:x}", self.yu()))}
        if self.wdu() != 0 { try!(write!(f, " wdu=0x{:x}", self.wdu()))}
        if self.mt() != 0 { try!(write!(f, " mt"))}
        if self.mu() != 0 { try!(write!(f, " mu=0x{:x}", self.mu()))}
        if self.dt() != 0 { try!(write!(f, " dt=0x{:x}", self.dt()))}
        if self.du() != 0 { try!(write!(f, " du=0x{:x}", self.du()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="Wakeup clock selection"]
    #[inline] pub fn wcksel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if WCKSEL != 0"]
    #[inline] pub fn test_wcksel(&self) -> bool {
        self.wcksel() != 0
    }

    #[doc="Sets the WCKSEL field."]
    #[inline] pub fn set_wcksel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Time-stamp event active edge"]
    #[inline] pub fn tsedge(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TSEDGE != 0"]
    #[inline] pub fn test_tsedge(&self) -> bool {
        self.tsedge() != 0
    }

    #[doc="Sets the TSEDGE field."]
    #[inline] pub fn set_tsedge<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Reference clock detection enable (50 or 60 Hz)"]
    #[inline] pub fn refckon(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if REFCKON != 0"]
    #[inline] pub fn test_refckon(&self) -> bool {
        self.refckon() != 0
    }

    #[doc="Sets the REFCKON field."]
    #[inline] pub fn set_refckon<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Bypass the shadow registers"]
    #[inline] pub fn bypshad(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if BYPSHAD != 0"]
    #[inline] pub fn test_bypshad(&self) -> bool {
        self.bypshad() != 0
    }

    #[doc="Sets the BYPSHAD field."]
    #[inline] pub fn set_bypshad<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Hour format"]
    #[inline] pub fn fmt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if FMT != 0"]
    #[inline] pub fn test_fmt(&self) -> bool {
        self.fmt() != 0
    }

    #[doc="Sets the FMT field."]
    #[inline] pub fn set_fmt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Alarm A enable"]
    #[inline] pub fn alrae(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if ALRAE != 0"]
    #[inline] pub fn test_alrae(&self) -> bool {
        self.alrae() != 0
    }

    #[doc="Sets the ALRAE field."]
    #[inline] pub fn set_alrae<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Alarm B enable"]
    #[inline] pub fn alrbe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if ALRBE != 0"]
    #[inline] pub fn test_alrbe(&self) -> bool {
        self.alrbe() != 0
    }

    #[doc="Sets the ALRBE field."]
    #[inline] pub fn set_alrbe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Wakeup timer enable"]
    #[inline] pub fn wute(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if WUTE != 0"]
    #[inline] pub fn test_wute(&self) -> bool {
        self.wute() != 0
    }

    #[doc="Sets the WUTE field."]
    #[inline] pub fn set_wute<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Time stamp enable"]
    #[inline] pub fn tse(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if TSE != 0"]
    #[inline] pub fn test_tse(&self) -> bool {
        self.tse() != 0
    }

    #[doc="Sets the TSE field."]
    #[inline] pub fn set_tse<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Alarm A interrupt enable"]
    #[inline] pub fn alraie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if ALRAIE != 0"]
    #[inline] pub fn test_alraie(&self) -> bool {
        self.alraie() != 0
    }

    #[doc="Sets the ALRAIE field."]
    #[inline] pub fn set_alraie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Alarm B interrupt enable"]
    #[inline] pub fn alrbie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if ALRBIE != 0"]
    #[inline] pub fn test_alrbie(&self) -> bool {
        self.alrbie() != 0
    }

    #[doc="Sets the ALRBIE field."]
    #[inline] pub fn set_alrbie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Wakeup timer interrupt enable"]
    #[inline] pub fn wutie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if WUTIE != 0"]
    #[inline] pub fn test_wutie(&self) -> bool {
        self.wutie() != 0
    }

    #[doc="Sets the WUTIE field."]
    #[inline] pub fn set_wutie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Time-stamp interrupt enable"]
    #[inline] pub fn tsie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if TSIE != 0"]
    #[inline] pub fn test_tsie(&self) -> bool {
        self.tsie() != 0
    }

    #[doc="Sets the TSIE field."]
    #[inline] pub fn set_tsie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Add 1 hour (summer time change)"]
    #[inline] pub fn add1h(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if ADD1H != 0"]
    #[inline] pub fn test_add1h(&self) -> bool {
        self.add1h() != 0
    }

    #[doc="Sets the ADD1H field."]
    #[inline] pub fn set_add1h<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Subtract 1 hour (winter time change)"]
    #[inline] pub fn sub1h(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if SUB1H != 0"]
    #[inline] pub fn test_sub1h(&self) -> bool {
        self.sub1h() != 0
    }

    #[doc="Sets the SUB1H field."]
    #[inline] pub fn set_sub1h<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Backup"]
    #[inline] pub fn bkp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if BKP != 0"]
    #[inline] pub fn test_bkp(&self) -> bool {
        self.bkp() != 0
    }

    #[doc="Sets the BKP field."]
    #[inline] pub fn set_bkp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Calibration output selection"]
    #[inline] pub fn cosel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if COSEL != 0"]
    #[inline] pub fn test_cosel(&self) -> bool {
        self.cosel() != 0
    }

    #[doc="Sets the COSEL field."]
    #[inline] pub fn set_cosel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Output polarity"]
    #[inline] pub fn pol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if POL != 0"]
    #[inline] pub fn test_pol(&self) -> bool {
        self.pol() != 0
    }

    #[doc="Sets the POL field."]
    #[inline] pub fn set_pol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Output selection"]
    #[inline] pub fn osel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x3) as u8) } // [22:21]
    }

    #[doc="Returns true if OSEL != 0"]
    #[inline] pub fn test_osel(&self) -> bool {
        self.osel() != 0
    }

    #[doc="Sets the OSEL field."]
    #[inline] pub fn set_osel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Calibration output enable"]
    #[inline] pub fn coe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if COE != 0"]
    #[inline] pub fn test_coe(&self) -> bool {
        self.coe() != 0
    }

    #[doc="Sets the COE field."]
    #[inline] pub fn set_coe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
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
        if self.wcksel() != 0 { try!(write!(f, " wcksel=0x{:x}", self.wcksel()))}
        if self.tsedge() != 0 { try!(write!(f, " tsedge"))}
        if self.refckon() != 0 { try!(write!(f, " refckon"))}
        if self.bypshad() != 0 { try!(write!(f, " bypshad"))}
        if self.fmt() != 0 { try!(write!(f, " fmt"))}
        if self.alrae() != 0 { try!(write!(f, " alrae"))}
        if self.alrbe() != 0 { try!(write!(f, " alrbe"))}
        if self.wute() != 0 { try!(write!(f, " wute"))}
        if self.tse() != 0 { try!(write!(f, " tse"))}
        if self.alraie() != 0 { try!(write!(f, " alraie"))}
        if self.alrbie() != 0 { try!(write!(f, " alrbie"))}
        if self.wutie() != 0 { try!(write!(f, " wutie"))}
        if self.tsie() != 0 { try!(write!(f, " tsie"))}
        if self.add1h() != 0 { try!(write!(f, " add1h"))}
        if self.sub1h() != 0 { try!(write!(f, " sub1h"))}
        if self.bkp() != 0 { try!(write!(f, " bkp"))}
        if self.cosel() != 0 { try!(write!(f, " cosel"))}
        if self.pol() != 0 { try!(write!(f, " pol"))}
        if self.osel() != 0 { try!(write!(f, " osel=0x{:x}", self.osel()))}
        if self.coe() != 0 { try!(write!(f, " coe"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="initialization and status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Isr(pub u32);
impl Isr {
    #[doc="Alarm A write flag"]
    #[inline] pub fn alrawf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ALRAWF != 0"]
    #[inline] pub fn test_alrawf(&self) -> bool {
        self.alrawf() != 0
    }

    #[doc="Sets the ALRAWF field."]
    #[inline] pub fn set_alrawf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Alarm B write flag"]
    #[inline] pub fn alrbwf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ALRBWF != 0"]
    #[inline] pub fn test_alrbwf(&self) -> bool {
        self.alrbwf() != 0
    }

    #[doc="Sets the ALRBWF field."]
    #[inline] pub fn set_alrbwf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Wakeup timer write flag"]
    #[inline] pub fn wutwf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if WUTWF != 0"]
    #[inline] pub fn test_wutwf(&self) -> bool {
        self.wutwf() != 0
    }

    #[doc="Sets the WUTWF field."]
    #[inline] pub fn set_wutwf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Shift operation pending"]
    #[inline] pub fn shpf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if SHPF != 0"]
    #[inline] pub fn test_shpf(&self) -> bool {
        self.shpf() != 0
    }

    #[doc="Sets the SHPF field."]
    #[inline] pub fn set_shpf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Initialization status flag"]
    #[inline] pub fn inits(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if INITS != 0"]
    #[inline] pub fn test_inits(&self) -> bool {
        self.inits() != 0
    }

    #[doc="Sets the INITS field."]
    #[inline] pub fn set_inits<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Registers synchronization flag"]
    #[inline] pub fn rsf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RSF != 0"]
    #[inline] pub fn test_rsf(&self) -> bool {
        self.rsf() != 0
    }

    #[doc="Sets the RSF field."]
    #[inline] pub fn set_rsf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Initialization flag"]
    #[inline] pub fn initf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if INITF != 0"]
    #[inline] pub fn test_initf(&self) -> bool {
        self.initf() != 0
    }

    #[doc="Sets the INITF field."]
    #[inline] pub fn set_initf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Initialization mode"]
    #[inline] pub fn init(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if INIT != 0"]
    #[inline] pub fn test_init(&self) -> bool {
        self.init() != 0
    }

    #[doc="Sets the INIT field."]
    #[inline] pub fn set_init<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Alarm A flag"]
    #[inline] pub fn alraf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if ALRAF != 0"]
    #[inline] pub fn test_alraf(&self) -> bool {
        self.alraf() != 0
    }

    #[doc="Sets the ALRAF field."]
    #[inline] pub fn set_alraf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Alarm B flag"]
    #[inline] pub fn alrbf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if ALRBF != 0"]
    #[inline] pub fn test_alrbf(&self) -> bool {
        self.alrbf() != 0
    }

    #[doc="Sets the ALRBF field."]
    #[inline] pub fn set_alrbf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Wakeup timer flag"]
    #[inline] pub fn wutf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if WUTF != 0"]
    #[inline] pub fn test_wutf(&self) -> bool {
        self.wutf() != 0
    }

    #[doc="Sets the WUTF field."]
    #[inline] pub fn set_wutf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Time-stamp flag"]
    #[inline] pub fn tsf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if TSF != 0"]
    #[inline] pub fn test_tsf(&self) -> bool {
        self.tsf() != 0
    }

    #[doc="Sets the TSF field."]
    #[inline] pub fn set_tsf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Time-stamp overflow flag"]
    #[inline] pub fn tsovf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if TSOVF != 0"]
    #[inline] pub fn test_tsovf(&self) -> bool {
        self.tsovf() != 0
    }

    #[doc="Sets the TSOVF field."]
    #[inline] pub fn set_tsovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Tamper detection flag"]
    #[inline] pub fn tamp1f(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if TAMP1F != 0"]
    #[inline] pub fn test_tamp1f(&self) -> bool {
        self.tamp1f() != 0
    }

    #[doc="Sets the TAMP1F field."]
    #[inline] pub fn set_tamp1f<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="RTC_TAMP2 detection flag"]
    #[inline] pub fn tamp2f(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TAMP2F != 0"]
    #[inline] pub fn test_tamp2f(&self) -> bool {
        self.tamp2f() != 0
    }

    #[doc="Sets the TAMP2F field."]
    #[inline] pub fn set_tamp2f<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="RTC_TAMP3 detection flag"]
    #[inline] pub fn tamp3f(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if TAMP3F != 0"]
    #[inline] pub fn test_tamp3f(&self) -> bool {
        self.tamp3f() != 0
    }

    #[doc="Sets the TAMP3F field."]
    #[inline] pub fn set_tamp3f<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Recalibration pending Flag"]
    #[inline] pub fn recalpf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if RECALPF != 0"]
    #[inline] pub fn test_recalpf(&self) -> bool {
        self.recalpf() != 0
    }

    #[doc="Sets the RECALPF field."]
    #[inline] pub fn set_recalpf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
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
        if self.alrawf() != 0 { try!(write!(f, " alrawf"))}
        if self.alrbwf() != 0 { try!(write!(f, " alrbwf"))}
        if self.wutwf() != 0 { try!(write!(f, " wutwf"))}
        if self.shpf() != 0 { try!(write!(f, " shpf"))}
        if self.inits() != 0 { try!(write!(f, " inits"))}
        if self.rsf() != 0 { try!(write!(f, " rsf"))}
        if self.initf() != 0 { try!(write!(f, " initf"))}
        if self.init() != 0 { try!(write!(f, " init"))}
        if self.alraf() != 0 { try!(write!(f, " alraf"))}
        if self.alrbf() != 0 { try!(write!(f, " alrbf"))}
        if self.wutf() != 0 { try!(write!(f, " wutf"))}
        if self.tsf() != 0 { try!(write!(f, " tsf"))}
        if self.tsovf() != 0 { try!(write!(f, " tsovf"))}
        if self.tamp1f() != 0 { try!(write!(f, " tamp1f"))}
        if self.tamp2f() != 0 { try!(write!(f, " tamp2f"))}
        if self.tamp3f() != 0 { try!(write!(f, " tamp3f"))}
        if self.recalpf() != 0 { try!(write!(f, " recalpf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="prescaler register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Prer(pub u32);
impl Prer {
    #[doc="Asynchronous prescaler factor"]
    #[inline] pub fn prediv_a(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7f) as u8) } // [22:16]
    }

    #[doc="Returns true if PREDIV_A != 0"]
    #[inline] pub fn test_prediv_a(&self) -> bool {
        self.prediv_a() != 0
    }

    #[doc="Sets the PREDIV_A field."]
    #[inline] pub fn set_prediv_a<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Synchronous prescaler factor"]
    #[inline] pub fn prediv_s(&self) -> bits::U15 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7fff) as u16) } // [14:0]
    }

    #[doc="Returns true if PREDIV_S != 0"]
    #[inline] pub fn test_prediv_s(&self) -> bool {
        self.prediv_s() != 0
    }

    #[doc="Sets the PREDIV_S field."]
    #[inline] pub fn set_prediv_s<V: Into<bits::U15>>(mut self, value: V) -> Self {
        let value: bits::U15 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7fff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Prer {
    #[inline]
    fn from(other: u32) -> Self {
         Prer(other)
    }
}

impl ::core::fmt::Display for Prer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Prer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.prediv_a() != 0 { try!(write!(f, " prediv_a=0x{:x}", self.prediv_a()))}
        if self.prediv_s() != 0 { try!(write!(f, " prediv_s=0x{:x}", self.prediv_s()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="wakeup timer register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Wutr(pub u32);
impl Wutr {
    #[doc="Wakeup auto-reload value bits"]
    #[inline] pub fn wut(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if WUT != 0"]
    #[inline] pub fn test_wut(&self) -> bool {
        self.wut() != 0
    }

    #[doc="Sets the WUT field."]
    #[inline] pub fn set_wut<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Wutr {
    #[inline]
    fn from(other: u32) -> Self {
         Wutr(other)
    }
}

impl ::core::fmt::Display for Wutr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Wutr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wut() != 0 { try!(write!(f, " wut=0x{:x}", self.wut()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="alarm A register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Alrmar(pub u32);
impl Alrmar {
    #[doc="Alarm A date mask"]
    #[inline] pub fn msk4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if MSK4 != 0"]
    #[inline] pub fn test_msk4(&self) -> bool {
        self.msk4() != 0
    }

    #[doc="Sets the MSK4 field."]
    #[inline] pub fn set_msk4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Week day selection"]
    #[inline] pub fn wdsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if WDSEL != 0"]
    #[inline] pub fn test_wdsel(&self) -> bool {
        self.wdsel() != 0
    }

    #[doc="Sets the WDSEL field."]
    #[inline] pub fn set_wdsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Date tens in BCD format"]
    #[inline] pub fn dt(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x3) as u8) } // [29:28]
    }

    #[doc="Returns true if DT != 0"]
    #[inline] pub fn test_dt(&self) -> bool {
        self.dt() != 0
    }

    #[doc="Sets the DT field."]
    #[inline] pub fn set_dt<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Date units or day in BCD format"]
    #[inline] pub fn du(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if DU != 0"]
    #[inline] pub fn test_du(&self) -> bool {
        self.du() != 0
    }

    #[doc="Sets the DU field."]
    #[inline] pub fn set_du<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Alarm A hours mask"]
    #[inline] pub fn msk3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if MSK3 != 0"]
    #[inline] pub fn test_msk3(&self) -> bool {
        self.msk3() != 0
    }

    #[doc="Sets the MSK3 field."]
    #[inline] pub fn set_msk3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="AM/PM notation"]
    #[inline] pub fn pm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if PM != 0"]
    #[inline] pub fn test_pm(&self) -> bool {
        self.pm() != 0
    }

    #[doc="Sets the PM field."]
    #[inline] pub fn set_pm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Hour tens in BCD format"]
    #[inline] pub fn ht(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
    }

    #[doc="Returns true if HT != 0"]
    #[inline] pub fn test_ht(&self) -> bool {
        self.ht() != 0
    }

    #[doc="Sets the HT field."]
    #[inline] pub fn set_ht<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Hour units in BCD format"]
    #[inline] pub fn hu(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if HU != 0"]
    #[inline] pub fn test_hu(&self) -> bool {
        self.hu() != 0
    }

    #[doc="Sets the HU field."]
    #[inline] pub fn set_hu<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Alarm A minutes mask"]
    #[inline] pub fn msk2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if MSK2 != 0"]
    #[inline] pub fn test_msk2(&self) -> bool {
        self.msk2() != 0
    }

    #[doc="Sets the MSK2 field."]
    #[inline] pub fn set_msk2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Minute tens in BCD format"]
    #[inline] pub fn mnt(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
    }

    #[doc="Returns true if MNT != 0"]
    #[inline] pub fn test_mnt(&self) -> bool {
        self.mnt() != 0
    }

    #[doc="Sets the MNT field."]
    #[inline] pub fn set_mnt<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Minute units in BCD format"]
    #[inline] pub fn mnu(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if MNU != 0"]
    #[inline] pub fn test_mnu(&self) -> bool {
        self.mnu() != 0
    }

    #[doc="Sets the MNU field."]
    #[inline] pub fn set_mnu<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Alarm A seconds mask"]
    #[inline] pub fn msk1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if MSK1 != 0"]
    #[inline] pub fn test_msk1(&self) -> bool {
        self.msk1() != 0
    }

    #[doc="Sets the MSK1 field."]
    #[inline] pub fn set_msk1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Second tens in BCD format"]
    #[inline] pub fn st(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if ST != 0"]
    #[inline] pub fn test_st(&self) -> bool {
        self.st() != 0
    }

    #[doc="Sets the ST field."]
    #[inline] pub fn set_st<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Second units in BCD format"]
    #[inline] pub fn su(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if SU != 0"]
    #[inline] pub fn test_su(&self) -> bool {
        self.su() != 0
    }

    #[doc="Sets the SU field."]
    #[inline] pub fn set_su<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Alrmar {
    #[inline]
    fn from(other: u32) -> Self {
         Alrmar(other)
    }
}

impl ::core::fmt::Display for Alrmar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Alrmar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.msk4() != 0 { try!(write!(f, " msk4"))}
        if self.wdsel() != 0 { try!(write!(f, " wdsel"))}
        if self.dt() != 0 { try!(write!(f, " dt=0x{:x}", self.dt()))}
        if self.du() != 0 { try!(write!(f, " du=0x{:x}", self.du()))}
        if self.msk3() != 0 { try!(write!(f, " msk3"))}
        if self.pm() != 0 { try!(write!(f, " pm"))}
        if self.ht() != 0 { try!(write!(f, " ht=0x{:x}", self.ht()))}
        if self.hu() != 0 { try!(write!(f, " hu=0x{:x}", self.hu()))}
        if self.msk2() != 0 { try!(write!(f, " msk2"))}
        if self.mnt() != 0 { try!(write!(f, " mnt=0x{:x}", self.mnt()))}
        if self.mnu() != 0 { try!(write!(f, " mnu=0x{:x}", self.mnu()))}
        if self.msk1() != 0 { try!(write!(f, " msk1"))}
        if self.st() != 0 { try!(write!(f, " st=0x{:x}", self.st()))}
        if self.su() != 0 { try!(write!(f, " su=0x{:x}", self.su()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="alarm B register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Alrmbr(pub u32);
impl Alrmbr {
    #[doc="Alarm B date mask"]
    #[inline] pub fn msk4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if MSK4 != 0"]
    #[inline] pub fn test_msk4(&self) -> bool {
        self.msk4() != 0
    }

    #[doc="Sets the MSK4 field."]
    #[inline] pub fn set_msk4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Week day selection"]
    #[inline] pub fn wdsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if WDSEL != 0"]
    #[inline] pub fn test_wdsel(&self) -> bool {
        self.wdsel() != 0
    }

    #[doc="Sets the WDSEL field."]
    #[inline] pub fn set_wdsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Date tens in BCD format"]
    #[inline] pub fn dt(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x3) as u8) } // [29:28]
    }

    #[doc="Returns true if DT != 0"]
    #[inline] pub fn test_dt(&self) -> bool {
        self.dt() != 0
    }

    #[doc="Sets the DT field."]
    #[inline] pub fn set_dt<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Date units or day in BCD format"]
    #[inline] pub fn du(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if DU != 0"]
    #[inline] pub fn test_du(&self) -> bool {
        self.du() != 0
    }

    #[doc="Sets the DU field."]
    #[inline] pub fn set_du<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Alarm B hours mask"]
    #[inline] pub fn msk3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if MSK3 != 0"]
    #[inline] pub fn test_msk3(&self) -> bool {
        self.msk3() != 0
    }

    #[doc="Sets the MSK3 field."]
    #[inline] pub fn set_msk3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="AM/PM notation"]
    #[inline] pub fn pm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if PM != 0"]
    #[inline] pub fn test_pm(&self) -> bool {
        self.pm() != 0
    }

    #[doc="Sets the PM field."]
    #[inline] pub fn set_pm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Hour tens in BCD format"]
    #[inline] pub fn ht(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
    }

    #[doc="Returns true if HT != 0"]
    #[inline] pub fn test_ht(&self) -> bool {
        self.ht() != 0
    }

    #[doc="Sets the HT field."]
    #[inline] pub fn set_ht<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Hour units in BCD format"]
    #[inline] pub fn hu(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if HU != 0"]
    #[inline] pub fn test_hu(&self) -> bool {
        self.hu() != 0
    }

    #[doc="Sets the HU field."]
    #[inline] pub fn set_hu<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Alarm B minutes mask"]
    #[inline] pub fn msk2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if MSK2 != 0"]
    #[inline] pub fn test_msk2(&self) -> bool {
        self.msk2() != 0
    }

    #[doc="Sets the MSK2 field."]
    #[inline] pub fn set_msk2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Minute tens in BCD format"]
    #[inline] pub fn mnt(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
    }

    #[doc="Returns true if MNT != 0"]
    #[inline] pub fn test_mnt(&self) -> bool {
        self.mnt() != 0
    }

    #[doc="Sets the MNT field."]
    #[inline] pub fn set_mnt<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Minute units in BCD format"]
    #[inline] pub fn mnu(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if MNU != 0"]
    #[inline] pub fn test_mnu(&self) -> bool {
        self.mnu() != 0
    }

    #[doc="Sets the MNU field."]
    #[inline] pub fn set_mnu<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Alarm B seconds mask"]
    #[inline] pub fn msk1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if MSK1 != 0"]
    #[inline] pub fn test_msk1(&self) -> bool {
        self.msk1() != 0
    }

    #[doc="Sets the MSK1 field."]
    #[inline] pub fn set_msk1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Second tens in BCD format"]
    #[inline] pub fn st(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if ST != 0"]
    #[inline] pub fn test_st(&self) -> bool {
        self.st() != 0
    }

    #[doc="Sets the ST field."]
    #[inline] pub fn set_st<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Second units in BCD format"]
    #[inline] pub fn su(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if SU != 0"]
    #[inline] pub fn test_su(&self) -> bool {
        self.su() != 0
    }

    #[doc="Sets the SU field."]
    #[inline] pub fn set_su<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Alrmbr {
    #[inline]
    fn from(other: u32) -> Self {
         Alrmbr(other)
    }
}

impl ::core::fmt::Display for Alrmbr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Alrmbr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.msk4() != 0 { try!(write!(f, " msk4"))}
        if self.wdsel() != 0 { try!(write!(f, " wdsel"))}
        if self.dt() != 0 { try!(write!(f, " dt=0x{:x}", self.dt()))}
        if self.du() != 0 { try!(write!(f, " du=0x{:x}", self.du()))}
        if self.msk3() != 0 { try!(write!(f, " msk3"))}
        if self.pm() != 0 { try!(write!(f, " pm"))}
        if self.ht() != 0 { try!(write!(f, " ht=0x{:x}", self.ht()))}
        if self.hu() != 0 { try!(write!(f, " hu=0x{:x}", self.hu()))}
        if self.msk2() != 0 { try!(write!(f, " msk2"))}
        if self.mnt() != 0 { try!(write!(f, " mnt=0x{:x}", self.mnt()))}
        if self.mnu() != 0 { try!(write!(f, " mnu=0x{:x}", self.mnu()))}
        if self.msk1() != 0 { try!(write!(f, " msk1"))}
        if self.st() != 0 { try!(write!(f, " st=0x{:x}", self.st()))}
        if self.su() != 0 { try!(write!(f, " su=0x{:x}", self.su()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="write protection register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Wpr(pub u32);
impl Wpr {
    #[doc="Write protection key"]
    #[inline] pub fn key(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if KEY != 0"]
    #[inline] pub fn test_key(&self) -> bool {
        self.key() != 0
    }

    #[doc="Sets the KEY field."]
    #[inline] pub fn set_key<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Wpr {
    #[inline]
    fn from(other: u32) -> Self {
         Wpr(other)
    }
}

impl ::core::fmt::Display for Wpr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Wpr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.key() != 0 { try!(write!(f, " key=0x{:x}", self.key()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="sub second register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ssr(pub u32);
impl Ssr {
    #[doc="Sub second value"]
    #[inline] pub fn ss(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if SS != 0"]
    #[inline] pub fn test_ss(&self) -> bool {
        self.ss() != 0
    }

    #[doc="Sets the SS field."]
    #[inline] pub fn set_ss<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ssr {
    #[inline]
    fn from(other: u32) -> Self {
         Ssr(other)
    }
}

impl ::core::fmt::Display for Ssr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ssr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ss() != 0 { try!(write!(f, " ss=0x{:x}", self.ss()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="shift control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Shiftr(pub u32);
impl Shiftr {
    #[doc="Add one second"]
    #[inline] pub fn add1s(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if ADD1S != 0"]
    #[inline] pub fn test_add1s(&self) -> bool {
        self.add1s() != 0
    }

    #[doc="Sets the ADD1S field."]
    #[inline] pub fn set_add1s<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Subtract a fraction of a second"]
    #[inline] pub fn subfs(&self) -> bits::U15 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7fff) as u16) } // [14:0]
    }

    #[doc="Returns true if SUBFS != 0"]
    #[inline] pub fn test_subfs(&self) -> bool {
        self.subfs() != 0
    }

    #[doc="Sets the SUBFS field."]
    #[inline] pub fn set_subfs<V: Into<bits::U15>>(mut self, value: V) -> Self {
        let value: bits::U15 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7fff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Shiftr {
    #[inline]
    fn from(other: u32) -> Self {
         Shiftr(other)
    }
}

impl ::core::fmt::Display for Shiftr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Shiftr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.add1s() != 0 { try!(write!(f, " add1s"))}
        if self.subfs() != 0 { try!(write!(f, " subfs=0x{:x}", self.subfs()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="time stamp time register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tstr(pub u32);
impl Tstr {
    #[doc="Second units in BCD format"]
    #[inline] pub fn su(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if SU != 0"]
    #[inline] pub fn test_su(&self) -> bool {
        self.su() != 0
    }

    #[doc="Sets the SU field."]
    #[inline] pub fn set_su<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Second tens in BCD format"]
    #[inline] pub fn st(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if ST != 0"]
    #[inline] pub fn test_st(&self) -> bool {
        self.st() != 0
    }

    #[doc="Sets the ST field."]
    #[inline] pub fn set_st<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Minute units in BCD format"]
    #[inline] pub fn mnu(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if MNU != 0"]
    #[inline] pub fn test_mnu(&self) -> bool {
        self.mnu() != 0
    }

    #[doc="Sets the MNU field."]
    #[inline] pub fn set_mnu<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Minute tens in BCD format"]
    #[inline] pub fn mnt(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
    }

    #[doc="Returns true if MNT != 0"]
    #[inline] pub fn test_mnt(&self) -> bool {
        self.mnt() != 0
    }

    #[doc="Sets the MNT field."]
    #[inline] pub fn set_mnt<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Hour units in BCD format"]
    #[inline] pub fn hu(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if HU != 0"]
    #[inline] pub fn test_hu(&self) -> bool {
        self.hu() != 0
    }

    #[doc="Sets the HU field."]
    #[inline] pub fn set_hu<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Hour tens in BCD format"]
    #[inline] pub fn ht(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
    }

    #[doc="Returns true if HT != 0"]
    #[inline] pub fn test_ht(&self) -> bool {
        self.ht() != 0
    }

    #[doc="Sets the HT field."]
    #[inline] pub fn set_ht<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="AM/PM notation"]
    #[inline] pub fn pm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if PM != 0"]
    #[inline] pub fn test_pm(&self) -> bool {
        self.pm() != 0
    }

    #[doc="Sets the PM field."]
    #[inline] pub fn set_pm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

}

impl From<u32> for Tstr {
    #[inline]
    fn from(other: u32) -> Self {
         Tstr(other)
    }
}

impl ::core::fmt::Display for Tstr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tstr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.su() != 0 { try!(write!(f, " su=0x{:x}", self.su()))}
        if self.st() != 0 { try!(write!(f, " st=0x{:x}", self.st()))}
        if self.mnu() != 0 { try!(write!(f, " mnu=0x{:x}", self.mnu()))}
        if self.mnt() != 0 { try!(write!(f, " mnt=0x{:x}", self.mnt()))}
        if self.hu() != 0 { try!(write!(f, " hu=0x{:x}", self.hu()))}
        if self.ht() != 0 { try!(write!(f, " ht=0x{:x}", self.ht()))}
        if self.pm() != 0 { try!(write!(f, " pm"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="time stamp date register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tsdr(pub u32);
impl Tsdr {
    #[doc="Week day units"]
    #[inline] pub fn wdu(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x7) as u8) } // [15:13]
    }

    #[doc="Returns true if WDU != 0"]
    #[inline] pub fn test_wdu(&self) -> bool {
        self.wdu() != 0
    }

    #[doc="Sets the WDU field."]
    #[inline] pub fn set_wdu<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Month tens in BCD format"]
    #[inline] pub fn mt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if MT != 0"]
    #[inline] pub fn test_mt(&self) -> bool {
        self.mt() != 0
    }

    #[doc="Sets the MT field."]
    #[inline] pub fn set_mt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Month units in BCD format"]
    #[inline] pub fn mu(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if MU != 0"]
    #[inline] pub fn test_mu(&self) -> bool {
        self.mu() != 0
    }

    #[doc="Sets the MU field."]
    #[inline] pub fn set_mu<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Date tens in BCD format"]
    #[inline] pub fn dt(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if DT != 0"]
    #[inline] pub fn test_dt(&self) -> bool {
        self.dt() != 0
    }

    #[doc="Sets the DT field."]
    #[inline] pub fn set_dt<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Date units in BCD format"]
    #[inline] pub fn du(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if DU != 0"]
    #[inline] pub fn test_du(&self) -> bool {
        self.du() != 0
    }

    #[doc="Sets the DU field."]
    #[inline] pub fn set_du<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tsdr {
    #[inline]
    fn from(other: u32) -> Self {
         Tsdr(other)
    }
}

impl ::core::fmt::Display for Tsdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tsdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wdu() != 0 { try!(write!(f, " wdu=0x{:x}", self.wdu()))}
        if self.mt() != 0 { try!(write!(f, " mt"))}
        if self.mu() != 0 { try!(write!(f, " mu=0x{:x}", self.mu()))}
        if self.dt() != 0 { try!(write!(f, " dt=0x{:x}", self.dt()))}
        if self.du() != 0 { try!(write!(f, " du=0x{:x}", self.du()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="timestamp sub second register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tsssr(pub u32);
impl Tsssr {
    #[doc="Sub second value"]
    #[inline] pub fn ss(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if SS != 0"]
    #[inline] pub fn test_ss(&self) -> bool {
        self.ss() != 0
    }

    #[doc="Sets the SS field."]
    #[inline] pub fn set_ss<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tsssr {
    #[inline]
    fn from(other: u32) -> Self {
         Tsssr(other)
    }
}

impl ::core::fmt::Display for Tsssr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tsssr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ss() != 0 { try!(write!(f, " ss=0x{:x}", self.ss()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="calibration register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Calr(pub u32);
impl Calr {
    #[doc="Increase frequency of RTC by 488.5 ppm"]
    #[inline] pub fn calp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CALP != 0"]
    #[inline] pub fn test_calp(&self) -> bool {
        self.calp() != 0
    }

    #[doc="Sets the CALP field."]
    #[inline] pub fn set_calp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Use an 8-second calibration cycle period"]
    #[inline] pub fn calw8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if CALW8 != 0"]
    #[inline] pub fn test_calw8(&self) -> bool {
        self.calw8() != 0
    }

    #[doc="Sets the CALW8 field."]
    #[inline] pub fn set_calw8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Use a 16-second calibration cycle period"]
    #[inline] pub fn calw16(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if CALW16 != 0"]
    #[inline] pub fn test_calw16(&self) -> bool {
        self.calw16() != 0
    }

    #[doc="Sets the CALW16 field."]
    #[inline] pub fn set_calw16<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Calibration minus"]
    #[inline] pub fn calm(&self) -> bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
    }

    #[doc="Returns true if CALM != 0"]
    #[inline] pub fn test_calm(&self) -> bool {
        self.calm() != 0
    }

    #[doc="Sets the CALM field."]
    #[inline] pub fn set_calm<V: Into<bits::U9>>(mut self, value: V) -> Self {
        let value: bits::U9 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Calr {
    #[inline]
    fn from(other: u32) -> Self {
         Calr(other)
    }
}

impl ::core::fmt::Display for Calr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Calr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.calp() != 0 { try!(write!(f, " calp"))}
        if self.calw8() != 0 { try!(write!(f, " calw8"))}
        if self.calw16() != 0 { try!(write!(f, " calw16"))}
        if self.calm() != 0 { try!(write!(f, " calm=0x{:x}", self.calm()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="tamper and alternate function configuration register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tafcr(pub u32);
impl Tafcr {
    #[doc="Tamper 1 detection enable"]
    #[inline] pub fn tamp1e(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TAMP1E != 0"]
    #[inline] pub fn test_tamp1e(&self) -> bool {
        self.tamp1e() != 0
    }

    #[doc="Sets the TAMP1E field."]
    #[inline] pub fn set_tamp1e<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Active level for tamper 1"]
    #[inline] pub fn tamp1trg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TAMP1TRG != 0"]
    #[inline] pub fn test_tamp1trg(&self) -> bool {
        self.tamp1trg() != 0
    }

    #[doc="Sets the TAMP1TRG field."]
    #[inline] pub fn set_tamp1trg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Tamper interrupt enable"]
    #[inline] pub fn tampie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TAMPIE != 0"]
    #[inline] pub fn test_tampie(&self) -> bool {
        self.tampie() != 0
    }

    #[doc="Sets the TAMPIE field."]
    #[inline] pub fn set_tampie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Tamper 2 detection enable"]
    #[inline] pub fn tamp2e(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TAMP2E != 0"]
    #[inline] pub fn test_tamp2e(&self) -> bool {
        self.tamp2e() != 0
    }

    #[doc="Sets the TAMP2E field."]
    #[inline] pub fn set_tamp2e<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Active level for tamper 2"]
    #[inline] pub fn tamp2trg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TAMP2TRG != 0"]
    #[inline] pub fn test_tamp2trg(&self) -> bool {
        self.tamp2trg() != 0
    }

    #[doc="Sets the TAMP2TRG field."]
    #[inline] pub fn set_tamp2trg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Tamper 3 detection enable"]
    #[inline] pub fn tamp3e(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TAMP3E != 0"]
    #[inline] pub fn test_tamp3e(&self) -> bool {
        self.tamp3e() != 0
    }

    #[doc="Sets the TAMP3E field."]
    #[inline] pub fn set_tamp3e<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Active level for tamper 3"]
    #[inline] pub fn tamp3trg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TAMP3TRG != 0"]
    #[inline] pub fn test_tamp3trg(&self) -> bool {
        self.tamp3trg() != 0
    }

    #[doc="Sets the TAMP3TRG field."]
    #[inline] pub fn set_tamp3trg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Activate timestamp on tamper detection event"]
    #[inline] pub fn tampts(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TAMPTS != 0"]
    #[inline] pub fn test_tampts(&self) -> bool {
        self.tampts() != 0
    }

    #[doc="Sets the TAMPTS field."]
    #[inline] pub fn set_tampts<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Tamper sampling frequency"]
    #[inline] pub fn tampfreq(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if TAMPFREQ != 0"]
    #[inline] pub fn test_tampfreq(&self) -> bool {
        self.tampfreq() != 0
    }

    #[doc="Sets the TAMPFREQ field."]
    #[inline] pub fn set_tampfreq<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Tamper filter count"]
    #[inline] pub fn tampflt(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x3) as u8) } // [12:11]
    }

    #[doc="Returns true if TAMPFLT != 0"]
    #[inline] pub fn test_tampflt(&self) -> bool {
        self.tampflt() != 0
    }

    #[doc="Sets the TAMPFLT field."]
    #[inline] pub fn set_tampflt<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Tamper precharge duration"]
    #[inline] pub fn tampprch(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x3) as u8) } // [14:13]
    }

    #[doc="Returns true if TAMPPRCH != 0"]
    #[inline] pub fn test_tampprch(&self) -> bool {
        self.tampprch() != 0
    }

    #[doc="Sets the TAMPPRCH field."]
    #[inline] pub fn set_tampprch<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="TAMPER pull-up disable"]
    #[inline] pub fn tamppudis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if TAMPPUDIS != 0"]
    #[inline] pub fn test_tamppudis(&self) -> bool {
        self.tamppudis() != 0
    }

    #[doc="Sets the TAMPPUDIS field."]
    #[inline] pub fn set_tamppudis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="PC13 value"]
    #[inline] pub fn pc13value(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if PC13VALUE != 0"]
    #[inline] pub fn test_pc13value(&self) -> bool {
        self.pc13value() != 0
    }

    #[doc="Sets the PC13VALUE field."]
    #[inline] pub fn set_pc13value<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="PC13 mode"]
    #[inline] pub fn pc13mode(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if PC13MODE != 0"]
    #[inline] pub fn test_pc13mode(&self) -> bool {
        self.pc13mode() != 0
    }

    #[doc="Sets the PC13MODE field."]
    #[inline] pub fn set_pc13mode<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="PC14 value"]
    #[inline] pub fn pc14value(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PC14VALUE != 0"]
    #[inline] pub fn test_pc14value(&self) -> bool {
        self.pc14value() != 0
    }

    #[doc="Sets the PC14VALUE field."]
    #[inline] pub fn set_pc14value<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="PC 14 mode"]
    #[inline] pub fn pc14mode(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if PC14MODE != 0"]
    #[inline] pub fn test_pc14mode(&self) -> bool {
        self.pc14mode() != 0
    }

    #[doc="Sets the PC14MODE field."]
    #[inline] pub fn set_pc14mode<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="PC15 value"]
    #[inline] pub fn pc15value(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if PC15VALUE != 0"]
    #[inline] pub fn test_pc15value(&self) -> bool {
        self.pc15value() != 0
    }

    #[doc="Sets the PC15VALUE field."]
    #[inline] pub fn set_pc15value<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="PC15 mode"]
    #[inline] pub fn pc15mode(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if PC15MODE != 0"]
    #[inline] pub fn test_pc15mode(&self) -> bool {
        self.pc15mode() != 0
    }

    #[doc="Sets the PC15MODE field."]
    #[inline] pub fn set_pc15mode<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

}

impl From<u32> for Tafcr {
    #[inline]
    fn from(other: u32) -> Self {
         Tafcr(other)
    }
}

impl ::core::fmt::Display for Tafcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tafcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tamp1e() != 0 { try!(write!(f, " tamp1e"))}
        if self.tamp1trg() != 0 { try!(write!(f, " tamp1trg"))}
        if self.tampie() != 0 { try!(write!(f, " tampie"))}
        if self.tamp2e() != 0 { try!(write!(f, " tamp2e"))}
        if self.tamp2trg() != 0 { try!(write!(f, " tamp2trg"))}
        if self.tamp3e() != 0 { try!(write!(f, " tamp3e"))}
        if self.tamp3trg() != 0 { try!(write!(f, " tamp3trg"))}
        if self.tampts() != 0 { try!(write!(f, " tampts"))}
        if self.tampfreq() != 0 { try!(write!(f, " tampfreq=0x{:x}", self.tampfreq()))}
        if self.tampflt() != 0 { try!(write!(f, " tampflt=0x{:x}", self.tampflt()))}
        if self.tampprch() != 0 { try!(write!(f, " tampprch=0x{:x}", self.tampprch()))}
        if self.tamppudis() != 0 { try!(write!(f, " tamppudis"))}
        if self.pc13value() != 0 { try!(write!(f, " pc13value"))}
        if self.pc13mode() != 0 { try!(write!(f, " pc13mode"))}
        if self.pc14value() != 0 { try!(write!(f, " pc14value"))}
        if self.pc14mode() != 0 { try!(write!(f, " pc14mode"))}
        if self.pc15value() != 0 { try!(write!(f, " pc15value"))}
        if self.pc15mode() != 0 { try!(write!(f, " pc15mode"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="alarm A sub second register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Alrmassr(pub u32);
impl Alrmassr {
    #[doc="Mask the most-significant bits starting at this bit"]
    #[inline] pub fn maskss(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if MASKSS != 0"]
    #[inline] pub fn test_maskss(&self) -> bool {
        self.maskss() != 0
    }

    #[doc="Sets the MASKSS field."]
    #[inline] pub fn set_maskss<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Sub seconds value"]
    #[inline] pub fn ss(&self) -> bits::U15 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7fff) as u16) } // [14:0]
    }

    #[doc="Returns true if SS != 0"]
    #[inline] pub fn test_ss(&self) -> bool {
        self.ss() != 0
    }

    #[doc="Sets the SS field."]
    #[inline] pub fn set_ss<V: Into<bits::U15>>(mut self, value: V) -> Self {
        let value: bits::U15 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7fff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Alrmassr {
    #[inline]
    fn from(other: u32) -> Self {
         Alrmassr(other)
    }
}

impl ::core::fmt::Display for Alrmassr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Alrmassr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.maskss() != 0 { try!(write!(f, " maskss=0x{:x}", self.maskss()))}
        if self.ss() != 0 { try!(write!(f, " ss=0x{:x}", self.ss()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="alarm B sub second register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Alrmbssr(pub u32);
impl Alrmbssr {
    #[doc="Mask the most-significant bits starting at this bit"]
    #[inline] pub fn maskss(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if MASKSS != 0"]
    #[inline] pub fn test_maskss(&self) -> bool {
        self.maskss() != 0
    }

    #[doc="Sets the MASKSS field."]
    #[inline] pub fn set_maskss<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Sub seconds value"]
    #[inline] pub fn ss(&self) -> bits::U15 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7fff) as u16) } // [14:0]
    }

    #[doc="Returns true if SS != 0"]
    #[inline] pub fn test_ss(&self) -> bool {
        self.ss() != 0
    }

    #[doc="Sets the SS field."]
    #[inline] pub fn set_ss<V: Into<bits::U15>>(mut self, value: V) -> Self {
        let value: bits::U15 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7fff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Alrmbssr {
    #[inline]
    fn from(other: u32) -> Self {
         Alrmbssr(other)
    }
}

impl ::core::fmt::Display for Alrmbssr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Alrmbssr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.maskss() != 0 { try!(write!(f, " maskss=0x{:x}", self.maskss()))}
        if self.ss() != 0 { try!(write!(f, " ss=0x{:x}", self.ss()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="backup register n"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bkpr(pub u32);
impl Bkpr {
    #[doc="BKP"]
    #[inline] pub fn bkp(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if BKP != 0"]
    #[inline] pub fn test_bkp(&self) -> bool {
        self.bkp() != 0
    }

    #[doc="Sets the BKP field."]
    #[inline] pub fn set_bkp<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Bkpr {
    #[inline]
    fn from(other: u32) -> Self {
         Bkpr(other)
    }
}

impl ::core::fmt::Display for Bkpr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bkpr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

