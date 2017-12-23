#[allow(unused_imports)] use bobbin_common::*;

periph!( LCD, Lcd, _LCD, LcdPeriph, 0x40002400);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="LCD Peripheral"]
pub struct LcdPeriph(pub usize); 



impl LcdPeriph {
    #[doc="Get the *mut pointer for the CR register."]
    #[inline] pub fn cr_mut(&self) -> *mut Cr { 
        (self.0 + 0x0) as *mut Cr
    }

    #[doc="Get the *const pointer for the CR register."]
    #[inline] pub fn cr_ptr(&self) -> *const Cr { 
           self.cr_mut()
    }

    #[doc="Read the CR register."]
    #[inline] pub fn cr(&self) -> Cr { 
        unsafe {
            read_volatile(self.cr_ptr())
        }
    }

    #[doc="Write the CR register."]
    #[inline] pub fn set_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cr_mut(), f(Cr(0)));
        }
        self
    }

    #[doc="Modify the CR register."]
    #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cr_mut(), f(self.cr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FCR register."]
    #[inline] pub fn fcr_mut(&self) -> *mut Fcr { 
        (self.0 + 0x4) as *mut Fcr
    }

    #[doc="Get the *const pointer for the FCR register."]
    #[inline] pub fn fcr_ptr(&self) -> *const Fcr { 
           self.fcr_mut()
    }

    #[doc="Read the FCR register."]
    #[inline] pub fn fcr(&self) -> Fcr { 
        unsafe {
            read_volatile(self.fcr_ptr())
        }
    }

    #[doc="Write the FCR register."]
    #[inline] pub fn set_fcr<F: FnOnce(Fcr) -> Fcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fcr_mut(), f(Fcr(0)));
        }
        self
    }

    #[doc="Modify the FCR register."]
    #[inline] pub fn with_fcr<F: FnOnce(Fcr) -> Fcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fcr_mut(), f(self.fcr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SR register."]
    #[inline] pub fn sr_mut(&self) -> *mut Sr { 
        (self.0 + 0x8) as *mut Sr
    }

    #[doc="Get the *const pointer for the SR register."]
    #[inline] pub fn sr_ptr(&self) -> *const Sr { 
           self.sr_mut()
    }

    #[doc="Read the SR register."]
    #[inline] pub fn sr(&self) -> Sr { 
        unsafe {
            read_volatile(self.sr_ptr())
        }
    }

    #[doc="Write the SR register."]
    #[inline] pub fn set_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sr_mut(), f(Sr(0)));
        }
        self
    }

    #[doc="Modify the SR register."]
    #[inline] pub fn with_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sr_mut(), f(self.sr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CLR register."]
    #[inline] pub fn clr_mut(&self) -> *mut Clr { 
        (self.0 + 0xc) as *mut Clr
    }

    #[doc="Get the *const pointer for the CLR register."]
    #[inline] pub fn clr_ptr(&self) -> *const Clr { 
           self.clr_mut()
    }

    #[doc="Write the CLR register."]
    #[inline] pub fn set_clr<F: FnOnce(Clr) -> Clr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clr_mut(), f(Clr(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the RAM_COM0 register."]
    #[inline] pub fn ram_com0_mut(&self) -> *mut RamCom0 { 
        (self.0 + 0x14) as *mut RamCom0
    }

    #[doc="Get the *const pointer for the RAM_COM0 register."]
    #[inline] pub fn ram_com0_ptr(&self) -> *const RamCom0 { 
           self.ram_com0_mut()
    }

    #[doc="Read the RAM_COM0 register."]
    #[inline] pub fn ram_com0(&self) -> RamCom0 { 
        unsafe {
            read_volatile(self.ram_com0_ptr())
        }
    }

    #[doc="Write the RAM_COM0 register."]
    #[inline] pub fn set_ram_com0<F: FnOnce(RamCom0) -> RamCom0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ram_com0_mut(), f(RamCom0(0)));
        }
        self
    }

    #[doc="Modify the RAM_COM0 register."]
    #[inline] pub fn with_ram_com0<F: FnOnce(RamCom0) -> RamCom0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ram_com0_mut(), f(self.ram_com0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the RAM_COM1 register."]
    #[inline] pub fn ram_com1_mut(&self) -> *mut RamCom1 { 
        (self.0 + 0x1c) as *mut RamCom1
    }

    #[doc="Get the *const pointer for the RAM_COM1 register."]
    #[inline] pub fn ram_com1_ptr(&self) -> *const RamCom1 { 
           self.ram_com1_mut()
    }

    #[doc="Read the RAM_COM1 register."]
    #[inline] pub fn ram_com1(&self) -> RamCom1 { 
        unsafe {
            read_volatile(self.ram_com1_ptr())
        }
    }

    #[doc="Write the RAM_COM1 register."]
    #[inline] pub fn set_ram_com1<F: FnOnce(RamCom1) -> RamCom1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ram_com1_mut(), f(RamCom1(0)));
        }
        self
    }

    #[doc="Modify the RAM_COM1 register."]
    #[inline] pub fn with_ram_com1<F: FnOnce(RamCom1) -> RamCom1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ram_com1_mut(), f(self.ram_com1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the RAM_COM2 register."]
    #[inline] pub fn ram_com2_mut(&self) -> *mut RamCom2 { 
        (self.0 + 0x24) as *mut RamCom2
    }

    #[doc="Get the *const pointer for the RAM_COM2 register."]
    #[inline] pub fn ram_com2_ptr(&self) -> *const RamCom2 { 
           self.ram_com2_mut()
    }

    #[doc="Read the RAM_COM2 register."]
    #[inline] pub fn ram_com2(&self) -> RamCom2 { 
        unsafe {
            read_volatile(self.ram_com2_ptr())
        }
    }

    #[doc="Write the RAM_COM2 register."]
    #[inline] pub fn set_ram_com2<F: FnOnce(RamCom2) -> RamCom2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ram_com2_mut(), f(RamCom2(0)));
        }
        self
    }

    #[doc="Modify the RAM_COM2 register."]
    #[inline] pub fn with_ram_com2<F: FnOnce(RamCom2) -> RamCom2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ram_com2_mut(), f(self.ram_com2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the RAM_COM3 register."]
    #[inline] pub fn ram_com3_mut(&self) -> *mut RamCom3 { 
        (self.0 + 0x2c) as *mut RamCom3
    }

    #[doc="Get the *const pointer for the RAM_COM3 register."]
    #[inline] pub fn ram_com3_ptr(&self) -> *const RamCom3 { 
           self.ram_com3_mut()
    }

    #[doc="Read the RAM_COM3 register."]
    #[inline] pub fn ram_com3(&self) -> RamCom3 { 
        unsafe {
            read_volatile(self.ram_com3_ptr())
        }
    }

    #[doc="Write the RAM_COM3 register."]
    #[inline] pub fn set_ram_com3<F: FnOnce(RamCom3) -> RamCom3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ram_com3_mut(), f(RamCom3(0)));
        }
        self
    }

    #[doc="Modify the RAM_COM3 register."]
    #[inline] pub fn with_ram_com3<F: FnOnce(RamCom3) -> RamCom3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ram_com3_mut(), f(self.ram_com3()));
        }
        self
    }

    #[doc="Get the *mut pointer for the RAM_COM4 register."]
    #[inline] pub fn ram_com4_mut(&self) -> *mut RamCom4 { 
        (self.0 + 0x34) as *mut RamCom4
    }

    #[doc="Get the *const pointer for the RAM_COM4 register."]
    #[inline] pub fn ram_com4_ptr(&self) -> *const RamCom4 { 
           self.ram_com4_mut()
    }

    #[doc="Read the RAM_COM4 register."]
    #[inline] pub fn ram_com4(&self) -> RamCom4 { 
        unsafe {
            read_volatile(self.ram_com4_ptr())
        }
    }

    #[doc="Write the RAM_COM4 register."]
    #[inline] pub fn set_ram_com4<F: FnOnce(RamCom4) -> RamCom4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ram_com4_mut(), f(RamCom4(0)));
        }
        self
    }

    #[doc="Modify the RAM_COM4 register."]
    #[inline] pub fn with_ram_com4<F: FnOnce(RamCom4) -> RamCom4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ram_com4_mut(), f(self.ram_com4()));
        }
        self
    }

    #[doc="Get the *mut pointer for the RAM_COM5 register."]
    #[inline] pub fn ram_com5_mut(&self) -> *mut RamCom5 { 
        (self.0 + 0x3c) as *mut RamCom5
    }

    #[doc="Get the *const pointer for the RAM_COM5 register."]
    #[inline] pub fn ram_com5_ptr(&self) -> *const RamCom5 { 
           self.ram_com5_mut()
    }

    #[doc="Read the RAM_COM5 register."]
    #[inline] pub fn ram_com5(&self) -> RamCom5 { 
        unsafe {
            read_volatile(self.ram_com5_ptr())
        }
    }

    #[doc="Write the RAM_COM5 register."]
    #[inline] pub fn set_ram_com5<F: FnOnce(RamCom5) -> RamCom5>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ram_com5_mut(), f(RamCom5(0)));
        }
        self
    }

    #[doc="Modify the RAM_COM5 register."]
    #[inline] pub fn with_ram_com5<F: FnOnce(RamCom5) -> RamCom5>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ram_com5_mut(), f(self.ram_com5()));
        }
        self
    }

    #[doc="Get the *mut pointer for the RAM_COM6 register."]
    #[inline] pub fn ram_com6_mut(&self) -> *mut RamCom6 { 
        (self.0 + 0x44) as *mut RamCom6
    }

    #[doc="Get the *const pointer for the RAM_COM6 register."]
    #[inline] pub fn ram_com6_ptr(&self) -> *const RamCom6 { 
           self.ram_com6_mut()
    }

    #[doc="Read the RAM_COM6 register."]
    #[inline] pub fn ram_com6(&self) -> RamCom6 { 
        unsafe {
            read_volatile(self.ram_com6_ptr())
        }
    }

    #[doc="Write the RAM_COM6 register."]
    #[inline] pub fn set_ram_com6<F: FnOnce(RamCom6) -> RamCom6>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ram_com6_mut(), f(RamCom6(0)));
        }
        self
    }

    #[doc="Modify the RAM_COM6 register."]
    #[inline] pub fn with_ram_com6<F: FnOnce(RamCom6) -> RamCom6>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ram_com6_mut(), f(self.ram_com6()));
        }
        self
    }

    #[doc="Get the *mut pointer for the RAM_COM7 register."]
    #[inline] pub fn ram_com7_mut(&self) -> *mut RamCom7 { 
        (self.0 + 0x4c) as *mut RamCom7
    }

    #[doc="Get the *const pointer for the RAM_COM7 register."]
    #[inline] pub fn ram_com7_ptr(&self) -> *const RamCom7 { 
           self.ram_com7_mut()
    }

    #[doc="Read the RAM_COM7 register."]
    #[inline] pub fn ram_com7(&self) -> RamCom7 { 
        unsafe {
            read_volatile(self.ram_com7_ptr())
        }
    }

    #[doc="Write the RAM_COM7 register."]
    #[inline] pub fn set_ram_com7<F: FnOnce(RamCom7) -> RamCom7>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ram_com7_mut(), f(RamCom7(0)));
        }
        self
    }

    #[doc="Modify the RAM_COM7 register."]
    #[inline] pub fn with_ram_com7<F: FnOnce(RamCom7) -> RamCom7>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ram_com7_mut(), f(self.ram_com7()));
        }
        self
    }

}

#[doc="control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="Bias selector"]
    #[inline] pub fn bias(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="Returns true if BIAS != 0"]
    #[inline] pub fn test_bias(&self) -> bool {
        self.bias() != 0
    }

    #[doc="Sets the BIAS field."]
    #[inline] pub fn set_bias<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Duty selection"]
    #[inline] pub fn duty(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x7) as u8) } // [4:2]
    }

    #[doc="Returns true if DUTY != 0"]
    #[inline] pub fn test_duty(&self) -> bool {
        self.duty() != 0
    }

    #[doc="Sets the DUTY field."]
    #[inline] pub fn set_duty<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Voltage source selection"]
    #[inline] pub fn vsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if VSEL != 0"]
    #[inline] pub fn test_vsel(&self) -> bool {
        self.vsel() != 0
    }

    #[doc="Sets the VSEL field."]
    #[inline] pub fn set_vsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="LCD controller enable"]
    #[inline] pub fn lcden(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LCDEN != 0"]
    #[inline] pub fn test_lcden(&self) -> bool {
        self.lcden() != 0
    }

    #[doc="Sets the LCDEN field."]
    #[inline] pub fn set_lcden<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Mux segment enable"]
    #[inline] pub fn mux_seg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if MUX_SEG != 0"]
    #[inline] pub fn test_mux_seg(&self) -> bool {
        self.mux_seg() != 0
    }

    #[doc="Sets the MUX_SEG field."]
    #[inline] pub fn set_mux_seg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Voltage output buffer enable"]
    #[inline] pub fn bufen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BUFEN != 0"]
    #[inline] pub fn test_bufen(&self) -> bool {
        self.bufen() != 0
    }

    #[doc="Sets the BUFEN field."]
    #[inline] pub fn set_bufen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
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
        if self.bias() != 0 { try!(write!(f, " bias=0x{:x}", self.bias()))}
        if self.duty() != 0 { try!(write!(f, " duty=0x{:x}", self.duty()))}
        if self.vsel() != 0 { try!(write!(f, " vsel"))}
        if self.lcden() != 0 { try!(write!(f, " lcden"))}
        if self.mux_seg() != 0 { try!(write!(f, " mux_seg"))}
        if self.bufen() != 0 { try!(write!(f, " bufen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="frame control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fcr(pub u32);
impl Fcr {
    #[doc="PS 16-bit prescaler"]
    #[inline] pub fn ps(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0xf) as u8) } // [25:22]
    }

    #[doc="Returns true if PS != 0"]
    #[inline] pub fn test_ps(&self) -> bool {
        self.ps() != 0
    }

    #[doc="Sets the PS field."]
    #[inline] pub fn set_ps<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="DIV clock divider"]
    #[inline] pub fn div(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0xf) as u8) } // [21:18]
    }

    #[doc="Returns true if DIV != 0"]
    #[inline] pub fn test_div(&self) -> bool {
        self.div() != 0
    }

    #[doc="Sets the DIV field."]
    #[inline] pub fn set_div<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Blink mode selection"]
    #[inline] pub fn blink(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if BLINK != 0"]
    #[inline] pub fn test_blink(&self) -> bool {
        self.blink() != 0
    }

    #[doc="Sets the BLINK field."]
    #[inline] pub fn set_blink<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Blink frequency selection"]
    #[inline] pub fn blinkf(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x7) as u8) } // [15:13]
    }

    #[doc="Returns true if BLINKF != 0"]
    #[inline] pub fn test_blinkf(&self) -> bool {
        self.blinkf() != 0
    }

    #[doc="Sets the BLINKF field."]
    #[inline] pub fn set_blinkf<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Contrast control"]
    #[inline] pub fn cc(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x7) as u8) } // [12:10]
    }

    #[doc="Returns true if CC != 0"]
    #[inline] pub fn test_cc(&self) -> bool {
        self.cc() != 0
    }

    #[doc="Sets the CC field."]
    #[inline] pub fn set_cc<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Dead time duration"]
    #[inline] pub fn dead(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x7) as u8) } // [9:7]
    }

    #[doc="Returns true if DEAD != 0"]
    #[inline] pub fn test_dead(&self) -> bool {
        self.dead() != 0
    }

    #[doc="Sets the DEAD field."]
    #[inline] pub fn set_dead<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Pulse ON duration"]
    #[inline] pub fn pon(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if PON != 0"]
    #[inline] pub fn test_pon(&self) -> bool {
        self.pon() != 0
    }

    #[doc="Sets the PON field."]
    #[inline] pub fn set_pon<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Update display done interrupt enable"]
    #[inline] pub fn uddie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if UDDIE != 0"]
    #[inline] pub fn test_uddie(&self) -> bool {
        self.uddie() != 0
    }

    #[doc="Sets the UDDIE field."]
    #[inline] pub fn set_uddie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Start of frame interrupt enable"]
    #[inline] pub fn sofie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SOFIE != 0"]
    #[inline] pub fn test_sofie(&self) -> bool {
        self.sofie() != 0
    }

    #[doc="Sets the SOFIE field."]
    #[inline] pub fn set_sofie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="High drive enable"]
    #[inline] pub fn hd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if HD != 0"]
    #[inline] pub fn test_hd(&self) -> bool {
        self.hd() != 0
    }

    #[doc="Sets the HD field."]
    #[inline] pub fn set_hd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Fcr {
    #[inline]
    fn from(other: u32) -> Self {
         Fcr(other)
    }
}

impl ::core::fmt::Display for Fcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ps() != 0 { try!(write!(f, " ps=0x{:x}", self.ps()))}
        if self.div() != 0 { try!(write!(f, " div=0x{:x}", self.div()))}
        if self.blink() != 0 { try!(write!(f, " blink=0x{:x}", self.blink()))}
        if self.blinkf() != 0 { try!(write!(f, " blinkf=0x{:x}", self.blinkf()))}
        if self.cc() != 0 { try!(write!(f, " cc=0x{:x}", self.cc()))}
        if self.dead() != 0 { try!(write!(f, " dead=0x{:x}", self.dead()))}
        if self.pon() != 0 { try!(write!(f, " pon=0x{:x}", self.pon()))}
        if self.uddie() != 0 { try!(write!(f, " uddie"))}
        if self.sofie() != 0 { try!(write!(f, " sofie"))}
        if self.hd() != 0 { try!(write!(f, " hd"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc="LCD Frame Control Register Synchronization flag"]
    #[inline] pub fn fcrsf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if FCRSF != 0"]
    #[inline] pub fn test_fcrsf(&self) -> bool {
        self.fcrsf() != 0
    }

    #[doc="Sets the FCRSF field."]
    #[inline] pub fn set_fcrsf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Ready flag"]
    #[inline] pub fn rdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RDY != 0"]
    #[inline] pub fn test_rdy(&self) -> bool {
        self.rdy() != 0
    }

    #[doc="Sets the RDY field."]
    #[inline] pub fn set_rdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Update Display Done"]
    #[inline] pub fn udd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if UDD != 0"]
    #[inline] pub fn test_udd(&self) -> bool {
        self.udd() != 0
    }

    #[doc="Sets the UDD field."]
    #[inline] pub fn set_udd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Update display request"]
    #[inline] pub fn udr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if UDR != 0"]
    #[inline] pub fn test_udr(&self) -> bool {
        self.udr() != 0
    }

    #[doc="Sets the UDR field."]
    #[inline] pub fn set_udr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Start of frame flag"]
    #[inline] pub fn sof(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SOF != 0"]
    #[inline] pub fn test_sof(&self) -> bool {
        self.sof() != 0
    }

    #[doc="Sets the SOF field."]
    #[inline] pub fn set_sof<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="ENS"]
    #[inline] pub fn ens(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ENS != 0"]
    #[inline] pub fn test_ens(&self) -> bool {
        self.ens() != 0
    }

    #[doc="Sets the ENS field."]
    #[inline] pub fn set_ens<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
        if self.fcrsf() != 0 { try!(write!(f, " fcrsf"))}
        if self.rdy() != 0 { try!(write!(f, " rdy"))}
        if self.udd() != 0 { try!(write!(f, " udd"))}
        if self.udr() != 0 { try!(write!(f, " udr"))}
        if self.sof() != 0 { try!(write!(f, " sof"))}
        if self.ens() != 0 { try!(write!(f, " ens"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="clear register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clr(pub u32);
impl Clr {
    #[doc="Update display done clear"]
    #[inline] pub fn uddc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if UDDC != 0"]
    #[inline] pub fn test_uddc(&self) -> bool {
        self.uddc() != 0
    }

    #[doc="Sets the UDDC field."]
    #[inline] pub fn set_uddc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Start of frame flag clear"]
    #[inline] pub fn sofc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SOFC != 0"]
    #[inline] pub fn test_sofc(&self) -> bool {
        self.sofc() != 0
    }

    #[doc="Sets the SOFC field."]
    #[inline] pub fn set_sofc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Clr {
    #[inline]
    fn from(other: u32) -> Self {
         Clr(other)
    }
}

impl ::core::fmt::Display for Clr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Clr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.uddc() != 0 { try!(write!(f, " uddc"))}
        if self.sofc() != 0 { try!(write!(f, " sofc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="display memory"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RamCom0(pub u32);
impl RamCom0 {
    #[doc="S30"]
    #[inline] pub fn s30(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if S30 != 0"]
    #[inline] pub fn test_s30(&self) -> bool {
        self.s30() != 0
    }

    #[doc="Sets the S30 field."]
    #[inline] pub fn set_s30<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="S29"]
    #[inline] pub fn s29(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if S29 != 0"]
    #[inline] pub fn test_s29(&self) -> bool {
        self.s29() != 0
    }

    #[doc="Sets the S29 field."]
    #[inline] pub fn set_s29<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="S28"]
    #[inline] pub fn s28(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if S28 != 0"]
    #[inline] pub fn test_s28(&self) -> bool {
        self.s28() != 0
    }

    #[doc="Sets the S28 field."]
    #[inline] pub fn set_s28<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="S27"]
    #[inline] pub fn s27(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if S27 != 0"]
    #[inline] pub fn test_s27(&self) -> bool {
        self.s27() != 0
    }

    #[doc="Sets the S27 field."]
    #[inline] pub fn set_s27<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="S26"]
    #[inline] pub fn s26(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if S26 != 0"]
    #[inline] pub fn test_s26(&self) -> bool {
        self.s26() != 0
    }

    #[doc="Sets the S26 field."]
    #[inline] pub fn set_s26<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="S25"]
    #[inline] pub fn s25(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if S25 != 0"]
    #[inline] pub fn test_s25(&self) -> bool {
        self.s25() != 0
    }

    #[doc="Sets the S25 field."]
    #[inline] pub fn set_s25<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="S24"]
    #[inline] pub fn s24(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if S24 != 0"]
    #[inline] pub fn test_s24(&self) -> bool {
        self.s24() != 0
    }

    #[doc="Sets the S24 field."]
    #[inline] pub fn set_s24<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="S23"]
    #[inline] pub fn s23(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if S23 != 0"]
    #[inline] pub fn test_s23(&self) -> bool {
        self.s23() != 0
    }

    #[doc="Sets the S23 field."]
    #[inline] pub fn set_s23<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="S22"]
    #[inline] pub fn s22(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if S22 != 0"]
    #[inline] pub fn test_s22(&self) -> bool {
        self.s22() != 0
    }

    #[doc="Sets the S22 field."]
    #[inline] pub fn set_s22<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="S21"]
    #[inline] pub fn s21(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if S21 != 0"]
    #[inline] pub fn test_s21(&self) -> bool {
        self.s21() != 0
    }

    #[doc="Sets the S21 field."]
    #[inline] pub fn set_s21<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="S20"]
    #[inline] pub fn s20(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if S20 != 0"]
    #[inline] pub fn test_s20(&self) -> bool {
        self.s20() != 0
    }

    #[doc="Sets the S20 field."]
    #[inline] pub fn set_s20<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="S19"]
    #[inline] pub fn s19(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if S19 != 0"]
    #[inline] pub fn test_s19(&self) -> bool {
        self.s19() != 0
    }

    #[doc="Sets the S19 field."]
    #[inline] pub fn set_s19<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="S18"]
    #[inline] pub fn s18(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if S18 != 0"]
    #[inline] pub fn test_s18(&self) -> bool {
        self.s18() != 0
    }

    #[doc="Sets the S18 field."]
    #[inline] pub fn set_s18<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="S17"]
    #[inline] pub fn s17(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if S17 != 0"]
    #[inline] pub fn test_s17(&self) -> bool {
        self.s17() != 0
    }

    #[doc="Sets the S17 field."]
    #[inline] pub fn set_s17<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="S16"]
    #[inline] pub fn s16(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if S16 != 0"]
    #[inline] pub fn test_s16(&self) -> bool {
        self.s16() != 0
    }

    #[doc="Sets the S16 field."]
    #[inline] pub fn set_s16<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="S15"]
    #[inline] pub fn s15(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if S15 != 0"]
    #[inline] pub fn test_s15(&self) -> bool {
        self.s15() != 0
    }

    #[doc="Sets the S15 field."]
    #[inline] pub fn set_s15<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="S14"]
    #[inline] pub fn s14(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if S14 != 0"]
    #[inline] pub fn test_s14(&self) -> bool {
        self.s14() != 0
    }

    #[doc="Sets the S14 field."]
    #[inline] pub fn set_s14<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="S13"]
    #[inline] pub fn s13(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if S13 != 0"]
    #[inline] pub fn test_s13(&self) -> bool {
        self.s13() != 0
    }

    #[doc="Sets the S13 field."]
    #[inline] pub fn set_s13<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="S12"]
    #[inline] pub fn s12(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if S12 != 0"]
    #[inline] pub fn test_s12(&self) -> bool {
        self.s12() != 0
    }

    #[doc="Sets the S12 field."]
    #[inline] pub fn set_s12<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="S11"]
    #[inline] pub fn s11(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if S11 != 0"]
    #[inline] pub fn test_s11(&self) -> bool {
        self.s11() != 0
    }

    #[doc="Sets the S11 field."]
    #[inline] pub fn set_s11<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="S10"]
    #[inline] pub fn s10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if S10 != 0"]
    #[inline] pub fn test_s10(&self) -> bool {
        self.s10() != 0
    }

    #[doc="Sets the S10 field."]
    #[inline] pub fn set_s10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="S09"]
    #[inline] pub fn s09(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if S09 != 0"]
    #[inline] pub fn test_s09(&self) -> bool {
        self.s09() != 0
    }

    #[doc="Sets the S09 field."]
    #[inline] pub fn set_s09<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="S08"]
    #[inline] pub fn s08(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if S08 != 0"]
    #[inline] pub fn test_s08(&self) -> bool {
        self.s08() != 0
    }

    #[doc="Sets the S08 field."]
    #[inline] pub fn set_s08<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="S07"]
    #[inline] pub fn s07(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if S07 != 0"]
    #[inline] pub fn test_s07(&self) -> bool {
        self.s07() != 0
    }

    #[doc="Sets the S07 field."]
    #[inline] pub fn set_s07<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="S06"]
    #[inline] pub fn s06(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if S06 != 0"]
    #[inline] pub fn test_s06(&self) -> bool {
        self.s06() != 0
    }

    #[doc="Sets the S06 field."]
    #[inline] pub fn set_s06<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="S05"]
    #[inline] pub fn s05(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if S05 != 0"]
    #[inline] pub fn test_s05(&self) -> bool {
        self.s05() != 0
    }

    #[doc="Sets the S05 field."]
    #[inline] pub fn set_s05<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="S04"]
    #[inline] pub fn s04(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if S04 != 0"]
    #[inline] pub fn test_s04(&self) -> bool {
        self.s04() != 0
    }

    #[doc="Sets the S04 field."]
    #[inline] pub fn set_s04<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="S03"]
    #[inline] pub fn s03(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if S03 != 0"]
    #[inline] pub fn test_s03(&self) -> bool {
        self.s03() != 0
    }

    #[doc="Sets the S03 field."]
    #[inline] pub fn set_s03<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="S02"]
    #[inline] pub fn s02(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if S02 != 0"]
    #[inline] pub fn test_s02(&self) -> bool {
        self.s02() != 0
    }

    #[doc="Sets the S02 field."]
    #[inline] pub fn set_s02<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="S01"]
    #[inline] pub fn s01(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if S01 != 0"]
    #[inline] pub fn test_s01(&self) -> bool {
        self.s01() != 0
    }

    #[doc="Sets the S01 field."]
    #[inline] pub fn set_s01<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="S00"]
    #[inline] pub fn s00(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if S00 != 0"]
    #[inline] pub fn test_s00(&self) -> bool {
        self.s00() != 0
    }

    #[doc="Sets the S00 field."]
    #[inline] pub fn set_s00<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for RamCom0 {
    #[inline]
    fn from(other: u32) -> Self {
         RamCom0(other)
    }
}

impl ::core::fmt::Display for RamCom0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RamCom0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.s30() != 0 { try!(write!(f, " s30"))}
        if self.s29() != 0 { try!(write!(f, " s29"))}
        if self.s28() != 0 { try!(write!(f, " s28"))}
        if self.s27() != 0 { try!(write!(f, " s27"))}
        if self.s26() != 0 { try!(write!(f, " s26"))}
        if self.s25() != 0 { try!(write!(f, " s25"))}
        if self.s24() != 0 { try!(write!(f, " s24"))}
        if self.s23() != 0 { try!(write!(f, " s23"))}
        if self.s22() != 0 { try!(write!(f, " s22"))}
        if self.s21() != 0 { try!(write!(f, " s21"))}
        if self.s20() != 0 { try!(write!(f, " s20"))}
        if self.s19() != 0 { try!(write!(f, " s19"))}
        if self.s18() != 0 { try!(write!(f, " s18"))}
        if self.s17() != 0 { try!(write!(f, " s17"))}
        if self.s16() != 0 { try!(write!(f, " s16"))}
        if self.s15() != 0 { try!(write!(f, " s15"))}
        if self.s14() != 0 { try!(write!(f, " s14"))}
        if self.s13() != 0 { try!(write!(f, " s13"))}
        if self.s12() != 0 { try!(write!(f, " s12"))}
        if self.s11() != 0 { try!(write!(f, " s11"))}
        if self.s10() != 0 { try!(write!(f, " s10"))}
        if self.s09() != 0 { try!(write!(f, " s09"))}
        if self.s08() != 0 { try!(write!(f, " s08"))}
        if self.s07() != 0 { try!(write!(f, " s07"))}
        if self.s06() != 0 { try!(write!(f, " s06"))}
        if self.s05() != 0 { try!(write!(f, " s05"))}
        if self.s04() != 0 { try!(write!(f, " s04"))}
        if self.s03() != 0 { try!(write!(f, " s03"))}
        if self.s02() != 0 { try!(write!(f, " s02"))}
        if self.s01() != 0 { try!(write!(f, " s01"))}
        if self.s00() != 0 { try!(write!(f, " s00"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="display memory"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RamCom1(pub u32);
impl RamCom1 {
    #[doc="S31"]
    #[inline] pub fn s31(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if S31 != 0"]
    #[inline] pub fn test_s31(&self) -> bool {
        self.s31() != 0
    }

    #[doc="Sets the S31 field."]
    #[inline] pub fn set_s31<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="S30"]
    #[inline] pub fn s30(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if S30 != 0"]
    #[inline] pub fn test_s30(&self) -> bool {
        self.s30() != 0
    }

    #[doc="Sets the S30 field."]
    #[inline] pub fn set_s30<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="S29"]
    #[inline] pub fn s29(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if S29 != 0"]
    #[inline] pub fn test_s29(&self) -> bool {
        self.s29() != 0
    }

    #[doc="Sets the S29 field."]
    #[inline] pub fn set_s29<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="S28"]
    #[inline] pub fn s28(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if S28 != 0"]
    #[inline] pub fn test_s28(&self) -> bool {
        self.s28() != 0
    }

    #[doc="Sets the S28 field."]
    #[inline] pub fn set_s28<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="S27"]
    #[inline] pub fn s27(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if S27 != 0"]
    #[inline] pub fn test_s27(&self) -> bool {
        self.s27() != 0
    }

    #[doc="Sets the S27 field."]
    #[inline] pub fn set_s27<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="S26"]
    #[inline] pub fn s26(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if S26 != 0"]
    #[inline] pub fn test_s26(&self) -> bool {
        self.s26() != 0
    }

    #[doc="Sets the S26 field."]
    #[inline] pub fn set_s26<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="S25"]
    #[inline] pub fn s25(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if S25 != 0"]
    #[inline] pub fn test_s25(&self) -> bool {
        self.s25() != 0
    }

    #[doc="Sets the S25 field."]
    #[inline] pub fn set_s25<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="S24"]
    #[inline] pub fn s24(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if S24 != 0"]
    #[inline] pub fn test_s24(&self) -> bool {
        self.s24() != 0
    }

    #[doc="Sets the S24 field."]
    #[inline] pub fn set_s24<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="S23"]
    #[inline] pub fn s23(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if S23 != 0"]
    #[inline] pub fn test_s23(&self) -> bool {
        self.s23() != 0
    }

    #[doc="Sets the S23 field."]
    #[inline] pub fn set_s23<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="S22"]
    #[inline] pub fn s22(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if S22 != 0"]
    #[inline] pub fn test_s22(&self) -> bool {
        self.s22() != 0
    }

    #[doc="Sets the S22 field."]
    #[inline] pub fn set_s22<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="S21"]
    #[inline] pub fn s21(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if S21 != 0"]
    #[inline] pub fn test_s21(&self) -> bool {
        self.s21() != 0
    }

    #[doc="Sets the S21 field."]
    #[inline] pub fn set_s21<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="S20"]
    #[inline] pub fn s20(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if S20 != 0"]
    #[inline] pub fn test_s20(&self) -> bool {
        self.s20() != 0
    }

    #[doc="Sets the S20 field."]
    #[inline] pub fn set_s20<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="S19"]
    #[inline] pub fn s19(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if S19 != 0"]
    #[inline] pub fn test_s19(&self) -> bool {
        self.s19() != 0
    }

    #[doc="Sets the S19 field."]
    #[inline] pub fn set_s19<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="S18"]
    #[inline] pub fn s18(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if S18 != 0"]
    #[inline] pub fn test_s18(&self) -> bool {
        self.s18() != 0
    }

    #[doc="Sets the S18 field."]
    #[inline] pub fn set_s18<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="S17"]
    #[inline] pub fn s17(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if S17 != 0"]
    #[inline] pub fn test_s17(&self) -> bool {
        self.s17() != 0
    }

    #[doc="Sets the S17 field."]
    #[inline] pub fn set_s17<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="S16"]
    #[inline] pub fn s16(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if S16 != 0"]
    #[inline] pub fn test_s16(&self) -> bool {
        self.s16() != 0
    }

    #[doc="Sets the S16 field."]
    #[inline] pub fn set_s16<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="S15"]
    #[inline] pub fn s15(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if S15 != 0"]
    #[inline] pub fn test_s15(&self) -> bool {
        self.s15() != 0
    }

    #[doc="Sets the S15 field."]
    #[inline] pub fn set_s15<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="S14"]
    #[inline] pub fn s14(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if S14 != 0"]
    #[inline] pub fn test_s14(&self) -> bool {
        self.s14() != 0
    }

    #[doc="Sets the S14 field."]
    #[inline] pub fn set_s14<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="S13"]
    #[inline] pub fn s13(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if S13 != 0"]
    #[inline] pub fn test_s13(&self) -> bool {
        self.s13() != 0
    }

    #[doc="Sets the S13 field."]
    #[inline] pub fn set_s13<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="S12"]
    #[inline] pub fn s12(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if S12 != 0"]
    #[inline] pub fn test_s12(&self) -> bool {
        self.s12() != 0
    }

    #[doc="Sets the S12 field."]
    #[inline] pub fn set_s12<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="S11"]
    #[inline] pub fn s11(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if S11 != 0"]
    #[inline] pub fn test_s11(&self) -> bool {
        self.s11() != 0
    }

    #[doc="Sets the S11 field."]
    #[inline] pub fn set_s11<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="S10"]
    #[inline] pub fn s10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if S10 != 0"]
    #[inline] pub fn test_s10(&self) -> bool {
        self.s10() != 0
    }

    #[doc="Sets the S10 field."]
    #[inline] pub fn set_s10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="S09"]
    #[inline] pub fn s09(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if S09 != 0"]
    #[inline] pub fn test_s09(&self) -> bool {
        self.s09() != 0
    }

    #[doc="Sets the S09 field."]
    #[inline] pub fn set_s09<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="S08"]
    #[inline] pub fn s08(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if S08 != 0"]
    #[inline] pub fn test_s08(&self) -> bool {
        self.s08() != 0
    }

    #[doc="Sets the S08 field."]
    #[inline] pub fn set_s08<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="S07"]
    #[inline] pub fn s07(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if S07 != 0"]
    #[inline] pub fn test_s07(&self) -> bool {
        self.s07() != 0
    }

    #[doc="Sets the S07 field."]
    #[inline] pub fn set_s07<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="S06"]
    #[inline] pub fn s06(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if S06 != 0"]
    #[inline] pub fn test_s06(&self) -> bool {
        self.s06() != 0
    }

    #[doc="Sets the S06 field."]
    #[inline] pub fn set_s06<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="S05"]
    #[inline] pub fn s05(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if S05 != 0"]
    #[inline] pub fn test_s05(&self) -> bool {
        self.s05() != 0
    }

    #[doc="Sets the S05 field."]
    #[inline] pub fn set_s05<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="S04"]
    #[inline] pub fn s04(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if S04 != 0"]
    #[inline] pub fn test_s04(&self) -> bool {
        self.s04() != 0
    }

    #[doc="Sets the S04 field."]
    #[inline] pub fn set_s04<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="S03"]
    #[inline] pub fn s03(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if S03 != 0"]
    #[inline] pub fn test_s03(&self) -> bool {
        self.s03() != 0
    }

    #[doc="Sets the S03 field."]
    #[inline] pub fn set_s03<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="S02"]
    #[inline] pub fn s02(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if S02 != 0"]
    #[inline] pub fn test_s02(&self) -> bool {
        self.s02() != 0
    }

    #[doc="Sets the S02 field."]
    #[inline] pub fn set_s02<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="S01"]
    #[inline] pub fn s01(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if S01 != 0"]
    #[inline] pub fn test_s01(&self) -> bool {
        self.s01() != 0
    }

    #[doc="Sets the S01 field."]
    #[inline] pub fn set_s01<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="S00"]
    #[inline] pub fn s00(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if S00 != 0"]
    #[inline] pub fn test_s00(&self) -> bool {
        self.s00() != 0
    }

    #[doc="Sets the S00 field."]
    #[inline] pub fn set_s00<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for RamCom1 {
    #[inline]
    fn from(other: u32) -> Self {
         RamCom1(other)
    }
}

impl ::core::fmt::Display for RamCom1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RamCom1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.s31() != 0 { try!(write!(f, " s31"))}
        if self.s30() != 0 { try!(write!(f, " s30"))}
        if self.s29() != 0 { try!(write!(f, " s29"))}
        if self.s28() != 0 { try!(write!(f, " s28"))}
        if self.s27() != 0 { try!(write!(f, " s27"))}
        if self.s26() != 0 { try!(write!(f, " s26"))}
        if self.s25() != 0 { try!(write!(f, " s25"))}
        if self.s24() != 0 { try!(write!(f, " s24"))}
        if self.s23() != 0 { try!(write!(f, " s23"))}
        if self.s22() != 0 { try!(write!(f, " s22"))}
        if self.s21() != 0 { try!(write!(f, " s21"))}
        if self.s20() != 0 { try!(write!(f, " s20"))}
        if self.s19() != 0 { try!(write!(f, " s19"))}
        if self.s18() != 0 { try!(write!(f, " s18"))}
        if self.s17() != 0 { try!(write!(f, " s17"))}
        if self.s16() != 0 { try!(write!(f, " s16"))}
        if self.s15() != 0 { try!(write!(f, " s15"))}
        if self.s14() != 0 { try!(write!(f, " s14"))}
        if self.s13() != 0 { try!(write!(f, " s13"))}
        if self.s12() != 0 { try!(write!(f, " s12"))}
        if self.s11() != 0 { try!(write!(f, " s11"))}
        if self.s10() != 0 { try!(write!(f, " s10"))}
        if self.s09() != 0 { try!(write!(f, " s09"))}
        if self.s08() != 0 { try!(write!(f, " s08"))}
        if self.s07() != 0 { try!(write!(f, " s07"))}
        if self.s06() != 0 { try!(write!(f, " s06"))}
        if self.s05() != 0 { try!(write!(f, " s05"))}
        if self.s04() != 0 { try!(write!(f, " s04"))}
        if self.s03() != 0 { try!(write!(f, " s03"))}
        if self.s02() != 0 { try!(write!(f, " s02"))}
        if self.s01() != 0 { try!(write!(f, " s01"))}
        if self.s00() != 0 { try!(write!(f, " s00"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="display memory"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RamCom2(pub u32);
impl RamCom2 {
    #[doc="S31"]
    #[inline] pub fn s31(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if S31 != 0"]
    #[inline] pub fn test_s31(&self) -> bool {
        self.s31() != 0
    }

    #[doc="Sets the S31 field."]
    #[inline] pub fn set_s31<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="S30"]
    #[inline] pub fn s30(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if S30 != 0"]
    #[inline] pub fn test_s30(&self) -> bool {
        self.s30() != 0
    }

    #[doc="Sets the S30 field."]
    #[inline] pub fn set_s30<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="S29"]
    #[inline] pub fn s29(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if S29 != 0"]
    #[inline] pub fn test_s29(&self) -> bool {
        self.s29() != 0
    }

    #[doc="Sets the S29 field."]
    #[inline] pub fn set_s29<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="S28"]
    #[inline] pub fn s28(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if S28 != 0"]
    #[inline] pub fn test_s28(&self) -> bool {
        self.s28() != 0
    }

    #[doc="Sets the S28 field."]
    #[inline] pub fn set_s28<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="S27"]
    #[inline] pub fn s27(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if S27 != 0"]
    #[inline] pub fn test_s27(&self) -> bool {
        self.s27() != 0
    }

    #[doc="Sets the S27 field."]
    #[inline] pub fn set_s27<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="S26"]
    #[inline] pub fn s26(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if S26 != 0"]
    #[inline] pub fn test_s26(&self) -> bool {
        self.s26() != 0
    }

    #[doc="Sets the S26 field."]
    #[inline] pub fn set_s26<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="S25"]
    #[inline] pub fn s25(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if S25 != 0"]
    #[inline] pub fn test_s25(&self) -> bool {
        self.s25() != 0
    }

    #[doc="Sets the S25 field."]
    #[inline] pub fn set_s25<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="S24"]
    #[inline] pub fn s24(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if S24 != 0"]
    #[inline] pub fn test_s24(&self) -> bool {
        self.s24() != 0
    }

    #[doc="Sets the S24 field."]
    #[inline] pub fn set_s24<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="S23"]
    #[inline] pub fn s23(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if S23 != 0"]
    #[inline] pub fn test_s23(&self) -> bool {
        self.s23() != 0
    }

    #[doc="Sets the S23 field."]
    #[inline] pub fn set_s23<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="S22"]
    #[inline] pub fn s22(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if S22 != 0"]
    #[inline] pub fn test_s22(&self) -> bool {
        self.s22() != 0
    }

    #[doc="Sets the S22 field."]
    #[inline] pub fn set_s22<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="S21"]
    #[inline] pub fn s21(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if S21 != 0"]
    #[inline] pub fn test_s21(&self) -> bool {
        self.s21() != 0
    }

    #[doc="Sets the S21 field."]
    #[inline] pub fn set_s21<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="S20"]
    #[inline] pub fn s20(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if S20 != 0"]
    #[inline] pub fn test_s20(&self) -> bool {
        self.s20() != 0
    }

    #[doc="Sets the S20 field."]
    #[inline] pub fn set_s20<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="S19"]
    #[inline] pub fn s19(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if S19 != 0"]
    #[inline] pub fn test_s19(&self) -> bool {
        self.s19() != 0
    }

    #[doc="Sets the S19 field."]
    #[inline] pub fn set_s19<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="S18"]
    #[inline] pub fn s18(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if S18 != 0"]
    #[inline] pub fn test_s18(&self) -> bool {
        self.s18() != 0
    }

    #[doc="Sets the S18 field."]
    #[inline] pub fn set_s18<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="S17"]
    #[inline] pub fn s17(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if S17 != 0"]
    #[inline] pub fn test_s17(&self) -> bool {
        self.s17() != 0
    }

    #[doc="Sets the S17 field."]
    #[inline] pub fn set_s17<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="S16"]
    #[inline] pub fn s16(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if S16 != 0"]
    #[inline] pub fn test_s16(&self) -> bool {
        self.s16() != 0
    }

    #[doc="Sets the S16 field."]
    #[inline] pub fn set_s16<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="S15"]
    #[inline] pub fn s15(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if S15 != 0"]
    #[inline] pub fn test_s15(&self) -> bool {
        self.s15() != 0
    }

    #[doc="Sets the S15 field."]
    #[inline] pub fn set_s15<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="S14"]
    #[inline] pub fn s14(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if S14 != 0"]
    #[inline] pub fn test_s14(&self) -> bool {
        self.s14() != 0
    }

    #[doc="Sets the S14 field."]
    #[inline] pub fn set_s14<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="S13"]
    #[inline] pub fn s13(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if S13 != 0"]
    #[inline] pub fn test_s13(&self) -> bool {
        self.s13() != 0
    }

    #[doc="Sets the S13 field."]
    #[inline] pub fn set_s13<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="S12"]
    #[inline] pub fn s12(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if S12 != 0"]
    #[inline] pub fn test_s12(&self) -> bool {
        self.s12() != 0
    }

    #[doc="Sets the S12 field."]
    #[inline] pub fn set_s12<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="S11"]
    #[inline] pub fn s11(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if S11 != 0"]
    #[inline] pub fn test_s11(&self) -> bool {
        self.s11() != 0
    }

    #[doc="Sets the S11 field."]
    #[inline] pub fn set_s11<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="S10"]
    #[inline] pub fn s10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if S10 != 0"]
    #[inline] pub fn test_s10(&self) -> bool {
        self.s10() != 0
    }

    #[doc="Sets the S10 field."]
    #[inline] pub fn set_s10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="S09"]
    #[inline] pub fn s09(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if S09 != 0"]
    #[inline] pub fn test_s09(&self) -> bool {
        self.s09() != 0
    }

    #[doc="Sets the S09 field."]
    #[inline] pub fn set_s09<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="S08"]
    #[inline] pub fn s08(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if S08 != 0"]
    #[inline] pub fn test_s08(&self) -> bool {
        self.s08() != 0
    }

    #[doc="Sets the S08 field."]
    #[inline] pub fn set_s08<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="S07"]
    #[inline] pub fn s07(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if S07 != 0"]
    #[inline] pub fn test_s07(&self) -> bool {
        self.s07() != 0
    }

    #[doc="Sets the S07 field."]
    #[inline] pub fn set_s07<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="S06"]
    #[inline] pub fn s06(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if S06 != 0"]
    #[inline] pub fn test_s06(&self) -> bool {
        self.s06() != 0
    }

    #[doc="Sets the S06 field."]
    #[inline] pub fn set_s06<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="S05"]
    #[inline] pub fn s05(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if S05 != 0"]
    #[inline] pub fn test_s05(&self) -> bool {
        self.s05() != 0
    }

    #[doc="Sets the S05 field."]
    #[inline] pub fn set_s05<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="S04"]
    #[inline] pub fn s04(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if S04 != 0"]
    #[inline] pub fn test_s04(&self) -> bool {
        self.s04() != 0
    }

    #[doc="Sets the S04 field."]
    #[inline] pub fn set_s04<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="S03"]
    #[inline] pub fn s03(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if S03 != 0"]
    #[inline] pub fn test_s03(&self) -> bool {
        self.s03() != 0
    }

    #[doc="Sets the S03 field."]
    #[inline] pub fn set_s03<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="S02"]
    #[inline] pub fn s02(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if S02 != 0"]
    #[inline] pub fn test_s02(&self) -> bool {
        self.s02() != 0
    }

    #[doc="Sets the S02 field."]
    #[inline] pub fn set_s02<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="S01"]
    #[inline] pub fn s01(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if S01 != 0"]
    #[inline] pub fn test_s01(&self) -> bool {
        self.s01() != 0
    }

    #[doc="Sets the S01 field."]
    #[inline] pub fn set_s01<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="S00"]
    #[inline] pub fn s00(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if S00 != 0"]
    #[inline] pub fn test_s00(&self) -> bool {
        self.s00() != 0
    }

    #[doc="Sets the S00 field."]
    #[inline] pub fn set_s00<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for RamCom2 {
    #[inline]
    fn from(other: u32) -> Self {
         RamCom2(other)
    }
}

impl ::core::fmt::Display for RamCom2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RamCom2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.s31() != 0 { try!(write!(f, " s31"))}
        if self.s30() != 0 { try!(write!(f, " s30"))}
        if self.s29() != 0 { try!(write!(f, " s29"))}
        if self.s28() != 0 { try!(write!(f, " s28"))}
        if self.s27() != 0 { try!(write!(f, " s27"))}
        if self.s26() != 0 { try!(write!(f, " s26"))}
        if self.s25() != 0 { try!(write!(f, " s25"))}
        if self.s24() != 0 { try!(write!(f, " s24"))}
        if self.s23() != 0 { try!(write!(f, " s23"))}
        if self.s22() != 0 { try!(write!(f, " s22"))}
        if self.s21() != 0 { try!(write!(f, " s21"))}
        if self.s20() != 0 { try!(write!(f, " s20"))}
        if self.s19() != 0 { try!(write!(f, " s19"))}
        if self.s18() != 0 { try!(write!(f, " s18"))}
        if self.s17() != 0 { try!(write!(f, " s17"))}
        if self.s16() != 0 { try!(write!(f, " s16"))}
        if self.s15() != 0 { try!(write!(f, " s15"))}
        if self.s14() != 0 { try!(write!(f, " s14"))}
        if self.s13() != 0 { try!(write!(f, " s13"))}
        if self.s12() != 0 { try!(write!(f, " s12"))}
        if self.s11() != 0 { try!(write!(f, " s11"))}
        if self.s10() != 0 { try!(write!(f, " s10"))}
        if self.s09() != 0 { try!(write!(f, " s09"))}
        if self.s08() != 0 { try!(write!(f, " s08"))}
        if self.s07() != 0 { try!(write!(f, " s07"))}
        if self.s06() != 0 { try!(write!(f, " s06"))}
        if self.s05() != 0 { try!(write!(f, " s05"))}
        if self.s04() != 0 { try!(write!(f, " s04"))}
        if self.s03() != 0 { try!(write!(f, " s03"))}
        if self.s02() != 0 { try!(write!(f, " s02"))}
        if self.s01() != 0 { try!(write!(f, " s01"))}
        if self.s00() != 0 { try!(write!(f, " s00"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="display memory"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RamCom3(pub u32);
impl RamCom3 {
    #[doc="S31"]
    #[inline] pub fn s31(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if S31 != 0"]
    #[inline] pub fn test_s31(&self) -> bool {
        self.s31() != 0
    }

    #[doc="Sets the S31 field."]
    #[inline] pub fn set_s31<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="S30"]
    #[inline] pub fn s30(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if S30 != 0"]
    #[inline] pub fn test_s30(&self) -> bool {
        self.s30() != 0
    }

    #[doc="Sets the S30 field."]
    #[inline] pub fn set_s30<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="S29"]
    #[inline] pub fn s29(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if S29 != 0"]
    #[inline] pub fn test_s29(&self) -> bool {
        self.s29() != 0
    }

    #[doc="Sets the S29 field."]
    #[inline] pub fn set_s29<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="S28"]
    #[inline] pub fn s28(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if S28 != 0"]
    #[inline] pub fn test_s28(&self) -> bool {
        self.s28() != 0
    }

    #[doc="Sets the S28 field."]
    #[inline] pub fn set_s28<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="S27"]
    #[inline] pub fn s27(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if S27 != 0"]
    #[inline] pub fn test_s27(&self) -> bool {
        self.s27() != 0
    }

    #[doc="Sets the S27 field."]
    #[inline] pub fn set_s27<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="S26"]
    #[inline] pub fn s26(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if S26 != 0"]
    #[inline] pub fn test_s26(&self) -> bool {
        self.s26() != 0
    }

    #[doc="Sets the S26 field."]
    #[inline] pub fn set_s26<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="S25"]
    #[inline] pub fn s25(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if S25 != 0"]
    #[inline] pub fn test_s25(&self) -> bool {
        self.s25() != 0
    }

    #[doc="Sets the S25 field."]
    #[inline] pub fn set_s25<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="S24"]
    #[inline] pub fn s24(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if S24 != 0"]
    #[inline] pub fn test_s24(&self) -> bool {
        self.s24() != 0
    }

    #[doc="Sets the S24 field."]
    #[inline] pub fn set_s24<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="S23"]
    #[inline] pub fn s23(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if S23 != 0"]
    #[inline] pub fn test_s23(&self) -> bool {
        self.s23() != 0
    }

    #[doc="Sets the S23 field."]
    #[inline] pub fn set_s23<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="S22"]
    #[inline] pub fn s22(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if S22 != 0"]
    #[inline] pub fn test_s22(&self) -> bool {
        self.s22() != 0
    }

    #[doc="Sets the S22 field."]
    #[inline] pub fn set_s22<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="S21"]
    #[inline] pub fn s21(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if S21 != 0"]
    #[inline] pub fn test_s21(&self) -> bool {
        self.s21() != 0
    }

    #[doc="Sets the S21 field."]
    #[inline] pub fn set_s21<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="S20"]
    #[inline] pub fn s20(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if S20 != 0"]
    #[inline] pub fn test_s20(&self) -> bool {
        self.s20() != 0
    }

    #[doc="Sets the S20 field."]
    #[inline] pub fn set_s20<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="S19"]
    #[inline] pub fn s19(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if S19 != 0"]
    #[inline] pub fn test_s19(&self) -> bool {
        self.s19() != 0
    }

    #[doc="Sets the S19 field."]
    #[inline] pub fn set_s19<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="S18"]
    #[inline] pub fn s18(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if S18 != 0"]
    #[inline] pub fn test_s18(&self) -> bool {
        self.s18() != 0
    }

    #[doc="Sets the S18 field."]
    #[inline] pub fn set_s18<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="S17"]
    #[inline] pub fn s17(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if S17 != 0"]
    #[inline] pub fn test_s17(&self) -> bool {
        self.s17() != 0
    }

    #[doc="Sets the S17 field."]
    #[inline] pub fn set_s17<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="S16"]
    #[inline] pub fn s16(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if S16 != 0"]
    #[inline] pub fn test_s16(&self) -> bool {
        self.s16() != 0
    }

    #[doc="Sets the S16 field."]
    #[inline] pub fn set_s16<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="S15"]
    #[inline] pub fn s15(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if S15 != 0"]
    #[inline] pub fn test_s15(&self) -> bool {
        self.s15() != 0
    }

    #[doc="Sets the S15 field."]
    #[inline] pub fn set_s15<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="S14"]
    #[inline] pub fn s14(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if S14 != 0"]
    #[inline] pub fn test_s14(&self) -> bool {
        self.s14() != 0
    }

    #[doc="Sets the S14 field."]
    #[inline] pub fn set_s14<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="S13"]
    #[inline] pub fn s13(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if S13 != 0"]
    #[inline] pub fn test_s13(&self) -> bool {
        self.s13() != 0
    }

    #[doc="Sets the S13 field."]
    #[inline] pub fn set_s13<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="S12"]
    #[inline] pub fn s12(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if S12 != 0"]
    #[inline] pub fn test_s12(&self) -> bool {
        self.s12() != 0
    }

    #[doc="Sets the S12 field."]
    #[inline] pub fn set_s12<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="S11"]
    #[inline] pub fn s11(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if S11 != 0"]
    #[inline] pub fn test_s11(&self) -> bool {
        self.s11() != 0
    }

    #[doc="Sets the S11 field."]
    #[inline] pub fn set_s11<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="S10"]
    #[inline] pub fn s10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if S10 != 0"]
    #[inline] pub fn test_s10(&self) -> bool {
        self.s10() != 0
    }

    #[doc="Sets the S10 field."]
    #[inline] pub fn set_s10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="S09"]
    #[inline] pub fn s09(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if S09 != 0"]
    #[inline] pub fn test_s09(&self) -> bool {
        self.s09() != 0
    }

    #[doc="Sets the S09 field."]
    #[inline] pub fn set_s09<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="S08"]
    #[inline] pub fn s08(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if S08 != 0"]
    #[inline] pub fn test_s08(&self) -> bool {
        self.s08() != 0
    }

    #[doc="Sets the S08 field."]
    #[inline] pub fn set_s08<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="S07"]
    #[inline] pub fn s07(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if S07 != 0"]
    #[inline] pub fn test_s07(&self) -> bool {
        self.s07() != 0
    }

    #[doc="Sets the S07 field."]
    #[inline] pub fn set_s07<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="S06"]
    #[inline] pub fn s06(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if S06 != 0"]
    #[inline] pub fn test_s06(&self) -> bool {
        self.s06() != 0
    }

    #[doc="Sets the S06 field."]
    #[inline] pub fn set_s06<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="S05"]
    #[inline] pub fn s05(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if S05 != 0"]
    #[inline] pub fn test_s05(&self) -> bool {
        self.s05() != 0
    }

    #[doc="Sets the S05 field."]
    #[inline] pub fn set_s05<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="S04"]
    #[inline] pub fn s04(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if S04 != 0"]
    #[inline] pub fn test_s04(&self) -> bool {
        self.s04() != 0
    }

    #[doc="Sets the S04 field."]
    #[inline] pub fn set_s04<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="S03"]
    #[inline] pub fn s03(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if S03 != 0"]
    #[inline] pub fn test_s03(&self) -> bool {
        self.s03() != 0
    }

    #[doc="Sets the S03 field."]
    #[inline] pub fn set_s03<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="S02"]
    #[inline] pub fn s02(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if S02 != 0"]
    #[inline] pub fn test_s02(&self) -> bool {
        self.s02() != 0
    }

    #[doc="Sets the S02 field."]
    #[inline] pub fn set_s02<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="S01"]
    #[inline] pub fn s01(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if S01 != 0"]
    #[inline] pub fn test_s01(&self) -> bool {
        self.s01() != 0
    }

    #[doc="Sets the S01 field."]
    #[inline] pub fn set_s01<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="S00"]
    #[inline] pub fn s00(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if S00 != 0"]
    #[inline] pub fn test_s00(&self) -> bool {
        self.s00() != 0
    }

    #[doc="Sets the S00 field."]
    #[inline] pub fn set_s00<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for RamCom3 {
    #[inline]
    fn from(other: u32) -> Self {
         RamCom3(other)
    }
}

impl ::core::fmt::Display for RamCom3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RamCom3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.s31() != 0 { try!(write!(f, " s31"))}
        if self.s30() != 0 { try!(write!(f, " s30"))}
        if self.s29() != 0 { try!(write!(f, " s29"))}
        if self.s28() != 0 { try!(write!(f, " s28"))}
        if self.s27() != 0 { try!(write!(f, " s27"))}
        if self.s26() != 0 { try!(write!(f, " s26"))}
        if self.s25() != 0 { try!(write!(f, " s25"))}
        if self.s24() != 0 { try!(write!(f, " s24"))}
        if self.s23() != 0 { try!(write!(f, " s23"))}
        if self.s22() != 0 { try!(write!(f, " s22"))}
        if self.s21() != 0 { try!(write!(f, " s21"))}
        if self.s20() != 0 { try!(write!(f, " s20"))}
        if self.s19() != 0 { try!(write!(f, " s19"))}
        if self.s18() != 0 { try!(write!(f, " s18"))}
        if self.s17() != 0 { try!(write!(f, " s17"))}
        if self.s16() != 0 { try!(write!(f, " s16"))}
        if self.s15() != 0 { try!(write!(f, " s15"))}
        if self.s14() != 0 { try!(write!(f, " s14"))}
        if self.s13() != 0 { try!(write!(f, " s13"))}
        if self.s12() != 0 { try!(write!(f, " s12"))}
        if self.s11() != 0 { try!(write!(f, " s11"))}
        if self.s10() != 0 { try!(write!(f, " s10"))}
        if self.s09() != 0 { try!(write!(f, " s09"))}
        if self.s08() != 0 { try!(write!(f, " s08"))}
        if self.s07() != 0 { try!(write!(f, " s07"))}
        if self.s06() != 0 { try!(write!(f, " s06"))}
        if self.s05() != 0 { try!(write!(f, " s05"))}
        if self.s04() != 0 { try!(write!(f, " s04"))}
        if self.s03() != 0 { try!(write!(f, " s03"))}
        if self.s02() != 0 { try!(write!(f, " s02"))}
        if self.s01() != 0 { try!(write!(f, " s01"))}
        if self.s00() != 0 { try!(write!(f, " s00"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="display memory"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RamCom4(pub u32);
impl RamCom4 {
    #[doc="S31"]
    #[inline] pub fn s31(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if S31 != 0"]
    #[inline] pub fn test_s31(&self) -> bool {
        self.s31() != 0
    }

    #[doc="Sets the S31 field."]
    #[inline] pub fn set_s31<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="S30"]
    #[inline] pub fn s30(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if S30 != 0"]
    #[inline] pub fn test_s30(&self) -> bool {
        self.s30() != 0
    }

    #[doc="Sets the S30 field."]
    #[inline] pub fn set_s30<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="S29"]
    #[inline] pub fn s29(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if S29 != 0"]
    #[inline] pub fn test_s29(&self) -> bool {
        self.s29() != 0
    }

    #[doc="Sets the S29 field."]
    #[inline] pub fn set_s29<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="S28"]
    #[inline] pub fn s28(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if S28 != 0"]
    #[inline] pub fn test_s28(&self) -> bool {
        self.s28() != 0
    }

    #[doc="Sets the S28 field."]
    #[inline] pub fn set_s28<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="S27"]
    #[inline] pub fn s27(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if S27 != 0"]
    #[inline] pub fn test_s27(&self) -> bool {
        self.s27() != 0
    }

    #[doc="Sets the S27 field."]
    #[inline] pub fn set_s27<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="S26"]
    #[inline] pub fn s26(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if S26 != 0"]
    #[inline] pub fn test_s26(&self) -> bool {
        self.s26() != 0
    }

    #[doc="Sets the S26 field."]
    #[inline] pub fn set_s26<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="S25"]
    #[inline] pub fn s25(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if S25 != 0"]
    #[inline] pub fn test_s25(&self) -> bool {
        self.s25() != 0
    }

    #[doc="Sets the S25 field."]
    #[inline] pub fn set_s25<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="S24"]
    #[inline] pub fn s24(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if S24 != 0"]
    #[inline] pub fn test_s24(&self) -> bool {
        self.s24() != 0
    }

    #[doc="Sets the S24 field."]
    #[inline] pub fn set_s24<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="S23"]
    #[inline] pub fn s23(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if S23 != 0"]
    #[inline] pub fn test_s23(&self) -> bool {
        self.s23() != 0
    }

    #[doc="Sets the S23 field."]
    #[inline] pub fn set_s23<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="S22"]
    #[inline] pub fn s22(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if S22 != 0"]
    #[inline] pub fn test_s22(&self) -> bool {
        self.s22() != 0
    }

    #[doc="Sets the S22 field."]
    #[inline] pub fn set_s22<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="S21"]
    #[inline] pub fn s21(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if S21 != 0"]
    #[inline] pub fn test_s21(&self) -> bool {
        self.s21() != 0
    }

    #[doc="Sets the S21 field."]
    #[inline] pub fn set_s21<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="S20"]
    #[inline] pub fn s20(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if S20 != 0"]
    #[inline] pub fn test_s20(&self) -> bool {
        self.s20() != 0
    }

    #[doc="Sets the S20 field."]
    #[inline] pub fn set_s20<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="S19"]
    #[inline] pub fn s19(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if S19 != 0"]
    #[inline] pub fn test_s19(&self) -> bool {
        self.s19() != 0
    }

    #[doc="Sets the S19 field."]
    #[inline] pub fn set_s19<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="S18"]
    #[inline] pub fn s18(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if S18 != 0"]
    #[inline] pub fn test_s18(&self) -> bool {
        self.s18() != 0
    }

    #[doc="Sets the S18 field."]
    #[inline] pub fn set_s18<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="S17"]
    #[inline] pub fn s17(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if S17 != 0"]
    #[inline] pub fn test_s17(&self) -> bool {
        self.s17() != 0
    }

    #[doc="Sets the S17 field."]
    #[inline] pub fn set_s17<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="S16"]
    #[inline] pub fn s16(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if S16 != 0"]
    #[inline] pub fn test_s16(&self) -> bool {
        self.s16() != 0
    }

    #[doc="Sets the S16 field."]
    #[inline] pub fn set_s16<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="S15"]
    #[inline] pub fn s15(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if S15 != 0"]
    #[inline] pub fn test_s15(&self) -> bool {
        self.s15() != 0
    }

    #[doc="Sets the S15 field."]
    #[inline] pub fn set_s15<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="S14"]
    #[inline] pub fn s14(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if S14 != 0"]
    #[inline] pub fn test_s14(&self) -> bool {
        self.s14() != 0
    }

    #[doc="Sets the S14 field."]
    #[inline] pub fn set_s14<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="S13"]
    #[inline] pub fn s13(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if S13 != 0"]
    #[inline] pub fn test_s13(&self) -> bool {
        self.s13() != 0
    }

    #[doc="Sets the S13 field."]
    #[inline] pub fn set_s13<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="S12"]
    #[inline] pub fn s12(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if S12 != 0"]
    #[inline] pub fn test_s12(&self) -> bool {
        self.s12() != 0
    }

    #[doc="Sets the S12 field."]
    #[inline] pub fn set_s12<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="S11"]
    #[inline] pub fn s11(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if S11 != 0"]
    #[inline] pub fn test_s11(&self) -> bool {
        self.s11() != 0
    }

    #[doc="Sets the S11 field."]
    #[inline] pub fn set_s11<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="S10"]
    #[inline] pub fn s10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if S10 != 0"]
    #[inline] pub fn test_s10(&self) -> bool {
        self.s10() != 0
    }

    #[doc="Sets the S10 field."]
    #[inline] pub fn set_s10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="S09"]
    #[inline] pub fn s09(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if S09 != 0"]
    #[inline] pub fn test_s09(&self) -> bool {
        self.s09() != 0
    }

    #[doc="Sets the S09 field."]
    #[inline] pub fn set_s09<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="S08"]
    #[inline] pub fn s08(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if S08 != 0"]
    #[inline] pub fn test_s08(&self) -> bool {
        self.s08() != 0
    }

    #[doc="Sets the S08 field."]
    #[inline] pub fn set_s08<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="S07"]
    #[inline] pub fn s07(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if S07 != 0"]
    #[inline] pub fn test_s07(&self) -> bool {
        self.s07() != 0
    }

    #[doc="Sets the S07 field."]
    #[inline] pub fn set_s07<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="S06"]
    #[inline] pub fn s06(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if S06 != 0"]
    #[inline] pub fn test_s06(&self) -> bool {
        self.s06() != 0
    }

    #[doc="Sets the S06 field."]
    #[inline] pub fn set_s06<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="S05"]
    #[inline] pub fn s05(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if S05 != 0"]
    #[inline] pub fn test_s05(&self) -> bool {
        self.s05() != 0
    }

    #[doc="Sets the S05 field."]
    #[inline] pub fn set_s05<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="S04"]
    #[inline] pub fn s04(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if S04 != 0"]
    #[inline] pub fn test_s04(&self) -> bool {
        self.s04() != 0
    }

    #[doc="Sets the S04 field."]
    #[inline] pub fn set_s04<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="S03"]
    #[inline] pub fn s03(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if S03 != 0"]
    #[inline] pub fn test_s03(&self) -> bool {
        self.s03() != 0
    }

    #[doc="Sets the S03 field."]
    #[inline] pub fn set_s03<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="S02"]
    #[inline] pub fn s02(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if S02 != 0"]
    #[inline] pub fn test_s02(&self) -> bool {
        self.s02() != 0
    }

    #[doc="Sets the S02 field."]
    #[inline] pub fn set_s02<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="S01"]
    #[inline] pub fn s01(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if S01 != 0"]
    #[inline] pub fn test_s01(&self) -> bool {
        self.s01() != 0
    }

    #[doc="Sets the S01 field."]
    #[inline] pub fn set_s01<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="S00"]
    #[inline] pub fn s00(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if S00 != 0"]
    #[inline] pub fn test_s00(&self) -> bool {
        self.s00() != 0
    }

    #[doc="Sets the S00 field."]
    #[inline] pub fn set_s00<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for RamCom4 {
    #[inline]
    fn from(other: u32) -> Self {
         RamCom4(other)
    }
}

impl ::core::fmt::Display for RamCom4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RamCom4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.s31() != 0 { try!(write!(f, " s31"))}
        if self.s30() != 0 { try!(write!(f, " s30"))}
        if self.s29() != 0 { try!(write!(f, " s29"))}
        if self.s28() != 0 { try!(write!(f, " s28"))}
        if self.s27() != 0 { try!(write!(f, " s27"))}
        if self.s26() != 0 { try!(write!(f, " s26"))}
        if self.s25() != 0 { try!(write!(f, " s25"))}
        if self.s24() != 0 { try!(write!(f, " s24"))}
        if self.s23() != 0 { try!(write!(f, " s23"))}
        if self.s22() != 0 { try!(write!(f, " s22"))}
        if self.s21() != 0 { try!(write!(f, " s21"))}
        if self.s20() != 0 { try!(write!(f, " s20"))}
        if self.s19() != 0 { try!(write!(f, " s19"))}
        if self.s18() != 0 { try!(write!(f, " s18"))}
        if self.s17() != 0 { try!(write!(f, " s17"))}
        if self.s16() != 0 { try!(write!(f, " s16"))}
        if self.s15() != 0 { try!(write!(f, " s15"))}
        if self.s14() != 0 { try!(write!(f, " s14"))}
        if self.s13() != 0 { try!(write!(f, " s13"))}
        if self.s12() != 0 { try!(write!(f, " s12"))}
        if self.s11() != 0 { try!(write!(f, " s11"))}
        if self.s10() != 0 { try!(write!(f, " s10"))}
        if self.s09() != 0 { try!(write!(f, " s09"))}
        if self.s08() != 0 { try!(write!(f, " s08"))}
        if self.s07() != 0 { try!(write!(f, " s07"))}
        if self.s06() != 0 { try!(write!(f, " s06"))}
        if self.s05() != 0 { try!(write!(f, " s05"))}
        if self.s04() != 0 { try!(write!(f, " s04"))}
        if self.s03() != 0 { try!(write!(f, " s03"))}
        if self.s02() != 0 { try!(write!(f, " s02"))}
        if self.s01() != 0 { try!(write!(f, " s01"))}
        if self.s00() != 0 { try!(write!(f, " s00"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="display memory"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RamCom5(pub u32);
impl RamCom5 {
    #[doc="S31"]
    #[inline] pub fn s31(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if S31 != 0"]
    #[inline] pub fn test_s31(&self) -> bool {
        self.s31() != 0
    }

    #[doc="Sets the S31 field."]
    #[inline] pub fn set_s31<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="S30"]
    #[inline] pub fn s30(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if S30 != 0"]
    #[inline] pub fn test_s30(&self) -> bool {
        self.s30() != 0
    }

    #[doc="Sets the S30 field."]
    #[inline] pub fn set_s30<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="S29"]
    #[inline] pub fn s29(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if S29 != 0"]
    #[inline] pub fn test_s29(&self) -> bool {
        self.s29() != 0
    }

    #[doc="Sets the S29 field."]
    #[inline] pub fn set_s29<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="S28"]
    #[inline] pub fn s28(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if S28 != 0"]
    #[inline] pub fn test_s28(&self) -> bool {
        self.s28() != 0
    }

    #[doc="Sets the S28 field."]
    #[inline] pub fn set_s28<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="S27"]
    #[inline] pub fn s27(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if S27 != 0"]
    #[inline] pub fn test_s27(&self) -> bool {
        self.s27() != 0
    }

    #[doc="Sets the S27 field."]
    #[inline] pub fn set_s27<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="S26"]
    #[inline] pub fn s26(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if S26 != 0"]
    #[inline] pub fn test_s26(&self) -> bool {
        self.s26() != 0
    }

    #[doc="Sets the S26 field."]
    #[inline] pub fn set_s26<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="S25"]
    #[inline] pub fn s25(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if S25 != 0"]
    #[inline] pub fn test_s25(&self) -> bool {
        self.s25() != 0
    }

    #[doc="Sets the S25 field."]
    #[inline] pub fn set_s25<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="S24"]
    #[inline] pub fn s24(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if S24 != 0"]
    #[inline] pub fn test_s24(&self) -> bool {
        self.s24() != 0
    }

    #[doc="Sets the S24 field."]
    #[inline] pub fn set_s24<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="S23"]
    #[inline] pub fn s23(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if S23 != 0"]
    #[inline] pub fn test_s23(&self) -> bool {
        self.s23() != 0
    }

    #[doc="Sets the S23 field."]
    #[inline] pub fn set_s23<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="S22"]
    #[inline] pub fn s22(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if S22 != 0"]
    #[inline] pub fn test_s22(&self) -> bool {
        self.s22() != 0
    }

    #[doc="Sets the S22 field."]
    #[inline] pub fn set_s22<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="S21"]
    #[inline] pub fn s21(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if S21 != 0"]
    #[inline] pub fn test_s21(&self) -> bool {
        self.s21() != 0
    }

    #[doc="Sets the S21 field."]
    #[inline] pub fn set_s21<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="S20"]
    #[inline] pub fn s20(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if S20 != 0"]
    #[inline] pub fn test_s20(&self) -> bool {
        self.s20() != 0
    }

    #[doc="Sets the S20 field."]
    #[inline] pub fn set_s20<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="S19"]
    #[inline] pub fn s19(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if S19 != 0"]
    #[inline] pub fn test_s19(&self) -> bool {
        self.s19() != 0
    }

    #[doc="Sets the S19 field."]
    #[inline] pub fn set_s19<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="S18"]
    #[inline] pub fn s18(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if S18 != 0"]
    #[inline] pub fn test_s18(&self) -> bool {
        self.s18() != 0
    }

    #[doc="Sets the S18 field."]
    #[inline] pub fn set_s18<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="S17"]
    #[inline] pub fn s17(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if S17 != 0"]
    #[inline] pub fn test_s17(&self) -> bool {
        self.s17() != 0
    }

    #[doc="Sets the S17 field."]
    #[inline] pub fn set_s17<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="S16"]
    #[inline] pub fn s16(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if S16 != 0"]
    #[inline] pub fn test_s16(&self) -> bool {
        self.s16() != 0
    }

    #[doc="Sets the S16 field."]
    #[inline] pub fn set_s16<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="S15"]
    #[inline] pub fn s15(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if S15 != 0"]
    #[inline] pub fn test_s15(&self) -> bool {
        self.s15() != 0
    }

    #[doc="Sets the S15 field."]
    #[inline] pub fn set_s15<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="S14"]
    #[inline] pub fn s14(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if S14 != 0"]
    #[inline] pub fn test_s14(&self) -> bool {
        self.s14() != 0
    }

    #[doc="Sets the S14 field."]
    #[inline] pub fn set_s14<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="S13"]
    #[inline] pub fn s13(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if S13 != 0"]
    #[inline] pub fn test_s13(&self) -> bool {
        self.s13() != 0
    }

    #[doc="Sets the S13 field."]
    #[inline] pub fn set_s13<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="S12"]
    #[inline] pub fn s12(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if S12 != 0"]
    #[inline] pub fn test_s12(&self) -> bool {
        self.s12() != 0
    }

    #[doc="Sets the S12 field."]
    #[inline] pub fn set_s12<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="S11"]
    #[inline] pub fn s11(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if S11 != 0"]
    #[inline] pub fn test_s11(&self) -> bool {
        self.s11() != 0
    }

    #[doc="Sets the S11 field."]
    #[inline] pub fn set_s11<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="S10"]
    #[inline] pub fn s10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if S10 != 0"]
    #[inline] pub fn test_s10(&self) -> bool {
        self.s10() != 0
    }

    #[doc="Sets the S10 field."]
    #[inline] pub fn set_s10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="S09"]
    #[inline] pub fn s09(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if S09 != 0"]
    #[inline] pub fn test_s09(&self) -> bool {
        self.s09() != 0
    }

    #[doc="Sets the S09 field."]
    #[inline] pub fn set_s09<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="S08"]
    #[inline] pub fn s08(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if S08 != 0"]
    #[inline] pub fn test_s08(&self) -> bool {
        self.s08() != 0
    }

    #[doc="Sets the S08 field."]
    #[inline] pub fn set_s08<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="S07"]
    #[inline] pub fn s07(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if S07 != 0"]
    #[inline] pub fn test_s07(&self) -> bool {
        self.s07() != 0
    }

    #[doc="Sets the S07 field."]
    #[inline] pub fn set_s07<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="S06"]
    #[inline] pub fn s06(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if S06 != 0"]
    #[inline] pub fn test_s06(&self) -> bool {
        self.s06() != 0
    }

    #[doc="Sets the S06 field."]
    #[inline] pub fn set_s06<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="S05"]
    #[inline] pub fn s05(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if S05 != 0"]
    #[inline] pub fn test_s05(&self) -> bool {
        self.s05() != 0
    }

    #[doc="Sets the S05 field."]
    #[inline] pub fn set_s05<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="S04"]
    #[inline] pub fn s04(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if S04 != 0"]
    #[inline] pub fn test_s04(&self) -> bool {
        self.s04() != 0
    }

    #[doc="Sets the S04 field."]
    #[inline] pub fn set_s04<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="S03"]
    #[inline] pub fn s03(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if S03 != 0"]
    #[inline] pub fn test_s03(&self) -> bool {
        self.s03() != 0
    }

    #[doc="Sets the S03 field."]
    #[inline] pub fn set_s03<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="S02"]
    #[inline] pub fn s02(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if S02 != 0"]
    #[inline] pub fn test_s02(&self) -> bool {
        self.s02() != 0
    }

    #[doc="Sets the S02 field."]
    #[inline] pub fn set_s02<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="S01"]
    #[inline] pub fn s01(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if S01 != 0"]
    #[inline] pub fn test_s01(&self) -> bool {
        self.s01() != 0
    }

    #[doc="Sets the S01 field."]
    #[inline] pub fn set_s01<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="S00"]
    #[inline] pub fn s00(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if S00 != 0"]
    #[inline] pub fn test_s00(&self) -> bool {
        self.s00() != 0
    }

    #[doc="Sets the S00 field."]
    #[inline] pub fn set_s00<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for RamCom5 {
    #[inline]
    fn from(other: u32) -> Self {
         RamCom5(other)
    }
}

impl ::core::fmt::Display for RamCom5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RamCom5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.s31() != 0 { try!(write!(f, " s31"))}
        if self.s30() != 0 { try!(write!(f, " s30"))}
        if self.s29() != 0 { try!(write!(f, " s29"))}
        if self.s28() != 0 { try!(write!(f, " s28"))}
        if self.s27() != 0 { try!(write!(f, " s27"))}
        if self.s26() != 0 { try!(write!(f, " s26"))}
        if self.s25() != 0 { try!(write!(f, " s25"))}
        if self.s24() != 0 { try!(write!(f, " s24"))}
        if self.s23() != 0 { try!(write!(f, " s23"))}
        if self.s22() != 0 { try!(write!(f, " s22"))}
        if self.s21() != 0 { try!(write!(f, " s21"))}
        if self.s20() != 0 { try!(write!(f, " s20"))}
        if self.s19() != 0 { try!(write!(f, " s19"))}
        if self.s18() != 0 { try!(write!(f, " s18"))}
        if self.s17() != 0 { try!(write!(f, " s17"))}
        if self.s16() != 0 { try!(write!(f, " s16"))}
        if self.s15() != 0 { try!(write!(f, " s15"))}
        if self.s14() != 0 { try!(write!(f, " s14"))}
        if self.s13() != 0 { try!(write!(f, " s13"))}
        if self.s12() != 0 { try!(write!(f, " s12"))}
        if self.s11() != 0 { try!(write!(f, " s11"))}
        if self.s10() != 0 { try!(write!(f, " s10"))}
        if self.s09() != 0 { try!(write!(f, " s09"))}
        if self.s08() != 0 { try!(write!(f, " s08"))}
        if self.s07() != 0 { try!(write!(f, " s07"))}
        if self.s06() != 0 { try!(write!(f, " s06"))}
        if self.s05() != 0 { try!(write!(f, " s05"))}
        if self.s04() != 0 { try!(write!(f, " s04"))}
        if self.s03() != 0 { try!(write!(f, " s03"))}
        if self.s02() != 0 { try!(write!(f, " s02"))}
        if self.s01() != 0 { try!(write!(f, " s01"))}
        if self.s00() != 0 { try!(write!(f, " s00"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="display memory"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RamCom6(pub u32);
impl RamCom6 {
    #[doc="S31"]
    #[inline] pub fn s31(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if S31 != 0"]
    #[inline] pub fn test_s31(&self) -> bool {
        self.s31() != 0
    }

    #[doc="Sets the S31 field."]
    #[inline] pub fn set_s31<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="S30"]
    #[inline] pub fn s30(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if S30 != 0"]
    #[inline] pub fn test_s30(&self) -> bool {
        self.s30() != 0
    }

    #[doc="Sets the S30 field."]
    #[inline] pub fn set_s30<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="S29"]
    #[inline] pub fn s29(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if S29 != 0"]
    #[inline] pub fn test_s29(&self) -> bool {
        self.s29() != 0
    }

    #[doc="Sets the S29 field."]
    #[inline] pub fn set_s29<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="S28"]
    #[inline] pub fn s28(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if S28 != 0"]
    #[inline] pub fn test_s28(&self) -> bool {
        self.s28() != 0
    }

    #[doc="Sets the S28 field."]
    #[inline] pub fn set_s28<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="S27"]
    #[inline] pub fn s27(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if S27 != 0"]
    #[inline] pub fn test_s27(&self) -> bool {
        self.s27() != 0
    }

    #[doc="Sets the S27 field."]
    #[inline] pub fn set_s27<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="S26"]
    #[inline] pub fn s26(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if S26 != 0"]
    #[inline] pub fn test_s26(&self) -> bool {
        self.s26() != 0
    }

    #[doc="Sets the S26 field."]
    #[inline] pub fn set_s26<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="S25"]
    #[inline] pub fn s25(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if S25 != 0"]
    #[inline] pub fn test_s25(&self) -> bool {
        self.s25() != 0
    }

    #[doc="Sets the S25 field."]
    #[inline] pub fn set_s25<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="S24"]
    #[inline] pub fn s24(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if S24 != 0"]
    #[inline] pub fn test_s24(&self) -> bool {
        self.s24() != 0
    }

    #[doc="Sets the S24 field."]
    #[inline] pub fn set_s24<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="S23"]
    #[inline] pub fn s23(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if S23 != 0"]
    #[inline] pub fn test_s23(&self) -> bool {
        self.s23() != 0
    }

    #[doc="Sets the S23 field."]
    #[inline] pub fn set_s23<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="S22"]
    #[inline] pub fn s22(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if S22 != 0"]
    #[inline] pub fn test_s22(&self) -> bool {
        self.s22() != 0
    }

    #[doc="Sets the S22 field."]
    #[inline] pub fn set_s22<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="S21"]
    #[inline] pub fn s21(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if S21 != 0"]
    #[inline] pub fn test_s21(&self) -> bool {
        self.s21() != 0
    }

    #[doc="Sets the S21 field."]
    #[inline] pub fn set_s21<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="S20"]
    #[inline] pub fn s20(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if S20 != 0"]
    #[inline] pub fn test_s20(&self) -> bool {
        self.s20() != 0
    }

    #[doc="Sets the S20 field."]
    #[inline] pub fn set_s20<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="S19"]
    #[inline] pub fn s19(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if S19 != 0"]
    #[inline] pub fn test_s19(&self) -> bool {
        self.s19() != 0
    }

    #[doc="Sets the S19 field."]
    #[inline] pub fn set_s19<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="S18"]
    #[inline] pub fn s18(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if S18 != 0"]
    #[inline] pub fn test_s18(&self) -> bool {
        self.s18() != 0
    }

    #[doc="Sets the S18 field."]
    #[inline] pub fn set_s18<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="S17"]
    #[inline] pub fn s17(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if S17 != 0"]
    #[inline] pub fn test_s17(&self) -> bool {
        self.s17() != 0
    }

    #[doc="Sets the S17 field."]
    #[inline] pub fn set_s17<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="S16"]
    #[inline] pub fn s16(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if S16 != 0"]
    #[inline] pub fn test_s16(&self) -> bool {
        self.s16() != 0
    }

    #[doc="Sets the S16 field."]
    #[inline] pub fn set_s16<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="S15"]
    #[inline] pub fn s15(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if S15 != 0"]
    #[inline] pub fn test_s15(&self) -> bool {
        self.s15() != 0
    }

    #[doc="Sets the S15 field."]
    #[inline] pub fn set_s15<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="S14"]
    #[inline] pub fn s14(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if S14 != 0"]
    #[inline] pub fn test_s14(&self) -> bool {
        self.s14() != 0
    }

    #[doc="Sets the S14 field."]
    #[inline] pub fn set_s14<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="S13"]
    #[inline] pub fn s13(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if S13 != 0"]
    #[inline] pub fn test_s13(&self) -> bool {
        self.s13() != 0
    }

    #[doc="Sets the S13 field."]
    #[inline] pub fn set_s13<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="S12"]
    #[inline] pub fn s12(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if S12 != 0"]
    #[inline] pub fn test_s12(&self) -> bool {
        self.s12() != 0
    }

    #[doc="Sets the S12 field."]
    #[inline] pub fn set_s12<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="S11"]
    #[inline] pub fn s11(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if S11 != 0"]
    #[inline] pub fn test_s11(&self) -> bool {
        self.s11() != 0
    }

    #[doc="Sets the S11 field."]
    #[inline] pub fn set_s11<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="S10"]
    #[inline] pub fn s10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if S10 != 0"]
    #[inline] pub fn test_s10(&self) -> bool {
        self.s10() != 0
    }

    #[doc="Sets the S10 field."]
    #[inline] pub fn set_s10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="S09"]
    #[inline] pub fn s09(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if S09 != 0"]
    #[inline] pub fn test_s09(&self) -> bool {
        self.s09() != 0
    }

    #[doc="Sets the S09 field."]
    #[inline] pub fn set_s09<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="S08"]
    #[inline] pub fn s08(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if S08 != 0"]
    #[inline] pub fn test_s08(&self) -> bool {
        self.s08() != 0
    }

    #[doc="Sets the S08 field."]
    #[inline] pub fn set_s08<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="S07"]
    #[inline] pub fn s07(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if S07 != 0"]
    #[inline] pub fn test_s07(&self) -> bool {
        self.s07() != 0
    }

    #[doc="Sets the S07 field."]
    #[inline] pub fn set_s07<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="S06"]
    #[inline] pub fn s06(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if S06 != 0"]
    #[inline] pub fn test_s06(&self) -> bool {
        self.s06() != 0
    }

    #[doc="Sets the S06 field."]
    #[inline] pub fn set_s06<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="S05"]
    #[inline] pub fn s05(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if S05 != 0"]
    #[inline] pub fn test_s05(&self) -> bool {
        self.s05() != 0
    }

    #[doc="Sets the S05 field."]
    #[inline] pub fn set_s05<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="S04"]
    #[inline] pub fn s04(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if S04 != 0"]
    #[inline] pub fn test_s04(&self) -> bool {
        self.s04() != 0
    }

    #[doc="Sets the S04 field."]
    #[inline] pub fn set_s04<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="S03"]
    #[inline] pub fn s03(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if S03 != 0"]
    #[inline] pub fn test_s03(&self) -> bool {
        self.s03() != 0
    }

    #[doc="Sets the S03 field."]
    #[inline] pub fn set_s03<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="S02"]
    #[inline] pub fn s02(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if S02 != 0"]
    #[inline] pub fn test_s02(&self) -> bool {
        self.s02() != 0
    }

    #[doc="Sets the S02 field."]
    #[inline] pub fn set_s02<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="S01"]
    #[inline] pub fn s01(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if S01 != 0"]
    #[inline] pub fn test_s01(&self) -> bool {
        self.s01() != 0
    }

    #[doc="Sets the S01 field."]
    #[inline] pub fn set_s01<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="S00"]
    #[inline] pub fn s00(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if S00 != 0"]
    #[inline] pub fn test_s00(&self) -> bool {
        self.s00() != 0
    }

    #[doc="Sets the S00 field."]
    #[inline] pub fn set_s00<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for RamCom6 {
    #[inline]
    fn from(other: u32) -> Self {
         RamCom6(other)
    }
}

impl ::core::fmt::Display for RamCom6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RamCom6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.s31() != 0 { try!(write!(f, " s31"))}
        if self.s30() != 0 { try!(write!(f, " s30"))}
        if self.s29() != 0 { try!(write!(f, " s29"))}
        if self.s28() != 0 { try!(write!(f, " s28"))}
        if self.s27() != 0 { try!(write!(f, " s27"))}
        if self.s26() != 0 { try!(write!(f, " s26"))}
        if self.s25() != 0 { try!(write!(f, " s25"))}
        if self.s24() != 0 { try!(write!(f, " s24"))}
        if self.s23() != 0 { try!(write!(f, " s23"))}
        if self.s22() != 0 { try!(write!(f, " s22"))}
        if self.s21() != 0 { try!(write!(f, " s21"))}
        if self.s20() != 0 { try!(write!(f, " s20"))}
        if self.s19() != 0 { try!(write!(f, " s19"))}
        if self.s18() != 0 { try!(write!(f, " s18"))}
        if self.s17() != 0 { try!(write!(f, " s17"))}
        if self.s16() != 0 { try!(write!(f, " s16"))}
        if self.s15() != 0 { try!(write!(f, " s15"))}
        if self.s14() != 0 { try!(write!(f, " s14"))}
        if self.s13() != 0 { try!(write!(f, " s13"))}
        if self.s12() != 0 { try!(write!(f, " s12"))}
        if self.s11() != 0 { try!(write!(f, " s11"))}
        if self.s10() != 0 { try!(write!(f, " s10"))}
        if self.s09() != 0 { try!(write!(f, " s09"))}
        if self.s08() != 0 { try!(write!(f, " s08"))}
        if self.s07() != 0 { try!(write!(f, " s07"))}
        if self.s06() != 0 { try!(write!(f, " s06"))}
        if self.s05() != 0 { try!(write!(f, " s05"))}
        if self.s04() != 0 { try!(write!(f, " s04"))}
        if self.s03() != 0 { try!(write!(f, " s03"))}
        if self.s02() != 0 { try!(write!(f, " s02"))}
        if self.s01() != 0 { try!(write!(f, " s01"))}
        if self.s00() != 0 { try!(write!(f, " s00"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="display memory"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RamCom7(pub u32);
impl RamCom7 {
    #[doc="S31"]
    #[inline] pub fn s31(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if S31 != 0"]
    #[inline] pub fn test_s31(&self) -> bool {
        self.s31() != 0
    }

    #[doc="Sets the S31 field."]
    #[inline] pub fn set_s31<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="S30"]
    #[inline] pub fn s30(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if S30 != 0"]
    #[inline] pub fn test_s30(&self) -> bool {
        self.s30() != 0
    }

    #[doc="Sets the S30 field."]
    #[inline] pub fn set_s30<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="S29"]
    #[inline] pub fn s29(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if S29 != 0"]
    #[inline] pub fn test_s29(&self) -> bool {
        self.s29() != 0
    }

    #[doc="Sets the S29 field."]
    #[inline] pub fn set_s29<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="S28"]
    #[inline] pub fn s28(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if S28 != 0"]
    #[inline] pub fn test_s28(&self) -> bool {
        self.s28() != 0
    }

    #[doc="Sets the S28 field."]
    #[inline] pub fn set_s28<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="S27"]
    #[inline] pub fn s27(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if S27 != 0"]
    #[inline] pub fn test_s27(&self) -> bool {
        self.s27() != 0
    }

    #[doc="Sets the S27 field."]
    #[inline] pub fn set_s27<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="S26"]
    #[inline] pub fn s26(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if S26 != 0"]
    #[inline] pub fn test_s26(&self) -> bool {
        self.s26() != 0
    }

    #[doc="Sets the S26 field."]
    #[inline] pub fn set_s26<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="S25"]
    #[inline] pub fn s25(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if S25 != 0"]
    #[inline] pub fn test_s25(&self) -> bool {
        self.s25() != 0
    }

    #[doc="Sets the S25 field."]
    #[inline] pub fn set_s25<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="S24"]
    #[inline] pub fn s24(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if S24 != 0"]
    #[inline] pub fn test_s24(&self) -> bool {
        self.s24() != 0
    }

    #[doc="Sets the S24 field."]
    #[inline] pub fn set_s24<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="S23"]
    #[inline] pub fn s23(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if S23 != 0"]
    #[inline] pub fn test_s23(&self) -> bool {
        self.s23() != 0
    }

    #[doc="Sets the S23 field."]
    #[inline] pub fn set_s23<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="S22"]
    #[inline] pub fn s22(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if S22 != 0"]
    #[inline] pub fn test_s22(&self) -> bool {
        self.s22() != 0
    }

    #[doc="Sets the S22 field."]
    #[inline] pub fn set_s22<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="S21"]
    #[inline] pub fn s21(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if S21 != 0"]
    #[inline] pub fn test_s21(&self) -> bool {
        self.s21() != 0
    }

    #[doc="Sets the S21 field."]
    #[inline] pub fn set_s21<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="S20"]
    #[inline] pub fn s20(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if S20 != 0"]
    #[inline] pub fn test_s20(&self) -> bool {
        self.s20() != 0
    }

    #[doc="Sets the S20 field."]
    #[inline] pub fn set_s20<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="S19"]
    #[inline] pub fn s19(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if S19 != 0"]
    #[inline] pub fn test_s19(&self) -> bool {
        self.s19() != 0
    }

    #[doc="Sets the S19 field."]
    #[inline] pub fn set_s19<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="S18"]
    #[inline] pub fn s18(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if S18 != 0"]
    #[inline] pub fn test_s18(&self) -> bool {
        self.s18() != 0
    }

    #[doc="Sets the S18 field."]
    #[inline] pub fn set_s18<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="S17"]
    #[inline] pub fn s17(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if S17 != 0"]
    #[inline] pub fn test_s17(&self) -> bool {
        self.s17() != 0
    }

    #[doc="Sets the S17 field."]
    #[inline] pub fn set_s17<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="S16"]
    #[inline] pub fn s16(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if S16 != 0"]
    #[inline] pub fn test_s16(&self) -> bool {
        self.s16() != 0
    }

    #[doc="Sets the S16 field."]
    #[inline] pub fn set_s16<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="S15"]
    #[inline] pub fn s15(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if S15 != 0"]
    #[inline] pub fn test_s15(&self) -> bool {
        self.s15() != 0
    }

    #[doc="Sets the S15 field."]
    #[inline] pub fn set_s15<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="S14"]
    #[inline] pub fn s14(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if S14 != 0"]
    #[inline] pub fn test_s14(&self) -> bool {
        self.s14() != 0
    }

    #[doc="Sets the S14 field."]
    #[inline] pub fn set_s14<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="S13"]
    #[inline] pub fn s13(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if S13 != 0"]
    #[inline] pub fn test_s13(&self) -> bool {
        self.s13() != 0
    }

    #[doc="Sets the S13 field."]
    #[inline] pub fn set_s13<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="S12"]
    #[inline] pub fn s12(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if S12 != 0"]
    #[inline] pub fn test_s12(&self) -> bool {
        self.s12() != 0
    }

    #[doc="Sets the S12 field."]
    #[inline] pub fn set_s12<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="S11"]
    #[inline] pub fn s11(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if S11 != 0"]
    #[inline] pub fn test_s11(&self) -> bool {
        self.s11() != 0
    }

    #[doc="Sets the S11 field."]
    #[inline] pub fn set_s11<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="S10"]
    #[inline] pub fn s10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if S10 != 0"]
    #[inline] pub fn test_s10(&self) -> bool {
        self.s10() != 0
    }

    #[doc="Sets the S10 field."]
    #[inline] pub fn set_s10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="S09"]
    #[inline] pub fn s09(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if S09 != 0"]
    #[inline] pub fn test_s09(&self) -> bool {
        self.s09() != 0
    }

    #[doc="Sets the S09 field."]
    #[inline] pub fn set_s09<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="S08"]
    #[inline] pub fn s08(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if S08 != 0"]
    #[inline] pub fn test_s08(&self) -> bool {
        self.s08() != 0
    }

    #[doc="Sets the S08 field."]
    #[inline] pub fn set_s08<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="S07"]
    #[inline] pub fn s07(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if S07 != 0"]
    #[inline] pub fn test_s07(&self) -> bool {
        self.s07() != 0
    }

    #[doc="Sets the S07 field."]
    #[inline] pub fn set_s07<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="S06"]
    #[inline] pub fn s06(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if S06 != 0"]
    #[inline] pub fn test_s06(&self) -> bool {
        self.s06() != 0
    }

    #[doc="Sets the S06 field."]
    #[inline] pub fn set_s06<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="S05"]
    #[inline] pub fn s05(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if S05 != 0"]
    #[inline] pub fn test_s05(&self) -> bool {
        self.s05() != 0
    }

    #[doc="Sets the S05 field."]
    #[inline] pub fn set_s05<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="S04"]
    #[inline] pub fn s04(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if S04 != 0"]
    #[inline] pub fn test_s04(&self) -> bool {
        self.s04() != 0
    }

    #[doc="Sets the S04 field."]
    #[inline] pub fn set_s04<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="S03"]
    #[inline] pub fn s03(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if S03 != 0"]
    #[inline] pub fn test_s03(&self) -> bool {
        self.s03() != 0
    }

    #[doc="Sets the S03 field."]
    #[inline] pub fn set_s03<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="S02"]
    #[inline] pub fn s02(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if S02 != 0"]
    #[inline] pub fn test_s02(&self) -> bool {
        self.s02() != 0
    }

    #[doc="Sets the S02 field."]
    #[inline] pub fn set_s02<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="S01"]
    #[inline] pub fn s01(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if S01 != 0"]
    #[inline] pub fn test_s01(&self) -> bool {
        self.s01() != 0
    }

    #[doc="Sets the S01 field."]
    #[inline] pub fn set_s01<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="S00"]
    #[inline] pub fn s00(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if S00 != 0"]
    #[inline] pub fn test_s00(&self) -> bool {
        self.s00() != 0
    }

    #[doc="Sets the S00 field."]
    #[inline] pub fn set_s00<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for RamCom7 {
    #[inline]
    fn from(other: u32) -> Self {
         RamCom7(other)
    }
}

impl ::core::fmt::Display for RamCom7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RamCom7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.s31() != 0 { try!(write!(f, " s31"))}
        if self.s30() != 0 { try!(write!(f, " s30"))}
        if self.s29() != 0 { try!(write!(f, " s29"))}
        if self.s28() != 0 { try!(write!(f, " s28"))}
        if self.s27() != 0 { try!(write!(f, " s27"))}
        if self.s26() != 0 { try!(write!(f, " s26"))}
        if self.s25() != 0 { try!(write!(f, " s25"))}
        if self.s24() != 0 { try!(write!(f, " s24"))}
        if self.s23() != 0 { try!(write!(f, " s23"))}
        if self.s22() != 0 { try!(write!(f, " s22"))}
        if self.s21() != 0 { try!(write!(f, " s21"))}
        if self.s20() != 0 { try!(write!(f, " s20"))}
        if self.s19() != 0 { try!(write!(f, " s19"))}
        if self.s18() != 0 { try!(write!(f, " s18"))}
        if self.s17() != 0 { try!(write!(f, " s17"))}
        if self.s16() != 0 { try!(write!(f, " s16"))}
        if self.s15() != 0 { try!(write!(f, " s15"))}
        if self.s14() != 0 { try!(write!(f, " s14"))}
        if self.s13() != 0 { try!(write!(f, " s13"))}
        if self.s12() != 0 { try!(write!(f, " s12"))}
        if self.s11() != 0 { try!(write!(f, " s11"))}
        if self.s10() != 0 { try!(write!(f, " s10"))}
        if self.s09() != 0 { try!(write!(f, " s09"))}
        if self.s08() != 0 { try!(write!(f, " s08"))}
        if self.s07() != 0 { try!(write!(f, " s07"))}
        if self.s06() != 0 { try!(write!(f, " s06"))}
        if self.s05() != 0 { try!(write!(f, " s05"))}
        if self.s04() != 0 { try!(write!(f, " s04"))}
        if self.s03() != 0 { try!(write!(f, " s03"))}
        if self.s02() != 0 { try!(write!(f, " s02"))}
        if self.s01() != 0 { try!(write!(f, " s01"))}
        if self.s00() != 0 { try!(write!(f, " s00"))}
        try!(write!(f, "]"));
        Ok(())
    }
}


