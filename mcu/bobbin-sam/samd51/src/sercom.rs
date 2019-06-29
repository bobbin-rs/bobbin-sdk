::bobbin_mcu::periph!( SERCOM0, Sercom0, SERCOM0_PERIPH, SercomPeriph, SERCOM0_OWNED, SERCOM0_REF_COUNT, 0x40003000, 0x00, 0x1f);
::bobbin_mcu::periph!( SERCOM1, Sercom1, SERCOM1_PERIPH, SercomPeriph, SERCOM1_OWNED, SERCOM1_REF_COUNT, 0x40003400, 0x01, 0x20);
::bobbin_mcu::periph!( SERCOM2, Sercom2, SERCOM2_PERIPH, SercomPeriph, SERCOM2_OWNED, SERCOM2_REF_COUNT, 0x41012000, 0x02, 0x21);
::bobbin_mcu::periph!( SERCOM3, Sercom3, SERCOM3_PERIPH, SercomPeriph, SERCOM3_OWNED, SERCOM3_REF_COUNT, 0x41014000, 0x03, 0x22);
::bobbin_mcu::periph!( SERCOM4, Sercom4, SERCOM4_PERIPH, SercomPeriph, SERCOM4_OWNED, SERCOM4_REF_COUNT, 0x43000000, 0x04, 0x23);
::bobbin_mcu::periph!( SERCOM5, Sercom5, SERCOM5_PERIPH, SercomPeriph, SERCOM5_OWNED, SERCOM5_REF_COUNT, 0x43000400, 0x05, 0x24);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="SERCOM Peripheral"]
pub struct SercomPeriph(pub usize); 

impl SercomPeriph {
    #[doc="Get I2C Master Mode Peripheral"]
    #[inline] pub fn i2cm(&self) -> i2cm::I2cm {
        i2cm::I2cm(self.0 + 0x0)
    }
    #[doc="Get I2C Slave Mode Peripheral"]
    #[inline] pub fn i2cs(&self) -> i2cs::I2cs {
        i2cs::I2cs(self.0 + 0x0)
    }
    #[doc="Get SPI Mode Peripheral"]
    #[inline] pub fn spi(&self) -> spi::Spi {
        spi::Spi(self.0 + 0x0)
    }
    #[doc="Get USART Mode Peripheral"]
    #[inline] pub fn usart(&self) -> usart::Usart {
        usart::Usart(self.0 + 0x0)
    }
}

#[doc="I2C Master Mode Cluster"]
pub mod i2cm {
    #[allow(unused_imports)] use super::*;
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[doc="I2C Master Mode Peripheral"]
    pub struct I2cm(pub usize);
impl I2cm {
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

    #[doc="Get the CTRLC Register."]
    #[inline] pub fn ctrlc_reg(&self) -> ::bobbin_mcu::register::Register<Ctrlc> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ctrlc, 0x8)
    }

    #[doc="Get the *mut pointer for the CTRLC register."]
    #[inline] pub fn ctrlc_mut(&self) -> *mut Ctrlc { 
        self.ctrlc_reg().ptr()
    }

    #[doc="Get the *const pointer for the CTRLC register."]
    #[inline] pub fn ctrlc_ptr(&self) -> *const Ctrlc { 
        self.ctrlc_reg().ptr()
    }

    #[doc="Read the CTRLC register."]
    #[inline] pub fn ctrlc(&self) -> Ctrlc { 
        self.ctrlc_reg().read()
    }

    #[doc="Write the CTRLC register."]
    #[inline] pub fn write_ctrlc(&self, value: Ctrlc) -> &Self { 
        self.ctrlc_reg().write(value);
        self
    }

    #[doc="Set the CTRLC register."]
    #[inline] pub fn set_ctrlc<F: FnOnce(Ctrlc) -> Ctrlc>(&self, f: F) -> &Self {
        self.ctrlc_reg().set(f);
        self
    }

    #[doc="Modify the CTRLC register."]
    #[inline] pub fn with_ctrlc<F: FnOnce(Ctrlc) -> Ctrlc>(&self, f: F) -> &Self {
        self.ctrlc_reg().with(f);
        self
    }

    #[doc="Get the BAUD Register."]
    #[inline] pub fn baud_reg(&self) -> ::bobbin_mcu::register::Register<Baud> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Baud, 0xc)
    }

    #[doc="Get the *mut pointer for the BAUD register."]
    #[inline] pub fn baud_mut(&self) -> *mut Baud { 
        self.baud_reg().ptr()
    }

    #[doc="Get the *const pointer for the BAUD register."]
    #[inline] pub fn baud_ptr(&self) -> *const Baud { 
        self.baud_reg().ptr()
    }

    #[doc="Read the BAUD register."]
    #[inline] pub fn baud(&self) -> Baud { 
        self.baud_reg().read()
    }

    #[doc="Write the BAUD register."]
    #[inline] pub fn write_baud(&self, value: Baud) -> &Self { 
        self.baud_reg().write(value);
        self
    }

    #[doc="Set the BAUD register."]
    #[inline] pub fn set_baud<F: FnOnce(Baud) -> Baud>(&self, f: F) -> &Self {
        self.baud_reg().set(f);
        self
    }

    #[doc="Modify the BAUD register."]
    #[inline] pub fn with_baud<F: FnOnce(Baud) -> Baud>(&self, f: F) -> &Self {
        self.baud_reg().with(f);
        self
    }

    #[doc="Get the INTENCLR Register."]
    #[inline] pub fn intenclr_reg(&self) -> ::bobbin_mcu::register::Register<Intenclr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenclr, 0x14)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenset, 0x16)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intflag, 0x18)
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

    #[doc="Get the STATUS Register."]
    #[inline] pub fn status_reg(&self) -> ::bobbin_mcu::register::Register<Status> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Status, 0x1a)
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

    #[doc="Get the SYNCBUSY Register."]
    #[inline] pub fn syncbusy_reg(&self) -> ::bobbin_mcu::register::Register<Syncbusy> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Syncbusy, 0x1c)
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

    #[doc="Get the ADDR Register."]
    #[inline] pub fn addr_reg(&self) -> ::bobbin_mcu::register::Register<Addr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Addr, 0x24)
    }

    #[doc="Get the *mut pointer for the ADDR register."]
    #[inline] pub fn addr_mut(&self) -> *mut Addr { 
        self.addr_reg().ptr()
    }

    #[doc="Get the *const pointer for the ADDR register."]
    #[inline] pub fn addr_ptr(&self) -> *const Addr { 
        self.addr_reg().ptr()
    }

    #[doc="Read the ADDR register."]
    #[inline] pub fn addr(&self) -> Addr { 
        self.addr_reg().read()
    }

    #[doc="Write the ADDR register."]
    #[inline] pub fn write_addr(&self, value: Addr) -> &Self { 
        self.addr_reg().write(value);
        self
    }

    #[doc="Set the ADDR register."]
    #[inline] pub fn set_addr<F: FnOnce(Addr) -> Addr>(&self, f: F) -> &Self {
        self.addr_reg().set(f);
        self
    }

    #[doc="Modify the ADDR register."]
    #[inline] pub fn with_addr<F: FnOnce(Addr) -> Addr>(&self, f: F) -> &Self {
        self.addr_reg().with(f);
        self
    }

    #[doc="Get the DATA Register."]
    #[inline] pub fn data_reg(&self) -> ::bobbin_mcu::register::Register<Data> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Data, 0x28)
    }

    #[doc="Get the *mut pointer for the DATA register."]
    #[inline] pub fn data_mut(&self) -> *mut Data { 
        self.data_reg().ptr()
    }

    #[doc="Get the *const pointer for the DATA register."]
    #[inline] pub fn data_ptr(&self) -> *const Data { 
        self.data_reg().ptr()
    }

    #[doc="Read the DATA register."]
    #[inline] pub fn data(&self) -> Data { 
        self.data_reg().read()
    }

    #[doc="Write the DATA register."]
    #[inline] pub fn write_data(&self, value: Data) -> &Self { 
        self.data_reg().write(value);
        self
    }

    #[doc="Set the DATA register."]
    #[inline] pub fn set_data<F: FnOnce(Data) -> Data>(&self, f: F) -> &Self {
        self.data_reg().set(f);
        self
    }

    #[doc="Modify the DATA register."]
    #[inline] pub fn with_data<F: FnOnce(Data) -> Data>(&self, f: F) -> &Self {
        self.data_reg().with(f);
        self
    }

    #[doc="Get the DBGCTRL Register."]
    #[inline] pub fn dbgctrl_reg(&self) -> ::bobbin_mcu::register::Register<Dbgctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dbgctrl, 0x30)
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

}

#[doc="I2CM Control A"]
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

    #[doc="Operating Mode"]
    #[inline] pub fn mode(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x7) as u8) } // [4:2]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode(&self) -> bool {
        self.mode() != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Run in Standby"]
    #[inline] pub fn runstdby(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if RUNSTDBY != 0"]
    #[inline] pub fn test_runstdby(&self) -> bool {
        self.runstdby() != 0
    }

    #[doc="Sets the RUNSTDBY field."]
    #[inline] pub fn set_runstdby<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Pin Usage"]
    #[inline] pub fn pinout(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PINOUT != 0"]
    #[inline] pub fn test_pinout(&self) -> bool {
        self.pinout() != 0
    }

    #[doc="Sets the PINOUT field."]
    #[inline] pub fn set_pinout<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="SDA Hold Time"]
    #[inline] pub fn sdahold(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
    }

    #[doc="Returns true if SDAHOLD != 0"]
    #[inline] pub fn test_sdahold(&self) -> bool {
        self.sdahold() != 0
    }

    #[doc="Sets the SDAHOLD field."]
    #[inline] pub fn set_sdahold<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Master SCL Low Extend Timeout"]
    #[inline] pub fn mexttoen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if MEXTTOEN != 0"]
    #[inline] pub fn test_mexttoen(&self) -> bool {
        self.mexttoen() != 0
    }

    #[doc="Sets the MEXTTOEN field."]
    #[inline] pub fn set_mexttoen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Slave SCL Low Extend Timeout"]
    #[inline] pub fn sexttoen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if SEXTTOEN != 0"]
    #[inline] pub fn test_sexttoen(&self) -> bool {
        self.sexttoen() != 0
    }

    #[doc="Sets the SEXTTOEN field."]
    #[inline] pub fn set_sexttoen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Transfer Speed"]
    #[inline] pub fn speed(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3) as u8) } // [25:24]
    }

    #[doc="Returns true if SPEED != 0"]
    #[inline] pub fn test_speed(&self) -> bool {
        self.speed() != 0
    }

    #[doc="Sets the SPEED field."]
    #[inline] pub fn set_speed<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="SCL Clock Stretch Mode"]
    #[inline] pub fn sclsm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if SCLSM != 0"]
    #[inline] pub fn test_sclsm(&self) -> bool {
        self.sclsm() != 0
    }

    #[doc="Sets the SCLSM field."]
    #[inline] pub fn set_sclsm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Inactive Time-Out"]
    #[inline] pub fn inactout(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x3) as u8) } // [29:28]
    }

    #[doc="Returns true if INACTOUT != 0"]
    #[inline] pub fn test_inactout(&self) -> bool {
        self.inactout() != 0
    }

    #[doc="Sets the INACTOUT field."]
    #[inline] pub fn set_inactout<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="SCL Low Timeout Enable"]
    #[inline] pub fn lowtouten(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if LOWTOUTEN != 0"]
    #[inline] pub fn test_lowtouten(&self) -> bool {
        self.lowtouten() != 0
    }

    #[doc="Sets the LOWTOUTEN field."]
    #[inline] pub fn set_lowtouten<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
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
        if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
        if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
        if self.pinout() != 0 { try!(write!(f, " pinout"))}
        if self.sdahold() != 0 { try!(write!(f, " sdahold=0x{:x}", self.sdahold()))}
        if self.mexttoen() != 0 { try!(write!(f, " mexttoen"))}
        if self.sexttoen() != 0 { try!(write!(f, " sexttoen"))}
        if self.speed() != 0 { try!(write!(f, " speed=0x{:x}", self.speed()))}
        if self.sclsm() != 0 { try!(write!(f, " sclsm"))}
        if self.inactout() != 0 { try!(write!(f, " inactout=0x{:x}", self.inactout()))}
        if self.lowtouten() != 0 { try!(write!(f, " lowtouten"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2CM Control B"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrlb(pub u32);
impl Ctrlb {
    #[doc="Smart Mode Enable"]
    #[inline] pub fn smen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if SMEN != 0"]
    #[inline] pub fn test_smen(&self) -> bool {
        self.smen() != 0
    }

    #[doc="Sets the SMEN field."]
    #[inline] pub fn set_smen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Quick Command Enable"]
    #[inline] pub fn qcen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if QCEN != 0"]
    #[inline] pub fn test_qcen(&self) -> bool {
        self.qcen() != 0
    }

    #[doc="Sets the QCEN field."]
    #[inline] pub fn set_qcen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Command"]
    #[inline] pub fn cmd(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if CMD != 0"]
    #[inline] pub fn test_cmd(&self) -> bool {
        self.cmd() != 0
    }

    #[doc="Sets the CMD field."]
    #[inline] pub fn set_cmd<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Acknowledge Action"]
    #[inline] pub fn ackact(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if ACKACT != 0"]
    #[inline] pub fn test_ackact(&self) -> bool {
        self.ackact() != 0
    }

    #[doc="Sets the ACKACT field."]
    #[inline] pub fn set_ackact<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

}

impl From<u32> for Ctrlb {
    #[inline]
    fn from(other: u32) -> Self {
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
        if self.smen() != 0 { try!(write!(f, " smen"))}
        if self.qcen() != 0 { try!(write!(f, " qcen"))}
        if self.cmd() != 0 { try!(write!(f, " cmd=0x{:x}", self.cmd()))}
        if self.ackact() != 0 { try!(write!(f, " ackact"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2CM Control C"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrlc(pub u32);
impl Ctrlc {
    #[doc="Data 32 Bit"]
    #[inline] pub fn data32b(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if DATA32B != 0"]
    #[inline] pub fn test_data32b(&self) -> bool {
        self.data32b() != 0
    }

    #[doc="Sets the DATA32B field."]
    #[inline] pub fn set_data32b<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Ctrlc {
    #[inline]
    fn from(other: u32) -> Self {
         Ctrlc(other)
    }
}

impl ::core::fmt::Display for Ctrlc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrlc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.data32b() != 0 { try!(write!(f, " data32b"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2CM Baud Rate"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Baud(pub u32);
impl Baud {
    #[doc="Baud Rate Value"]
    #[inline] pub fn baud(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if BAUD != 0"]
    #[inline] pub fn test_baud(&self) -> bool {
        self.baud() != 0
    }

    #[doc="Sets the BAUD field."]
    #[inline] pub fn set_baud<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Baud Rate Value Low"]
    #[inline] pub fn baudlow(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if BAUDLOW != 0"]
    #[inline] pub fn test_baudlow(&self) -> bool {
        self.baudlow() != 0
    }

    #[doc="Sets the BAUDLOW field."]
    #[inline] pub fn set_baudlow<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="High Speed Baud Rate Value"]
    #[inline] pub fn hsbaud(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if HSBAUD != 0"]
    #[inline] pub fn test_hsbaud(&self) -> bool {
        self.hsbaud() != 0
    }

    #[doc="Sets the HSBAUD field."]
    #[inline] pub fn set_hsbaud<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="High Speed Baud Rate Value Low"]
    #[inline] pub fn hsbaudlow(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if HSBAUDLOW != 0"]
    #[inline] pub fn test_hsbaudlow(&self) -> bool {
        self.hsbaudlow() != 0
    }

    #[doc="Sets the HSBAUDLOW field."]
    #[inline] pub fn set_hsbaudlow<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Baud {
    #[inline]
    fn from(other: u32) -> Self {
         Baud(other)
    }
}

impl ::core::fmt::Display for Baud {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Baud {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.baud() != 0 { try!(write!(f, " baud=0x{:x}", self.baud()))}
        if self.baudlow() != 0 { try!(write!(f, " baudlow=0x{:x}", self.baudlow()))}
        if self.hsbaud() != 0 { try!(write!(f, " hsbaud=0x{:x}", self.hsbaud()))}
        if self.hsbaudlow() != 0 { try!(write!(f, " hsbaudlow=0x{:x}", self.hsbaudlow()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2CM Interrupt Enable Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
    #[doc="Master On Bus Interrupt Disable"]
    #[inline] pub fn mb(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if MB != 0"]
    #[inline] pub fn test_mb(&self) -> bool {
        self.mb() != 0
    }

    #[doc="Sets the MB field."]
    #[inline] pub fn set_mb<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Slave On Bus Interrupt Disable"]
    #[inline] pub fn sb(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SB != 0"]
    #[inline] pub fn test_sb(&self) -> bool {
        self.sb() != 0
    }

    #[doc="Sets the SB field."]
    #[inline] pub fn set_sb<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Combined Error Interrupt Disable"]
    #[inline] pub fn error(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ERROR != 0"]
    #[inline] pub fn test_error(&self) -> bool {
        self.error() != 0
    }

    #[doc="Sets the ERROR field."]
    #[inline] pub fn set_error<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.mb() != 0 { try!(write!(f, " mb"))}
        if self.sb() != 0 { try!(write!(f, " sb"))}
        if self.error() != 0 { try!(write!(f, " error"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2CM Interrupt Enable Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
    #[doc="Master On Bus Interrupt Enable"]
    #[inline] pub fn mb(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if MB != 0"]
    #[inline] pub fn test_mb(&self) -> bool {
        self.mb() != 0
    }

    #[doc="Sets the MB field."]
    #[inline] pub fn set_mb<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Slave On Bus Interrupt Enable"]
    #[inline] pub fn sb(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SB != 0"]
    #[inline] pub fn test_sb(&self) -> bool {
        self.sb() != 0
    }

    #[doc="Sets the SB field."]
    #[inline] pub fn set_sb<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Combined Error Interrupt Enable"]
    #[inline] pub fn error(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ERROR != 0"]
    #[inline] pub fn test_error(&self) -> bool {
        self.error() != 0
    }

    #[doc="Sets the ERROR field."]
    #[inline] pub fn set_error<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.mb() != 0 { try!(write!(f, " mb"))}
        if self.sb() != 0 { try!(write!(f, " sb"))}
        if self.error() != 0 { try!(write!(f, " error"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2CM Interrupt Flag Status and Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
    #[doc="Master On Bus Interrupt"]
    #[inline] pub fn mb(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if MB != 0"]
    #[inline] pub fn test_mb(&self) -> bool {
        self.mb() != 0
    }

    #[doc="Sets the MB field."]
    #[inline] pub fn set_mb<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Slave On Bus Interrupt"]
    #[inline] pub fn sb(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SB != 0"]
    #[inline] pub fn test_sb(&self) -> bool {
        self.sb() != 0
    }

    #[doc="Sets the SB field."]
    #[inline] pub fn set_sb<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Combined Error Interrupt"]
    #[inline] pub fn error(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ERROR != 0"]
    #[inline] pub fn test_error(&self) -> bool {
        self.error() != 0
    }

    #[doc="Sets the ERROR field."]
    #[inline] pub fn set_error<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.mb() != 0 { try!(write!(f, " mb"))}
        if self.sb() != 0 { try!(write!(f, " sb"))}
        if self.error() != 0 { try!(write!(f, " error"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2CM Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u16);
impl Status {
    #[doc="Bus Error"]
    #[inline] pub fn buserr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if BUSERR != 0"]
    #[inline] pub fn test_buserr(&self) -> bool {
        self.buserr() != 0
    }

    #[doc="Sets the BUSERR field."]
    #[inline] pub fn set_buserr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Arbitration Lost"]
    #[inline] pub fn arblost(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ARBLOST != 0"]
    #[inline] pub fn test_arblost(&self) -> bool {
        self.arblost() != 0
    }

    #[doc="Sets the ARBLOST field."]
    #[inline] pub fn set_arblost<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Received Not Acknowledge"]
    #[inline] pub fn rxnack(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RXNACK != 0"]
    #[inline] pub fn test_rxnack(&self) -> bool {
        self.rxnack() != 0
    }

    #[doc="Sets the RXNACK field."]
    #[inline] pub fn set_rxnack<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Bus State"]
    #[inline] pub fn busstate(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if BUSSTATE != 0"]
    #[inline] pub fn test_busstate(&self) -> bool {
        self.busstate() != 0
    }

    #[doc="Sets the BUSSTATE field."]
    #[inline] pub fn set_busstate<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="SCL Low Timeout"]
    #[inline] pub fn lowtout(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if LOWTOUT != 0"]
    #[inline] pub fn test_lowtout(&self) -> bool {
        self.lowtout() != 0
    }

    #[doc="Sets the LOWTOUT field."]
    #[inline] pub fn set_lowtout<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Clock Hold"]
    #[inline] pub fn clkhold(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CLKHOLD != 0"]
    #[inline] pub fn test_clkhold(&self) -> bool {
        self.clkhold() != 0
    }

    #[doc="Sets the CLKHOLD field."]
    #[inline] pub fn set_clkhold<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Master SCL Low Extend Timeout"]
    #[inline] pub fn mexttout(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if MEXTTOUT != 0"]
    #[inline] pub fn test_mexttout(&self) -> bool {
        self.mexttout() != 0
    }

    #[doc="Sets the MEXTTOUT field."]
    #[inline] pub fn set_mexttout<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Slave SCL Low Extend Timeout"]
    #[inline] pub fn sexttout(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if SEXTTOUT != 0"]
    #[inline] pub fn test_sexttout(&self) -> bool {
        self.sexttout() != 0
    }

    #[doc="Sets the SEXTTOUT field."]
    #[inline] pub fn set_sexttout<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Length Error"]
    #[inline] pub fn lenerr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if LENERR != 0"]
    #[inline] pub fn test_lenerr(&self) -> bool {
        self.lenerr() != 0
    }

    #[doc="Sets the LENERR field."]
    #[inline] pub fn set_lenerr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

}

impl From<u16> for Status {
    #[inline]
    fn from(other: u16) -> Self {
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
        if self.buserr() != 0 { try!(write!(f, " buserr"))}
        if self.arblost() != 0 { try!(write!(f, " arblost"))}
        if self.rxnack() != 0 { try!(write!(f, " rxnack"))}
        if self.busstate() != 0 { try!(write!(f, " busstate=0x{:x}", self.busstate()))}
        if self.lowtout() != 0 { try!(write!(f, " lowtout"))}
        if self.clkhold() != 0 { try!(write!(f, " clkhold"))}
        if self.mexttout() != 0 { try!(write!(f, " mexttout"))}
        if self.sexttout() != 0 { try!(write!(f, " sexttout"))}
        if self.lenerr() != 0 { try!(write!(f, " lenerr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2CM Synchronization Busy"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Syncbusy(pub u32);
impl Syncbusy {
    #[doc="Software Reset Synchronization Busy"]
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

    #[doc="SERCOM Enable Synchronization Busy"]
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

    #[doc="System Operation Synchronization Busy"]
    #[inline] pub fn sysop(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SYSOP != 0"]
    #[inline] pub fn test_sysop(&self) -> bool {
        self.sysop() != 0
    }

    #[doc="Sets the SYSOP field."]
    #[inline] pub fn set_sysop<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Length Synchronization Busy"]
    #[inline] pub fn length(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if LENGTH != 0"]
    #[inline] pub fn test_length(&self) -> bool {
        self.length() != 0
    }

    #[doc="Sets the LENGTH field."]
    #[inline] pub fn set_length<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
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
        if self.sysop() != 0 { try!(write!(f, " sysop"))}
        if self.length() != 0 { try!(write!(f, " length"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2CM Address"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Addr(pub u32);
impl Addr {
    #[doc="Address Value"]
    #[inline] pub fn addr(&self) -> ::bobbin_bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
    }

    #[doc="Returns true if ADDR != 0"]
    #[inline] pub fn test_addr(&self) -> bool {
        self.addr() != 0
    }

    #[doc="Sets the ADDR field."]
    #[inline] pub fn set_addr<V: Into<::bobbin_bits::U11>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Length Enable"]
    #[inline] pub fn lenen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if LENEN != 0"]
    #[inline] pub fn test_lenen(&self) -> bool {
        self.lenen() != 0
    }

    #[doc="Sets the LENEN field."]
    #[inline] pub fn set_lenen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="High Speed Mode"]
    #[inline] pub fn hs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if HS != 0"]
    #[inline] pub fn test_hs(&self) -> bool {
        self.hs() != 0
    }

    #[doc="Sets the HS field."]
    #[inline] pub fn set_hs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Ten Bit Addressing Enable"]
    #[inline] pub fn tenbiten(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if TENBITEN != 0"]
    #[inline] pub fn test_tenbiten(&self) -> bool {
        self.tenbiten() != 0
    }

    #[doc="Sets the TENBITEN field."]
    #[inline] pub fn set_tenbiten<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Length"]
    #[inline] pub fn len(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if LEN != 0"]
    #[inline] pub fn test_len(&self) -> bool {
        self.len() != 0
    }

    #[doc="Sets the LEN field."]
    #[inline] pub fn set_len<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Addr {
    #[inline]
    fn from(other: u32) -> Self {
         Addr(other)
    }
}

impl ::core::fmt::Display for Addr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Addr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.addr() != 0 { try!(write!(f, " addr=0x{:x}", self.addr()))}
        if self.lenen() != 0 { try!(write!(f, " lenen"))}
        if self.hs() != 0 { try!(write!(f, " hs"))}
        if self.tenbiten() != 0 { try!(write!(f, " tenbiten"))}
        if self.len() != 0 { try!(write!(f, " len=0x{:x}", self.len()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2CM Data"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Data(pub u32);
impl Data {
    #[doc="Data Value"]
    #[inline] pub fn data(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if DATA != 0"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data() != 0
    }

    #[doc="Sets the DATA field."]
    #[inline] pub fn set_data<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Data {
    #[inline]
    fn from(other: u32) -> Self {
         Data(other)
    }
}

impl ::core::fmt::Display for Data {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Data {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2CM Debug Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
    #[doc="Debug Mode"]
    #[inline] pub fn dbgstop(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DBGSTOP != 0"]
    #[inline] pub fn test_dbgstop(&self) -> bool {
        self.dbgstop() != 0
    }

    #[doc="Sets the DBGSTOP field."]
    #[inline] pub fn set_dbgstop<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.dbgstop() != 0 { try!(write!(f, " dbgstop"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

}
// End of i2cm

#[doc="I2C Slave Mode Cluster"]
pub mod i2cs {
    #[allow(unused_imports)] use super::*;
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[doc="I2C Slave Mode Peripheral"]
    pub struct I2cs(pub usize);
impl I2cs {
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

    #[doc="Get the CTRLC Register."]
    #[inline] pub fn ctrlc_reg(&self) -> ::bobbin_mcu::register::Register<Ctrlc> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ctrlc, 0x8)
    }

    #[doc="Get the *mut pointer for the CTRLC register."]
    #[inline] pub fn ctrlc_mut(&self) -> *mut Ctrlc { 
        self.ctrlc_reg().ptr()
    }

    #[doc="Get the *const pointer for the CTRLC register."]
    #[inline] pub fn ctrlc_ptr(&self) -> *const Ctrlc { 
        self.ctrlc_reg().ptr()
    }

    #[doc="Read the CTRLC register."]
    #[inline] pub fn ctrlc(&self) -> Ctrlc { 
        self.ctrlc_reg().read()
    }

    #[doc="Write the CTRLC register."]
    #[inline] pub fn write_ctrlc(&self, value: Ctrlc) -> &Self { 
        self.ctrlc_reg().write(value);
        self
    }

    #[doc="Set the CTRLC register."]
    #[inline] pub fn set_ctrlc<F: FnOnce(Ctrlc) -> Ctrlc>(&self, f: F) -> &Self {
        self.ctrlc_reg().set(f);
        self
    }

    #[doc="Modify the CTRLC register."]
    #[inline] pub fn with_ctrlc<F: FnOnce(Ctrlc) -> Ctrlc>(&self, f: F) -> &Self {
        self.ctrlc_reg().with(f);
        self
    }

    #[doc="Get the INTENCLR Register."]
    #[inline] pub fn intenclr_reg(&self) -> ::bobbin_mcu::register::Register<Intenclr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenclr, 0x14)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenset, 0x16)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intflag, 0x18)
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

    #[doc="Get the STATUS Register."]
    #[inline] pub fn status_reg(&self) -> ::bobbin_mcu::register::Register<Status> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Status, 0x1a)
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

    #[doc="Get the SYNCBUSY Register."]
    #[inline] pub fn syncbusy_reg(&self) -> ::bobbin_mcu::register::Register<Syncbusy> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Syncbusy, 0x1c)
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

    #[doc="Get the LENGTH Register."]
    #[inline] pub fn length_reg(&self) -> ::bobbin_mcu::register::Register<Length> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Length, 0x22)
    }

    #[doc="Get the *mut pointer for the LENGTH register."]
    #[inline] pub fn length_mut(&self) -> *mut Length { 
        self.length_reg().ptr()
    }

    #[doc="Get the *const pointer for the LENGTH register."]
    #[inline] pub fn length_ptr(&self) -> *const Length { 
        self.length_reg().ptr()
    }

    #[doc="Read the LENGTH register."]
    #[inline] pub fn length(&self) -> Length { 
        self.length_reg().read()
    }

    #[doc="Write the LENGTH register."]
    #[inline] pub fn write_length(&self, value: Length) -> &Self { 
        self.length_reg().write(value);
        self
    }

    #[doc="Set the LENGTH register."]
    #[inline] pub fn set_length<F: FnOnce(Length) -> Length>(&self, f: F) -> &Self {
        self.length_reg().set(f);
        self
    }

    #[doc="Modify the LENGTH register."]
    #[inline] pub fn with_length<F: FnOnce(Length) -> Length>(&self, f: F) -> &Self {
        self.length_reg().with(f);
        self
    }

    #[doc="Get the ADDR Register."]
    #[inline] pub fn addr_reg(&self) -> ::bobbin_mcu::register::Register<Addr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Addr, 0x24)
    }

    #[doc="Get the *mut pointer for the ADDR register."]
    #[inline] pub fn addr_mut(&self) -> *mut Addr { 
        self.addr_reg().ptr()
    }

    #[doc="Get the *const pointer for the ADDR register."]
    #[inline] pub fn addr_ptr(&self) -> *const Addr { 
        self.addr_reg().ptr()
    }

    #[doc="Read the ADDR register."]
    #[inline] pub fn addr(&self) -> Addr { 
        self.addr_reg().read()
    }

    #[doc="Write the ADDR register."]
    #[inline] pub fn write_addr(&self, value: Addr) -> &Self { 
        self.addr_reg().write(value);
        self
    }

    #[doc="Set the ADDR register."]
    #[inline] pub fn set_addr<F: FnOnce(Addr) -> Addr>(&self, f: F) -> &Self {
        self.addr_reg().set(f);
        self
    }

    #[doc="Modify the ADDR register."]
    #[inline] pub fn with_addr<F: FnOnce(Addr) -> Addr>(&self, f: F) -> &Self {
        self.addr_reg().with(f);
        self
    }

    #[doc="Get the DATA Register."]
    #[inline] pub fn data_reg(&self) -> ::bobbin_mcu::register::Register<Data> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Data, 0x28)
    }

    #[doc="Get the *mut pointer for the DATA register."]
    #[inline] pub fn data_mut(&self) -> *mut Data { 
        self.data_reg().ptr()
    }

    #[doc="Get the *const pointer for the DATA register."]
    #[inline] pub fn data_ptr(&self) -> *const Data { 
        self.data_reg().ptr()
    }

    #[doc="Read the DATA register."]
    #[inline] pub fn data(&self) -> Data { 
        self.data_reg().read()
    }

    #[doc="Write the DATA register."]
    #[inline] pub fn write_data(&self, value: Data) -> &Self { 
        self.data_reg().write(value);
        self
    }

    #[doc="Set the DATA register."]
    #[inline] pub fn set_data<F: FnOnce(Data) -> Data>(&self, f: F) -> &Self {
        self.data_reg().set(f);
        self
    }

    #[doc="Modify the DATA register."]
    #[inline] pub fn with_data<F: FnOnce(Data) -> Data>(&self, f: F) -> &Self {
        self.data_reg().with(f);
        self
    }

}

#[doc="I2CS Control A"]
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

    #[doc="Operating Mode"]
    #[inline] pub fn mode(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x7) as u8) } // [4:2]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode(&self) -> bool {
        self.mode() != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Run during Standby"]
    #[inline] pub fn runstdby(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if RUNSTDBY != 0"]
    #[inline] pub fn test_runstdby(&self) -> bool {
        self.runstdby() != 0
    }

    #[doc="Sets the RUNSTDBY field."]
    #[inline] pub fn set_runstdby<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Pin Usage"]
    #[inline] pub fn pinout(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PINOUT != 0"]
    #[inline] pub fn test_pinout(&self) -> bool {
        self.pinout() != 0
    }

    #[doc="Sets the PINOUT field."]
    #[inline] pub fn set_pinout<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="SDA Hold Time"]
    #[inline] pub fn sdahold(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
    }

    #[doc="Returns true if SDAHOLD != 0"]
    #[inline] pub fn test_sdahold(&self) -> bool {
        self.sdahold() != 0
    }

    #[doc="Sets the SDAHOLD field."]
    #[inline] pub fn set_sdahold<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Slave SCL Low Extend Timeout"]
    #[inline] pub fn sexttoen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if SEXTTOEN != 0"]
    #[inline] pub fn test_sexttoen(&self) -> bool {
        self.sexttoen() != 0
    }

    #[doc="Sets the SEXTTOEN field."]
    #[inline] pub fn set_sexttoen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Transfer Speed"]
    #[inline] pub fn speed(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3) as u8) } // [25:24]
    }

    #[doc="Returns true if SPEED != 0"]
    #[inline] pub fn test_speed(&self) -> bool {
        self.speed() != 0
    }

    #[doc="Sets the SPEED field."]
    #[inline] pub fn set_speed<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="SCL Clock Stretch Mode"]
    #[inline] pub fn sclsm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if SCLSM != 0"]
    #[inline] pub fn test_sclsm(&self) -> bool {
        self.sclsm() != 0
    }

    #[doc="Sets the SCLSM field."]
    #[inline] pub fn set_sclsm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="SCL Low Timeout Enable"]
    #[inline] pub fn lowtouten(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if LOWTOUTEN != 0"]
    #[inline] pub fn test_lowtouten(&self) -> bool {
        self.lowtouten() != 0
    }

    #[doc="Sets the LOWTOUTEN field."]
    #[inline] pub fn set_lowtouten<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
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
        if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
        if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
        if self.pinout() != 0 { try!(write!(f, " pinout"))}
        if self.sdahold() != 0 { try!(write!(f, " sdahold=0x{:x}", self.sdahold()))}
        if self.sexttoen() != 0 { try!(write!(f, " sexttoen"))}
        if self.speed() != 0 { try!(write!(f, " speed=0x{:x}", self.speed()))}
        if self.sclsm() != 0 { try!(write!(f, " sclsm"))}
        if self.lowtouten() != 0 { try!(write!(f, " lowtouten"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2CS Control B"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrlb(pub u32);
impl Ctrlb {
    #[doc="Smart Mode Enable"]
    #[inline] pub fn smen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if SMEN != 0"]
    #[inline] pub fn test_smen(&self) -> bool {
        self.smen() != 0
    }

    #[doc="Sets the SMEN field."]
    #[inline] pub fn set_smen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="PMBus Group Command"]
    #[inline] pub fn gcmd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if GCMD != 0"]
    #[inline] pub fn test_gcmd(&self) -> bool {
        self.gcmd() != 0
    }

    #[doc="Sets the GCMD field."]
    #[inline] pub fn set_gcmd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Automatic Address Acknowledge"]
    #[inline] pub fn aacken(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if AACKEN != 0"]
    #[inline] pub fn test_aacken(&self) -> bool {
        self.aacken() != 0
    }

    #[doc="Sets the AACKEN field."]
    #[inline] pub fn set_aacken<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Address Mode"]
    #[inline] pub fn amode(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x3) as u8) } // [15:14]
    }

    #[doc="Returns true if AMODE != 0"]
    #[inline] pub fn test_amode(&self) -> bool {
        self.amode() != 0
    }

    #[doc="Sets the AMODE field."]
    #[inline] pub fn set_amode<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Command"]
    #[inline] pub fn cmd(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if CMD != 0"]
    #[inline] pub fn test_cmd(&self) -> bool {
        self.cmd() != 0
    }

    #[doc="Sets the CMD field."]
    #[inline] pub fn set_cmd<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Acknowledge Action"]
    #[inline] pub fn ackact(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if ACKACT != 0"]
    #[inline] pub fn test_ackact(&self) -> bool {
        self.ackact() != 0
    }

    #[doc="Sets the ACKACT field."]
    #[inline] pub fn set_ackact<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

}

impl From<u32> for Ctrlb {
    #[inline]
    fn from(other: u32) -> Self {
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
        if self.smen() != 0 { try!(write!(f, " smen"))}
        if self.gcmd() != 0 { try!(write!(f, " gcmd"))}
        if self.aacken() != 0 { try!(write!(f, " aacken"))}
        if self.amode() != 0 { try!(write!(f, " amode=0x{:x}", self.amode()))}
        if self.cmd() != 0 { try!(write!(f, " cmd=0x{:x}", self.cmd()))}
        if self.ackact() != 0 { try!(write!(f, " ackact"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2CS Control C"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrlc(pub u32);
impl Ctrlc {
    #[doc="SDA Setup Time"]
    #[inline] pub fn sdasetup(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if SDASETUP != 0"]
    #[inline] pub fn test_sdasetup(&self) -> bool {
        self.sdasetup() != 0
    }

    #[doc="Sets the SDASETUP field."]
    #[inline] pub fn set_sdasetup<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Data 32 Bit"]
    #[inline] pub fn data32b(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if DATA32B != 0"]
    #[inline] pub fn test_data32b(&self) -> bool {
        self.data32b() != 0
    }

    #[doc="Sets the DATA32B field."]
    #[inline] pub fn set_data32b<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Ctrlc {
    #[inline]
    fn from(other: u32) -> Self {
         Ctrlc(other)
    }
}

impl ::core::fmt::Display for Ctrlc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrlc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sdasetup() != 0 { try!(write!(f, " sdasetup=0x{:x}", self.sdasetup()))}
        if self.data32b() != 0 { try!(write!(f, " data32b"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2CS Interrupt Enable Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
    #[doc="Stop Received Interrupt Disable"]
    #[inline] pub fn prec(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PREC != 0"]
    #[inline] pub fn test_prec(&self) -> bool {
        self.prec() != 0
    }

    #[doc="Sets the PREC field."]
    #[inline] pub fn set_prec<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Address Match Interrupt Disable"]
    #[inline] pub fn amatch(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if AMATCH != 0"]
    #[inline] pub fn test_amatch(&self) -> bool {
        self.amatch() != 0
    }

    #[doc="Sets the AMATCH field."]
    #[inline] pub fn set_amatch<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Data Interrupt Disable"]
    #[inline] pub fn drdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DRDY != 0"]
    #[inline] pub fn test_drdy(&self) -> bool {
        self.drdy() != 0
    }

    #[doc="Sets the DRDY field."]
    #[inline] pub fn set_drdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Combined Error Interrupt Disable"]
    #[inline] pub fn error(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ERROR != 0"]
    #[inline] pub fn test_error(&self) -> bool {
        self.error() != 0
    }

    #[doc="Sets the ERROR field."]
    #[inline] pub fn set_error<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.prec() != 0 { try!(write!(f, " prec"))}
        if self.amatch() != 0 { try!(write!(f, " amatch"))}
        if self.drdy() != 0 { try!(write!(f, " drdy"))}
        if self.error() != 0 { try!(write!(f, " error"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2CS Interrupt Enable Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
    #[doc="Stop Received Interrupt Enable"]
    #[inline] pub fn prec(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PREC != 0"]
    #[inline] pub fn test_prec(&self) -> bool {
        self.prec() != 0
    }

    #[doc="Sets the PREC field."]
    #[inline] pub fn set_prec<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Address Match Interrupt Enable"]
    #[inline] pub fn amatch(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if AMATCH != 0"]
    #[inline] pub fn test_amatch(&self) -> bool {
        self.amatch() != 0
    }

    #[doc="Sets the AMATCH field."]
    #[inline] pub fn set_amatch<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Data Interrupt Enable"]
    #[inline] pub fn drdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DRDY != 0"]
    #[inline] pub fn test_drdy(&self) -> bool {
        self.drdy() != 0
    }

    #[doc="Sets the DRDY field."]
    #[inline] pub fn set_drdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Combined Error Interrupt Enable"]
    #[inline] pub fn error(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ERROR != 0"]
    #[inline] pub fn test_error(&self) -> bool {
        self.error() != 0
    }

    #[doc="Sets the ERROR field."]
    #[inline] pub fn set_error<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.prec() != 0 { try!(write!(f, " prec"))}
        if self.amatch() != 0 { try!(write!(f, " amatch"))}
        if self.drdy() != 0 { try!(write!(f, " drdy"))}
        if self.error() != 0 { try!(write!(f, " error"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2CS Interrupt Flag Status and Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
    #[doc="Stop Received Interrupt"]
    #[inline] pub fn prec(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PREC != 0"]
    #[inline] pub fn test_prec(&self) -> bool {
        self.prec() != 0
    }

    #[doc="Sets the PREC field."]
    #[inline] pub fn set_prec<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Address Match Interrupt"]
    #[inline] pub fn amatch(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if AMATCH != 0"]
    #[inline] pub fn test_amatch(&self) -> bool {
        self.amatch() != 0
    }

    #[doc="Sets the AMATCH field."]
    #[inline] pub fn set_amatch<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Data Interrupt"]
    #[inline] pub fn drdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DRDY != 0"]
    #[inline] pub fn test_drdy(&self) -> bool {
        self.drdy() != 0
    }

    #[doc="Sets the DRDY field."]
    #[inline] pub fn set_drdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Combined Error Interrupt"]
    #[inline] pub fn error(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ERROR != 0"]
    #[inline] pub fn test_error(&self) -> bool {
        self.error() != 0
    }

    #[doc="Sets the ERROR field."]
    #[inline] pub fn set_error<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.prec() != 0 { try!(write!(f, " prec"))}
        if self.amatch() != 0 { try!(write!(f, " amatch"))}
        if self.drdy() != 0 { try!(write!(f, " drdy"))}
        if self.error() != 0 { try!(write!(f, " error"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2CS Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u16);
impl Status {
    #[doc="Bus Error"]
    #[inline] pub fn buserr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if BUSERR != 0"]
    #[inline] pub fn test_buserr(&self) -> bool {
        self.buserr() != 0
    }

    #[doc="Sets the BUSERR field."]
    #[inline] pub fn set_buserr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transmit Collision"]
    #[inline] pub fn coll(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if COLL != 0"]
    #[inline] pub fn test_coll(&self) -> bool {
        self.coll() != 0
    }

    #[doc="Sets the COLL field."]
    #[inline] pub fn set_coll<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Received Not Acknowledge"]
    #[inline] pub fn rxnack(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RXNACK != 0"]
    #[inline] pub fn test_rxnack(&self) -> bool {
        self.rxnack() != 0
    }

    #[doc="Sets the RXNACK field."]
    #[inline] pub fn set_rxnack<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Read/Write Direction"]
    #[inline] pub fn dir(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DIR != 0"]
    #[inline] pub fn test_dir(&self) -> bool {
        self.dir() != 0
    }

    #[doc="Sets the DIR field."]
    #[inline] pub fn set_dir<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Repeated Start"]
    #[inline] pub fn sr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if SR != 0"]
    #[inline] pub fn test_sr(&self) -> bool {
        self.sr() != 0
    }

    #[doc="Sets the SR field."]
    #[inline] pub fn set_sr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="SCL Low Timeout"]
    #[inline] pub fn lowtout(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if LOWTOUT != 0"]
    #[inline] pub fn test_lowtout(&self) -> bool {
        self.lowtout() != 0
    }

    #[doc="Sets the LOWTOUT field."]
    #[inline] pub fn set_lowtout<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Clock Hold"]
    #[inline] pub fn clkhold(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CLKHOLD != 0"]
    #[inline] pub fn test_clkhold(&self) -> bool {
        self.clkhold() != 0
    }

    #[doc="Sets the CLKHOLD field."]
    #[inline] pub fn set_clkhold<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Slave SCL Low Extend Timeout"]
    #[inline] pub fn sexttout(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if SEXTTOUT != 0"]
    #[inline] pub fn test_sexttout(&self) -> bool {
        self.sexttout() != 0
    }

    #[doc="Sets the SEXTTOUT field."]
    #[inline] pub fn set_sexttout<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="High Speed"]
    #[inline] pub fn hs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if HS != 0"]
    #[inline] pub fn test_hs(&self) -> bool {
        self.hs() != 0
    }

    #[doc="Sets the HS field."]
    #[inline] pub fn set_hs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Transaction Length Error"]
    #[inline] pub fn lenerr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if LENERR != 0"]
    #[inline] pub fn test_lenerr(&self) -> bool {
        self.lenerr() != 0
    }

    #[doc="Sets the LENERR field."]
    #[inline] pub fn set_lenerr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

}

impl From<u16> for Status {
    #[inline]
    fn from(other: u16) -> Self {
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
        if self.buserr() != 0 { try!(write!(f, " buserr"))}
        if self.coll() != 0 { try!(write!(f, " coll"))}
        if self.rxnack() != 0 { try!(write!(f, " rxnack"))}
        if self.dir() != 0 { try!(write!(f, " dir"))}
        if self.sr() != 0 { try!(write!(f, " sr"))}
        if self.lowtout() != 0 { try!(write!(f, " lowtout"))}
        if self.clkhold() != 0 { try!(write!(f, " clkhold"))}
        if self.sexttout() != 0 { try!(write!(f, " sexttout"))}
        if self.hs() != 0 { try!(write!(f, " hs"))}
        if self.lenerr() != 0 { try!(write!(f, " lenerr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2CS Synchronization Busy"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Syncbusy(pub u32);
impl Syncbusy {
    #[doc="Software Reset Synchronization Busy"]
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

    #[doc="SERCOM Enable Synchronization Busy"]
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

    #[doc="Length Synchronization Busy"]
    #[inline] pub fn length(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if LENGTH != 0"]
    #[inline] pub fn test_length(&self) -> bool {
        self.length() != 0
    }

    #[doc="Sets the LENGTH field."]
    #[inline] pub fn set_length<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
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
        if self.length() != 0 { try!(write!(f, " length"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2CS Length"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Length(pub u16);
impl Length {
    #[doc="Data Length"]
    #[inline] pub fn len(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if LEN != 0"]
    #[inline] pub fn test_len(&self) -> bool {
        self.len() != 0
    }

    #[doc="Sets the LEN field."]
    #[inline] pub fn set_len<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Data Length Enable"]
    #[inline] pub fn lenen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if LENEN != 0"]
    #[inline] pub fn test_lenen(&self) -> bool {
        self.lenen() != 0
    }

    #[doc="Sets the LENEN field."]
    #[inline] pub fn set_lenen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u16> for Length {
    #[inline]
    fn from(other: u16) -> Self {
         Length(other)
    }
}

impl ::core::fmt::Display for Length {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Length {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.len() != 0 { try!(write!(f, " len=0x{:x}", self.len()))}
        if self.lenen() != 0 { try!(write!(f, " lenen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2CS Address"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Addr(pub u32);
impl Addr {
    #[doc="General Call Address Enable"]
    #[inline] pub fn gencen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if GENCEN != 0"]
    #[inline] pub fn test_gencen(&self) -> bool {
        self.gencen() != 0
    }

    #[doc="Sets the GENCEN field."]
    #[inline] pub fn set_gencen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Address Value"]
    #[inline] pub fn addr(&self) -> ::bobbin_bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x3ff) as u16) } // [10:1]
    }

    #[doc="Returns true if ADDR != 0"]
    #[inline] pub fn test_addr(&self) -> bool {
        self.addr() != 0
    }

    #[doc="Sets the ADDR field."]
    #[inline] pub fn set_addr<V: Into<::bobbin_bits::U10>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Ten Bit Addressing Enable"]
    #[inline] pub fn tenbiten(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if TENBITEN != 0"]
    #[inline] pub fn test_tenbiten(&self) -> bool {
        self.tenbiten() != 0
    }

    #[doc="Sets the TENBITEN field."]
    #[inline] pub fn set_tenbiten<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Address Mask"]
    #[inline] pub fn addrmask(&self) -> ::bobbin_bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x3ff) as u16) } // [26:17]
    }

    #[doc="Returns true if ADDRMASK != 0"]
    #[inline] pub fn test_addrmask(&self) -> bool {
        self.addrmask() != 0
    }

    #[doc="Sets the ADDRMASK field."]
    #[inline] pub fn set_addrmask<V: Into<::bobbin_bits::U10>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 17);
        self.0 |= value << 17;
        self
    }

}

impl From<u32> for Addr {
    #[inline]
    fn from(other: u32) -> Self {
         Addr(other)
    }
}

impl ::core::fmt::Display for Addr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Addr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gencen() != 0 { try!(write!(f, " gencen"))}
        if self.addr() != 0 { try!(write!(f, " addr=0x{:x}", self.addr()))}
        if self.tenbiten() != 0 { try!(write!(f, " tenbiten"))}
        if self.addrmask() != 0 { try!(write!(f, " addrmask=0x{:x}", self.addrmask()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2CS Data"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Data(pub u32);
impl Data {
    #[doc="Data Value"]
    #[inline] pub fn data(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if DATA != 0"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data() != 0
    }

    #[doc="Sets the DATA field."]
    #[inline] pub fn set_data<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Data {
    #[inline]
    fn from(other: u32) -> Self {
         Data(other)
    }
}

impl ::core::fmt::Display for Data {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Data {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

}
// End of i2cs

#[doc="SPI Mode Cluster"]
pub mod spi {
    #[allow(unused_imports)] use super::*;
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[doc="SPI Mode Peripheral"]
    pub struct Spi(pub usize);
impl Spi {
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

    #[doc="Get the CTRLC Register."]
    #[inline] pub fn ctrlc_reg(&self) -> ::bobbin_mcu::register::Register<Ctrlc> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ctrlc, 0x8)
    }

    #[doc="Get the *mut pointer for the CTRLC register."]
    #[inline] pub fn ctrlc_mut(&self) -> *mut Ctrlc { 
        self.ctrlc_reg().ptr()
    }

    #[doc="Get the *const pointer for the CTRLC register."]
    #[inline] pub fn ctrlc_ptr(&self) -> *const Ctrlc { 
        self.ctrlc_reg().ptr()
    }

    #[doc="Read the CTRLC register."]
    #[inline] pub fn ctrlc(&self) -> Ctrlc { 
        self.ctrlc_reg().read()
    }

    #[doc="Write the CTRLC register."]
    #[inline] pub fn write_ctrlc(&self, value: Ctrlc) -> &Self { 
        self.ctrlc_reg().write(value);
        self
    }

    #[doc="Set the CTRLC register."]
    #[inline] pub fn set_ctrlc<F: FnOnce(Ctrlc) -> Ctrlc>(&self, f: F) -> &Self {
        self.ctrlc_reg().set(f);
        self
    }

    #[doc="Modify the CTRLC register."]
    #[inline] pub fn with_ctrlc<F: FnOnce(Ctrlc) -> Ctrlc>(&self, f: F) -> &Self {
        self.ctrlc_reg().with(f);
        self
    }

    #[doc="Get the BAUD Register."]
    #[inline] pub fn baud_reg(&self) -> ::bobbin_mcu::register::Register<Baud> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Baud, 0xc)
    }

    #[doc="Get the *mut pointer for the BAUD register."]
    #[inline] pub fn baud_mut(&self) -> *mut Baud { 
        self.baud_reg().ptr()
    }

    #[doc="Get the *const pointer for the BAUD register."]
    #[inline] pub fn baud_ptr(&self) -> *const Baud { 
        self.baud_reg().ptr()
    }

    #[doc="Read the BAUD register."]
    #[inline] pub fn baud(&self) -> Baud { 
        self.baud_reg().read()
    }

    #[doc="Write the BAUD register."]
    #[inline] pub fn write_baud(&self, value: Baud) -> &Self { 
        self.baud_reg().write(value);
        self
    }

    #[doc="Set the BAUD register."]
    #[inline] pub fn set_baud<F: FnOnce(Baud) -> Baud>(&self, f: F) -> &Self {
        self.baud_reg().set(f);
        self
    }

    #[doc="Modify the BAUD register."]
    #[inline] pub fn with_baud<F: FnOnce(Baud) -> Baud>(&self, f: F) -> &Self {
        self.baud_reg().with(f);
        self
    }

    #[doc="Get the INTENCLR Register."]
    #[inline] pub fn intenclr_reg(&self) -> ::bobbin_mcu::register::Register<Intenclr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenclr, 0x14)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenset, 0x16)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intflag, 0x18)
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

    #[doc="Get the STATUS Register."]
    #[inline] pub fn status_reg(&self) -> ::bobbin_mcu::register::Register<Status> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Status, 0x1a)
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

    #[doc="Get the SYNCBUSY Register."]
    #[inline] pub fn syncbusy_reg(&self) -> ::bobbin_mcu::register::Register<Syncbusy> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Syncbusy, 0x1c)
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

    #[doc="Get the LENGTH Register."]
    #[inline] pub fn length_reg(&self) -> ::bobbin_mcu::register::Register<Length> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Length, 0x22)
    }

    #[doc="Get the *mut pointer for the LENGTH register."]
    #[inline] pub fn length_mut(&self) -> *mut Length { 
        self.length_reg().ptr()
    }

    #[doc="Get the *const pointer for the LENGTH register."]
    #[inline] pub fn length_ptr(&self) -> *const Length { 
        self.length_reg().ptr()
    }

    #[doc="Read the LENGTH register."]
    #[inline] pub fn length(&self) -> Length { 
        self.length_reg().read()
    }

    #[doc="Write the LENGTH register."]
    #[inline] pub fn write_length(&self, value: Length) -> &Self { 
        self.length_reg().write(value);
        self
    }

    #[doc="Set the LENGTH register."]
    #[inline] pub fn set_length<F: FnOnce(Length) -> Length>(&self, f: F) -> &Self {
        self.length_reg().set(f);
        self
    }

    #[doc="Modify the LENGTH register."]
    #[inline] pub fn with_length<F: FnOnce(Length) -> Length>(&self, f: F) -> &Self {
        self.length_reg().with(f);
        self
    }

    #[doc="Get the ADDR Register."]
    #[inline] pub fn addr_reg(&self) -> ::bobbin_mcu::register::Register<Addr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Addr, 0x24)
    }

    #[doc="Get the *mut pointer for the ADDR register."]
    #[inline] pub fn addr_mut(&self) -> *mut Addr { 
        self.addr_reg().ptr()
    }

    #[doc="Get the *const pointer for the ADDR register."]
    #[inline] pub fn addr_ptr(&self) -> *const Addr { 
        self.addr_reg().ptr()
    }

    #[doc="Read the ADDR register."]
    #[inline] pub fn addr(&self) -> Addr { 
        self.addr_reg().read()
    }

    #[doc="Write the ADDR register."]
    #[inline] pub fn write_addr(&self, value: Addr) -> &Self { 
        self.addr_reg().write(value);
        self
    }

    #[doc="Set the ADDR register."]
    #[inline] pub fn set_addr<F: FnOnce(Addr) -> Addr>(&self, f: F) -> &Self {
        self.addr_reg().set(f);
        self
    }

    #[doc="Modify the ADDR register."]
    #[inline] pub fn with_addr<F: FnOnce(Addr) -> Addr>(&self, f: F) -> &Self {
        self.addr_reg().with(f);
        self
    }

    #[doc="Get the DATA Register."]
    #[inline] pub fn data_reg(&self) -> ::bobbin_mcu::register::Register<Data> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Data, 0x28)
    }

    #[doc="Get the *mut pointer for the DATA register."]
    #[inline] pub fn data_mut(&self) -> *mut Data { 
        self.data_reg().ptr()
    }

    #[doc="Get the *const pointer for the DATA register."]
    #[inline] pub fn data_ptr(&self) -> *const Data { 
        self.data_reg().ptr()
    }

    #[doc="Read the DATA register."]
    #[inline] pub fn data(&self) -> Data { 
        self.data_reg().read()
    }

    #[doc="Write the DATA register."]
    #[inline] pub fn write_data(&self, value: Data) -> &Self { 
        self.data_reg().write(value);
        self
    }

    #[doc="Set the DATA register."]
    #[inline] pub fn set_data<F: FnOnce(Data) -> Data>(&self, f: F) -> &Self {
        self.data_reg().set(f);
        self
    }

    #[doc="Modify the DATA register."]
    #[inline] pub fn with_data<F: FnOnce(Data) -> Data>(&self, f: F) -> &Self {
        self.data_reg().with(f);
        self
    }

    #[doc="Get the DBGCTRL Register."]
    #[inline] pub fn dbgctrl_reg(&self) -> ::bobbin_mcu::register::Register<Dbgctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dbgctrl, 0x30)
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

}

#[doc="SPI Control A"]
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

    #[doc="Operating Mode"]
    #[inline] pub fn mode(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x7) as u8) } // [4:2]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode(&self) -> bool {
        self.mode() != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Run during Standby"]
    #[inline] pub fn runstdby(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if RUNSTDBY != 0"]
    #[inline] pub fn test_runstdby(&self) -> bool {
        self.runstdby() != 0
    }

    #[doc="Sets the RUNSTDBY field."]
    #[inline] pub fn set_runstdby<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Immediate Buffer Overflow Notification"]
    #[inline] pub fn ibon(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if IBON != 0"]
    #[inline] pub fn test_ibon(&self) -> bool {
        self.ibon() != 0
    }

    #[doc="Sets the IBON field."]
    #[inline] pub fn set_ibon<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Data Out Pinout"]
    #[inline] pub fn dopo(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if DOPO != 0"]
    #[inline] pub fn test_dopo(&self) -> bool {
        self.dopo() != 0
    }

    #[doc="Sets the DOPO field."]
    #[inline] pub fn set_dopo<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Data In Pinout"]
    #[inline] pub fn dipo(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
    }

    #[doc="Returns true if DIPO != 0"]
    #[inline] pub fn test_dipo(&self) -> bool {
        self.dipo() != 0
    }

    #[doc="Sets the DIPO field."]
    #[inline] pub fn set_dipo<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Frame Format"]
    #[inline] pub fn form(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if FORM != 0"]
    #[inline] pub fn test_form(&self) -> bool {
        self.form() != 0
    }

    #[doc="Sets the FORM field."]
    #[inline] pub fn set_form<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Clock Phase"]
    #[inline] pub fn cpha(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if CPHA != 0"]
    #[inline] pub fn test_cpha(&self) -> bool {
        self.cpha() != 0
    }

    #[doc="Sets the CPHA field."]
    #[inline] pub fn set_cpha<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Clock Polarity"]
    #[inline] pub fn cpol(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if CPOL != 0"]
    #[inline] pub fn test_cpol(&self) -> bool {
        self.cpol() != 0
    }

    #[doc="Sets the CPOL field."]
    #[inline] pub fn set_cpol<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Data Order"]
    #[inline] pub fn dord(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if DORD != 0"]
    #[inline] pub fn test_dord(&self) -> bool {
        self.dord() != 0
    }

    #[doc="Sets the DORD field."]
    #[inline] pub fn set_dord<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
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
        if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
        if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
        if self.ibon() != 0 { try!(write!(f, " ibon"))}
        if self.dopo() != 0 { try!(write!(f, " dopo=0x{:x}", self.dopo()))}
        if self.dipo() != 0 { try!(write!(f, " dipo=0x{:x}", self.dipo()))}
        if self.form() != 0 { try!(write!(f, " form=0x{:x}", self.form()))}
        if self.cpha() != 0 { try!(write!(f, " cpha"))}
        if self.cpol() != 0 { try!(write!(f, " cpol"))}
        if self.dord() != 0 { try!(write!(f, " dord"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SPI Control B"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrlb(pub u32);
impl Ctrlb {
    #[doc="Character Size"]
    #[inline] pub fn chsize(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if CHSIZE != 0"]
    #[inline] pub fn test_chsize(&self) -> bool {
        self.chsize() != 0
    }

    #[doc="Sets the CHSIZE field."]
    #[inline] pub fn set_chsize<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Data Preload Enable"]
    #[inline] pub fn ploaden(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PLOADEN != 0"]
    #[inline] pub fn test_ploaden(&self) -> bool {
        self.ploaden() != 0
    }

    #[doc="Sets the PLOADEN field."]
    #[inline] pub fn set_ploaden<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Slave Select Low Detect Enable"]
    #[inline] pub fn ssde(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if SSDE != 0"]
    #[inline] pub fn test_ssde(&self) -> bool {
        self.ssde() != 0
    }

    #[doc="Sets the SSDE field."]
    #[inline] pub fn set_ssde<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Master Slave Select Enable"]
    #[inline] pub fn mssen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if MSSEN != 0"]
    #[inline] pub fn test_mssen(&self) -> bool {
        self.mssen() != 0
    }

    #[doc="Sets the MSSEN field."]
    #[inline] pub fn set_mssen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Address Mode"]
    #[inline] pub fn amode(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x3) as u8) } // [15:14]
    }

    #[doc="Returns true if AMODE != 0"]
    #[inline] pub fn test_amode(&self) -> bool {
        self.amode() != 0
    }

    #[doc="Sets the AMODE field."]
    #[inline] pub fn set_amode<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Receiver Enable"]
    #[inline] pub fn rxen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if RXEN != 0"]
    #[inline] pub fn test_rxen(&self) -> bool {
        self.rxen() != 0
    }

    #[doc="Sets the RXEN field."]
    #[inline] pub fn set_rxen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

}

impl From<u32> for Ctrlb {
    #[inline]
    fn from(other: u32) -> Self {
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
        if self.chsize() != 0 { try!(write!(f, " chsize=0x{:x}", self.chsize()))}
        if self.ploaden() != 0 { try!(write!(f, " ploaden"))}
        if self.ssde() != 0 { try!(write!(f, " ssde"))}
        if self.mssen() != 0 { try!(write!(f, " mssen"))}
        if self.amode() != 0 { try!(write!(f, " amode=0x{:x}", self.amode()))}
        if self.rxen() != 0 { try!(write!(f, " rxen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SPI Control C"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrlc(pub u32);
impl Ctrlc {
    #[doc="Inter-Character Spacing"]
    #[inline] pub fn icspace(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if ICSPACE != 0"]
    #[inline] pub fn test_icspace(&self) -> bool {
        self.icspace() != 0
    }

    #[doc="Sets the ICSPACE field."]
    #[inline] pub fn set_icspace<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Data 32 Bit"]
    #[inline] pub fn data32b(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if DATA32B != 0"]
    #[inline] pub fn test_data32b(&self) -> bool {
        self.data32b() != 0
    }

    #[doc="Sets the DATA32B field."]
    #[inline] pub fn set_data32b<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Ctrlc {
    #[inline]
    fn from(other: u32) -> Self {
         Ctrlc(other)
    }
}

impl ::core::fmt::Display for Ctrlc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrlc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.icspace() != 0 { try!(write!(f, " icspace=0x{:x}", self.icspace()))}
        if self.data32b() != 0 { try!(write!(f, " data32b"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SPI Baud Rate"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Baud(pub u8);
impl Baud {
    #[doc="Baud Rate Value"]
    #[inline] pub fn baud(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if BAUD != 0"]
    #[inline] pub fn test_baud(&self) -> bool {
        self.baud() != 0
    }

    #[doc="Sets the BAUD field."]
    #[inline] pub fn set_baud<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Baud {
    #[inline]
    fn from(other: u8) -> Self {
         Baud(other)
    }
}

impl ::core::fmt::Display for Baud {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Baud {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.baud() != 0 { try!(write!(f, " baud=0x{:x}", self.baud()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SPI Interrupt Enable Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
    #[doc="Data Register Empty Interrupt Disable"]
    #[inline] pub fn dre(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DRE != 0"]
    #[inline] pub fn test_dre(&self) -> bool {
        self.dre() != 0
    }

    #[doc="Sets the DRE field."]
    #[inline] pub fn set_dre<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transmit Complete Interrupt Disable"]
    #[inline] pub fn txc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TXC != 0"]
    #[inline] pub fn test_txc(&self) -> bool {
        self.txc() != 0
    }

    #[doc="Sets the TXC field."]
    #[inline] pub fn set_txc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Receive Complete Interrupt Disable"]
    #[inline] pub fn rxc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RXC != 0"]
    #[inline] pub fn test_rxc(&self) -> bool {
        self.rxc() != 0
    }

    #[doc="Sets the RXC field."]
    #[inline] pub fn set_rxc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Slave Select Low Interrupt Disable"]
    #[inline] pub fn ssl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if SSL != 0"]
    #[inline] pub fn test_ssl(&self) -> bool {
        self.ssl() != 0
    }

    #[doc="Sets the SSL field."]
    #[inline] pub fn set_ssl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Combined Error Interrupt Disable"]
    #[inline] pub fn error(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ERROR != 0"]
    #[inline] pub fn test_error(&self) -> bool {
        self.error() != 0
    }

    #[doc="Sets the ERROR field."]
    #[inline] pub fn set_error<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.dre() != 0 { try!(write!(f, " dre"))}
        if self.txc() != 0 { try!(write!(f, " txc"))}
        if self.rxc() != 0 { try!(write!(f, " rxc"))}
        if self.ssl() != 0 { try!(write!(f, " ssl"))}
        if self.error() != 0 { try!(write!(f, " error"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SPI Interrupt Enable Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
    #[doc="Data Register Empty Interrupt Enable"]
    #[inline] pub fn dre(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DRE != 0"]
    #[inline] pub fn test_dre(&self) -> bool {
        self.dre() != 0
    }

    #[doc="Sets the DRE field."]
    #[inline] pub fn set_dre<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transmit Complete Interrupt Enable"]
    #[inline] pub fn txc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TXC != 0"]
    #[inline] pub fn test_txc(&self) -> bool {
        self.txc() != 0
    }

    #[doc="Sets the TXC field."]
    #[inline] pub fn set_txc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Receive Complete Interrupt Enable"]
    #[inline] pub fn rxc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RXC != 0"]
    #[inline] pub fn test_rxc(&self) -> bool {
        self.rxc() != 0
    }

    #[doc="Sets the RXC field."]
    #[inline] pub fn set_rxc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Slave Select Low Interrupt Enable"]
    #[inline] pub fn ssl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if SSL != 0"]
    #[inline] pub fn test_ssl(&self) -> bool {
        self.ssl() != 0
    }

    #[doc="Sets the SSL field."]
    #[inline] pub fn set_ssl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Combined Error Interrupt Enable"]
    #[inline] pub fn error(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ERROR != 0"]
    #[inline] pub fn test_error(&self) -> bool {
        self.error() != 0
    }

    #[doc="Sets the ERROR field."]
    #[inline] pub fn set_error<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.dre() != 0 { try!(write!(f, " dre"))}
        if self.txc() != 0 { try!(write!(f, " txc"))}
        if self.rxc() != 0 { try!(write!(f, " rxc"))}
        if self.ssl() != 0 { try!(write!(f, " ssl"))}
        if self.error() != 0 { try!(write!(f, " error"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SPI Interrupt Flag Status and Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
    #[doc="Data Register Empty Interrupt"]
    #[inline] pub fn dre(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DRE != 0"]
    #[inline] pub fn test_dre(&self) -> bool {
        self.dre() != 0
    }

    #[doc="Sets the DRE field."]
    #[inline] pub fn set_dre<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transmit Complete Interrupt"]
    #[inline] pub fn txc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TXC != 0"]
    #[inline] pub fn test_txc(&self) -> bool {
        self.txc() != 0
    }

    #[doc="Sets the TXC field."]
    #[inline] pub fn set_txc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Receive Complete Interrupt"]
    #[inline] pub fn rxc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RXC != 0"]
    #[inline] pub fn test_rxc(&self) -> bool {
        self.rxc() != 0
    }

    #[doc="Sets the RXC field."]
    #[inline] pub fn set_rxc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Slave Select Low Interrupt Flag"]
    #[inline] pub fn ssl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if SSL != 0"]
    #[inline] pub fn test_ssl(&self) -> bool {
        self.ssl() != 0
    }

    #[doc="Sets the SSL field."]
    #[inline] pub fn set_ssl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Combined Error Interrupt"]
    #[inline] pub fn error(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ERROR != 0"]
    #[inline] pub fn test_error(&self) -> bool {
        self.error() != 0
    }

    #[doc="Sets the ERROR field."]
    #[inline] pub fn set_error<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.dre() != 0 { try!(write!(f, " dre"))}
        if self.txc() != 0 { try!(write!(f, " txc"))}
        if self.rxc() != 0 { try!(write!(f, " rxc"))}
        if self.ssl() != 0 { try!(write!(f, " ssl"))}
        if self.error() != 0 { try!(write!(f, " error"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SPI Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u16);
impl Status {
    #[doc="Buffer Overflow"]
    #[inline] pub fn bufovf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if BUFOVF != 0"]
    #[inline] pub fn test_bufovf(&self) -> bool {
        self.bufovf() != 0
    }

    #[doc="Sets the BUFOVF field."]
    #[inline] pub fn set_bufovf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Transaction Length Error"]
    #[inline] pub fn lenerr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if LENERR != 0"]
    #[inline] pub fn test_lenerr(&self) -> bool {
        self.lenerr() != 0
    }

    #[doc="Sets the LENERR field."]
    #[inline] pub fn set_lenerr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

}

impl From<u16> for Status {
    #[inline]
    fn from(other: u16) -> Self {
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
        if self.bufovf() != 0 { try!(write!(f, " bufovf"))}
        if self.lenerr() != 0 { try!(write!(f, " lenerr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SPI Synchronization Busy"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Syncbusy(pub u32);
impl Syncbusy {
    #[doc="Software Reset Synchronization Busy"]
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

    #[doc="SERCOM Enable Synchronization Busy"]
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

    #[doc="CTRLB Synchronization Busy"]
    #[inline] pub fn ctrlb(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CTRLB != 0"]
    #[inline] pub fn test_ctrlb(&self) -> bool {
        self.ctrlb() != 0
    }

    #[doc="Sets the CTRLB field."]
    #[inline] pub fn set_ctrlb<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="LENGTH Synchronization Busy"]
    #[inline] pub fn length(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if LENGTH != 0"]
    #[inline] pub fn test_length(&self) -> bool {
        self.length() != 0
    }

    #[doc="Sets the LENGTH field."]
    #[inline] pub fn set_length<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
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
        if self.ctrlb() != 0 { try!(write!(f, " ctrlb"))}
        if self.length() != 0 { try!(write!(f, " length"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SPI Length"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Length(pub u16);
impl Length {
    #[doc="Data Length"]
    #[inline] pub fn len(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if LEN != 0"]
    #[inline] pub fn test_len(&self) -> bool {
        self.len() != 0
    }

    #[doc="Sets the LEN field."]
    #[inline] pub fn set_len<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Data Length Enable"]
    #[inline] pub fn lenen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if LENEN != 0"]
    #[inline] pub fn test_lenen(&self) -> bool {
        self.lenen() != 0
    }

    #[doc="Sets the LENEN field."]
    #[inline] pub fn set_lenen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u16> for Length {
    #[inline]
    fn from(other: u16) -> Self {
         Length(other)
    }
}

impl ::core::fmt::Display for Length {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Length {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.len() != 0 { try!(write!(f, " len=0x{:x}", self.len()))}
        if self.lenen() != 0 { try!(write!(f, " lenen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SPI Address"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Addr(pub u32);
impl Addr {
    #[doc="Address Value"]
    #[inline] pub fn addr(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if ADDR != 0"]
    #[inline] pub fn test_addr(&self) -> bool {
        self.addr() != 0
    }

    #[doc="Sets the ADDR field."]
    #[inline] pub fn set_addr<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Address Mask"]
    #[inline] pub fn addrmask(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if ADDRMASK != 0"]
    #[inline] pub fn test_addrmask(&self) -> bool {
        self.addrmask() != 0
    }

    #[doc="Sets the ADDRMASK field."]
    #[inline] pub fn set_addrmask<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Addr {
    #[inline]
    fn from(other: u32) -> Self {
         Addr(other)
    }
}

impl ::core::fmt::Display for Addr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Addr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.addr() != 0 { try!(write!(f, " addr=0x{:x}", self.addr()))}
        if self.addrmask() != 0 { try!(write!(f, " addrmask=0x{:x}", self.addrmask()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SPI Data"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Data(pub u32);
impl Data {
    #[doc="Data Value"]
    #[inline] pub fn data(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if DATA != 0"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data() != 0
    }

    #[doc="Sets the DATA field."]
    #[inline] pub fn set_data<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Data {
    #[inline]
    fn from(other: u32) -> Self {
         Data(other)
    }
}

impl ::core::fmt::Display for Data {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Data {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SPI Debug Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
    #[doc="Debug Mode"]
    #[inline] pub fn dbgstop(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DBGSTOP != 0"]
    #[inline] pub fn test_dbgstop(&self) -> bool {
        self.dbgstop() != 0
    }

    #[doc="Sets the DBGSTOP field."]
    #[inline] pub fn set_dbgstop<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.dbgstop() != 0 { try!(write!(f, " dbgstop"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

}
// End of spi

#[doc="USART Mode Cluster"]
pub mod usart {
    #[allow(unused_imports)] use super::*;
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[doc="USART Mode Peripheral"]
    pub struct Usart(pub usize);
impl Usart {
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

    #[doc="Get the CTRLC Register."]
    #[inline] pub fn ctrlc_reg(&self) -> ::bobbin_mcu::register::Register<Ctrlc> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ctrlc, 0x8)
    }

    #[doc="Get the *mut pointer for the CTRLC register."]
    #[inline] pub fn ctrlc_mut(&self) -> *mut Ctrlc { 
        self.ctrlc_reg().ptr()
    }

    #[doc="Get the *const pointer for the CTRLC register."]
    #[inline] pub fn ctrlc_ptr(&self) -> *const Ctrlc { 
        self.ctrlc_reg().ptr()
    }

    #[doc="Read the CTRLC register."]
    #[inline] pub fn ctrlc(&self) -> Ctrlc { 
        self.ctrlc_reg().read()
    }

    #[doc="Write the CTRLC register."]
    #[inline] pub fn write_ctrlc(&self, value: Ctrlc) -> &Self { 
        self.ctrlc_reg().write(value);
        self
    }

    #[doc="Set the CTRLC register."]
    #[inline] pub fn set_ctrlc<F: FnOnce(Ctrlc) -> Ctrlc>(&self, f: F) -> &Self {
        self.ctrlc_reg().set(f);
        self
    }

    #[doc="Modify the CTRLC register."]
    #[inline] pub fn with_ctrlc<F: FnOnce(Ctrlc) -> Ctrlc>(&self, f: F) -> &Self {
        self.ctrlc_reg().with(f);
        self
    }

    #[doc="Get the BAUD Register."]
    #[inline] pub fn baud_reg(&self) -> ::bobbin_mcu::register::Register<Baud> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Baud, 0xc)
    }

    #[doc="Get the *mut pointer for the BAUD register."]
    #[inline] pub fn baud_mut(&self) -> *mut Baud { 
        self.baud_reg().ptr()
    }

    #[doc="Get the *const pointer for the BAUD register."]
    #[inline] pub fn baud_ptr(&self) -> *const Baud { 
        self.baud_reg().ptr()
    }

    #[doc="Read the BAUD register."]
    #[inline] pub fn baud(&self) -> Baud { 
        self.baud_reg().read()
    }

    #[doc="Write the BAUD register."]
    #[inline] pub fn write_baud(&self, value: Baud) -> &Self { 
        self.baud_reg().write(value);
        self
    }

    #[doc="Set the BAUD register."]
    #[inline] pub fn set_baud<F: FnOnce(Baud) -> Baud>(&self, f: F) -> &Self {
        self.baud_reg().set(f);
        self
    }

    #[doc="Modify the BAUD register."]
    #[inline] pub fn with_baud<F: FnOnce(Baud) -> Baud>(&self, f: F) -> &Self {
        self.baud_reg().with(f);
        self
    }

    #[doc="Get the BAUD_FRAC_MODE Register."]
    #[inline] pub fn baud_frac_mode_reg(&self) -> ::bobbin_mcu::register::Register<BaudFracMode> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut BaudFracMode, 0xc)
    }

    #[doc="Get the *mut pointer for the BAUD_FRAC_MODE register."]
    #[inline] pub fn baud_frac_mode_mut(&self) -> *mut BaudFracMode { 
        self.baud_frac_mode_reg().ptr()
    }

    #[doc="Get the *const pointer for the BAUD_FRAC_MODE register."]
    #[inline] pub fn baud_frac_mode_ptr(&self) -> *const BaudFracMode { 
        self.baud_frac_mode_reg().ptr()
    }

    #[doc="Read the BAUD_FRAC_MODE register."]
    #[inline] pub fn baud_frac_mode(&self) -> BaudFracMode { 
        self.baud_frac_mode_reg().read()
    }

    #[doc="Write the BAUD_FRAC_MODE register."]
    #[inline] pub fn write_baud_frac_mode(&self, value: BaudFracMode) -> &Self { 
        self.baud_frac_mode_reg().write(value);
        self
    }

    #[doc="Set the BAUD_FRAC_MODE register."]
    #[inline] pub fn set_baud_frac_mode<F: FnOnce(BaudFracMode) -> BaudFracMode>(&self, f: F) -> &Self {
        self.baud_frac_mode_reg().set(f);
        self
    }

    #[doc="Modify the BAUD_FRAC_MODE register."]
    #[inline] pub fn with_baud_frac_mode<F: FnOnce(BaudFracMode) -> BaudFracMode>(&self, f: F) -> &Self {
        self.baud_frac_mode_reg().with(f);
        self
    }

    #[doc="Get the BAUD_FRACFP_MODE Register."]
    #[inline] pub fn baud_fracfp_mode_reg(&self) -> ::bobbin_mcu::register::Register<BaudFracfpMode> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut BaudFracfpMode, 0xc)
    }

    #[doc="Get the *mut pointer for the BAUD_FRACFP_MODE register."]
    #[inline] pub fn baud_fracfp_mode_mut(&self) -> *mut BaudFracfpMode { 
        self.baud_fracfp_mode_reg().ptr()
    }

    #[doc="Get the *const pointer for the BAUD_FRACFP_MODE register."]
    #[inline] pub fn baud_fracfp_mode_ptr(&self) -> *const BaudFracfpMode { 
        self.baud_fracfp_mode_reg().ptr()
    }

    #[doc="Read the BAUD_FRACFP_MODE register."]
    #[inline] pub fn baud_fracfp_mode(&self) -> BaudFracfpMode { 
        self.baud_fracfp_mode_reg().read()
    }

    #[doc="Write the BAUD_FRACFP_MODE register."]
    #[inline] pub fn write_baud_fracfp_mode(&self, value: BaudFracfpMode) -> &Self { 
        self.baud_fracfp_mode_reg().write(value);
        self
    }

    #[doc="Set the BAUD_FRACFP_MODE register."]
    #[inline] pub fn set_baud_fracfp_mode<F: FnOnce(BaudFracfpMode) -> BaudFracfpMode>(&self, f: F) -> &Self {
        self.baud_fracfp_mode_reg().set(f);
        self
    }

    #[doc="Modify the BAUD_FRACFP_MODE register."]
    #[inline] pub fn with_baud_fracfp_mode<F: FnOnce(BaudFracfpMode) -> BaudFracfpMode>(&self, f: F) -> &Self {
        self.baud_fracfp_mode_reg().with(f);
        self
    }

    #[doc="Get the BAUD_USARTFP_MODE Register."]
    #[inline] pub fn baud_usartfp_mode_reg(&self) -> ::bobbin_mcu::register::Register<BaudUsartfpMode> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut BaudUsartfpMode, 0xc)
    }

    #[doc="Get the *mut pointer for the BAUD_USARTFP_MODE register."]
    #[inline] pub fn baud_usartfp_mode_mut(&self) -> *mut BaudUsartfpMode { 
        self.baud_usartfp_mode_reg().ptr()
    }

    #[doc="Get the *const pointer for the BAUD_USARTFP_MODE register."]
    #[inline] pub fn baud_usartfp_mode_ptr(&self) -> *const BaudUsartfpMode { 
        self.baud_usartfp_mode_reg().ptr()
    }

    #[doc="Read the BAUD_USARTFP_MODE register."]
    #[inline] pub fn baud_usartfp_mode(&self) -> BaudUsartfpMode { 
        self.baud_usartfp_mode_reg().read()
    }

    #[doc="Write the BAUD_USARTFP_MODE register."]
    #[inline] pub fn write_baud_usartfp_mode(&self, value: BaudUsartfpMode) -> &Self { 
        self.baud_usartfp_mode_reg().write(value);
        self
    }

    #[doc="Set the BAUD_USARTFP_MODE register."]
    #[inline] pub fn set_baud_usartfp_mode<F: FnOnce(BaudUsartfpMode) -> BaudUsartfpMode>(&self, f: F) -> &Self {
        self.baud_usartfp_mode_reg().set(f);
        self
    }

    #[doc="Modify the BAUD_USARTFP_MODE register."]
    #[inline] pub fn with_baud_usartfp_mode<F: FnOnce(BaudUsartfpMode) -> BaudUsartfpMode>(&self, f: F) -> &Self {
        self.baud_usartfp_mode_reg().with(f);
        self
    }

    #[doc="Get the RXPL Register."]
    #[inline] pub fn rxpl_reg(&self) -> ::bobbin_mcu::register::Register<Rxpl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rxpl, 0xe)
    }

    #[doc="Get the *mut pointer for the RXPL register."]
    #[inline] pub fn rxpl_mut(&self) -> *mut Rxpl { 
        self.rxpl_reg().ptr()
    }

    #[doc="Get the *const pointer for the RXPL register."]
    #[inline] pub fn rxpl_ptr(&self) -> *const Rxpl { 
        self.rxpl_reg().ptr()
    }

    #[doc="Read the RXPL register."]
    #[inline] pub fn rxpl(&self) -> Rxpl { 
        self.rxpl_reg().read()
    }

    #[doc="Write the RXPL register."]
    #[inline] pub fn write_rxpl(&self, value: Rxpl) -> &Self { 
        self.rxpl_reg().write(value);
        self
    }

    #[doc="Set the RXPL register."]
    #[inline] pub fn set_rxpl<F: FnOnce(Rxpl) -> Rxpl>(&self, f: F) -> &Self {
        self.rxpl_reg().set(f);
        self
    }

    #[doc="Modify the RXPL register."]
    #[inline] pub fn with_rxpl<F: FnOnce(Rxpl) -> Rxpl>(&self, f: F) -> &Self {
        self.rxpl_reg().with(f);
        self
    }

    #[doc="Get the INTENCLR Register."]
    #[inline] pub fn intenclr_reg(&self) -> ::bobbin_mcu::register::Register<Intenclr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenclr, 0x14)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenset, 0x16)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intflag, 0x18)
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

    #[doc="Get the STATUS Register."]
    #[inline] pub fn status_reg(&self) -> ::bobbin_mcu::register::Register<Status> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Status, 0x1a)
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

    #[doc="Get the SYNCBUSY Register."]
    #[inline] pub fn syncbusy_reg(&self) -> ::bobbin_mcu::register::Register<Syncbusy> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Syncbusy, 0x1c)
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

    #[doc="Get the RXERRCNT Register."]
    #[inline] pub fn rxerrcnt_reg(&self) -> ::bobbin_mcu::register::Register<Rxerrcnt> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rxerrcnt, 0x20)
    }

    #[doc="Get the *mut pointer for the RXERRCNT register."]
    #[inline] pub fn rxerrcnt_mut(&self) -> *mut Rxerrcnt { 
        self.rxerrcnt_reg().ptr()
    }

    #[doc="Get the *const pointer for the RXERRCNT register."]
    #[inline] pub fn rxerrcnt_ptr(&self) -> *const Rxerrcnt { 
        self.rxerrcnt_reg().ptr()
    }

    #[doc="Read the RXERRCNT register."]
    #[inline] pub fn rxerrcnt(&self) -> Rxerrcnt { 
        self.rxerrcnt_reg().read()
    }

    #[doc="Get the LENGTH Register."]
    #[inline] pub fn length_reg(&self) -> ::bobbin_mcu::register::Register<Length> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Length, 0x22)
    }

    #[doc="Get the *mut pointer for the LENGTH register."]
    #[inline] pub fn length_mut(&self) -> *mut Length { 
        self.length_reg().ptr()
    }

    #[doc="Get the *const pointer for the LENGTH register."]
    #[inline] pub fn length_ptr(&self) -> *const Length { 
        self.length_reg().ptr()
    }

    #[doc="Read the LENGTH register."]
    #[inline] pub fn length(&self) -> Length { 
        self.length_reg().read()
    }

    #[doc="Write the LENGTH register."]
    #[inline] pub fn write_length(&self, value: Length) -> &Self { 
        self.length_reg().write(value);
        self
    }

    #[doc="Set the LENGTH register."]
    #[inline] pub fn set_length<F: FnOnce(Length) -> Length>(&self, f: F) -> &Self {
        self.length_reg().set(f);
        self
    }

    #[doc="Modify the LENGTH register."]
    #[inline] pub fn with_length<F: FnOnce(Length) -> Length>(&self, f: F) -> &Self {
        self.length_reg().with(f);
        self
    }

    #[doc="Get the DATA Register."]
    #[inline] pub fn data_reg(&self) -> ::bobbin_mcu::register::Register<Data> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Data, 0x28)
    }

    #[doc="Get the *mut pointer for the DATA register."]
    #[inline] pub fn data_mut(&self) -> *mut Data { 
        self.data_reg().ptr()
    }

    #[doc="Get the *const pointer for the DATA register."]
    #[inline] pub fn data_ptr(&self) -> *const Data { 
        self.data_reg().ptr()
    }

    #[doc="Read the DATA register."]
    #[inline] pub fn data(&self) -> Data { 
        self.data_reg().read()
    }

    #[doc="Write the DATA register."]
    #[inline] pub fn write_data(&self, value: Data) -> &Self { 
        self.data_reg().write(value);
        self
    }

    #[doc="Set the DATA register."]
    #[inline] pub fn set_data<F: FnOnce(Data) -> Data>(&self, f: F) -> &Self {
        self.data_reg().set(f);
        self
    }

    #[doc="Modify the DATA register."]
    #[inline] pub fn with_data<F: FnOnce(Data) -> Data>(&self, f: F) -> &Self {
        self.data_reg().with(f);
        self
    }

    #[doc="Get the DBGCTRL Register."]
    #[inline] pub fn dbgctrl_reg(&self) -> ::bobbin_mcu::register::Register<Dbgctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dbgctrl, 0x30)
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

}

#[doc="USART Control A"]
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

    #[doc="Operating Mode"]
    #[inline] pub fn mode(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x7) as u8) } // [4:2]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode(&self) -> bool {
        self.mode() != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Run during Standby"]
    #[inline] pub fn runstdby(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if RUNSTDBY != 0"]
    #[inline] pub fn test_runstdby(&self) -> bool {
        self.runstdby() != 0
    }

    #[doc="Sets the RUNSTDBY field."]
    #[inline] pub fn set_runstdby<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Immediate Buffer Overflow Notification"]
    #[inline] pub fn ibon(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if IBON != 0"]
    #[inline] pub fn test_ibon(&self) -> bool {
        self.ibon() != 0
    }

    #[doc="Sets the IBON field."]
    #[inline] pub fn set_ibon<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Transmit Data Invert"]
    #[inline] pub fn txinv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if TXINV != 0"]
    #[inline] pub fn test_txinv(&self) -> bool {
        self.txinv() != 0
    }

    #[doc="Sets the TXINV field."]
    #[inline] pub fn set_txinv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Receive Data Invert"]
    #[inline] pub fn rxinv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if RXINV != 0"]
    #[inline] pub fn test_rxinv(&self) -> bool {
        self.rxinv() != 0
    }

    #[doc="Sets the RXINV field."]
    #[inline] pub fn set_rxinv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Sample"]
    #[inline] pub fn sampr(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x7) as u8) } // [15:13]
    }

    #[doc="Returns true if SAMPR != 0"]
    #[inline] pub fn test_sampr(&self) -> bool {
        self.sampr() != 0
    }

    #[doc="Sets the SAMPR field."]
    #[inline] pub fn set_sampr<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Transmit Data Pinout"]
    #[inline] pub fn txpo(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if TXPO != 0"]
    #[inline] pub fn test_txpo(&self) -> bool {
        self.txpo() != 0
    }

    #[doc="Sets the TXPO field."]
    #[inline] pub fn set_txpo<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Receive Data Pinout"]
    #[inline] pub fn rxpo(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
    }

    #[doc="Returns true if RXPO != 0"]
    #[inline] pub fn test_rxpo(&self) -> bool {
        self.rxpo() != 0
    }

    #[doc="Sets the RXPO field."]
    #[inline] pub fn set_rxpo<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Sample Adjustment"]
    #[inline] pub fn sampa(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x3) as u8) } // [23:22]
    }

    #[doc="Returns true if SAMPA != 0"]
    #[inline] pub fn test_sampa(&self) -> bool {
        self.sampa() != 0
    }

    #[doc="Sets the SAMPA field."]
    #[inline] pub fn set_sampa<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Frame Format"]
    #[inline] pub fn form(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if FORM != 0"]
    #[inline] pub fn test_form(&self) -> bool {
        self.form() != 0
    }

    #[doc="Sets the FORM field."]
    #[inline] pub fn set_form<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Communication Mode"]
    #[inline] pub fn cmode(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if CMODE != 0"]
    #[inline] pub fn test_cmode(&self) -> bool {
        self.cmode() != 0
    }

    #[doc="Sets the CMODE field."]
    #[inline] pub fn set_cmode<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Clock Polarity"]
    #[inline] pub fn cpol(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if CPOL != 0"]
    #[inline] pub fn test_cpol(&self) -> bool {
        self.cpol() != 0
    }

    #[doc="Sets the CPOL field."]
    #[inline] pub fn set_cpol<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Data Order"]
    #[inline] pub fn dord(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if DORD != 0"]
    #[inline] pub fn test_dord(&self) -> bool {
        self.dord() != 0
    }

    #[doc="Sets the DORD field."]
    #[inline] pub fn set_dord<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
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
        if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
        if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
        if self.ibon() != 0 { try!(write!(f, " ibon"))}
        if self.txinv() != 0 { try!(write!(f, " txinv"))}
        if self.rxinv() != 0 { try!(write!(f, " rxinv"))}
        if self.sampr() != 0 { try!(write!(f, " sampr=0x{:x}", self.sampr()))}
        if self.txpo() != 0 { try!(write!(f, " txpo=0x{:x}", self.txpo()))}
        if self.rxpo() != 0 { try!(write!(f, " rxpo=0x{:x}", self.rxpo()))}
        if self.sampa() != 0 { try!(write!(f, " sampa=0x{:x}", self.sampa()))}
        if self.form() != 0 { try!(write!(f, " form=0x{:x}", self.form()))}
        if self.cmode() != 0 { try!(write!(f, " cmode"))}
        if self.cpol() != 0 { try!(write!(f, " cpol"))}
        if self.dord() != 0 { try!(write!(f, " dord"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="USART Control B"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrlb(pub u32);
impl Ctrlb {
    #[doc="Character Size"]
    #[inline] pub fn chsize(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if CHSIZE != 0"]
    #[inline] pub fn test_chsize(&self) -> bool {
        self.chsize() != 0
    }

    #[doc="Sets the CHSIZE field."]
    #[inline] pub fn set_chsize<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Stop Bit Mode"]
    #[inline] pub fn sbmode(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if SBMODE != 0"]
    #[inline] pub fn test_sbmode(&self) -> bool {
        self.sbmode() != 0
    }

    #[doc="Sets the SBMODE field."]
    #[inline] pub fn set_sbmode<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Collision Detection Enable"]
    #[inline] pub fn colden(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if COLDEN != 0"]
    #[inline] pub fn test_colden(&self) -> bool {
        self.colden() != 0
    }

    #[doc="Sets the COLDEN field."]
    #[inline] pub fn set_colden<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Start of Frame Detection Enable"]
    #[inline] pub fn sfde(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if SFDE != 0"]
    #[inline] pub fn test_sfde(&self) -> bool {
        self.sfde() != 0
    }

    #[doc="Sets the SFDE field."]
    #[inline] pub fn set_sfde<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Encoding Format"]
    #[inline] pub fn enc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if ENC != 0"]
    #[inline] pub fn test_enc(&self) -> bool {
        self.enc() != 0
    }

    #[doc="Sets the ENC field."]
    #[inline] pub fn set_enc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Parity Mode"]
    #[inline] pub fn pmode(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PMODE != 0"]
    #[inline] pub fn test_pmode(&self) -> bool {
        self.pmode() != 0
    }

    #[doc="Sets the PMODE field."]
    #[inline] pub fn set_pmode<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Transmitter Enable"]
    #[inline] pub fn txen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TXEN != 0"]
    #[inline] pub fn test_txen(&self) -> bool {
        self.txen() != 0
    }

    #[doc="Sets the TXEN field."]
    #[inline] pub fn set_txen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Receiver Enable"]
    #[inline] pub fn rxen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if RXEN != 0"]
    #[inline] pub fn test_rxen(&self) -> bool {
        self.rxen() != 0
    }

    #[doc="Sets the RXEN field."]
    #[inline] pub fn set_rxen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="LIN Command"]
    #[inline] pub fn lincmd(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3) as u8) } // [25:24]
    }

    #[doc="Returns true if LINCMD != 0"]
    #[inline] pub fn test_lincmd(&self) -> bool {
        self.lincmd() != 0
    }

    #[doc="Sets the LINCMD field."]
    #[inline] pub fn set_lincmd<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Ctrlb {
    #[inline]
    fn from(other: u32) -> Self {
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
        if self.chsize() != 0 { try!(write!(f, " chsize=0x{:x}", self.chsize()))}
        if self.sbmode() != 0 { try!(write!(f, " sbmode"))}
        if self.colden() != 0 { try!(write!(f, " colden"))}
        if self.sfde() != 0 { try!(write!(f, " sfde"))}
        if self.enc() != 0 { try!(write!(f, " enc"))}
        if self.pmode() != 0 { try!(write!(f, " pmode"))}
        if self.txen() != 0 { try!(write!(f, " txen"))}
        if self.rxen() != 0 { try!(write!(f, " rxen"))}
        if self.lincmd() != 0 { try!(write!(f, " lincmd=0x{:x}", self.lincmd()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="USART Control C"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrlc(pub u32);
impl Ctrlc {
    #[doc="Guard Time"]
    #[inline] pub fn gtime(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if GTIME != 0"]
    #[inline] pub fn test_gtime(&self) -> bool {
        self.gtime() != 0
    }

    #[doc="Sets the GTIME field."]
    #[inline] pub fn set_gtime<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="LIN Master Break Length"]
    #[inline] pub fn brklen(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if BRKLEN != 0"]
    #[inline] pub fn test_brklen(&self) -> bool {
        self.brklen() != 0
    }

    #[doc="Sets the BRKLEN field."]
    #[inline] pub fn set_brklen<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="LIN Master Header Delay"]
    #[inline] pub fn hdrdly(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3) as u8) } // [11:10]
    }

    #[doc="Returns true if HDRDLY != 0"]
    #[inline] pub fn test_hdrdly(&self) -> bool {
        self.hdrdly() != 0
    }

    #[doc="Sets the HDRDLY field."]
    #[inline] pub fn set_hdrdly<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Inhibit Not Acknowledge"]
    #[inline] pub fn inack(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if INACK != 0"]
    #[inline] pub fn test_inack(&self) -> bool {
        self.inack() != 0
    }

    #[doc="Sets the INACK field."]
    #[inline] pub fn set_inack<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Disable Successive NACK"]
    #[inline] pub fn dsnack(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if DSNACK != 0"]
    #[inline] pub fn test_dsnack(&self) -> bool {
        self.dsnack() != 0
    }

    #[doc="Sets the DSNACK field."]
    #[inline] pub fn set_dsnack<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Maximum Iterations"]
    #[inline] pub fn maxiter(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x7) as u8) } // [22:20]
    }

    #[doc="Returns true if MAXITER != 0"]
    #[inline] pub fn test_maxiter(&self) -> bool {
        self.maxiter() != 0
    }

    #[doc="Sets the MAXITER field."]
    #[inline] pub fn set_maxiter<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Data 32 Bit"]
    #[inline] pub fn data32b(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3) as u8) } // [25:24]
    }

    #[doc="Returns true if DATA32B != 0"]
    #[inline] pub fn test_data32b(&self) -> bool {
        self.data32b() != 0
    }

    #[doc="Sets the DATA32B field."]
    #[inline] pub fn set_data32b<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Ctrlc {
    #[inline]
    fn from(other: u32) -> Self {
         Ctrlc(other)
    }
}

impl ::core::fmt::Display for Ctrlc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrlc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gtime() != 0 { try!(write!(f, " gtime=0x{:x}", self.gtime()))}
        if self.brklen() != 0 { try!(write!(f, " brklen=0x{:x}", self.brklen()))}
        if self.hdrdly() != 0 { try!(write!(f, " hdrdly=0x{:x}", self.hdrdly()))}
        if self.inack() != 0 { try!(write!(f, " inack"))}
        if self.dsnack() != 0 { try!(write!(f, " dsnack"))}
        if self.maxiter() != 0 { try!(write!(f, " maxiter=0x{:x}", self.maxiter()))}
        if self.data32b() != 0 { try!(write!(f, " data32b=0x{:x}", self.data32b()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="USART Baud Rate"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Baud(pub u16);
impl Baud {
    #[doc="Baud Rate Value"]
    #[inline] pub fn baud(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if BAUD != 0"]
    #[inline] pub fn test_baud(&self) -> bool {
        self.baud() != 0
    }

    #[doc="Sets the BAUD field."]
    #[inline] pub fn set_baud<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Baud {
    #[inline]
    fn from(other: u16) -> Self {
         Baud(other)
    }
}

impl ::core::fmt::Display for Baud {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Baud {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.baud() != 0 { try!(write!(f, " baud=0x{:x}", self.baud()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="USART Baud Rate"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct BaudFracMode(pub u16);
impl BaudFracMode {
    #[doc="Baud Rate Value"]
    #[inline] pub fn baud(&self) -> ::bobbin_bits::U13 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1fff) as u16) } // [12:0]
    }

    #[doc="Returns true if BAUD != 0"]
    #[inline] pub fn test_baud(&self) -> bool {
        self.baud() != 0
    }

    #[doc="Sets the BAUD field."]
    #[inline] pub fn set_baud<V: Into<::bobbin_bits::U13>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U13 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1fff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Fractional Part"]
    #[inline] pub fn fp(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x7) as u8) } // [15:13]
    }

    #[doc="Returns true if FP != 0"]
    #[inline] pub fn test_fp(&self) -> bool {
        self.fp() != 0
    }

    #[doc="Sets the FP field."]
    #[inline] pub fn set_fp<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7 << 13);
        self.0 |= value << 13;
        self
    }

}

impl From<u16> for BaudFracMode {
    #[inline]
    fn from(other: u16) -> Self {
         BaudFracMode(other)
    }
}

impl ::core::fmt::Display for BaudFracMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for BaudFracMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.baud() != 0 { try!(write!(f, " baud=0x{:x}", self.baud()))}
        if self.fp() != 0 { try!(write!(f, " fp=0x{:x}", self.fp()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="USART Baud Rate"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct BaudFracfpMode(pub u16);
impl BaudFracfpMode {
    #[doc="Baud Rate Value"]
    #[inline] pub fn baud(&self) -> ::bobbin_bits::U13 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1fff) as u16) } // [12:0]
    }

    #[doc="Returns true if BAUD != 0"]
    #[inline] pub fn test_baud(&self) -> bool {
        self.baud() != 0
    }

    #[doc="Sets the BAUD field."]
    #[inline] pub fn set_baud<V: Into<::bobbin_bits::U13>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U13 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1fff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Fractional Part"]
    #[inline] pub fn fp(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x7) as u8) } // [15:13]
    }

    #[doc="Returns true if FP != 0"]
    #[inline] pub fn test_fp(&self) -> bool {
        self.fp() != 0
    }

    #[doc="Sets the FP field."]
    #[inline] pub fn set_fp<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7 << 13);
        self.0 |= value << 13;
        self
    }

}

impl From<u16> for BaudFracfpMode {
    #[inline]
    fn from(other: u16) -> Self {
         BaudFracfpMode(other)
    }
}

impl ::core::fmt::Display for BaudFracfpMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for BaudFracfpMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.baud() != 0 { try!(write!(f, " baud=0x{:x}", self.baud()))}
        if self.fp() != 0 { try!(write!(f, " fp=0x{:x}", self.fp()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="USART Baud Rate"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct BaudUsartfpMode(pub u16);
impl BaudUsartfpMode {
    #[doc="Baud Rate Value"]
    #[inline] pub fn baud(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if BAUD != 0"]
    #[inline] pub fn test_baud(&self) -> bool {
        self.baud() != 0
    }

    #[doc="Sets the BAUD field."]
    #[inline] pub fn set_baud<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for BaudUsartfpMode {
    #[inline]
    fn from(other: u16) -> Self {
         BaudUsartfpMode(other)
    }
}

impl ::core::fmt::Display for BaudUsartfpMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for BaudUsartfpMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.baud() != 0 { try!(write!(f, " baud=0x{:x}", self.baud()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="USART Receive Pulse Length"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rxpl(pub u8);
impl Rxpl {
    #[doc="Receive Pulse Length"]
    #[inline] pub fn rxpl(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if RXPL != 0"]
    #[inline] pub fn test_rxpl(&self) -> bool {
        self.rxpl() != 0
    }

    #[doc="Sets the RXPL field."]
    #[inline] pub fn set_rxpl<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Rxpl {
    #[inline]
    fn from(other: u8) -> Self {
         Rxpl(other)
    }
}

impl ::core::fmt::Display for Rxpl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rxpl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rxpl() != 0 { try!(write!(f, " rxpl=0x{:x}", self.rxpl()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="USART Interrupt Enable Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
    #[doc="Data Register Empty Interrupt Disable"]
    #[inline] pub fn dre(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DRE != 0"]
    #[inline] pub fn test_dre(&self) -> bool {
        self.dre() != 0
    }

    #[doc="Sets the DRE field."]
    #[inline] pub fn set_dre<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transmit Complete Interrupt Disable"]
    #[inline] pub fn txc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TXC != 0"]
    #[inline] pub fn test_txc(&self) -> bool {
        self.txc() != 0
    }

    #[doc="Sets the TXC field."]
    #[inline] pub fn set_txc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Receive Complete Interrupt Disable"]
    #[inline] pub fn rxc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RXC != 0"]
    #[inline] pub fn test_rxc(&self) -> bool {
        self.rxc() != 0
    }

    #[doc="Sets the RXC field."]
    #[inline] pub fn set_rxc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Receive Start Interrupt Disable"]
    #[inline] pub fn rxs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RXS != 0"]
    #[inline] pub fn test_rxs(&self) -> bool {
        self.rxs() != 0
    }

    #[doc="Sets the RXS field."]
    #[inline] pub fn set_rxs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Clear To Send Input Change Interrupt Disable"]
    #[inline] pub fn ctsic(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CTSIC != 0"]
    #[inline] pub fn test_ctsic(&self) -> bool {
        self.ctsic() != 0
    }

    #[doc="Sets the CTSIC field."]
    #[inline] pub fn set_ctsic<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Break Received Interrupt Disable"]
    #[inline] pub fn rxbrk(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RXBRK != 0"]
    #[inline] pub fn test_rxbrk(&self) -> bool {
        self.rxbrk() != 0
    }

    #[doc="Sets the RXBRK field."]
    #[inline] pub fn set_rxbrk<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Combined Error Interrupt Disable"]
    #[inline] pub fn error(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ERROR != 0"]
    #[inline] pub fn test_error(&self) -> bool {
        self.error() != 0
    }

    #[doc="Sets the ERROR field."]
    #[inline] pub fn set_error<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.dre() != 0 { try!(write!(f, " dre"))}
        if self.txc() != 0 { try!(write!(f, " txc"))}
        if self.rxc() != 0 { try!(write!(f, " rxc"))}
        if self.rxs() != 0 { try!(write!(f, " rxs"))}
        if self.ctsic() != 0 { try!(write!(f, " ctsic"))}
        if self.rxbrk() != 0 { try!(write!(f, " rxbrk"))}
        if self.error() != 0 { try!(write!(f, " error"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="USART Interrupt Enable Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
    #[doc="Data Register Empty Interrupt Enable"]
    #[inline] pub fn dre(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DRE != 0"]
    #[inline] pub fn test_dre(&self) -> bool {
        self.dre() != 0
    }

    #[doc="Sets the DRE field."]
    #[inline] pub fn set_dre<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transmit Complete Interrupt Enable"]
    #[inline] pub fn txc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TXC != 0"]
    #[inline] pub fn test_txc(&self) -> bool {
        self.txc() != 0
    }

    #[doc="Sets the TXC field."]
    #[inline] pub fn set_txc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Receive Complete Interrupt Enable"]
    #[inline] pub fn rxc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RXC != 0"]
    #[inline] pub fn test_rxc(&self) -> bool {
        self.rxc() != 0
    }

    #[doc="Sets the RXC field."]
    #[inline] pub fn set_rxc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Receive Start Interrupt Enable"]
    #[inline] pub fn rxs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RXS != 0"]
    #[inline] pub fn test_rxs(&self) -> bool {
        self.rxs() != 0
    }

    #[doc="Sets the RXS field."]
    #[inline] pub fn set_rxs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Clear To Send Input Change Interrupt Enable"]
    #[inline] pub fn ctsic(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CTSIC != 0"]
    #[inline] pub fn test_ctsic(&self) -> bool {
        self.ctsic() != 0
    }

    #[doc="Sets the CTSIC field."]
    #[inline] pub fn set_ctsic<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Break Received Interrupt Enable"]
    #[inline] pub fn rxbrk(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RXBRK != 0"]
    #[inline] pub fn test_rxbrk(&self) -> bool {
        self.rxbrk() != 0
    }

    #[doc="Sets the RXBRK field."]
    #[inline] pub fn set_rxbrk<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Combined Error Interrupt Enable"]
    #[inline] pub fn error(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ERROR != 0"]
    #[inline] pub fn test_error(&self) -> bool {
        self.error() != 0
    }

    #[doc="Sets the ERROR field."]
    #[inline] pub fn set_error<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.dre() != 0 { try!(write!(f, " dre"))}
        if self.txc() != 0 { try!(write!(f, " txc"))}
        if self.rxc() != 0 { try!(write!(f, " rxc"))}
        if self.rxs() != 0 { try!(write!(f, " rxs"))}
        if self.ctsic() != 0 { try!(write!(f, " ctsic"))}
        if self.rxbrk() != 0 { try!(write!(f, " rxbrk"))}
        if self.error() != 0 { try!(write!(f, " error"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="USART Interrupt Flag Status and Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
    #[doc="Data Register Empty Interrupt"]
    #[inline] pub fn dre(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DRE != 0"]
    #[inline] pub fn test_dre(&self) -> bool {
        self.dre() != 0
    }

    #[doc="Sets the DRE field."]
    #[inline] pub fn set_dre<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transmit Complete Interrupt"]
    #[inline] pub fn txc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TXC != 0"]
    #[inline] pub fn test_txc(&self) -> bool {
        self.txc() != 0
    }

    #[doc="Sets the TXC field."]
    #[inline] pub fn set_txc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Receive Complete Interrupt"]
    #[inline] pub fn rxc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RXC != 0"]
    #[inline] pub fn test_rxc(&self) -> bool {
        self.rxc() != 0
    }

    #[doc="Sets the RXC field."]
    #[inline] pub fn set_rxc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Receive Start Interrupt"]
    #[inline] pub fn rxs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RXS != 0"]
    #[inline] pub fn test_rxs(&self) -> bool {
        self.rxs() != 0
    }

    #[doc="Sets the RXS field."]
    #[inline] pub fn set_rxs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Clear To Send Input Change Interrupt"]
    #[inline] pub fn ctsic(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CTSIC != 0"]
    #[inline] pub fn test_ctsic(&self) -> bool {
        self.ctsic() != 0
    }

    #[doc="Sets the CTSIC field."]
    #[inline] pub fn set_ctsic<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Break Received Interrupt"]
    #[inline] pub fn rxbrk(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RXBRK != 0"]
    #[inline] pub fn test_rxbrk(&self) -> bool {
        self.rxbrk() != 0
    }

    #[doc="Sets the RXBRK field."]
    #[inline] pub fn set_rxbrk<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Combined Error Interrupt"]
    #[inline] pub fn error(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ERROR != 0"]
    #[inline] pub fn test_error(&self) -> bool {
        self.error() != 0
    }

    #[doc="Sets the ERROR field."]
    #[inline] pub fn set_error<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.dre() != 0 { try!(write!(f, " dre"))}
        if self.txc() != 0 { try!(write!(f, " txc"))}
        if self.rxc() != 0 { try!(write!(f, " rxc"))}
        if self.rxs() != 0 { try!(write!(f, " rxs"))}
        if self.ctsic() != 0 { try!(write!(f, " ctsic"))}
        if self.rxbrk() != 0 { try!(write!(f, " rxbrk"))}
        if self.error() != 0 { try!(write!(f, " error"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="USART Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u16);
impl Status {
    #[doc="Parity Error"]
    #[inline] pub fn perr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PERR != 0"]
    #[inline] pub fn test_perr(&self) -> bool {
        self.perr() != 0
    }

    #[doc="Sets the PERR field."]
    #[inline] pub fn set_perr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Frame Error"]
    #[inline] pub fn ferr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if FERR != 0"]
    #[inline] pub fn test_ferr(&self) -> bool {
        self.ferr() != 0
    }

    #[doc="Sets the FERR field."]
    #[inline] pub fn set_ferr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Buffer Overflow"]
    #[inline] pub fn bufovf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if BUFOVF != 0"]
    #[inline] pub fn test_bufovf(&self) -> bool {
        self.bufovf() != 0
    }

    #[doc="Sets the BUFOVF field."]
    #[inline] pub fn set_bufovf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Clear To Send"]
    #[inline] pub fn cts(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CTS != 0"]
    #[inline] pub fn test_cts(&self) -> bool {
        self.cts() != 0
    }

    #[doc="Sets the CTS field."]
    #[inline] pub fn set_cts<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Inconsistent Sync Field"]
    #[inline] pub fn isf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ISF != 0"]
    #[inline] pub fn test_isf(&self) -> bool {
        self.isf() != 0
    }

    #[doc="Sets the ISF field."]
    #[inline] pub fn set_isf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Collision Detected"]
    #[inline] pub fn coll(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if COLL != 0"]
    #[inline] pub fn test_coll(&self) -> bool {
        self.coll() != 0
    }

    #[doc="Sets the COLL field."]
    #[inline] pub fn set_coll<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Transmitter Empty"]
    #[inline] pub fn txe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TXE != 0"]
    #[inline] pub fn test_txe(&self) -> bool {
        self.txe() != 0
    }

    #[doc="Sets the TXE field."]
    #[inline] pub fn set_txe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Maximum Number of Repetitions Reached"]
    #[inline] pub fn iter(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ITER != 0"]
    #[inline] pub fn test_iter(&self) -> bool {
        self.iter() != 0
    }

    #[doc="Sets the ITER field."]
    #[inline] pub fn set_iter<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u16> for Status {
    #[inline]
    fn from(other: u16) -> Self {
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
        if self.perr() != 0 { try!(write!(f, " perr"))}
        if self.ferr() != 0 { try!(write!(f, " ferr"))}
        if self.bufovf() != 0 { try!(write!(f, " bufovf"))}
        if self.cts() != 0 { try!(write!(f, " cts"))}
        if self.isf() != 0 { try!(write!(f, " isf"))}
        if self.coll() != 0 { try!(write!(f, " coll"))}
        if self.txe() != 0 { try!(write!(f, " txe"))}
        if self.iter() != 0 { try!(write!(f, " iter"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="USART Synchronization Busy"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Syncbusy(pub u32);
impl Syncbusy {
    #[doc="Software Reset Synchronization Busy"]
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

    #[doc="SERCOM Enable Synchronization Busy"]
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

    #[doc="CTRLB Synchronization Busy"]
    #[inline] pub fn ctrlb(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CTRLB != 0"]
    #[inline] pub fn test_ctrlb(&self) -> bool {
        self.ctrlb() != 0
    }

    #[doc="Sets the CTRLB field."]
    #[inline] pub fn set_ctrlb<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="RXERRCNT Synchronization Busy"]
    #[inline] pub fn rxerrcnt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RXERRCNT != 0"]
    #[inline] pub fn test_rxerrcnt(&self) -> bool {
        self.rxerrcnt() != 0
    }

    #[doc="Sets the RXERRCNT field."]
    #[inline] pub fn set_rxerrcnt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="LENGTH Synchronization Busy"]
    #[inline] pub fn length(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if LENGTH != 0"]
    #[inline] pub fn test_length(&self) -> bool {
        self.length() != 0
    }

    #[doc="Sets the LENGTH field."]
    #[inline] pub fn set_length<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
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
        if self.ctrlb() != 0 { try!(write!(f, " ctrlb"))}
        if self.rxerrcnt() != 0 { try!(write!(f, " rxerrcnt"))}
        if self.length() != 0 { try!(write!(f, " length"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="USART Receive Error Count"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rxerrcnt(pub u8);
impl Rxerrcnt {
}

impl From<u8> for Rxerrcnt {
    #[inline]
    fn from(other: u8) -> Self {
         Rxerrcnt(other)
    }
}

impl ::core::fmt::Display for Rxerrcnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rxerrcnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="USART Length"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Length(pub u16);
impl Length {
    #[doc="Data Length"]
    #[inline] pub fn len(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if LEN != 0"]
    #[inline] pub fn test_len(&self) -> bool {
        self.len() != 0
    }

    #[doc="Sets the LEN field."]
    #[inline] pub fn set_len<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Data Length Enable"]
    #[inline] pub fn lenen(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if LENEN != 0"]
    #[inline] pub fn test_lenen(&self) -> bool {
        self.lenen() != 0
    }

    #[doc="Sets the LENEN field."]
    #[inline] pub fn set_lenen<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u16> for Length {
    #[inline]
    fn from(other: u16) -> Self {
         Length(other)
    }
}

impl ::core::fmt::Display for Length {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Length {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.len() != 0 { try!(write!(f, " len=0x{:x}", self.len()))}
        if self.lenen() != 0 { try!(write!(f, " lenen=0x{:x}", self.lenen()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="USART Data"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Data(pub u32);
impl Data {
    #[doc="Data Value"]
    #[inline] pub fn data(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if DATA != 0"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data() != 0
    }

    #[doc="Sets the DATA field."]
    #[inline] pub fn set_data<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Data {
    #[inline]
    fn from(other: u32) -> Self {
         Data(other)
    }
}

impl ::core::fmt::Display for Data {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Data {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="USART Debug Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
    #[doc="Debug Mode"]
    #[inline] pub fn dbgstop(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DBGSTOP != 0"]
    #[inline] pub fn test_dbgstop(&self) -> bool {
        self.dbgstop() != 0
    }

    #[doc="Sets the DBGSTOP field."]
    #[inline] pub fn set_dbgstop<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.dbgstop() != 0 { try!(write!(f, " dbgstop"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

}
// End of usart

