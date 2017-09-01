#[allow(unused_imports)] use bobbin_common::*;

periph!( I2C0, I2c0, _I2C0, I2cPeriph, 0x40020000);
periph!( I2C1, I2c1, _I2C1, I2cPeriph, 0x40021000);
periph!( I2C2, I2c2, _I2C2, I2cPeriph, 0x40022000);
periph!( I2C3, I2c3, _I2C3, I2cPeriph, 0x40023000);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="I2C Peripheral"]
pub struct I2cPeriph(pub usize); 






impl I2cPeriph {
    #[doc="Get the *const pointer for the FIFODATA register."]
    #[inline] pub fn fifodata_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0xf00) as *const u32
    }

    #[doc="Get the *mut pointer for the FIFODATA register."]
    #[inline] pub fn fifodata_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0xf00) as *mut u32
    }

    #[doc="Read the FIFODATA register."]
    #[inline] pub fn fifodata(&self) -> Fifodata { 
        unsafe {
            Fifodata(read_volatile((self.0 + 0xf00) as *const u32))
        }
    }

    #[doc="Write the FIFODATA register."]
    #[inline] pub fn set_fifodata<F: FnOnce(Fifodata) -> Fifodata>(&self, f: F) -> &Self {
        let value = f(Fifodata(0));
        unsafe {
            write_volatile((self.0 + 0xf00) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the FIFODATA register."]
    #[inline] pub fn with_fifodata<F: FnOnce(Fifodata) -> Fifodata>(&self, f: F) -> &Self {
        let tmp = self.fifodata();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0xf00) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the FIFOCTL register."]
    #[inline] pub fn fifoctl_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0xf04) as *const u32
    }

    #[doc="Get the *mut pointer for the FIFOCTL register."]
    #[inline] pub fn fifoctl_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0xf04) as *mut u32
    }

    #[doc="Read the FIFOCTL register."]
    #[inline] pub fn fifoctl(&self) -> Fifoctl { 
        unsafe {
            Fifoctl(read_volatile((self.0 + 0xf04) as *const u32))
        }
    }

    #[doc="Write the FIFOCTL register."]
    #[inline] pub fn set_fifoctl<F: FnOnce(Fifoctl) -> Fifoctl>(&self, f: F) -> &Self {
        let value = f(Fifoctl(0));
        unsafe {
            write_volatile((self.0 + 0xf04) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the FIFOCTL register."]
    #[inline] pub fn with_fifoctl<F: FnOnce(Fifoctl) -> Fifoctl>(&self, f: F) -> &Self {
        let tmp = self.fifoctl();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0xf04) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the FIFOSTATUS register."]
    #[inline] pub fn fifostatus_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0xf08) as *const u32
    }

    #[doc="Get the *mut pointer for the FIFOSTATUS register."]
    #[inline] pub fn fifostatus_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0xf08) as *mut u32
    }

    #[doc="Read the FIFOSTATUS register."]
    #[inline] pub fn fifostatus(&self) -> Fifostatus { 
        unsafe {
            Fifostatus(read_volatile((self.0 + 0xf08) as *const u32))
        }
    }

    #[doc="Write the FIFOSTATUS register."]
    #[inline] pub fn set_fifostatus<F: FnOnce(Fifostatus) -> Fifostatus>(&self, f: F) -> &Self {
        let value = f(Fifostatus(0));
        unsafe {
            write_volatile((self.0 + 0xf08) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the FIFOSTATUS register."]
    #[inline] pub fn with_fifostatus<F: FnOnce(Fifostatus) -> Fifostatus>(&self, f: F) -> &Self {
        let tmp = self.fifostatus();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0xf08) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the PP register."]
    #[inline] pub fn pp_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0xfc0) as *const u32
    }

    #[doc="Get the *mut pointer for the PP register."]
    #[inline] pub fn pp_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0xfc0) as *mut u32
    }

    #[doc="Read the PP register."]
    #[inline] pub fn pp(&self) -> Pp { 
        unsafe {
            Pp(read_volatile((self.0 + 0xfc0) as *const u32))
        }
    }

    #[doc="Write the PP register."]
    #[inline] pub fn set_pp<F: FnOnce(Pp) -> Pp>(&self, f: F) -> &Self {
        let value = f(Pp(0));
        unsafe {
            write_volatile((self.0 + 0xfc0) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the PP register."]
    #[inline] pub fn with_pp<F: FnOnce(Pp) -> Pp>(&self, f: F) -> &Self {
        let tmp = self.pp();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0xfc0) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the PC register."]
    #[inline] pub fn pc_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0xfc4) as *const u32
    }

    #[doc="Get the *mut pointer for the PC register."]
    #[inline] pub fn pc_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0xfc4) as *mut u32
    }

    #[doc="Read the PC register."]
    #[inline] pub fn pc(&self) -> Pc { 
        unsafe {
            Pc(read_volatile((self.0 + 0xfc4) as *const u32))
        }
    }

    #[doc="Write the PC register."]
    #[inline] pub fn set_pc<F: FnOnce(Pc) -> Pc>(&self, f: F) -> &Self {
        let value = f(Pc(0));
        unsafe {
            write_volatile((self.0 + 0xfc4) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the PC register."]
    #[inline] pub fn with_pc<F: FnOnce(Pc) -> Pc>(&self, f: F) -> &Self {
        let tmp = self.pc();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0xfc4) as *mut u32, value.0);
        }
        self
    }

}

#[doc="I2C FIFO Data"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fifodata(pub u32);
impl Fifodata {
    #[doc="I2C TX FIFO Write Data Byte"]
    #[inline] pub fn data(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="I2C TX FIFO Write Data Byte"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data != 0
    }

    #[doc="I2C TX FIFO Write Data Byte"]
    #[inline] pub fn set_data<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Fifodata {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fifodata {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.data() != 0 { try!(write!(f, " data=0x{:x}", self.data()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C FIFO Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fifoctl(pub u32);
impl Fifoctl {
    #[doc="TX FIFO Trigger"]
    #[inline] pub fn txtrig(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="TX FIFO Trigger"]
    #[inline] pub fn test_txtrig(&self) -> bool {
        self.txtrig != 0
    }

    #[doc="TX FIFO Trigger"]
    #[inline] pub fn set_txtrig<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="DMA TX Channel Enable"]
    #[inline] pub fn dmatxena(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="DMA TX Channel Enable"]
    #[inline] pub fn test_dmatxena(&self) -> bool {
        self.dmatxena != 0
    }

    #[doc="DMA TX Channel Enable"]
    #[inline] pub fn set_dmatxena<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="TX FIFO Flush"]
    #[inline] pub fn txflush(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="TX FIFO Flush"]
    #[inline] pub fn test_txflush(&self) -> bool {
        self.txflush != 0
    }

    #[doc="TX FIFO Flush"]
    #[inline] pub fn set_txflush<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="TX Control Assignment"]
    #[inline] pub fn txasgnmt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="TX Control Assignment"]
    #[inline] pub fn test_txasgnmt(&self) -> bool {
        self.txasgnmt != 0
    }

    #[doc="TX Control Assignment"]
    #[inline] pub fn set_txasgnmt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="RX FIFO Trigger"]
    #[inline] pub fn rxtrig(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7) as u8) } // [18:16]
    }

    #[doc="RX FIFO Trigger"]
    #[inline] pub fn test_rxtrig(&self) -> bool {
        self.rxtrig != 0
    }

    #[doc="RX FIFO Trigger"]
    #[inline] pub fn set_rxtrig<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="DMA RX Channel Enable"]
    #[inline] pub fn dmarxena(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="DMA RX Channel Enable"]
    #[inline] pub fn test_dmarxena(&self) -> bool {
        self.dmarxena != 0
    }

    #[doc="DMA RX Channel Enable"]
    #[inline] pub fn set_dmarxena<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="RX FIFO Flush"]
    #[inline] pub fn rxflush(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="RX FIFO Flush"]
    #[inline] pub fn test_rxflush(&self) -> bool {
        self.rxflush != 0
    }

    #[doc="RX FIFO Flush"]
    #[inline] pub fn set_rxflush<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="RX Control Assignment"]
    #[inline] pub fn rxasgnmt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="RX Control Assignment"]
    #[inline] pub fn test_rxasgnmt(&self) -> bool {
        self.rxasgnmt != 0
    }

    #[doc="RX Control Assignment"]
    #[inline] pub fn set_rxasgnmt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl ::core::fmt::Display for Fifoctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fifoctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txtrig() != 0 { try!(write!(f, " txtrig=0x{:x}", self.txtrig()))}
        if self.dmatxena() != 0 { try!(write!(f, " dmatxena"))}
        if self.txflush() != 0 { try!(write!(f, " txflush"))}
        if self.txasgnmt() != 0 { try!(write!(f, " txasgnmt"))}
        if self.rxtrig() != 0 { try!(write!(f, " rxtrig=0x{:x}", self.rxtrig()))}
        if self.dmarxena() != 0 { try!(write!(f, " dmarxena"))}
        if self.rxflush() != 0 { try!(write!(f, " rxflush"))}
        if self.rxasgnmt() != 0 { try!(write!(f, " rxasgnmt"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C FIFO Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fifostatus(pub u32);
impl Fifostatus {
    #[doc="TX FIFO Empty"]
    #[inline] pub fn txfe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="TX FIFO Empty"]
    #[inline] pub fn test_txfe(&self) -> bool {
        self.txfe != 0
    }

    #[doc="TX FIFO Empty"]
    #[inline] pub fn set_txfe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="TX FIFO Full"]
    #[inline] pub fn txff(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="TX FIFO Full"]
    #[inline] pub fn test_txff(&self) -> bool {
        self.txff != 0
    }

    #[doc="TX FIFO Full"]
    #[inline] pub fn set_txff<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="TX FIFO Below Trigger Level"]
    #[inline] pub fn txblwtrig(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="TX FIFO Below Trigger Level"]
    #[inline] pub fn test_txblwtrig(&self) -> bool {
        self.txblwtrig != 0
    }

    #[doc="TX FIFO Below Trigger Level"]
    #[inline] pub fn set_txblwtrig<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="RX FIFO Empty"]
    #[inline] pub fn rxfe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="RX FIFO Empty"]
    #[inline] pub fn test_rxfe(&self) -> bool {
        self.rxfe != 0
    }

    #[doc="RX FIFO Empty"]
    #[inline] pub fn set_rxfe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="RX FIFO Full"]
    #[inline] pub fn rxff(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="RX FIFO Full"]
    #[inline] pub fn test_rxff(&self) -> bool {
        self.rxff != 0
    }

    #[doc="RX FIFO Full"]
    #[inline] pub fn set_rxff<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="RX FIFO Above Trigger Level"]
    #[inline] pub fn rxabvtrig(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="RX FIFO Above Trigger Level"]
    #[inline] pub fn test_rxabvtrig(&self) -> bool {
        self.rxabvtrig != 0
    }

    #[doc="RX FIFO Above Trigger Level"]
    #[inline] pub fn set_rxabvtrig<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

}

impl ::core::fmt::Display for Fifostatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fifostatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txfe() != 0 { try!(write!(f, " txfe"))}
        if self.txff() != 0 { try!(write!(f, " txff"))}
        if self.txblwtrig() != 0 { try!(write!(f, " txblwtrig"))}
        if self.rxfe() != 0 { try!(write!(f, " rxfe"))}
        if self.rxff() != 0 { try!(write!(f, " rxff"))}
        if self.rxabvtrig() != 0 { try!(write!(f, " rxabvtrig"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Peripheral Properties"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pp(pub u32);
impl Pp {
    #[doc="High-Speed Capable"]
    #[inline] pub fn hs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="High-Speed Capable"]
    #[inline] pub fn test_hs(&self) -> bool {
        self.hs != 0
    }

    #[doc="High-Speed Capable"]
    #[inline] pub fn set_hs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Pp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.hs() != 0 { try!(write!(f, " hs"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Peripheral Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pc(pub u32);
impl Pc {
    #[doc="High-Speed Capable"]
    #[inline] pub fn hs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="High-Speed Capable"]
    #[inline] pub fn test_hs(&self) -> bool {
        self.hs != 0
    }

    #[doc="High-Speed Capable"]
    #[inline] pub fn set_hs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Pc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.hs() != 0 { try!(write!(f, " hs"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

impl I2cPeriph {
    #[inline] pub fn master(&self) -> master::Master {
        master::Master(self.0 + 0x0)
    }
    #[inline] pub fn slave(&self) -> slave::Slave {
        slave::Slave(self.0 + 0x0)
    }
}

pub mod master {
#[allow(unused_imports)] use bobbin_common::*;
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Master(pub usize);
impl Master {
    #[doc="Get the *const pointer for the MSA register."]
    #[inline] pub fn msa_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x0) as *const u32
    }

    #[doc="Get the *mut pointer for the MSA register."]
    #[inline] pub fn msa_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x0) as *mut u32
    }

    #[doc="Read the MSA register."]
    #[inline] pub fn msa(&self) -> Msa { 
        unsafe {
            Msa(read_volatile((self.0 + 0x0) as *const u32))
        }
    }

    #[doc="Write the MSA register."]
    #[inline] pub fn set_msa<F: FnOnce(Msa) -> Msa>(&self, f: F) -> &Self {
        let value = f(Msa(0));
        unsafe {
            write_volatile((self.0 + 0x0) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the MSA register."]
    #[inline] pub fn with_msa<F: FnOnce(Msa) -> Msa>(&self, f: F) -> &Self {
        let tmp = self.msa();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x0) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the MCS_WRITE register."]
    #[inline] pub fn mcs_write_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x4) as *const u32
    }

    #[doc="Get the *mut pointer for the MCS_WRITE register."]
    #[inline] pub fn mcs_write_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x4) as *mut u32
    }

    #[doc="Write the MCS_WRITE register."]
    #[inline] pub fn set_mcs_write<F: FnOnce(McsWrite) -> McsWrite>(&self, f: F) -> &Self {
        let value = f(McsWrite(0));
        unsafe {
            write_volatile((self.0 + 0x4) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the MCS_READ register."]
    #[inline] pub fn mcs_read_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x4) as *const u32
    }

    #[doc="Get the *mut pointer for the MCS_READ register."]
    #[inline] pub fn mcs_read_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x4) as *mut u32
    }

    #[doc="Read the MCS_READ register."]
    #[inline] pub fn mcs_read(&self) -> McsRead { 
        unsafe {
            McsRead(read_volatile((self.0 + 0x4) as *const u32))
        }
    }

    #[doc="Get the *const pointer for the MDR register."]
    #[inline] pub fn mdr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x8) as *const u32
    }

    #[doc="Get the *mut pointer for the MDR register."]
    #[inline] pub fn mdr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x8) as *mut u32
    }

    #[doc="Read the MDR register."]
    #[inline] pub fn mdr(&self) -> Mdr { 
        unsafe {
            Mdr(read_volatile((self.0 + 0x8) as *const u32))
        }
    }

    #[doc="Write the MDR register."]
    #[inline] pub fn set_mdr<F: FnOnce(Mdr) -> Mdr>(&self, f: F) -> &Self {
        let value = f(Mdr(0));
        unsafe {
            write_volatile((self.0 + 0x8) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the MDR register."]
    #[inline] pub fn with_mdr<F: FnOnce(Mdr) -> Mdr>(&self, f: F) -> &Self {
        let tmp = self.mdr();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x8) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the MTPR register."]
    #[inline] pub fn mtpr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0xc) as *const u32
    }

    #[doc="Get the *mut pointer for the MTPR register."]
    #[inline] pub fn mtpr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0xc) as *mut u32
    }

    #[doc="Read the MTPR register."]
    #[inline] pub fn mtpr(&self) -> Mtpr { 
        unsafe {
            Mtpr(read_volatile((self.0 + 0xc) as *const u32))
        }
    }

    #[doc="Write the MTPR register."]
    #[inline] pub fn set_mtpr<F: FnOnce(Mtpr) -> Mtpr>(&self, f: F) -> &Self {
        let value = f(Mtpr(0));
        unsafe {
            write_volatile((self.0 + 0xc) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the MTPR register."]
    #[inline] pub fn with_mtpr<F: FnOnce(Mtpr) -> Mtpr>(&self, f: F) -> &Self {
        let tmp = self.mtpr();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0xc) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the MIMR register."]
    #[inline] pub fn mimr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x10) as *const u32
    }

    #[doc="Get the *mut pointer for the MIMR register."]
    #[inline] pub fn mimr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x10) as *mut u32
    }

    #[doc="Read the MIMR register."]
    #[inline] pub fn mimr(&self) -> Mimr { 
        unsafe {
            Mimr(read_volatile((self.0 + 0x10) as *const u32))
        }
    }

    #[doc="Write the MIMR register."]
    #[inline] pub fn set_mimr<F: FnOnce(Mimr) -> Mimr>(&self, f: F) -> &Self {
        let value = f(Mimr(0));
        unsafe {
            write_volatile((self.0 + 0x10) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the MIMR register."]
    #[inline] pub fn with_mimr<F: FnOnce(Mimr) -> Mimr>(&self, f: F) -> &Self {
        let tmp = self.mimr();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x10) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the MRIS register."]
    #[inline] pub fn mris_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x14) as *const u32
    }

    #[doc="Get the *mut pointer for the MRIS register."]
    #[inline] pub fn mris_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x14) as *mut u32
    }

    #[doc="Read the MRIS register."]
    #[inline] pub fn mris(&self) -> Mris { 
        unsafe {
            Mris(read_volatile((self.0 + 0x14) as *const u32))
        }
    }

    #[doc="Write the MRIS register."]
    #[inline] pub fn set_mris<F: FnOnce(Mris) -> Mris>(&self, f: F) -> &Self {
        let value = f(Mris(0));
        unsafe {
            write_volatile((self.0 + 0x14) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the MRIS register."]
    #[inline] pub fn with_mris<F: FnOnce(Mris) -> Mris>(&self, f: F) -> &Self {
        let tmp = self.mris();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x14) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the MMIS register."]
    #[inline] pub fn mmis_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x18) as *const u32
    }

    #[doc="Get the *mut pointer for the MMIS register."]
    #[inline] pub fn mmis_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x18) as *mut u32
    }

    #[doc="Read the MMIS register."]
    #[inline] pub fn mmis(&self) -> Mmis { 
        unsafe {
            Mmis(read_volatile((self.0 + 0x18) as *const u32))
        }
    }

    #[doc="Write the MMIS register."]
    #[inline] pub fn set_mmis<F: FnOnce(Mmis) -> Mmis>(&self, f: F) -> &Self {
        let value = f(Mmis(0));
        unsafe {
            write_volatile((self.0 + 0x18) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the MMIS register."]
    #[inline] pub fn with_mmis<F: FnOnce(Mmis) -> Mmis>(&self, f: F) -> &Self {
        let tmp = self.mmis();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x18) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the MICR register."]
    #[inline] pub fn micr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x1c) as *const u32
    }

    #[doc="Get the *mut pointer for the MICR register."]
    #[inline] pub fn micr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x1c) as *mut u32
    }

    #[doc="Write the MICR register."]
    #[inline] pub fn set_micr<F: FnOnce(Micr) -> Micr>(&self, f: F) -> &Self {
        let value = f(Micr(0));
        unsafe {
            write_volatile((self.0 + 0x1c) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the MCR register."]
    #[inline] pub fn mcr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x20) as *const u32
    }

    #[doc="Get the *mut pointer for the MCR register."]
    #[inline] pub fn mcr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x20) as *mut u32
    }

    #[doc="Read the MCR register."]
    #[inline] pub fn mcr(&self) -> Mcr { 
        unsafe {
            Mcr(read_volatile((self.0 + 0x20) as *const u32))
        }
    }

    #[doc="Write the MCR register."]
    #[inline] pub fn set_mcr<F: FnOnce(Mcr) -> Mcr>(&self, f: F) -> &Self {
        let value = f(Mcr(0));
        unsafe {
            write_volatile((self.0 + 0x20) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the MCR register."]
    #[inline] pub fn with_mcr<F: FnOnce(Mcr) -> Mcr>(&self, f: F) -> &Self {
        let tmp = self.mcr();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x20) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the MCLKOCNT register."]
    #[inline] pub fn mclkocnt_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x24) as *const u32
    }

    #[doc="Get the *mut pointer for the MCLKOCNT register."]
    #[inline] pub fn mclkocnt_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x24) as *mut u32
    }

    #[doc="Read the MCLKOCNT register."]
    #[inline] pub fn mclkocnt(&self) -> Mclkocnt { 
        unsafe {
            Mclkocnt(read_volatile((self.0 + 0x24) as *const u32))
        }
    }

    #[doc="Write the MCLKOCNT register."]
    #[inline] pub fn set_mclkocnt<F: FnOnce(Mclkocnt) -> Mclkocnt>(&self, f: F) -> &Self {
        let value = f(Mclkocnt(0));
        unsafe {
            write_volatile((self.0 + 0x24) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the MCLKOCNT register."]
    #[inline] pub fn with_mclkocnt<F: FnOnce(Mclkocnt) -> Mclkocnt>(&self, f: F) -> &Self {
        let tmp = self.mclkocnt();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x24) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the MBMON register."]
    #[inline] pub fn mbmon_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x2c) as *const u32
    }

    #[doc="Get the *mut pointer for the MBMON register."]
    #[inline] pub fn mbmon_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x2c) as *mut u32
    }

    #[doc="Read the MBMON register."]
    #[inline] pub fn mbmon(&self) -> Mbmon { 
        unsafe {
            Mbmon(read_volatile((self.0 + 0x2c) as *const u32))
        }
    }

    #[doc="Write the MBMON register."]
    #[inline] pub fn set_mbmon<F: FnOnce(Mbmon) -> Mbmon>(&self, f: F) -> &Self {
        let value = f(Mbmon(0));
        unsafe {
            write_volatile((self.0 + 0x2c) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the MBMON register."]
    #[inline] pub fn with_mbmon<F: FnOnce(Mbmon) -> Mbmon>(&self, f: F) -> &Self {
        let tmp = self.mbmon();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x2c) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the MBLEN register."]
    #[inline] pub fn mblen_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x30) as *const u32
    }

    #[doc="Get the *mut pointer for the MBLEN register."]
    #[inline] pub fn mblen_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x30) as *mut u32
    }

    #[doc="Read the MBLEN register."]
    #[inline] pub fn mblen(&self) -> Mblen { 
        unsafe {
            Mblen(read_volatile((self.0 + 0x30) as *const u32))
        }
    }

    #[doc="Write the MBLEN register."]
    #[inline] pub fn set_mblen<F: FnOnce(Mblen) -> Mblen>(&self, f: F) -> &Self {
        let value = f(Mblen(0));
        unsafe {
            write_volatile((self.0 + 0x30) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the MBLEN register."]
    #[inline] pub fn with_mblen<F: FnOnce(Mblen) -> Mblen>(&self, f: F) -> &Self {
        let tmp = self.mblen();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x30) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the MBCNT register."]
    #[inline] pub fn mbcnt_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x34) as *const u32
    }

    #[doc="Get the *mut pointer for the MBCNT register."]
    #[inline] pub fn mbcnt_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x34) as *mut u32
    }

    #[doc="Read the MBCNT register."]
    #[inline] pub fn mbcnt(&self) -> Mbcnt { 
        unsafe {
            Mbcnt(read_volatile((self.0 + 0x34) as *const u32))
        }
    }

    #[doc="Write the MBCNT register."]
    #[inline] pub fn set_mbcnt<F: FnOnce(Mbcnt) -> Mbcnt>(&self, f: F) -> &Self {
        let value = f(Mbcnt(0));
        unsafe {
            write_volatile((self.0 + 0x34) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the MBCNT register."]
    #[inline] pub fn with_mbcnt<F: FnOnce(Mbcnt) -> Mbcnt>(&self, f: F) -> &Self {
        let tmp = self.mbcnt();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x34) as *mut u32, value.0);
        }
        self
    }

}

#[doc="I2C Master Slave Address"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Msa(pub u32);
impl Msa {
    #[doc="Receive not send"]
    #[inline] pub fn rs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Receive not send"]
    #[inline] pub fn test_rs(&self) -> bool {
        self.rs != 0
    }

    #[doc="Receive not send"]
    #[inline] pub fn set_rs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="I2C Slave Address"]
    #[inline] pub fn sa(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x7f) as u8) } // [7:1]
    }

    #[doc="I2C Slave Address"]
    #[inline] pub fn test_sa(&self) -> bool {
        self.sa != 0
    }

    #[doc="I2C Slave Address"]
    #[inline] pub fn set_sa<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 1);
        self.0 |= value << 1;
        self
    }

}

impl ::core::fmt::Display for Msa {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Msa {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rs() != 0 { try!(write!(f, " rs"))}
        if self.sa() != 0 { try!(write!(f, " sa=0x{:x}", self.sa()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Master Control/Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct McsWrite(pub u32);
impl McsWrite {
    #[doc="I2C Master Enable"]
    #[inline] pub fn run(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="I2C Master Enable"]
    #[inline] pub fn test_run(&self) -> bool {
        self.run != 0
    }

    #[doc="I2C Master Enable"]
    #[inline] pub fn set_run<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Generate START"]
    #[inline] pub fn start(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Generate START"]
    #[inline] pub fn test_start(&self) -> bool {
        self.start != 0
    }

    #[doc="Generate START"]
    #[inline] pub fn set_start<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Generate STOP"]
    #[inline] pub fn stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Generate STOP"]
    #[inline] pub fn test_stop(&self) -> bool {
        self.stop != 0
    }

    #[doc="Generate STOP"]
    #[inline] pub fn set_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Data Acknowledge Enable"]
    #[inline] pub fn ack(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Data Acknowledge Enable"]
    #[inline] pub fn test_ack(&self) -> bool {
        self.ack != 0
    }

    #[doc="Data Acknowledge Enable"]
    #[inline] pub fn set_ack<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="High Speed"]
    #[inline] pub fn hs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="High Speed"]
    #[inline] pub fn test_hs(&self) -> bool {
        self.hs != 0
    }

    #[doc="High Speed"]
    #[inline] pub fn set_hs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Quick Command"]
    #[inline] pub fn qcmd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Quick Command"]
    #[inline] pub fn test_qcmd(&self) -> bool {
        self.qcmd != 0
    }

    #[doc="Quick Command"]
    #[inline] pub fn set_qcmd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Burst Enable"]
    #[inline] pub fn burst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Burst Enable"]
    #[inline] pub fn test_burst(&self) -> bool {
        self.burst != 0
    }

    #[doc="Burst Enable"]
    #[inline] pub fn set_burst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

}

impl ::core::fmt::Display for McsWrite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for McsWrite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.run() != 0 { try!(write!(f, " run"))}
        if self.start() != 0 { try!(write!(f, " start"))}
        if self.stop() != 0 { try!(write!(f, " stop"))}
        if self.ack() != 0 { try!(write!(f, " ack"))}
        if self.hs() != 0 { try!(write!(f, " hs"))}
        if self.qcmd() != 0 { try!(write!(f, " qcmd"))}
        if self.burst() != 0 { try!(write!(f, " burst"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Master Control/Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct McsRead(pub u32);
impl McsRead {
    #[doc="I2C Busy"]
    #[inline] pub fn busy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="I2C Busy"]
    #[inline] pub fn test_busy(&self) -> bool {
        self.busy != 0
    }

    #[doc="I2C Busy"]
    #[inline] pub fn set_busy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Error"]
    #[inline] pub fn error(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Error"]
    #[inline] pub fn test_error(&self) -> bool {
        self.error != 0
    }

    #[doc="Error"]
    #[inline] pub fn set_error<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Acknowledge Address"]
    #[inline] pub fn adrack(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Acknowledge Address"]
    #[inline] pub fn test_adrack(&self) -> bool {
        self.adrack != 0
    }

    #[doc="Acknowledge Address"]
    #[inline] pub fn set_adrack<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Acknowledge Data"]
    #[inline] pub fn datack(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Acknowledge Data"]
    #[inline] pub fn test_datack(&self) -> bool {
        self.datack != 0
    }

    #[doc="Acknowledge Data"]
    #[inline] pub fn set_datack<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Arbitration Lost"]
    #[inline] pub fn arblst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Arbitration Lost"]
    #[inline] pub fn test_arblst(&self) -> bool {
        self.arblst != 0
    }

    #[doc="Arbitration Lost"]
    #[inline] pub fn set_arblst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="I2C Idle"]
    #[inline] pub fn idle(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="I2C Idle"]
    #[inline] pub fn test_idle(&self) -> bool {
        self.idle != 0
    }

    #[doc="I2C Idle"]
    #[inline] pub fn set_idle<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Bus Busy"]
    #[inline] pub fn busbsy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Bus Busy"]
    #[inline] pub fn test_busbsy(&self) -> bool {
        self.busbsy != 0
    }

    #[doc="Bus Busy"]
    #[inline] pub fn set_busbsy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Clock Timeout Error"]
    #[inline] pub fn clkto(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Clock Timeout Error"]
    #[inline] pub fn test_clkto(&self) -> bool {
        self.clkto != 0
    }

    #[doc="Clock Timeout Error"]
    #[inline] pub fn set_clkto<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="DMA TX Active Status"]
    #[inline] pub fn actdmatx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="DMA TX Active Status"]
    #[inline] pub fn test_actdmatx(&self) -> bool {
        self.actdmatx != 0
    }

    #[doc="DMA TX Active Status"]
    #[inline] pub fn set_actdmatx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="DMA RX Active Status"]
    #[inline] pub fn actdmarx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="DMA RX Active Status"]
    #[inline] pub fn test_actdmarx(&self) -> bool {
        self.actdmarx != 0
    }

    #[doc="DMA RX Active Status"]
    #[inline] pub fn set_actdmarx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl ::core::fmt::Display for McsRead {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for McsRead {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.busy() != 0 { try!(write!(f, " busy"))}
        if self.error() != 0 { try!(write!(f, " error"))}
        if self.adrack() != 0 { try!(write!(f, " adrack"))}
        if self.datack() != 0 { try!(write!(f, " datack"))}
        if self.arblst() != 0 { try!(write!(f, " arblst"))}
        if self.idle() != 0 { try!(write!(f, " idle"))}
        if self.busbsy() != 0 { try!(write!(f, " busbsy"))}
        if self.clkto() != 0 { try!(write!(f, " clkto"))}
        if self.actdmatx() != 0 { try!(write!(f, " actdmatx"))}
        if self.actdmarx() != 0 { try!(write!(f, " actdmarx"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Master Data"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mdr(pub u32);
impl Mdr {
    #[doc="This byte contains the data transferred during a transaction"]
    #[inline] pub fn data(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="This byte contains the data transferred during a transaction"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data != 0
    }

    #[doc="This byte contains the data transferred during a transaction"]
    #[inline] pub fn set_data<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Mdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.data() != 0 { try!(write!(f, " data=0x{:x}", self.data()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Master Timer Period"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mtpr(pub u32);
impl Mtpr {
    #[doc="Timer Period"]
    #[inline] pub fn tpr(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="Timer Period"]
    #[inline] pub fn test_tpr(&self) -> bool {
        self.tpr != 0
    }

    #[doc="Timer Period"]
    #[inline] pub fn set_tpr<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="High-Speed Enable"]
    #[inline] pub fn hs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="High-Speed Enable"]
    #[inline] pub fn test_hs(&self) -> bool {
        self.hs != 0
    }

    #[doc="High-Speed Enable"]
    #[inline] pub fn set_hs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Glitch Suppression Pulse Width"]
    #[inline] pub fn pulsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7) as u8) } // [18:16]
    }

    #[doc="Glitch Suppression Pulse Width"]
    #[inline] pub fn test_pulsel(&self) -> bool {
        self.pulsel != 0
    }

    #[doc="Glitch Suppression Pulse Width"]
    #[inline] pub fn set_pulsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 16);
        self.0 |= value << 16;
        self
    }

}

impl ::core::fmt::Display for Mtpr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mtpr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tpr() != 0 { try!(write!(f, " tpr=0x{:x}", self.tpr()))}
        if self.hs() != 0 { try!(write!(f, " hs"))}
        if self.pulsel() != 0 { try!(write!(f, " pulsel=0x{:x}", self.pulsel()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Master Interrupt Mask"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mimr(pub u32);
impl Mimr {
    #[doc="Master Interrupt Mask"]
    #[inline] pub fn im(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Master Interrupt Mask"]
    #[inline] pub fn test_im(&self) -> bool {
        self.im != 0
    }

    #[doc="Master Interrupt Mask"]
    #[inline] pub fn set_im<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Clock Timeout Interrupt Mask"]
    #[inline] pub fn clkim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Clock Timeout Interrupt Mask"]
    #[inline] pub fn test_clkim(&self) -> bool {
        self.clkim != 0
    }

    #[doc="Clock Timeout Interrupt Mask"]
    #[inline] pub fn set_clkim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Receive DMA Interrupt Mask"]
    #[inline] pub fn dmarxim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Receive DMA Interrupt Mask"]
    #[inline] pub fn test_dmarxim(&self) -> bool {
        self.dmarxim != 0
    }

    #[doc="Receive DMA Interrupt Mask"]
    #[inline] pub fn set_dmarxim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Transmit DMA Interrupt Mask"]
    #[inline] pub fn dmatxim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Transmit DMA Interrupt Mask"]
    #[inline] pub fn test_dmatxim(&self) -> bool {
        self.dmatxim != 0
    }

    #[doc="Transmit DMA Interrupt Mask"]
    #[inline] pub fn set_dmatxim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Address/Data NACK Interrupt Mask"]
    #[inline] pub fn nackim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Address/Data NACK Interrupt Mask"]
    #[inline] pub fn test_nackim(&self) -> bool {
        self.nackim != 0
    }

    #[doc="Address/Data NACK Interrupt Mask"]
    #[inline] pub fn set_nackim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="START Detection Interrupt Mask"]
    #[inline] pub fn startim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="START Detection Interrupt Mask"]
    #[inline] pub fn test_startim(&self) -> bool {
        self.startim != 0
    }

    #[doc="START Detection Interrupt Mask"]
    #[inline] pub fn set_startim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="STOP Detection Interrupt Mask"]
    #[inline] pub fn stopim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="STOP Detection Interrupt Mask"]
    #[inline] pub fn test_stopim(&self) -> bool {
        self.stopim != 0
    }

    #[doc="STOP Detection Interrupt Mask"]
    #[inline] pub fn set_stopim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Arbitration Lost Interrupt Mask"]
    #[inline] pub fn arblostim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Arbitration Lost Interrupt Mask"]
    #[inline] pub fn test_arblostim(&self) -> bool {
        self.arblostim != 0
    }

    #[doc="Arbitration Lost Interrupt Mask"]
    #[inline] pub fn set_arblostim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Transmit FIFO Request Interrupt Mask"]
    #[inline] pub fn txim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Transmit FIFO Request Interrupt Mask"]
    #[inline] pub fn test_txim(&self) -> bool {
        self.txim != 0
    }

    #[doc="Transmit FIFO Request Interrupt Mask"]
    #[inline] pub fn set_txim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Receive FIFO Request Interrupt Mask"]
    #[inline] pub fn rxim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Receive FIFO Request Interrupt Mask"]
    #[inline] pub fn test_rxim(&self) -> bool {
        self.rxim != 0
    }

    #[doc="Receive FIFO Request Interrupt Mask"]
    #[inline] pub fn set_rxim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Transmit FIFO Empty Interrupt Mask"]
    #[inline] pub fn txfeim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Transmit FIFO Empty Interrupt Mask"]
    #[inline] pub fn test_txfeim(&self) -> bool {
        self.txfeim != 0
    }

    #[doc="Transmit FIFO Empty Interrupt Mask"]
    #[inline] pub fn set_txfeim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Receive FIFO Full Interrupt Mask"]
    #[inline] pub fn rxffim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Receive FIFO Full Interrupt Mask"]
    #[inline] pub fn test_rxffim(&self) -> bool {
        self.rxffim != 0
    }

    #[doc="Receive FIFO Full Interrupt Mask"]
    #[inline] pub fn set_rxffim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

}

impl ::core::fmt::Display for Mimr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mimr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.im() != 0 { try!(write!(f, " im"))}
        if self.clkim() != 0 { try!(write!(f, " clkim"))}
        if self.dmarxim() != 0 { try!(write!(f, " dmarxim"))}
        if self.dmatxim() != 0 { try!(write!(f, " dmatxim"))}
        if self.nackim() != 0 { try!(write!(f, " nackim"))}
        if self.startim() != 0 { try!(write!(f, " startim"))}
        if self.stopim() != 0 { try!(write!(f, " stopim"))}
        if self.arblostim() != 0 { try!(write!(f, " arblostim"))}
        if self.txim() != 0 { try!(write!(f, " txim"))}
        if self.rxim() != 0 { try!(write!(f, " rxim"))}
        if self.txfeim() != 0 { try!(write!(f, " txfeim"))}
        if self.rxffim() != 0 { try!(write!(f, " rxffim"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Master Raw Interrupt Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mris(pub u32);
impl Mris {
    #[doc="Master Raw Interrupt Status"]
    #[inline] pub fn ris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Master Raw Interrupt Status"]
    #[inline] pub fn test_ris(&self) -> bool {
        self.ris != 0
    }

    #[doc="Master Raw Interrupt Status"]
    #[inline] pub fn set_ris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Clock Timeout Raw Interrupt Status"]
    #[inline] pub fn clkris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Clock Timeout Raw Interrupt Status"]
    #[inline] pub fn test_clkris(&self) -> bool {
        self.clkris != 0
    }

    #[doc="Clock Timeout Raw Interrupt Status"]
    #[inline] pub fn set_clkris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Receive DMA Raw Interrupt Status"]
    #[inline] pub fn dmarxris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Receive DMA Raw Interrupt Status"]
    #[inline] pub fn test_dmarxris(&self) -> bool {
        self.dmarxris != 0
    }

    #[doc="Receive DMA Raw Interrupt Status"]
    #[inline] pub fn set_dmarxris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Transmit DMA Raw Interrupt Status"]
    #[inline] pub fn dmatxris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Transmit DMA Raw Interrupt Status"]
    #[inline] pub fn test_dmatxris(&self) -> bool {
        self.dmatxris != 0
    }

    #[doc="Transmit DMA Raw Interrupt Status"]
    #[inline] pub fn set_dmatxris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Address/Data NACK Raw Interrupt Status"]
    #[inline] pub fn nackris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Address/Data NACK Raw Interrupt Status"]
    #[inline] pub fn test_nackris(&self) -> bool {
        self.nackris != 0
    }

    #[doc="Address/Data NACK Raw Interrupt Status"]
    #[inline] pub fn set_nackris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="START Detection Raw Interrupt Status"]
    #[inline] pub fn startris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="START Detection Raw Interrupt Status"]
    #[inline] pub fn test_startris(&self) -> bool {
        self.startris != 0
    }

    #[doc="START Detection Raw Interrupt Status"]
    #[inline] pub fn set_startris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="STOP Detection Raw Interrupt Status"]
    #[inline] pub fn stopris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="STOP Detection Raw Interrupt Status"]
    #[inline] pub fn test_stopris(&self) -> bool {
        self.stopris != 0
    }

    #[doc="STOP Detection Raw Interrupt Status"]
    #[inline] pub fn set_stopris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Arbitration Lost Raw Interrupt Status"]
    #[inline] pub fn arblostris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Arbitration Lost Raw Interrupt Status"]
    #[inline] pub fn test_arblostris(&self) -> bool {
        self.arblostris != 0
    }

    #[doc="Arbitration Lost Raw Interrupt Status"]
    #[inline] pub fn set_arblostris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Transmit Request Raw Interrupt Status"]
    #[inline] pub fn txris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Transmit Request Raw Interrupt Status"]
    #[inline] pub fn test_txris(&self) -> bool {
        self.txris != 0
    }

    #[doc="Transmit Request Raw Interrupt Status"]
    #[inline] pub fn set_txris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Receive FIFO Request Raw Interrupt Status"]
    #[inline] pub fn rxris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Receive FIFO Request Raw Interrupt Status"]
    #[inline] pub fn test_rxris(&self) -> bool {
        self.rxris != 0
    }

    #[doc="Receive FIFO Request Raw Interrupt Status"]
    #[inline] pub fn set_rxris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Transmit FIFO Empty Raw Interrupt Status"]
    #[inline] pub fn txferis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Transmit FIFO Empty Raw Interrupt Status"]
    #[inline] pub fn test_txferis(&self) -> bool {
        self.txferis != 0
    }

    #[doc="Transmit FIFO Empty Raw Interrupt Status"]
    #[inline] pub fn set_txferis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Receive FIFO Full Raw Interrupt Status"]
    #[inline] pub fn rxffris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Receive FIFO Full Raw Interrupt Status"]
    #[inline] pub fn test_rxffris(&self) -> bool {
        self.rxffris != 0
    }

    #[doc="Receive FIFO Full Raw Interrupt Status"]
    #[inline] pub fn set_rxffris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

}

impl ::core::fmt::Display for Mris {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mris {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ris() != 0 { try!(write!(f, " ris"))}
        if self.clkris() != 0 { try!(write!(f, " clkris"))}
        if self.dmarxris() != 0 { try!(write!(f, " dmarxris"))}
        if self.dmatxris() != 0 { try!(write!(f, " dmatxris"))}
        if self.nackris() != 0 { try!(write!(f, " nackris"))}
        if self.startris() != 0 { try!(write!(f, " startris"))}
        if self.stopris() != 0 { try!(write!(f, " stopris"))}
        if self.arblostris() != 0 { try!(write!(f, " arblostris"))}
        if self.txris() != 0 { try!(write!(f, " txris"))}
        if self.rxris() != 0 { try!(write!(f, " rxris"))}
        if self.txferis() != 0 { try!(write!(f, " txferis"))}
        if self.rxffris() != 0 { try!(write!(f, " rxffris"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Master Masked Interrupt Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mmis(pub u32);
impl Mmis {
    #[doc="Masked Interrupt Status"]
    #[inline] pub fn mis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Masked Interrupt Status"]
    #[inline] pub fn test_mis(&self) -> bool {
        self.mis != 0
    }

    #[doc="Masked Interrupt Status"]
    #[inline] pub fn set_mis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Clock Timeout Masked Interrupt Status"]
    #[inline] pub fn clkmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Clock Timeout Masked Interrupt Status"]
    #[inline] pub fn test_clkmis(&self) -> bool {
        self.clkmis != 0
    }

    #[doc="Clock Timeout Masked Interrupt Status"]
    #[inline] pub fn set_clkmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Receive DMA Interrupt Status"]
    #[inline] pub fn dmarxmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Receive DMA Interrupt Status"]
    #[inline] pub fn test_dmarxmis(&self) -> bool {
        self.dmarxmis != 0
    }

    #[doc="Receive DMA Interrupt Status"]
    #[inline] pub fn set_dmarxmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Transmit DMA Interrupt Status"]
    #[inline] pub fn dmatxmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Transmit DMA Interrupt Status"]
    #[inline] pub fn test_dmatxmis(&self) -> bool {
        self.dmatxmis != 0
    }

    #[doc="Transmit DMA Interrupt Status"]
    #[inline] pub fn set_dmatxmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Address/Data NACK Interrupt Mask"]
    #[inline] pub fn nackmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Address/Data NACK Interrupt Mask"]
    #[inline] pub fn test_nackmis(&self) -> bool {
        self.nackmis != 0
    }

    #[doc="Address/Data NACK Interrupt Mask"]
    #[inline] pub fn set_nackmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="START Detection Interrupt Mask"]
    #[inline] pub fn startmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="START Detection Interrupt Mask"]
    #[inline] pub fn test_startmis(&self) -> bool {
        self.startmis != 0
    }

    #[doc="START Detection Interrupt Mask"]
    #[inline] pub fn set_startmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="STOP Detection Interrupt Mask"]
    #[inline] pub fn stopmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="STOP Detection Interrupt Mask"]
    #[inline] pub fn test_stopmis(&self) -> bool {
        self.stopmis != 0
    }

    #[doc="STOP Detection Interrupt Mask"]
    #[inline] pub fn set_stopmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Arbitration Lost Interrupt Mask"]
    #[inline] pub fn arblostmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Arbitration Lost Interrupt Mask"]
    #[inline] pub fn test_arblostmis(&self) -> bool {
        self.arblostmis != 0
    }

    #[doc="Arbitration Lost Interrupt Mask"]
    #[inline] pub fn set_arblostmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Transmit Request Interrupt Mask"]
    #[inline] pub fn txmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Transmit Request Interrupt Mask"]
    #[inline] pub fn test_txmis(&self) -> bool {
        self.txmis != 0
    }

    #[doc="Transmit Request Interrupt Mask"]
    #[inline] pub fn set_txmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Receive FIFO Request Interrupt Mask"]
    #[inline] pub fn rxmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Receive FIFO Request Interrupt Mask"]
    #[inline] pub fn test_rxmis(&self) -> bool {
        self.rxmis != 0
    }

    #[doc="Receive FIFO Request Interrupt Mask"]
    #[inline] pub fn set_rxmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Transmit FIFO Empty Interrupt Mask"]
    #[inline] pub fn txfemis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Transmit FIFO Empty Interrupt Mask"]
    #[inline] pub fn test_txfemis(&self) -> bool {
        self.txfemis != 0
    }

    #[doc="Transmit FIFO Empty Interrupt Mask"]
    #[inline] pub fn set_txfemis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Receive FIFO Full Interrupt Mask"]
    #[inline] pub fn rxffmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Receive FIFO Full Interrupt Mask"]
    #[inline] pub fn test_rxffmis(&self) -> bool {
        self.rxffmis != 0
    }

    #[doc="Receive FIFO Full Interrupt Mask"]
    #[inline] pub fn set_rxffmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

}

impl ::core::fmt::Display for Mmis {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mmis {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mis() != 0 { try!(write!(f, " mis"))}
        if self.clkmis() != 0 { try!(write!(f, " clkmis"))}
        if self.dmarxmis() != 0 { try!(write!(f, " dmarxmis"))}
        if self.dmatxmis() != 0 { try!(write!(f, " dmatxmis"))}
        if self.nackmis() != 0 { try!(write!(f, " nackmis"))}
        if self.startmis() != 0 { try!(write!(f, " startmis"))}
        if self.stopmis() != 0 { try!(write!(f, " stopmis"))}
        if self.arblostmis() != 0 { try!(write!(f, " arblostmis"))}
        if self.txmis() != 0 { try!(write!(f, " txmis"))}
        if self.rxmis() != 0 { try!(write!(f, " rxmis"))}
        if self.txfemis() != 0 { try!(write!(f, " txfemis"))}
        if self.rxffmis() != 0 { try!(write!(f, " rxffmis"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Master Interrupt Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Micr(pub u32);
impl Micr {
    #[doc="Master Interrupt Clear"]
    #[inline] pub fn ic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Master Interrupt Clear"]
    #[inline] pub fn test_ic(&self) -> bool {
        self.ic != 0
    }

    #[doc="Master Interrupt Clear"]
    #[inline] pub fn set_ic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Clock Timeout Interrupt Clear"]
    #[inline] pub fn clkic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Clock Timeout Interrupt Clear"]
    #[inline] pub fn test_clkic(&self) -> bool {
        self.clkic != 0
    }

    #[doc="Clock Timeout Interrupt Clear"]
    #[inline] pub fn set_clkic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Receive DMA Interrupt Clear"]
    #[inline] pub fn dmarxic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Receive DMA Interrupt Clear"]
    #[inline] pub fn test_dmarxic(&self) -> bool {
        self.dmarxic != 0
    }

    #[doc="Receive DMA Interrupt Clear"]
    #[inline] pub fn set_dmarxic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Transmit DMA Interrupt Clear"]
    #[inline] pub fn dmatxic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Transmit DMA Interrupt Clear"]
    #[inline] pub fn test_dmatxic(&self) -> bool {
        self.dmatxic != 0
    }

    #[doc="Transmit DMA Interrupt Clear"]
    #[inline] pub fn set_dmatxic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Address/Data NACK Interrupt Clear"]
    #[inline] pub fn nackic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Address/Data NACK Interrupt Clear"]
    #[inline] pub fn test_nackic(&self) -> bool {
        self.nackic != 0
    }

    #[doc="Address/Data NACK Interrupt Clear"]
    #[inline] pub fn set_nackic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="START Detection Interrupt Clear"]
    #[inline] pub fn startic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="START Detection Interrupt Clear"]
    #[inline] pub fn test_startic(&self) -> bool {
        self.startic != 0
    }

    #[doc="START Detection Interrupt Clear"]
    #[inline] pub fn set_startic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="STOP Detection Interrupt Clear"]
    #[inline] pub fn stopic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="STOP Detection Interrupt Clear"]
    #[inline] pub fn test_stopic(&self) -> bool {
        self.stopic != 0
    }

    #[doc="STOP Detection Interrupt Clear"]
    #[inline] pub fn set_stopic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Arbitration Lost Interrupt Clear"]
    #[inline] pub fn arblostic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Arbitration Lost Interrupt Clear"]
    #[inline] pub fn test_arblostic(&self) -> bool {
        self.arblostic != 0
    }

    #[doc="Arbitration Lost Interrupt Clear"]
    #[inline] pub fn set_arblostic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Transmit FIFO Request Interrupt Clear"]
    #[inline] pub fn txic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Transmit FIFO Request Interrupt Clear"]
    #[inline] pub fn test_txic(&self) -> bool {
        self.txic != 0
    }

    #[doc="Transmit FIFO Request Interrupt Clear"]
    #[inline] pub fn set_txic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Receive FIFO Request Interrupt Clear"]
    #[inline] pub fn rxic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Receive FIFO Request Interrupt Clear"]
    #[inline] pub fn test_rxic(&self) -> bool {
        self.rxic != 0
    }

    #[doc="Receive FIFO Request Interrupt Clear"]
    #[inline] pub fn set_rxic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Transmit FIFO Empty Interrupt Clear"]
    #[inline] pub fn txfeic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Transmit FIFO Empty Interrupt Clear"]
    #[inline] pub fn test_txfeic(&self) -> bool {
        self.txfeic != 0
    }

    #[doc="Transmit FIFO Empty Interrupt Clear"]
    #[inline] pub fn set_txfeic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Receive FIFO Full Interrupt Clear"]
    #[inline] pub fn rxffic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Receive FIFO Full Interrupt Clear"]
    #[inline] pub fn test_rxffic(&self) -> bool {
        self.rxffic != 0
    }

    #[doc="Receive FIFO Full Interrupt Clear"]
    #[inline] pub fn set_rxffic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

}

impl ::core::fmt::Display for Micr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Micr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ic() != 0 { try!(write!(f, " ic"))}
        if self.clkic() != 0 { try!(write!(f, " clkic"))}
        if self.dmarxic() != 0 { try!(write!(f, " dmarxic"))}
        if self.dmatxic() != 0 { try!(write!(f, " dmatxic"))}
        if self.nackic() != 0 { try!(write!(f, " nackic"))}
        if self.startic() != 0 { try!(write!(f, " startic"))}
        if self.stopic() != 0 { try!(write!(f, " stopic"))}
        if self.arblostic() != 0 { try!(write!(f, " arblostic"))}
        if self.txic() != 0 { try!(write!(f, " txic"))}
        if self.rxic() != 0 { try!(write!(f, " rxic"))}
        if self.txfeic() != 0 { try!(write!(f, " txfeic"))}
        if self.rxffic() != 0 { try!(write!(f, " rxffic"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Master Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mcr(pub u32);
impl Mcr {
    #[doc="I2C Loopback"]
    #[inline] pub fn lpbk(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="I2C Loopback"]
    #[inline] pub fn test_lpbk(&self) -> bool {
        self.lpbk != 0
    }

    #[doc="I2C Loopback"]
    #[inline] pub fn set_lpbk<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="I2C Master Function Enable"]
    #[inline] pub fn mfe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="I2C Master Function Enable"]
    #[inline] pub fn test_mfe(&self) -> bool {
        self.mfe != 0
    }

    #[doc="I2C Master Function Enable"]
    #[inline] pub fn set_mfe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="I2C Slave Function Enable"]
    #[inline] pub fn sfe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="I2C Slave Function Enable"]
    #[inline] pub fn test_sfe(&self) -> bool {
        self.sfe != 0
    }

    #[doc="I2C Slave Function Enable"]
    #[inline] pub fn set_sfe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

}

impl ::core::fmt::Display for Mcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lpbk() != 0 { try!(write!(f, " lpbk"))}
        if self.mfe() != 0 { try!(write!(f, " mfe"))}
        if self.sfe() != 0 { try!(write!(f, " sfe"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Master Clock Low Timeout Count"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mclkocnt(pub u32);
impl Mclkocnt {
    #[doc="I2C Master Count"]
    #[inline] pub fn cntl(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="I2C Master Count"]
    #[inline] pub fn test_cntl(&self) -> bool {
        self.cntl != 0
    }

    #[doc="I2C Master Count"]
    #[inline] pub fn set_cntl<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Mclkocnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mclkocnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cntl() != 0 { try!(write!(f, " cntl=0x{:x}", self.cntl()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Master Bus Monitor"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mbmon(pub u32);
impl Mbmon {
    #[doc="I2C SCL Status"]
    #[inline] pub fn scl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="I2C SCL Status"]
    #[inline] pub fn test_scl(&self) -> bool {
        self.scl != 0
    }

    #[doc="I2C SCL Status"]
    #[inline] pub fn set_scl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="I2C SDA Status"]
    #[inline] pub fn sda(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="I2C SDA Status"]
    #[inline] pub fn test_sda(&self) -> bool {
        self.sda != 0
    }

    #[doc="I2C SDA Status"]
    #[inline] pub fn set_sda<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl ::core::fmt::Display for Mbmon {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mbmon {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.scl() != 0 { try!(write!(f, " scl"))}
        if self.sda() != 0 { try!(write!(f, " sda"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Master Burst Length"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mblen(pub u32);
impl Mblen {
    #[doc="I2C Burst Length"]
    #[inline] pub fn cntl(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="I2C Burst Length"]
    #[inline] pub fn test_cntl(&self) -> bool {
        self.cntl != 0
    }

    #[doc="I2C Burst Length"]
    #[inline] pub fn set_cntl<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Mblen {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mblen {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cntl() != 0 { try!(write!(f, " cntl=0x{:x}", self.cntl()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Master Burst Count"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mbcnt(pub u32);
impl Mbcnt {
    #[doc="I2C Master Burst Count"]
    #[inline] pub fn cntl(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="I2C Master Burst Count"]
    #[inline] pub fn test_cntl(&self) -> bool {
        self.cntl != 0
    }

    #[doc="I2C Master Burst Count"]
    #[inline] pub fn set_cntl<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Mbcnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mbcnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cntl() != 0 { try!(write!(f, " cntl=0x{:x}", self.cntl()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

}
// End of master

pub mod slave {
#[allow(unused_imports)] use bobbin_common::*;
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Slave(pub usize);
impl Slave {
    #[doc="Get the *const pointer for the SOAR register."]
    #[inline] pub fn soar_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x800) as *const u32
    }

    #[doc="Get the *mut pointer for the SOAR register."]
    #[inline] pub fn soar_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x800) as *mut u32
    }

    #[doc="Read the SOAR register."]
    #[inline] pub fn soar(&self) -> Soar { 
        unsafe {
            Soar(read_volatile((self.0 + 0x800) as *const u32))
        }
    }

    #[doc="Write the SOAR register."]
    #[inline] pub fn set_soar<F: FnOnce(Soar) -> Soar>(&self, f: F) -> &Self {
        let value = f(Soar(0));
        unsafe {
            write_volatile((self.0 + 0x800) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the SOAR register."]
    #[inline] pub fn with_soar<F: FnOnce(Soar) -> Soar>(&self, f: F) -> &Self {
        let tmp = self.soar();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x800) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the SCSR_READ register."]
    #[inline] pub fn scsr_read_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x804) as *const u32
    }

    #[doc="Get the *mut pointer for the SCSR_READ register."]
    #[inline] pub fn scsr_read_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x804) as *mut u32
    }

    #[doc="Read the SCSR_READ register."]
    #[inline] pub fn scsr_read(&self) -> ScsrRead { 
        unsafe {
            ScsrRead(read_volatile((self.0 + 0x804) as *const u32))
        }
    }

    #[doc="Write the SCSR_READ register."]
    #[inline] pub fn set_scsr_read<F: FnOnce(ScsrRead) -> ScsrRead>(&self, f: F) -> &Self {
        let value = f(ScsrRead(0));
        unsafe {
            write_volatile((self.0 + 0x804) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the SCSR_READ register."]
    #[inline] pub fn with_scsr_read<F: FnOnce(ScsrRead) -> ScsrRead>(&self, f: F) -> &Self {
        let tmp = self.scsr_read();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x804) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the SCSR_WRITE register."]
    #[inline] pub fn scsr_write_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x804) as *const u32
    }

    #[doc="Get the *mut pointer for the SCSR_WRITE register."]
    #[inline] pub fn scsr_write_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x804) as *mut u32
    }

    #[doc="Read the SCSR_WRITE register."]
    #[inline] pub fn scsr_write(&self) -> ScsrWrite { 
        unsafe {
            ScsrWrite(read_volatile((self.0 + 0x804) as *const u32))
        }
    }

    #[doc="Write the SCSR_WRITE register."]
    #[inline] pub fn set_scsr_write<F: FnOnce(ScsrWrite) -> ScsrWrite>(&self, f: F) -> &Self {
        let value = f(ScsrWrite(0));
        unsafe {
            write_volatile((self.0 + 0x804) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the SCSR_WRITE register."]
    #[inline] pub fn with_scsr_write<F: FnOnce(ScsrWrite) -> ScsrWrite>(&self, f: F) -> &Self {
        let tmp = self.scsr_write();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x804) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the SDR register."]
    #[inline] pub fn sdr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x808) as *const u32
    }

    #[doc="Get the *mut pointer for the SDR register."]
    #[inline] pub fn sdr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x808) as *mut u32
    }

    #[doc="Read the SDR register."]
    #[inline] pub fn sdr(&self) -> Sdr { 
        unsafe {
            Sdr(read_volatile((self.0 + 0x808) as *const u32))
        }
    }

    #[doc="Write the SDR register."]
    #[inline] pub fn set_sdr<F: FnOnce(Sdr) -> Sdr>(&self, f: F) -> &Self {
        let value = f(Sdr(0));
        unsafe {
            write_volatile((self.0 + 0x808) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the SDR register."]
    #[inline] pub fn with_sdr<F: FnOnce(Sdr) -> Sdr>(&self, f: F) -> &Self {
        let tmp = self.sdr();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x808) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the SIMR register."]
    #[inline] pub fn simr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x80c) as *const u32
    }

    #[doc="Get the *mut pointer for the SIMR register."]
    #[inline] pub fn simr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x80c) as *mut u32
    }

    #[doc="Read the SIMR register."]
    #[inline] pub fn simr(&self) -> Simr { 
        unsafe {
            Simr(read_volatile((self.0 + 0x80c) as *const u32))
        }
    }

    #[doc="Write the SIMR register."]
    #[inline] pub fn set_simr<F: FnOnce(Simr) -> Simr>(&self, f: F) -> &Self {
        let value = f(Simr(0));
        unsafe {
            write_volatile((self.0 + 0x80c) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the SIMR register."]
    #[inline] pub fn with_simr<F: FnOnce(Simr) -> Simr>(&self, f: F) -> &Self {
        let tmp = self.simr();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x80c) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the SRIS register."]
    #[inline] pub fn sris_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x810) as *const u32
    }

    #[doc="Get the *mut pointer for the SRIS register."]
    #[inline] pub fn sris_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x810) as *mut u32
    }

    #[doc="Read the SRIS register."]
    #[inline] pub fn sris(&self) -> Sris { 
        unsafe {
            Sris(read_volatile((self.0 + 0x810) as *const u32))
        }
    }

    #[doc="Write the SRIS register."]
    #[inline] pub fn set_sris<F: FnOnce(Sris) -> Sris>(&self, f: F) -> &Self {
        let value = f(Sris(0));
        unsafe {
            write_volatile((self.0 + 0x810) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the SRIS register."]
    #[inline] pub fn with_sris<F: FnOnce(Sris) -> Sris>(&self, f: F) -> &Self {
        let tmp = self.sris();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x810) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the SMIS register."]
    #[inline] pub fn smis_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x814) as *const u32
    }

    #[doc="Get the *mut pointer for the SMIS register."]
    #[inline] pub fn smis_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x814) as *mut u32
    }

    #[doc="Read the SMIS register."]
    #[inline] pub fn smis(&self) -> Smis { 
        unsafe {
            Smis(read_volatile((self.0 + 0x814) as *const u32))
        }
    }

    #[doc="Write the SMIS register."]
    #[inline] pub fn set_smis<F: FnOnce(Smis) -> Smis>(&self, f: F) -> &Self {
        let value = f(Smis(0));
        unsafe {
            write_volatile((self.0 + 0x814) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the SMIS register."]
    #[inline] pub fn with_smis<F: FnOnce(Smis) -> Smis>(&self, f: F) -> &Self {
        let tmp = self.smis();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x814) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the SICR register."]
    #[inline] pub fn sicr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x818) as *const u32
    }

    #[doc="Get the *mut pointer for the SICR register."]
    #[inline] pub fn sicr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x818) as *mut u32
    }

    #[doc="Write the SICR register."]
    #[inline] pub fn set_sicr<F: FnOnce(Sicr) -> Sicr>(&self, f: F) -> &Self {
        let value = f(Sicr(0));
        unsafe {
            write_volatile((self.0 + 0x818) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the SOAR2 register."]
    #[inline] pub fn soar2_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x81c) as *const u32
    }

    #[doc="Get the *mut pointer for the SOAR2 register."]
    #[inline] pub fn soar2_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x81c) as *mut u32
    }

    #[doc="Read the SOAR2 register."]
    #[inline] pub fn soar2(&self) -> Soar2 { 
        unsafe {
            Soar2(read_volatile((self.0 + 0x81c) as *const u32))
        }
    }

    #[doc="Write the SOAR2 register."]
    #[inline] pub fn set_soar2<F: FnOnce(Soar2) -> Soar2>(&self, f: F) -> &Self {
        let value = f(Soar2(0));
        unsafe {
            write_volatile((self.0 + 0x81c) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the SOAR2 register."]
    #[inline] pub fn with_soar2<F: FnOnce(Soar2) -> Soar2>(&self, f: F) -> &Self {
        let tmp = self.soar2();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x81c) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the SACKCTL register."]
    #[inline] pub fn sackctl_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x820) as *const u32
    }

    #[doc="Get the *mut pointer for the SACKCTL register."]
    #[inline] pub fn sackctl_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x820) as *mut u32
    }

    #[doc="Read the SACKCTL register."]
    #[inline] pub fn sackctl(&self) -> Sackctl { 
        unsafe {
            Sackctl(read_volatile((self.0 + 0x820) as *const u32))
        }
    }

    #[doc="Write the SACKCTL register."]
    #[inline] pub fn set_sackctl<F: FnOnce(Sackctl) -> Sackctl>(&self, f: F) -> &Self {
        let value = f(Sackctl(0));
        unsafe {
            write_volatile((self.0 + 0x820) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the SACKCTL register."]
    #[inline] pub fn with_sackctl<F: FnOnce(Sackctl) -> Sackctl>(&self, f: F) -> &Self {
        let tmp = self.sackctl();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x820) as *mut u32, value.0);
        }
        self
    }

}

#[doc="I2C Slave Own Address"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Soar(pub u32);
impl Soar {
    #[doc="I2C Slave Own Address"]
    #[inline] pub fn oar(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="I2C Slave Own Address"]
    #[inline] pub fn test_oar(&self) -> bool {
        self.oar != 0
    }

    #[doc="I2C Slave Own Address"]
    #[inline] pub fn set_oar<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Soar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Soar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.oar() != 0 { try!(write!(f, " oar=0x{:x}", self.oar()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Slave Control/Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct ScsrRead(pub u32);
impl ScsrRead {
    #[doc="Receive Request"]
    #[inline] pub fn rreq(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Receive Request"]
    #[inline] pub fn test_rreq(&self) -> bool {
        self.rreq != 0
    }

    #[doc="Receive Request"]
    #[inline] pub fn set_rreq<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="TX FIFO Enable"]
    #[inline] pub fn txfifo(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="TX FIFO Enable"]
    #[inline] pub fn test_txfifo(&self) -> bool {
        self.txfifo != 0
    }

    #[doc="TX FIFO Enable"]
    #[inline] pub fn set_txfifo<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="First Byte Received"]
    #[inline] pub fn fbr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="First Byte Received"]
    #[inline] pub fn test_fbr(&self) -> bool {
        self.fbr != 0
    }

    #[doc="First Byte Received"]
    #[inline] pub fn set_fbr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="OAR2 Address Matched"]
    #[inline] pub fn oar2sel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="OAR2 Address Matched"]
    #[inline] pub fn test_oar2sel(&self) -> bool {
        self.oar2sel != 0
    }

    #[doc="OAR2 Address Matched"]
    #[inline] pub fn set_oar2sel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Quick Command Status"]
    #[inline] pub fn qcmdst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Quick Command Status"]
    #[inline] pub fn test_qcmdst(&self) -> bool {
        self.qcmdst != 0
    }

    #[doc="Quick Command Status"]
    #[inline] pub fn set_qcmdst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Quick Command Read / Write"]
    #[inline] pub fn qcmdrw(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Quick Command Read / Write"]
    #[inline] pub fn test_qcmdrw(&self) -> bool {
        self.qcmdrw != 0
    }

    #[doc="Quick Command Read / Write"]
    #[inline] pub fn set_qcmdrw<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="DMA TX Active Status"]
    #[inline] pub fn actdmatx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="DMA TX Active Status"]
    #[inline] pub fn test_actdmatx(&self) -> bool {
        self.actdmatx != 0
    }

    #[doc="DMA TX Active Status"]
    #[inline] pub fn set_actdmatx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="DMA RX Active Status"]
    #[inline] pub fn actdmarx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="DMA RX Active Status"]
    #[inline] pub fn test_actdmarx(&self) -> bool {
        self.actdmarx != 0
    }

    #[doc="DMA RX Active Status"]
    #[inline] pub fn set_actdmarx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl ::core::fmt::Display for ScsrRead {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for ScsrRead {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rreq() != 0 { try!(write!(f, " rreq"))}
        if self.txfifo() != 0 { try!(write!(f, " txfifo"))}
        if self.fbr() != 0 { try!(write!(f, " fbr"))}
        if self.oar2sel() != 0 { try!(write!(f, " oar2sel"))}
        if self.qcmdst() != 0 { try!(write!(f, " qcmdst"))}
        if self.qcmdrw() != 0 { try!(write!(f, " qcmdrw"))}
        if self.actdmatx() != 0 { try!(write!(f, " actdmatx"))}
        if self.actdmarx() != 0 { try!(write!(f, " actdmarx"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Slave Control/Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct ScsrWrite(pub u32);
impl ScsrWrite {
    #[doc="Device Active"]
    #[inline] pub fn da(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Device Active"]
    #[inline] pub fn test_da(&self) -> bool {
        self.da != 0
    }

    #[doc="Device Active"]
    #[inline] pub fn set_da<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transmit Request"]
    #[inline] pub fn treq(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Transmit Request"]
    #[inline] pub fn test_treq(&self) -> bool {
        self.treq != 0
    }

    #[doc="Transmit Request"]
    #[inline] pub fn set_treq<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="RX FIFO Enable"]
    #[inline] pub fn rxfifo(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="RX FIFO Enable"]
    #[inline] pub fn test_rxfifo(&self) -> bool {
        self.rxfifo != 0
    }

    #[doc="RX FIFO Enable"]
    #[inline] pub fn set_rxfifo<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

}

impl ::core::fmt::Display for ScsrWrite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for ScsrWrite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.da() != 0 { try!(write!(f, " da"))}
        if self.treq() != 0 { try!(write!(f, " treq"))}
        if self.rxfifo() != 0 { try!(write!(f, " rxfifo"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Slave Data"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sdr(pub u32);
impl Sdr {
    #[doc="Data for Transfer"]
    #[inline] pub fn data(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Data for Transfer"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data != 0
    }

    #[doc="Data for Transfer"]
    #[inline] pub fn set_data<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Sdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.data() != 0 { try!(write!(f, " data=0x{:x}", self.data()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Slave Interrupt Mask"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Simr(pub u32);
impl Simr {
    #[doc="Data Interrupt Mask"]
    #[inline] pub fn dataim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Data Interrupt Mask"]
    #[inline] pub fn test_dataim(&self) -> bool {
        self.dataim != 0
    }

    #[doc="Data Interrupt Mask"]
    #[inline] pub fn set_dataim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Start Condition Interrupt Mask"]
    #[inline] pub fn startim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Start Condition Interrupt Mask"]
    #[inline] pub fn test_startim(&self) -> bool {
        self.startim != 0
    }

    #[doc="Start Condition Interrupt Mask"]
    #[inline] pub fn set_startim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Stop Condition Interrupt Mask"]
    #[inline] pub fn stopim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Stop Condition Interrupt Mask"]
    #[inline] pub fn test_stopim(&self) -> bool {
        self.stopim != 0
    }

    #[doc="Stop Condition Interrupt Mask"]
    #[inline] pub fn set_stopim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Receive DMA Interrupt Mask"]
    #[inline] pub fn dmarxim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Receive DMA Interrupt Mask"]
    #[inline] pub fn test_dmarxim(&self) -> bool {
        self.dmarxim != 0
    }

    #[doc="Receive DMA Interrupt Mask"]
    #[inline] pub fn set_dmarxim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Transmit DMA Interrupt Mask"]
    #[inline] pub fn dmatxim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Transmit DMA Interrupt Mask"]
    #[inline] pub fn test_dmatxim(&self) -> bool {
        self.dmatxim != 0
    }

    #[doc="Transmit DMA Interrupt Mask"]
    #[inline] pub fn set_dmatxim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Transmit FIFO Request Interrupt Mask"]
    #[inline] pub fn txim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Transmit FIFO Request Interrupt Mask"]
    #[inline] pub fn test_txim(&self) -> bool {
        self.txim != 0
    }

    #[doc="Transmit FIFO Request Interrupt Mask"]
    #[inline] pub fn set_txim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Receive FIFO Request Interrupt Mask"]
    #[inline] pub fn rxim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Receive FIFO Request Interrupt Mask"]
    #[inline] pub fn test_rxim(&self) -> bool {
        self.rxim != 0
    }

    #[doc="Receive FIFO Request Interrupt Mask"]
    #[inline] pub fn set_rxim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Transmit FIFO Empty Interrupt Mask"]
    #[inline] pub fn txfeim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Transmit FIFO Empty Interrupt Mask"]
    #[inline] pub fn test_txfeim(&self) -> bool {
        self.txfeim != 0
    }

    #[doc="Transmit FIFO Empty Interrupt Mask"]
    #[inline] pub fn set_txfeim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Receive FIFO Full Interrupt Mask"]
    #[inline] pub fn rxffim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Receive FIFO Full Interrupt Mask"]
    #[inline] pub fn test_rxffim(&self) -> bool {
        self.rxffim != 0
    }

    #[doc="Receive FIFO Full Interrupt Mask"]
    #[inline] pub fn set_rxffim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

}

impl ::core::fmt::Display for Simr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Simr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dataim() != 0 { try!(write!(f, " dataim"))}
        if self.startim() != 0 { try!(write!(f, " startim"))}
        if self.stopim() != 0 { try!(write!(f, " stopim"))}
        if self.dmarxim() != 0 { try!(write!(f, " dmarxim"))}
        if self.dmatxim() != 0 { try!(write!(f, " dmatxim"))}
        if self.txim() != 0 { try!(write!(f, " txim"))}
        if self.rxim() != 0 { try!(write!(f, " rxim"))}
        if self.txfeim() != 0 { try!(write!(f, " txfeim"))}
        if self.rxffim() != 0 { try!(write!(f, " rxffim"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Slave Raw Interrupt Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sris(pub u32);
impl Sris {
    #[doc="Data Raw Interrupt Status"]
    #[inline] pub fn dataris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Data Raw Interrupt Status"]
    #[inline] pub fn test_dataris(&self) -> bool {
        self.dataris != 0
    }

    #[doc="Data Raw Interrupt Status"]
    #[inline] pub fn set_dataris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Start Condition Raw Interrupt Status"]
    #[inline] pub fn startris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Start Condition Raw Interrupt Status"]
    #[inline] pub fn test_startris(&self) -> bool {
        self.startris != 0
    }

    #[doc="Start Condition Raw Interrupt Status"]
    #[inline] pub fn set_startris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Stop Condition Raw Interrupt Status"]
    #[inline] pub fn stopris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Stop Condition Raw Interrupt Status"]
    #[inline] pub fn test_stopris(&self) -> bool {
        self.stopris != 0
    }

    #[doc="Stop Condition Raw Interrupt Status"]
    #[inline] pub fn set_stopris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Receive DMA Raw Interrupt Status"]
    #[inline] pub fn dmarxris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Receive DMA Raw Interrupt Status"]
    #[inline] pub fn test_dmarxris(&self) -> bool {
        self.dmarxris != 0
    }

    #[doc="Receive DMA Raw Interrupt Status"]
    #[inline] pub fn set_dmarxris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Transmit DMA Raw Interrupt Status"]
    #[inline] pub fn dmatxris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Transmit DMA Raw Interrupt Status"]
    #[inline] pub fn test_dmatxris(&self) -> bool {
        self.dmatxris != 0
    }

    #[doc="Transmit DMA Raw Interrupt Status"]
    #[inline] pub fn set_dmatxris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Transmit Request Raw Interrupt Status"]
    #[inline] pub fn txris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Transmit Request Raw Interrupt Status"]
    #[inline] pub fn test_txris(&self) -> bool {
        self.txris != 0
    }

    #[doc="Transmit Request Raw Interrupt Status"]
    #[inline] pub fn set_txris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Receive FIFO Request Raw Interrupt Status"]
    #[inline] pub fn rxris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Receive FIFO Request Raw Interrupt Status"]
    #[inline] pub fn test_rxris(&self) -> bool {
        self.rxris != 0
    }

    #[doc="Receive FIFO Request Raw Interrupt Status"]
    #[inline] pub fn set_rxris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Transmit FIFO Empty Raw Interrupt Status"]
    #[inline] pub fn txferis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Transmit FIFO Empty Raw Interrupt Status"]
    #[inline] pub fn test_txferis(&self) -> bool {
        self.txferis != 0
    }

    #[doc="Transmit FIFO Empty Raw Interrupt Status"]
    #[inline] pub fn set_txferis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Receive FIFO Full Raw Interrupt Status"]
    #[inline] pub fn rxffris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Receive FIFO Full Raw Interrupt Status"]
    #[inline] pub fn test_rxffris(&self) -> bool {
        self.rxffris != 0
    }

    #[doc="Receive FIFO Full Raw Interrupt Status"]
    #[inline] pub fn set_rxffris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

}

impl ::core::fmt::Display for Sris {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sris {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dataris() != 0 { try!(write!(f, " dataris"))}
        if self.startris() != 0 { try!(write!(f, " startris"))}
        if self.stopris() != 0 { try!(write!(f, " stopris"))}
        if self.dmarxris() != 0 { try!(write!(f, " dmarxris"))}
        if self.dmatxris() != 0 { try!(write!(f, " dmatxris"))}
        if self.txris() != 0 { try!(write!(f, " txris"))}
        if self.rxris() != 0 { try!(write!(f, " rxris"))}
        if self.txferis() != 0 { try!(write!(f, " txferis"))}
        if self.rxffris() != 0 { try!(write!(f, " rxffris"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Slave Masked Interrupt Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Smis(pub u32);
impl Smis {
    #[doc="Data Masked Interrupt Status"]
    #[inline] pub fn datamis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Data Masked Interrupt Status"]
    #[inline] pub fn test_datamis(&self) -> bool {
        self.datamis != 0
    }

    #[doc="Data Masked Interrupt Status"]
    #[inline] pub fn set_datamis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Start Condition Masked Interrupt Status"]
    #[inline] pub fn startmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Start Condition Masked Interrupt Status"]
    #[inline] pub fn test_startmis(&self) -> bool {
        self.startmis != 0
    }

    #[doc="Start Condition Masked Interrupt Status"]
    #[inline] pub fn set_startmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Stop Condition Masked Interrupt Status"]
    #[inline] pub fn stopmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Stop Condition Masked Interrupt Status"]
    #[inline] pub fn test_stopmis(&self) -> bool {
        self.stopmis != 0
    }

    #[doc="Stop Condition Masked Interrupt Status"]
    #[inline] pub fn set_stopmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Receive DMA Masked Interrupt Status"]
    #[inline] pub fn dmarxmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Receive DMA Masked Interrupt Status"]
    #[inline] pub fn test_dmarxmis(&self) -> bool {
        self.dmarxmis != 0
    }

    #[doc="Receive DMA Masked Interrupt Status"]
    #[inline] pub fn set_dmarxmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Transmit DMA Masked Interrupt Status"]
    #[inline] pub fn dmatxmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Transmit DMA Masked Interrupt Status"]
    #[inline] pub fn test_dmatxmis(&self) -> bool {
        self.dmatxmis != 0
    }

    #[doc="Transmit DMA Masked Interrupt Status"]
    #[inline] pub fn set_dmatxmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Transmit FIFO Request Interrupt Mask"]
    #[inline] pub fn txmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Transmit FIFO Request Interrupt Mask"]
    #[inline] pub fn test_txmis(&self) -> bool {
        self.txmis != 0
    }

    #[doc="Transmit FIFO Request Interrupt Mask"]
    #[inline] pub fn set_txmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Receive FIFO Request Interrupt Mask"]
    #[inline] pub fn rxmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Receive FIFO Request Interrupt Mask"]
    #[inline] pub fn test_rxmis(&self) -> bool {
        self.rxmis != 0
    }

    #[doc="Receive FIFO Request Interrupt Mask"]
    #[inline] pub fn set_rxmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Transmit FIFO Empty Interrupt Mask"]
    #[inline] pub fn txfemis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Transmit FIFO Empty Interrupt Mask"]
    #[inline] pub fn test_txfemis(&self) -> bool {
        self.txfemis != 0
    }

    #[doc="Transmit FIFO Empty Interrupt Mask"]
    #[inline] pub fn set_txfemis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Receive FIFO Full Interrupt Mask"]
    #[inline] pub fn rxffmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Receive FIFO Full Interrupt Mask"]
    #[inline] pub fn test_rxffmis(&self) -> bool {
        self.rxffmis != 0
    }

    #[doc="Receive FIFO Full Interrupt Mask"]
    #[inline] pub fn set_rxffmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

}

impl ::core::fmt::Display for Smis {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Smis {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.datamis() != 0 { try!(write!(f, " datamis"))}
        if self.startmis() != 0 { try!(write!(f, " startmis"))}
        if self.stopmis() != 0 { try!(write!(f, " stopmis"))}
        if self.dmarxmis() != 0 { try!(write!(f, " dmarxmis"))}
        if self.dmatxmis() != 0 { try!(write!(f, " dmatxmis"))}
        if self.txmis() != 0 { try!(write!(f, " txmis"))}
        if self.rxmis() != 0 { try!(write!(f, " rxmis"))}
        if self.txfemis() != 0 { try!(write!(f, " txfemis"))}
        if self.rxffmis() != 0 { try!(write!(f, " rxffmis"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Slave Interrupt Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sicr(pub u32);
impl Sicr {
    #[doc="Data Interrupt Clear"]
    #[inline] pub fn dataic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Data Interrupt Clear"]
    #[inline] pub fn test_dataic(&self) -> bool {
        self.dataic != 0
    }

    #[doc="Data Interrupt Clear"]
    #[inline] pub fn set_dataic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Start Condition Interrupt Clear"]
    #[inline] pub fn startic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Start Condition Interrupt Clear"]
    #[inline] pub fn test_startic(&self) -> bool {
        self.startic != 0
    }

    #[doc="Start Condition Interrupt Clear"]
    #[inline] pub fn set_startic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Stop Condition Interrupt Clear"]
    #[inline] pub fn stopic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Stop Condition Interrupt Clear"]
    #[inline] pub fn test_stopic(&self) -> bool {
        self.stopic != 0
    }

    #[doc="Stop Condition Interrupt Clear"]
    #[inline] pub fn set_stopic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Receive DMA Interrupt Clear"]
    #[inline] pub fn dmarxic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Receive DMA Interrupt Clear"]
    #[inline] pub fn test_dmarxic(&self) -> bool {
        self.dmarxic != 0
    }

    #[doc="Receive DMA Interrupt Clear"]
    #[inline] pub fn set_dmarxic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Transmit DMA Interrupt Clear"]
    #[inline] pub fn dmatxic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Transmit DMA Interrupt Clear"]
    #[inline] pub fn test_dmatxic(&self) -> bool {
        self.dmatxic != 0
    }

    #[doc="Transmit DMA Interrupt Clear"]
    #[inline] pub fn set_dmatxic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Transmit Request Interrupt Mask"]
    #[inline] pub fn txic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Transmit Request Interrupt Mask"]
    #[inline] pub fn test_txic(&self) -> bool {
        self.txic != 0
    }

    #[doc="Transmit Request Interrupt Mask"]
    #[inline] pub fn set_txic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Receive Request Interrupt Mask"]
    #[inline] pub fn rxic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Receive Request Interrupt Mask"]
    #[inline] pub fn test_rxic(&self) -> bool {
        self.rxic != 0
    }

    #[doc="Receive Request Interrupt Mask"]
    #[inline] pub fn set_rxic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Transmit FIFO Empty Interrupt Mask"]
    #[inline] pub fn txfeic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Transmit FIFO Empty Interrupt Mask"]
    #[inline] pub fn test_txfeic(&self) -> bool {
        self.txfeic != 0
    }

    #[doc="Transmit FIFO Empty Interrupt Mask"]
    #[inline] pub fn set_txfeic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Receive FIFO Full Interrupt Mask"]
    #[inline] pub fn rxffic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Receive FIFO Full Interrupt Mask"]
    #[inline] pub fn test_rxffic(&self) -> bool {
        self.rxffic != 0
    }

    #[doc="Receive FIFO Full Interrupt Mask"]
    #[inline] pub fn set_rxffic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

}

impl ::core::fmt::Display for Sicr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sicr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dataic() != 0 { try!(write!(f, " dataic"))}
        if self.startic() != 0 { try!(write!(f, " startic"))}
        if self.stopic() != 0 { try!(write!(f, " stopic"))}
        if self.dmarxic() != 0 { try!(write!(f, " dmarxic"))}
        if self.dmatxic() != 0 { try!(write!(f, " dmatxic"))}
        if self.txic() != 0 { try!(write!(f, " txic"))}
        if self.rxic() != 0 { try!(write!(f, " rxic"))}
        if self.txfeic() != 0 { try!(write!(f, " txfeic"))}
        if self.rxffic() != 0 { try!(write!(f, " rxffic"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Slave Own Address 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Soar2(pub u32);
impl Soar2 {
    #[doc="I2C Slave Own Address 2"]
    #[inline] pub fn oar2(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="I2C Slave Own Address 2"]
    #[inline] pub fn test_oar2(&self) -> bool {
        self.oar2 != 0
    }

    #[doc="I2C Slave Own Address 2"]
    #[inline] pub fn set_oar2<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="I2C Slave Own Address 2 Enable"]
    #[inline] pub fn oar2en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="I2C Slave Own Address 2 Enable"]
    #[inline] pub fn test_oar2en(&self) -> bool {
        self.oar2en != 0
    }

    #[doc="I2C Slave Own Address 2 Enable"]
    #[inline] pub fn set_oar2en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl ::core::fmt::Display for Soar2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Soar2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.oar2() != 0 { try!(write!(f, " oar2=0x{:x}", self.oar2()))}
        if self.oar2en() != 0 { try!(write!(f, " oar2en"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Slave ACK Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sackctl(pub u32);
impl Sackctl {
    #[doc="I2C Slave ACK Override Enable"]
    #[inline] pub fn ackoen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="I2C Slave ACK Override Enable"]
    #[inline] pub fn test_ackoen(&self) -> bool {
        self.ackoen != 0
    }

    #[doc="I2C Slave ACK Override Enable"]
    #[inline] pub fn set_ackoen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="I2C Slave ACK Override Value"]
    #[inline] pub fn ackoval(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="I2C Slave ACK Override Value"]
    #[inline] pub fn test_ackoval(&self) -> bool {
        self.ackoval != 0
    }

    #[doc="I2C Slave ACK Override Value"]
    #[inline] pub fn set_ackoval<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl ::core::fmt::Display for Sackctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sackctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ackoen() != 0 { try!(write!(f, " ackoen"))}
        if self.ackoval() != 0 { try!(write!(f, " ackoval"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

}
// End of slave


