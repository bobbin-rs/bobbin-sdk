
#[allow(unused_imports)] use bobbin_common::*;

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="WDOG Peripheral"]
pub struct WdogPeriph(pub usize); 

impl WdogPeriph {
    #[doc="Get the STCTRLH Register."]
    #[inline] pub fn stctrlh_reg(&self) -> Register<Stctrlh> { 
        Register::new(self.0 as *mut Stctrlh, 0x0)
    }

    #[doc="Get the *mut pointer for the STCTRLH register."]
    #[inline] pub fn stctrlh_mut(&self) -> *mut Stctrlh { 
        self.stctrlh_reg().ptr()
    }

    #[doc="Get the *const pointer for the STCTRLH register."]
    #[inline] pub fn stctrlh_ptr(&self) -> *const Stctrlh { 
        self.stctrlh_reg().ptr()
    }

    #[doc="Read the STCTRLH register."]
    #[inline] pub fn stctrlh(&self) -> Stctrlh { 
        self.stctrlh_reg().read()
    }

    #[doc="Write the STCTRLH register."]
    #[inline] pub fn write_stctrlh(&self, value: Stctrlh) -> &Self { 
        self.stctrlh_reg().write(value);
        self
    }

    #[doc="Set the STCTRLH register."]
    #[inline] pub fn set_stctrlh<F: FnOnce(Stctrlh) -> Stctrlh>(&self, f: F) -> &Self {
        self.stctrlh_reg().set(f);
        self
    }

    #[doc="Modify the STCTRLH register."]
    #[inline] pub fn with_stctrlh<F: FnOnce(Stctrlh) -> Stctrlh>(&self, f: F) -> &Self {
        self.stctrlh_reg().with(f);
        self
    }

    #[doc="Get the STCTRLL Register."]
    #[inline] pub fn stctrll_reg(&self) -> Register<Stctrll> { 
        Register::new(self.0 as *mut Stctrll, 0x2)
    }

    #[doc="Get the *mut pointer for the STCTRLL register."]
    #[inline] pub fn stctrll_mut(&self) -> *mut Stctrll { 
        self.stctrll_reg().ptr()
    }

    #[doc="Get the *const pointer for the STCTRLL register."]
    #[inline] pub fn stctrll_ptr(&self) -> *const Stctrll { 
        self.stctrll_reg().ptr()
    }

    #[doc="Read the STCTRLL register."]
    #[inline] pub fn stctrll(&self) -> Stctrll { 
        self.stctrll_reg().read()
    }

    #[doc="Write the STCTRLL register."]
    #[inline] pub fn write_stctrll(&self, value: Stctrll) -> &Self { 
        self.stctrll_reg().write(value);
        self
    }

    #[doc="Set the STCTRLL register."]
    #[inline] pub fn set_stctrll<F: FnOnce(Stctrll) -> Stctrll>(&self, f: F) -> &Self {
        self.stctrll_reg().set(f);
        self
    }

    #[doc="Modify the STCTRLL register."]
    #[inline] pub fn with_stctrll<F: FnOnce(Stctrll) -> Stctrll>(&self, f: F) -> &Self {
        self.stctrll_reg().with(f);
        self
    }

    #[doc="Get the TOVALH Register."]
    #[inline] pub fn tovalh_reg(&self) -> Register<Tovalh> { 
        Register::new(self.0 as *mut Tovalh, 0x4)
    }

    #[doc="Get the *mut pointer for the TOVALH register."]
    #[inline] pub fn tovalh_mut(&self) -> *mut Tovalh { 
        self.tovalh_reg().ptr()
    }

    #[doc="Get the *const pointer for the TOVALH register."]
    #[inline] pub fn tovalh_ptr(&self) -> *const Tovalh { 
        self.tovalh_reg().ptr()
    }

    #[doc="Read the TOVALH register."]
    #[inline] pub fn tovalh(&self) -> Tovalh { 
        self.tovalh_reg().read()
    }

    #[doc="Write the TOVALH register."]
    #[inline] pub fn write_tovalh(&self, value: Tovalh) -> &Self { 
        self.tovalh_reg().write(value);
        self
    }

    #[doc="Set the TOVALH register."]
    #[inline] pub fn set_tovalh<F: FnOnce(Tovalh) -> Tovalh>(&self, f: F) -> &Self {
        self.tovalh_reg().set(f);
        self
    }

    #[doc="Modify the TOVALH register."]
    #[inline] pub fn with_tovalh<F: FnOnce(Tovalh) -> Tovalh>(&self, f: F) -> &Self {
        self.tovalh_reg().with(f);
        self
    }

    #[doc="Get the TOVALL Register."]
    #[inline] pub fn tovall_reg(&self) -> Register<Tovall> { 
        Register::new(self.0 as *mut Tovall, 0x6)
    }

    #[doc="Get the *mut pointer for the TOVALL register."]
    #[inline] pub fn tovall_mut(&self) -> *mut Tovall { 
        self.tovall_reg().ptr()
    }

    #[doc="Get the *const pointer for the TOVALL register."]
    #[inline] pub fn tovall_ptr(&self) -> *const Tovall { 
        self.tovall_reg().ptr()
    }

    #[doc="Read the TOVALL register."]
    #[inline] pub fn tovall(&self) -> Tovall { 
        self.tovall_reg().read()
    }

    #[doc="Write the TOVALL register."]
    #[inline] pub fn write_tovall(&self, value: Tovall) -> &Self { 
        self.tovall_reg().write(value);
        self
    }

    #[doc="Set the TOVALL register."]
    #[inline] pub fn set_tovall<F: FnOnce(Tovall) -> Tovall>(&self, f: F) -> &Self {
        self.tovall_reg().set(f);
        self
    }

    #[doc="Modify the TOVALL register."]
    #[inline] pub fn with_tovall<F: FnOnce(Tovall) -> Tovall>(&self, f: F) -> &Self {
        self.tovall_reg().with(f);
        self
    }

    #[doc="Get the WINH Register."]
    #[inline] pub fn winh_reg(&self) -> Register<Winh> { 
        Register::new(self.0 as *mut Winh, 0x8)
    }

    #[doc="Get the *mut pointer for the WINH register."]
    #[inline] pub fn winh_mut(&self) -> *mut Winh { 
        self.winh_reg().ptr()
    }

    #[doc="Get the *const pointer for the WINH register."]
    #[inline] pub fn winh_ptr(&self) -> *const Winh { 
        self.winh_reg().ptr()
    }

    #[doc="Read the WINH register."]
    #[inline] pub fn winh(&self) -> Winh { 
        self.winh_reg().read()
    }

    #[doc="Write the WINH register."]
    #[inline] pub fn write_winh(&self, value: Winh) -> &Self { 
        self.winh_reg().write(value);
        self
    }

    #[doc="Set the WINH register."]
    #[inline] pub fn set_winh<F: FnOnce(Winh) -> Winh>(&self, f: F) -> &Self {
        self.winh_reg().set(f);
        self
    }

    #[doc="Modify the WINH register."]
    #[inline] pub fn with_winh<F: FnOnce(Winh) -> Winh>(&self, f: F) -> &Self {
        self.winh_reg().with(f);
        self
    }

    #[doc="Get the WINL Register."]
    #[inline] pub fn winl_reg(&self) -> Register<Winl> { 
        Register::new(self.0 as *mut Winl, 0xa)
    }

    #[doc="Get the *mut pointer for the WINL register."]
    #[inline] pub fn winl_mut(&self) -> *mut Winl { 
        self.winl_reg().ptr()
    }

    #[doc="Get the *const pointer for the WINL register."]
    #[inline] pub fn winl_ptr(&self) -> *const Winl { 
        self.winl_reg().ptr()
    }

    #[doc="Read the WINL register."]
    #[inline] pub fn winl(&self) -> Winl { 
        self.winl_reg().read()
    }

    #[doc="Write the WINL register."]
    #[inline] pub fn write_winl(&self, value: Winl) -> &Self { 
        self.winl_reg().write(value);
        self
    }

    #[doc="Set the WINL register."]
    #[inline] pub fn set_winl<F: FnOnce(Winl) -> Winl>(&self, f: F) -> &Self {
        self.winl_reg().set(f);
        self
    }

    #[doc="Modify the WINL register."]
    #[inline] pub fn with_winl<F: FnOnce(Winl) -> Winl>(&self, f: F) -> &Self {
        self.winl_reg().with(f);
        self
    }

    #[doc="Get the REFRESH Register."]
    #[inline] pub fn refresh_reg(&self) -> Register<Refresh> { 
        Register::new(self.0 as *mut Refresh, 0xc)
    }

    #[doc="Get the *mut pointer for the REFRESH register."]
    #[inline] pub fn refresh_mut(&self) -> *mut Refresh { 
        self.refresh_reg().ptr()
    }

    #[doc="Get the *const pointer for the REFRESH register."]
    #[inline] pub fn refresh_ptr(&self) -> *const Refresh { 
        self.refresh_reg().ptr()
    }

    #[doc="Read the REFRESH register."]
    #[inline] pub fn refresh(&self) -> Refresh { 
        self.refresh_reg().read()
    }

    #[doc="Write the REFRESH register."]
    #[inline] pub fn write_refresh(&self, value: Refresh) -> &Self { 
        self.refresh_reg().write(value);
        self
    }

    #[doc="Set the REFRESH register."]
    #[inline] pub fn set_refresh<F: FnOnce(Refresh) -> Refresh>(&self, f: F) -> &Self {
        self.refresh_reg().set(f);
        self
    }

    #[doc="Modify the REFRESH register."]
    #[inline] pub fn with_refresh<F: FnOnce(Refresh) -> Refresh>(&self, f: F) -> &Self {
        self.refresh_reg().with(f);
        self
    }

    #[doc="Get the UNLOCK Register."]
    #[inline] pub fn unlock_reg(&self) -> Register<Unlock> { 
        Register::new(self.0 as *mut Unlock, 0xe)
    }

    #[doc="Get the *mut pointer for the UNLOCK register."]
    #[inline] pub fn unlock_mut(&self) -> *mut Unlock { 
        self.unlock_reg().ptr()
    }

    #[doc="Get the *const pointer for the UNLOCK register."]
    #[inline] pub fn unlock_ptr(&self) -> *const Unlock { 
        self.unlock_reg().ptr()
    }

    #[doc="Read the UNLOCK register."]
    #[inline] pub fn unlock(&self) -> Unlock { 
        self.unlock_reg().read()
    }

    #[doc="Write the UNLOCK register."]
    #[inline] pub fn write_unlock(&self, value: Unlock) -> &Self { 
        self.unlock_reg().write(value);
        self
    }

    #[doc="Set the UNLOCK register."]
    #[inline] pub fn set_unlock<F: FnOnce(Unlock) -> Unlock>(&self, f: F) -> &Self {
        self.unlock_reg().set(f);
        self
    }

    #[doc="Modify the UNLOCK register."]
    #[inline] pub fn with_unlock<F: FnOnce(Unlock) -> Unlock>(&self, f: F) -> &Self {
        self.unlock_reg().with(f);
        self
    }

    #[doc="Get the TMROUTH Register."]
    #[inline] pub fn tmrouth_reg(&self) -> Register<Tmrouth> { 
        Register::new(self.0 as *mut Tmrouth, 0x10)
    }

    #[doc="Get the *mut pointer for the TMROUTH register."]
    #[inline] pub fn tmrouth_mut(&self) -> *mut Tmrouth { 
        self.tmrouth_reg().ptr()
    }

    #[doc="Get the *const pointer for the TMROUTH register."]
    #[inline] pub fn tmrouth_ptr(&self) -> *const Tmrouth { 
        self.tmrouth_reg().ptr()
    }

    #[doc="Read the TMROUTH register."]
    #[inline] pub fn tmrouth(&self) -> Tmrouth { 
        self.tmrouth_reg().read()
    }

    #[doc="Write the TMROUTH register."]
    #[inline] pub fn write_tmrouth(&self, value: Tmrouth) -> &Self { 
        self.tmrouth_reg().write(value);
        self
    }

    #[doc="Set the TMROUTH register."]
    #[inline] pub fn set_tmrouth<F: FnOnce(Tmrouth) -> Tmrouth>(&self, f: F) -> &Self {
        self.tmrouth_reg().set(f);
        self
    }

    #[doc="Modify the TMROUTH register."]
    #[inline] pub fn with_tmrouth<F: FnOnce(Tmrouth) -> Tmrouth>(&self, f: F) -> &Self {
        self.tmrouth_reg().with(f);
        self
    }

    #[doc="Get the TMROUTL Register."]
    #[inline] pub fn tmroutl_reg(&self) -> Register<Tmroutl> { 
        Register::new(self.0 as *mut Tmroutl, 0x12)
    }

    #[doc="Get the *mut pointer for the TMROUTL register."]
    #[inline] pub fn tmroutl_mut(&self) -> *mut Tmroutl { 
        self.tmroutl_reg().ptr()
    }

    #[doc="Get the *const pointer for the TMROUTL register."]
    #[inline] pub fn tmroutl_ptr(&self) -> *const Tmroutl { 
        self.tmroutl_reg().ptr()
    }

    #[doc="Read the TMROUTL register."]
    #[inline] pub fn tmroutl(&self) -> Tmroutl { 
        self.tmroutl_reg().read()
    }

    #[doc="Write the TMROUTL register."]
    #[inline] pub fn write_tmroutl(&self, value: Tmroutl) -> &Self { 
        self.tmroutl_reg().write(value);
        self
    }

    #[doc="Set the TMROUTL register."]
    #[inline] pub fn set_tmroutl<F: FnOnce(Tmroutl) -> Tmroutl>(&self, f: F) -> &Self {
        self.tmroutl_reg().set(f);
        self
    }

    #[doc="Modify the TMROUTL register."]
    #[inline] pub fn with_tmroutl<F: FnOnce(Tmroutl) -> Tmroutl>(&self, f: F) -> &Self {
        self.tmroutl_reg().with(f);
        self
    }

    #[doc="Get the RSTCNT Register."]
    #[inline] pub fn rstcnt_reg(&self) -> Register<Rstcnt> { 
        Register::new(self.0 as *mut Rstcnt, 0x14)
    }

    #[doc="Get the *mut pointer for the RSTCNT register."]
    #[inline] pub fn rstcnt_mut(&self) -> *mut Rstcnt { 
        self.rstcnt_reg().ptr()
    }

    #[doc="Get the *const pointer for the RSTCNT register."]
    #[inline] pub fn rstcnt_ptr(&self) -> *const Rstcnt { 
        self.rstcnt_reg().ptr()
    }

    #[doc="Read the RSTCNT register."]
    #[inline] pub fn rstcnt(&self) -> Rstcnt { 
        self.rstcnt_reg().read()
    }

    #[doc="Write the RSTCNT register."]
    #[inline] pub fn write_rstcnt(&self, value: Rstcnt) -> &Self { 
        self.rstcnt_reg().write(value);
        self
    }

    #[doc="Set the RSTCNT register."]
    #[inline] pub fn set_rstcnt<F: FnOnce(Rstcnt) -> Rstcnt>(&self, f: F) -> &Self {
        self.rstcnt_reg().set(f);
        self
    }

    #[doc="Modify the RSTCNT register."]
    #[inline] pub fn with_rstcnt<F: FnOnce(Rstcnt) -> Rstcnt>(&self, f: F) -> &Self {
        self.rstcnt_reg().with(f);
        self
    }

    #[doc="Get the PRESC Register."]
    #[inline] pub fn presc_reg(&self) -> Register<Presc> { 
        Register::new(self.0 as *mut Presc, 0x16)
    }

    #[doc="Get the *mut pointer for the PRESC register."]
    #[inline] pub fn presc_mut(&self) -> *mut Presc { 
        self.presc_reg().ptr()
    }

    #[doc="Get the *const pointer for the PRESC register."]
    #[inline] pub fn presc_ptr(&self) -> *const Presc { 
        self.presc_reg().ptr()
    }

    #[doc="Read the PRESC register."]
    #[inline] pub fn presc(&self) -> Presc { 
        self.presc_reg().read()
    }

    #[doc="Write the PRESC register."]
    #[inline] pub fn write_presc(&self, value: Presc) -> &Self { 
        self.presc_reg().write(value);
        self
    }

    #[doc="Set the PRESC register."]
    #[inline] pub fn set_presc<F: FnOnce(Presc) -> Presc>(&self, f: F) -> &Self {
        self.presc_reg().set(f);
        self
    }

    #[doc="Modify the PRESC register."]
    #[inline] pub fn with_presc<F: FnOnce(Presc) -> Presc>(&self, f: F) -> &Self {
        self.presc_reg().with(f);
        self
    }

}

#[doc="Watchdog Status and Control Register High"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Stctrlh(pub u16);
impl Stctrlh {
    #[doc="Enables or disables the WDOG\'s operation"]
    #[inline] pub fn wdogen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if WDOGEN != 0"]
    #[inline] pub fn test_wdogen(&self) -> bool {
        self.wdogen() != 0
    }

    #[doc="Sets the WDOGEN field."]
    #[inline] pub fn set_wdogen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Selects clock source for the WDOG timer and other internal timing operations."]
    #[inline] pub fn clksrc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CLKSRC != 0"]
    #[inline] pub fn test_clksrc(&self) -> bool {
        self.clksrc() != 0
    }

    #[doc="Sets the CLKSRC field."]
    #[inline] pub fn set_clksrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Used to enable the debug breadcrumbs feature"]
    #[inline] pub fn irqrsten(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if IRQRSTEN != 0"]
    #[inline] pub fn test_irqrsten(&self) -> bool {
        self.irqrsten() != 0
    }

    #[doc="Sets the IRQRSTEN field."]
    #[inline] pub fn set_irqrsten<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Enables Windowing mode."]
    #[inline] pub fn winen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if WINEN != 0"]
    #[inline] pub fn test_winen(&self) -> bool {
        self.winen() != 0
    }

    #[doc="Sets the WINEN field."]
    #[inline] pub fn set_winen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Enables updates to watchdog write-once registers, after the reset-triggered initial configuration window (WCT) closes, through unlock sequence"]
    #[inline] pub fn allowupdate(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ALLOWUPDATE != 0"]
    #[inline] pub fn test_allowupdate(&self) -> bool {
        self.allowupdate() != 0
    }

    #[doc="Sets the ALLOWUPDATE field."]
    #[inline] pub fn set_allowupdate<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Enables or disables WDOG in Debug mode."]
    #[inline] pub fn dbgen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DBGEN != 0"]
    #[inline] pub fn test_dbgen(&self) -> bool {
        self.dbgen() != 0
    }

    #[doc="Sets the DBGEN field."]
    #[inline] pub fn set_dbgen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Enables or disables WDOG in Stop mode."]
    #[inline] pub fn stopen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if STOPEN != 0"]
    #[inline] pub fn test_stopen(&self) -> bool {
        self.stopen() != 0
    }

    #[doc="Sets the STOPEN field."]
    #[inline] pub fn set_stopen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Enables or disables WDOG in Wait mode."]
    #[inline] pub fn waiten(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if WAITEN != 0"]
    #[inline] pub fn test_waiten(&self) -> bool {
        self.waiten() != 0
    }

    #[doc="Sets the WAITEN field."]
    #[inline] pub fn set_waiten<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Puts the watchdog in the functional test mode"]
    #[inline] pub fn testwdog(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if TESTWDOG != 0"]
    #[inline] pub fn test_testwdog(&self) -> bool {
        self.testwdog() != 0
    }

    #[doc="Sets the TESTWDOG field."]
    #[inline] pub fn set_testwdog<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Effective only if TESTWDOG is set. Selects the test to be run on the watchdog timer."]
    #[inline] pub fn testsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if TESTSEL != 0"]
    #[inline] pub fn test_testsel(&self) -> bool {
        self.testsel() != 0
    }

    #[doc="Sets the TESTSEL field."]
    #[inline] pub fn set_testsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="This 2-bit field selects the byte to be tested when the watchdog is in the byte test mode."]
    #[inline] pub fn bytesel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if BYTESEL != 0"]
    #[inline] pub fn test_bytesel(&self) -> bool {
        self.bytesel() != 0
    }

    #[doc="Sets the BYTESEL field."]
    #[inline] pub fn set_bytesel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Allows the WDOG\'s functional test mode to be disabled permanently"]
    #[inline] pub fn distestwdog(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if DISTESTWDOG != 0"]
    #[inline] pub fn test_distestwdog(&self) -> bool {
        self.distestwdog() != 0
    }

    #[doc="Sets the DISTESTWDOG field."]
    #[inline] pub fn set_distestwdog<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

}

impl From<u16> for Stctrlh {
    #[inline]
    fn from(other: u16) -> Self {
         Stctrlh(other)
    }
}

impl ::core::fmt::Display for Stctrlh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Stctrlh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wdogen() != 0 { try!(write!(f, " wdogen"))}
        if self.clksrc() != 0 { try!(write!(f, " clksrc"))}
        if self.irqrsten() != 0 { try!(write!(f, " irqrsten"))}
        if self.winen() != 0 { try!(write!(f, " winen"))}
        if self.allowupdate() != 0 { try!(write!(f, " allowupdate"))}
        if self.dbgen() != 0 { try!(write!(f, " dbgen"))}
        if self.stopen() != 0 { try!(write!(f, " stopen"))}
        if self.waiten() != 0 { try!(write!(f, " waiten"))}
        if self.testwdog() != 0 { try!(write!(f, " testwdog"))}
        if self.testsel() != 0 { try!(write!(f, " testsel"))}
        if self.bytesel() != 0 { try!(write!(f, " bytesel=0x{:x}", self.bytesel()))}
        if self.distestwdog() != 0 { try!(write!(f, " distestwdog"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Watchdog Status and Control Register Low"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Stctrll(pub u16);
impl Stctrll {
    #[doc="Interrupt flag"]
    #[inline] pub fn intflg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if INTFLG != 0"]
    #[inline] pub fn test_intflg(&self) -> bool {
        self.intflg() != 0
    }

    #[doc="Sets the INTFLG field."]
    #[inline] pub fn set_intflg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Stctrll {
    #[inline]
    fn from(other: u16) -> Self {
         Stctrll(other)
    }
}

impl ::core::fmt::Display for Stctrll {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Stctrll {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.intflg() != 0 { try!(write!(f, " intflg"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Watchdog Time-out Value Register High"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tovalh(pub u16);
impl Tovalh {
    #[doc="Defines the upper 16 bits of the 32-bit time-out value for the watchdog timer"]
    #[inline] pub fn tovalhigh(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if TOVALHIGH != 0"]
    #[inline] pub fn test_tovalhigh(&self) -> bool {
        self.tovalhigh() != 0
    }

    #[doc="Sets the TOVALHIGH field."]
    #[inline] pub fn set_tovalhigh<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Tovalh {
    #[inline]
    fn from(other: u16) -> Self {
         Tovalh(other)
    }
}

impl ::core::fmt::Display for Tovalh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tovalh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tovalhigh() != 0 { try!(write!(f, " tovalhigh=0x{:x}", self.tovalhigh()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Watchdog Time-out Value Register Low"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tovall(pub u16);
impl Tovall {
    #[doc="Defines the lower 16 bits of the 32-bit time-out value for the watchdog timer"]
    #[inline] pub fn tovallow(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if TOVALLOW != 0"]
    #[inline] pub fn test_tovallow(&self) -> bool {
        self.tovallow() != 0
    }

    #[doc="Sets the TOVALLOW field."]
    #[inline] pub fn set_tovallow<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Tovall {
    #[inline]
    fn from(other: u16) -> Self {
         Tovall(other)
    }
}

impl ::core::fmt::Display for Tovall {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tovall {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tovallow() != 0 { try!(write!(f, " tovallow=0x{:x}", self.tovallow()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Watchdog Window Register High"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Winh(pub u16);
impl Winh {
    #[doc="Defines the upper 16 bits of the 32-bit window for the windowed mode of operation of the watchdog"]
    #[inline] pub fn winhigh(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if WINHIGH != 0"]
    #[inline] pub fn test_winhigh(&self) -> bool {
        self.winhigh() != 0
    }

    #[doc="Sets the WINHIGH field."]
    #[inline] pub fn set_winhigh<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Winh {
    #[inline]
    fn from(other: u16) -> Self {
         Winh(other)
    }
}

impl ::core::fmt::Display for Winh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Winh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.winhigh() != 0 { try!(write!(f, " winhigh=0x{:x}", self.winhigh()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Watchdog Window Register Low"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Winl(pub u16);
impl Winl {
    #[doc="Defines the lower 16 bits of the 32-bit window for the windowed mode of operation of the watchdog"]
    #[inline] pub fn winlow(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if WINLOW != 0"]
    #[inline] pub fn test_winlow(&self) -> bool {
        self.winlow() != 0
    }

    #[doc="Sets the WINLOW field."]
    #[inline] pub fn set_winlow<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Winl {
    #[inline]
    fn from(other: u16) -> Self {
         Winl(other)
    }
}

impl ::core::fmt::Display for Winl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Winl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.winlow() != 0 { try!(write!(f, " winlow=0x{:x}", self.winlow()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Watchdog Refresh register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Refresh(pub u16);
impl Refresh {
    #[doc="Watchdog refresh register"]
    #[inline] pub fn wdogrefresh(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if WDOGREFRESH != 0"]
    #[inline] pub fn test_wdogrefresh(&self) -> bool {
        self.wdogrefresh() != 0
    }

    #[doc="Sets the WDOGREFRESH field."]
    #[inline] pub fn set_wdogrefresh<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Refresh {
    #[inline]
    fn from(other: u16) -> Self {
         Refresh(other)
    }
}

impl ::core::fmt::Display for Refresh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Refresh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wdogrefresh() != 0 { try!(write!(f, " wdogrefresh=0x{:x}", self.wdogrefresh()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Watchdog Unlock register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Unlock(pub u16);
impl Unlock {
    #[doc="Writing the unlock sequence values to this register to makes the watchdog write-once registers writable again"]
    #[inline] pub fn wdogunlock(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if WDOGUNLOCK != 0"]
    #[inline] pub fn test_wdogunlock(&self) -> bool {
        self.wdogunlock() != 0
    }

    #[doc="Sets the WDOGUNLOCK field."]
    #[inline] pub fn set_wdogunlock<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Unlock {
    #[inline]
    fn from(other: u16) -> Self {
         Unlock(other)
    }
}

impl ::core::fmt::Display for Unlock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Unlock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wdogunlock() != 0 { try!(write!(f, " wdogunlock=0x{:x}", self.wdogunlock()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Watchdog Timer Output Register High"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tmrouth(pub u16);
impl Tmrouth {
    #[doc="Shows the value of the upper 16 bits of the watchdog timer."]
    #[inline] pub fn timerouthigh(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if TIMEROUTHIGH != 0"]
    #[inline] pub fn test_timerouthigh(&self) -> bool {
        self.timerouthigh() != 0
    }

    #[doc="Sets the TIMEROUTHIGH field."]
    #[inline] pub fn set_timerouthigh<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Tmrouth {
    #[inline]
    fn from(other: u16) -> Self {
         Tmrouth(other)
    }
}

impl ::core::fmt::Display for Tmrouth {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tmrouth {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.timerouthigh() != 0 { try!(write!(f, " timerouthigh=0x{:x}", self.timerouthigh()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Watchdog Timer Output Register Low"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tmroutl(pub u16);
impl Tmroutl {
    #[doc="Shows the value of the lower 16 bits of the watchdog timer."]
    #[inline] pub fn timeroutlow(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if TIMEROUTLOW != 0"]
    #[inline] pub fn test_timeroutlow(&self) -> bool {
        self.timeroutlow() != 0
    }

    #[doc="Sets the TIMEROUTLOW field."]
    #[inline] pub fn set_timeroutlow<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Tmroutl {
    #[inline]
    fn from(other: u16) -> Self {
         Tmroutl(other)
    }
}

impl ::core::fmt::Display for Tmroutl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tmroutl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.timeroutlow() != 0 { try!(write!(f, " timeroutlow=0x{:x}", self.timeroutlow()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Watchdog Reset Count register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rstcnt(pub u16);
impl Rstcnt {
    #[doc="Counts the number of times the watchdog resets the system"]
    #[inline] pub fn rstcnt(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if RSTCNT != 0"]
    #[inline] pub fn test_rstcnt(&self) -> bool {
        self.rstcnt() != 0
    }

    #[doc="Sets the RSTCNT field."]
    #[inline] pub fn set_rstcnt<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Rstcnt {
    #[inline]
    fn from(other: u16) -> Self {
         Rstcnt(other)
    }
}

impl ::core::fmt::Display for Rstcnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rstcnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rstcnt() != 0 { try!(write!(f, " rstcnt=0x{:x}", self.rstcnt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Watchdog Prescaler register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Presc(pub u16);
impl Presc {
    #[doc="3-bit prescaler for the watchdog clock source"]
    #[inline] pub fn prescval(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if PRESCVAL != 0"]
    #[inline] pub fn test_prescval(&self) -> bool {
        self.prescval() != 0
    }

    #[doc="Sets the PRESCVAL field."]
    #[inline] pub fn set_prescval<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u16> for Presc {
    #[inline]
    fn from(other: u16) -> Self {
         Presc(other)
    }
}

impl ::core::fmt::Display for Presc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Presc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.prescval() != 0 { try!(write!(f, " prescval=0x{:x}", self.prescval()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

