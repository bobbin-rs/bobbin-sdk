#[allow(unused_imports)] use ::bobbin_common::*;

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="I2C Peripheral"]
pub struct I2cPeriph(pub usize); 

impl I2cPeriph {
    #[doc="Get the *mut pointer for the CR1 register."]
    #[inline] pub fn cr1_mut(&self) -> *mut Cr1 { 
        (self.0 + 0x0) as *mut Cr1
    }

    #[doc="Get the *const pointer for the CR1 register."]
    #[inline] pub fn cr1_ptr(&self) -> *const Cr1 { 
           self.cr1_mut()
    }

    #[doc="Read the CR1 register."]
    #[inline] pub fn cr1(&self) -> Cr1 { 
        unsafe {
            read_volatile(self.cr1_ptr())
        }
    }

    #[doc="Write the CR1 register."]
    #[inline] pub fn set_cr1<F: FnOnce(Cr1) -> Cr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cr1_mut(), f(Cr1(0)));
        }
        self
    }

    #[doc="Modify the CR1 register."]
    #[inline] pub fn with_cr1<F: FnOnce(Cr1) -> Cr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cr1_mut(), f(self.cr1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CR2 register."]
    #[inline] pub fn cr2_mut(&self) -> *mut Cr2 { 
        (self.0 + 0x4) as *mut Cr2
    }

    #[doc="Get the *const pointer for the CR2 register."]
    #[inline] pub fn cr2_ptr(&self) -> *const Cr2 { 
           self.cr2_mut()
    }

    #[doc="Read the CR2 register."]
    #[inline] pub fn cr2(&self) -> Cr2 { 
        unsafe {
            read_volatile(self.cr2_ptr())
        }
    }

    #[doc="Write the CR2 register."]
    #[inline] pub fn set_cr2<F: FnOnce(Cr2) -> Cr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cr2_mut(), f(Cr2(0)));
        }
        self
    }

    #[doc="Modify the CR2 register."]
    #[inline] pub fn with_cr2<F: FnOnce(Cr2) -> Cr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cr2_mut(), f(self.cr2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the OAR1 register."]
    #[inline] pub fn oar1_mut(&self) -> *mut Oar1 { 
        (self.0 + 0x8) as *mut Oar1
    }

    #[doc="Get the *const pointer for the OAR1 register."]
    #[inline] pub fn oar1_ptr(&self) -> *const Oar1 { 
           self.oar1_mut()
    }

    #[doc="Read the OAR1 register."]
    #[inline] pub fn oar1(&self) -> Oar1 { 
        unsafe {
            read_volatile(self.oar1_ptr())
        }
    }

    #[doc="Write the OAR1 register."]
    #[inline] pub fn set_oar1<F: FnOnce(Oar1) -> Oar1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.oar1_mut(), f(Oar1(0)));
        }
        self
    }

    #[doc="Modify the OAR1 register."]
    #[inline] pub fn with_oar1<F: FnOnce(Oar1) -> Oar1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.oar1_mut(), f(self.oar1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the OAR2 register."]
    #[inline] pub fn oar2_mut(&self) -> *mut Oar2 { 
        (self.0 + 0xc) as *mut Oar2
    }

    #[doc="Get the *const pointer for the OAR2 register."]
    #[inline] pub fn oar2_ptr(&self) -> *const Oar2 { 
           self.oar2_mut()
    }

    #[doc="Read the OAR2 register."]
    #[inline] pub fn oar2(&self) -> Oar2 { 
        unsafe {
            read_volatile(self.oar2_ptr())
        }
    }

    #[doc="Write the OAR2 register."]
    #[inline] pub fn set_oar2<F: FnOnce(Oar2) -> Oar2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.oar2_mut(), f(Oar2(0)));
        }
        self
    }

    #[doc="Modify the OAR2 register."]
    #[inline] pub fn with_oar2<F: FnOnce(Oar2) -> Oar2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.oar2_mut(), f(self.oar2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TIMINGR register."]
    #[inline] pub fn timingr_mut(&self) -> *mut Timingr { 
        (self.0 + 0x10) as *mut Timingr
    }

    #[doc="Get the *const pointer for the TIMINGR register."]
    #[inline] pub fn timingr_ptr(&self) -> *const Timingr { 
           self.timingr_mut()
    }

    #[doc="Read the TIMINGR register."]
    #[inline] pub fn timingr(&self) -> Timingr { 
        unsafe {
            read_volatile(self.timingr_ptr())
        }
    }

    #[doc="Write the TIMINGR register."]
    #[inline] pub fn set_timingr<F: FnOnce(Timingr) -> Timingr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.timingr_mut(), f(Timingr(0)));
        }
        self
    }

    #[doc="Modify the TIMINGR register."]
    #[inline] pub fn with_timingr<F: FnOnce(Timingr) -> Timingr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.timingr_mut(), f(self.timingr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TIMEOUTR register."]
    #[inline] pub fn timeoutr_mut(&self) -> *mut Timeoutr { 
        (self.0 + 0x14) as *mut Timeoutr
    }

    #[doc="Get the *const pointer for the TIMEOUTR register."]
    #[inline] pub fn timeoutr_ptr(&self) -> *const Timeoutr { 
           self.timeoutr_mut()
    }

    #[doc="Read the TIMEOUTR register."]
    #[inline] pub fn timeoutr(&self) -> Timeoutr { 
        unsafe {
            read_volatile(self.timeoutr_ptr())
        }
    }

    #[doc="Write the TIMEOUTR register."]
    #[inline] pub fn set_timeoutr<F: FnOnce(Timeoutr) -> Timeoutr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.timeoutr_mut(), f(Timeoutr(0)));
        }
        self
    }

    #[doc="Modify the TIMEOUTR register."]
    #[inline] pub fn with_timeoutr<F: FnOnce(Timeoutr) -> Timeoutr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.timeoutr_mut(), f(self.timeoutr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ISR register."]
    #[inline] pub fn isr_mut(&self) -> *mut Isr { 
        (self.0 + 0x18) as *mut Isr
    }

    #[doc="Get the *const pointer for the ISR register."]
    #[inline] pub fn isr_ptr(&self) -> *const Isr { 
           self.isr_mut()
    }

    #[doc="Read the ISR register."]
    #[inline] pub fn isr(&self) -> Isr { 
        unsafe {
            read_volatile(self.isr_ptr())
        }
    }

    #[doc="Write the ISR register."]
    #[inline] pub fn set_isr<F: FnOnce(Isr) -> Isr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.isr_mut(), f(Isr(0)));
        }
        self
    }

    #[doc="Modify the ISR register."]
    #[inline] pub fn with_isr<F: FnOnce(Isr) -> Isr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.isr_mut(), f(self.isr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ICR register."]
    #[inline] pub fn icr_mut(&self) -> *mut Icr { 
        (self.0 + 0x1c) as *mut Icr
    }

    #[doc="Get the *const pointer for the ICR register."]
    #[inline] pub fn icr_ptr(&self) -> *const Icr { 
           self.icr_mut()
    }

    #[doc="Write the ICR register."]
    #[inline] pub fn set_icr<F: FnOnce(Icr) -> Icr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.icr_mut(), f(Icr(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the PECR register."]
    #[inline] pub fn pecr_mut(&self) -> *mut Pecr { 
        (self.0 + 0x20) as *mut Pecr
    }

    #[doc="Get the *const pointer for the PECR register."]
    #[inline] pub fn pecr_ptr(&self) -> *const Pecr { 
           self.pecr_mut()
    }

    #[doc="Read the PECR register."]
    #[inline] pub fn pecr(&self) -> Pecr { 
        unsafe {
            read_volatile(self.pecr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RXDR register."]
    #[inline] pub fn rxdr_mut(&self) -> *mut Rxdr { 
        (self.0 + 0x24) as *mut Rxdr
    }

    #[doc="Get the *const pointer for the RXDR register."]
    #[inline] pub fn rxdr_ptr(&self) -> *const Rxdr { 
           self.rxdr_mut()
    }

    #[doc="Read the RXDR register."]
    #[inline] pub fn rxdr(&self) -> Rxdr { 
        unsafe {
            read_volatile(self.rxdr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the TXDR register."]
    #[inline] pub fn txdr_mut(&self) -> *mut Txdr { 
        (self.0 + 0x28) as *mut Txdr
    }

    #[doc="Get the *const pointer for the TXDR register."]
    #[inline] pub fn txdr_ptr(&self) -> *const Txdr { 
           self.txdr_mut()
    }

    #[doc="Read the TXDR register."]
    #[inline] pub fn txdr(&self) -> Txdr { 
        unsafe {
            read_volatile(self.txdr_ptr())
        }
    }

    #[doc="Write the TXDR register."]
    #[inline] pub fn set_txdr<F: FnOnce(Txdr) -> Txdr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.txdr_mut(), f(Txdr(0)));
        }
        self
    }

    #[doc="Modify the TXDR register."]
    #[inline] pub fn with_txdr<F: FnOnce(Txdr) -> Txdr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.txdr_mut(), f(self.txdr()));
        }
        self
    }

}

#[doc="Control register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr1(pub u32);
impl Cr1 {
    #[doc="Peripheral enable"]
    #[inline] pub fn pe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PE != 0"]
    #[inline] pub fn test_pe(&self) -> bool {
        self.pe() != 0
    }

    #[doc="Sets the PE field."]
    #[inline] pub fn set_pe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="TX Interrupt enable"]
    #[inline] pub fn txie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TXIE != 0"]
    #[inline] pub fn test_txie(&self) -> bool {
        self.txie() != 0
    }

    #[doc="Sets the TXIE field."]
    #[inline] pub fn set_txie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="RX Interrupt enable"]
    #[inline] pub fn rxie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RXIE != 0"]
    #[inline] pub fn test_rxie(&self) -> bool {
        self.rxie() != 0
    }

    #[doc="Sets the RXIE field."]
    #[inline] pub fn set_rxie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Address match interrupt enable (slave only)"]
    #[inline] pub fn addrie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ADDRIE != 0"]
    #[inline] pub fn test_addrie(&self) -> bool {
        self.addrie() != 0
    }

    #[doc="Sets the ADDRIE field."]
    #[inline] pub fn set_addrie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Not acknowledge received interrupt enable"]
    #[inline] pub fn nackie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NACKIE != 0"]
    #[inline] pub fn test_nackie(&self) -> bool {
        self.nackie() != 0
    }

    #[doc="Sets the NACKIE field."]
    #[inline] pub fn set_nackie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="STOP detection Interrupt enable"]
    #[inline] pub fn stopie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if STOPIE != 0"]
    #[inline] pub fn test_stopie(&self) -> bool {
        self.stopie() != 0
    }

    #[doc="Sets the STOPIE field."]
    #[inline] pub fn set_stopie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Transfer Complete interrupt enable"]
    #[inline] pub fn tcie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TCIE != 0"]
    #[inline] pub fn test_tcie(&self) -> bool {
        self.tcie() != 0
    }

    #[doc="Sets the TCIE field."]
    #[inline] pub fn set_tcie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Error interrupts enable"]
    #[inline] pub fn errie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ERRIE != 0"]
    #[inline] pub fn test_errie(&self) -> bool {
        self.errie() != 0
    }

    #[doc="Sets the ERRIE field."]
    #[inline] pub fn set_errie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Digital noise filter"]
    #[inline] pub fn dnf(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if DNF != 0"]
    #[inline] pub fn test_dnf(&self) -> bool {
        self.dnf() != 0
    }

    #[doc="Sets the DNF field."]
    #[inline] pub fn set_dnf<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Analog noise filter OFF"]
    #[inline] pub fn anfoff(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if ANFOFF != 0"]
    #[inline] pub fn test_anfoff(&self) -> bool {
        self.anfoff() != 0
    }

    #[doc="Sets the ANFOFF field."]
    #[inline] pub fn set_anfoff<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Software reset"]
    #[inline] pub fn swrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if SWRST != 0"]
    #[inline] pub fn test_swrst(&self) -> bool {
        self.swrst() != 0
    }

    #[doc="Sets the SWRST field."]
    #[inline] pub fn set_swrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="DMA transmission requests enable"]
    #[inline] pub fn txdmaen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TXDMAEN != 0"]
    #[inline] pub fn test_txdmaen(&self) -> bool {
        self.txdmaen() != 0
    }

    #[doc="Sets the TXDMAEN field."]
    #[inline] pub fn set_txdmaen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="DMA reception requests enable"]
    #[inline] pub fn rxdmaen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if RXDMAEN != 0"]
    #[inline] pub fn test_rxdmaen(&self) -> bool {
        self.rxdmaen() != 0
    }

    #[doc="Sets the RXDMAEN field."]
    #[inline] pub fn set_rxdmaen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Slave byte control"]
    #[inline] pub fn sbc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if SBC != 0"]
    #[inline] pub fn test_sbc(&self) -> bool {
        self.sbc() != 0
    }

    #[doc="Sets the SBC field."]
    #[inline] pub fn set_sbc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Clock stretching disable"]
    #[inline] pub fn nostretch(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if NOSTRETCH != 0"]
    #[inline] pub fn test_nostretch(&self) -> bool {
        self.nostretch() != 0
    }

    #[doc="Sets the NOSTRETCH field."]
    #[inline] pub fn set_nostretch<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Wakeup from STOP enable"]
    #[inline] pub fn wupen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if WUPEN != 0"]
    #[inline] pub fn test_wupen(&self) -> bool {
        self.wupen() != 0
    }

    #[doc="Sets the WUPEN field."]
    #[inline] pub fn set_wupen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="General call enable"]
    #[inline] pub fn gcen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if GCEN != 0"]
    #[inline] pub fn test_gcen(&self) -> bool {
        self.gcen() != 0
    }

    #[doc="Sets the GCEN field."]
    #[inline] pub fn set_gcen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="SMBus Host address enable"]
    #[inline] pub fn smbhen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if SMBHEN != 0"]
    #[inline] pub fn test_smbhen(&self) -> bool {
        self.smbhen() != 0
    }

    #[doc="Sets the SMBHEN field."]
    #[inline] pub fn set_smbhen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="SMBus Device Default address enable"]
    #[inline] pub fn smbden(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if SMBDEN != 0"]
    #[inline] pub fn test_smbden(&self) -> bool {
        self.smbden() != 0
    }

    #[doc="Sets the SMBDEN field."]
    #[inline] pub fn set_smbden<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="SMBUS alert enable"]
    #[inline] pub fn alerten(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if ALERTEN != 0"]
    #[inline] pub fn test_alerten(&self) -> bool {
        self.alerten() != 0
    }

    #[doc="Sets the ALERTEN field."]
    #[inline] pub fn set_alerten<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="PEC enable"]
    #[inline] pub fn pecen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if PECEN != 0"]
    #[inline] pub fn test_pecen(&self) -> bool {
        self.pecen() != 0
    }

    #[doc="Sets the PECEN field."]
    #[inline] pub fn set_pecen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

}

impl From<u32> for Cr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Cr1(other)
    }
}

impl ::core::fmt::Display for Cr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pe() != 0 { try!(write!(f, " pe"))}
        if self.txie() != 0 { try!(write!(f, " txie"))}
        if self.rxie() != 0 { try!(write!(f, " rxie"))}
        if self.addrie() != 0 { try!(write!(f, " addrie"))}
        if self.nackie() != 0 { try!(write!(f, " nackie"))}
        if self.stopie() != 0 { try!(write!(f, " stopie"))}
        if self.tcie() != 0 { try!(write!(f, " tcie"))}
        if self.errie() != 0 { try!(write!(f, " errie"))}
        if self.dnf() != 0 { try!(write!(f, " dnf=0x{:x}", self.dnf()))}
        if self.anfoff() != 0 { try!(write!(f, " anfoff"))}
        if self.swrst() != 0 { try!(write!(f, " swrst"))}
        if self.txdmaen() != 0 { try!(write!(f, " txdmaen"))}
        if self.rxdmaen() != 0 { try!(write!(f, " rxdmaen"))}
        if self.sbc() != 0 { try!(write!(f, " sbc"))}
        if self.nostretch() != 0 { try!(write!(f, " nostretch"))}
        if self.wupen() != 0 { try!(write!(f, " wupen"))}
        if self.gcen() != 0 { try!(write!(f, " gcen"))}
        if self.smbhen() != 0 { try!(write!(f, " smbhen"))}
        if self.smbden() != 0 { try!(write!(f, " smbden"))}
        if self.alerten() != 0 { try!(write!(f, " alerten"))}
        if self.pecen() != 0 { try!(write!(f, " pecen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr2(pub u32);
impl Cr2 {
    #[doc="Packet error checking byte"]
    #[inline] pub fn pecbyte(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if PECBYTE != 0"]
    #[inline] pub fn test_pecbyte(&self) -> bool {
        self.pecbyte() != 0
    }

    #[doc="Sets the PECBYTE field."]
    #[inline] pub fn set_pecbyte<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Automatic end mode (master mode)"]
    #[inline] pub fn autoend(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if AUTOEND != 0"]
    #[inline] pub fn test_autoend(&self) -> bool {
        self.autoend() != 0
    }

    #[doc="Sets the AUTOEND field."]
    #[inline] pub fn set_autoend<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="NBYTES reload mode"]
    #[inline] pub fn reload(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if RELOAD != 0"]
    #[inline] pub fn test_reload(&self) -> bool {
        self.reload() != 0
    }

    #[doc="Sets the RELOAD field."]
    #[inline] pub fn set_reload<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Number of bytes"]
    #[inline] pub fn nbytes(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if NBYTES != 0"]
    #[inline] pub fn test_nbytes(&self) -> bool {
        self.nbytes() != 0
    }

    #[doc="Sets the NBYTES field."]
    #[inline] pub fn set_nbytes<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="NACK generation (slave mode)"]
    #[inline] pub fn nack(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if NACK != 0"]
    #[inline] pub fn test_nack(&self) -> bool {
        self.nack() != 0
    }

    #[doc="Sets the NACK field."]
    #[inline] pub fn set_nack<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Stop generation (master mode)"]
    #[inline] pub fn stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if STOP != 0"]
    #[inline] pub fn test_stop(&self) -> bool {
        self.stop() != 0
    }

    #[doc="Sets the STOP field."]
    #[inline] pub fn set_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Start generation"]
    #[inline] pub fn start(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if START != 0"]
    #[inline] pub fn test_start(&self) -> bool {
        self.start() != 0
    }

    #[doc="Sets the START field."]
    #[inline] pub fn set_start<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="10-bit address header only read direction (master receiver mode)"]
    #[inline] pub fn head10r(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if HEAD10R != 0"]
    #[inline] pub fn test_head10r(&self) -> bool {
        self.head10r() != 0
    }

    #[doc="Sets the HEAD10R field."]
    #[inline] pub fn set_head10r<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="10-bit addressing mode (master mode)"]
    #[inline] pub fn add10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if ADD10 != 0"]
    #[inline] pub fn test_add10(&self) -> bool {
        self.add10() != 0
    }

    #[doc="Sets the ADD10 field."]
    #[inline] pub fn set_add10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Transfer direction (master mode)"]
    #[inline] pub fn rd_wrn(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if RD_WRN != 0"]
    #[inline] pub fn test_rd_wrn(&self) -> bool {
        self.rd_wrn() != 0
    }

    #[doc="Sets the RD_WRN field."]
    #[inline] pub fn set_rd_wrn<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Slave address 9:0 (master mode)"]
    #[inline] pub fn sadd(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="Returns true if SADD != 0"]
    #[inline] pub fn test_sadd(&self) -> bool {
        self.sadd() != 0
    }

    #[doc="Sets the SADD field."]
    #[inline] pub fn set_sadd<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Cr2(other)
    }
}

impl ::core::fmt::Display for Cr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pecbyte() != 0 { try!(write!(f, " pecbyte"))}
        if self.autoend() != 0 { try!(write!(f, " autoend"))}
        if self.reload() != 0 { try!(write!(f, " reload"))}
        if self.nbytes() != 0 { try!(write!(f, " nbytes=0x{:x}", self.nbytes()))}
        if self.nack() != 0 { try!(write!(f, " nack"))}
        if self.stop() != 0 { try!(write!(f, " stop"))}
        if self.start() != 0 { try!(write!(f, " start"))}
        if self.head10r() != 0 { try!(write!(f, " head10r"))}
        if self.add10() != 0 { try!(write!(f, " add10"))}
        if self.rd_wrn() != 0 { try!(write!(f, " rd_wrn"))}
        if self.sadd() != 0 { try!(write!(f, " sadd=0x{:x}", self.sadd()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Own address register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Oar1(pub u32);
impl Oar1 {
    #[doc="Interface address"]
    #[inline] pub fn oa1_0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OA1_0 != 0"]
    #[inline] pub fn test_oa1_0(&self) -> bool {
        self.oa1_0() != 0
    }

    #[doc="Sets the OA1_0 field."]
    #[inline] pub fn set_oa1_0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Interface address"]
    #[inline] pub fn oa1_1(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x7f) as u8) } // [7:1]
    }

    #[doc="Returns true if OA1_1 != 0"]
    #[inline] pub fn test_oa1_1(&self) -> bool {
        self.oa1_1() != 0
    }

    #[doc="Sets the OA1_1 field."]
    #[inline] pub fn set_oa1_1<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Interface address"]
    #[inline] pub fn oa1_8(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if OA1_8 != 0"]
    #[inline] pub fn test_oa1_8(&self) -> bool {
        self.oa1_8() != 0
    }

    #[doc="Sets the OA1_8 field."]
    #[inline] pub fn set_oa1_8<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Own Address 1 10-bit mode"]
    #[inline] pub fn oa1mode(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if OA1MODE != 0"]
    #[inline] pub fn test_oa1mode(&self) -> bool {
        self.oa1mode() != 0
    }

    #[doc="Sets the OA1MODE field."]
    #[inline] pub fn set_oa1mode<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Own Address 1 enable"]
    #[inline] pub fn oa1en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if OA1EN != 0"]
    #[inline] pub fn test_oa1en(&self) -> bool {
        self.oa1en() != 0
    }

    #[doc="Sets the OA1EN field."]
    #[inline] pub fn set_oa1en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u32> for Oar1 {
    #[inline]
    fn from(other: u32) -> Self {
         Oar1(other)
    }
}

impl ::core::fmt::Display for Oar1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Oar1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.oa1_0() != 0 { try!(write!(f, " oa1_0"))}
        if self.oa1_1() != 0 { try!(write!(f, " oa1_1=0x{:x}", self.oa1_1()))}
        if self.oa1_8() != 0 { try!(write!(f, " oa1_8=0x{:x}", self.oa1_8()))}
        if self.oa1mode() != 0 { try!(write!(f, " oa1mode"))}
        if self.oa1en() != 0 { try!(write!(f, " oa1en"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Own address register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Oar2(pub u32);
impl Oar2 {
    #[doc="Interface address"]
    #[inline] pub fn oa2(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x7f) as u8) } // [7:1]
    }

    #[doc="Returns true if OA2 != 0"]
    #[inline] pub fn test_oa2(&self) -> bool {
        self.oa2() != 0
    }

    #[doc="Sets the OA2 field."]
    #[inline] pub fn set_oa2<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Own Address 2 masks"]
    #[inline] pub fn oa2msk(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if OA2MSK != 0"]
    #[inline] pub fn test_oa2msk(&self) -> bool {
        self.oa2msk() != 0
    }

    #[doc="Sets the OA2MSK field."]
    #[inline] pub fn set_oa2msk<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Own Address 2 enable"]
    #[inline] pub fn oa2en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if OA2EN != 0"]
    #[inline] pub fn test_oa2en(&self) -> bool {
        self.oa2en() != 0
    }

    #[doc="Sets the OA2EN field."]
    #[inline] pub fn set_oa2en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u32> for Oar2 {
    #[inline]
    fn from(other: u32) -> Self {
         Oar2(other)
    }
}

impl ::core::fmt::Display for Oar2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Oar2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.oa2() != 0 { try!(write!(f, " oa2=0x{:x}", self.oa2()))}
        if self.oa2msk() != 0 { try!(write!(f, " oa2msk=0x{:x}", self.oa2msk()))}
        if self.oa2en() != 0 { try!(write!(f, " oa2en"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Timing register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Timingr(pub u32);
impl Timingr {
    #[doc="SCL low period (master mode)"]
    #[inline] pub fn scll(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if SCLL != 0"]
    #[inline] pub fn test_scll(&self) -> bool {
        self.scll() != 0
    }

    #[doc="Sets the SCLL field."]
    #[inline] pub fn set_scll<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="SCL high period (master mode)"]
    #[inline] pub fn sclh(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if SCLH != 0"]
    #[inline] pub fn test_sclh(&self) -> bool {
        self.sclh() != 0
    }

    #[doc="Sets the SCLH field."]
    #[inline] pub fn set_sclh<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Data hold time"]
    #[inline] pub fn sdadel(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if SDADEL != 0"]
    #[inline] pub fn test_sdadel(&self) -> bool {
        self.sdadel() != 0
    }

    #[doc="Sets the SDADEL field."]
    #[inline] pub fn set_sdadel<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Data setup time"]
    #[inline] pub fn scldel(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
    }

    #[doc="Returns true if SCLDEL != 0"]
    #[inline] pub fn test_scldel(&self) -> bool {
        self.scldel() != 0
    }

    #[doc="Sets the SCLDEL field."]
    #[inline] pub fn set_scldel<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Timing prescaler"]
    #[inline] pub fn presc(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0xf) as u8) } // [31:28]
    }

    #[doc="Returns true if PRESC != 0"]
    #[inline] pub fn test_presc(&self) -> bool {
        self.presc() != 0
    }

    #[doc="Sets the PRESC field."]
    #[inline] pub fn set_presc<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 28);
        self.0 |= value << 28;
        self
    }

}

impl From<u32> for Timingr {
    #[inline]
    fn from(other: u32) -> Self {
         Timingr(other)
    }
}

impl ::core::fmt::Display for Timingr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Timingr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.scll() != 0 { try!(write!(f, " scll=0x{:x}", self.scll()))}
        if self.sclh() != 0 { try!(write!(f, " sclh=0x{:x}", self.sclh()))}
        if self.sdadel() != 0 { try!(write!(f, " sdadel=0x{:x}", self.sdadel()))}
        if self.scldel() != 0 { try!(write!(f, " scldel=0x{:x}", self.scldel()))}
        if self.presc() != 0 { try!(write!(f, " presc=0x{:x}", self.presc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Timeoutr(pub u32);
impl Timeoutr {
    #[doc="Bus timeout A"]
    #[inline] pub fn timeouta(&self) -> bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if TIMEOUTA != 0"]
    #[inline] pub fn test_timeouta(&self) -> bool {
        self.timeouta() != 0
    }

    #[doc="Sets the TIMEOUTA field."]
    #[inline] pub fn set_timeouta<V: Into<bits::U12>>(mut self, value: V) -> Self {
        let value: bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Idle clock timeout detection"]
    #[inline] pub fn tidle(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if TIDLE != 0"]
    #[inline] pub fn test_tidle(&self) -> bool {
        self.tidle() != 0
    }

    #[doc="Sets the TIDLE field."]
    #[inline] pub fn set_tidle<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Clock timeout enable"]
    #[inline] pub fn timouten(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if TIMOUTEN != 0"]
    #[inline] pub fn test_timouten(&self) -> bool {
        self.timouten() != 0
    }

    #[doc="Sets the TIMOUTEN field."]
    #[inline] pub fn set_timouten<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Bus timeout B"]
    #[inline] pub fn timeoutb(&self) -> bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xfff) as u16) } // [27:16]
    }

    #[doc="Returns true if TIMEOUTB != 0"]
    #[inline] pub fn test_timeoutb(&self) -> bool {
        self.timeoutb() != 0
    }

    #[doc="Sets the TIMEOUTB field."]
    #[inline] pub fn set_timeoutb<V: Into<bits::U12>>(mut self, value: V) -> Self {
        let value: bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Extended clock timeout enable"]
    #[inline] pub fn texten(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if TEXTEN != 0"]
    #[inline] pub fn test_texten(&self) -> bool {
        self.texten() != 0
    }

    #[doc="Sets the TEXTEN field."]
    #[inline] pub fn set_texten<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Timeoutr {
    #[inline]
    fn from(other: u32) -> Self {
         Timeoutr(other)
    }
}

impl ::core::fmt::Display for Timeoutr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Timeoutr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.timeouta() != 0 { try!(write!(f, " timeouta=0x{:x}", self.timeouta()))}
        if self.tidle() != 0 { try!(write!(f, " tidle"))}
        if self.timouten() != 0 { try!(write!(f, " timouten"))}
        if self.timeoutb() != 0 { try!(write!(f, " timeoutb=0x{:x}", self.timeoutb()))}
        if self.texten() != 0 { try!(write!(f, " texten"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt and Status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Isr(pub u32);
impl Isr {
    #[doc="Address match code (Slave mode)"]
    #[inline] pub fn addcode(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x7f) as u8) } // [23:17]
    }

    #[doc="Returns true if ADDCODE != 0"]
    #[inline] pub fn test_addcode(&self) -> bool {
        self.addcode() != 0
    }

    #[doc="Sets the ADDCODE field."]
    #[inline] pub fn set_addcode<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Transfer direction (Slave mode)"]
    #[inline] pub fn dir(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if DIR != 0"]
    #[inline] pub fn test_dir(&self) -> bool {
        self.dir() != 0
    }

    #[doc="Sets the DIR field."]
    #[inline] pub fn set_dir<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Bus busy"]
    #[inline] pub fn busy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if BUSY != 0"]
    #[inline] pub fn test_busy(&self) -> bool {
        self.busy() != 0
    }

    #[doc="Sets the BUSY field."]
    #[inline] pub fn set_busy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="SMBus alert"]
    #[inline] pub fn alert(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if ALERT != 0"]
    #[inline] pub fn test_alert(&self) -> bool {
        self.alert() != 0
    }

    #[doc="Sets the ALERT field."]
    #[inline] pub fn set_alert<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Timeout or t_low detection flag"]
    #[inline] pub fn timeout(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if TIMEOUT != 0"]
    #[inline] pub fn test_timeout(&self) -> bool {
        self.timeout() != 0
    }

    #[doc="Sets the TIMEOUT field."]
    #[inline] pub fn set_timeout<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="PEC Error in reception"]
    #[inline] pub fn pecerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PECERR != 0"]
    #[inline] pub fn test_pecerr(&self) -> bool {
        self.pecerr() != 0
    }

    #[doc="Sets the PECERR field."]
    #[inline] pub fn set_pecerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Overrun/Underrun (slave mode)"]
    #[inline] pub fn ovr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if OVR != 0"]
    #[inline] pub fn test_ovr(&self) -> bool {
        self.ovr() != 0
    }

    #[doc="Sets the OVR field."]
    #[inline] pub fn set_ovr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Arbitration lost"]
    #[inline] pub fn arlo(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if ARLO != 0"]
    #[inline] pub fn test_arlo(&self) -> bool {
        self.arlo() != 0
    }

    #[doc="Sets the ARLO field."]
    #[inline] pub fn set_arlo<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Bus error"]
    #[inline] pub fn berr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BERR != 0"]
    #[inline] pub fn test_berr(&self) -> bool {
        self.berr() != 0
    }

    #[doc="Sets the BERR field."]
    #[inline] pub fn set_berr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Transfer Complete Reload"]
    #[inline] pub fn tcr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TCR != 0"]
    #[inline] pub fn test_tcr(&self) -> bool {
        self.tcr() != 0
    }

    #[doc="Sets the TCR field."]
    #[inline] pub fn set_tcr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Transfer Complete (master mode)"]
    #[inline] pub fn tc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TC != 0"]
    #[inline] pub fn test_tc(&self) -> bool {
        self.tc() != 0
    }

    #[doc="Sets the TC field."]
    #[inline] pub fn set_tc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Stop detection flag"]
    #[inline] pub fn stopf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if STOPF != 0"]
    #[inline] pub fn test_stopf(&self) -> bool {
        self.stopf() != 0
    }

    #[doc="Sets the STOPF field."]
    #[inline] pub fn set_stopf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Not acknowledge received flag"]
    #[inline] pub fn nackf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NACKF != 0"]
    #[inline] pub fn test_nackf(&self) -> bool {
        self.nackf() != 0
    }

    #[doc="Sets the NACKF field."]
    #[inline] pub fn set_nackf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Address matched (slave mode)"]
    #[inline] pub fn addr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ADDR != 0"]
    #[inline] pub fn test_addr(&self) -> bool {
        self.addr() != 0
    }

    #[doc="Sets the ADDR field."]
    #[inline] pub fn set_addr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Receive data register not empty (receivers)"]
    #[inline] pub fn rxne(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RXNE != 0"]
    #[inline] pub fn test_rxne(&self) -> bool {
        self.rxne() != 0
    }

    #[doc="Sets the RXNE field."]
    #[inline] pub fn set_rxne<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Transmit interrupt status (transmitters)"]
    #[inline] pub fn txis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TXIS != 0"]
    #[inline] pub fn test_txis(&self) -> bool {
        self.txis() != 0
    }

    #[doc="Sets the TXIS field."]
    #[inline] pub fn set_txis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Transmit data register empty (transmitters)"]
    #[inline] pub fn txe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TXE != 0"]
    #[inline] pub fn test_txe(&self) -> bool {
        self.txe() != 0
    }

    #[doc="Sets the TXE field."]
    #[inline] pub fn set_txe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
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
        if self.addcode() != 0 { try!(write!(f, " addcode=0x{:x}", self.addcode()))}
        if self.dir() != 0 { try!(write!(f, " dir"))}
        if self.busy() != 0 { try!(write!(f, " busy"))}
        if self.alert() != 0 { try!(write!(f, " alert"))}
        if self.timeout() != 0 { try!(write!(f, " timeout"))}
        if self.pecerr() != 0 { try!(write!(f, " pecerr"))}
        if self.ovr() != 0 { try!(write!(f, " ovr"))}
        if self.arlo() != 0 { try!(write!(f, " arlo"))}
        if self.berr() != 0 { try!(write!(f, " berr"))}
        if self.tcr() != 0 { try!(write!(f, " tcr"))}
        if self.tc() != 0 { try!(write!(f, " tc"))}
        if self.stopf() != 0 { try!(write!(f, " stopf"))}
        if self.nackf() != 0 { try!(write!(f, " nackf"))}
        if self.addr() != 0 { try!(write!(f, " addr"))}
        if self.rxne() != 0 { try!(write!(f, " rxne"))}
        if self.txis() != 0 { try!(write!(f, " txis"))}
        if self.txe() != 0 { try!(write!(f, " txe"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt clear register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Icr(pub u32);
impl Icr {
    #[doc="Alert flag clear"]
    #[inline] pub fn alertcf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if ALERTCF != 0"]
    #[inline] pub fn test_alertcf(&self) -> bool {
        self.alertcf() != 0
    }

    #[doc="Sets the ALERTCF field."]
    #[inline] pub fn set_alertcf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Timeout detection flag clear"]
    #[inline] pub fn timoutcf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if TIMOUTCF != 0"]
    #[inline] pub fn test_timoutcf(&self) -> bool {
        self.timoutcf() != 0
    }

    #[doc="Sets the TIMOUTCF field."]
    #[inline] pub fn set_timoutcf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="PEC Error flag clear"]
    #[inline] pub fn peccf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PECCF != 0"]
    #[inline] pub fn test_peccf(&self) -> bool {
        self.peccf() != 0
    }

    #[doc="Sets the PECCF field."]
    #[inline] pub fn set_peccf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Overrun/Underrun flag clear"]
    #[inline] pub fn ovrcf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if OVRCF != 0"]
    #[inline] pub fn test_ovrcf(&self) -> bool {
        self.ovrcf() != 0
    }

    #[doc="Sets the OVRCF field."]
    #[inline] pub fn set_ovrcf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Arbitration lost flag clear"]
    #[inline] pub fn arlocf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if ARLOCF != 0"]
    #[inline] pub fn test_arlocf(&self) -> bool {
        self.arlocf() != 0
    }

    #[doc="Sets the ARLOCF field."]
    #[inline] pub fn set_arlocf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Bus error flag clear"]
    #[inline] pub fn berrcf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BERRCF != 0"]
    #[inline] pub fn test_berrcf(&self) -> bool {
        self.berrcf() != 0
    }

    #[doc="Sets the BERRCF field."]
    #[inline] pub fn set_berrcf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Stop detection flag clear"]
    #[inline] pub fn stopcf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if STOPCF != 0"]
    #[inline] pub fn test_stopcf(&self) -> bool {
        self.stopcf() != 0
    }

    #[doc="Sets the STOPCF field."]
    #[inline] pub fn set_stopcf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Not Acknowledge flag clear"]
    #[inline] pub fn nackcf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NACKCF != 0"]
    #[inline] pub fn test_nackcf(&self) -> bool {
        self.nackcf() != 0
    }

    #[doc="Sets the NACKCF field."]
    #[inline] pub fn set_nackcf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Address Matched flag clear"]
    #[inline] pub fn addrcf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ADDRCF != 0"]
    #[inline] pub fn test_addrcf(&self) -> bool {
        self.addrcf() != 0
    }

    #[doc="Sets the ADDRCF field."]
    #[inline] pub fn set_addrcf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Icr {
    #[inline]
    fn from(other: u32) -> Self {
         Icr(other)
    }
}

impl ::core::fmt::Display for Icr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Icr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.alertcf() != 0 { try!(write!(f, " alertcf"))}
        if self.timoutcf() != 0 { try!(write!(f, " timoutcf"))}
        if self.peccf() != 0 { try!(write!(f, " peccf"))}
        if self.ovrcf() != 0 { try!(write!(f, " ovrcf"))}
        if self.arlocf() != 0 { try!(write!(f, " arlocf"))}
        if self.berrcf() != 0 { try!(write!(f, " berrcf"))}
        if self.stopcf() != 0 { try!(write!(f, " stopcf"))}
        if self.nackcf() != 0 { try!(write!(f, " nackcf"))}
        if self.addrcf() != 0 { try!(write!(f, " addrcf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PEC register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pecr(pub u32);
impl Pecr {
    #[doc="Packet error checking register"]
    #[inline] pub fn pec(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PEC != 0"]
    #[inline] pub fn test_pec(&self) -> bool {
        self.pec() != 0
    }

    #[doc="Sets the PEC field."]
    #[inline] pub fn set_pec<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pecr {
    #[inline]
    fn from(other: u32) -> Self {
         Pecr(other)
    }
}

impl ::core::fmt::Display for Pecr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pecr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pec() != 0 { try!(write!(f, " pec=0x{:x}", self.pec()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Receive data register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rxdr(pub u32);
impl Rxdr {
    #[doc="8-bit receive data"]
    #[inline] pub fn rxdata(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if RXDATA != 0"]
    #[inline] pub fn test_rxdata(&self) -> bool {
        self.rxdata() != 0
    }

    #[doc="Sets the RXDATA field."]
    #[inline] pub fn set_rxdata<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rxdr {
    #[inline]
    fn from(other: u32) -> Self {
         Rxdr(other)
    }
}

impl ::core::fmt::Display for Rxdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rxdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rxdata() != 0 { try!(write!(f, " rxdata=0x{:x}", self.rxdata()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Transmit data register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Txdr(pub u32);
impl Txdr {
    #[doc="8-bit transmit data"]
    #[inline] pub fn txdata(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if TXDATA != 0"]
    #[inline] pub fn test_txdata(&self) -> bool {
        self.txdata() != 0
    }

    #[doc="Sets the TXDATA field."]
    #[inline] pub fn set_txdata<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Txdr {
    #[inline]
    fn from(other: u32) -> Self {
         Txdr(other)
    }
}

impl ::core::fmt::Display for Txdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Txdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txdata() != 0 { try!(write!(f, " txdata=0x{:x}", self.txdata()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

