
::bobbin_mcu::periph!( RTC, Rtc, RTC_PERIPH, RtcPeriph, RTC_OWNED, RTC_REF_COUNT, 0x40001400, 0x00, 0x05);


// Gate { name: None, gate_type: Some("EN"), periph: Some("PM"), register: Some("APBAMASK"), field: Some("RTC"), description: None }
impl ::bobbin_mcu::gate::GateEn for Rtc {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::pm::PM.apbamask().rtc() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::pm::PM.with_apbamask(|r| r.set_rtc(value));
        self
    }
}

#[doc="Real-Time Counter"]
#[derive(Clone, Copy, PartialEq, Eq)]
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
    #[doc="Get the DBGCTRL Register."]
    #[inline] pub fn dbgctrl_reg(&self) -> ::bobbin_mcu::register::Register<Dbgctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dbgctrl, 0xb)
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

    #[doc="Get the FREQCORR Register."]
    #[inline] pub fn freqcorr_reg(&self) -> ::bobbin_mcu::register::Register<Freqcorr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Freqcorr, 0xc)
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

    #[doc="Get the COMP Register."]
    #[inline] pub fn comp_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Comp, ::bobbin_bits::R1> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Comp, 0x18, 0x4)
    }

    #[doc="Get the *mut pointer for the COMP register."]
    #[inline] pub fn comp_mut<I: Into<::bobbin_bits::R1>>(&self, index: I) -> *mut Comp { 
        self.comp_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the COMP register."]
    #[inline] pub fn comp_ptr<I: Into<::bobbin_bits::R1>>(&self, index: I) -> *const Comp { 
        self.comp_reg().ptr(index.into())
    }

    #[doc="Read the COMP register."]
    #[inline] pub fn comp<I: Into<::bobbin_bits::R1>>(&self, index: I) -> Comp { 
        self.comp_reg().read(index.into())
    }

    #[doc="Write the COMP register."]
    #[inline] pub fn write_comp<I: Into<::bobbin_bits::R1>>(&self, index: I, value: Comp) -> &Self {
        self.comp_reg().write(index.into(), value);
        self
    }

    #[doc="Set the COMP register."]
    #[inline] pub fn set_comp<I: Into<::bobbin_bits::R1>, F: FnOnce(Comp) -> Comp>(&self, index: I, f: F) -> &Self {
        self.comp_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the COMP register."]
    #[inline] pub fn with_comp<I: Into<::bobbin_bits::R1> + Copy, F: FnOnce(Comp) -> Comp>(&self, index: I, f: F) -> &Self {
        self.comp_reg().with(index.into(), f);
        self
    }

    #[doc="Get the COUNT Register."]
    #[inline] pub fn count_reg(&self) -> ::bobbin_mcu::register::Register<Count> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Count, 0x10)
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

    #[doc="Get the CTRL Register."]
    #[inline] pub fn ctrl_reg(&self) -> ::bobbin_mcu::register::Register<Ctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ctrl, 0x0)
    }

    #[doc="Get the *mut pointer for the CTRL register."]
    #[inline] pub fn ctrl_mut(&self) -> *mut Ctrl { 
        self.ctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the CTRL register."]
    #[inline] pub fn ctrl_ptr(&self) -> *const Ctrl { 
        self.ctrl_reg().ptr()
    }

    #[doc="Read the CTRL register."]
    #[inline] pub fn ctrl(&self) -> Ctrl { 
        self.ctrl_reg().read()
    }

    #[doc="Write the CTRL register."]
    #[inline] pub fn write_ctrl(&self, value: Ctrl) -> &Self { 
        self.ctrl_reg().write(value);
        self
    }

    #[doc="Set the CTRL register."]
    #[inline] pub fn set_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
        self.ctrl_reg().set(f);
        self
    }

    #[doc="Modify the CTRL register."]
    #[inline] pub fn with_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
        self.ctrl_reg().with(f);
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenclr, 0x6)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenset, 0x7)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intflag, 0x8)
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

    #[doc="Get the READREQ Register."]
    #[inline] pub fn readreq_reg(&self) -> ::bobbin_mcu::register::Register<Readreq> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Readreq, 0x2)
    }

    #[doc="Get the *mut pointer for the READREQ register."]
    #[inline] pub fn readreq_mut(&self) -> *mut Readreq { 
        self.readreq_reg().ptr()
    }

    #[doc="Get the *const pointer for the READREQ register."]
    #[inline] pub fn readreq_ptr(&self) -> *const Readreq { 
        self.readreq_reg().ptr()
    }

    #[doc="Read the READREQ register."]
    #[inline] pub fn readreq(&self) -> Readreq { 
        self.readreq_reg().read()
    }

    #[doc="Write the READREQ register."]
    #[inline] pub fn write_readreq(&self, value: Readreq) -> &Self { 
        self.readreq_reg().write(value);
        self
    }

    #[doc="Set the READREQ register."]
    #[inline] pub fn set_readreq<F: FnOnce(Readreq) -> Readreq>(&self, f: F) -> &Self {
        self.readreq_reg().set(f);
        self
    }

    #[doc="Modify the READREQ register."]
    #[inline] pub fn with_readreq<F: FnOnce(Readreq) -> Readreq>(&self, f: F) -> &Self {
        self.readreq_reg().with(f);
        self
    }

    #[doc="Get the STATUS Register."]
    #[inline] pub fn status_reg(&self) -> ::bobbin_mcu::register::Register<Status> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Status, 0xa)
    }

    #[doc="Get the *mut pointer for the STATUS register."]
    #[inline] pub fn status_mut(&self) -> *mut Status { 
        self.status_reg().ptr()
    }

    #[doc="Get the *const pointer for the STATUS register."]
    #[inline] pub fn status_ptr(&self) -> *const Status { 
        self.status_reg().ptr()
    }

    #[doc="Read the STATUS register."]
    #[inline] pub fn status(&self) -> Status { 
        self.status_reg().read()
    }

    #[doc="Write the STATUS register."]
    #[inline] pub fn write_status(&self, value: Status) -> &Self { 
        self.status_reg().write(value);
        self
    }

    #[doc="Set the STATUS register."]
    #[inline] pub fn set_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
        self.status_reg().set(f);
        self
    }

    #[doc="Modify the STATUS register."]
    #[inline] pub fn with_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
        self.status_reg().with(f);
        self
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

#[doc="MODE0 Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrl(pub u16);
impl Ctrl {
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

}

impl From<u16> for Ctrl {
    #[inline]
    fn from(other: u16) -> Self {
         Ctrl(other)
    }
}

impl ::core::fmt::Display for Ctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.swrst() != 0 { try!(write!(f, " swrst"))}
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
        if self.matchclr() != 0 { try!(write!(f, " matchclr"))}
        if self.prescaler() != 0 { try!(write!(f, " prescaler=0x{:x}", self.prescaler()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MODE0 Event Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Evctrl(pub u16);
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
        let value: u16 = value.into();
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
        let value: u16 = value.into();
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
        let value: u16 = value.into();
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
        let value: u16 = value.into();
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
        let value: u16 = value.into();
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
        let value: u16 = value.into();
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
        let value: u16 = value.into();
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
        let value: u16 = value.into();
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
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
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
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Evctrl {
    #[inline]
    fn from(other: u16) -> Self {
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
        if self.ovfeo() != 0 { try!(write!(f, " ovfeo"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MODE0 Interrupt Enable Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
    #[doc="Compare 0 Interrupt Enable"]
    #[inline] pub fn cmp0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CMP0 != 0"]
    #[inline] pub fn test_cmp0(&self) -> bool {
        self.cmp0() != 0
    }

    #[doc="Sets the CMP0 field."]
    #[inline] pub fn set_cmp0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Synchronization Ready Interrupt Enable"]
    #[inline] pub fn syncrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if SYNCRDY != 0"]
    #[inline] pub fn test_syncrdy(&self) -> bool {
        self.syncrdy() != 0
    }

    #[doc="Sets the SYNCRDY field."]
    #[inline] pub fn set_syncrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Overflow Interrupt Enable"]
    #[inline] pub fn ovf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if OVF != 0"]
    #[inline] pub fn test_ovf(&self) -> bool {
        self.ovf() != 0
    }

    #[doc="Sets the OVF field."]
    #[inline] pub fn set_ovf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
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
        if self.cmp0() != 0 { try!(write!(f, " cmp0"))}
        if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
        if self.ovf() != 0 { try!(write!(f, " ovf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MODE0 Interrupt Enable Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
    #[doc="Compare 0 Interrupt Enable"]
    #[inline] pub fn cmp0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CMP0 != 0"]
    #[inline] pub fn test_cmp0(&self) -> bool {
        self.cmp0() != 0
    }

    #[doc="Sets the CMP0 field."]
    #[inline] pub fn set_cmp0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Synchronization Ready Interrupt Enable"]
    #[inline] pub fn syncrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if SYNCRDY != 0"]
    #[inline] pub fn test_syncrdy(&self) -> bool {
        self.syncrdy() != 0
    }

    #[doc="Sets the SYNCRDY field."]
    #[inline] pub fn set_syncrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Overflow Interrupt Enable"]
    #[inline] pub fn ovf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if OVF != 0"]
    #[inline] pub fn test_ovf(&self) -> bool {
        self.ovf() != 0
    }

    #[doc="Sets the OVF field."]
    #[inline] pub fn set_ovf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
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
        if self.cmp0() != 0 { try!(write!(f, " cmp0"))}
        if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
        if self.ovf() != 0 { try!(write!(f, " ovf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MODE0 Interrupt Flag Status and Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
    #[doc="Compare 0"]
    #[inline] pub fn cmp0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CMP0 != 0"]
    #[inline] pub fn test_cmp0(&self) -> bool {
        self.cmp0() != 0
    }

    #[doc="Sets the CMP0 field."]
    #[inline] pub fn set_cmp0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Synchronization Ready"]
    #[inline] pub fn syncrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if SYNCRDY != 0"]
    #[inline] pub fn test_syncrdy(&self) -> bool {
        self.syncrdy() != 0
    }

    #[doc="Sets the SYNCRDY field."]
    #[inline] pub fn set_syncrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Overflow"]
    #[inline] pub fn ovf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if OVF != 0"]
    #[inline] pub fn test_ovf(&self) -> bool {
        self.ovf() != 0
    }

    #[doc="Sets the OVF field."]
    #[inline] pub fn set_ovf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
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
        if self.cmp0() != 0 { try!(write!(f, " cmp0"))}
        if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
        if self.ovf() != 0 { try!(write!(f, " ovf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Read Request"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Readreq(pub u16);
impl Readreq {
    #[doc="Address"]
    #[inline] pub fn addr(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if ADDR != 0"]
    #[inline] pub fn test_addr(&self) -> bool {
        self.addr() != 0
    }

    #[doc="Sets the ADDR field."]
    #[inline] pub fn set_addr<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Read Continuously"]
    #[inline] pub fn rcont(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if RCONT != 0"]
    #[inline] pub fn test_rcont(&self) -> bool {
        self.rcont() != 0
    }

    #[doc="Sets the RCONT field."]
    #[inline] pub fn set_rcont<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Read Request"]
    #[inline] pub fn rreq(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if RREQ != 0"]
    #[inline] pub fn test_rreq(&self) -> bool {
        self.rreq() != 0
    }

    #[doc="Sets the RREQ field."]
    #[inline] pub fn set_rreq<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Readreq {
    #[inline]
    fn from(other: u16) -> Self {
         Readreq(other)
    }
}

impl ::core::fmt::Display for Readreq {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Readreq {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.addr() != 0 { try!(write!(f, " addr=0x{:x}", self.addr()))}
        if self.rcont() != 0 { try!(write!(f, " rcont"))}
        if self.rreq() != 0 { try!(write!(f, " rreq"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u8);
impl Status {
    #[doc="Synchronization Busy"]
    #[inline] pub fn syncbusy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SYNCBUSY != 0"]
    #[inline] pub fn test_syncbusy(&self) -> bool {
        self.syncbusy() != 0
    }

    #[doc="Sets the SYNCBUSY field."]
    #[inline] pub fn set_syncbusy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Status {
    #[inline]
    fn from(other: u8) -> Self {
         Status(other)
    }
}

impl ::core::fmt::Display for Status {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Status {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.syncbusy() != 0 { try!(write!(f, " syncbusy"))}
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
    #[doc="Get the DBGCTRL Register."]
    #[inline] pub fn dbgctrl_reg(&self) -> ::bobbin_mcu::register::Register<Dbgctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dbgctrl, 0xb)
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

    #[doc="Get the FREQCORR Register."]
    #[inline] pub fn freqcorr_reg(&self) -> ::bobbin_mcu::register::Register<Freqcorr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Freqcorr, 0xc)
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

    #[doc="Get the COMP Register."]
    #[inline] pub fn comp_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Comp, ::bobbin_bits::R2> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Comp, 0x18, 0x2)
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

    #[doc="Get the COUNT Register."]
    #[inline] pub fn count_reg(&self) -> ::bobbin_mcu::register::Register<Count> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Count, 0x10)
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

    #[doc="Get the CTRL Register."]
    #[inline] pub fn ctrl_reg(&self) -> ::bobbin_mcu::register::Register<Ctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ctrl, 0x0)
    }

    #[doc="Get the *mut pointer for the CTRL register."]
    #[inline] pub fn ctrl_mut(&self) -> *mut Ctrl { 
        self.ctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the CTRL register."]
    #[inline] pub fn ctrl_ptr(&self) -> *const Ctrl { 
        self.ctrl_reg().ptr()
    }

    #[doc="Read the CTRL register."]
    #[inline] pub fn ctrl(&self) -> Ctrl { 
        self.ctrl_reg().read()
    }

    #[doc="Write the CTRL register."]
    #[inline] pub fn write_ctrl(&self, value: Ctrl) -> &Self { 
        self.ctrl_reg().write(value);
        self
    }

    #[doc="Set the CTRL register."]
    #[inline] pub fn set_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
        self.ctrl_reg().set(f);
        self
    }

    #[doc="Modify the CTRL register."]
    #[inline] pub fn with_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
        self.ctrl_reg().with(f);
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenclr, 0x6)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenset, 0x7)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intflag, 0x8)
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

    #[doc="Get the PER Register."]
    #[inline] pub fn per_reg(&self) -> ::bobbin_mcu::register::Register<Per> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Per, 0x14)
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

    #[doc="Get the READREQ Register."]
    #[inline] pub fn readreq_reg(&self) -> ::bobbin_mcu::register::Register<Readreq> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Readreq, 0x2)
    }

    #[doc="Get the *mut pointer for the READREQ register."]
    #[inline] pub fn readreq_mut(&self) -> *mut Readreq { 
        self.readreq_reg().ptr()
    }

    #[doc="Get the *const pointer for the READREQ register."]
    #[inline] pub fn readreq_ptr(&self) -> *const Readreq { 
        self.readreq_reg().ptr()
    }

    #[doc="Read the READREQ register."]
    #[inline] pub fn readreq(&self) -> Readreq { 
        self.readreq_reg().read()
    }

    #[doc="Write the READREQ register."]
    #[inline] pub fn write_readreq(&self, value: Readreq) -> &Self { 
        self.readreq_reg().write(value);
        self
    }

    #[doc="Set the READREQ register."]
    #[inline] pub fn set_readreq<F: FnOnce(Readreq) -> Readreq>(&self, f: F) -> &Self {
        self.readreq_reg().set(f);
        self
    }

    #[doc="Modify the READREQ register."]
    #[inline] pub fn with_readreq<F: FnOnce(Readreq) -> Readreq>(&self, f: F) -> &Self {
        self.readreq_reg().with(f);
        self
    }

    #[doc="Get the STATUS Register."]
    #[inline] pub fn status_reg(&self) -> ::bobbin_mcu::register::Register<Status> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Status, 0xa)
    }

    #[doc="Get the *mut pointer for the STATUS register."]
    #[inline] pub fn status_mut(&self) -> *mut Status { 
        self.status_reg().ptr()
    }

    #[doc="Get the *const pointer for the STATUS register."]
    #[inline] pub fn status_ptr(&self) -> *const Status { 
        self.status_reg().ptr()
    }

    #[doc="Read the STATUS register."]
    #[inline] pub fn status(&self) -> Status { 
        self.status_reg().read()
    }

    #[doc="Write the STATUS register."]
    #[inline] pub fn write_status(&self, value: Status) -> &Self { 
        self.status_reg().write(value);
        self
    }

    #[doc="Set the STATUS register."]
    #[inline] pub fn set_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
        self.status_reg().set(f);
        self
    }

    #[doc="Modify the STATUS register."]
    #[inline] pub fn with_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
        self.status_reg().with(f);
        self
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

#[doc="MODE1 Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrl(pub u16);
impl Ctrl {
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

}

impl From<u16> for Ctrl {
    #[inline]
    fn from(other: u16) -> Self {
         Ctrl(other)
    }
}

impl ::core::fmt::Display for Ctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.swrst() != 0 { try!(write!(f, " swrst"))}
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
        if self.prescaler() != 0 { try!(write!(f, " prescaler=0x{:x}", self.prescaler()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MODE1 Event Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Evctrl(pub u16);
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
        let value: u16 = value.into();
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
        let value: u16 = value.into();
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
        let value: u16 = value.into();
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
        let value: u16 = value.into();
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
        let value: u16 = value.into();
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
        let value: u16 = value.into();
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
        let value: u16 = value.into();
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
        let value: u16 = value.into();
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
        let value: u16 = value.into();
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
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
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
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Evctrl {
    #[inline]
    fn from(other: u16) -> Self {
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
        if self.ovfeo() != 0 { try!(write!(f, " ovfeo"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MODE1 Interrupt Enable Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
    #[doc="Compare 0 Interrupt Enable"]
    #[inline] pub fn cmp0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CMP0 != 0"]
    #[inline] pub fn test_cmp0(&self) -> bool {
        self.cmp0() != 0
    }

    #[doc="Sets the CMP0 field."]
    #[inline] pub fn set_cmp0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Compare 1 Interrupt Enable"]
    #[inline] pub fn cmp1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CMP1 != 0"]
    #[inline] pub fn test_cmp1(&self) -> bool {
        self.cmp1() != 0
    }

    #[doc="Sets the CMP1 field."]
    #[inline] pub fn set_cmp1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Synchronization Ready Interrupt Enable"]
    #[inline] pub fn syncrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if SYNCRDY != 0"]
    #[inline] pub fn test_syncrdy(&self) -> bool {
        self.syncrdy() != 0
    }

    #[doc="Sets the SYNCRDY field."]
    #[inline] pub fn set_syncrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Overflow Interrupt Enable"]
    #[inline] pub fn ovf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if OVF != 0"]
    #[inline] pub fn test_ovf(&self) -> bool {
        self.ovf() != 0
    }

    #[doc="Sets the OVF field."]
    #[inline] pub fn set_ovf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
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
        if self.cmp0() != 0 { try!(write!(f, " cmp0"))}
        if self.cmp1() != 0 { try!(write!(f, " cmp1"))}
        if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
        if self.ovf() != 0 { try!(write!(f, " ovf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MODE1 Interrupt Enable Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
    #[doc="Compare 0 Interrupt Enable"]
    #[inline] pub fn cmp0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CMP0 != 0"]
    #[inline] pub fn test_cmp0(&self) -> bool {
        self.cmp0() != 0
    }

    #[doc="Sets the CMP0 field."]
    #[inline] pub fn set_cmp0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Compare 1 Interrupt Enable"]
    #[inline] pub fn cmp1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CMP1 != 0"]
    #[inline] pub fn test_cmp1(&self) -> bool {
        self.cmp1() != 0
    }

    #[doc="Sets the CMP1 field."]
    #[inline] pub fn set_cmp1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Synchronization Ready Interrupt Enable"]
    #[inline] pub fn syncrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if SYNCRDY != 0"]
    #[inline] pub fn test_syncrdy(&self) -> bool {
        self.syncrdy() != 0
    }

    #[doc="Sets the SYNCRDY field."]
    #[inline] pub fn set_syncrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Overflow Interrupt Enable"]
    #[inline] pub fn ovf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if OVF != 0"]
    #[inline] pub fn test_ovf(&self) -> bool {
        self.ovf() != 0
    }

    #[doc="Sets the OVF field."]
    #[inline] pub fn set_ovf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
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
        if self.cmp0() != 0 { try!(write!(f, " cmp0"))}
        if self.cmp1() != 0 { try!(write!(f, " cmp1"))}
        if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
        if self.ovf() != 0 { try!(write!(f, " ovf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MODE1 Interrupt Flag Status and Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
    #[doc="Compare 0"]
    #[inline] pub fn cmp0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CMP0 != 0"]
    #[inline] pub fn test_cmp0(&self) -> bool {
        self.cmp0() != 0
    }

    #[doc="Sets the CMP0 field."]
    #[inline] pub fn set_cmp0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Compare 1"]
    #[inline] pub fn cmp1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CMP1 != 0"]
    #[inline] pub fn test_cmp1(&self) -> bool {
        self.cmp1() != 0
    }

    #[doc="Sets the CMP1 field."]
    #[inline] pub fn set_cmp1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Synchronization Ready"]
    #[inline] pub fn syncrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if SYNCRDY != 0"]
    #[inline] pub fn test_syncrdy(&self) -> bool {
        self.syncrdy() != 0
    }

    #[doc="Sets the SYNCRDY field."]
    #[inline] pub fn set_syncrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Overflow"]
    #[inline] pub fn ovf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if OVF != 0"]
    #[inline] pub fn test_ovf(&self) -> bool {
        self.ovf() != 0
    }

    #[doc="Sets the OVF field."]
    #[inline] pub fn set_ovf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
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
        if self.cmp0() != 0 { try!(write!(f, " cmp0"))}
        if self.cmp1() != 0 { try!(write!(f, " cmp1"))}
        if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
        if self.ovf() != 0 { try!(write!(f, " ovf"))}
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

#[doc="Read Request"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Readreq(pub u16);
impl Readreq {
    #[doc="Address"]
    #[inline] pub fn addr(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if ADDR != 0"]
    #[inline] pub fn test_addr(&self) -> bool {
        self.addr() != 0
    }

    #[doc="Sets the ADDR field."]
    #[inline] pub fn set_addr<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Read Continuously"]
    #[inline] pub fn rcont(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if RCONT != 0"]
    #[inline] pub fn test_rcont(&self) -> bool {
        self.rcont() != 0
    }

    #[doc="Sets the RCONT field."]
    #[inline] pub fn set_rcont<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Read Request"]
    #[inline] pub fn rreq(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if RREQ != 0"]
    #[inline] pub fn test_rreq(&self) -> bool {
        self.rreq() != 0
    }

    #[doc="Sets the RREQ field."]
    #[inline] pub fn set_rreq<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Readreq {
    #[inline]
    fn from(other: u16) -> Self {
         Readreq(other)
    }
}

impl ::core::fmt::Display for Readreq {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Readreq {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.addr() != 0 { try!(write!(f, " addr=0x{:x}", self.addr()))}
        if self.rcont() != 0 { try!(write!(f, " rcont"))}
        if self.rreq() != 0 { try!(write!(f, " rreq"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u8);
impl Status {
    #[doc="Synchronization Busy"]
    #[inline] pub fn syncbusy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SYNCBUSY != 0"]
    #[inline] pub fn test_syncbusy(&self) -> bool {
        self.syncbusy() != 0
    }

    #[doc="Sets the SYNCBUSY field."]
    #[inline] pub fn set_syncbusy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Status {
    #[inline]
    fn from(other: u8) -> Self {
         Status(other)
    }
}

impl ::core::fmt::Display for Status {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Status {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.syncbusy() != 0 { try!(write!(f, " syncbusy"))}
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
    #[doc="Get the DBGCTRL Register."]
    #[inline] pub fn dbgctrl_reg(&self) -> ::bobbin_mcu::register::Register<Dbgctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dbgctrl, 0xb)
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

    #[doc="Get the FREQCORR Register."]
    #[inline] pub fn freqcorr_reg(&self) -> ::bobbin_mcu::register::Register<Freqcorr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Freqcorr, 0xc)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Clock, 0x10)
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

    #[doc="Get the CTRL Register."]
    #[inline] pub fn ctrl_reg(&self) -> ::bobbin_mcu::register::Register<Ctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ctrl, 0x0)
    }

    #[doc="Get the *mut pointer for the CTRL register."]
    #[inline] pub fn ctrl_mut(&self) -> *mut Ctrl { 
        self.ctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the CTRL register."]
    #[inline] pub fn ctrl_ptr(&self) -> *const Ctrl { 
        self.ctrl_reg().ptr()
    }

    #[doc="Read the CTRL register."]
    #[inline] pub fn ctrl(&self) -> Ctrl { 
        self.ctrl_reg().read()
    }

    #[doc="Write the CTRL register."]
    #[inline] pub fn write_ctrl(&self, value: Ctrl) -> &Self { 
        self.ctrl_reg().write(value);
        self
    }

    #[doc="Set the CTRL register."]
    #[inline] pub fn set_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
        self.ctrl_reg().set(f);
        self
    }

    #[doc="Modify the CTRL register."]
    #[inline] pub fn with_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
        self.ctrl_reg().with(f);
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenclr, 0x6)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenset, 0x7)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intflag, 0x8)
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

    #[doc="Get the ALARM Register."]
    #[inline] pub fn alarm_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Alarm, ::bobbin_bits::R1> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Alarm, 0x18, 0x8)
    }

    #[doc="Get the *mut pointer for the ALARM register."]
    #[inline] pub fn alarm_mut<I: Into<::bobbin_bits::R1>>(&self, index: I) -> *mut Alarm { 
        self.alarm_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the ALARM register."]
    #[inline] pub fn alarm_ptr<I: Into<::bobbin_bits::R1>>(&self, index: I) -> *const Alarm { 
        self.alarm_reg().ptr(index.into())
    }

    #[doc="Read the ALARM register."]
    #[inline] pub fn alarm<I: Into<::bobbin_bits::R1>>(&self, index: I) -> Alarm { 
        self.alarm_reg().read(index.into())
    }

    #[doc="Write the ALARM register."]
    #[inline] pub fn write_alarm<I: Into<::bobbin_bits::R1>>(&self, index: I, value: Alarm) -> &Self {
        self.alarm_reg().write(index.into(), value);
        self
    }

    #[doc="Set the ALARM register."]
    #[inline] pub fn set_alarm<I: Into<::bobbin_bits::R1>, F: FnOnce(Alarm) -> Alarm>(&self, index: I, f: F) -> &Self {
        self.alarm_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the ALARM register."]
    #[inline] pub fn with_alarm<I: Into<::bobbin_bits::R1> + Copy, F: FnOnce(Alarm) -> Alarm>(&self, index: I, f: F) -> &Self {
        self.alarm_reg().with(index.into(), f);
        self
    }

    #[doc="Get the MASK Register."]
    #[inline] pub fn mask_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Mask, ::bobbin_bits::R1> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Mask, 0x1c, 0x8)
    }

    #[doc="Get the *mut pointer for the MASK register."]
    #[inline] pub fn mask_mut<I: Into<::bobbin_bits::R1>>(&self, index: I) -> *mut Mask { 
        self.mask_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the MASK register."]
    #[inline] pub fn mask_ptr<I: Into<::bobbin_bits::R1>>(&self, index: I) -> *const Mask { 
        self.mask_reg().ptr(index.into())
    }

    #[doc="Read the MASK register."]
    #[inline] pub fn mask<I: Into<::bobbin_bits::R1>>(&self, index: I) -> Mask { 
        self.mask_reg().read(index.into())
    }

    #[doc="Write the MASK register."]
    #[inline] pub fn write_mask<I: Into<::bobbin_bits::R1>>(&self, index: I, value: Mask) -> &Self {
        self.mask_reg().write(index.into(), value);
        self
    }

    #[doc="Set the MASK register."]
    #[inline] pub fn set_mask<I: Into<::bobbin_bits::R1>, F: FnOnce(Mask) -> Mask>(&self, index: I, f: F) -> &Self {
        self.mask_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the MASK register."]
    #[inline] pub fn with_mask<I: Into<::bobbin_bits::R1> + Copy, F: FnOnce(Mask) -> Mask>(&self, index: I, f: F) -> &Self {
        self.mask_reg().with(index.into(), f);
        self
    }

    #[doc="Get the READREQ Register."]
    #[inline] pub fn readreq_reg(&self) -> ::bobbin_mcu::register::Register<Readreq> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Readreq, 0x2)
    }

    #[doc="Get the *mut pointer for the READREQ register."]
    #[inline] pub fn readreq_mut(&self) -> *mut Readreq { 
        self.readreq_reg().ptr()
    }

    #[doc="Get the *const pointer for the READREQ register."]
    #[inline] pub fn readreq_ptr(&self) -> *const Readreq { 
        self.readreq_reg().ptr()
    }

    #[doc="Read the READREQ register."]
    #[inline] pub fn readreq(&self) -> Readreq { 
        self.readreq_reg().read()
    }

    #[doc="Write the READREQ register."]
    #[inline] pub fn write_readreq(&self, value: Readreq) -> &Self { 
        self.readreq_reg().write(value);
        self
    }

    #[doc="Set the READREQ register."]
    #[inline] pub fn set_readreq<F: FnOnce(Readreq) -> Readreq>(&self, f: F) -> &Self {
        self.readreq_reg().set(f);
        self
    }

    #[doc="Modify the READREQ register."]
    #[inline] pub fn with_readreq<F: FnOnce(Readreq) -> Readreq>(&self, f: F) -> &Self {
        self.readreq_reg().with(f);
        self
    }

    #[doc="Get the STATUS Register."]
    #[inline] pub fn status_reg(&self) -> ::bobbin_mcu::register::Register<Status> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Status, 0xa)
    }

    #[doc="Get the *mut pointer for the STATUS register."]
    #[inline] pub fn status_mut(&self) -> *mut Status { 
        self.status_reg().ptr()
    }

    #[doc="Get the *const pointer for the STATUS register."]
    #[inline] pub fn status_ptr(&self) -> *const Status { 
        self.status_reg().ptr()
    }

    #[doc="Read the STATUS register."]
    #[inline] pub fn status(&self) -> Status { 
        self.status_reg().read()
    }

    #[doc="Write the STATUS register."]
    #[inline] pub fn write_status(&self, value: Status) -> &Self { 
        self.status_reg().write(value);
        self
    }

    #[doc="Set the STATUS register."]
    #[inline] pub fn set_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
        self.status_reg().set(f);
        self
    }

    #[doc="Modify the STATUS register."]
    #[inline] pub fn with_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
        self.status_reg().with(f);
        self
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

#[doc="MODE2 Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrl(pub u16);
impl Ctrl {
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

}

impl From<u16> for Ctrl {
    #[inline]
    fn from(other: u16) -> Self {
         Ctrl(other)
    }
}

impl ::core::fmt::Display for Ctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.swrst() != 0 { try!(write!(f, " swrst"))}
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
        if self.clkrep() != 0 { try!(write!(f, " clkrep"))}
        if self.matchclr() != 0 { try!(write!(f, " matchclr"))}
        if self.prescaler() != 0 { try!(write!(f, " prescaler=0x{:x}", self.prescaler()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MODE2 Event Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Evctrl(pub u16);
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
        let value: u16 = value.into();
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
        let value: u16 = value.into();
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
        let value: u16 = value.into();
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
        let value: u16 = value.into();
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
        let value: u16 = value.into();
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
        let value: u16 = value.into();
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
        let value: u16 = value.into();
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
        let value: u16 = value.into();
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
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
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
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Evctrl {
    #[inline]
    fn from(other: u16) -> Self {
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
        if self.ovfeo() != 0 { try!(write!(f, " ovfeo"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MODE2 Interrupt Enable Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
    #[doc="Alarm 0 Interrupt Enable"]
    #[inline] pub fn alarm0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ALARM0 != 0"]
    #[inline] pub fn test_alarm0(&self) -> bool {
        self.alarm0() != 0
    }

    #[doc="Sets the ALARM0 field."]
    #[inline] pub fn set_alarm0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Synchronization Ready Interrupt Enable"]
    #[inline] pub fn syncrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if SYNCRDY != 0"]
    #[inline] pub fn test_syncrdy(&self) -> bool {
        self.syncrdy() != 0
    }

    #[doc="Sets the SYNCRDY field."]
    #[inline] pub fn set_syncrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Overflow Interrupt Enable"]
    #[inline] pub fn ovf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if OVF != 0"]
    #[inline] pub fn test_ovf(&self) -> bool {
        self.ovf() != 0
    }

    #[doc="Sets the OVF field."]
    #[inline] pub fn set_ovf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
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
        if self.alarm0() != 0 { try!(write!(f, " alarm0"))}
        if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
        if self.ovf() != 0 { try!(write!(f, " ovf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MODE2 Interrupt Enable Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
    #[doc="Alarm 0 Interrupt Enable"]
    #[inline] pub fn alarm0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ALARM0 != 0"]
    #[inline] pub fn test_alarm0(&self) -> bool {
        self.alarm0() != 0
    }

    #[doc="Sets the ALARM0 field."]
    #[inline] pub fn set_alarm0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Synchronization Ready Interrupt Enable"]
    #[inline] pub fn syncrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if SYNCRDY != 0"]
    #[inline] pub fn test_syncrdy(&self) -> bool {
        self.syncrdy() != 0
    }

    #[doc="Sets the SYNCRDY field."]
    #[inline] pub fn set_syncrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Overflow Interrupt Enable"]
    #[inline] pub fn ovf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if OVF != 0"]
    #[inline] pub fn test_ovf(&self) -> bool {
        self.ovf() != 0
    }

    #[doc="Sets the OVF field."]
    #[inline] pub fn set_ovf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
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
        if self.alarm0() != 0 { try!(write!(f, " alarm0"))}
        if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
        if self.ovf() != 0 { try!(write!(f, " ovf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MODE2 Interrupt Flag Status and Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
    #[doc="Alarm 0"]
    #[inline] pub fn alarm0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ALARM0 != 0"]
    #[inline] pub fn test_alarm0(&self) -> bool {
        self.alarm0() != 0
    }

    #[doc="Sets the ALARM0 field."]
    #[inline] pub fn set_alarm0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Synchronization Ready"]
    #[inline] pub fn syncrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if SYNCRDY != 0"]
    #[inline] pub fn test_syncrdy(&self) -> bool {
        self.syncrdy() != 0
    }

    #[doc="Sets the SYNCRDY field."]
    #[inline] pub fn set_syncrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Overflow"]
    #[inline] pub fn ovf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if OVF != 0"]
    #[inline] pub fn test_ovf(&self) -> bool {
        self.ovf() != 0
    }

    #[doc="Sets the OVF field."]
    #[inline] pub fn set_ovf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
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
        if self.alarm0() != 0 { try!(write!(f, " alarm0"))}
        if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
        if self.ovf() != 0 { try!(write!(f, " ovf"))}
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

#[doc="Read Request"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Readreq(pub u16);
impl Readreq {
    #[doc="Address"]
    #[inline] pub fn addr(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if ADDR != 0"]
    #[inline] pub fn test_addr(&self) -> bool {
        self.addr() != 0
    }

    #[doc="Sets the ADDR field."]
    #[inline] pub fn set_addr<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Read Continuously"]
    #[inline] pub fn rcont(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if RCONT != 0"]
    #[inline] pub fn test_rcont(&self) -> bool {
        self.rcont() != 0
    }

    #[doc="Sets the RCONT field."]
    #[inline] pub fn set_rcont<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Read Request"]
    #[inline] pub fn rreq(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if RREQ != 0"]
    #[inline] pub fn test_rreq(&self) -> bool {
        self.rreq() != 0
    }

    #[doc="Sets the RREQ field."]
    #[inline] pub fn set_rreq<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Readreq {
    #[inline]
    fn from(other: u16) -> Self {
         Readreq(other)
    }
}

impl ::core::fmt::Display for Readreq {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Readreq {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.addr() != 0 { try!(write!(f, " addr=0x{:x}", self.addr()))}
        if self.rcont() != 0 { try!(write!(f, " rcont"))}
        if self.rreq() != 0 { try!(write!(f, " rreq"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u8);
impl Status {
    #[doc="Synchronization Busy"]
    #[inline] pub fn syncbusy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SYNCBUSY != 0"]
    #[inline] pub fn test_syncbusy(&self) -> bool {
        self.syncbusy() != 0
    }

    #[doc="Sets the SYNCBUSY field."]
    #[inline] pub fn set_syncbusy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Status {
    #[inline]
    fn from(other: u8) -> Self {
         Status(other)
    }
}

impl ::core::fmt::Display for Status {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Status {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.syncbusy() != 0 { try!(write!(f, " syncbusy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

}
// End of mode2

