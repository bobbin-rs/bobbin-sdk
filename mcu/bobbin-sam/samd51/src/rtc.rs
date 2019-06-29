::bobbin_mcu::periph!( RTC, Rtc, RTC_PERIPH, RtcPeriph, RTC_OWNED, RTC_REF_COUNT, 0x40002400, 0x00, 0x1d);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="RTC Peripheral"]
pub struct RtcPeriph(pub usize); 

impl RtcPeriph {
    #[doc="Get 32-bit Counter with Single 32-bit Compare Peripheral"]
    #[inline] pub fn mode0(&self) -> mode0::Mode0 {
        mode0::Mode0(self.0 + 0x0)
    }
    #[doc="Get 16-bit Counter with Two 16-bit Compares Peripheral"]
    #[inline] pub fn mode1(&self) -> mode1::Mode1 {
        mode1::Mode1(self.0 + 0x0)
    }
    #[doc="Get Clock/Calendar with Alarm Peripheral"]
    #[inline] pub fn mode2(&self) -> mode2::Mode2 {
        mode2::Mode2(self.0 + 0x0)
    }
}

#[doc="32-bit Counter with Single 32-bit Compare Cluster"]
pub mod mode0 {
    #[allow(unused_imports)] use super::*;
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[doc="32-bit Counter with Single 32-bit Compare Peripheral"]
    pub struct Mode0(pub usize);
impl Mode0 {
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ctrlb, 0x2)
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

    #[doc="Get the EVCTRL Register."]
    #[inline] pub fn evctrl_reg(&self) -> ::bobbin_mcu::register::Register<Evctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Evctrl, 0x4)
    }

    #[doc="Get the *mut pointer for the EVCTRL register."]
    #[inline] pub fn evctrl_mut(&self) -> *mut Evctrl { 
        self.evctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the EVCTRL register."]
    #[inline] pub fn evctrl_ptr(&self) -> *const Evctrl { 
        self.evctrl_reg().ptr()
    }

    #[doc="Read the EVCTRL register."]
    #[inline] pub fn evctrl(&self) -> Evctrl { 
        self.evctrl_reg().read()
    }

    #[doc="Write the EVCTRL register."]
    #[inline] pub fn write_evctrl(&self, value: Evctrl) -> &Self { 
        self.evctrl_reg().write(value);
        self
    }

    #[doc="Set the EVCTRL register."]
    #[inline] pub fn set_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
        self.evctrl_reg().set(f);
        self
    }

    #[doc="Modify the EVCTRL register."]
    #[inline] pub fn with_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
        self.evctrl_reg().with(f);
        self
    }

    #[doc="Get the INTENCLR Register."]
    #[inline] pub fn intenclr_reg(&self) -> ::bobbin_mcu::register::Register<Intenclr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenclr, 0x8)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenset, 0xa)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intflag, 0xc)
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

    #[doc="Get the DBGCTRL Register."]
    #[inline] pub fn dbgctrl_reg(&self) -> ::bobbin_mcu::register::Register<Dbgctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dbgctrl, 0xe)
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

    #[doc="Get the SYNCBUSY Register."]
    #[inline] pub fn syncbusy_reg(&self) -> ::bobbin_mcu::register::Register<Syncbusy> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Syncbusy, 0x10)
    }

    #[doc="Get the *mut pointer for the SYNCBUSY register."]
    #[inline] pub fn syncbusy_mut(&self) -> *mut Syncbusy { 
        self.syncbusy_reg().ptr()
    }

    #[doc="Get the *const pointer for the SYNCBUSY register."]
    #[inline] pub fn syncbusy_ptr(&self) -> *const Syncbusy { 
        self.syncbusy_reg().ptr()
    }

    #[doc="Read the SYNCBUSY register."]
    #[inline] pub fn syncbusy(&self) -> Syncbusy { 
        self.syncbusy_reg().read()
    }

    #[doc="Get the FREQCORR Register."]
    #[inline] pub fn freqcorr_reg(&self) -> ::bobbin_mcu::register::Register<Freqcorr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Freqcorr, 0x14)
    }

    #[doc="Get the *mut pointer for the FREQCORR register."]
    #[inline] pub fn freqcorr_mut(&self) -> *mut Freqcorr { 
        self.freqcorr_reg().ptr()
    }

    #[doc="Get the *const pointer for the FREQCORR register."]
    #[inline] pub fn freqcorr_ptr(&self) -> *const Freqcorr { 
        self.freqcorr_reg().ptr()
    }

    #[doc="Read the FREQCORR register."]
    #[inline] pub fn freqcorr(&self) -> Freqcorr { 
        self.freqcorr_reg().read()
    }

    #[doc="Write the FREQCORR register."]
    #[inline] pub fn write_freqcorr(&self, value: Freqcorr) -> &Self { 
        self.freqcorr_reg().write(value);
        self
    }

    #[doc="Set the FREQCORR register."]
    #[inline] pub fn set_freqcorr<F: FnOnce(Freqcorr) -> Freqcorr>(&self, f: F) -> &Self {
        self.freqcorr_reg().set(f);
        self
    }

    #[doc="Modify the FREQCORR register."]
    #[inline] pub fn with_freqcorr<F: FnOnce(Freqcorr) -> Freqcorr>(&self, f: F) -> &Self {
        self.freqcorr_reg().with(f);
        self
    }

    #[doc="Get the COUNT Register."]
    #[inline] pub fn count_reg(&self) -> ::bobbin_mcu::register::Register<Count> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Count, 0x18)
    }

    #[doc="Get the *mut pointer for the COUNT register."]
    #[inline] pub fn count_mut(&self) -> *mut Count { 
        self.count_reg().ptr()
    }

    #[doc="Get the *const pointer for the COUNT register."]
    #[inline] pub fn count_ptr(&self) -> *const Count { 
        self.count_reg().ptr()
    }

    #[doc="Read the COUNT register."]
    #[inline] pub fn count(&self) -> Count { 
        self.count_reg().read()
    }

    #[doc="Write the COUNT register."]
    #[inline] pub fn write_count(&self, value: Count) -> &Self { 
        self.count_reg().write(value);
        self
    }

    #[doc="Set the COUNT register."]
    #[inline] pub fn set_count<F: FnOnce(Count) -> Count>(&self, f: F) -> &Self {
        self.count_reg().set(f);
        self
    }

    #[doc="Modify the COUNT register."]
    #[inline] pub fn with_count<F: FnOnce(Count) -> Count>(&self, f: F) -> &Self {
        self.count_reg().with(f);
        self
    }

    #[doc="Get the COMP Register."]
    #[inline] pub fn comp_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Comp, ::bobbin_bits::R2> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Comp, 0x20, 0x4)
    }

    #[doc="Get the *mut pointer for the COMP register."]
    #[inline] pub fn comp_mut<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *mut Comp { 
        self.comp_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the COMP register."]
    #[inline] pub fn comp_ptr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *const Comp { 
        self.comp_reg().ptr(index.into())
    }

    #[doc="Read the COMP register."]
    #[inline] pub fn comp<I: Into<::bobbin_bits::R2>>(&self, index: I) -> Comp { 
        self.comp_reg().read(index.into())
    }

    #[doc="Write the COMP register."]
    #[inline] pub fn write_comp<I: Into<::bobbin_bits::R2>>(&self, index: I, value: Comp) -> &Self {
        self.comp_reg().write(index.into(), value);
        self
    }

    #[doc="Set the COMP register."]
    #[inline] pub fn set_comp<I: Into<::bobbin_bits::R2>, F: FnOnce(Comp) -> Comp>(&self, index: I, f: F) -> &Self {
        self.comp_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the COMP register."]
    #[inline] pub fn with_comp<I: Into<::bobbin_bits::R2> + Copy, F: FnOnce(Comp) -> Comp>(&self, index: I, f: F) -> &Self {
        self.comp_reg().with(index.into(), f);
        self
    }

    #[doc="Get the GP Register."]
    #[inline] pub fn gp_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Gp, ::bobbin_bits::R4> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Gp, 0x40, 0x4)
    }

    #[doc="Get the *mut pointer for the GP register."]
    #[inline] pub fn gp_mut<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *mut Gp { 
        self.gp_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the GP register."]
    #[inline] pub fn gp_ptr<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *const Gp { 
        self.gp_reg().ptr(index.into())
    }

    #[doc="Read the GP register."]
    #[inline] pub fn gp<I: Into<::bobbin_bits::R4>>(&self, index: I) -> Gp { 
        self.gp_reg().read(index.into())
    }

    #[doc="Write the GP register."]
    #[inline] pub fn write_gp<I: Into<::bobbin_bits::R4>>(&self, index: I, value: Gp) -> &Self {
        self.gp_reg().write(index.into(), value);
        self
    }

    #[doc="Set the GP register."]
    #[inline] pub fn set_gp<I: Into<::bobbin_bits::R4>, F: FnOnce(Gp) -> Gp>(&self, index: I, f: F) -> &Self {
        self.gp_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the GP register."]
    #[inline] pub fn with_gp<I: Into<::bobbin_bits::R4> + Copy, F: FnOnce(Gp) -> Gp>(&self, index: I, f: F) -> &Self {
        self.gp_reg().with(index.into(), f);
        self
    }

    #[doc="Get the TAMPCTRL Register."]
    #[inline] pub fn tampctrl_reg(&self) -> ::bobbin_mcu::register::Register<Tampctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Tampctrl, 0x60)
    }

    #[doc="Get the *mut pointer for the TAMPCTRL register."]
    #[inline] pub fn tampctrl_mut(&self) -> *mut Tampctrl { 
        self.tampctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the TAMPCTRL register."]
    #[inline] pub fn tampctrl_ptr(&self) -> *const Tampctrl { 
        self.tampctrl_reg().ptr()
    }

    #[doc="Read the TAMPCTRL register."]
    #[inline] pub fn tampctrl(&self) -> Tampctrl { 
        self.tampctrl_reg().read()
    }

    #[doc="Write the TAMPCTRL register."]
    #[inline] pub fn write_tampctrl(&self, value: Tampctrl) -> &Self { 
        self.tampctrl_reg().write(value);
        self
    }

    #[doc="Set the TAMPCTRL register."]
    #[inline] pub fn set_tampctrl<F: FnOnce(Tampctrl) -> Tampctrl>(&self, f: F) -> &Self {
        self.tampctrl_reg().set(f);
        self
    }

    #[doc="Modify the TAMPCTRL register."]
    #[inline] pub fn with_tampctrl<F: FnOnce(Tampctrl) -> Tampctrl>(&self, f: F) -> &Self {
        self.tampctrl_reg().with(f);
        self
    }

    #[doc="Get the TIMESTAMP Register."]
    #[inline] pub fn timestamp_reg(&self) -> ::bobbin_mcu::register::Register<Timestamp> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Timestamp, 0x64)
    }

    #[doc="Get the *mut pointer for the TIMESTAMP register."]
    #[inline] pub fn timestamp_mut(&self) -> *mut Timestamp { 
        self.timestamp_reg().ptr()
    }

    #[doc="Get the *const pointer for the TIMESTAMP register."]
    #[inline] pub fn timestamp_ptr(&self) -> *const Timestamp { 
        self.timestamp_reg().ptr()
    }

    #[doc="Read the TIMESTAMP register."]
    #[inline] pub fn timestamp(&self) -> Timestamp { 
        self.timestamp_reg().read()
    }

    #[doc="Get the TAMPID Register."]
    #[inline] pub fn tampid_reg(&self) -> ::bobbin_mcu::register::Register<Tampid> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Tampid, 0x68)
    }

    #[doc="Get the *mut pointer for the TAMPID register."]
    #[inline] pub fn tampid_mut(&self) -> *mut Tampid { 
        self.tampid_reg().ptr()
    }

    #[doc="Get the *const pointer for the TAMPID register."]
    #[inline] pub fn tampid_ptr(&self) -> *const Tampid { 
        self.tampid_reg().ptr()
    }

    #[doc="Read the TAMPID register."]
    #[inline] pub fn tampid(&self) -> Tampid { 
        self.tampid_reg().read()
    }

    #[doc="Write the TAMPID register."]
    #[inline] pub fn write_tampid(&self, value: Tampid) -> &Self { 
        self.tampid_reg().write(value);
        self
    }

    #[doc="Set the TAMPID register."]
    #[inline] pub fn set_tampid<F: FnOnce(Tampid) -> Tampid>(&self, f: F) -> &Self {
        self.tampid_reg().set(f);
        self
    }

    #[doc="Modify the TAMPID register."]
    #[inline] pub fn with_tampid<F: FnOnce(Tampid) -> Tampid>(&self, f: F) -> &Self {
        self.tampid_reg().with(f);
        self
    }

    #[doc="Get the BKUP Register."]
    #[inline] pub fn bkup_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Bkup, ::bobbin_bits::R8> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Bkup, 0x80, 0x4)
    }

    #[doc="Get the *mut pointer for the BKUP register."]
    #[inline] pub fn bkup_mut<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *mut Bkup { 
        self.bkup_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the BKUP register."]
    #[inline] pub fn bkup_ptr<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *const Bkup { 
        self.bkup_reg().ptr(index.into())
    }

    #[doc="Read the BKUP register."]
    #[inline] pub fn bkup<I: Into<::bobbin_bits::R8>>(&self, index: I) -> Bkup { 
        self.bkup_reg().read(index.into())
    }

    #[doc="Write the BKUP register."]
    #[inline] pub fn write_bkup<I: Into<::bobbin_bits::R8>>(&self, index: I, value: Bkup) -> &Self {
        self.bkup_reg().write(index.into(), value);
        self
    }

    #[doc="Set the BKUP register."]
    #[inline] pub fn set_bkup<I: Into<::bobbin_bits::R8>, F: FnOnce(Bkup) -> Bkup>(&self, index: I, f: F) -> &Self {
        self.bkup_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the BKUP register."]
    #[inline] pub fn with_bkup<I: Into<::bobbin_bits::R8> + Copy, F: FnOnce(Bkup) -> Bkup>(&self, index: I, f: F) -> &Self {
        self.bkup_reg().with(index.into(), f);
        self
    }

}

#[doc="MODE0 Control A"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrla(pub u16);
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
        let value: u16 = value.into();
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
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Operating Mode"]
    #[inline] pub fn mode(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode(&self) -> bool {
        self.mode() != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Clear on Match"]
    #[inline] pub fn matchclr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if MATCHCLR != 0"]
    #[inline] pub fn test_matchclr(&self) -> bool {
        self.matchclr() != 0
    }

    #[doc="Sets the MATCHCLR field."]
    #[inline] pub fn set_matchclr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Prescaler"]
    #[inline] pub fn prescaler(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if PRESCALER != 0"]
    #[inline] pub fn test_prescaler(&self) -> bool {
        self.prescaler() != 0
    }

    #[doc="Sets the PRESCALER field."]
    #[inline] pub fn set_prescaler<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="BKUP Registers Reset On Tamper Enable"]
    #[inline] pub fn bktrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if BKTRST != 0"]
    #[inline] pub fn test_bktrst(&self) -> bool {
        self.bktrst() != 0
    }

    #[doc="Sets the BKTRST field."]
    #[inline] pub fn set_bktrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="GP Registers Reset On Tamper Enable"]
    #[inline] pub fn gptrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if GPTRST != 0"]
    #[inline] pub fn test_gptrst(&self) -> bool {
        self.gptrst() != 0
    }

    #[doc="Sets the GPTRST field."]
    #[inline] pub fn set_gptrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Count Read Synchronization Enable"]
    #[inline] pub fn countsync(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if COUNTSYNC != 0"]
    #[inline] pub fn test_countsync(&self) -> bool {
        self.countsync() != 0
    }

    #[doc="Sets the COUNTSYNC field."]
    #[inline] pub fn set_countsync<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Ctrla {
    #[inline]
    fn from(other: u16) -> Self {
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
        if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
        if self.matchclr() != 0 { try!(write!(f, " matchclr"))}
        if self.prescaler() != 0 { try!(write!(f, " prescaler=0x{:x}", self.prescaler()))}
        if self.bktrst() != 0 { try!(write!(f, " bktrst"))}
        if self.gptrst() != 0 { try!(write!(f, " gptrst"))}
        if self.countsync() != 0 { try!(write!(f, " countsync"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MODE0 Control B"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrlb(pub u16);
impl Ctrlb {
    #[doc="General Purpose 0 Enable"]
    #[inline] pub fn gp0en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if GP0EN != 0"]
    #[inline] pub fn test_gp0en(&self) -> bool {
        self.gp0en() != 0
    }

    #[doc="Sets the GP0EN field."]
    #[inline] pub fn set_gp0en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="General Purpose 2 Enable"]
    #[inline] pub fn gp2en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if GP2EN != 0"]
    #[inline] pub fn test_gp2en(&self) -> bool {
        self.gp2en() != 0
    }

    #[doc="Sets the GP2EN field."]
    #[inline] pub fn set_gp2en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Debouncer Majority Enable"]
    #[inline] pub fn debmaj(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DEBMAJ != 0"]
    #[inline] pub fn test_debmaj(&self) -> bool {
        self.debmaj() != 0
    }

    #[doc="Sets the DEBMAJ field."]
    #[inline] pub fn set_debmaj<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Debouncer Asynchronous Enable"]
    #[inline] pub fn debasync(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DEBASYNC != 0"]
    #[inline] pub fn test_debasync(&self) -> bool {
        self.debasync() != 0
    }

    #[doc="Sets the DEBASYNC field."]
    #[inline] pub fn set_debasync<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="RTC Output Enable"]
    #[inline] pub fn rtcout(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RTCOUT != 0"]
    #[inline] pub fn test_rtcout(&self) -> bool {
        self.rtcout() != 0
    }

    #[doc="Sets the RTCOUT field."]
    #[inline] pub fn set_rtcout<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="DMA Enable"]
    #[inline] pub fn dmaen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if DMAEN != 0"]
    #[inline] pub fn test_dmaen(&self) -> bool {
        self.dmaen() != 0
    }

    #[doc="Sets the DMAEN field."]
    #[inline] pub fn set_dmaen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Debounce Freqnuency"]
    #[inline] pub fn debf(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if DEBF != 0"]
    #[inline] pub fn test_debf(&self) -> bool {
        self.debf() != 0
    }

    #[doc="Sets the DEBF field."]
    #[inline] pub fn set_debf<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Active Layer Freqnuency"]
    #[inline] pub fn actf(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
    }

    #[doc="Returns true if ACTF != 0"]
    #[inline] pub fn test_actf(&self) -> bool {
        self.actf() != 0
    }

    #[doc="Sets the ACTF field."]
    #[inline] pub fn set_actf<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7 << 12);
        self.0 |= value << 12;
        self
    }

}

impl From<u16> for Ctrlb {
    #[inline]
    fn from(other: u16) -> Self {
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
        if self.gp0en() != 0 { try!(write!(f, " gp0en"))}
        if self.gp2en() != 0 { try!(write!(f, " gp2en"))}
        if self.debmaj() != 0 { try!(write!(f, " debmaj"))}
        if self.debasync() != 0 { try!(write!(f, " debasync"))}
        if self.rtcout() != 0 { try!(write!(f, " rtcout"))}
        if self.dmaen() != 0 { try!(write!(f, " dmaen"))}
        if self.debf() != 0 { try!(write!(f, " debf=0x{:x}", self.debf()))}
        if self.actf() != 0 { try!(write!(f, " actf=0x{:x}", self.actf()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MODE0 Event Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Evctrl(pub u32);
impl Evctrl {
    #[doc="Periodic Interval 0 Event Output Enable"]
    #[inline] pub fn pereo0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PEREO0 != 0"]
    #[inline] pub fn test_pereo0(&self) -> bool {
        self.pereo0() != 0
    }

    #[doc="Sets the PEREO0 field."]
    #[inline] pub fn set_pereo0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Periodic Interval 1 Event Output Enable"]
    #[inline] pub fn pereo1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PEREO1 != 0"]
    #[inline] pub fn test_pereo1(&self) -> bool {
        self.pereo1() != 0
    }

    #[doc="Sets the PEREO1 field."]
    #[inline] pub fn set_pereo1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Periodic Interval 2 Event Output Enable"]
    #[inline] pub fn pereo2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PEREO2 != 0"]
    #[inline] pub fn test_pereo2(&self) -> bool {
        self.pereo2() != 0
    }

    #[doc="Sets the PEREO2 field."]
    #[inline] pub fn set_pereo2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Periodic Interval 3 Event Output Enable"]
    #[inline] pub fn pereo3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PEREO3 != 0"]
    #[inline] pub fn test_pereo3(&self) -> bool {
        self.pereo3() != 0
    }

    #[doc="Sets the PEREO3 field."]
    #[inline] pub fn set_pereo3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Periodic Interval 4 Event Output Enable"]
    #[inline] pub fn pereo4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PEREO4 != 0"]
    #[inline] pub fn test_pereo4(&self) -> bool {
        self.pereo4() != 0
    }

    #[doc="Sets the PEREO4 field."]
    #[inline] pub fn set_pereo4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Periodic Interval 5 Event Output Enable"]
    #[inline] pub fn pereo5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PEREO5 != 0"]
    #[inline] pub fn test_pereo5(&self) -> bool {
        self.pereo5() != 0
    }

    #[doc="Sets the PEREO5 field."]
    #[inline] pub fn set_pereo5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Periodic Interval 6 Event Output Enable"]
    #[inline] pub fn pereo6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PEREO6 != 0"]
    #[inline] pub fn test_pereo6(&self) -> bool {
        self.pereo6() != 0
    }

    #[doc="Sets the PEREO6 field."]
    #[inline] pub fn set_pereo6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Periodic Interval 7 Event Output Enable"]
    #[inline] pub fn pereo7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PEREO7 != 0"]
    #[inline] pub fn test_pereo7(&self) -> bool {
        self.pereo7() != 0
    }

    #[doc="Sets the PEREO7 field."]
    #[inline] pub fn set_pereo7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Compare 0 Event Output Enable"]
    #[inline] pub fn cmpeo0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CMPEO0 != 0"]
    #[inline] pub fn test_cmpeo0(&self) -> bool {
        self.cmpeo0() != 0
    }

    #[doc="Sets the CMPEO0 field."]
    #[inline] pub fn set_cmpeo0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Compare 1 Event Output Enable"]
    #[inline] pub fn cmpeo1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CMPEO1 != 0"]
    #[inline] pub fn test_cmpeo1(&self) -> bool {
        self.cmpeo1() != 0
    }

    #[doc="Sets the CMPEO1 field."]
    #[inline] pub fn set_cmpeo1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Tamper Event Output Enable"]
    #[inline] pub fn tampereo(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TAMPEREO != 0"]
    #[inline] pub fn test_tampereo(&self) -> bool {
        self.tampereo() != 0
    }

    #[doc="Sets the TAMPEREO field."]
    #[inline] pub fn set_tampereo<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Overflow Event Output Enable"]
    #[inline] pub fn ovfeo(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if OVFEO != 0"]
    #[inline] pub fn test_ovfeo(&self) -> bool {
        self.ovfeo() != 0
    }

    #[doc="Sets the OVFEO field."]
    #[inline] pub fn set_ovfeo<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Tamper Event Input Enable"]
    #[inline] pub fn tampevei(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TAMPEVEI != 0"]
    #[inline] pub fn test_tampevei(&self) -> bool {
        self.tampevei() != 0
    }

    #[doc="Sets the TAMPEVEI field."]
    #[inline] pub fn set_tampevei<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Evctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Evctrl(other)
    }
}

impl ::core::fmt::Display for Evctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Evctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pereo0() != 0 { try!(write!(f, " pereo0"))}
        if self.pereo1() != 0 { try!(write!(f, " pereo1"))}
        if self.pereo2() != 0 { try!(write!(f, " pereo2"))}
        if self.pereo3() != 0 { try!(write!(f, " pereo3"))}
        if self.pereo4() != 0 { try!(write!(f, " pereo4"))}
        if self.pereo5() != 0 { try!(write!(f, " pereo5"))}
        if self.pereo6() != 0 { try!(write!(f, " pereo6"))}
        if self.pereo7() != 0 { try!(write!(f, " pereo7"))}
        if self.cmpeo0() != 0 { try!(write!(f, " cmpeo0"))}
        if self.cmpeo1() != 0 { try!(write!(f, " cmpeo1"))}
        if self.tampereo() != 0 { try!(write!(f, " tampereo"))}
        if self.ovfeo() != 0 { try!(write!(f, " ovfeo"))}
        if self.tampevei() != 0 { try!(write!(f, " tampevei"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MODE0 Interrupt Enable Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u16);
impl Intenclr {
    #[doc="Periodic Interval 0 Interrupt Enable"]
    #[inline] pub fn per0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PER0 != 0"]
    #[inline] pub fn test_per0(&self) -> bool {
        self.per0() != 0
    }

    #[doc="Sets the PER0 field."]
    #[inline] pub fn set_per0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Periodic Interval 1 Interrupt Enable"]
    #[inline] pub fn per1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PER1 != 0"]
    #[inline] pub fn test_per1(&self) -> bool {
        self.per1() != 0
    }

    #[doc="Sets the PER1 field."]
    #[inline] pub fn set_per1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Periodic Interval 2 Interrupt Enable"]
    #[inline] pub fn per2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PER2 != 0"]
    #[inline] pub fn test_per2(&self) -> bool {
        self.per2() != 0
    }

    #[doc="Sets the PER2 field."]
    #[inline] pub fn set_per2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Periodic Interval 3 Interrupt Enable"]
    #[inline] pub fn per3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PER3 != 0"]
    #[inline] pub fn test_per3(&self) -> bool {
        self.per3() != 0
    }

    #[doc="Sets the PER3 field."]
    #[inline] pub fn set_per3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Periodic Interval 4 Interrupt Enable"]
    #[inline] pub fn per4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PER4 != 0"]
    #[inline] pub fn test_per4(&self) -> bool {
        self.per4() != 0
    }

    #[doc="Sets the PER4 field."]
    #[inline] pub fn set_per4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Periodic Interval 5 Interrupt Enable"]
    #[inline] pub fn per5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PER5 != 0"]
    #[inline] pub fn test_per5(&self) -> bool {
        self.per5() != 0
    }

    #[doc="Sets the PER5 field."]
    #[inline] pub fn set_per5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Periodic Interval 6 Interrupt Enable"]
    #[inline] pub fn per6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PER6 != 0"]
    #[inline] pub fn test_per6(&self) -> bool {
        self.per6() != 0
    }

    #[doc="Sets the PER6 field."]
    #[inline] pub fn set_per6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Periodic Interval 7 Interrupt Enable"]
    #[inline] pub fn per7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PER7 != 0"]
    #[inline] pub fn test_per7(&self) -> bool {
        self.per7() != 0
    }

    #[doc="Sets the PER7 field."]
    #[inline] pub fn set_per7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Compare 0 Interrupt Enable"]
    #[inline] pub fn cmp0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CMP0 != 0"]
    #[inline] pub fn test_cmp0(&self) -> bool {
        self.cmp0() != 0
    }

    #[doc="Sets the CMP0 field."]
    #[inline] pub fn set_cmp0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Compare 1 Interrupt Enable"]
    #[inline] pub fn cmp1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CMP1 != 0"]
    #[inline] pub fn test_cmp1(&self) -> bool {
        self.cmp1() != 0
    }

    #[doc="Sets the CMP1 field."]
    #[inline] pub fn set_cmp1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Tamper Enable"]
    #[inline] pub fn tamper(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TAMPER != 0"]
    #[inline] pub fn test_tamper(&self) -> bool {
        self.tamper() != 0
    }

    #[doc="Sets the TAMPER field."]
    #[inline] pub fn set_tamper<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Overflow Interrupt Enable"]
    #[inline] pub fn ovf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if OVF != 0"]
    #[inline] pub fn test_ovf(&self) -> bool {
        self.ovf() != 0
    }

    #[doc="Sets the OVF field."]
    #[inline] pub fn set_ovf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Intenclr {
    #[inline]
    fn from(other: u16) -> Self {
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
        if self.per0() != 0 { try!(write!(f, " per0"))}
        if self.per1() != 0 { try!(write!(f, " per1"))}
        if self.per2() != 0 { try!(write!(f, " per2"))}
        if self.per3() != 0 { try!(write!(f, " per3"))}
        if self.per4() != 0 { try!(write!(f, " per4"))}
        if self.per5() != 0 { try!(write!(f, " per5"))}
        if self.per6() != 0 { try!(write!(f, " per6"))}
        if self.per7() != 0 { try!(write!(f, " per7"))}
        if self.cmp0() != 0 { try!(write!(f, " cmp0"))}
        if self.cmp1() != 0 { try!(write!(f, " cmp1"))}
        if self.tamper() != 0 { try!(write!(f, " tamper"))}
        if self.ovf() != 0 { try!(write!(f, " ovf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MODE0 Interrupt Enable Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u16);
impl Intenset {
    #[doc="Periodic Interval 0 Interrupt Enable"]
    #[inline] pub fn per0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PER0 != 0"]
    #[inline] pub fn test_per0(&self) -> bool {
        self.per0() != 0
    }

    #[doc="Sets the PER0 field."]
    #[inline] pub fn set_per0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Periodic Interval 1 Interrupt Enable"]
    #[inline] pub fn per1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PER1 != 0"]
    #[inline] pub fn test_per1(&self) -> bool {
        self.per1() != 0
    }

    #[doc="Sets the PER1 field."]
    #[inline] pub fn set_per1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Periodic Interval 2 Interrupt Enable"]
    #[inline] pub fn per2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PER2 != 0"]
    #[inline] pub fn test_per2(&self) -> bool {
        self.per2() != 0
    }

    #[doc="Sets the PER2 field."]
    #[inline] pub fn set_per2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Periodic Interval 3 Interrupt Enable"]
    #[inline] pub fn per3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PER3 != 0"]
    #[inline] pub fn test_per3(&self) -> bool {
        self.per3() != 0
    }

    #[doc="Sets the PER3 field."]
    #[inline] pub fn set_per3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Periodic Interval 4 Interrupt Enable"]
    #[inline] pub fn per4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PER4 != 0"]
    #[inline] pub fn test_per4(&self) -> bool {
        self.per4() != 0
    }

    #[doc="Sets the PER4 field."]
    #[inline] pub fn set_per4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Periodic Interval 5 Interrupt Enable"]
    #[inline] pub fn per5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PER5 != 0"]
    #[inline] pub fn test_per5(&self) -> bool {
        self.per5() != 0
    }

    #[doc="Sets the PER5 field."]
    #[inline] pub fn set_per5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Periodic Interval 6 Interrupt Enable"]
    #[inline] pub fn per6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PER6 != 0"]
    #[inline] pub fn test_per6(&self) -> bool {
        self.per6() != 0
    }

    #[doc="Sets the PER6 field."]
    #[inline] pub fn set_per6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Periodic Interval 7 Interrupt Enable"]
    #[inline] pub fn per7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PER7 != 0"]
    #[inline] pub fn test_per7(&self) -> bool {
        self.per7() != 0
    }

    #[doc="Sets the PER7 field."]
    #[inline] pub fn set_per7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Compare 0 Interrupt Enable"]
    #[inline] pub fn cmp0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CMP0 != 0"]
    #[inline] pub fn test_cmp0(&self) -> bool {
        self.cmp0() != 0
    }

    #[doc="Sets the CMP0 field."]
    #[inline] pub fn set_cmp0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Compare 1 Interrupt Enable"]
    #[inline] pub fn cmp1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CMP1 != 0"]
    #[inline] pub fn test_cmp1(&self) -> bool {
        self.cmp1() != 0
    }

    #[doc="Sets the CMP1 field."]
    #[inline] pub fn set_cmp1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Tamper Enable"]
    #[inline] pub fn tamper(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TAMPER != 0"]
    #[inline] pub fn test_tamper(&self) -> bool {
        self.tamper() != 0
    }

    #[doc="Sets the TAMPER field."]
    #[inline] pub fn set_tamper<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Overflow Interrupt Enable"]
    #[inline] pub fn ovf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if OVF != 0"]
    #[inline] pub fn test_ovf(&self) -> bool {
        self.ovf() != 0
    }

    #[doc="Sets the OVF field."]
    #[inline] pub fn set_ovf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Intenset {
    #[inline]
    fn from(other: u16) -> Self {
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
        if self.per0() != 0 { try!(write!(f, " per0"))}
        if self.per1() != 0 { try!(write!(f, " per1"))}
        if self.per2() != 0 { try!(write!(f, " per2"))}
        if self.per3() != 0 { try!(write!(f, " per3"))}
        if self.per4() != 0 { try!(write!(f, " per4"))}
        if self.per5() != 0 { try!(write!(f, " per5"))}
        if self.per6() != 0 { try!(write!(f, " per6"))}
        if self.per7() != 0 { try!(write!(f, " per7"))}
        if self.cmp0() != 0 { try!(write!(f, " cmp0"))}
        if self.cmp1() != 0 { try!(write!(f, " cmp1"))}
        if self.tamper() != 0 { try!(write!(f, " tamper"))}
        if self.ovf() != 0 { try!(write!(f, " ovf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MODE0 Interrupt Flag Status and Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u16);
impl Intflag {
    #[doc="Periodic Interval 0"]
    #[inline] pub fn per0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PER0 != 0"]
    #[inline] pub fn test_per0(&self) -> bool {
        self.per0() != 0
    }

    #[doc="Sets the PER0 field."]
    #[inline] pub fn set_per0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Periodic Interval 1"]
    #[inline] pub fn per1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PER1 != 0"]
    #[inline] pub fn test_per1(&self) -> bool {
        self.per1() != 0
    }

    #[doc="Sets the PER1 field."]
    #[inline] pub fn set_per1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Periodic Interval 2"]
    #[inline] pub fn per2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PER2 != 0"]
    #[inline] pub fn test_per2(&self) -> bool {
        self.per2() != 0
    }

    #[doc="Sets the PER2 field."]
    #[inline] pub fn set_per2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Periodic Interval 3"]
    #[inline] pub fn per3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PER3 != 0"]
    #[inline] pub fn test_per3(&self) -> bool {
        self.per3() != 0
    }

    #[doc="Sets the PER3 field."]
    #[inline] pub fn set_per3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Periodic Interval 4"]
    #[inline] pub fn per4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PER4 != 0"]
    #[inline] pub fn test_per4(&self) -> bool {
        self.per4() != 0
    }

    #[doc="Sets the PER4 field."]
    #[inline] pub fn set_per4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Periodic Interval 5"]
    #[inline] pub fn per5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PER5 != 0"]
    #[inline] pub fn test_per5(&self) -> bool {
        self.per5() != 0
    }

    #[doc="Sets the PER5 field."]
    #[inline] pub fn set_per5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Periodic Interval 6"]
    #[inline] pub fn per6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PER6 != 0"]
    #[inline] pub fn test_per6(&self) -> bool {
        self.per6() != 0
    }

    #[doc="Sets the PER6 field."]
    #[inline] pub fn set_per6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Periodic Interval 7"]
    #[inline] pub fn per7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PER7 != 0"]
    #[inline] pub fn test_per7(&self) -> bool {
        self.per7() != 0
    }

    #[doc="Sets the PER7 field."]
    #[inline] pub fn set_per7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Compare 0"]
    #[inline] pub fn cmp0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CMP0 != 0"]
    #[inline] pub fn test_cmp0(&self) -> bool {
        self.cmp0() != 0
    }

    #[doc="Sets the CMP0 field."]
    #[inline] pub fn set_cmp0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Compare 1"]
    #[inline] pub fn cmp1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CMP1 != 0"]
    #[inline] pub fn test_cmp1(&self) -> bool {
        self.cmp1() != 0
    }

    #[doc="Sets the CMP1 field."]
    #[inline] pub fn set_cmp1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Tamper"]
    #[inline] pub fn tamper(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TAMPER != 0"]
    #[inline] pub fn test_tamper(&self) -> bool {
        self.tamper() != 0
    }

    #[doc="Sets the TAMPER field."]
    #[inline] pub fn set_tamper<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Overflow"]
    #[inline] pub fn ovf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if OVF != 0"]
    #[inline] pub fn test_ovf(&self) -> bool {
        self.ovf() != 0
    }

    #[doc="Sets the OVF field."]
    #[inline] pub fn set_ovf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Intflag {
    #[inline]
    fn from(other: u16) -> Self {
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
        if self.per0() != 0 { try!(write!(f, " per0"))}
        if self.per1() != 0 { try!(write!(f, " per1"))}
        if self.per2() != 0 { try!(write!(f, " per2"))}
        if self.per3() != 0 { try!(write!(f, " per3"))}
        if self.per4() != 0 { try!(write!(f, " per4"))}
        if self.per5() != 0 { try!(write!(f, " per5"))}
        if self.per6() != 0 { try!(write!(f, " per6"))}
        if self.per7() != 0 { try!(write!(f, " per7"))}
        if self.cmp0() != 0 { try!(write!(f, " cmp0"))}
        if self.cmp1() != 0 { try!(write!(f, " cmp1"))}
        if self.tamper() != 0 { try!(write!(f, " tamper"))}
        if self.ovf() != 0 { try!(write!(f, " ovf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Debug Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
    #[doc="Run During Debug"]
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

#[doc="MODE0 Synchronization Busy Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Syncbusy(pub u32);
impl Syncbusy {
    #[doc="Software Reset Busy"]
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

    #[doc="Enable Bit Busy"]
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

    #[doc="FREQCORR Register Busy"]
    #[inline] pub fn freqcorr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if FREQCORR != 0"]
    #[inline] pub fn test_freqcorr(&self) -> bool {
        self.freqcorr() != 0
    }

    #[doc="Sets the FREQCORR field."]
    #[inline] pub fn set_freqcorr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="COUNT Register Busy"]
    #[inline] pub fn count(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if COUNT != 0"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Sets the COUNT field."]
    #[inline] pub fn set_count<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="COMP 0 Register Busy"]
    #[inline] pub fn comp0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if COMP0 != 0"]
    #[inline] pub fn test_comp0(&self) -> bool {
        self.comp0() != 0
    }

    #[doc="Sets the COMP0 field."]
    #[inline] pub fn set_comp0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="COMP 1 Register Busy"]
    #[inline] pub fn comp1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if COMP1 != 0"]
    #[inline] pub fn test_comp1(&self) -> bool {
        self.comp1() != 0
    }

    #[doc="Sets the COMP1 field."]
    #[inline] pub fn set_comp1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Count Synchronization Enable Bit Busy"]
    #[inline] pub fn countsync(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if COUNTSYNC != 0"]
    #[inline] pub fn test_countsync(&self) -> bool {
        self.countsync() != 0
    }

    #[doc="Sets the COUNTSYNC field."]
    #[inline] pub fn set_countsync<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="General Purpose 0 Register Busy"]
    #[inline] pub fn gp0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if GP0 != 0"]
    #[inline] pub fn test_gp0(&self) -> bool {
        self.gp0() != 0
    }

    #[doc="Sets the GP0 field."]
    #[inline] pub fn set_gp0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="General Purpose 1 Register Busy"]
    #[inline] pub fn gp1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if GP1 != 0"]
    #[inline] pub fn test_gp1(&self) -> bool {
        self.gp1() != 0
    }

    #[doc="Sets the GP1 field."]
    #[inline] pub fn set_gp1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="General Purpose 2 Register Busy"]
    #[inline] pub fn gp2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if GP2 != 0"]
    #[inline] pub fn test_gp2(&self) -> bool {
        self.gp2() != 0
    }

    #[doc="Sets the GP2 field."]
    #[inline] pub fn set_gp2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="General Purpose 3 Register Busy"]
    #[inline] pub fn gp3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if GP3 != 0"]
    #[inline] pub fn test_gp3(&self) -> bool {
        self.gp3() != 0
    }

    #[doc="Sets the GP3 field."]
    #[inline] pub fn set_gp3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

}

impl From<u32> for Syncbusy {
    #[inline]
    fn from(other: u32) -> Self {
         Syncbusy(other)
    }
}

impl ::core::fmt::Display for Syncbusy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Syncbusy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.swrst() != 0 { try!(write!(f, " swrst"))}
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.freqcorr() != 0 { try!(write!(f, " freqcorr"))}
        if self.count() != 0 { try!(write!(f, " count"))}
        if self.comp0() != 0 { try!(write!(f, " comp0"))}
        if self.comp1() != 0 { try!(write!(f, " comp1"))}
        if self.countsync() != 0 { try!(write!(f, " countsync"))}
        if self.gp0() != 0 { try!(write!(f, " gp0"))}
        if self.gp1() != 0 { try!(write!(f, " gp1"))}
        if self.gp2() != 0 { try!(write!(f, " gp2"))}
        if self.gp3() != 0 { try!(write!(f, " gp3"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Frequency Correction"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Freqcorr(pub u8);
impl Freqcorr {
    #[doc="Correction Value"]
    #[inline] pub fn value(&self) -> ::bobbin_bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="Returns true if VALUE != 0"]
    #[inline] pub fn test_value(&self) -> bool {
        self.value() != 0
    }

    #[doc="Sets the VALUE field."]
    #[inline] pub fn set_value<V: Into<::bobbin_bits::U7>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U7 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Correction Sign"]
    #[inline] pub fn sign(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SIGN != 0"]
    #[inline] pub fn test_sign(&self) -> bool {
        self.sign() != 0
    }

    #[doc="Sets the SIGN field."]
    #[inline] pub fn set_sign<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Freqcorr {
    #[inline]
    fn from(other: u8) -> Self {
         Freqcorr(other)
    }
}

impl ::core::fmt::Display for Freqcorr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Freqcorr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.value() != 0 { try!(write!(f, " value=0x{:x}", self.value()))}
        if self.sign() != 0 { try!(write!(f, " sign"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MODE0 Counter Value"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Count(pub u32);
impl Count {
    #[doc="Counter Value"]
    #[inline] pub fn count(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if COUNT != 0"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Sets the COUNT field."]
    #[inline] pub fn set_count<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Count {
    #[inline]
    fn from(other: u32) -> Self {
         Count(other)
    }
}

impl ::core::fmt::Display for Count {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Count {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MODE0 Compare n Value"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Comp(pub u32);
impl Comp {
    #[doc="Compare Value"]
    #[inline] pub fn comp(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if COMP != 0"]
    #[inline] pub fn test_comp(&self) -> bool {
        self.comp() != 0
    }

    #[doc="Sets the COMP field."]
    #[inline] pub fn set_comp<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Comp {
    #[inline]
    fn from(other: u32) -> Self {
         Comp(other)
    }
}

impl ::core::fmt::Display for Comp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Comp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="General Purpose"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Gp(pub u32);
impl Gp {
    #[doc="General Purpose"]
    #[inline] pub fn gp(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if GP != 0"]
    #[inline] pub fn test_gp(&self) -> bool {
        self.gp() != 0
    }

    #[doc="Sets the GP field."]
    #[inline] pub fn set_gp<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Gp {
    #[inline]
    fn from(other: u32) -> Self {
         Gp(other)
    }
}

impl ::core::fmt::Display for Gp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Gp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Tamper Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tampctrl(pub u32);
impl Tampctrl {
    #[doc="Tamper Input 0 Action"]
    #[inline] pub fn in0act(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if IN0ACT != 0"]
    #[inline] pub fn test_in0act(&self) -> bool {
        self.in0act() != 0
    }

    #[doc="Sets the IN0ACT field."]
    #[inline] pub fn set_in0act<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Tamper Input 1 Action"]
    #[inline] pub fn in1act(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if IN1ACT != 0"]
    #[inline] pub fn test_in1act(&self) -> bool {
        self.in1act() != 0
    }

    #[doc="Sets the IN1ACT field."]
    #[inline] pub fn set_in1act<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Tamper Input 2 Action"]
    #[inline] pub fn in2act(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if IN2ACT != 0"]
    #[inline] pub fn test_in2act(&self) -> bool {
        self.in2act() != 0
    }

    #[doc="Sets the IN2ACT field."]
    #[inline] pub fn set_in2act<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Tamper Input 3 Action"]
    #[inline] pub fn in3act(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if IN3ACT != 0"]
    #[inline] pub fn test_in3act(&self) -> bool {
        self.in3act() != 0
    }

    #[doc="Sets the IN3ACT field."]
    #[inline] pub fn set_in3act<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Tamper Input 4 Action"]
    #[inline] pub fn in4act(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if IN4ACT != 0"]
    #[inline] pub fn test_in4act(&self) -> bool {
        self.in4act() != 0
    }

    #[doc="Sets the IN4ACT field."]
    #[inline] pub fn set_in4act<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Tamper Level Select 0"]
    #[inline] pub fn tamlvl0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TAMLVL0 != 0"]
    #[inline] pub fn test_tamlvl0(&self) -> bool {
        self.tamlvl0() != 0
    }

    #[doc="Sets the TAMLVL0 field."]
    #[inline] pub fn set_tamlvl0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Tamper Level Select 1"]
    #[inline] pub fn tamlvl1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if TAMLVL1 != 0"]
    #[inline] pub fn test_tamlvl1(&self) -> bool {
        self.tamlvl1() != 0
    }

    #[doc="Sets the TAMLVL1 field."]
    #[inline] pub fn set_tamlvl1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Tamper Level Select 2"]
    #[inline] pub fn tamlvl2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if TAMLVL2 != 0"]
    #[inline] pub fn test_tamlvl2(&self) -> bool {
        self.tamlvl2() != 0
    }

    #[doc="Sets the TAMLVL2 field."]
    #[inline] pub fn set_tamlvl2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Tamper Level Select 3"]
    #[inline] pub fn tamlvl3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if TAMLVL3 != 0"]
    #[inline] pub fn test_tamlvl3(&self) -> bool {
        self.tamlvl3() != 0
    }

    #[doc="Sets the TAMLVL3 field."]
    #[inline] pub fn set_tamlvl3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Tamper Level Select 4"]
    #[inline] pub fn tamlvl4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if TAMLVL4 != 0"]
    #[inline] pub fn test_tamlvl4(&self) -> bool {
        self.tamlvl4() != 0
    }

    #[doc="Sets the TAMLVL4 field."]
    #[inline] pub fn set_tamlvl4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Debouncer Enable 0"]
    #[inline] pub fn debnc0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if DEBNC0 != 0"]
    #[inline] pub fn test_debnc0(&self) -> bool {
        self.debnc0() != 0
    }

    #[doc="Sets the DEBNC0 field."]
    #[inline] pub fn set_debnc0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Debouncer Enable 1"]
    #[inline] pub fn debnc1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if DEBNC1 != 0"]
    #[inline] pub fn test_debnc1(&self) -> bool {
        self.debnc1() != 0
    }

    #[doc="Sets the DEBNC1 field."]
    #[inline] pub fn set_debnc1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Debouncer Enable 2"]
    #[inline] pub fn debnc2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if DEBNC2 != 0"]
    #[inline] pub fn test_debnc2(&self) -> bool {
        self.debnc2() != 0
    }

    #[doc="Sets the DEBNC2 field."]
    #[inline] pub fn set_debnc2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Debouncer Enable 3"]
    #[inline] pub fn debnc3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if DEBNC3 != 0"]
    #[inline] pub fn test_debnc3(&self) -> bool {
        self.debnc3() != 0
    }

    #[doc="Sets the DEBNC3 field."]
    #[inline] pub fn set_debnc3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Debouncer Enable 4"]
    #[inline] pub fn debnc4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if DEBNC4 != 0"]
    #[inline] pub fn test_debnc4(&self) -> bool {
        self.debnc4() != 0
    }

    #[doc="Sets the DEBNC4 field."]
    #[inline] pub fn set_debnc4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

}

impl From<u32> for Tampctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Tampctrl(other)
    }
}

impl ::core::fmt::Display for Tampctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tampctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.in0act() != 0 { try!(write!(f, " in0act=0x{:x}", self.in0act()))}
        if self.in1act() != 0 { try!(write!(f, " in1act=0x{:x}", self.in1act()))}
        if self.in2act() != 0 { try!(write!(f, " in2act=0x{:x}", self.in2act()))}
        if self.in3act() != 0 { try!(write!(f, " in3act=0x{:x}", self.in3act()))}
        if self.in4act() != 0 { try!(write!(f, " in4act=0x{:x}", self.in4act()))}
        if self.tamlvl0() != 0 { try!(write!(f, " tamlvl0"))}
        if self.tamlvl1() != 0 { try!(write!(f, " tamlvl1"))}
        if self.tamlvl2() != 0 { try!(write!(f, " tamlvl2"))}
        if self.tamlvl3() != 0 { try!(write!(f, " tamlvl3"))}
        if self.tamlvl4() != 0 { try!(write!(f, " tamlvl4"))}
        if self.debnc0() != 0 { try!(write!(f, " debnc0"))}
        if self.debnc1() != 0 { try!(write!(f, " debnc1"))}
        if self.debnc2() != 0 { try!(write!(f, " debnc2"))}
        if self.debnc3() != 0 { try!(write!(f, " debnc3"))}
        if self.debnc4() != 0 { try!(write!(f, " debnc4"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MODE0 Timestamp"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Timestamp(pub u32);
impl Timestamp {
    #[doc="Count Timestamp Value"]
    #[inline] pub fn count(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if COUNT != 0"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Sets the COUNT field."]
    #[inline] pub fn set_count<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Timestamp {
    #[inline]
    fn from(other: u32) -> Self {
         Timestamp(other)
    }
}

impl ::core::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Timestamp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Tamper ID"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tampid(pub u32);
impl Tampid {
    #[doc="Tamper Input 0 Detected"]
    #[inline] pub fn tampid0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TAMPID0 != 0"]
    #[inline] pub fn test_tampid0(&self) -> bool {
        self.tampid0() != 0
    }

    #[doc="Sets the TAMPID0 field."]
    #[inline] pub fn set_tampid0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Tamper Input 1 Detected"]
    #[inline] pub fn tampid1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TAMPID1 != 0"]
    #[inline] pub fn test_tampid1(&self) -> bool {
        self.tampid1() != 0
    }

    #[doc="Sets the TAMPID1 field."]
    #[inline] pub fn set_tampid1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Tamper Input 2 Detected"]
    #[inline] pub fn tampid2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TAMPID2 != 0"]
    #[inline] pub fn test_tampid2(&self) -> bool {
        self.tampid2() != 0
    }

    #[doc="Sets the TAMPID2 field."]
    #[inline] pub fn set_tampid2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Tamper Input 3 Detected"]
    #[inline] pub fn tampid3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TAMPID3 != 0"]
    #[inline] pub fn test_tampid3(&self) -> bool {
        self.tampid3() != 0
    }

    #[doc="Sets the TAMPID3 field."]
    #[inline] pub fn set_tampid3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Tamper Input 4 Detected"]
    #[inline] pub fn tampid4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TAMPID4 != 0"]
    #[inline] pub fn test_tampid4(&self) -> bool {
        self.tampid4() != 0
    }

    #[doc="Sets the TAMPID4 field."]
    #[inline] pub fn set_tampid4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Tamper Event Detected"]
    #[inline] pub fn tampevt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if TAMPEVT != 0"]
    #[inline] pub fn test_tampevt(&self) -> bool {
        self.tampevt() != 0
    }

    #[doc="Sets the TAMPEVT field."]
    #[inline] pub fn set_tampevt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Tampid {
    #[inline]
    fn from(other: u32) -> Self {
         Tampid(other)
    }
}

impl ::core::fmt::Display for Tampid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tampid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tampid0() != 0 { try!(write!(f, " tampid0"))}
        if self.tampid1() != 0 { try!(write!(f, " tampid1"))}
        if self.tampid2() != 0 { try!(write!(f, " tampid2"))}
        if self.tampid3() != 0 { try!(write!(f, " tampid3"))}
        if self.tampid4() != 0 { try!(write!(f, " tampid4"))}
        if self.tampevt() != 0 { try!(write!(f, " tampevt"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Backup"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bkup(pub u32);
impl Bkup {
    #[doc="Backup"]
    #[inline] pub fn bkup(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if BKUP != 0"]
    #[inline] pub fn test_bkup(&self) -> bool {
        self.bkup() != 0
    }

    #[doc="Sets the BKUP field."]
    #[inline] pub fn set_bkup<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Bkup {
    #[inline]
    fn from(other: u32) -> Self {
         Bkup(other)
    }
}

impl ::core::fmt::Display for Bkup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bkup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

}
// End of mode0

#[doc="16-bit Counter with Two 16-bit Compares Cluster"]
pub mod mode1 {
    #[allow(unused_imports)] use super::*;
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[doc="16-bit Counter with Two 16-bit Compares Peripheral"]
    pub struct Mode1(pub usize);
impl Mode1 {
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ctrlb, 0x2)
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

    #[doc="Get the EVCTRL Register."]
    #[inline] pub fn evctrl_reg(&self) -> ::bobbin_mcu::register::Register<Evctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Evctrl, 0x4)
    }

    #[doc="Get the *mut pointer for the EVCTRL register."]
    #[inline] pub fn evctrl_mut(&self) -> *mut Evctrl { 
        self.evctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the EVCTRL register."]
    #[inline] pub fn evctrl_ptr(&self) -> *const Evctrl { 
        self.evctrl_reg().ptr()
    }

    #[doc="Read the EVCTRL register."]
    #[inline] pub fn evctrl(&self) -> Evctrl { 
        self.evctrl_reg().read()
    }

    #[doc="Write the EVCTRL register."]
    #[inline] pub fn write_evctrl(&self, value: Evctrl) -> &Self { 
        self.evctrl_reg().write(value);
        self
    }

    #[doc="Set the EVCTRL register."]
    #[inline] pub fn set_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
        self.evctrl_reg().set(f);
        self
    }

    #[doc="Modify the EVCTRL register."]
    #[inline] pub fn with_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
        self.evctrl_reg().with(f);
        self
    }

    #[doc="Get the INTENCLR Register."]
    #[inline] pub fn intenclr_reg(&self) -> ::bobbin_mcu::register::Register<Intenclr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenclr, 0x8)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenset, 0xa)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intflag, 0xc)
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

    #[doc="Get the DBGCTRL Register."]
    #[inline] pub fn dbgctrl_reg(&self) -> ::bobbin_mcu::register::Register<Dbgctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dbgctrl, 0xe)
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

    #[doc="Get the SYNCBUSY Register."]
    #[inline] pub fn syncbusy_reg(&self) -> ::bobbin_mcu::register::Register<Syncbusy> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Syncbusy, 0x10)
    }

    #[doc="Get the *mut pointer for the SYNCBUSY register."]
    #[inline] pub fn syncbusy_mut(&self) -> *mut Syncbusy { 
        self.syncbusy_reg().ptr()
    }

    #[doc="Get the *const pointer for the SYNCBUSY register."]
    #[inline] pub fn syncbusy_ptr(&self) -> *const Syncbusy { 
        self.syncbusy_reg().ptr()
    }

    #[doc="Read the SYNCBUSY register."]
    #[inline] pub fn syncbusy(&self) -> Syncbusy { 
        self.syncbusy_reg().read()
    }

    #[doc="Get the FREQCORR Register."]
    #[inline] pub fn freqcorr_reg(&self) -> ::bobbin_mcu::register::Register<Freqcorr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Freqcorr, 0x14)
    }

    #[doc="Get the *mut pointer for the FREQCORR register."]
    #[inline] pub fn freqcorr_mut(&self) -> *mut Freqcorr { 
        self.freqcorr_reg().ptr()
    }

    #[doc="Get the *const pointer for the FREQCORR register."]
    #[inline] pub fn freqcorr_ptr(&self) -> *const Freqcorr { 
        self.freqcorr_reg().ptr()
    }

    #[doc="Read the FREQCORR register."]
    #[inline] pub fn freqcorr(&self) -> Freqcorr { 
        self.freqcorr_reg().read()
    }

    #[doc="Write the FREQCORR register."]
    #[inline] pub fn write_freqcorr(&self, value: Freqcorr) -> &Self { 
        self.freqcorr_reg().write(value);
        self
    }

    #[doc="Set the FREQCORR register."]
    #[inline] pub fn set_freqcorr<F: FnOnce(Freqcorr) -> Freqcorr>(&self, f: F) -> &Self {
        self.freqcorr_reg().set(f);
        self
    }

    #[doc="Modify the FREQCORR register."]
    #[inline] pub fn with_freqcorr<F: FnOnce(Freqcorr) -> Freqcorr>(&self, f: F) -> &Self {
        self.freqcorr_reg().with(f);
        self
    }

    #[doc="Get the COUNT Register."]
    #[inline] pub fn count_reg(&self) -> ::bobbin_mcu::register::Register<Count> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Count, 0x18)
    }

    #[doc="Get the *mut pointer for the COUNT register."]
    #[inline] pub fn count_mut(&self) -> *mut Count { 
        self.count_reg().ptr()
    }

    #[doc="Get the *const pointer for the COUNT register."]
    #[inline] pub fn count_ptr(&self) -> *const Count { 
        self.count_reg().ptr()
    }

    #[doc="Read the COUNT register."]
    #[inline] pub fn count(&self) -> Count { 
        self.count_reg().read()
    }

    #[doc="Write the COUNT register."]
    #[inline] pub fn write_count(&self, value: Count) -> &Self { 
        self.count_reg().write(value);
        self
    }

    #[doc="Set the COUNT register."]
    #[inline] pub fn set_count<F: FnOnce(Count) -> Count>(&self, f: F) -> &Self {
        self.count_reg().set(f);
        self
    }

    #[doc="Modify the COUNT register."]
    #[inline] pub fn with_count<F: FnOnce(Count) -> Count>(&self, f: F) -> &Self {
        self.count_reg().with(f);
        self
    }

    #[doc="Get the PER Register."]
    #[inline] pub fn per_reg(&self) -> ::bobbin_mcu::register::Register<Per> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Per, 0x1c)
    }

    #[doc="Get the *mut pointer for the PER register."]
    #[inline] pub fn per_mut(&self) -> *mut Per { 
        self.per_reg().ptr()
    }

    #[doc="Get the *const pointer for the PER register."]
    #[inline] pub fn per_ptr(&self) -> *const Per { 
        self.per_reg().ptr()
    }

    #[doc="Read the PER register."]
    #[inline] pub fn per(&self) -> Per { 
        self.per_reg().read()
    }

    #[doc="Write the PER register."]
    #[inline] pub fn write_per(&self, value: Per) -> &Self { 
        self.per_reg().write(value);
        self
    }

    #[doc="Set the PER register."]
    #[inline] pub fn set_per<F: FnOnce(Per) -> Per>(&self, f: F) -> &Self {
        self.per_reg().set(f);
        self
    }

    #[doc="Modify the PER register."]
    #[inline] pub fn with_per<F: FnOnce(Per) -> Per>(&self, f: F) -> &Self {
        self.per_reg().with(f);
        self
    }

    #[doc="Get the COMP Register."]
    #[inline] pub fn comp_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Comp, ::bobbin_bits::R4> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Comp, 0x20, 0x2)
    }

    #[doc="Get the *mut pointer for the COMP register."]
    #[inline] pub fn comp_mut<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *mut Comp { 
        self.comp_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the COMP register."]
    #[inline] pub fn comp_ptr<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *const Comp { 
        self.comp_reg().ptr(index.into())
    }

    #[doc="Read the COMP register."]
    #[inline] pub fn comp<I: Into<::bobbin_bits::R4>>(&self, index: I) -> Comp { 
        self.comp_reg().read(index.into())
    }

    #[doc="Write the COMP register."]
    #[inline] pub fn write_comp<I: Into<::bobbin_bits::R4>>(&self, index: I, value: Comp) -> &Self {
        self.comp_reg().write(index.into(), value);
        self
    }

    #[doc="Set the COMP register."]
    #[inline] pub fn set_comp<I: Into<::bobbin_bits::R4>, F: FnOnce(Comp) -> Comp>(&self, index: I, f: F) -> &Self {
        self.comp_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the COMP register."]
    #[inline] pub fn with_comp<I: Into<::bobbin_bits::R4> + Copy, F: FnOnce(Comp) -> Comp>(&self, index: I, f: F) -> &Self {
        self.comp_reg().with(index.into(), f);
        self
    }

    #[doc="Get the GP Register."]
    #[inline] pub fn gp_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Gp, ::bobbin_bits::R4> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Gp, 0x40, 0x4)
    }

    #[doc="Get the *mut pointer for the GP register."]
    #[inline] pub fn gp_mut<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *mut Gp { 
        self.gp_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the GP register."]
    #[inline] pub fn gp_ptr<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *const Gp { 
        self.gp_reg().ptr(index.into())
    }

    #[doc="Read the GP register."]
    #[inline] pub fn gp<I: Into<::bobbin_bits::R4>>(&self, index: I) -> Gp { 
        self.gp_reg().read(index.into())
    }

    #[doc="Write the GP register."]
    #[inline] pub fn write_gp<I: Into<::bobbin_bits::R4>>(&self, index: I, value: Gp) -> &Self {
        self.gp_reg().write(index.into(), value);
        self
    }

    #[doc="Set the GP register."]
    #[inline] pub fn set_gp<I: Into<::bobbin_bits::R4>, F: FnOnce(Gp) -> Gp>(&self, index: I, f: F) -> &Self {
        self.gp_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the GP register."]
    #[inline] pub fn with_gp<I: Into<::bobbin_bits::R4> + Copy, F: FnOnce(Gp) -> Gp>(&self, index: I, f: F) -> &Self {
        self.gp_reg().with(index.into(), f);
        self
    }

    #[doc="Get the TAMPCTRL Register."]
    #[inline] pub fn tampctrl_reg(&self) -> ::bobbin_mcu::register::Register<Tampctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Tampctrl, 0x60)
    }

    #[doc="Get the *mut pointer for the TAMPCTRL register."]
    #[inline] pub fn tampctrl_mut(&self) -> *mut Tampctrl { 
        self.tampctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the TAMPCTRL register."]
    #[inline] pub fn tampctrl_ptr(&self) -> *const Tampctrl { 
        self.tampctrl_reg().ptr()
    }

    #[doc="Read the TAMPCTRL register."]
    #[inline] pub fn tampctrl(&self) -> Tampctrl { 
        self.tampctrl_reg().read()
    }

    #[doc="Write the TAMPCTRL register."]
    #[inline] pub fn write_tampctrl(&self, value: Tampctrl) -> &Self { 
        self.tampctrl_reg().write(value);
        self
    }

    #[doc="Set the TAMPCTRL register."]
    #[inline] pub fn set_tampctrl<F: FnOnce(Tampctrl) -> Tampctrl>(&self, f: F) -> &Self {
        self.tampctrl_reg().set(f);
        self
    }

    #[doc="Modify the TAMPCTRL register."]
    #[inline] pub fn with_tampctrl<F: FnOnce(Tampctrl) -> Tampctrl>(&self, f: F) -> &Self {
        self.tampctrl_reg().with(f);
        self
    }

    #[doc="Get the TIMESTAMP Register."]
    #[inline] pub fn timestamp_reg(&self) -> ::bobbin_mcu::register::Register<Timestamp> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Timestamp, 0x64)
    }

    #[doc="Get the *mut pointer for the TIMESTAMP register."]
    #[inline] pub fn timestamp_mut(&self) -> *mut Timestamp { 
        self.timestamp_reg().ptr()
    }

    #[doc="Get the *const pointer for the TIMESTAMP register."]
    #[inline] pub fn timestamp_ptr(&self) -> *const Timestamp { 
        self.timestamp_reg().ptr()
    }

    #[doc="Read the TIMESTAMP register."]
    #[inline] pub fn timestamp(&self) -> Timestamp { 
        self.timestamp_reg().read()
    }

    #[doc="Get the TAMPID Register."]
    #[inline] pub fn tampid_reg(&self) -> ::bobbin_mcu::register::Register<Tampid> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Tampid, 0x68)
    }

    #[doc="Get the *mut pointer for the TAMPID register."]
    #[inline] pub fn tampid_mut(&self) -> *mut Tampid { 
        self.tampid_reg().ptr()
    }

    #[doc="Get the *const pointer for the TAMPID register."]
    #[inline] pub fn tampid_ptr(&self) -> *const Tampid { 
        self.tampid_reg().ptr()
    }

    #[doc="Read the TAMPID register."]
    #[inline] pub fn tampid(&self) -> Tampid { 
        self.tampid_reg().read()
    }

    #[doc="Write the TAMPID register."]
    #[inline] pub fn write_tampid(&self, value: Tampid) -> &Self { 
        self.tampid_reg().write(value);
        self
    }

    #[doc="Set the TAMPID register."]
    #[inline] pub fn set_tampid<F: FnOnce(Tampid) -> Tampid>(&self, f: F) -> &Self {
        self.tampid_reg().set(f);
        self
    }

    #[doc="Modify the TAMPID register."]
    #[inline] pub fn with_tampid<F: FnOnce(Tampid) -> Tampid>(&self, f: F) -> &Self {
        self.tampid_reg().with(f);
        self
    }

    #[doc="Get the BKUP Register."]
    #[inline] pub fn bkup_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Bkup, ::bobbin_bits::R8> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Bkup, 0x80, 0x4)
    }

    #[doc="Get the *mut pointer for the BKUP register."]
    #[inline] pub fn bkup_mut<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *mut Bkup { 
        self.bkup_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the BKUP register."]
    #[inline] pub fn bkup_ptr<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *const Bkup { 
        self.bkup_reg().ptr(index.into())
    }

    #[doc="Read the BKUP register."]
    #[inline] pub fn bkup<I: Into<::bobbin_bits::R8>>(&self, index: I) -> Bkup { 
        self.bkup_reg().read(index.into())
    }

    #[doc="Write the BKUP register."]
    #[inline] pub fn write_bkup<I: Into<::bobbin_bits::R8>>(&self, index: I, value: Bkup) -> &Self {
        self.bkup_reg().write(index.into(), value);
        self
    }

    #[doc="Set the BKUP register."]
    #[inline] pub fn set_bkup<I: Into<::bobbin_bits::R8>, F: FnOnce(Bkup) -> Bkup>(&self, index: I, f: F) -> &Self {
        self.bkup_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the BKUP register."]
    #[inline] pub fn with_bkup<I: Into<::bobbin_bits::R8> + Copy, F: FnOnce(Bkup) -> Bkup>(&self, index: I, f: F) -> &Self {
        self.bkup_reg().with(index.into(), f);
        self
    }

}

#[doc="MODE1 Control A"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrla(pub u16);
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
        let value: u16 = value.into();
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
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Operating Mode"]
    #[inline] pub fn mode(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode(&self) -> bool {
        self.mode() != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Prescaler"]
    #[inline] pub fn prescaler(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if PRESCALER != 0"]
    #[inline] pub fn test_prescaler(&self) -> bool {
        self.prescaler() != 0
    }

    #[doc="Sets the PRESCALER field."]
    #[inline] pub fn set_prescaler<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="BKUP Registers Reset On Tamper Enable"]
    #[inline] pub fn bktrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if BKTRST != 0"]
    #[inline] pub fn test_bktrst(&self) -> bool {
        self.bktrst() != 0
    }

    #[doc="Sets the BKTRST field."]
    #[inline] pub fn set_bktrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="GP Registers Reset On Tamper Enable"]
    #[inline] pub fn gptrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if GPTRST != 0"]
    #[inline] pub fn test_gptrst(&self) -> bool {
        self.gptrst() != 0
    }

    #[doc="Sets the GPTRST field."]
    #[inline] pub fn set_gptrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Count Read Synchronization Enable"]
    #[inline] pub fn countsync(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if COUNTSYNC != 0"]
    #[inline] pub fn test_countsync(&self) -> bool {
        self.countsync() != 0
    }

    #[doc="Sets the COUNTSYNC field."]
    #[inline] pub fn set_countsync<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Ctrla {
    #[inline]
    fn from(other: u16) -> Self {
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
        if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
        if self.prescaler() != 0 { try!(write!(f, " prescaler=0x{:x}", self.prescaler()))}
        if self.bktrst() != 0 { try!(write!(f, " bktrst"))}
        if self.gptrst() != 0 { try!(write!(f, " gptrst"))}
        if self.countsync() != 0 { try!(write!(f, " countsync"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MODE1 Control B"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrlb(pub u16);
impl Ctrlb {
    #[doc="General Purpose 0 Enable"]
    #[inline] pub fn gp0en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if GP0EN != 0"]
    #[inline] pub fn test_gp0en(&self) -> bool {
        self.gp0en() != 0
    }

    #[doc="Sets the GP0EN field."]
    #[inline] pub fn set_gp0en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="General Purpose 2 Enable"]
    #[inline] pub fn gp2en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if GP2EN != 0"]
    #[inline] pub fn test_gp2en(&self) -> bool {
        self.gp2en() != 0
    }

    #[doc="Sets the GP2EN field."]
    #[inline] pub fn set_gp2en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Debouncer Majority Enable"]
    #[inline] pub fn debmaj(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DEBMAJ != 0"]
    #[inline] pub fn test_debmaj(&self) -> bool {
        self.debmaj() != 0
    }

    #[doc="Sets the DEBMAJ field."]
    #[inline] pub fn set_debmaj<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Debouncer Asynchronous Enable"]
    #[inline] pub fn debasync(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DEBASYNC != 0"]
    #[inline] pub fn test_debasync(&self) -> bool {
        self.debasync() != 0
    }

    #[doc="Sets the DEBASYNC field."]
    #[inline] pub fn set_debasync<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="RTC Output Enable"]
    #[inline] pub fn rtcout(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RTCOUT != 0"]
    #[inline] pub fn test_rtcout(&self) -> bool {
        self.rtcout() != 0
    }

    #[doc="Sets the RTCOUT field."]
    #[inline] pub fn set_rtcout<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="DMA Enable"]
    #[inline] pub fn dmaen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if DMAEN != 0"]
    #[inline] pub fn test_dmaen(&self) -> bool {
        self.dmaen() != 0
    }

    #[doc="Sets the DMAEN field."]
    #[inline] pub fn set_dmaen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Debounce Freqnuency"]
    #[inline] pub fn debf(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if DEBF != 0"]
    #[inline] pub fn test_debf(&self) -> bool {
        self.debf() != 0
    }

    #[doc="Sets the DEBF field."]
    #[inline] pub fn set_debf<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Active Layer Freqnuency"]
    #[inline] pub fn actf(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
    }

    #[doc="Returns true if ACTF != 0"]
    #[inline] pub fn test_actf(&self) -> bool {
        self.actf() != 0
    }

    #[doc="Sets the ACTF field."]
    #[inline] pub fn set_actf<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7 << 12);
        self.0 |= value << 12;
        self
    }

}

impl From<u16> for Ctrlb {
    #[inline]
    fn from(other: u16) -> Self {
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
        if self.gp0en() != 0 { try!(write!(f, " gp0en"))}
        if self.gp2en() != 0 { try!(write!(f, " gp2en"))}
        if self.debmaj() != 0 { try!(write!(f, " debmaj"))}
        if self.debasync() != 0 { try!(write!(f, " debasync"))}
        if self.rtcout() != 0 { try!(write!(f, " rtcout"))}
        if self.dmaen() != 0 { try!(write!(f, " dmaen"))}
        if self.debf() != 0 { try!(write!(f, " debf=0x{:x}", self.debf()))}
        if self.actf() != 0 { try!(write!(f, " actf=0x{:x}", self.actf()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MODE1 Event Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Evctrl(pub u32);
impl Evctrl {
    #[doc="Periodic Interval 0 Event Output Enable"]
    #[inline] pub fn pereo0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PEREO0 != 0"]
    #[inline] pub fn test_pereo0(&self) -> bool {
        self.pereo0() != 0
    }

    #[doc="Sets the PEREO0 field."]
    #[inline] pub fn set_pereo0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Periodic Interval 1 Event Output Enable"]
    #[inline] pub fn pereo1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PEREO1 != 0"]
    #[inline] pub fn test_pereo1(&self) -> bool {
        self.pereo1() != 0
    }

    #[doc="Sets the PEREO1 field."]
    #[inline] pub fn set_pereo1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Periodic Interval 2 Event Output Enable"]
    #[inline] pub fn pereo2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PEREO2 != 0"]
    #[inline] pub fn test_pereo2(&self) -> bool {
        self.pereo2() != 0
    }

    #[doc="Sets the PEREO2 field."]
    #[inline] pub fn set_pereo2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Periodic Interval 3 Event Output Enable"]
    #[inline] pub fn pereo3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PEREO3 != 0"]
    #[inline] pub fn test_pereo3(&self) -> bool {
        self.pereo3() != 0
    }

    #[doc="Sets the PEREO3 field."]
    #[inline] pub fn set_pereo3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Periodic Interval 4 Event Output Enable"]
    #[inline] pub fn pereo4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PEREO4 != 0"]
    #[inline] pub fn test_pereo4(&self) -> bool {
        self.pereo4() != 0
    }

    #[doc="Sets the PEREO4 field."]
    #[inline] pub fn set_pereo4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Periodic Interval 5 Event Output Enable"]
    #[inline] pub fn pereo5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PEREO5 != 0"]
    #[inline] pub fn test_pereo5(&self) -> bool {
        self.pereo5() != 0
    }

    #[doc="Sets the PEREO5 field."]
    #[inline] pub fn set_pereo5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Periodic Interval 6 Event Output Enable"]
    #[inline] pub fn pereo6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PEREO6 != 0"]
    #[inline] pub fn test_pereo6(&self) -> bool {
        self.pereo6() != 0
    }

    #[doc="Sets the PEREO6 field."]
    #[inline] pub fn set_pereo6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Periodic Interval 7 Event Output Enable"]
    #[inline] pub fn pereo7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PEREO7 != 0"]
    #[inline] pub fn test_pereo7(&self) -> bool {
        self.pereo7() != 0
    }

    #[doc="Sets the PEREO7 field."]
    #[inline] pub fn set_pereo7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Compare 0 Event Output Enable"]
    #[inline] pub fn cmpeo0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CMPEO0 != 0"]
    #[inline] pub fn test_cmpeo0(&self) -> bool {
        self.cmpeo0() != 0
    }

    #[doc="Sets the CMPEO0 field."]
    #[inline] pub fn set_cmpeo0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Compare 1 Event Output Enable"]
    #[inline] pub fn cmpeo1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CMPEO1 != 0"]
    #[inline] pub fn test_cmpeo1(&self) -> bool {
        self.cmpeo1() != 0
    }

    #[doc="Sets the CMPEO1 field."]
    #[inline] pub fn set_cmpeo1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Compare 2 Event Output Enable"]
    #[inline] pub fn cmpeo2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CMPEO2 != 0"]
    #[inline] pub fn test_cmpeo2(&self) -> bool {
        self.cmpeo2() != 0
    }

    #[doc="Sets the CMPEO2 field."]
    #[inline] pub fn set_cmpeo2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Compare 3 Event Output Enable"]
    #[inline] pub fn cmpeo3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CMPEO3 != 0"]
    #[inline] pub fn test_cmpeo3(&self) -> bool {
        self.cmpeo3() != 0
    }

    #[doc="Sets the CMPEO3 field."]
    #[inline] pub fn set_cmpeo3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Tamper Event Output Enable"]
    #[inline] pub fn tampereo(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TAMPEREO != 0"]
    #[inline] pub fn test_tampereo(&self) -> bool {
        self.tampereo() != 0
    }

    #[doc="Sets the TAMPEREO field."]
    #[inline] pub fn set_tampereo<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Overflow Event Output Enable"]
    #[inline] pub fn ovfeo(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if OVFEO != 0"]
    #[inline] pub fn test_ovfeo(&self) -> bool {
        self.ovfeo() != 0
    }

    #[doc="Sets the OVFEO field."]
    #[inline] pub fn set_ovfeo<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Tamper Event Input Enable"]
    #[inline] pub fn tampevei(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TAMPEVEI != 0"]
    #[inline] pub fn test_tampevei(&self) -> bool {
        self.tampevei() != 0
    }

    #[doc="Sets the TAMPEVEI field."]
    #[inline] pub fn set_tampevei<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Evctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Evctrl(other)
    }
}

impl ::core::fmt::Display for Evctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Evctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pereo0() != 0 { try!(write!(f, " pereo0"))}
        if self.pereo1() != 0 { try!(write!(f, " pereo1"))}
        if self.pereo2() != 0 { try!(write!(f, " pereo2"))}
        if self.pereo3() != 0 { try!(write!(f, " pereo3"))}
        if self.pereo4() != 0 { try!(write!(f, " pereo4"))}
        if self.pereo5() != 0 { try!(write!(f, " pereo5"))}
        if self.pereo6() != 0 { try!(write!(f, " pereo6"))}
        if self.pereo7() != 0 { try!(write!(f, " pereo7"))}
        if self.cmpeo0() != 0 { try!(write!(f, " cmpeo0"))}
        if self.cmpeo1() != 0 { try!(write!(f, " cmpeo1"))}
        if self.cmpeo2() != 0 { try!(write!(f, " cmpeo2"))}
        if self.cmpeo3() != 0 { try!(write!(f, " cmpeo3"))}
        if self.tampereo() != 0 { try!(write!(f, " tampereo"))}
        if self.ovfeo() != 0 { try!(write!(f, " ovfeo"))}
        if self.tampevei() != 0 { try!(write!(f, " tampevei"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MODE1 Interrupt Enable Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u16);
impl Intenclr {
    #[doc="Periodic Interval 0 Interrupt Enable"]
    #[inline] pub fn per0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PER0 != 0"]
    #[inline] pub fn test_per0(&self) -> bool {
        self.per0() != 0
    }

    #[doc="Sets the PER0 field."]
    #[inline] pub fn set_per0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Periodic Interval 1 Interrupt Enable"]
    #[inline] pub fn per1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PER1 != 0"]
    #[inline] pub fn test_per1(&self) -> bool {
        self.per1() != 0
    }

    #[doc="Sets the PER1 field."]
    #[inline] pub fn set_per1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Periodic Interval 2 Interrupt Enable"]
    #[inline] pub fn per2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PER2 != 0"]
    #[inline] pub fn test_per2(&self) -> bool {
        self.per2() != 0
    }

    #[doc="Sets the PER2 field."]
    #[inline] pub fn set_per2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Periodic Interval 3 Interrupt Enable"]
    #[inline] pub fn per3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PER3 != 0"]
    #[inline] pub fn test_per3(&self) -> bool {
        self.per3() != 0
    }

    #[doc="Sets the PER3 field."]
    #[inline] pub fn set_per3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Periodic Interval 4 Interrupt Enable"]
    #[inline] pub fn per4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PER4 != 0"]
    #[inline] pub fn test_per4(&self) -> bool {
        self.per4() != 0
    }

    #[doc="Sets the PER4 field."]
    #[inline] pub fn set_per4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Periodic Interval 5 Interrupt Enable"]
    #[inline] pub fn per5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PER5 != 0"]
    #[inline] pub fn test_per5(&self) -> bool {
        self.per5() != 0
    }

    #[doc="Sets the PER5 field."]
    #[inline] pub fn set_per5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Periodic Interval 6 Interrupt Enable"]
    #[inline] pub fn per6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PER6 != 0"]
    #[inline] pub fn test_per6(&self) -> bool {
        self.per6() != 0
    }

    #[doc="Sets the PER6 field."]
    #[inline] pub fn set_per6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Periodic Interval 7 Interrupt Enable"]
    #[inline] pub fn per7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PER7 != 0"]
    #[inline] pub fn test_per7(&self) -> bool {
        self.per7() != 0
    }

    #[doc="Sets the PER7 field."]
    #[inline] pub fn set_per7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Compare 0 Interrupt Enable"]
    #[inline] pub fn cmp0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CMP0 != 0"]
    #[inline] pub fn test_cmp0(&self) -> bool {
        self.cmp0() != 0
    }

    #[doc="Sets the CMP0 field."]
    #[inline] pub fn set_cmp0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Compare 1 Interrupt Enable"]
    #[inline] pub fn cmp1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CMP1 != 0"]
    #[inline] pub fn test_cmp1(&self) -> bool {
        self.cmp1() != 0
    }

    #[doc="Sets the CMP1 field."]
    #[inline] pub fn set_cmp1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Compare 2 Interrupt Enable"]
    #[inline] pub fn cmp2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CMP2 != 0"]
    #[inline] pub fn test_cmp2(&self) -> bool {
        self.cmp2() != 0
    }

    #[doc="Sets the CMP2 field."]
    #[inline] pub fn set_cmp2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Compare 3 Interrupt Enable"]
    #[inline] pub fn cmp3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CMP3 != 0"]
    #[inline] pub fn test_cmp3(&self) -> bool {
        self.cmp3() != 0
    }

    #[doc="Sets the CMP3 field."]
    #[inline] pub fn set_cmp3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Tamper Enable"]
    #[inline] pub fn tamper(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TAMPER != 0"]
    #[inline] pub fn test_tamper(&self) -> bool {
        self.tamper() != 0
    }

    #[doc="Sets the TAMPER field."]
    #[inline] pub fn set_tamper<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Overflow Interrupt Enable"]
    #[inline] pub fn ovf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if OVF != 0"]
    #[inline] pub fn test_ovf(&self) -> bool {
        self.ovf() != 0
    }

    #[doc="Sets the OVF field."]
    #[inline] pub fn set_ovf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Intenclr {
    #[inline]
    fn from(other: u16) -> Self {
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
        if self.per0() != 0 { try!(write!(f, " per0"))}
        if self.per1() != 0 { try!(write!(f, " per1"))}
        if self.per2() != 0 { try!(write!(f, " per2"))}
        if self.per3() != 0 { try!(write!(f, " per3"))}
        if self.per4() != 0 { try!(write!(f, " per4"))}
        if self.per5() != 0 { try!(write!(f, " per5"))}
        if self.per6() != 0 { try!(write!(f, " per6"))}
        if self.per7() != 0 { try!(write!(f, " per7"))}
        if self.cmp0() != 0 { try!(write!(f, " cmp0"))}
        if self.cmp1() != 0 { try!(write!(f, " cmp1"))}
        if self.cmp2() != 0 { try!(write!(f, " cmp2"))}
        if self.cmp3() != 0 { try!(write!(f, " cmp3"))}
        if self.tamper() != 0 { try!(write!(f, " tamper"))}
        if self.ovf() != 0 { try!(write!(f, " ovf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MODE1 Interrupt Enable Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u16);
impl Intenset {
    #[doc="Periodic Interval 0 Interrupt Enable"]
    #[inline] pub fn per0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PER0 != 0"]
    #[inline] pub fn test_per0(&self) -> bool {
        self.per0() != 0
    }

    #[doc="Sets the PER0 field."]
    #[inline] pub fn set_per0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Periodic Interval 1 Interrupt Enable"]
    #[inline] pub fn per1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PER1 != 0"]
    #[inline] pub fn test_per1(&self) -> bool {
        self.per1() != 0
    }

    #[doc="Sets the PER1 field."]
    #[inline] pub fn set_per1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Periodic Interval 2 Interrupt Enable"]
    #[inline] pub fn per2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PER2 != 0"]
    #[inline] pub fn test_per2(&self) -> bool {
        self.per2() != 0
    }

    #[doc="Sets the PER2 field."]
    #[inline] pub fn set_per2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Periodic Interval 3 Interrupt Enable"]
    #[inline] pub fn per3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PER3 != 0"]
    #[inline] pub fn test_per3(&self) -> bool {
        self.per3() != 0
    }

    #[doc="Sets the PER3 field."]
    #[inline] pub fn set_per3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Periodic Interval 4 Interrupt Enable"]
    #[inline] pub fn per4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PER4 != 0"]
    #[inline] pub fn test_per4(&self) -> bool {
        self.per4() != 0
    }

    #[doc="Sets the PER4 field."]
    #[inline] pub fn set_per4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Periodic Interval 5 Interrupt Enable"]
    #[inline] pub fn per5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PER5 != 0"]
    #[inline] pub fn test_per5(&self) -> bool {
        self.per5() != 0
    }

    #[doc="Sets the PER5 field."]
    #[inline] pub fn set_per5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Periodic Interval 6 Interrupt Enable"]
    #[inline] pub fn per6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PER6 != 0"]
    #[inline] pub fn test_per6(&self) -> bool {
        self.per6() != 0
    }

    #[doc="Sets the PER6 field."]
    #[inline] pub fn set_per6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Periodic Interval 7 Interrupt Enable"]
    #[inline] pub fn per7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PER7 != 0"]
    #[inline] pub fn test_per7(&self) -> bool {
        self.per7() != 0
    }

    #[doc="Sets the PER7 field."]
    #[inline] pub fn set_per7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Compare 0 Interrupt Enable"]
    #[inline] pub fn cmp0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CMP0 != 0"]
    #[inline] pub fn test_cmp0(&self) -> bool {
        self.cmp0() != 0
    }

    #[doc="Sets the CMP0 field."]
    #[inline] pub fn set_cmp0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Compare 1 Interrupt Enable"]
    #[inline] pub fn cmp1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CMP1 != 0"]
    #[inline] pub fn test_cmp1(&self) -> bool {
        self.cmp1() != 0
    }

    #[doc="Sets the CMP1 field."]
    #[inline] pub fn set_cmp1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Compare 2 Interrupt Enable"]
    #[inline] pub fn cmp2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CMP2 != 0"]
    #[inline] pub fn test_cmp2(&self) -> bool {
        self.cmp2() != 0
    }

    #[doc="Sets the CMP2 field."]
    #[inline] pub fn set_cmp2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Compare 3 Interrupt Enable"]
    #[inline] pub fn cmp3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CMP3 != 0"]
    #[inline] pub fn test_cmp3(&self) -> bool {
        self.cmp3() != 0
    }

    #[doc="Sets the CMP3 field."]
    #[inline] pub fn set_cmp3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Tamper Enable"]
    #[inline] pub fn tamper(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TAMPER != 0"]
    #[inline] pub fn test_tamper(&self) -> bool {
        self.tamper() != 0
    }

    #[doc="Sets the TAMPER field."]
    #[inline] pub fn set_tamper<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Overflow Interrupt Enable"]
    #[inline] pub fn ovf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if OVF != 0"]
    #[inline] pub fn test_ovf(&self) -> bool {
        self.ovf() != 0
    }

    #[doc="Sets the OVF field."]
    #[inline] pub fn set_ovf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Intenset {
    #[inline]
    fn from(other: u16) -> Self {
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
        if self.per0() != 0 { try!(write!(f, " per0"))}
        if self.per1() != 0 { try!(write!(f, " per1"))}
        if self.per2() != 0 { try!(write!(f, " per2"))}
        if self.per3() != 0 { try!(write!(f, " per3"))}
        if self.per4() != 0 { try!(write!(f, " per4"))}
        if self.per5() != 0 { try!(write!(f, " per5"))}
        if self.per6() != 0 { try!(write!(f, " per6"))}
        if self.per7() != 0 { try!(write!(f, " per7"))}
        if self.cmp0() != 0 { try!(write!(f, " cmp0"))}
        if self.cmp1() != 0 { try!(write!(f, " cmp1"))}
        if self.cmp2() != 0 { try!(write!(f, " cmp2"))}
        if self.cmp3() != 0 { try!(write!(f, " cmp3"))}
        if self.tamper() != 0 { try!(write!(f, " tamper"))}
        if self.ovf() != 0 { try!(write!(f, " ovf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MODE1 Interrupt Flag Status and Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u16);
impl Intflag {
    #[doc="Periodic Interval 0"]
    #[inline] pub fn per0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PER0 != 0"]
    #[inline] pub fn test_per0(&self) -> bool {
        self.per0() != 0
    }

    #[doc="Sets the PER0 field."]
    #[inline] pub fn set_per0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Periodic Interval 1"]
    #[inline] pub fn per1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PER1 != 0"]
    #[inline] pub fn test_per1(&self) -> bool {
        self.per1() != 0
    }

    #[doc="Sets the PER1 field."]
    #[inline] pub fn set_per1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Periodic Interval 2"]
    #[inline] pub fn per2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PER2 != 0"]
    #[inline] pub fn test_per2(&self) -> bool {
        self.per2() != 0
    }

    #[doc="Sets the PER2 field."]
    #[inline] pub fn set_per2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Periodic Interval 3"]
    #[inline] pub fn per3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PER3 != 0"]
    #[inline] pub fn test_per3(&self) -> bool {
        self.per3() != 0
    }

    #[doc="Sets the PER3 field."]
    #[inline] pub fn set_per3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Periodic Interval 4"]
    #[inline] pub fn per4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PER4 != 0"]
    #[inline] pub fn test_per4(&self) -> bool {
        self.per4() != 0
    }

    #[doc="Sets the PER4 field."]
    #[inline] pub fn set_per4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Periodic Interval 5"]
    #[inline] pub fn per5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PER5 != 0"]
    #[inline] pub fn test_per5(&self) -> bool {
        self.per5() != 0
    }

    #[doc="Sets the PER5 field."]
    #[inline] pub fn set_per5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Periodic Interval 6"]
    #[inline] pub fn per6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PER6 != 0"]
    #[inline] pub fn test_per6(&self) -> bool {
        self.per6() != 0
    }

    #[doc="Sets the PER6 field."]
    #[inline] pub fn set_per6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Periodic Interval 7"]
    #[inline] pub fn per7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PER7 != 0"]
    #[inline] pub fn test_per7(&self) -> bool {
        self.per7() != 0
    }

    #[doc="Sets the PER7 field."]
    #[inline] pub fn set_per7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Compare 0"]
    #[inline] pub fn cmp0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CMP0 != 0"]
    #[inline] pub fn test_cmp0(&self) -> bool {
        self.cmp0() != 0
    }

    #[doc="Sets the CMP0 field."]
    #[inline] pub fn set_cmp0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Compare 1"]
    #[inline] pub fn cmp1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CMP1 != 0"]
    #[inline] pub fn test_cmp1(&self) -> bool {
        self.cmp1() != 0
    }

    #[doc="Sets the CMP1 field."]
    #[inline] pub fn set_cmp1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Compare 2"]
    #[inline] pub fn cmp2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CMP2 != 0"]
    #[inline] pub fn test_cmp2(&self) -> bool {
        self.cmp2() != 0
    }

    #[doc="Sets the CMP2 field."]
    #[inline] pub fn set_cmp2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Compare 3"]
    #[inline] pub fn cmp3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CMP3 != 0"]
    #[inline] pub fn test_cmp3(&self) -> bool {
        self.cmp3() != 0
    }

    #[doc="Sets the CMP3 field."]
    #[inline] pub fn set_cmp3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Tamper"]
    #[inline] pub fn tamper(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TAMPER != 0"]
    #[inline] pub fn test_tamper(&self) -> bool {
        self.tamper() != 0
    }

    #[doc="Sets the TAMPER field."]
    #[inline] pub fn set_tamper<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Overflow"]
    #[inline] pub fn ovf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if OVF != 0"]
    #[inline] pub fn test_ovf(&self) -> bool {
        self.ovf() != 0
    }

    #[doc="Sets the OVF field."]
    #[inline] pub fn set_ovf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Intflag {
    #[inline]
    fn from(other: u16) -> Self {
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
        if self.per0() != 0 { try!(write!(f, " per0"))}
        if self.per1() != 0 { try!(write!(f, " per1"))}
        if self.per2() != 0 { try!(write!(f, " per2"))}
        if self.per3() != 0 { try!(write!(f, " per3"))}
        if self.per4() != 0 { try!(write!(f, " per4"))}
        if self.per5() != 0 { try!(write!(f, " per5"))}
        if self.per6() != 0 { try!(write!(f, " per6"))}
        if self.per7() != 0 { try!(write!(f, " per7"))}
        if self.cmp0() != 0 { try!(write!(f, " cmp0"))}
        if self.cmp1() != 0 { try!(write!(f, " cmp1"))}
        if self.cmp2() != 0 { try!(write!(f, " cmp2"))}
        if self.cmp3() != 0 { try!(write!(f, " cmp3"))}
        if self.tamper() != 0 { try!(write!(f, " tamper"))}
        if self.ovf() != 0 { try!(write!(f, " ovf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Debug Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
    #[doc="Run During Debug"]
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

#[doc="MODE1 Synchronization Busy Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Syncbusy(pub u32);
impl Syncbusy {
    #[doc="Software Reset Bit Busy"]
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

    #[doc="Enable Bit Busy"]
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

    #[doc="FREQCORR Register Busy"]
    #[inline] pub fn freqcorr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if FREQCORR != 0"]
    #[inline] pub fn test_freqcorr(&self) -> bool {
        self.freqcorr() != 0
    }

    #[doc="Sets the FREQCORR field."]
    #[inline] pub fn set_freqcorr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="COUNT Register Busy"]
    #[inline] pub fn count(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if COUNT != 0"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Sets the COUNT field."]
    #[inline] pub fn set_count<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="PER Register Busy"]
    #[inline] pub fn per(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PER != 0"]
    #[inline] pub fn test_per(&self) -> bool {
        self.per() != 0
    }

    #[doc="Sets the PER field."]
    #[inline] pub fn set_per<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="COMP 0 Register Busy"]
    #[inline] pub fn comp0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if COMP0 != 0"]
    #[inline] pub fn test_comp0(&self) -> bool {
        self.comp0() != 0
    }

    #[doc="Sets the COMP0 field."]
    #[inline] pub fn set_comp0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="COMP 1 Register Busy"]
    #[inline] pub fn comp1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if COMP1 != 0"]
    #[inline] pub fn test_comp1(&self) -> bool {
        self.comp1() != 0
    }

    #[doc="Sets the COMP1 field."]
    #[inline] pub fn set_comp1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="COMP 2 Register Busy"]
    #[inline] pub fn comp2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if COMP2 != 0"]
    #[inline] pub fn test_comp2(&self) -> bool {
        self.comp2() != 0
    }

    #[doc="Sets the COMP2 field."]
    #[inline] pub fn set_comp2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="COMP 3 Register Busy"]
    #[inline] pub fn comp3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if COMP3 != 0"]
    #[inline] pub fn test_comp3(&self) -> bool {
        self.comp3() != 0
    }

    #[doc="Sets the COMP3 field."]
    #[inline] pub fn set_comp3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Count Synchronization Enable Bit Busy"]
    #[inline] pub fn countsync(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if COUNTSYNC != 0"]
    #[inline] pub fn test_countsync(&self) -> bool {
        self.countsync() != 0
    }

    #[doc="Sets the COUNTSYNC field."]
    #[inline] pub fn set_countsync<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="General Purpose 0 Register Busy"]
    #[inline] pub fn gp0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if GP0 != 0"]
    #[inline] pub fn test_gp0(&self) -> bool {
        self.gp0() != 0
    }

    #[doc="Sets the GP0 field."]
    #[inline] pub fn set_gp0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="General Purpose 1 Register Busy"]
    #[inline] pub fn gp1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if GP1 != 0"]
    #[inline] pub fn test_gp1(&self) -> bool {
        self.gp1() != 0
    }

    #[doc="Sets the GP1 field."]
    #[inline] pub fn set_gp1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="General Purpose 2 Register Busy"]
    #[inline] pub fn gp2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if GP2 != 0"]
    #[inline] pub fn test_gp2(&self) -> bool {
        self.gp2() != 0
    }

    #[doc="Sets the GP2 field."]
    #[inline] pub fn set_gp2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="General Purpose 3 Register Busy"]
    #[inline] pub fn gp3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if GP3 != 0"]
    #[inline] pub fn test_gp3(&self) -> bool {
        self.gp3() != 0
    }

    #[doc="Sets the GP3 field."]
    #[inline] pub fn set_gp3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

}

impl From<u32> for Syncbusy {
    #[inline]
    fn from(other: u32) -> Self {
         Syncbusy(other)
    }
}

impl ::core::fmt::Display for Syncbusy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Syncbusy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.swrst() != 0 { try!(write!(f, " swrst"))}
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.freqcorr() != 0 { try!(write!(f, " freqcorr"))}
        if self.count() != 0 { try!(write!(f, " count"))}
        if self.per() != 0 { try!(write!(f, " per"))}
        if self.comp0() != 0 { try!(write!(f, " comp0"))}
        if self.comp1() != 0 { try!(write!(f, " comp1"))}
        if self.comp2() != 0 { try!(write!(f, " comp2"))}
        if self.comp3() != 0 { try!(write!(f, " comp3"))}
        if self.countsync() != 0 { try!(write!(f, " countsync"))}
        if self.gp0() != 0 { try!(write!(f, " gp0"))}
        if self.gp1() != 0 { try!(write!(f, " gp1"))}
        if self.gp2() != 0 { try!(write!(f, " gp2"))}
        if self.gp3() != 0 { try!(write!(f, " gp3"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Frequency Correction"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Freqcorr(pub u8);
impl Freqcorr {
    #[doc="Correction Value"]
    #[inline] pub fn value(&self) -> ::bobbin_bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="Returns true if VALUE != 0"]
    #[inline] pub fn test_value(&self) -> bool {
        self.value() != 0
    }

    #[doc="Sets the VALUE field."]
    #[inline] pub fn set_value<V: Into<::bobbin_bits::U7>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U7 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Correction Sign"]
    #[inline] pub fn sign(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SIGN != 0"]
    #[inline] pub fn test_sign(&self) -> bool {
        self.sign() != 0
    }

    #[doc="Sets the SIGN field."]
    #[inline] pub fn set_sign<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Freqcorr {
    #[inline]
    fn from(other: u8) -> Self {
         Freqcorr(other)
    }
}

impl ::core::fmt::Display for Freqcorr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Freqcorr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.value() != 0 { try!(write!(f, " value=0x{:x}", self.value()))}
        if self.sign() != 0 { try!(write!(f, " sign"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MODE1 Counter Value"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Count(pub u16);
impl Count {
    #[doc="Counter Value"]
    #[inline] pub fn count(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if COUNT != 0"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Sets the COUNT field."]
    #[inline] pub fn set_count<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Count {
    #[inline]
    fn from(other: u16) -> Self {
         Count(other)
    }
}

impl ::core::fmt::Display for Count {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Count {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MODE1 Counter Period"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Per(pub u16);
impl Per {
    #[doc="Counter Period"]
    #[inline] pub fn per(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if PER != 0"]
    #[inline] pub fn test_per(&self) -> bool {
        self.per() != 0
    }

    #[doc="Sets the PER field."]
    #[inline] pub fn set_per<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Per {
    #[inline]
    fn from(other: u16) -> Self {
         Per(other)
    }
}

impl ::core::fmt::Display for Per {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Per {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.per() != 0 { try!(write!(f, " per=0x{:x}", self.per()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MODE1 Compare n Value"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Comp(pub u16);
impl Comp {
    #[doc="Compare Value"]
    #[inline] pub fn comp(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if COMP != 0"]
    #[inline] pub fn test_comp(&self) -> bool {
        self.comp() != 0
    }

    #[doc="Sets the COMP field."]
    #[inline] pub fn set_comp<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Comp {
    #[inline]
    fn from(other: u16) -> Self {
         Comp(other)
    }
}

impl ::core::fmt::Display for Comp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Comp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.comp() != 0 { try!(write!(f, " comp=0x{:x}", self.comp()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="General Purpose"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Gp(pub u32);
impl Gp {
    #[doc="General Purpose"]
    #[inline] pub fn gp(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if GP != 0"]
    #[inline] pub fn test_gp(&self) -> bool {
        self.gp() != 0
    }

    #[doc="Sets the GP field."]
    #[inline] pub fn set_gp<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Gp {
    #[inline]
    fn from(other: u32) -> Self {
         Gp(other)
    }
}

impl ::core::fmt::Display for Gp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Gp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Tamper Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tampctrl(pub u32);
impl Tampctrl {
    #[doc="Tamper Input 0 Action"]
    #[inline] pub fn in0act(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if IN0ACT != 0"]
    #[inline] pub fn test_in0act(&self) -> bool {
        self.in0act() != 0
    }

    #[doc="Sets the IN0ACT field."]
    #[inline] pub fn set_in0act<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Tamper Input 1 Action"]
    #[inline] pub fn in1act(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if IN1ACT != 0"]
    #[inline] pub fn test_in1act(&self) -> bool {
        self.in1act() != 0
    }

    #[doc="Sets the IN1ACT field."]
    #[inline] pub fn set_in1act<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Tamper Input 2 Action"]
    #[inline] pub fn in2act(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if IN2ACT != 0"]
    #[inline] pub fn test_in2act(&self) -> bool {
        self.in2act() != 0
    }

    #[doc="Sets the IN2ACT field."]
    #[inline] pub fn set_in2act<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Tamper Input 3 Action"]
    #[inline] pub fn in3act(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if IN3ACT != 0"]
    #[inline] pub fn test_in3act(&self) -> bool {
        self.in3act() != 0
    }

    #[doc="Sets the IN3ACT field."]
    #[inline] pub fn set_in3act<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Tamper Input 4 Action"]
    #[inline] pub fn in4act(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if IN4ACT != 0"]
    #[inline] pub fn test_in4act(&self) -> bool {
        self.in4act() != 0
    }

    #[doc="Sets the IN4ACT field."]
    #[inline] pub fn set_in4act<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Tamper Level Select 0"]
    #[inline] pub fn tamlvl0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TAMLVL0 != 0"]
    #[inline] pub fn test_tamlvl0(&self) -> bool {
        self.tamlvl0() != 0
    }

    #[doc="Sets the TAMLVL0 field."]
    #[inline] pub fn set_tamlvl0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Tamper Level Select 1"]
    #[inline] pub fn tamlvl1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if TAMLVL1 != 0"]
    #[inline] pub fn test_tamlvl1(&self) -> bool {
        self.tamlvl1() != 0
    }

    #[doc="Sets the TAMLVL1 field."]
    #[inline] pub fn set_tamlvl1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Tamper Level Select 2"]
    #[inline] pub fn tamlvl2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if TAMLVL2 != 0"]
    #[inline] pub fn test_tamlvl2(&self) -> bool {
        self.tamlvl2() != 0
    }

    #[doc="Sets the TAMLVL2 field."]
    #[inline] pub fn set_tamlvl2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Tamper Level Select 3"]
    #[inline] pub fn tamlvl3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if TAMLVL3 != 0"]
    #[inline] pub fn test_tamlvl3(&self) -> bool {
        self.tamlvl3() != 0
    }

    #[doc="Sets the TAMLVL3 field."]
    #[inline] pub fn set_tamlvl3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Tamper Level Select 4"]
    #[inline] pub fn tamlvl4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if TAMLVL4 != 0"]
    #[inline] pub fn test_tamlvl4(&self) -> bool {
        self.tamlvl4() != 0
    }

    #[doc="Sets the TAMLVL4 field."]
    #[inline] pub fn set_tamlvl4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Debouncer Enable 0"]
    #[inline] pub fn debnc0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if DEBNC0 != 0"]
    #[inline] pub fn test_debnc0(&self) -> bool {
        self.debnc0() != 0
    }

    #[doc="Sets the DEBNC0 field."]
    #[inline] pub fn set_debnc0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Debouncer Enable 1"]
    #[inline] pub fn debnc1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if DEBNC1 != 0"]
    #[inline] pub fn test_debnc1(&self) -> bool {
        self.debnc1() != 0
    }

    #[doc="Sets the DEBNC1 field."]
    #[inline] pub fn set_debnc1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Debouncer Enable 2"]
    #[inline] pub fn debnc2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if DEBNC2 != 0"]
    #[inline] pub fn test_debnc2(&self) -> bool {
        self.debnc2() != 0
    }

    #[doc="Sets the DEBNC2 field."]
    #[inline] pub fn set_debnc2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Debouncer Enable 3"]
    #[inline] pub fn debnc3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if DEBNC3 != 0"]
    #[inline] pub fn test_debnc3(&self) -> bool {
        self.debnc3() != 0
    }

    #[doc="Sets the DEBNC3 field."]
    #[inline] pub fn set_debnc3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Debouncer Enable 4"]
    #[inline] pub fn debnc4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if DEBNC4 != 0"]
    #[inline] pub fn test_debnc4(&self) -> bool {
        self.debnc4() != 0
    }

    #[doc="Sets the DEBNC4 field."]
    #[inline] pub fn set_debnc4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

}

impl From<u32> for Tampctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Tampctrl(other)
    }
}

impl ::core::fmt::Display for Tampctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tampctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.in0act() != 0 { try!(write!(f, " in0act=0x{:x}", self.in0act()))}
        if self.in1act() != 0 { try!(write!(f, " in1act=0x{:x}", self.in1act()))}
        if self.in2act() != 0 { try!(write!(f, " in2act=0x{:x}", self.in2act()))}
        if self.in3act() != 0 { try!(write!(f, " in3act=0x{:x}", self.in3act()))}
        if self.in4act() != 0 { try!(write!(f, " in4act=0x{:x}", self.in4act()))}
        if self.tamlvl0() != 0 { try!(write!(f, " tamlvl0"))}
        if self.tamlvl1() != 0 { try!(write!(f, " tamlvl1"))}
        if self.tamlvl2() != 0 { try!(write!(f, " tamlvl2"))}
        if self.tamlvl3() != 0 { try!(write!(f, " tamlvl3"))}
        if self.tamlvl4() != 0 { try!(write!(f, " tamlvl4"))}
        if self.debnc0() != 0 { try!(write!(f, " debnc0"))}
        if self.debnc1() != 0 { try!(write!(f, " debnc1"))}
        if self.debnc2() != 0 { try!(write!(f, " debnc2"))}
        if self.debnc3() != 0 { try!(write!(f, " debnc3"))}
        if self.debnc4() != 0 { try!(write!(f, " debnc4"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MODE1 Timestamp"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Timestamp(pub u32);
impl Timestamp {
    #[doc="Count Timestamp Value"]
    #[inline] pub fn count(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if COUNT != 0"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Sets the COUNT field."]
    #[inline] pub fn set_count<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Timestamp {
    #[inline]
    fn from(other: u32) -> Self {
         Timestamp(other)
    }
}

impl ::core::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Timestamp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Tamper ID"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tampid(pub u32);
impl Tampid {
    #[doc="Tamper Input 0 Detected"]
    #[inline] pub fn tampid0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TAMPID0 != 0"]
    #[inline] pub fn test_tampid0(&self) -> bool {
        self.tampid0() != 0
    }

    #[doc="Sets the TAMPID0 field."]
    #[inline] pub fn set_tampid0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Tamper Input 1 Detected"]
    #[inline] pub fn tampid1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TAMPID1 != 0"]
    #[inline] pub fn test_tampid1(&self) -> bool {
        self.tampid1() != 0
    }

    #[doc="Sets the TAMPID1 field."]
    #[inline] pub fn set_tampid1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Tamper Input 2 Detected"]
    #[inline] pub fn tampid2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TAMPID2 != 0"]
    #[inline] pub fn test_tampid2(&self) -> bool {
        self.tampid2() != 0
    }

    #[doc="Sets the TAMPID2 field."]
    #[inline] pub fn set_tampid2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Tamper Input 3 Detected"]
    #[inline] pub fn tampid3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TAMPID3 != 0"]
    #[inline] pub fn test_tampid3(&self) -> bool {
        self.tampid3() != 0
    }

    #[doc="Sets the TAMPID3 field."]
    #[inline] pub fn set_tampid3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Tamper Input 4 Detected"]
    #[inline] pub fn tampid4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TAMPID4 != 0"]
    #[inline] pub fn test_tampid4(&self) -> bool {
        self.tampid4() != 0
    }

    #[doc="Sets the TAMPID4 field."]
    #[inline] pub fn set_tampid4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Tamper Event Detected"]
    #[inline] pub fn tampevt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if TAMPEVT != 0"]
    #[inline] pub fn test_tampevt(&self) -> bool {
        self.tampevt() != 0
    }

    #[doc="Sets the TAMPEVT field."]
    #[inline] pub fn set_tampevt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Tampid {
    #[inline]
    fn from(other: u32) -> Self {
         Tampid(other)
    }
}

impl ::core::fmt::Display for Tampid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tampid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tampid0() != 0 { try!(write!(f, " tampid0"))}
        if self.tampid1() != 0 { try!(write!(f, " tampid1"))}
        if self.tampid2() != 0 { try!(write!(f, " tampid2"))}
        if self.tampid3() != 0 { try!(write!(f, " tampid3"))}
        if self.tampid4() != 0 { try!(write!(f, " tampid4"))}
        if self.tampevt() != 0 { try!(write!(f, " tampevt"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Backup"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bkup(pub u32);
impl Bkup {
    #[doc="Backup"]
    #[inline] pub fn bkup(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if BKUP != 0"]
    #[inline] pub fn test_bkup(&self) -> bool {
        self.bkup() != 0
    }

    #[doc="Sets the BKUP field."]
    #[inline] pub fn set_bkup<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Bkup {
    #[inline]
    fn from(other: u32) -> Self {
         Bkup(other)
    }
}

impl ::core::fmt::Display for Bkup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bkup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

}
// End of mode1

#[doc="Clock/Calendar with Alarm Cluster"]
pub mod mode2 {
    #[allow(unused_imports)] use super::*;
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[doc="Clock/Calendar with Alarm Peripheral"]
    pub struct Mode2(pub usize);
impl Mode2 {
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ctrlb, 0x2)
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

    #[doc="Get the EVCTRL Register."]
    #[inline] pub fn evctrl_reg(&self) -> ::bobbin_mcu::register::Register<Evctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Evctrl, 0x4)
    }

    #[doc="Get the *mut pointer for the EVCTRL register."]
    #[inline] pub fn evctrl_mut(&self) -> *mut Evctrl { 
        self.evctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the EVCTRL register."]
    #[inline] pub fn evctrl_ptr(&self) -> *const Evctrl { 
        self.evctrl_reg().ptr()
    }

    #[doc="Read the EVCTRL register."]
    #[inline] pub fn evctrl(&self) -> Evctrl { 
        self.evctrl_reg().read()
    }

    #[doc="Write the EVCTRL register."]
    #[inline] pub fn write_evctrl(&self, value: Evctrl) -> &Self { 
        self.evctrl_reg().write(value);
        self
    }

    #[doc="Set the EVCTRL register."]
    #[inline] pub fn set_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
        self.evctrl_reg().set(f);
        self
    }

    #[doc="Modify the EVCTRL register."]
    #[inline] pub fn with_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
        self.evctrl_reg().with(f);
        self
    }

    #[doc="Get the INTENCLR Register."]
    #[inline] pub fn intenclr_reg(&self) -> ::bobbin_mcu::register::Register<Intenclr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenclr, 0x8)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenset, 0xa)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intflag, 0xc)
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

    #[doc="Get the DBGCTRL Register."]
    #[inline] pub fn dbgctrl_reg(&self) -> ::bobbin_mcu::register::Register<Dbgctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dbgctrl, 0xe)
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

    #[doc="Get the SYNCBUSY Register."]
    #[inline] pub fn syncbusy_reg(&self) -> ::bobbin_mcu::register::Register<Syncbusy> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Syncbusy, 0x10)
    }

    #[doc="Get the *mut pointer for the SYNCBUSY register."]
    #[inline] pub fn syncbusy_mut(&self) -> *mut Syncbusy { 
        self.syncbusy_reg().ptr()
    }

    #[doc="Get the *const pointer for the SYNCBUSY register."]
    #[inline] pub fn syncbusy_ptr(&self) -> *const Syncbusy { 
        self.syncbusy_reg().ptr()
    }

    #[doc="Read the SYNCBUSY register."]
    #[inline] pub fn syncbusy(&self) -> Syncbusy { 
        self.syncbusy_reg().read()
    }

    #[doc="Get the FREQCORR Register."]
    #[inline] pub fn freqcorr_reg(&self) -> ::bobbin_mcu::register::Register<Freqcorr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Freqcorr, 0x14)
    }

    #[doc="Get the *mut pointer for the FREQCORR register."]
    #[inline] pub fn freqcorr_mut(&self) -> *mut Freqcorr { 
        self.freqcorr_reg().ptr()
    }

    #[doc="Get the *const pointer for the FREQCORR register."]
    #[inline] pub fn freqcorr_ptr(&self) -> *const Freqcorr { 
        self.freqcorr_reg().ptr()
    }

    #[doc="Read the FREQCORR register."]
    #[inline] pub fn freqcorr(&self) -> Freqcorr { 
        self.freqcorr_reg().read()
    }

    #[doc="Write the FREQCORR register."]
    #[inline] pub fn write_freqcorr(&self, value: Freqcorr) -> &Self { 
        self.freqcorr_reg().write(value);
        self
    }

    #[doc="Set the FREQCORR register."]
    #[inline] pub fn set_freqcorr<F: FnOnce(Freqcorr) -> Freqcorr>(&self, f: F) -> &Self {
        self.freqcorr_reg().set(f);
        self
    }

    #[doc="Modify the FREQCORR register."]
    #[inline] pub fn with_freqcorr<F: FnOnce(Freqcorr) -> Freqcorr>(&self, f: F) -> &Self {
        self.freqcorr_reg().with(f);
        self
    }

    #[doc="Get the CLOCK Register."]
    #[inline] pub fn clock_reg(&self) -> ::bobbin_mcu::register::Register<Clock> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Clock, 0x18)
    }

    #[doc="Get the *mut pointer for the CLOCK register."]
    #[inline] pub fn clock_mut(&self) -> *mut Clock { 
        self.clock_reg().ptr()
    }

    #[doc="Get the *const pointer for the CLOCK register."]
    #[inline] pub fn clock_ptr(&self) -> *const Clock { 
        self.clock_reg().ptr()
    }

    #[doc="Read the CLOCK register."]
    #[inline] pub fn clock(&self) -> Clock { 
        self.clock_reg().read()
    }

    #[doc="Write the CLOCK register."]
    #[inline] pub fn write_clock(&self, value: Clock) -> &Self { 
        self.clock_reg().write(value);
        self
    }

    #[doc="Set the CLOCK register."]
    #[inline] pub fn set_clock<F: FnOnce(Clock) -> Clock>(&self, f: F) -> &Self {
        self.clock_reg().set(f);
        self
    }

    #[doc="Modify the CLOCK register."]
    #[inline] pub fn with_clock<F: FnOnce(Clock) -> Clock>(&self, f: F) -> &Self {
        self.clock_reg().with(f);
        self
    }

    #[doc="Get the ALARM Register."]
    #[inline] pub fn alarm_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Alarm, ::bobbin_bits::R2> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Alarm, 0x20, 0x8)
    }

    #[doc="Get the *mut pointer for the ALARM register."]
    #[inline] pub fn alarm_mut<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *mut Alarm { 
        self.alarm_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the ALARM register."]
    #[inline] pub fn alarm_ptr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *const Alarm { 
        self.alarm_reg().ptr(index.into())
    }

    #[doc="Read the ALARM register."]
    #[inline] pub fn alarm<I: Into<::bobbin_bits::R2>>(&self, index: I) -> Alarm { 
        self.alarm_reg().read(index.into())
    }

    #[doc="Write the ALARM register."]
    #[inline] pub fn write_alarm<I: Into<::bobbin_bits::R2>>(&self, index: I, value: Alarm) -> &Self {
        self.alarm_reg().write(index.into(), value);
        self
    }

    #[doc="Set the ALARM register."]
    #[inline] pub fn set_alarm<I: Into<::bobbin_bits::R2>, F: FnOnce(Alarm) -> Alarm>(&self, index: I, f: F) -> &Self {
        self.alarm_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the ALARM register."]
    #[inline] pub fn with_alarm<I: Into<::bobbin_bits::R2> + Copy, F: FnOnce(Alarm) -> Alarm>(&self, index: I, f: F) -> &Self {
        self.alarm_reg().with(index.into(), f);
        self
    }

    #[doc="Get the MASK Register."]
    #[inline] pub fn mask_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Mask, ::bobbin_bits::R2> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Mask, 0x24, 0x8)
    }

    #[doc="Get the *mut pointer for the MASK register."]
    #[inline] pub fn mask_mut<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *mut Mask { 
        self.mask_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the MASK register."]
    #[inline] pub fn mask_ptr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *const Mask { 
        self.mask_reg().ptr(index.into())
    }

    #[doc="Read the MASK register."]
    #[inline] pub fn mask<I: Into<::bobbin_bits::R2>>(&self, index: I) -> Mask { 
        self.mask_reg().read(index.into())
    }

    #[doc="Write the MASK register."]
    #[inline] pub fn write_mask<I: Into<::bobbin_bits::R2>>(&self, index: I, value: Mask) -> &Self {
        self.mask_reg().write(index.into(), value);
        self
    }

    #[doc="Set the MASK register."]
    #[inline] pub fn set_mask<I: Into<::bobbin_bits::R2>, F: FnOnce(Mask) -> Mask>(&self, index: I, f: F) -> &Self {
        self.mask_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the MASK register."]
    #[inline] pub fn with_mask<I: Into<::bobbin_bits::R2> + Copy, F: FnOnce(Mask) -> Mask>(&self, index: I, f: F) -> &Self {
        self.mask_reg().with(index.into(), f);
        self
    }

    #[doc="Get the GP Register."]
    #[inline] pub fn gp_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Gp, ::bobbin_bits::R4> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Gp, 0x40, 0x4)
    }

    #[doc="Get the *mut pointer for the GP register."]
    #[inline] pub fn gp_mut<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *mut Gp { 
        self.gp_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the GP register."]
    #[inline] pub fn gp_ptr<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *const Gp { 
        self.gp_reg().ptr(index.into())
    }

    #[doc="Read the GP register."]
    #[inline] pub fn gp<I: Into<::bobbin_bits::R4>>(&self, index: I) -> Gp { 
        self.gp_reg().read(index.into())
    }

    #[doc="Write the GP register."]
    #[inline] pub fn write_gp<I: Into<::bobbin_bits::R4>>(&self, index: I, value: Gp) -> &Self {
        self.gp_reg().write(index.into(), value);
        self
    }

    #[doc="Set the GP register."]
    #[inline] pub fn set_gp<I: Into<::bobbin_bits::R4>, F: FnOnce(Gp) -> Gp>(&self, index: I, f: F) -> &Self {
        self.gp_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the GP register."]
    #[inline] pub fn with_gp<I: Into<::bobbin_bits::R4> + Copy, F: FnOnce(Gp) -> Gp>(&self, index: I, f: F) -> &Self {
        self.gp_reg().with(index.into(), f);
        self
    }

    #[doc="Get the TAMPCTRL Register."]
    #[inline] pub fn tampctrl_reg(&self) -> ::bobbin_mcu::register::Register<Tampctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Tampctrl, 0x60)
    }

    #[doc="Get the *mut pointer for the TAMPCTRL register."]
    #[inline] pub fn tampctrl_mut(&self) -> *mut Tampctrl { 
        self.tampctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the TAMPCTRL register."]
    #[inline] pub fn tampctrl_ptr(&self) -> *const Tampctrl { 
        self.tampctrl_reg().ptr()
    }

    #[doc="Read the TAMPCTRL register."]
    #[inline] pub fn tampctrl(&self) -> Tampctrl { 
        self.tampctrl_reg().read()
    }

    #[doc="Write the TAMPCTRL register."]
    #[inline] pub fn write_tampctrl(&self, value: Tampctrl) -> &Self { 
        self.tampctrl_reg().write(value);
        self
    }

    #[doc="Set the TAMPCTRL register."]
    #[inline] pub fn set_tampctrl<F: FnOnce(Tampctrl) -> Tampctrl>(&self, f: F) -> &Self {
        self.tampctrl_reg().set(f);
        self
    }

    #[doc="Modify the TAMPCTRL register."]
    #[inline] pub fn with_tampctrl<F: FnOnce(Tampctrl) -> Tampctrl>(&self, f: F) -> &Self {
        self.tampctrl_reg().with(f);
        self
    }

    #[doc="Get the TIMESTAMP Register."]
    #[inline] pub fn timestamp_reg(&self) -> ::bobbin_mcu::register::Register<Timestamp> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Timestamp, 0x64)
    }

    #[doc="Get the *mut pointer for the TIMESTAMP register."]
    #[inline] pub fn timestamp_mut(&self) -> *mut Timestamp { 
        self.timestamp_reg().ptr()
    }

    #[doc="Get the *const pointer for the TIMESTAMP register."]
    #[inline] pub fn timestamp_ptr(&self) -> *const Timestamp { 
        self.timestamp_reg().ptr()
    }

    #[doc="Read the TIMESTAMP register."]
    #[inline] pub fn timestamp(&self) -> Timestamp { 
        self.timestamp_reg().read()
    }

    #[doc="Get the TAMPID Register."]
    #[inline] pub fn tampid_reg(&self) -> ::bobbin_mcu::register::Register<Tampid> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Tampid, 0x68)
    }

    #[doc="Get the *mut pointer for the TAMPID register."]
    #[inline] pub fn tampid_mut(&self) -> *mut Tampid { 
        self.tampid_reg().ptr()
    }

    #[doc="Get the *const pointer for the TAMPID register."]
    #[inline] pub fn tampid_ptr(&self) -> *const Tampid { 
        self.tampid_reg().ptr()
    }

    #[doc="Read the TAMPID register."]
    #[inline] pub fn tampid(&self) -> Tampid { 
        self.tampid_reg().read()
    }

    #[doc="Write the TAMPID register."]
    #[inline] pub fn write_tampid(&self, value: Tampid) -> &Self { 
        self.tampid_reg().write(value);
        self
    }

    #[doc="Set the TAMPID register."]
    #[inline] pub fn set_tampid<F: FnOnce(Tampid) -> Tampid>(&self, f: F) -> &Self {
        self.tampid_reg().set(f);
        self
    }

    #[doc="Modify the TAMPID register."]
    #[inline] pub fn with_tampid<F: FnOnce(Tampid) -> Tampid>(&self, f: F) -> &Self {
        self.tampid_reg().with(f);
        self
    }

    #[doc="Get the BKUP Register."]
    #[inline] pub fn bkup_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Bkup, ::bobbin_bits::R8> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Bkup, 0x80, 0x4)
    }

    #[doc="Get the *mut pointer for the BKUP register."]
    #[inline] pub fn bkup_mut<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *mut Bkup { 
        self.bkup_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the BKUP register."]
    #[inline] pub fn bkup_ptr<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *const Bkup { 
        self.bkup_reg().ptr(index.into())
    }

    #[doc="Read the BKUP register."]
    #[inline] pub fn bkup<I: Into<::bobbin_bits::R8>>(&self, index: I) -> Bkup { 
        self.bkup_reg().read(index.into())
    }

    #[doc="Write the BKUP register."]
    #[inline] pub fn write_bkup<I: Into<::bobbin_bits::R8>>(&self, index: I, value: Bkup) -> &Self {
        self.bkup_reg().write(index.into(), value);
        self
    }

    #[doc="Set the BKUP register."]
    #[inline] pub fn set_bkup<I: Into<::bobbin_bits::R8>, F: FnOnce(Bkup) -> Bkup>(&self, index: I, f: F) -> &Self {
        self.bkup_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the BKUP register."]
    #[inline] pub fn with_bkup<I: Into<::bobbin_bits::R8> + Copy, F: FnOnce(Bkup) -> Bkup>(&self, index: I, f: F) -> &Self {
        self.bkup_reg().with(index.into(), f);
        self
    }

}

#[doc="MODE2 Control A"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrla(pub u16);
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
        let value: u16 = value.into();
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
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Operating Mode"]
    #[inline] pub fn mode(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode(&self) -> bool {
        self.mode() != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Clock Representation"]
    #[inline] pub fn clkrep(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CLKREP != 0"]
    #[inline] pub fn test_clkrep(&self) -> bool {
        self.clkrep() != 0
    }

    #[doc="Sets the CLKREP field."]
    #[inline] pub fn set_clkrep<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Clear on Match"]
    #[inline] pub fn matchclr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if MATCHCLR != 0"]
    #[inline] pub fn test_matchclr(&self) -> bool {
        self.matchclr() != 0
    }

    #[doc="Sets the MATCHCLR field."]
    #[inline] pub fn set_matchclr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Prescaler"]
    #[inline] pub fn prescaler(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if PRESCALER != 0"]
    #[inline] pub fn test_prescaler(&self) -> bool {
        self.prescaler() != 0
    }

    #[doc="Sets the PRESCALER field."]
    #[inline] pub fn set_prescaler<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="BKUP Registers Reset On Tamper Enable"]
    #[inline] pub fn bktrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if BKTRST != 0"]
    #[inline] pub fn test_bktrst(&self) -> bool {
        self.bktrst() != 0
    }

    #[doc="Sets the BKTRST field."]
    #[inline] pub fn set_bktrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="GP Registers Reset On Tamper Enable"]
    #[inline] pub fn gptrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if GPTRST != 0"]
    #[inline] pub fn test_gptrst(&self) -> bool {
        self.gptrst() != 0
    }

    #[doc="Sets the GPTRST field."]
    #[inline] pub fn set_gptrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Clock Read Synchronization Enable"]
    #[inline] pub fn clocksync(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CLOCKSYNC != 0"]
    #[inline] pub fn test_clocksync(&self) -> bool {
        self.clocksync() != 0
    }

    #[doc="Sets the CLOCKSYNC field."]
    #[inline] pub fn set_clocksync<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Ctrla {
    #[inline]
    fn from(other: u16) -> Self {
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
        if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
        if self.clkrep() != 0 { try!(write!(f, " clkrep"))}
        if self.matchclr() != 0 { try!(write!(f, " matchclr"))}
        if self.prescaler() != 0 { try!(write!(f, " prescaler=0x{:x}", self.prescaler()))}
        if self.bktrst() != 0 { try!(write!(f, " bktrst"))}
        if self.gptrst() != 0 { try!(write!(f, " gptrst"))}
        if self.clocksync() != 0 { try!(write!(f, " clocksync"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MODE2 Control B"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrlb(pub u16);
impl Ctrlb {
    #[doc="General Purpose 0 Enable"]
    #[inline] pub fn gp0en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if GP0EN != 0"]
    #[inline] pub fn test_gp0en(&self) -> bool {
        self.gp0en() != 0
    }

    #[doc="Sets the GP0EN field."]
    #[inline] pub fn set_gp0en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="General Purpose 2 Enable"]
    #[inline] pub fn gp2en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if GP2EN != 0"]
    #[inline] pub fn test_gp2en(&self) -> bool {
        self.gp2en() != 0
    }

    #[doc="Sets the GP2EN field."]
    #[inline] pub fn set_gp2en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Debouncer Majority Enable"]
    #[inline] pub fn debmaj(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DEBMAJ != 0"]
    #[inline] pub fn test_debmaj(&self) -> bool {
        self.debmaj() != 0
    }

    #[doc="Sets the DEBMAJ field."]
    #[inline] pub fn set_debmaj<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Debouncer Asynchronous Enable"]
    #[inline] pub fn debasync(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DEBASYNC != 0"]
    #[inline] pub fn test_debasync(&self) -> bool {
        self.debasync() != 0
    }

    #[doc="Sets the DEBASYNC field."]
    #[inline] pub fn set_debasync<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="RTC Output Enable"]
    #[inline] pub fn rtcout(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RTCOUT != 0"]
    #[inline] pub fn test_rtcout(&self) -> bool {
        self.rtcout() != 0
    }

    #[doc="Sets the RTCOUT field."]
    #[inline] pub fn set_rtcout<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="DMA Enable"]
    #[inline] pub fn dmaen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if DMAEN != 0"]
    #[inline] pub fn test_dmaen(&self) -> bool {
        self.dmaen() != 0
    }

    #[doc="Sets the DMAEN field."]
    #[inline] pub fn set_dmaen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Debounce Freqnuency"]
    #[inline] pub fn debf(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if DEBF != 0"]
    #[inline] pub fn test_debf(&self) -> bool {
        self.debf() != 0
    }

    #[doc="Sets the DEBF field."]
    #[inline] pub fn set_debf<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Active Layer Freqnuency"]
    #[inline] pub fn actf(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
    }

    #[doc="Returns true if ACTF != 0"]
    #[inline] pub fn test_actf(&self) -> bool {
        self.actf() != 0
    }

    #[doc="Sets the ACTF field."]
    #[inline] pub fn set_actf<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7 << 12);
        self.0 |= value << 12;
        self
    }

}

impl From<u16> for Ctrlb {
    #[inline]
    fn from(other: u16) -> Self {
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
        if self.gp0en() != 0 { try!(write!(f, " gp0en"))}
        if self.gp2en() != 0 { try!(write!(f, " gp2en"))}
        if self.debmaj() != 0 { try!(write!(f, " debmaj"))}
        if self.debasync() != 0 { try!(write!(f, " debasync"))}
        if self.rtcout() != 0 { try!(write!(f, " rtcout"))}
        if self.dmaen() != 0 { try!(write!(f, " dmaen"))}
        if self.debf() != 0 { try!(write!(f, " debf=0x{:x}", self.debf()))}
        if self.actf() != 0 { try!(write!(f, " actf=0x{:x}", self.actf()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MODE2 Event Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Evctrl(pub u32);
impl Evctrl {
    #[doc="Periodic Interval 0 Event Output Enable"]
    #[inline] pub fn pereo0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PEREO0 != 0"]
    #[inline] pub fn test_pereo0(&self) -> bool {
        self.pereo0() != 0
    }

    #[doc="Sets the PEREO0 field."]
    #[inline] pub fn set_pereo0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Periodic Interval 1 Event Output Enable"]
    #[inline] pub fn pereo1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PEREO1 != 0"]
    #[inline] pub fn test_pereo1(&self) -> bool {
        self.pereo1() != 0
    }

    #[doc="Sets the PEREO1 field."]
    #[inline] pub fn set_pereo1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Periodic Interval 2 Event Output Enable"]
    #[inline] pub fn pereo2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PEREO2 != 0"]
    #[inline] pub fn test_pereo2(&self) -> bool {
        self.pereo2() != 0
    }

    #[doc="Sets the PEREO2 field."]
    #[inline] pub fn set_pereo2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Periodic Interval 3 Event Output Enable"]
    #[inline] pub fn pereo3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PEREO3 != 0"]
    #[inline] pub fn test_pereo3(&self) -> bool {
        self.pereo3() != 0
    }

    #[doc="Sets the PEREO3 field."]
    #[inline] pub fn set_pereo3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Periodic Interval 4 Event Output Enable"]
    #[inline] pub fn pereo4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PEREO4 != 0"]
    #[inline] pub fn test_pereo4(&self) -> bool {
        self.pereo4() != 0
    }

    #[doc="Sets the PEREO4 field."]
    #[inline] pub fn set_pereo4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Periodic Interval 5 Event Output Enable"]
    #[inline] pub fn pereo5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PEREO5 != 0"]
    #[inline] pub fn test_pereo5(&self) -> bool {
        self.pereo5() != 0
    }

    #[doc="Sets the PEREO5 field."]
    #[inline] pub fn set_pereo5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Periodic Interval 6 Event Output Enable"]
    #[inline] pub fn pereo6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PEREO6 != 0"]
    #[inline] pub fn test_pereo6(&self) -> bool {
        self.pereo6() != 0
    }

    #[doc="Sets the PEREO6 field."]
    #[inline] pub fn set_pereo6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Periodic Interval 7 Event Output Enable"]
    #[inline] pub fn pereo7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PEREO7 != 0"]
    #[inline] pub fn test_pereo7(&self) -> bool {
        self.pereo7() != 0
    }

    #[doc="Sets the PEREO7 field."]
    #[inline] pub fn set_pereo7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Alarm 0 Event Output Enable"]
    #[inline] pub fn alarmeo0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if ALARMEO0 != 0"]
    #[inline] pub fn test_alarmeo0(&self) -> bool {
        self.alarmeo0() != 0
    }

    #[doc="Sets the ALARMEO0 field."]
    #[inline] pub fn set_alarmeo0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Alarm 1 Event Output Enable"]
    #[inline] pub fn alarmeo1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if ALARMEO1 != 0"]
    #[inline] pub fn test_alarmeo1(&self) -> bool {
        self.alarmeo1() != 0
    }

    #[doc="Sets the ALARMEO1 field."]
    #[inline] pub fn set_alarmeo1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Tamper Event Output Enable"]
    #[inline] pub fn tampereo(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TAMPEREO != 0"]
    #[inline] pub fn test_tampereo(&self) -> bool {
        self.tampereo() != 0
    }

    #[doc="Sets the TAMPEREO field."]
    #[inline] pub fn set_tampereo<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Overflow Event Output Enable"]
    #[inline] pub fn ovfeo(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if OVFEO != 0"]
    #[inline] pub fn test_ovfeo(&self) -> bool {
        self.ovfeo() != 0
    }

    #[doc="Sets the OVFEO field."]
    #[inline] pub fn set_ovfeo<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Tamper Event Input Enable"]
    #[inline] pub fn tampevei(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TAMPEVEI != 0"]
    #[inline] pub fn test_tampevei(&self) -> bool {
        self.tampevei() != 0
    }

    #[doc="Sets the TAMPEVEI field."]
    #[inline] pub fn set_tampevei<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Evctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Evctrl(other)
    }
}

impl ::core::fmt::Display for Evctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Evctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pereo0() != 0 { try!(write!(f, " pereo0"))}
        if self.pereo1() != 0 { try!(write!(f, " pereo1"))}
        if self.pereo2() != 0 { try!(write!(f, " pereo2"))}
        if self.pereo3() != 0 { try!(write!(f, " pereo3"))}
        if self.pereo4() != 0 { try!(write!(f, " pereo4"))}
        if self.pereo5() != 0 { try!(write!(f, " pereo5"))}
        if self.pereo6() != 0 { try!(write!(f, " pereo6"))}
        if self.pereo7() != 0 { try!(write!(f, " pereo7"))}
        if self.alarmeo0() != 0 { try!(write!(f, " alarmeo0"))}
        if self.alarmeo1() != 0 { try!(write!(f, " alarmeo1"))}
        if self.tampereo() != 0 { try!(write!(f, " tampereo"))}
        if self.ovfeo() != 0 { try!(write!(f, " ovfeo"))}
        if self.tampevei() != 0 { try!(write!(f, " tampevei"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MODE2 Interrupt Enable Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u16);
impl Intenclr {
    #[doc="Periodic Interval 0 Interrupt Enable"]
    #[inline] pub fn per0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PER0 != 0"]
    #[inline] pub fn test_per0(&self) -> bool {
        self.per0() != 0
    }

    #[doc="Sets the PER0 field."]
    #[inline] pub fn set_per0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Periodic Interval 1 Interrupt Enable"]
    #[inline] pub fn per1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PER1 != 0"]
    #[inline] pub fn test_per1(&self) -> bool {
        self.per1() != 0
    }

    #[doc="Sets the PER1 field."]
    #[inline] pub fn set_per1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Periodic Interval 2 Interrupt Enable"]
    #[inline] pub fn per2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PER2 != 0"]
    #[inline] pub fn test_per2(&self) -> bool {
        self.per2() != 0
    }

    #[doc="Sets the PER2 field."]
    #[inline] pub fn set_per2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Periodic Interval 3 Interrupt Enable"]
    #[inline] pub fn per3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PER3 != 0"]
    #[inline] pub fn test_per3(&self) -> bool {
        self.per3() != 0
    }

    #[doc="Sets the PER3 field."]
    #[inline] pub fn set_per3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Periodic Interval 4 Interrupt Enable"]
    #[inline] pub fn per4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PER4 != 0"]
    #[inline] pub fn test_per4(&self) -> bool {
        self.per4() != 0
    }

    #[doc="Sets the PER4 field."]
    #[inline] pub fn set_per4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Periodic Interval 5 Interrupt Enable"]
    #[inline] pub fn per5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PER5 != 0"]
    #[inline] pub fn test_per5(&self) -> bool {
        self.per5() != 0
    }

    #[doc="Sets the PER5 field."]
    #[inline] pub fn set_per5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Periodic Interval 6 Interrupt Enable"]
    #[inline] pub fn per6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PER6 != 0"]
    #[inline] pub fn test_per6(&self) -> bool {
        self.per6() != 0
    }

    #[doc="Sets the PER6 field."]
    #[inline] pub fn set_per6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Periodic Interval 7 Interrupt Enable"]
    #[inline] pub fn per7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PER7 != 0"]
    #[inline] pub fn test_per7(&self) -> bool {
        self.per7() != 0
    }

    #[doc="Sets the PER7 field."]
    #[inline] pub fn set_per7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Alarm 0 Interrupt Enable"]
    #[inline] pub fn alarm0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if ALARM0 != 0"]
    #[inline] pub fn test_alarm0(&self) -> bool {
        self.alarm0() != 0
    }

    #[doc="Sets the ALARM0 field."]
    #[inline] pub fn set_alarm0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Alarm 1 Interrupt Enable"]
    #[inline] pub fn alarm1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if ALARM1 != 0"]
    #[inline] pub fn test_alarm1(&self) -> bool {
        self.alarm1() != 0
    }

    #[doc="Sets the ALARM1 field."]
    #[inline] pub fn set_alarm1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Tamper Enable"]
    #[inline] pub fn tamper(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TAMPER != 0"]
    #[inline] pub fn test_tamper(&self) -> bool {
        self.tamper() != 0
    }

    #[doc="Sets the TAMPER field."]
    #[inline] pub fn set_tamper<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Overflow Interrupt Enable"]
    #[inline] pub fn ovf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if OVF != 0"]
    #[inline] pub fn test_ovf(&self) -> bool {
        self.ovf() != 0
    }

    #[doc="Sets the OVF field."]
    #[inline] pub fn set_ovf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Intenclr {
    #[inline]
    fn from(other: u16) -> Self {
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
        if self.per0() != 0 { try!(write!(f, " per0"))}
        if self.per1() != 0 { try!(write!(f, " per1"))}
        if self.per2() != 0 { try!(write!(f, " per2"))}
        if self.per3() != 0 { try!(write!(f, " per3"))}
        if self.per4() != 0 { try!(write!(f, " per4"))}
        if self.per5() != 0 { try!(write!(f, " per5"))}
        if self.per6() != 0 { try!(write!(f, " per6"))}
        if self.per7() != 0 { try!(write!(f, " per7"))}
        if self.alarm0() != 0 { try!(write!(f, " alarm0"))}
        if self.alarm1() != 0 { try!(write!(f, " alarm1"))}
        if self.tamper() != 0 { try!(write!(f, " tamper"))}
        if self.ovf() != 0 { try!(write!(f, " ovf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MODE2 Interrupt Enable Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u16);
impl Intenset {
    #[doc="Periodic Interval 0 Enable"]
    #[inline] pub fn per0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PER0 != 0"]
    #[inline] pub fn test_per0(&self) -> bool {
        self.per0() != 0
    }

    #[doc="Sets the PER0 field."]
    #[inline] pub fn set_per0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Periodic Interval 1 Enable"]
    #[inline] pub fn per1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PER1 != 0"]
    #[inline] pub fn test_per1(&self) -> bool {
        self.per1() != 0
    }

    #[doc="Sets the PER1 field."]
    #[inline] pub fn set_per1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Periodic Interval 2 Enable"]
    #[inline] pub fn per2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PER2 != 0"]
    #[inline] pub fn test_per2(&self) -> bool {
        self.per2() != 0
    }

    #[doc="Sets the PER2 field."]
    #[inline] pub fn set_per2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Periodic Interval 3 Enable"]
    #[inline] pub fn per3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PER3 != 0"]
    #[inline] pub fn test_per3(&self) -> bool {
        self.per3() != 0
    }

    #[doc="Sets the PER3 field."]
    #[inline] pub fn set_per3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Periodic Interval 4 Enable"]
    #[inline] pub fn per4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PER4 != 0"]
    #[inline] pub fn test_per4(&self) -> bool {
        self.per4() != 0
    }

    #[doc="Sets the PER4 field."]
    #[inline] pub fn set_per4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Periodic Interval 5 Enable"]
    #[inline] pub fn per5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PER5 != 0"]
    #[inline] pub fn test_per5(&self) -> bool {
        self.per5() != 0
    }

    #[doc="Sets the PER5 field."]
    #[inline] pub fn set_per5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Periodic Interval 6 Enable"]
    #[inline] pub fn per6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PER6 != 0"]
    #[inline] pub fn test_per6(&self) -> bool {
        self.per6() != 0
    }

    #[doc="Sets the PER6 field."]
    #[inline] pub fn set_per6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Periodic Interval 7 Enable"]
    #[inline] pub fn per7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PER7 != 0"]
    #[inline] pub fn test_per7(&self) -> bool {
        self.per7() != 0
    }

    #[doc="Sets the PER7 field."]
    #[inline] pub fn set_per7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Alarm 0 Interrupt Enable"]
    #[inline] pub fn alarm0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if ALARM0 != 0"]
    #[inline] pub fn test_alarm0(&self) -> bool {
        self.alarm0() != 0
    }

    #[doc="Sets the ALARM0 field."]
    #[inline] pub fn set_alarm0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Alarm 1 Interrupt Enable"]
    #[inline] pub fn alarm1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if ALARM1 != 0"]
    #[inline] pub fn test_alarm1(&self) -> bool {
        self.alarm1() != 0
    }

    #[doc="Sets the ALARM1 field."]
    #[inline] pub fn set_alarm1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Tamper Enable"]
    #[inline] pub fn tamper(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TAMPER != 0"]
    #[inline] pub fn test_tamper(&self) -> bool {
        self.tamper() != 0
    }

    #[doc="Sets the TAMPER field."]
    #[inline] pub fn set_tamper<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Overflow Interrupt Enable"]
    #[inline] pub fn ovf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if OVF != 0"]
    #[inline] pub fn test_ovf(&self) -> bool {
        self.ovf() != 0
    }

    #[doc="Sets the OVF field."]
    #[inline] pub fn set_ovf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Intenset {
    #[inline]
    fn from(other: u16) -> Self {
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
        if self.per0() != 0 { try!(write!(f, " per0"))}
        if self.per1() != 0 { try!(write!(f, " per1"))}
        if self.per2() != 0 { try!(write!(f, " per2"))}
        if self.per3() != 0 { try!(write!(f, " per3"))}
        if self.per4() != 0 { try!(write!(f, " per4"))}
        if self.per5() != 0 { try!(write!(f, " per5"))}
        if self.per6() != 0 { try!(write!(f, " per6"))}
        if self.per7() != 0 { try!(write!(f, " per7"))}
        if self.alarm0() != 0 { try!(write!(f, " alarm0"))}
        if self.alarm1() != 0 { try!(write!(f, " alarm1"))}
        if self.tamper() != 0 { try!(write!(f, " tamper"))}
        if self.ovf() != 0 { try!(write!(f, " ovf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MODE2 Interrupt Flag Status and Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u16);
impl Intflag {
    #[doc="Periodic Interval 0"]
    #[inline] pub fn per0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PER0 != 0"]
    #[inline] pub fn test_per0(&self) -> bool {
        self.per0() != 0
    }

    #[doc="Sets the PER0 field."]
    #[inline] pub fn set_per0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Periodic Interval 1"]
    #[inline] pub fn per1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PER1 != 0"]
    #[inline] pub fn test_per1(&self) -> bool {
        self.per1() != 0
    }

    #[doc="Sets the PER1 field."]
    #[inline] pub fn set_per1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Periodic Interval 2"]
    #[inline] pub fn per2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PER2 != 0"]
    #[inline] pub fn test_per2(&self) -> bool {
        self.per2() != 0
    }

    #[doc="Sets the PER2 field."]
    #[inline] pub fn set_per2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Periodic Interval 3"]
    #[inline] pub fn per3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PER3 != 0"]
    #[inline] pub fn test_per3(&self) -> bool {
        self.per3() != 0
    }

    #[doc="Sets the PER3 field."]
    #[inline] pub fn set_per3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Periodic Interval 4"]
    #[inline] pub fn per4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PER4 != 0"]
    #[inline] pub fn test_per4(&self) -> bool {
        self.per4() != 0
    }

    #[doc="Sets the PER4 field."]
    #[inline] pub fn set_per4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Periodic Interval 5"]
    #[inline] pub fn per5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PER5 != 0"]
    #[inline] pub fn test_per5(&self) -> bool {
        self.per5() != 0
    }

    #[doc="Sets the PER5 field."]
    #[inline] pub fn set_per5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Periodic Interval 6"]
    #[inline] pub fn per6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PER6 != 0"]
    #[inline] pub fn test_per6(&self) -> bool {
        self.per6() != 0
    }

    #[doc="Sets the PER6 field."]
    #[inline] pub fn set_per6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Periodic Interval 7"]
    #[inline] pub fn per7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PER7 != 0"]
    #[inline] pub fn test_per7(&self) -> bool {
        self.per7() != 0
    }

    #[doc="Sets the PER7 field."]
    #[inline] pub fn set_per7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Alarm 0"]
    #[inline] pub fn alarm0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if ALARM0 != 0"]
    #[inline] pub fn test_alarm0(&self) -> bool {
        self.alarm0() != 0
    }

    #[doc="Sets the ALARM0 field."]
    #[inline] pub fn set_alarm0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Alarm 1"]
    #[inline] pub fn alarm1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if ALARM1 != 0"]
    #[inline] pub fn test_alarm1(&self) -> bool {
        self.alarm1() != 0
    }

    #[doc="Sets the ALARM1 field."]
    #[inline] pub fn set_alarm1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Tamper"]
    #[inline] pub fn tamper(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TAMPER != 0"]
    #[inline] pub fn test_tamper(&self) -> bool {
        self.tamper() != 0
    }

    #[doc="Sets the TAMPER field."]
    #[inline] pub fn set_tamper<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Overflow"]
    #[inline] pub fn ovf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if OVF != 0"]
    #[inline] pub fn test_ovf(&self) -> bool {
        self.ovf() != 0
    }

    #[doc="Sets the OVF field."]
    #[inline] pub fn set_ovf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Intflag {
    #[inline]
    fn from(other: u16) -> Self {
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
        if self.per0() != 0 { try!(write!(f, " per0"))}
        if self.per1() != 0 { try!(write!(f, " per1"))}
        if self.per2() != 0 { try!(write!(f, " per2"))}
        if self.per3() != 0 { try!(write!(f, " per3"))}
        if self.per4() != 0 { try!(write!(f, " per4"))}
        if self.per5() != 0 { try!(write!(f, " per5"))}
        if self.per6() != 0 { try!(write!(f, " per6"))}
        if self.per7() != 0 { try!(write!(f, " per7"))}
        if self.alarm0() != 0 { try!(write!(f, " alarm0"))}
        if self.alarm1() != 0 { try!(write!(f, " alarm1"))}
        if self.tamper() != 0 { try!(write!(f, " tamper"))}
        if self.ovf() != 0 { try!(write!(f, " ovf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Debug Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
    #[doc="Run During Debug"]
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

#[doc="MODE2 Synchronization Busy Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Syncbusy(pub u32);
impl Syncbusy {
    #[doc="Software Reset Bit Busy"]
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

    #[doc="Enable Bit Busy"]
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

    #[doc="FREQCORR Register Busy"]
    #[inline] pub fn freqcorr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if FREQCORR != 0"]
    #[inline] pub fn test_freqcorr(&self) -> bool {
        self.freqcorr() != 0
    }

    #[doc="Sets the FREQCORR field."]
    #[inline] pub fn set_freqcorr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="CLOCK Register Busy"]
    #[inline] pub fn clock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CLOCK != 0"]
    #[inline] pub fn test_clock(&self) -> bool {
        self.clock() != 0
    }

    #[doc="Sets the CLOCK field."]
    #[inline] pub fn set_clock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="ALARM 0 Register Busy"]
    #[inline] pub fn alarm0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ALARM0 != 0"]
    #[inline] pub fn test_alarm0(&self) -> bool {
        self.alarm0() != 0
    }

    #[doc="Sets the ALARM0 field."]
    #[inline] pub fn set_alarm0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="ALARM 1 Register Busy"]
    #[inline] pub fn alarm1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if ALARM1 != 0"]
    #[inline] pub fn test_alarm1(&self) -> bool {
        self.alarm1() != 0
    }

    #[doc="Sets the ALARM1 field."]
    #[inline] pub fn set_alarm1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="MASK 0 Register Busy"]
    #[inline] pub fn mask0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if MASK0 != 0"]
    #[inline] pub fn test_mask0(&self) -> bool {
        self.mask0() != 0
    }

    #[doc="Sets the MASK0 field."]
    #[inline] pub fn set_mask0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="MASK 1 Register Busy"]
    #[inline] pub fn mask1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if MASK1 != 0"]
    #[inline] pub fn test_mask1(&self) -> bool {
        self.mask1() != 0
    }

    #[doc="Sets the MASK1 field."]
    #[inline] pub fn set_mask1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Clock Synchronization Enable Bit Busy"]
    #[inline] pub fn clocksync(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CLOCKSYNC != 0"]
    #[inline] pub fn test_clocksync(&self) -> bool {
        self.clocksync() != 0
    }

    #[doc="Sets the CLOCKSYNC field."]
    #[inline] pub fn set_clocksync<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="General Purpose 0 Register Busy"]
    #[inline] pub fn gp0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if GP0 != 0"]
    #[inline] pub fn test_gp0(&self) -> bool {
        self.gp0() != 0
    }

    #[doc="Sets the GP0 field."]
    #[inline] pub fn set_gp0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="General Purpose 1 Register Busy"]
    #[inline] pub fn gp1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if GP1 != 0"]
    #[inline] pub fn test_gp1(&self) -> bool {
        self.gp1() != 0
    }

    #[doc="Sets the GP1 field."]
    #[inline] pub fn set_gp1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="General Purpose 2 Register Busy"]
    #[inline] pub fn gp2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if GP2 != 0"]
    #[inline] pub fn test_gp2(&self) -> bool {
        self.gp2() != 0
    }

    #[doc="Sets the GP2 field."]
    #[inline] pub fn set_gp2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="General Purpose 3 Register Busy"]
    #[inline] pub fn gp3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if GP3 != 0"]
    #[inline] pub fn test_gp3(&self) -> bool {
        self.gp3() != 0
    }

    #[doc="Sets the GP3 field."]
    #[inline] pub fn set_gp3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

}

impl From<u32> for Syncbusy {
    #[inline]
    fn from(other: u32) -> Self {
         Syncbusy(other)
    }
}

impl ::core::fmt::Display for Syncbusy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Syncbusy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.swrst() != 0 { try!(write!(f, " swrst"))}
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.freqcorr() != 0 { try!(write!(f, " freqcorr"))}
        if self.clock() != 0 { try!(write!(f, " clock"))}
        if self.alarm0() != 0 { try!(write!(f, " alarm0"))}
        if self.alarm1() != 0 { try!(write!(f, " alarm1"))}
        if self.mask0() != 0 { try!(write!(f, " mask0"))}
        if self.mask1() != 0 { try!(write!(f, " mask1"))}
        if self.clocksync() != 0 { try!(write!(f, " clocksync"))}
        if self.gp0() != 0 { try!(write!(f, " gp0"))}
        if self.gp1() != 0 { try!(write!(f, " gp1"))}
        if self.gp2() != 0 { try!(write!(f, " gp2"))}
        if self.gp3() != 0 { try!(write!(f, " gp3"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Frequency Correction"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Freqcorr(pub u8);
impl Freqcorr {
    #[doc="Correction Value"]
    #[inline] pub fn value(&self) -> ::bobbin_bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="Returns true if VALUE != 0"]
    #[inline] pub fn test_value(&self) -> bool {
        self.value() != 0
    }

    #[doc="Sets the VALUE field."]
    #[inline] pub fn set_value<V: Into<::bobbin_bits::U7>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U7 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Correction Sign"]
    #[inline] pub fn sign(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SIGN != 0"]
    #[inline] pub fn test_sign(&self) -> bool {
        self.sign() != 0
    }

    #[doc="Sets the SIGN field."]
    #[inline] pub fn set_sign<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Freqcorr {
    #[inline]
    fn from(other: u8) -> Self {
         Freqcorr(other)
    }
}

impl ::core::fmt::Display for Freqcorr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Freqcorr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.value() != 0 { try!(write!(f, " value=0x{:x}", self.value()))}
        if self.sign() != 0 { try!(write!(f, " sign"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MODE2 Clock Value"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clock(pub u32);
impl Clock {
    #[doc="Second"]
    #[inline] pub fn second(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if SECOND != 0"]
    #[inline] pub fn test_second(&self) -> bool {
        self.second() != 0
    }

    #[doc="Sets the SECOND field."]
    #[inline] pub fn set_second<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Minute"]
    #[inline] pub fn minute(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3f) as u8) } // [11:6]
    }

    #[doc="Returns true if MINUTE != 0"]
    #[inline] pub fn test_minute(&self) -> bool {
        self.minute() != 0
    }

    #[doc="Sets the MINUTE field."]
    #[inline] pub fn set_minute<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Hour"]
    #[inline] pub fn hour(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1f) as u8) } // [16:12]
    }

    #[doc="Returns true if HOUR != 0"]
    #[inline] pub fn test_hour(&self) -> bool {
        self.hour() != 0
    }

    #[doc="Sets the HOUR field."]
    #[inline] pub fn set_hour<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Day"]
    #[inline] pub fn day(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1f) as u8) } // [21:17]
    }

    #[doc="Returns true if DAY != 0"]
    #[inline] pub fn test_day(&self) -> bool {
        self.day() != 0
    }

    #[doc="Sets the DAY field."]
    #[inline] pub fn set_day<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Month"]
    #[inline] pub fn month(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0xf) as u8) } // [25:22]
    }

    #[doc="Returns true if MONTH != 0"]
    #[inline] pub fn test_month(&self) -> bool {
        self.month() != 0
    }

    #[doc="Sets the MONTH field."]
    #[inline] pub fn set_month<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Year"]
    #[inline] pub fn year(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x3f) as u8) } // [31:26]
    }

    #[doc="Returns true if YEAR != 0"]
    #[inline] pub fn test_year(&self) -> bool {
        self.year() != 0
    }

    #[doc="Sets the YEAR field."]
    #[inline] pub fn set_year<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 26);
        self.0 |= value << 26;
        self
    }

}

impl From<u32> for Clock {
    #[inline]
    fn from(other: u32) -> Self {
         Clock(other)
    }
}

impl ::core::fmt::Display for Clock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Clock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.second() != 0 { try!(write!(f, " second=0x{:x}", self.second()))}
        if self.minute() != 0 { try!(write!(f, " minute=0x{:x}", self.minute()))}
        if self.hour() != 0 { try!(write!(f, " hour=0x{:x}", self.hour()))}
        if self.day() != 0 { try!(write!(f, " day=0x{:x}", self.day()))}
        if self.month() != 0 { try!(write!(f, " month=0x{:x}", self.month()))}
        if self.year() != 0 { try!(write!(f, " year=0x{:x}", self.year()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MODE2 Alarm n Value"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Alarm(pub u32);
impl Alarm {
    #[doc="Second"]
    #[inline] pub fn second(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if SECOND != 0"]
    #[inline] pub fn test_second(&self) -> bool {
        self.second() != 0
    }

    #[doc="Sets the SECOND field."]
    #[inline] pub fn set_second<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Minute"]
    #[inline] pub fn minute(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3f) as u8) } // [11:6]
    }

    #[doc="Returns true if MINUTE != 0"]
    #[inline] pub fn test_minute(&self) -> bool {
        self.minute() != 0
    }

    #[doc="Sets the MINUTE field."]
    #[inline] pub fn set_minute<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Hour"]
    #[inline] pub fn hour(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1f) as u8) } // [16:12]
    }

    #[doc="Returns true if HOUR != 0"]
    #[inline] pub fn test_hour(&self) -> bool {
        self.hour() != 0
    }

    #[doc="Sets the HOUR field."]
    #[inline] pub fn set_hour<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Day"]
    #[inline] pub fn day(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1f) as u8) } // [21:17]
    }

    #[doc="Returns true if DAY != 0"]
    #[inline] pub fn test_day(&self) -> bool {
        self.day() != 0
    }

    #[doc="Sets the DAY field."]
    #[inline] pub fn set_day<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Month"]
    #[inline] pub fn month(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0xf) as u8) } // [25:22]
    }

    #[doc="Returns true if MONTH != 0"]
    #[inline] pub fn test_month(&self) -> bool {
        self.month() != 0
    }

    #[doc="Sets the MONTH field."]
    #[inline] pub fn set_month<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Year"]
    #[inline] pub fn year(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x3f) as u8) } // [31:26]
    }

    #[doc="Returns true if YEAR != 0"]
    #[inline] pub fn test_year(&self) -> bool {
        self.year() != 0
    }

    #[doc="Sets the YEAR field."]
    #[inline] pub fn set_year<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 26);
        self.0 |= value << 26;
        self
    }

}

impl From<u32> for Alarm {
    #[inline]
    fn from(other: u32) -> Self {
         Alarm(other)
    }
}

impl ::core::fmt::Display for Alarm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Alarm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.second() != 0 { try!(write!(f, " second=0x{:x}", self.second()))}
        if self.minute() != 0 { try!(write!(f, " minute=0x{:x}", self.minute()))}
        if self.hour() != 0 { try!(write!(f, " hour=0x{:x}", self.hour()))}
        if self.day() != 0 { try!(write!(f, " day=0x{:x}", self.day()))}
        if self.month() != 0 { try!(write!(f, " month=0x{:x}", self.month()))}
        if self.year() != 0 { try!(write!(f, " year=0x{:x}", self.year()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MODE2 Alarm n Mask"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mask(pub u8);
impl Mask {
    #[doc="Alarm Mask Selection"]
    #[inline] pub fn sel(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if SEL != 0"]
    #[inline] pub fn test_sel(&self) -> bool {
        self.sel() != 0
    }

    #[doc="Sets the SEL field."]
    #[inline] pub fn set_sel<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Mask {
    #[inline]
    fn from(other: u8) -> Self {
         Mask(other)
    }
}

impl ::core::fmt::Display for Mask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sel() != 0 { try!(write!(f, " sel=0x{:x}", self.sel()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="General Purpose"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Gp(pub u32);
impl Gp {
    #[doc="General Purpose"]
    #[inline] pub fn gp(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if GP != 0"]
    #[inline] pub fn test_gp(&self) -> bool {
        self.gp() != 0
    }

    #[doc="Sets the GP field."]
    #[inline] pub fn set_gp<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Gp {
    #[inline]
    fn from(other: u32) -> Self {
         Gp(other)
    }
}

impl ::core::fmt::Display for Gp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Gp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Tamper Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tampctrl(pub u32);
impl Tampctrl {
    #[doc="Tamper Input 0 Action"]
    #[inline] pub fn in0act(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if IN0ACT != 0"]
    #[inline] pub fn test_in0act(&self) -> bool {
        self.in0act() != 0
    }

    #[doc="Sets the IN0ACT field."]
    #[inline] pub fn set_in0act<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Tamper Input 1 Action"]
    #[inline] pub fn in1act(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if IN1ACT != 0"]
    #[inline] pub fn test_in1act(&self) -> bool {
        self.in1act() != 0
    }

    #[doc="Sets the IN1ACT field."]
    #[inline] pub fn set_in1act<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Tamper Input 2 Action"]
    #[inline] pub fn in2act(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if IN2ACT != 0"]
    #[inline] pub fn test_in2act(&self) -> bool {
        self.in2act() != 0
    }

    #[doc="Sets the IN2ACT field."]
    #[inline] pub fn set_in2act<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Tamper Input 3 Action"]
    #[inline] pub fn in3act(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if IN3ACT != 0"]
    #[inline] pub fn test_in3act(&self) -> bool {
        self.in3act() != 0
    }

    #[doc="Sets the IN3ACT field."]
    #[inline] pub fn set_in3act<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Tamper Input 4 Action"]
    #[inline] pub fn in4act(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if IN4ACT != 0"]
    #[inline] pub fn test_in4act(&self) -> bool {
        self.in4act() != 0
    }

    #[doc="Sets the IN4ACT field."]
    #[inline] pub fn set_in4act<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Tamper Level Select 0"]
    #[inline] pub fn tamlvl0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TAMLVL0 != 0"]
    #[inline] pub fn test_tamlvl0(&self) -> bool {
        self.tamlvl0() != 0
    }

    #[doc="Sets the TAMLVL0 field."]
    #[inline] pub fn set_tamlvl0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Tamper Level Select 1"]
    #[inline] pub fn tamlvl1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if TAMLVL1 != 0"]
    #[inline] pub fn test_tamlvl1(&self) -> bool {
        self.tamlvl1() != 0
    }

    #[doc="Sets the TAMLVL1 field."]
    #[inline] pub fn set_tamlvl1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Tamper Level Select 2"]
    #[inline] pub fn tamlvl2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if TAMLVL2 != 0"]
    #[inline] pub fn test_tamlvl2(&self) -> bool {
        self.tamlvl2() != 0
    }

    #[doc="Sets the TAMLVL2 field."]
    #[inline] pub fn set_tamlvl2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Tamper Level Select 3"]
    #[inline] pub fn tamlvl3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if TAMLVL3 != 0"]
    #[inline] pub fn test_tamlvl3(&self) -> bool {
        self.tamlvl3() != 0
    }

    #[doc="Sets the TAMLVL3 field."]
    #[inline] pub fn set_tamlvl3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Tamper Level Select 4"]
    #[inline] pub fn tamlvl4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if TAMLVL4 != 0"]
    #[inline] pub fn test_tamlvl4(&self) -> bool {
        self.tamlvl4() != 0
    }

    #[doc="Sets the TAMLVL4 field."]
    #[inline] pub fn set_tamlvl4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Debouncer Enable 0"]
    #[inline] pub fn debnc0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if DEBNC0 != 0"]
    #[inline] pub fn test_debnc0(&self) -> bool {
        self.debnc0() != 0
    }

    #[doc="Sets the DEBNC0 field."]
    #[inline] pub fn set_debnc0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Debouncer Enable 1"]
    #[inline] pub fn debnc1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if DEBNC1 != 0"]
    #[inline] pub fn test_debnc1(&self) -> bool {
        self.debnc1() != 0
    }

    #[doc="Sets the DEBNC1 field."]
    #[inline] pub fn set_debnc1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Debouncer Enable 2"]
    #[inline] pub fn debnc2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if DEBNC2 != 0"]
    #[inline] pub fn test_debnc2(&self) -> bool {
        self.debnc2() != 0
    }

    #[doc="Sets the DEBNC2 field."]
    #[inline] pub fn set_debnc2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Debouncer Enable 3"]
    #[inline] pub fn debnc3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if DEBNC3 != 0"]
    #[inline] pub fn test_debnc3(&self) -> bool {
        self.debnc3() != 0
    }

    #[doc="Sets the DEBNC3 field."]
    #[inline] pub fn set_debnc3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Debouncer Enable 4"]
    #[inline] pub fn debnc4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if DEBNC4 != 0"]
    #[inline] pub fn test_debnc4(&self) -> bool {
        self.debnc4() != 0
    }

    #[doc="Sets the DEBNC4 field."]
    #[inline] pub fn set_debnc4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

}

impl From<u32> for Tampctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Tampctrl(other)
    }
}

impl ::core::fmt::Display for Tampctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tampctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.in0act() != 0 { try!(write!(f, " in0act=0x{:x}", self.in0act()))}
        if self.in1act() != 0 { try!(write!(f, " in1act=0x{:x}", self.in1act()))}
        if self.in2act() != 0 { try!(write!(f, " in2act=0x{:x}", self.in2act()))}
        if self.in3act() != 0 { try!(write!(f, " in3act=0x{:x}", self.in3act()))}
        if self.in4act() != 0 { try!(write!(f, " in4act=0x{:x}", self.in4act()))}
        if self.tamlvl0() != 0 { try!(write!(f, " tamlvl0"))}
        if self.tamlvl1() != 0 { try!(write!(f, " tamlvl1"))}
        if self.tamlvl2() != 0 { try!(write!(f, " tamlvl2"))}
        if self.tamlvl3() != 0 { try!(write!(f, " tamlvl3"))}
        if self.tamlvl4() != 0 { try!(write!(f, " tamlvl4"))}
        if self.debnc0() != 0 { try!(write!(f, " debnc0"))}
        if self.debnc1() != 0 { try!(write!(f, " debnc1"))}
        if self.debnc2() != 0 { try!(write!(f, " debnc2"))}
        if self.debnc3() != 0 { try!(write!(f, " debnc3"))}
        if self.debnc4() != 0 { try!(write!(f, " debnc4"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MODE2 Timestamp"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Timestamp(pub u32);
impl Timestamp {
    #[doc="Second Timestamp Value"]
    #[inline] pub fn second(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if SECOND != 0"]
    #[inline] pub fn test_second(&self) -> bool {
        self.second() != 0
    }

    #[doc="Sets the SECOND field."]
    #[inline] pub fn set_second<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Minute Timestamp Value"]
    #[inline] pub fn minute(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3f) as u8) } // [11:6]
    }

    #[doc="Returns true if MINUTE != 0"]
    #[inline] pub fn test_minute(&self) -> bool {
        self.minute() != 0
    }

    #[doc="Sets the MINUTE field."]
    #[inline] pub fn set_minute<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Hour Timestamp Value"]
    #[inline] pub fn hour(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1f) as u8) } // [16:12]
    }

    #[doc="Returns true if HOUR != 0"]
    #[inline] pub fn test_hour(&self) -> bool {
        self.hour() != 0
    }

    #[doc="Sets the HOUR field."]
    #[inline] pub fn set_hour<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Day Timestamp Value"]
    #[inline] pub fn day(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1f) as u8) } // [21:17]
    }

    #[doc="Returns true if DAY != 0"]
    #[inline] pub fn test_day(&self) -> bool {
        self.day() != 0
    }

    #[doc="Sets the DAY field."]
    #[inline] pub fn set_day<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Month Timestamp Value"]
    #[inline] pub fn month(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0xf) as u8) } // [25:22]
    }

    #[doc="Returns true if MONTH != 0"]
    #[inline] pub fn test_month(&self) -> bool {
        self.month() != 0
    }

    #[doc="Sets the MONTH field."]
    #[inline] pub fn set_month<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Year Timestamp Value"]
    #[inline] pub fn year(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x3f) as u8) } // [31:26]
    }

    #[doc="Returns true if YEAR != 0"]
    #[inline] pub fn test_year(&self) -> bool {
        self.year() != 0
    }

    #[doc="Sets the YEAR field."]
    #[inline] pub fn set_year<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 26);
        self.0 |= value << 26;
        self
    }

}

impl From<u32> for Timestamp {
    #[inline]
    fn from(other: u32) -> Self {
         Timestamp(other)
    }
}

impl ::core::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Timestamp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.second() != 0 { try!(write!(f, " second=0x{:x}", self.second()))}
        if self.minute() != 0 { try!(write!(f, " minute=0x{:x}", self.minute()))}
        if self.hour() != 0 { try!(write!(f, " hour=0x{:x}", self.hour()))}
        if self.day() != 0 { try!(write!(f, " day=0x{:x}", self.day()))}
        if self.month() != 0 { try!(write!(f, " month=0x{:x}", self.month()))}
        if self.year() != 0 { try!(write!(f, " year=0x{:x}", self.year()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Tamper ID"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tampid(pub u32);
impl Tampid {
    #[doc="Tamper Input 0 Detected"]
    #[inline] pub fn tampid0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TAMPID0 != 0"]
    #[inline] pub fn test_tampid0(&self) -> bool {
        self.tampid0() != 0
    }

    #[doc="Sets the TAMPID0 field."]
    #[inline] pub fn set_tampid0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Tamper Input 1 Detected"]
    #[inline] pub fn tampid1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TAMPID1 != 0"]
    #[inline] pub fn test_tampid1(&self) -> bool {
        self.tampid1() != 0
    }

    #[doc="Sets the TAMPID1 field."]
    #[inline] pub fn set_tampid1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Tamper Input 2 Detected"]
    #[inline] pub fn tampid2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TAMPID2 != 0"]
    #[inline] pub fn test_tampid2(&self) -> bool {
        self.tampid2() != 0
    }

    #[doc="Sets the TAMPID2 field."]
    #[inline] pub fn set_tampid2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Tamper Input 3 Detected"]
    #[inline] pub fn tampid3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TAMPID3 != 0"]
    #[inline] pub fn test_tampid3(&self) -> bool {
        self.tampid3() != 0
    }

    #[doc="Sets the TAMPID3 field."]
    #[inline] pub fn set_tampid3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Tamper Input 4 Detected"]
    #[inline] pub fn tampid4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TAMPID4 != 0"]
    #[inline] pub fn test_tampid4(&self) -> bool {
        self.tampid4() != 0
    }

    #[doc="Sets the TAMPID4 field."]
    #[inline] pub fn set_tampid4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Tamper Event Detected"]
    #[inline] pub fn tampevt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if TAMPEVT != 0"]
    #[inline] pub fn test_tampevt(&self) -> bool {
        self.tampevt() != 0
    }

    #[doc="Sets the TAMPEVT field."]
    #[inline] pub fn set_tampevt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Tampid {
    #[inline]
    fn from(other: u32) -> Self {
         Tampid(other)
    }
}

impl ::core::fmt::Display for Tampid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tampid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tampid0() != 0 { try!(write!(f, " tampid0"))}
        if self.tampid1() != 0 { try!(write!(f, " tampid1"))}
        if self.tampid2() != 0 { try!(write!(f, " tampid2"))}
        if self.tampid3() != 0 { try!(write!(f, " tampid3"))}
        if self.tampid4() != 0 { try!(write!(f, " tampid4"))}
        if self.tampevt() != 0 { try!(write!(f, " tampevt"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Backup"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bkup(pub u32);
impl Bkup {
    #[doc="Backup"]
    #[inline] pub fn bkup(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if BKUP != 0"]
    #[inline] pub fn test_bkup(&self) -> bool {
        self.bkup() != 0
    }

    #[doc="Sets the BKUP field."]
    #[inline] pub fn set_bkup<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Bkup {
    #[inline]
    fn from(other: u32) -> Self {
         Bkup(other)
    }
}

impl ::core::fmt::Display for Bkup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bkup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

}
// End of mode2

