#[allow(unused_imports)] use bobbin_common::*;

periph!( GPIOA, Gpioa, _GPIOA, GpioPeriph, 0x4000a000);
periph!( GPIOB, Gpiob, _GPIOB, GpioPeriph, 0x4000a030);
periph!( GPIOC, Gpioc, _GPIOC, GpioPeriph, 0x4000a060);
periph!( GPIOD, Gpiod, _GPIOD, GpioPeriph, 0x4000a090);
periph!( GPIOE, Gpioe, _GPIOE, GpioPeriph, 0x4000a0c0);
periph!( GPIOF, Gpiof, _GPIOF, GpioPeriph, 0x4000a0f0);
periph!( GPIOG, Gpiog, _GPIOG, GpioPeriph, 0x4000a120);
periph!( GPIOH, Gpioh, _GPIOH, GpioPeriph, 0x4000a150);
periph!( GPIOI, Gpioi, _GPIOI, GpioPeriph, 0x4000a180);
periph!( GPIOJ, Gpioj, _GPIOJ, GpioPeriph, 0x4000a1b0);
periph!( GPIOK, Gpiok, _GPIOK, GpioPeriph, 0x4000a1e0);
periph!( GPIOL, Gpiol, _GPIOL, GpioPeriph, 0x4000a210);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="GPIO Peripheral"]
pub struct GpioPeriph(pub usize); 














impl GpioPeriph {
    #[doc="Get the *mut pointer for the CTRL register."]
    #[inline] pub fn ctrl_mut(&self) -> *mut Ctrl { 
        (self.0 + 0x0) as *mut Ctrl
    }

    #[doc="Get the *const pointer for the CTRL register."]
    #[inline] pub fn ctrl_ptr(&self) -> *const Ctrl { 
           self.ctrl_mut()
    }

    #[doc="Read the CTRL register."]
    #[inline] pub fn ctrl(&self) -> Ctrl { 
        unsafe {
            read_volatile(self.ctrl_ptr())
        }
    }

    #[doc="Write the CTRL register."]
    #[inline] pub fn set_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrl_mut(), f(Ctrl(0)));
        }
        self
    }

    #[doc="Modify the CTRL register."]
    #[inline] pub fn with_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrl_mut(), f(self.ctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MODEL register."]
    #[inline] pub fn model_mut(&self) -> *mut Model { 
        (self.0 + 0x4) as *mut Model
    }

    #[doc="Get the *const pointer for the MODEL register."]
    #[inline] pub fn model_ptr(&self) -> *const Model { 
           self.model_mut()
    }

    #[doc="Read the MODEL register."]
    #[inline] pub fn model(&self) -> Model { 
        unsafe {
            read_volatile(self.model_ptr())
        }
    }

    #[doc="Write the MODEL register."]
    #[inline] pub fn set_model<F: FnOnce(Model) -> Model>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.model_mut(), f(Model(0)));
        }
        self
    }

    #[doc="Modify the MODEL register."]
    #[inline] pub fn with_model<F: FnOnce(Model) -> Model>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.model_mut(), f(self.model()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MODEH register."]
    #[inline] pub fn modeh_mut(&self) -> *mut Modeh { 
        (self.0 + 0x8) as *mut Modeh
    }

    #[doc="Get the *const pointer for the MODEH register."]
    #[inline] pub fn modeh_ptr(&self) -> *const Modeh { 
           self.modeh_mut()
    }

    #[doc="Read the MODEH register."]
    #[inline] pub fn modeh(&self) -> Modeh { 
        unsafe {
            read_volatile(self.modeh_ptr())
        }
    }

    #[doc="Write the MODEH register."]
    #[inline] pub fn set_modeh<F: FnOnce(Modeh) -> Modeh>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.modeh_mut(), f(Modeh(0)));
        }
        self
    }

    #[doc="Modify the MODEH register."]
    #[inline] pub fn with_modeh<F: FnOnce(Modeh) -> Modeh>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.modeh_mut(), f(self.modeh()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DOUT register."]
    #[inline] pub fn dout_mut(&self) -> *mut Dout { 
        (self.0 + 0xc) as *mut Dout
    }

    #[doc="Get the *const pointer for the DOUT register."]
    #[inline] pub fn dout_ptr(&self) -> *const Dout { 
           self.dout_mut()
    }

    #[doc="Read the DOUT register."]
    #[inline] pub fn dout(&self) -> Dout { 
        unsafe {
            read_volatile(self.dout_ptr())
        }
    }

    #[doc="Write the DOUT register."]
    #[inline] pub fn set_dout<F: FnOnce(Dout) -> Dout>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dout_mut(), f(Dout(0)));
        }
        self
    }

    #[doc="Modify the DOUT register."]
    #[inline] pub fn with_dout<F: FnOnce(Dout) -> Dout>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dout_mut(), f(self.dout()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DOUTTGL register."]
    #[inline] pub fn douttgl_mut(&self) -> *mut Douttgl { 
        (self.0 + 0x18) as *mut Douttgl
    }

    #[doc="Get the *const pointer for the DOUTTGL register."]
    #[inline] pub fn douttgl_ptr(&self) -> *const Douttgl { 
           self.douttgl_mut()
    }

    #[doc="Write the DOUTTGL register."]
    #[inline] pub fn set_douttgl<F: FnOnce(Douttgl) -> Douttgl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.douttgl_mut(), f(Douttgl(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the DIN register."]
    #[inline] pub fn din_mut(&self) -> *mut Din { 
        (self.0 + 0x1c) as *mut Din
    }

    #[doc="Get the *const pointer for the DIN register."]
    #[inline] pub fn din_ptr(&self) -> *const Din { 
           self.din_mut()
    }

    #[doc="Read the DIN register."]
    #[inline] pub fn din(&self) -> Din { 
        unsafe {
            read_volatile(self.din_ptr())
        }
    }

    #[doc="Get the *mut pointer for the PINLOCKN register."]
    #[inline] pub fn pinlockn_mut(&self) -> *mut Pinlockn { 
        (self.0 + 0x20) as *mut Pinlockn
    }

    #[doc="Get the *const pointer for the PINLOCKN register."]
    #[inline] pub fn pinlockn_ptr(&self) -> *const Pinlockn { 
           self.pinlockn_mut()
    }

    #[doc="Read the PINLOCKN register."]
    #[inline] pub fn pinlockn(&self) -> Pinlockn { 
        unsafe {
            read_volatile(self.pinlockn_ptr())
        }
    }

    #[doc="Write the PINLOCKN register."]
    #[inline] pub fn set_pinlockn<F: FnOnce(Pinlockn) -> Pinlockn>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pinlockn_mut(), f(Pinlockn(0)));
        }
        self
    }

    #[doc="Modify the PINLOCKN register."]
    #[inline] pub fn with_pinlockn<F: FnOnce(Pinlockn) -> Pinlockn>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pinlockn_mut(), f(self.pinlockn()));
        }
        self
    }

    #[doc="Get the *mut pointer for the OVTDIS register."]
    #[inline] pub fn ovtdis_mut(&self) -> *mut Ovtdis { 
        (self.0 + 0x28) as *mut Ovtdis
    }

    #[doc="Get the *const pointer for the OVTDIS register."]
    #[inline] pub fn ovtdis_ptr(&self) -> *const Ovtdis { 
           self.ovtdis_mut()
    }

    #[doc="Read the OVTDIS register."]
    #[inline] pub fn ovtdis(&self) -> Ovtdis { 
        unsafe {
            read_volatile(self.ovtdis_ptr())
        }
    }

    #[doc="Write the OVTDIS register."]
    #[inline] pub fn set_ovtdis<F: FnOnce(Ovtdis) -> Ovtdis>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ovtdis_mut(), f(Ovtdis(0)));
        }
        self
    }

    #[doc="Modify the OVTDIS register."]
    #[inline] pub fn with_ovtdis<F: FnOnce(Ovtdis) -> Ovtdis>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ovtdis_mut(), f(self.ovtdis()));
        }
        self
    }

}

#[doc="Port Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc="Drive strength for port"]
    #[inline] pub fn drivestrength(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DRIVESTRENGTH != 0"]
    #[inline] pub fn test_drivestrength(&self) -> bool {
        self.drivestrength() != 0
    }

    #[doc="Sets the DRIVESTRENGTH field."]
    #[inline] pub fn set_drivestrength<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Slewrate limit for port"]
    #[inline] pub fn slewrate(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if SLEWRATE != 0"]
    #[inline] pub fn test_slewrate(&self) -> bool {
        self.slewrate() != 0
    }

    #[doc="Sets the SLEWRATE field."]
    #[inline] pub fn set_slewrate<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Data In Disable"]
    #[inline] pub fn dindis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if DINDIS != 0"]
    #[inline] pub fn test_dindis(&self) -> bool {
        self.dindis() != 0
    }

    #[doc="Sets the DINDIS field."]
    #[inline] pub fn set_dindis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Alternate drive strength for port"]
    #[inline] pub fn drivestrengthalt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if DRIVESTRENGTHALT != 0"]
    #[inline] pub fn test_drivestrengthalt(&self) -> bool {
        self.drivestrengthalt() != 0
    }

    #[doc="Sets the DRIVESTRENGTHALT field."]
    #[inline] pub fn set_drivestrengthalt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Alternate slewrate limit for port"]
    #[inline] pub fn slewratealt(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x7) as u8) } // [22:20]
    }

    #[doc="Returns true if SLEWRATEALT != 0"]
    #[inline] pub fn test_slewratealt(&self) -> bool {
        self.slewratealt() != 0
    }

    #[doc="Sets the SLEWRATEALT field."]
    #[inline] pub fn set_slewratealt<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Alternate Data In Disable"]
    #[inline] pub fn dindisalt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if DINDISALT != 0"]
    #[inline] pub fn test_dindisalt(&self) -> bool {
        self.dindisalt() != 0
    }

    #[doc="Sets the DINDISALT field."]
    #[inline] pub fn set_dindisalt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

}

impl From<u32> for Ctrl {
    #[inline]
    fn from(other: u32) -> Self {
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
        if self.drivestrength() != 0 { try!(write!(f, " drivestrength"))}
        if self.slewrate() != 0 { try!(write!(f, " slewrate=0x{:x}", self.slewrate()))}
        if self.dindis() != 0 { try!(write!(f, " dindis"))}
        if self.drivestrengthalt() != 0 { try!(write!(f, " drivestrengthalt"))}
        if self.slewratealt() != 0 { try!(write!(f, " slewratealt=0x{:x}", self.slewratealt()))}
        if self.dindisalt() != 0 { try!(write!(f, " dindisalt"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Port Pin Mode Low Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Model(pub u32);
impl Model {
    #[doc="Pin n Mode"]
    #[inline] pub fn mode<I: Into<bits::R8>>(&self, index: I) -> bits::U4 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index << 2);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode<I: Into<bits::R8>>(&self, index: I) -> bool{
        self.mode(index) != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<I: Into<bits::R8>, V: Into<bits::U4>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index << 2);
        self.0 &= !(0xf << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Model {
    #[inline]
    fn from(other: u32) -> Self {
         Model(other)
    }
}

impl ::core::fmt::Display for Model {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Model {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mode(0) != 0 { try!(write!(f, " mode[0]=0x{:x}", self.mode(0)))}
        if self.mode(1) != 0 { try!(write!(f, " mode[1]=0x{:x}", self.mode(1)))}
        if self.mode(2) != 0 { try!(write!(f, " mode[2]=0x{:x}", self.mode(2)))}
        if self.mode(3) != 0 { try!(write!(f, " mode[3]=0x{:x}", self.mode(3)))}
        if self.mode(4) != 0 { try!(write!(f, " mode[4]=0x{:x}", self.mode(4)))}
        if self.mode(5) != 0 { try!(write!(f, " mode[5]=0x{:x}", self.mode(5)))}
        if self.mode(6) != 0 { try!(write!(f, " mode[6]=0x{:x}", self.mode(6)))}
        if self.mode(7) != 0 { try!(write!(f, " mode[7]=0x{:x}", self.mode(7)))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Port Pin Mode High Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Modeh(pub u32);
impl Modeh {
    #[doc="Pin 8 Mode"]
    #[inline] pub fn mode<I: Into<bits::R8>>(&self, index: I) -> bits::U4 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index << 2);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode<I: Into<bits::R8>>(&self, index: I) -> bool{
        self.mode(index) != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<I: Into<bits::R8>, V: Into<bits::U4>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index << 2);
        self.0 &= !(0xf << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Modeh {
    #[inline]
    fn from(other: u32) -> Self {
         Modeh(other)
    }
}

impl ::core::fmt::Display for Modeh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Modeh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mode(0) != 0 { try!(write!(f, " mode[0]=0x{:x}", self.mode(0)))}
        if self.mode(1) != 0 { try!(write!(f, " mode[1]=0x{:x}", self.mode(1)))}
        if self.mode(2) != 0 { try!(write!(f, " mode[2]=0x{:x}", self.mode(2)))}
        if self.mode(3) != 0 { try!(write!(f, " mode[3]=0x{:x}", self.mode(3)))}
        if self.mode(4) != 0 { try!(write!(f, " mode[4]=0x{:x}", self.mode(4)))}
        if self.mode(5) != 0 { try!(write!(f, " mode[5]=0x{:x}", self.mode(5)))}
        if self.mode(6) != 0 { try!(write!(f, " mode[6]=0x{:x}", self.mode(6)))}
        if self.mode(7) != 0 { try!(write!(f, " mode[7]=0x{:x}", self.mode(7)))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Port Data Out Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dout(pub u32);
impl Dout {
    #[doc="Data Out"]
    #[inline] pub fn dout<I: Into<bits::R16>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DOUT != 0"]
    #[inline] pub fn test_dout<I: Into<bits::R16>>(&self, index: I) -> bool{
        self.dout(index) != 0
    }

    #[doc="Sets the DOUT field."]
    #[inline] pub fn set_dout<I: Into<bits::R16>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Dout {
    #[inline]
    fn from(other: u32) -> Self {
         Dout(other)
    }
}

impl ::core::fmt::Display for Dout {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dout {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dout(0) != 0 { try!(write!(f, " dout[0]"))}
        if self.dout(1) != 0 { try!(write!(f, " dout[1]"))}
        if self.dout(2) != 0 { try!(write!(f, " dout[2]"))}
        if self.dout(3) != 0 { try!(write!(f, " dout[3]"))}
        if self.dout(4) != 0 { try!(write!(f, " dout[4]"))}
        if self.dout(5) != 0 { try!(write!(f, " dout[5]"))}
        if self.dout(6) != 0 { try!(write!(f, " dout[6]"))}
        if self.dout(7) != 0 { try!(write!(f, " dout[7]"))}
        if self.dout(8) != 0 { try!(write!(f, " dout[8]"))}
        if self.dout(9) != 0 { try!(write!(f, " dout[9]"))}
        if self.dout(10) != 0 { try!(write!(f, " dout[10]"))}
        if self.dout(11) != 0 { try!(write!(f, " dout[11]"))}
        if self.dout(12) != 0 { try!(write!(f, " dout[12]"))}
        if self.dout(13) != 0 { try!(write!(f, " dout[13]"))}
        if self.dout(14) != 0 { try!(write!(f, " dout[14]"))}
        if self.dout(15) != 0 { try!(write!(f, " dout[15]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Port Data Out Toggle Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Douttgl(pub u32);
impl Douttgl {
    #[doc="Data Out Toggle"]
    #[inline] pub fn douttgl<I: Into<bits::R16>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DOUTTGL != 0"]
    #[inline] pub fn test_douttgl<I: Into<bits::R16>>(&self, index: I) -> bool{
        self.douttgl(index) != 0
    }

    #[doc="Sets the DOUTTGL field."]
    #[inline] pub fn set_douttgl<I: Into<bits::R16>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Douttgl {
    #[inline]
    fn from(other: u32) -> Self {
         Douttgl(other)
    }
}

impl ::core::fmt::Display for Douttgl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Douttgl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.douttgl(0) != 0 { try!(write!(f, " douttgl[0]"))}
        if self.douttgl(1) != 0 { try!(write!(f, " douttgl[1]"))}
        if self.douttgl(2) != 0 { try!(write!(f, " douttgl[2]"))}
        if self.douttgl(3) != 0 { try!(write!(f, " douttgl[3]"))}
        if self.douttgl(4) != 0 { try!(write!(f, " douttgl[4]"))}
        if self.douttgl(5) != 0 { try!(write!(f, " douttgl[5]"))}
        if self.douttgl(6) != 0 { try!(write!(f, " douttgl[6]"))}
        if self.douttgl(7) != 0 { try!(write!(f, " douttgl[7]"))}
        if self.douttgl(8) != 0 { try!(write!(f, " douttgl[8]"))}
        if self.douttgl(9) != 0 { try!(write!(f, " douttgl[9]"))}
        if self.douttgl(10) != 0 { try!(write!(f, " douttgl[10]"))}
        if self.douttgl(11) != 0 { try!(write!(f, " douttgl[11]"))}
        if self.douttgl(12) != 0 { try!(write!(f, " douttgl[12]"))}
        if self.douttgl(13) != 0 { try!(write!(f, " douttgl[13]"))}
        if self.douttgl(14) != 0 { try!(write!(f, " douttgl[14]"))}
        if self.douttgl(15) != 0 { try!(write!(f, " douttgl[15]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Port Data In Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Din(pub u32);
impl Din {
    #[doc="Data In"]
    #[inline] pub fn din<I: Into<bits::R16>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DIN != 0"]
    #[inline] pub fn test_din<I: Into<bits::R16>>(&self, index: I) -> bool{
        self.din(index) != 0
    }

    #[doc="Sets the DIN field."]
    #[inline] pub fn set_din<I: Into<bits::R16>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
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
        if self.din(0) != 0 { try!(write!(f, " din[0]"))}
        if self.din(1) != 0 { try!(write!(f, " din[1]"))}
        if self.din(2) != 0 { try!(write!(f, " din[2]"))}
        if self.din(3) != 0 { try!(write!(f, " din[3]"))}
        if self.din(4) != 0 { try!(write!(f, " din[4]"))}
        if self.din(5) != 0 { try!(write!(f, " din[5]"))}
        if self.din(6) != 0 { try!(write!(f, " din[6]"))}
        if self.din(7) != 0 { try!(write!(f, " din[7]"))}
        if self.din(8) != 0 { try!(write!(f, " din[8]"))}
        if self.din(9) != 0 { try!(write!(f, " din[9]"))}
        if self.din(10) != 0 { try!(write!(f, " din[10]"))}
        if self.din(11) != 0 { try!(write!(f, " din[11]"))}
        if self.din(12) != 0 { try!(write!(f, " din[12]"))}
        if self.din(13) != 0 { try!(write!(f, " din[13]"))}
        if self.din(14) != 0 { try!(write!(f, " din[14]"))}
        if self.din(15) != 0 { try!(write!(f, " din[15]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Port Unlocked Pins Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pinlockn(pub u32);
impl Pinlockn {
    #[doc="Unlocked Pins"]
    #[inline] pub fn pinlockn<I: Into<bits::R16>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PINLOCKN != 0"]
    #[inline] pub fn test_pinlockn<I: Into<bits::R16>>(&self, index: I) -> bool{
        self.pinlockn(index) != 0
    }

    #[doc="Sets the PINLOCKN field."]
    #[inline] pub fn set_pinlockn<I: Into<bits::R16>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Pinlockn {
    #[inline]
    fn from(other: u32) -> Self {
         Pinlockn(other)
    }
}

impl ::core::fmt::Display for Pinlockn {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pinlockn {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pinlockn(0) != 0 { try!(write!(f, " pinlockn[0]"))}
        if self.pinlockn(1) != 0 { try!(write!(f, " pinlockn[1]"))}
        if self.pinlockn(2) != 0 { try!(write!(f, " pinlockn[2]"))}
        if self.pinlockn(3) != 0 { try!(write!(f, " pinlockn[3]"))}
        if self.pinlockn(4) != 0 { try!(write!(f, " pinlockn[4]"))}
        if self.pinlockn(5) != 0 { try!(write!(f, " pinlockn[5]"))}
        if self.pinlockn(6) != 0 { try!(write!(f, " pinlockn[6]"))}
        if self.pinlockn(7) != 0 { try!(write!(f, " pinlockn[7]"))}
        if self.pinlockn(8) != 0 { try!(write!(f, " pinlockn[8]"))}
        if self.pinlockn(9) != 0 { try!(write!(f, " pinlockn[9]"))}
        if self.pinlockn(10) != 0 { try!(write!(f, " pinlockn[10]"))}
        if self.pinlockn(11) != 0 { try!(write!(f, " pinlockn[11]"))}
        if self.pinlockn(12) != 0 { try!(write!(f, " pinlockn[12]"))}
        if self.pinlockn(13) != 0 { try!(write!(f, " pinlockn[13]"))}
        if self.pinlockn(14) != 0 { try!(write!(f, " pinlockn[14]"))}
        if self.pinlockn(15) != 0 { try!(write!(f, " pinlockn[15]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Over Voltage Disable for all modes"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ovtdis(pub u32);
impl Ovtdis {
    #[doc="Disable Over Voltage capability"]
    #[inline] pub fn ovtdis<I: Into<bits::R16>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OVTDIS != 0"]
    #[inline] pub fn test_ovtdis<I: Into<bits::R16>>(&self, index: I) -> bool{
        self.ovtdis(index) != 0
    }

    #[doc="Sets the OVTDIS field."]
    #[inline] pub fn set_ovtdis<I: Into<bits::R16>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Ovtdis {
    #[inline]
    fn from(other: u32) -> Self {
         Ovtdis(other)
    }
}

impl ::core::fmt::Display for Ovtdis {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ovtdis {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ovtdis(0) != 0 { try!(write!(f, " ovtdis[0]"))}
        if self.ovtdis(1) != 0 { try!(write!(f, " ovtdis[1]"))}
        if self.ovtdis(2) != 0 { try!(write!(f, " ovtdis[2]"))}
        if self.ovtdis(3) != 0 { try!(write!(f, " ovtdis[3]"))}
        if self.ovtdis(4) != 0 { try!(write!(f, " ovtdis[4]"))}
        if self.ovtdis(5) != 0 { try!(write!(f, " ovtdis[5]"))}
        if self.ovtdis(6) != 0 { try!(write!(f, " ovtdis[6]"))}
        if self.ovtdis(7) != 0 { try!(write!(f, " ovtdis[7]"))}
        if self.ovtdis(8) != 0 { try!(write!(f, " ovtdis[8]"))}
        if self.ovtdis(9) != 0 { try!(write!(f, " ovtdis[9]"))}
        if self.ovtdis(10) != 0 { try!(write!(f, " ovtdis[10]"))}
        if self.ovtdis(11) != 0 { try!(write!(f, " ovtdis[11]"))}
        if self.ovtdis(12) != 0 { try!(write!(f, " ovtdis[12]"))}
        if self.ovtdis(13) != 0 { try!(write!(f, " ovtdis[13]"))}
        if self.ovtdis(14) != 0 { try!(write!(f, " ovtdis[14]"))}
        if self.ovtdis(15) != 0 { try!(write!(f, " ovtdis[15]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

pub struct GpioPin { pub port: GpioPeriph, pub index: usize }
pin!(PA0, Pa0, GPIOA, Gpioa, _PA0, GpioPin, _GPIOA, 0);

pin!(PA1, Pa1, GPIOA, Gpioa, _PA1, GpioPin, _GPIOA, 1);

pin!(PA2, Pa2, GPIOA, Gpioa, _PA2, GpioPin, _GPIOA, 2);

pin!(PA3, Pa3, GPIOA, Gpioa, _PA3, GpioPin, _GPIOA, 3);

pin!(PA4, Pa4, GPIOA, Gpioa, _PA4, GpioPin, _GPIOA, 4);

pin!(PA5, Pa5, GPIOA, Gpioa, _PA5, GpioPin, _GPIOA, 5);

pin!(PA6, Pa6, GPIOA, Gpioa, _PA6, GpioPin, _GPIOA, 6);

pin!(PA7, Pa7, GPIOA, Gpioa, _PA7, GpioPin, _GPIOA, 7);

pin!(PA8, Pa8, GPIOA, Gpioa, _PA8, GpioPin, _GPIOA, 8);

pin!(PA9, Pa9, GPIOA, Gpioa, _PA9, GpioPin, _GPIOA, 9);

pin!(PA10, Pa10, GPIOA, Gpioa, _PA10, GpioPin, _GPIOA, 10);

pin!(PA11, Pa11, GPIOA, Gpioa, _PA11, GpioPin, _GPIOA, 11);

pin!(PA12, Pa12, GPIOA, Gpioa, _PA12, GpioPin, _GPIOA, 12);

pin!(PA13, Pa13, GPIOA, Gpioa, _PA13, GpioPin, _GPIOA, 13);

pin!(PA14, Pa14, GPIOA, Gpioa, _PA14, GpioPin, _GPIOA, 14);

pin!(PA15, Pa15, GPIOA, Gpioa, _PA15, GpioPin, _GPIOA, 15);

pin!(PB0, Pb0, GPIOB, Gpiob, _PB0, GpioPin, _GPIOB, 0);

pin!(PB1, Pb1, GPIOB, Gpiob, _PB1, GpioPin, _GPIOB, 1);

pin!(PB2, Pb2, GPIOB, Gpiob, _PB2, GpioPin, _GPIOB, 2);

pin!(PB3, Pb3, GPIOB, Gpiob, _PB3, GpioPin, _GPIOB, 3);

pin!(PB4, Pb4, GPIOB, Gpiob, _PB4, GpioPin, _GPIOB, 4);

pin!(PB5, Pb5, GPIOB, Gpiob, _PB5, GpioPin, _GPIOB, 5);

pin!(PB6, Pb6, GPIOB, Gpiob, _PB6, GpioPin, _GPIOB, 6);

pin!(PB7, Pb7, GPIOB, Gpiob, _PB7, GpioPin, _GPIOB, 7);

pin!(PB8, Pb8, GPIOB, Gpiob, _PB8, GpioPin, _GPIOB, 8);

pin!(PB9, Pb9, GPIOB, Gpiob, _PB9, GpioPin, _GPIOB, 9);

pin!(PB10, Pb10, GPIOB, Gpiob, _PB10, GpioPin, _GPIOB, 10);

pin!(PB11, Pb11, GPIOB, Gpiob, _PB11, GpioPin, _GPIOB, 11);

pin!(PB12, Pb12, GPIOB, Gpiob, _PB12, GpioPin, _GPIOB, 12);

pin!(PB13, Pb13, GPIOB, Gpiob, _PB13, GpioPin, _GPIOB, 13);

pin!(PB14, Pb14, GPIOB, Gpiob, _PB14, GpioPin, _GPIOB, 14);

pin!(PB15, Pb15, GPIOB, Gpiob, _PB15, GpioPin, _GPIOB, 15);

pin!(PC0, Pc0, GPIOC, Gpioc, _PC0, GpioPin, _GPIOC, 0);

pin!(PC1, Pc1, GPIOC, Gpioc, _PC1, GpioPin, _GPIOC, 1);

pin!(PC2, Pc2, GPIOC, Gpioc, _PC2, GpioPin, _GPIOC, 2);

pin!(PC3, Pc3, GPIOC, Gpioc, _PC3, GpioPin, _GPIOC, 3);

pin!(PC4, Pc4, GPIOC, Gpioc, _PC4, GpioPin, _GPIOC, 4);

pin!(PC5, Pc5, GPIOC, Gpioc, _PC5, GpioPin, _GPIOC, 5);

pin!(PC6, Pc6, GPIOC, Gpioc, _PC6, GpioPin, _GPIOC, 6);

pin!(PC7, Pc7, GPIOC, Gpioc, _PC7, GpioPin, _GPIOC, 7);

pin!(PC8, Pc8, GPIOC, Gpioc, _PC8, GpioPin, _GPIOC, 8);

pin!(PC9, Pc9, GPIOC, Gpioc, _PC9, GpioPin, _GPIOC, 9);

pin!(PC10, Pc10, GPIOC, Gpioc, _PC10, GpioPin, _GPIOC, 10);

pin!(PC11, Pc11, GPIOC, Gpioc, _PC11, GpioPin, _GPIOC, 11);

pin!(PC12, Pc12, GPIOC, Gpioc, _PC12, GpioPin, _GPIOC, 12);

pin!(PC13, Pc13, GPIOC, Gpioc, _PC13, GpioPin, _GPIOC, 13);

pin!(PC14, Pc14, GPIOC, Gpioc, _PC14, GpioPin, _GPIOC, 14);

pin!(PC15, Pc15, GPIOC, Gpioc, _PC15, GpioPin, _GPIOC, 15);

pin!(PD0, Pd0, GPIOD, Gpiod, _PD0, GpioPin, _GPIOD, 0);

pin!(PD1, Pd1, GPIOD, Gpiod, _PD1, GpioPin, _GPIOD, 1);

pin!(PD2, Pd2, GPIOD, Gpiod, _PD2, GpioPin, _GPIOD, 2);

pin!(PD3, Pd3, GPIOD, Gpiod, _PD3, GpioPin, _GPIOD, 3);

pin!(PD4, Pd4, GPIOD, Gpiod, _PD4, GpioPin, _GPIOD, 4);

pin!(PD5, Pd5, GPIOD, Gpiod, _PD5, GpioPin, _GPIOD, 5);

pin!(PD6, Pd6, GPIOD, Gpiod, _PD6, GpioPin, _GPIOD, 6);

pin!(PD7, Pd7, GPIOD, Gpiod, _PD7, GpioPin, _GPIOD, 7);

pin!(PD8, Pd8, GPIOD, Gpiod, _PD8, GpioPin, _GPIOD, 8);

pin!(PD9, Pd9, GPIOD, Gpiod, _PD9, GpioPin, _GPIOD, 9);

pin!(PD10, Pd10, GPIOD, Gpiod, _PD10, GpioPin, _GPIOD, 10);

pin!(PD11, Pd11, GPIOD, Gpiod, _PD11, GpioPin, _GPIOD, 11);

pin!(PD12, Pd12, GPIOD, Gpiod, _PD12, GpioPin, _GPIOD, 12);

pin!(PD13, Pd13, GPIOD, Gpiod, _PD13, GpioPin, _GPIOD, 13);

pin!(PD14, Pd14, GPIOD, Gpiod, _PD14, GpioPin, _GPIOD, 14);

pin!(PD15, Pd15, GPIOD, Gpiod, _PD15, GpioPin, _GPIOD, 15);

pin!(PE0, Pe0, GPIOE, Gpioe, _PE0, GpioPin, _GPIOE, 0);

pin!(PE1, Pe1, GPIOE, Gpioe, _PE1, GpioPin, _GPIOE, 1);

pin!(PE2, Pe2, GPIOE, Gpioe, _PE2, GpioPin, _GPIOE, 2);

pin!(PE3, Pe3, GPIOE, Gpioe, _PE3, GpioPin, _GPIOE, 3);

pin!(PE4, Pe4, GPIOE, Gpioe, _PE4, GpioPin, _GPIOE, 4);

pin!(PE5, Pe5, GPIOE, Gpioe, _PE5, GpioPin, _GPIOE, 5);

pin!(PE6, Pe6, GPIOE, Gpioe, _PE6, GpioPin, _GPIOE, 6);

pin!(PE7, Pe7, GPIOE, Gpioe, _PE7, GpioPin, _GPIOE, 7);

pin!(PE8, Pe8, GPIOE, Gpioe, _PE8, GpioPin, _GPIOE, 8);

pin!(PE9, Pe9, GPIOE, Gpioe, _PE9, GpioPin, _GPIOE, 9);

pin!(PE10, Pe10, GPIOE, Gpioe, _PE10, GpioPin, _GPIOE, 10);

pin!(PE11, Pe11, GPIOE, Gpioe, _PE11, GpioPin, _GPIOE, 11);

pin!(PE12, Pe12, GPIOE, Gpioe, _PE12, GpioPin, _GPIOE, 12);

pin!(PE13, Pe13, GPIOE, Gpioe, _PE13, GpioPin, _GPIOE, 13);

pin!(PE14, Pe14, GPIOE, Gpioe, _PE14, GpioPin, _GPIOE, 14);

pin!(PE15, Pe15, GPIOE, Gpioe, _PE15, GpioPin, _GPIOE, 15);

pin!(PF0, Pf0, GPIOF, Gpiof, _PF0, GpioPin, _GPIOF, 0);

pin!(PF1, Pf1, GPIOF, Gpiof, _PF1, GpioPin, _GPIOF, 1);

pin!(PF2, Pf2, GPIOF, Gpiof, _PF2, GpioPin, _GPIOF, 2);

pin!(PF3, Pf3, GPIOF, Gpiof, _PF3, GpioPin, _GPIOF, 3);

pin!(PF4, Pf4, GPIOF, Gpiof, _PF4, GpioPin, _GPIOF, 4);

pin!(PF5, Pf5, GPIOF, Gpiof, _PF5, GpioPin, _GPIOF, 5);

pin!(PF6, Pf6, GPIOF, Gpiof, _PF6, GpioPin, _GPIOF, 6);

pin!(PF7, Pf7, GPIOF, Gpiof, _PF7, GpioPin, _GPIOF, 7);

pin!(PF8, Pf8, GPIOF, Gpiof, _PF8, GpioPin, _GPIOF, 8);

pin!(PF9, Pf9, GPIOF, Gpiof, _PF9, GpioPin, _GPIOF, 9);

pin!(PF10, Pf10, GPIOF, Gpiof, _PF10, GpioPin, _GPIOF, 10);

pin!(PF11, Pf11, GPIOF, Gpiof, _PF11, GpioPin, _GPIOF, 11);

pin!(PF12, Pf12, GPIOF, Gpiof, _PF12, GpioPin, _GPIOF, 12);

pin!(PF13, Pf13, GPIOF, Gpiof, _PF13, GpioPin, _GPIOF, 13);

pin!(PF14, Pf14, GPIOF, Gpiof, _PF14, GpioPin, _GPIOF, 14);

pin!(PF15, Pf15, GPIOF, Gpiof, _PF15, GpioPin, _GPIOF, 15);

pin!(PG0, Pg0, GPIOG, Gpiog, _PG0, GpioPin, _GPIOG, 0);

pin!(PG1, Pg1, GPIOG, Gpiog, _PG1, GpioPin, _GPIOG, 1);

pin!(PG2, Pg2, GPIOG, Gpiog, _PG2, GpioPin, _GPIOG, 2);

pin!(PG3, Pg3, GPIOG, Gpiog, _PG3, GpioPin, _GPIOG, 3);

pin!(PG4, Pg4, GPIOG, Gpiog, _PG4, GpioPin, _GPIOG, 4);

pin!(PG5, Pg5, GPIOG, Gpiog, _PG5, GpioPin, _GPIOG, 5);

pin!(PG6, Pg6, GPIOG, Gpiog, _PG6, GpioPin, _GPIOG, 6);

pin!(PG7, Pg7, GPIOG, Gpiog, _PG7, GpioPin, _GPIOG, 7);

pin!(PG8, Pg8, GPIOG, Gpiog, _PG8, GpioPin, _GPIOG, 8);

pin!(PG9, Pg9, GPIOG, Gpiog, _PG9, GpioPin, _GPIOG, 9);

pin!(PG10, Pg10, GPIOG, Gpiog, _PG10, GpioPin, _GPIOG, 10);

pin!(PG11, Pg11, GPIOG, Gpiog, _PG11, GpioPin, _GPIOG, 11);

pin!(PG12, Pg12, GPIOG, Gpiog, _PG12, GpioPin, _GPIOG, 12);

pin!(PG13, Pg13, GPIOG, Gpiog, _PG13, GpioPin, _GPIOG, 13);

pin!(PG14, Pg14, GPIOG, Gpiog, _PG14, GpioPin, _GPIOG, 14);

pin!(PG15, Pg15, GPIOG, Gpiog, _PG15, GpioPin, _GPIOG, 15);

pin!(PH0, Ph0, GPIOH, Gpioh, _PH0, GpioPin, _GPIOH, 0);

pin!(PH1, Ph1, GPIOH, Gpioh, _PH1, GpioPin, _GPIOH, 1);

pin!(PH2, Ph2, GPIOH, Gpioh, _PH2, GpioPin, _GPIOH, 2);

pin!(PH3, Ph3, GPIOH, Gpioh, _PH3, GpioPin, _GPIOH, 3);

pin!(PH4, Ph4, GPIOH, Gpioh, _PH4, GpioPin, _GPIOH, 4);

pin!(PH5, Ph5, GPIOH, Gpioh, _PH5, GpioPin, _GPIOH, 5);

pin!(PH6, Ph6, GPIOH, Gpioh, _PH6, GpioPin, _GPIOH, 6);

pin!(PH7, Ph7, GPIOH, Gpioh, _PH7, GpioPin, _GPIOH, 7);

pin!(PH8, Ph8, GPIOH, Gpioh, _PH8, GpioPin, _GPIOH, 8);

pin!(PH9, Ph9, GPIOH, Gpioh, _PH9, GpioPin, _GPIOH, 9);

pin!(PH10, Ph10, GPIOH, Gpioh, _PH10, GpioPin, _GPIOH, 10);

pin!(PH11, Ph11, GPIOH, Gpioh, _PH11, GpioPin, _GPIOH, 11);

pin!(PH12, Ph12, GPIOH, Gpioh, _PH12, GpioPin, _GPIOH, 12);

pin!(PH13, Ph13, GPIOH, Gpioh, _PH13, GpioPin, _GPIOH, 13);

pin!(PH14, Ph14, GPIOH, Gpioh, _PH14, GpioPin, _GPIOH, 14);

pin!(PH15, Ph15, GPIOH, Gpioh, _PH15, GpioPin, _GPIOH, 15);

pin!(PI0, Pi0, GPIOI, Gpioi, _PI0, GpioPin, _GPIOI, 0);

pin!(PI1, Pi1, GPIOI, Gpioi, _PI1, GpioPin, _GPIOI, 1);

pin!(PI2, Pi2, GPIOI, Gpioi, _PI2, GpioPin, _GPIOI, 2);

pin!(PI3, Pi3, GPIOI, Gpioi, _PI3, GpioPin, _GPIOI, 3);

pin!(PI4, Pi4, GPIOI, Gpioi, _PI4, GpioPin, _GPIOI, 4);

pin!(PI5, Pi5, GPIOI, Gpioi, _PI5, GpioPin, _GPIOI, 5);

pin!(PI6, Pi6, GPIOI, Gpioi, _PI6, GpioPin, _GPIOI, 6);

pin!(PI7, Pi7, GPIOI, Gpioi, _PI7, GpioPin, _GPIOI, 7);

pin!(PI8, Pi8, GPIOI, Gpioi, _PI8, GpioPin, _GPIOI, 8);

pin!(PI9, Pi9, GPIOI, Gpioi, _PI9, GpioPin, _GPIOI, 9);

pin!(PI10, Pi10, GPIOI, Gpioi, _PI10, GpioPin, _GPIOI, 10);

pin!(PI11, Pi11, GPIOI, Gpioi, _PI11, GpioPin, _GPIOI, 11);

pin!(PI12, Pi12, GPIOI, Gpioi, _PI12, GpioPin, _GPIOI, 12);

pin!(PI13, Pi13, GPIOI, Gpioi, _PI13, GpioPin, _GPIOI, 13);

pin!(PI14, Pi14, GPIOI, Gpioi, _PI14, GpioPin, _GPIOI, 14);

pin!(PI15, Pi15, GPIOI, Gpioi, _PI15, GpioPin, _GPIOI, 15);

pin!(PJ0, Pj0, GPIOJ, Gpioj, _PJ0, GpioPin, _GPIOJ, 0);

pin!(PJ1, Pj1, GPIOJ, Gpioj, _PJ1, GpioPin, _GPIOJ, 1);

pin!(PJ2, Pj2, GPIOJ, Gpioj, _PJ2, GpioPin, _GPIOJ, 2);

pin!(PJ3, Pj3, GPIOJ, Gpioj, _PJ3, GpioPin, _GPIOJ, 3);

pin!(PJ4, Pj4, GPIOJ, Gpioj, _PJ4, GpioPin, _GPIOJ, 4);

pin!(PJ5, Pj5, GPIOJ, Gpioj, _PJ5, GpioPin, _GPIOJ, 5);

pin!(PJ6, Pj6, GPIOJ, Gpioj, _PJ6, GpioPin, _GPIOJ, 6);

pin!(PJ7, Pj7, GPIOJ, Gpioj, _PJ7, GpioPin, _GPIOJ, 7);

pin!(PJ8, Pj8, GPIOJ, Gpioj, _PJ8, GpioPin, _GPIOJ, 8);

pin!(PJ9, Pj9, GPIOJ, Gpioj, _PJ9, GpioPin, _GPIOJ, 9);

pin!(PJ10, Pj10, GPIOJ, Gpioj, _PJ10, GpioPin, _GPIOJ, 10);

pin!(PJ11, Pj11, GPIOJ, Gpioj, _PJ11, GpioPin, _GPIOJ, 11);

pin!(PJ12, Pj12, GPIOJ, Gpioj, _PJ12, GpioPin, _GPIOJ, 12);

pin!(PJ13, Pj13, GPIOJ, Gpioj, _PJ13, GpioPin, _GPIOJ, 13);

pin!(PJ14, Pj14, GPIOJ, Gpioj, _PJ14, GpioPin, _GPIOJ, 14);

pin!(PJ15, Pj15, GPIOJ, Gpioj, _PJ15, GpioPin, _GPIOJ, 15);

pin!(PK0, Pk0, GPIOK, Gpiok, _PK0, GpioPin, _GPIOK, 0);

pin!(PK1, Pk1, GPIOK, Gpiok, _PK1, GpioPin, _GPIOK, 1);

pin!(PK2, Pk2, GPIOK, Gpiok, _PK2, GpioPin, _GPIOK, 2);

pin!(PK3, Pk3, GPIOK, Gpiok, _PK3, GpioPin, _GPIOK, 3);

pin!(PK4, Pk4, GPIOK, Gpiok, _PK4, GpioPin, _GPIOK, 4);

pin!(PK5, Pk5, GPIOK, Gpiok, _PK5, GpioPin, _GPIOK, 5);

pin!(PK6, Pk6, GPIOK, Gpiok, _PK6, GpioPin, _GPIOK, 6);

pin!(PK7, Pk7, GPIOK, Gpiok, _PK7, GpioPin, _GPIOK, 7);

pin!(PK8, Pk8, GPIOK, Gpiok, _PK8, GpioPin, _GPIOK, 8);

pin!(PK9, Pk9, GPIOK, Gpiok, _PK9, GpioPin, _GPIOK, 9);

pin!(PK10, Pk10, GPIOK, Gpiok, _PK10, GpioPin, _GPIOK, 10);

pin!(PK11, Pk11, GPIOK, Gpiok, _PK11, GpioPin, _GPIOK, 11);

pin!(PK12, Pk12, GPIOK, Gpiok, _PK12, GpioPin, _GPIOK, 12);

pin!(PK13, Pk13, GPIOK, Gpiok, _PK13, GpioPin, _GPIOK, 13);

pin!(PK14, Pk14, GPIOK, Gpiok, _PK14, GpioPin, _GPIOK, 14);

pin!(PK15, Pk15, GPIOK, Gpiok, _PK15, GpioPin, _GPIOK, 15);

pin!(PL0, Pl0, GPIOL, Gpiol, _PL0, GpioPin, _GPIOL, 0);

pin!(PL1, Pl1, GPIOL, Gpiol, _PL1, GpioPin, _GPIOL, 1);

pin!(PL2, Pl2, GPIOL, Gpiol, _PL2, GpioPin, _GPIOL, 2);

pin!(PL3, Pl3, GPIOL, Gpiol, _PL3, GpioPin, _GPIOL, 3);

pin!(PL4, Pl4, GPIOL, Gpiol, _PL4, GpioPin, _GPIOL, 4);

pin!(PL5, Pl5, GPIOL, Gpiol, _PL5, GpioPin, _GPIOL, 5);

pin!(PL6, Pl6, GPIOL, Gpiol, _PL6, GpioPin, _GPIOL, 6);

pin!(PL7, Pl7, GPIOL, Gpiol, _PL7, GpioPin, _GPIOL, 7);

pin!(PL8, Pl8, GPIOL, Gpiol, _PL8, GpioPin, _GPIOL, 8);

pin!(PL9, Pl9, GPIOL, Gpiol, _PL9, GpioPin, _GPIOL, 9);

pin!(PL10, Pl10, GPIOL, Gpiol, _PL10, GpioPin, _GPIOL, 10);

pin!(PL11, Pl11, GPIOL, Gpiol, _PL11, GpioPin, _GPIOL, 11);

pin!(PL12, Pl12, GPIOL, Gpiol, _PL12, GpioPin, _GPIOL, 12);

pin!(PL13, Pl13, GPIOL, Gpiol, _PL13, GpioPin, _GPIOL, 13);

pin!(PL14, Pl14, GPIOL, Gpiol, _PL14, GpioPin, _GPIOL, 14);

pin!(PL15, Pl15, GPIOL, Gpiol, _PL15, GpioPin, _GPIOL, 15);


