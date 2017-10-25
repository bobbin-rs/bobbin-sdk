#[allow(unused_imports)] use bobbin_common::*;

periph!( USB_FS_DEVICE, UsbFsDevice, _USB_FS_DEVICE, UsbFsDevicePeriph, 0x50000800);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="USB_FS_DEVICE Peripheral"]
pub struct UsbFsDevicePeriph(pub usize); 



impl UsbFsDevicePeriph {
    #[doc="Get the *mut pointer for the DCFG register."]
    #[inline] pub fn dcfg_mut(&self) -> *mut Dcfg { 
        (self.0 + 0x0) as *mut Dcfg
    }

    #[doc="Get the *const pointer for the DCFG register."]
    #[inline] pub fn dcfg_ptr(&self) -> *const Dcfg { 
           self.dcfg_mut()
    }

    #[doc="Read the DCFG register."]
    #[inline] pub fn dcfg(&self) -> Dcfg { 
        unsafe {
            read_volatile(self.dcfg_ptr())
        }
    }

    #[doc="Write the DCFG register."]
    #[inline] pub fn set_dcfg<F: FnOnce(Dcfg) -> Dcfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dcfg_mut(), f(Dcfg(0)));
        }
        self
    }

    #[doc="Modify the DCFG register."]
    #[inline] pub fn with_dcfg<F: FnOnce(Dcfg) -> Dcfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dcfg_mut(), f(self.dcfg()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DCTL register."]
    #[inline] pub fn dctl_mut(&self) -> *mut Dctl { 
        (self.0 + 0x4) as *mut Dctl
    }

    #[doc="Get the *const pointer for the DCTL register."]
    #[inline] pub fn dctl_ptr(&self) -> *const Dctl { 
           self.dctl_mut()
    }

    #[doc="Read the DCTL register."]
    #[inline] pub fn dctl(&self) -> Dctl { 
        unsafe {
            read_volatile(self.dctl_ptr())
        }
    }

    #[doc="Write the DCTL register."]
    #[inline] pub fn set_dctl<F: FnOnce(Dctl) -> Dctl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dctl_mut(), f(Dctl(0)));
        }
        self
    }

    #[doc="Modify the DCTL register."]
    #[inline] pub fn with_dctl<F: FnOnce(Dctl) -> Dctl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dctl_mut(), f(self.dctl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DSTS register."]
    #[inline] pub fn dsts_mut(&self) -> *mut Dsts { 
        (self.0 + 0x8) as *mut Dsts
    }

    #[doc="Get the *const pointer for the DSTS register."]
    #[inline] pub fn dsts_ptr(&self) -> *const Dsts { 
           self.dsts_mut()
    }

    #[doc="Read the DSTS register."]
    #[inline] pub fn dsts(&self) -> Dsts { 
        unsafe {
            read_volatile(self.dsts_ptr())
        }
    }

    #[doc="Get the *mut pointer for the DIEPMSK register."]
    #[inline] pub fn diepmsk_mut(&self) -> *mut Diepmsk { 
        (self.0 + 0x10) as *mut Diepmsk
    }

    #[doc="Get the *const pointer for the DIEPMSK register."]
    #[inline] pub fn diepmsk_ptr(&self) -> *const Diepmsk { 
           self.diepmsk_mut()
    }

    #[doc="Read the DIEPMSK register."]
    #[inline] pub fn diepmsk(&self) -> Diepmsk { 
        unsafe {
            read_volatile(self.diepmsk_ptr())
        }
    }

    #[doc="Write the DIEPMSK register."]
    #[inline] pub fn set_diepmsk<F: FnOnce(Diepmsk) -> Diepmsk>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.diepmsk_mut(), f(Diepmsk(0)));
        }
        self
    }

    #[doc="Modify the DIEPMSK register."]
    #[inline] pub fn with_diepmsk<F: FnOnce(Diepmsk) -> Diepmsk>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.diepmsk_mut(), f(self.diepmsk()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DOEPMSK register."]
    #[inline] pub fn doepmsk_mut(&self) -> *mut Doepmsk { 
        (self.0 + 0x14) as *mut Doepmsk
    }

    #[doc="Get the *const pointer for the DOEPMSK register."]
    #[inline] pub fn doepmsk_ptr(&self) -> *const Doepmsk { 
           self.doepmsk_mut()
    }

    #[doc="Read the DOEPMSK register."]
    #[inline] pub fn doepmsk(&self) -> Doepmsk { 
        unsafe {
            read_volatile(self.doepmsk_ptr())
        }
    }

    #[doc="Write the DOEPMSK register."]
    #[inline] pub fn set_doepmsk<F: FnOnce(Doepmsk) -> Doepmsk>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.doepmsk_mut(), f(Doepmsk(0)));
        }
        self
    }

    #[doc="Modify the DOEPMSK register."]
    #[inline] pub fn with_doepmsk<F: FnOnce(Doepmsk) -> Doepmsk>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.doepmsk_mut(), f(self.doepmsk()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DAINT register."]
    #[inline] pub fn daint_mut(&self) -> *mut Daint { 
        (self.0 + 0x18) as *mut Daint
    }

    #[doc="Get the *const pointer for the DAINT register."]
    #[inline] pub fn daint_ptr(&self) -> *const Daint { 
           self.daint_mut()
    }

    #[doc="Read the DAINT register."]
    #[inline] pub fn daint(&self) -> Daint { 
        unsafe {
            read_volatile(self.daint_ptr())
        }
    }

    #[doc="Get the *mut pointer for the DAINTMSK register."]
    #[inline] pub fn daintmsk_mut(&self) -> *mut Daintmsk { 
        (self.0 + 0x1c) as *mut Daintmsk
    }

    #[doc="Get the *const pointer for the DAINTMSK register."]
    #[inline] pub fn daintmsk_ptr(&self) -> *const Daintmsk { 
           self.daintmsk_mut()
    }

    #[doc="Read the DAINTMSK register."]
    #[inline] pub fn daintmsk(&self) -> Daintmsk { 
        unsafe {
            read_volatile(self.daintmsk_ptr())
        }
    }

    #[doc="Write the DAINTMSK register."]
    #[inline] pub fn set_daintmsk<F: FnOnce(Daintmsk) -> Daintmsk>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.daintmsk_mut(), f(Daintmsk(0)));
        }
        self
    }

    #[doc="Modify the DAINTMSK register."]
    #[inline] pub fn with_daintmsk<F: FnOnce(Daintmsk) -> Daintmsk>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.daintmsk_mut(), f(self.daintmsk()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DVBUSDIS register."]
    #[inline] pub fn dvbusdis_mut(&self) -> *mut Dvbusdis { 
        (self.0 + 0x28) as *mut Dvbusdis
    }

    #[doc="Get the *const pointer for the DVBUSDIS register."]
    #[inline] pub fn dvbusdis_ptr(&self) -> *const Dvbusdis { 
           self.dvbusdis_mut()
    }

    #[doc="Read the DVBUSDIS register."]
    #[inline] pub fn dvbusdis(&self) -> Dvbusdis { 
        unsafe {
            read_volatile(self.dvbusdis_ptr())
        }
    }

    #[doc="Write the DVBUSDIS register."]
    #[inline] pub fn set_dvbusdis<F: FnOnce(Dvbusdis) -> Dvbusdis>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dvbusdis_mut(), f(Dvbusdis(0)));
        }
        self
    }

    #[doc="Modify the DVBUSDIS register."]
    #[inline] pub fn with_dvbusdis<F: FnOnce(Dvbusdis) -> Dvbusdis>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dvbusdis_mut(), f(self.dvbusdis()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DVBUSPULSE register."]
    #[inline] pub fn dvbuspulse_mut(&self) -> *mut Dvbuspulse { 
        (self.0 + 0x2c) as *mut Dvbuspulse
    }

    #[doc="Get the *const pointer for the DVBUSPULSE register."]
    #[inline] pub fn dvbuspulse_ptr(&self) -> *const Dvbuspulse { 
           self.dvbuspulse_mut()
    }

    #[doc="Read the DVBUSPULSE register."]
    #[inline] pub fn dvbuspulse(&self) -> Dvbuspulse { 
        unsafe {
            read_volatile(self.dvbuspulse_ptr())
        }
    }

    #[doc="Write the DVBUSPULSE register."]
    #[inline] pub fn set_dvbuspulse<F: FnOnce(Dvbuspulse) -> Dvbuspulse>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dvbuspulse_mut(), f(Dvbuspulse(0)));
        }
        self
    }

    #[doc="Modify the DVBUSPULSE register."]
    #[inline] pub fn with_dvbuspulse<F: FnOnce(Dvbuspulse) -> Dvbuspulse>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dvbuspulse_mut(), f(self.dvbuspulse()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DIEPEMPMSK register."]
    #[inline] pub fn diepempmsk_mut(&self) -> *mut Diepempmsk { 
        (self.0 + 0x34) as *mut Diepempmsk
    }

    #[doc="Get the *const pointer for the DIEPEMPMSK register."]
    #[inline] pub fn diepempmsk_ptr(&self) -> *const Diepempmsk { 
           self.diepempmsk_mut()
    }

    #[doc="Read the DIEPEMPMSK register."]
    #[inline] pub fn diepempmsk(&self) -> Diepempmsk { 
        unsafe {
            read_volatile(self.diepempmsk_ptr())
        }
    }

    #[doc="Write the DIEPEMPMSK register."]
    #[inline] pub fn set_diepempmsk<F: FnOnce(Diepempmsk) -> Diepempmsk>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.diepempmsk_mut(), f(Diepempmsk(0)));
        }
        self
    }

    #[doc="Modify the DIEPEMPMSK register."]
    #[inline] pub fn with_diepempmsk<F: FnOnce(Diepempmsk) -> Diepempmsk>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.diepempmsk_mut(), f(self.diepempmsk()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DIEPCTL0 register."]
    #[inline] pub fn diepctl0_mut(&self) -> *mut Diepctl0 { 
        (self.0 + 0x100) as *mut Diepctl0
    }

    #[doc="Get the *const pointer for the DIEPCTL0 register."]
    #[inline] pub fn diepctl0_ptr(&self) -> *const Diepctl0 { 
           self.diepctl0_mut()
    }

    #[doc="Read the DIEPCTL0 register."]
    #[inline] pub fn diepctl0(&self) -> Diepctl0 { 
        unsafe {
            read_volatile(self.diepctl0_ptr())
        }
    }

    #[doc="Write the DIEPCTL0 register."]
    #[inline] pub fn set_diepctl0<F: FnOnce(Diepctl0) -> Diepctl0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.diepctl0_mut(), f(Diepctl0(0)));
        }
        self
    }

    #[doc="Modify the DIEPCTL0 register."]
    #[inline] pub fn with_diepctl0<F: FnOnce(Diepctl0) -> Diepctl0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.diepctl0_mut(), f(self.diepctl0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DIEPCTL register."]
    #[inline] pub fn diepctl_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Diepctl { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x100 + (index * 32)) as *mut Diepctl
    }

    #[doc="Get the *const pointer for the DIEPCTL register."]
    #[inline] pub fn diepctl_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Diepctl { 
           self.diepctl_mut(index)
    }

    #[doc="Read the DIEPCTL register."]
    #[inline] pub fn diepctl<I: Into<bits::R4>>(&self, index: I) -> Diepctl { 
        unsafe {
            read_volatile(self.diepctl_ptr(index))
        }
    }

    #[doc="Write the DIEPCTL register."]
    #[inline] pub fn set_diepctl<I: Into<bits::R4>, F: FnOnce(Diepctl) -> Diepctl>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.diepctl_mut(index), f(Diepctl(0)));
        }
        self
    }

    #[doc="Modify the DIEPCTL register."]
    #[inline] pub fn with_diepctl<I: Into<bits::R4> + Copy, F: FnOnce(Diepctl) -> Diepctl>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.diepctl_mut(index), f(self.diepctl(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the DOEPCTL0 register."]
    #[inline] pub fn doepctl0_mut(&self) -> *mut Doepctl0 { 
        (self.0 + 0x300) as *mut Doepctl0
    }

    #[doc="Get the *const pointer for the DOEPCTL0 register."]
    #[inline] pub fn doepctl0_ptr(&self) -> *const Doepctl0 { 
           self.doepctl0_mut()
    }

    #[doc="Read the DOEPCTL0 register."]
    #[inline] pub fn doepctl0(&self) -> Doepctl0 { 
        unsafe {
            read_volatile(self.doepctl0_ptr())
        }
    }

    #[doc="Write the DOEPCTL0 register."]
    #[inline] pub fn set_doepctl0<F: FnOnce(Doepctl0) -> Doepctl0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.doepctl0_mut(), f(Doepctl0(0)));
        }
        self
    }

    #[doc="Modify the DOEPCTL0 register."]
    #[inline] pub fn with_doepctl0<F: FnOnce(Doepctl0) -> Doepctl0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.doepctl0_mut(), f(self.doepctl0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DOEPCTL register."]
    #[inline] pub fn doepctl_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Doepctl { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x300 + (index * 32)) as *mut Doepctl
    }

    #[doc="Get the *const pointer for the DOEPCTL register."]
    #[inline] pub fn doepctl_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Doepctl { 
           self.doepctl_mut(index)
    }

    #[doc="Read the DOEPCTL register."]
    #[inline] pub fn doepctl<I: Into<bits::R4>>(&self, index: I) -> Doepctl { 
        unsafe {
            read_volatile(self.doepctl_ptr(index))
        }
    }

    #[doc="Write the DOEPCTL register."]
    #[inline] pub fn set_doepctl<I: Into<bits::R4>, F: FnOnce(Doepctl) -> Doepctl>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.doepctl_mut(index), f(Doepctl(0)));
        }
        self
    }

    #[doc="Modify the DOEPCTL register."]
    #[inline] pub fn with_doepctl<I: Into<bits::R4> + Copy, F: FnOnce(Doepctl) -> Doepctl>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.doepctl_mut(index), f(self.doepctl(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the DIEPINT register."]
    #[inline] pub fn diepint_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Diepint { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x108 + (index * 32)) as *mut Diepint
    }

    #[doc="Get the *const pointer for the DIEPINT register."]
    #[inline] pub fn diepint_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Diepint { 
           self.diepint_mut(index)
    }

    #[doc="Read the DIEPINT register."]
    #[inline] pub fn diepint<I: Into<bits::R4>>(&self, index: I) -> Diepint { 
        unsafe {
            read_volatile(self.diepint_ptr(index))
        }
    }

    #[doc="Write the DIEPINT register."]
    #[inline] pub fn set_diepint<I: Into<bits::R4>, F: FnOnce(Diepint) -> Diepint>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.diepint_mut(index), f(Diepint(0)));
        }
        self
    }

    #[doc="Modify the DIEPINT register."]
    #[inline] pub fn with_diepint<I: Into<bits::R4> + Copy, F: FnOnce(Diepint) -> Diepint>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.diepint_mut(index), f(self.diepint(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the DOEPINT register."]
    #[inline] pub fn doepint_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Doepint { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x308 + (index * 32)) as *mut Doepint
    }

    #[doc="Get the *const pointer for the DOEPINT register."]
    #[inline] pub fn doepint_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Doepint { 
           self.doepint_mut(index)
    }

    #[doc="Read the DOEPINT register."]
    #[inline] pub fn doepint<I: Into<bits::R4>>(&self, index: I) -> Doepint { 
        unsafe {
            read_volatile(self.doepint_ptr(index))
        }
    }

    #[doc="Write the DOEPINT register."]
    #[inline] pub fn set_doepint<I: Into<bits::R4>, F: FnOnce(Doepint) -> Doepint>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.doepint_mut(index), f(Doepint(0)));
        }
        self
    }

    #[doc="Modify the DOEPINT register."]
    #[inline] pub fn with_doepint<I: Into<bits::R4> + Copy, F: FnOnce(Doepint) -> Doepint>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.doepint_mut(index), f(self.doepint(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the DIEPTSIZ0 register."]
    #[inline] pub fn dieptsiz0_mut(&self) -> *mut Dieptsiz0 { 
        (self.0 + 0x110) as *mut Dieptsiz0
    }

    #[doc="Get the *const pointer for the DIEPTSIZ0 register."]
    #[inline] pub fn dieptsiz0_ptr(&self) -> *const Dieptsiz0 { 
           self.dieptsiz0_mut()
    }

    #[doc="Read the DIEPTSIZ0 register."]
    #[inline] pub fn dieptsiz0(&self) -> Dieptsiz0 { 
        unsafe {
            read_volatile(self.dieptsiz0_ptr())
        }
    }

    #[doc="Write the DIEPTSIZ0 register."]
    #[inline] pub fn set_dieptsiz0<F: FnOnce(Dieptsiz0) -> Dieptsiz0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dieptsiz0_mut(), f(Dieptsiz0(0)));
        }
        self
    }

    #[doc="Modify the DIEPTSIZ0 register."]
    #[inline] pub fn with_dieptsiz0<F: FnOnce(Dieptsiz0) -> Dieptsiz0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dieptsiz0_mut(), f(self.dieptsiz0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DOEPTSIZ0 register."]
    #[inline] pub fn doeptsiz0_mut(&self) -> *mut Doeptsiz0 { 
        (self.0 + 0x310) as *mut Doeptsiz0
    }

    #[doc="Get the *const pointer for the DOEPTSIZ0 register."]
    #[inline] pub fn doeptsiz0_ptr(&self) -> *const Doeptsiz0 { 
           self.doeptsiz0_mut()
    }

    #[doc="Read the DOEPTSIZ0 register."]
    #[inline] pub fn doeptsiz0(&self) -> Doeptsiz0 { 
        unsafe {
            read_volatile(self.doeptsiz0_ptr())
        }
    }

    #[doc="Write the DOEPTSIZ0 register."]
    #[inline] pub fn set_doeptsiz0<F: FnOnce(Doeptsiz0) -> Doeptsiz0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.doeptsiz0_mut(), f(Doeptsiz0(0)));
        }
        self
    }

    #[doc="Modify the DOEPTSIZ0 register."]
    #[inline] pub fn with_doeptsiz0<F: FnOnce(Doeptsiz0) -> Doeptsiz0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.doeptsiz0_mut(), f(self.doeptsiz0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DIEPTSIZ1 register."]
    #[inline] pub fn dieptsiz1_mut(&self) -> *mut Dieptsiz1 { 
        (self.0 + 0x130) as *mut Dieptsiz1
    }

    #[doc="Get the *const pointer for the DIEPTSIZ1 register."]
    #[inline] pub fn dieptsiz1_ptr(&self) -> *const Dieptsiz1 { 
           self.dieptsiz1_mut()
    }

    #[doc="Read the DIEPTSIZ1 register."]
    #[inline] pub fn dieptsiz1(&self) -> Dieptsiz1 { 
        unsafe {
            read_volatile(self.dieptsiz1_ptr())
        }
    }

    #[doc="Write the DIEPTSIZ1 register."]
    #[inline] pub fn set_dieptsiz1<F: FnOnce(Dieptsiz1) -> Dieptsiz1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dieptsiz1_mut(), f(Dieptsiz1(0)));
        }
        self
    }

    #[doc="Modify the DIEPTSIZ1 register."]
    #[inline] pub fn with_dieptsiz1<F: FnOnce(Dieptsiz1) -> Dieptsiz1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dieptsiz1_mut(), f(self.dieptsiz1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DIEPTSIZ2 register."]
    #[inline] pub fn dieptsiz2_mut(&self) -> *mut Dieptsiz2 { 
        (self.0 + 0x150) as *mut Dieptsiz2
    }

    #[doc="Get the *const pointer for the DIEPTSIZ2 register."]
    #[inline] pub fn dieptsiz2_ptr(&self) -> *const Dieptsiz2 { 
           self.dieptsiz2_mut()
    }

    #[doc="Read the DIEPTSIZ2 register."]
    #[inline] pub fn dieptsiz2(&self) -> Dieptsiz2 { 
        unsafe {
            read_volatile(self.dieptsiz2_ptr())
        }
    }

    #[doc="Write the DIEPTSIZ2 register."]
    #[inline] pub fn set_dieptsiz2<F: FnOnce(Dieptsiz2) -> Dieptsiz2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dieptsiz2_mut(), f(Dieptsiz2(0)));
        }
        self
    }

    #[doc="Modify the DIEPTSIZ2 register."]
    #[inline] pub fn with_dieptsiz2<F: FnOnce(Dieptsiz2) -> Dieptsiz2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dieptsiz2_mut(), f(self.dieptsiz2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DIEPTSIZ3 register."]
    #[inline] pub fn dieptsiz3_mut(&self) -> *mut Dieptsiz3 { 
        (self.0 + 0x170) as *mut Dieptsiz3
    }

    #[doc="Get the *const pointer for the DIEPTSIZ3 register."]
    #[inline] pub fn dieptsiz3_ptr(&self) -> *const Dieptsiz3 { 
           self.dieptsiz3_mut()
    }

    #[doc="Read the DIEPTSIZ3 register."]
    #[inline] pub fn dieptsiz3(&self) -> Dieptsiz3 { 
        unsafe {
            read_volatile(self.dieptsiz3_ptr())
        }
    }

    #[doc="Write the DIEPTSIZ3 register."]
    #[inline] pub fn set_dieptsiz3<F: FnOnce(Dieptsiz3) -> Dieptsiz3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dieptsiz3_mut(), f(Dieptsiz3(0)));
        }
        self
    }

    #[doc="Modify the DIEPTSIZ3 register."]
    #[inline] pub fn with_dieptsiz3<F: FnOnce(Dieptsiz3) -> Dieptsiz3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dieptsiz3_mut(), f(self.dieptsiz3()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DTXFSTS0 register."]
    #[inline] pub fn dtxfsts0_mut(&self) -> *mut Dtxfsts0 { 
        (self.0 + 0x118) as *mut Dtxfsts0
    }

    #[doc="Get the *const pointer for the DTXFSTS0 register."]
    #[inline] pub fn dtxfsts0_ptr(&self) -> *const Dtxfsts0 { 
           self.dtxfsts0_mut()
    }

    #[doc="Read the DTXFSTS0 register."]
    #[inline] pub fn dtxfsts0(&self) -> Dtxfsts0 { 
        unsafe {
            read_volatile(self.dtxfsts0_ptr())
        }
    }

    #[doc="Get the *mut pointer for the DTXFSTS1 register."]
    #[inline] pub fn dtxfsts1_mut(&self) -> *mut Dtxfsts1 { 
        (self.0 + 0x138) as *mut Dtxfsts1
    }

    #[doc="Get the *const pointer for the DTXFSTS1 register."]
    #[inline] pub fn dtxfsts1_ptr(&self) -> *const Dtxfsts1 { 
           self.dtxfsts1_mut()
    }

    #[doc="Read the DTXFSTS1 register."]
    #[inline] pub fn dtxfsts1(&self) -> Dtxfsts1 { 
        unsafe {
            read_volatile(self.dtxfsts1_ptr())
        }
    }

    #[doc="Get the *mut pointer for the DTXFSTS2 register."]
    #[inline] pub fn dtxfsts2_mut(&self) -> *mut Dtxfsts2 { 
        (self.0 + 0x158) as *mut Dtxfsts2
    }

    #[doc="Get the *const pointer for the DTXFSTS2 register."]
    #[inline] pub fn dtxfsts2_ptr(&self) -> *const Dtxfsts2 { 
           self.dtxfsts2_mut()
    }

    #[doc="Read the DTXFSTS2 register."]
    #[inline] pub fn dtxfsts2(&self) -> Dtxfsts2 { 
        unsafe {
            read_volatile(self.dtxfsts2_ptr())
        }
    }

    #[doc="Get the *mut pointer for the DTXFSTS3 register."]
    #[inline] pub fn dtxfsts3_mut(&self) -> *mut Dtxfsts3 { 
        (self.0 + 0x178) as *mut Dtxfsts3
    }

    #[doc="Get the *const pointer for the DTXFSTS3 register."]
    #[inline] pub fn dtxfsts3_ptr(&self) -> *const Dtxfsts3 { 
           self.dtxfsts3_mut()
    }

    #[doc="Read the DTXFSTS3 register."]
    #[inline] pub fn dtxfsts3(&self) -> Dtxfsts3 { 
        unsafe {
            read_volatile(self.dtxfsts3_ptr())
        }
    }

    #[doc="Get the *mut pointer for the DOEPTSIZ1 register."]
    #[inline] pub fn doeptsiz1_mut(&self) -> *mut Doeptsiz1 { 
        (self.0 + 0x330) as *mut Doeptsiz1
    }

    #[doc="Get the *const pointer for the DOEPTSIZ1 register."]
    #[inline] pub fn doeptsiz1_ptr(&self) -> *const Doeptsiz1 { 
           self.doeptsiz1_mut()
    }

    #[doc="Read the DOEPTSIZ1 register."]
    #[inline] pub fn doeptsiz1(&self) -> Doeptsiz1 { 
        unsafe {
            read_volatile(self.doeptsiz1_ptr())
        }
    }

    #[doc="Write the DOEPTSIZ1 register."]
    #[inline] pub fn set_doeptsiz1<F: FnOnce(Doeptsiz1) -> Doeptsiz1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.doeptsiz1_mut(), f(Doeptsiz1(0)));
        }
        self
    }

    #[doc="Modify the DOEPTSIZ1 register."]
    #[inline] pub fn with_doeptsiz1<F: FnOnce(Doeptsiz1) -> Doeptsiz1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.doeptsiz1_mut(), f(self.doeptsiz1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DOEPTSIZ2 register."]
    #[inline] pub fn doeptsiz2_mut(&self) -> *mut Doeptsiz2 { 
        (self.0 + 0x350) as *mut Doeptsiz2
    }

    #[doc="Get the *const pointer for the DOEPTSIZ2 register."]
    #[inline] pub fn doeptsiz2_ptr(&self) -> *const Doeptsiz2 { 
           self.doeptsiz2_mut()
    }

    #[doc="Read the DOEPTSIZ2 register."]
    #[inline] pub fn doeptsiz2(&self) -> Doeptsiz2 { 
        unsafe {
            read_volatile(self.doeptsiz2_ptr())
        }
    }

    #[doc="Write the DOEPTSIZ2 register."]
    #[inline] pub fn set_doeptsiz2<F: FnOnce(Doeptsiz2) -> Doeptsiz2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.doeptsiz2_mut(), f(Doeptsiz2(0)));
        }
        self
    }

    #[doc="Modify the DOEPTSIZ2 register."]
    #[inline] pub fn with_doeptsiz2<F: FnOnce(Doeptsiz2) -> Doeptsiz2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.doeptsiz2_mut(), f(self.doeptsiz2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DOEPTSIZ3 register."]
    #[inline] pub fn doeptsiz3_mut(&self) -> *mut Doeptsiz3 { 
        (self.0 + 0x370) as *mut Doeptsiz3
    }

    #[doc="Get the *const pointer for the DOEPTSIZ3 register."]
    #[inline] pub fn doeptsiz3_ptr(&self) -> *const Doeptsiz3 { 
           self.doeptsiz3_mut()
    }

    #[doc="Read the DOEPTSIZ3 register."]
    #[inline] pub fn doeptsiz3(&self) -> Doeptsiz3 { 
        unsafe {
            read_volatile(self.doeptsiz3_ptr())
        }
    }

    #[doc="Write the DOEPTSIZ3 register."]
    #[inline] pub fn set_doeptsiz3<F: FnOnce(Doeptsiz3) -> Doeptsiz3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.doeptsiz3_mut(), f(Doeptsiz3(0)));
        }
        self
    }

    #[doc="Modify the DOEPTSIZ3 register."]
    #[inline] pub fn with_doeptsiz3<F: FnOnce(Doeptsiz3) -> Doeptsiz3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.doeptsiz3_mut(), f(self.doeptsiz3()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DIEPCTL4 register."]
    #[inline] pub fn diepctl4_mut(&self) -> *mut Diepctl4 { 
        (self.0 + 0x180) as *mut Diepctl4
    }

    #[doc="Get the *const pointer for the DIEPCTL4 register."]
    #[inline] pub fn diepctl4_ptr(&self) -> *const Diepctl4 { 
           self.diepctl4_mut()
    }

    #[doc="Read the DIEPCTL4 register."]
    #[inline] pub fn diepctl4(&self) -> Diepctl4 { 
        unsafe {
            read_volatile(self.diepctl4_ptr())
        }
    }

    #[doc="Write the DIEPCTL4 register."]
    #[inline] pub fn set_diepctl4<F: FnOnce(Diepctl4) -> Diepctl4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.diepctl4_mut(), f(Diepctl4(0)));
        }
        self
    }

    #[doc="Modify the DIEPCTL4 register."]
    #[inline] pub fn with_diepctl4<F: FnOnce(Diepctl4) -> Diepctl4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.diepctl4_mut(), f(self.diepctl4()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DIEPINT4 register."]
    #[inline] pub fn diepint4_mut(&self) -> *mut Diepint4 { 
        (self.0 + 0x188) as *mut Diepint4
    }

    #[doc="Get the *const pointer for the DIEPINT4 register."]
    #[inline] pub fn diepint4_ptr(&self) -> *const Diepint4 { 
           self.diepint4_mut()
    }

    #[doc="Read the DIEPINT4 register."]
    #[inline] pub fn diepint4(&self) -> Diepint4 { 
        unsafe {
            read_volatile(self.diepint4_ptr())
        }
    }

    #[doc="Write the DIEPINT4 register."]
    #[inline] pub fn set_diepint4<F: FnOnce(Diepint4) -> Diepint4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.diepint4_mut(), f(Diepint4(0)));
        }
        self
    }

    #[doc="Modify the DIEPINT4 register."]
    #[inline] pub fn with_diepint4<F: FnOnce(Diepint4) -> Diepint4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.diepint4_mut(), f(self.diepint4()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DIEPTSIZ4 register."]
    #[inline] pub fn dieptsiz4_mut(&self) -> *mut Dieptsiz4 { 
        (self.0 + 0x194) as *mut Dieptsiz4
    }

    #[doc="Get the *const pointer for the DIEPTSIZ4 register."]
    #[inline] pub fn dieptsiz4_ptr(&self) -> *const Dieptsiz4 { 
           self.dieptsiz4_mut()
    }

    #[doc="Read the DIEPTSIZ4 register."]
    #[inline] pub fn dieptsiz4(&self) -> Dieptsiz4 { 
        unsafe {
            read_volatile(self.dieptsiz4_ptr())
        }
    }

    #[doc="Write the DIEPTSIZ4 register."]
    #[inline] pub fn set_dieptsiz4<F: FnOnce(Dieptsiz4) -> Dieptsiz4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dieptsiz4_mut(), f(Dieptsiz4(0)));
        }
        self
    }

    #[doc="Modify the DIEPTSIZ4 register."]
    #[inline] pub fn with_dieptsiz4<F: FnOnce(Dieptsiz4) -> Dieptsiz4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dieptsiz4_mut(), f(self.dieptsiz4()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DTXFSTS4 register."]
    #[inline] pub fn dtxfsts4_mut(&self) -> *mut Dtxfsts4 { 
        (self.0 + 0x19c) as *mut Dtxfsts4
    }

    #[doc="Get the *const pointer for the DTXFSTS4 register."]
    #[inline] pub fn dtxfsts4_ptr(&self) -> *const Dtxfsts4 { 
           self.dtxfsts4_mut()
    }

    #[doc="Read the DTXFSTS4 register."]
    #[inline] pub fn dtxfsts4(&self) -> Dtxfsts4 { 
        unsafe {
            read_volatile(self.dtxfsts4_ptr())
        }
    }

    #[doc="Write the DTXFSTS4 register."]
    #[inline] pub fn set_dtxfsts4<F: FnOnce(Dtxfsts4) -> Dtxfsts4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dtxfsts4_mut(), f(Dtxfsts4(0)));
        }
        self
    }

    #[doc="Modify the DTXFSTS4 register."]
    #[inline] pub fn with_dtxfsts4<F: FnOnce(Dtxfsts4) -> Dtxfsts4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dtxfsts4_mut(), f(self.dtxfsts4()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DIEPCTL5 register."]
    #[inline] pub fn diepctl5_mut(&self) -> *mut Diepctl5 { 
        (self.0 + 0x1a0) as *mut Diepctl5
    }

    #[doc="Get the *const pointer for the DIEPCTL5 register."]
    #[inline] pub fn diepctl5_ptr(&self) -> *const Diepctl5 { 
           self.diepctl5_mut()
    }

    #[doc="Read the DIEPCTL5 register."]
    #[inline] pub fn diepctl5(&self) -> Diepctl5 { 
        unsafe {
            read_volatile(self.diepctl5_ptr())
        }
    }

    #[doc="Write the DIEPCTL5 register."]
    #[inline] pub fn set_diepctl5<F: FnOnce(Diepctl5) -> Diepctl5>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.diepctl5_mut(), f(Diepctl5(0)));
        }
        self
    }

    #[doc="Modify the DIEPCTL5 register."]
    #[inline] pub fn with_diepctl5<F: FnOnce(Diepctl5) -> Diepctl5>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.diepctl5_mut(), f(self.diepctl5()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DIEPINT5 register."]
    #[inline] pub fn diepint5_mut(&self) -> *mut Diepint5 { 
        (self.0 + 0x1a8) as *mut Diepint5
    }

    #[doc="Get the *const pointer for the DIEPINT5 register."]
    #[inline] pub fn diepint5_ptr(&self) -> *const Diepint5 { 
           self.diepint5_mut()
    }

    #[doc="Read the DIEPINT5 register."]
    #[inline] pub fn diepint5(&self) -> Diepint5 { 
        unsafe {
            read_volatile(self.diepint5_ptr())
        }
    }

    #[doc="Write the DIEPINT5 register."]
    #[inline] pub fn set_diepint5<F: FnOnce(Diepint5) -> Diepint5>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.diepint5_mut(), f(Diepint5(0)));
        }
        self
    }

    #[doc="Modify the DIEPINT5 register."]
    #[inline] pub fn with_diepint5<F: FnOnce(Diepint5) -> Diepint5>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.diepint5_mut(), f(self.diepint5()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DIEPTSIZ55 register."]
    #[inline] pub fn dieptsiz55_mut(&self) -> *mut Dieptsiz55 { 
        (self.0 + 0x1b0) as *mut Dieptsiz55
    }

    #[doc="Get the *const pointer for the DIEPTSIZ55 register."]
    #[inline] pub fn dieptsiz55_ptr(&self) -> *const Dieptsiz55 { 
           self.dieptsiz55_mut()
    }

    #[doc="Read the DIEPTSIZ55 register."]
    #[inline] pub fn dieptsiz55(&self) -> Dieptsiz55 { 
        unsafe {
            read_volatile(self.dieptsiz55_ptr())
        }
    }

    #[doc="Write the DIEPTSIZ55 register."]
    #[inline] pub fn set_dieptsiz55<F: FnOnce(Dieptsiz55) -> Dieptsiz55>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dieptsiz55_mut(), f(Dieptsiz55(0)));
        }
        self
    }

    #[doc="Modify the DIEPTSIZ55 register."]
    #[inline] pub fn with_dieptsiz55<F: FnOnce(Dieptsiz55) -> Dieptsiz55>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dieptsiz55_mut(), f(self.dieptsiz55()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DTXFSTS55 register."]
    #[inline] pub fn dtxfsts55_mut(&self) -> *mut Dtxfsts55 { 
        (self.0 + 0x1b8) as *mut Dtxfsts55
    }

    #[doc="Get the *const pointer for the DTXFSTS55 register."]
    #[inline] pub fn dtxfsts55_ptr(&self) -> *const Dtxfsts55 { 
           self.dtxfsts55_mut()
    }

    #[doc="Read the DTXFSTS55 register."]
    #[inline] pub fn dtxfsts55(&self) -> Dtxfsts55 { 
        unsafe {
            read_volatile(self.dtxfsts55_ptr())
        }
    }

    #[doc="Write the DTXFSTS55 register."]
    #[inline] pub fn set_dtxfsts55<F: FnOnce(Dtxfsts55) -> Dtxfsts55>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dtxfsts55_mut(), f(Dtxfsts55(0)));
        }
        self
    }

    #[doc="Modify the DTXFSTS55 register."]
    #[inline] pub fn with_dtxfsts55<F: FnOnce(Dtxfsts55) -> Dtxfsts55>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dtxfsts55_mut(), f(self.dtxfsts55()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DOEPCTL4 register."]
    #[inline] pub fn doepctl4_mut(&self) -> *mut Doepctl4 { 
        (self.0 + 0x378) as *mut Doepctl4
    }

    #[doc="Get the *const pointer for the DOEPCTL4 register."]
    #[inline] pub fn doepctl4_ptr(&self) -> *const Doepctl4 { 
           self.doepctl4_mut()
    }

    #[doc="Read the DOEPCTL4 register."]
    #[inline] pub fn doepctl4(&self) -> Doepctl4 { 
        unsafe {
            read_volatile(self.doepctl4_ptr())
        }
    }

    #[doc="Write the DOEPCTL4 register."]
    #[inline] pub fn set_doepctl4<F: FnOnce(Doepctl4) -> Doepctl4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.doepctl4_mut(), f(Doepctl4(0)));
        }
        self
    }

    #[doc="Modify the DOEPCTL4 register."]
    #[inline] pub fn with_doepctl4<F: FnOnce(Doepctl4) -> Doepctl4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.doepctl4_mut(), f(self.doepctl4()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DOEPINT4 register."]
    #[inline] pub fn doepint4_mut(&self) -> *mut Doepint4 { 
        (self.0 + 0x380) as *mut Doepint4
    }

    #[doc="Get the *const pointer for the DOEPINT4 register."]
    #[inline] pub fn doepint4_ptr(&self) -> *const Doepint4 { 
           self.doepint4_mut()
    }

    #[doc="Read the DOEPINT4 register."]
    #[inline] pub fn doepint4(&self) -> Doepint4 { 
        unsafe {
            read_volatile(self.doepint4_ptr())
        }
    }

    #[doc="Write the DOEPINT4 register."]
    #[inline] pub fn set_doepint4<F: FnOnce(Doepint4) -> Doepint4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.doepint4_mut(), f(Doepint4(0)));
        }
        self
    }

    #[doc="Modify the DOEPINT4 register."]
    #[inline] pub fn with_doepint4<F: FnOnce(Doepint4) -> Doepint4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.doepint4_mut(), f(self.doepint4()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DOEPTSIZ4 register."]
    #[inline] pub fn doeptsiz4_mut(&self) -> *mut Doeptsiz4 { 
        (self.0 + 0x388) as *mut Doeptsiz4
    }

    #[doc="Get the *const pointer for the DOEPTSIZ4 register."]
    #[inline] pub fn doeptsiz4_ptr(&self) -> *const Doeptsiz4 { 
           self.doeptsiz4_mut()
    }

    #[doc="Read the DOEPTSIZ4 register."]
    #[inline] pub fn doeptsiz4(&self) -> Doeptsiz4 { 
        unsafe {
            read_volatile(self.doeptsiz4_ptr())
        }
    }

    #[doc="Write the DOEPTSIZ4 register."]
    #[inline] pub fn set_doeptsiz4<F: FnOnce(Doeptsiz4) -> Doeptsiz4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.doeptsiz4_mut(), f(Doeptsiz4(0)));
        }
        self
    }

    #[doc="Modify the DOEPTSIZ4 register."]
    #[inline] pub fn with_doeptsiz4<F: FnOnce(Doeptsiz4) -> Doeptsiz4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.doeptsiz4_mut(), f(self.doeptsiz4()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DOEPCTL5 register."]
    #[inline] pub fn doepctl5_mut(&self) -> *mut Doepctl5 { 
        (self.0 + 0x390) as *mut Doepctl5
    }

    #[doc="Get the *const pointer for the DOEPCTL5 register."]
    #[inline] pub fn doepctl5_ptr(&self) -> *const Doepctl5 { 
           self.doepctl5_mut()
    }

    #[doc="Read the DOEPCTL5 register."]
    #[inline] pub fn doepctl5(&self) -> Doepctl5 { 
        unsafe {
            read_volatile(self.doepctl5_ptr())
        }
    }

    #[doc="Write the DOEPCTL5 register."]
    #[inline] pub fn set_doepctl5<F: FnOnce(Doepctl5) -> Doepctl5>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.doepctl5_mut(), f(Doepctl5(0)));
        }
        self
    }

    #[doc="Modify the DOEPCTL5 register."]
    #[inline] pub fn with_doepctl5<F: FnOnce(Doepctl5) -> Doepctl5>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.doepctl5_mut(), f(self.doepctl5()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DOEPINT5 register."]
    #[inline] pub fn doepint5_mut(&self) -> *mut Doepint5 { 
        (self.0 + 0x398) as *mut Doepint5
    }

    #[doc="Get the *const pointer for the DOEPINT5 register."]
    #[inline] pub fn doepint5_ptr(&self) -> *const Doepint5 { 
           self.doepint5_mut()
    }

    #[doc="Read the DOEPINT5 register."]
    #[inline] pub fn doepint5(&self) -> Doepint5 { 
        unsafe {
            read_volatile(self.doepint5_ptr())
        }
    }

    #[doc="Write the DOEPINT5 register."]
    #[inline] pub fn set_doepint5<F: FnOnce(Doepint5) -> Doepint5>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.doepint5_mut(), f(Doepint5(0)));
        }
        self
    }

    #[doc="Modify the DOEPINT5 register."]
    #[inline] pub fn with_doepint5<F: FnOnce(Doepint5) -> Doepint5>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.doepint5_mut(), f(self.doepint5()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DOEPTSIZ5 register."]
    #[inline] pub fn doeptsiz5_mut(&self) -> *mut Doeptsiz5 { 
        (self.0 + 0x3a0) as *mut Doeptsiz5
    }

    #[doc="Get the *const pointer for the DOEPTSIZ5 register."]
    #[inline] pub fn doeptsiz5_ptr(&self) -> *const Doeptsiz5 { 
           self.doeptsiz5_mut()
    }

    #[doc="Read the DOEPTSIZ5 register."]
    #[inline] pub fn doeptsiz5(&self) -> Doeptsiz5 { 
        unsafe {
            read_volatile(self.doeptsiz5_ptr())
        }
    }

    #[doc="Write the DOEPTSIZ5 register."]
    #[inline] pub fn set_doeptsiz5<F: FnOnce(Doeptsiz5) -> Doeptsiz5>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.doeptsiz5_mut(), f(Doeptsiz5(0)));
        }
        self
    }

    #[doc="Modify the DOEPTSIZ5 register."]
    #[inline] pub fn with_doeptsiz5<F: FnOnce(Doeptsiz5) -> Doeptsiz5>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.doeptsiz5_mut(), f(self.doeptsiz5()));
        }
        self
    }

}

#[doc="OTG_FS device configuration register (OTG_FS_DCFG)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dcfg(pub u32);
impl Dcfg {
    #[doc="Device speed"]
    #[inline] pub fn dspd(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if DSPD != 0"]
    #[inline] pub fn test_dspd(&self) -> bool {
        self.dspd() != 0
    }

    #[doc="Sets the DSPD field."]
    #[inline] pub fn set_dspd<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Non-zero-length status OUT handshake"]
    #[inline] pub fn nzlsohsk(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if NZLSOHSK != 0"]
    #[inline] pub fn test_nzlsohsk(&self) -> bool {
        self.nzlsohsk() != 0
    }

    #[doc="Sets the NZLSOHSK field."]
    #[inline] pub fn set_nzlsohsk<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Device address"]
    #[inline] pub fn dad(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7f) as u8) } // [10:4]
    }

    #[doc="Returns true if DAD != 0"]
    #[inline] pub fn test_dad(&self) -> bool {
        self.dad() != 0
    }

    #[doc="Sets the DAD field."]
    #[inline] pub fn set_dad<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Periodic frame interval"]
    #[inline] pub fn pfivl(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x3) as u8) } // [12:11]
    }

    #[doc="Returns true if PFIVL != 0"]
    #[inline] pub fn test_pfivl(&self) -> bool {
        self.pfivl() != 0
    }

    #[doc="Sets the PFIVL field."]
    #[inline] pub fn set_pfivl<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 11);
        self.0 |= value << 11;
        self
    }

}

impl From<u32> for Dcfg {
    #[inline]
    fn from(other: u32) -> Self {
         Dcfg(other)
    }
}

impl ::core::fmt::Display for Dcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dspd() != 0 { try!(write!(f, " dspd=0x{:x}", self.dspd()))}
        if self.nzlsohsk() != 0 { try!(write!(f, " nzlsohsk"))}
        if self.dad() != 0 { try!(write!(f, " dad=0x{:x}", self.dad()))}
        if self.pfivl() != 0 { try!(write!(f, " pfivl=0x{:x}", self.pfivl()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS device control register (OTG_FS_DCTL)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dctl(pub u32);
impl Dctl {
    #[doc="Remote wakeup signaling"]
    #[inline] pub fn rwusig(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RWUSIG != 0"]
    #[inline] pub fn test_rwusig(&self) -> bool {
        self.rwusig() != 0
    }

    #[doc="Sets the RWUSIG field."]
    #[inline] pub fn set_rwusig<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Soft disconnect"]
    #[inline] pub fn sdis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SDIS != 0"]
    #[inline] pub fn test_sdis(&self) -> bool {
        self.sdis() != 0
    }

    #[doc="Sets the SDIS field."]
    #[inline] pub fn set_sdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Global IN NAK status"]
    #[inline] pub fn ginsts(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if GINSTS != 0"]
    #[inline] pub fn test_ginsts(&self) -> bool {
        self.ginsts() != 0
    }

    #[doc="Sets the GINSTS field."]
    #[inline] pub fn set_ginsts<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Global OUT NAK status"]
    #[inline] pub fn gonsts(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if GONSTS != 0"]
    #[inline] pub fn test_gonsts(&self) -> bool {
        self.gonsts() != 0
    }

    #[doc="Sets the GONSTS field."]
    #[inline] pub fn set_gonsts<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Test control"]
    #[inline] pub fn tctl(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if TCTL != 0"]
    #[inline] pub fn test_tctl(&self) -> bool {
        self.tctl() != 0
    }

    #[doc="Sets the TCTL field."]
    #[inline] pub fn set_tctl<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Set global IN NAK"]
    #[inline] pub fn sginak(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SGINAK != 0"]
    #[inline] pub fn test_sginak(&self) -> bool {
        self.sginak() != 0
    }

    #[doc="Sets the SGINAK field."]
    #[inline] pub fn set_sginak<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Clear global IN NAK"]
    #[inline] pub fn cginak(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CGINAK != 0"]
    #[inline] pub fn test_cginak(&self) -> bool {
        self.cginak() != 0
    }

    #[doc="Sets the CGINAK field."]
    #[inline] pub fn set_cginak<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Set global OUT NAK"]
    #[inline] pub fn sgonak(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if SGONAK != 0"]
    #[inline] pub fn test_sgonak(&self) -> bool {
        self.sgonak() != 0
    }

    #[doc="Sets the SGONAK field."]
    #[inline] pub fn set_sgonak<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Clear global OUT NAK"]
    #[inline] pub fn cgonak(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CGONAK != 0"]
    #[inline] pub fn test_cgonak(&self) -> bool {
        self.cgonak() != 0
    }

    #[doc="Sets the CGONAK field."]
    #[inline] pub fn set_cgonak<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Power-on programming done"]
    #[inline] pub fn poprgdne(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if POPRGDNE != 0"]
    #[inline] pub fn test_poprgdne(&self) -> bool {
        self.poprgdne() != 0
    }

    #[doc="Sets the POPRGDNE field."]
    #[inline] pub fn set_poprgdne<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

}

impl From<u32> for Dctl {
    #[inline]
    fn from(other: u32) -> Self {
         Dctl(other)
    }
}

impl ::core::fmt::Display for Dctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rwusig() != 0 { try!(write!(f, " rwusig"))}
        if self.sdis() != 0 { try!(write!(f, " sdis"))}
        if self.ginsts() != 0 { try!(write!(f, " ginsts"))}
        if self.gonsts() != 0 { try!(write!(f, " gonsts"))}
        if self.tctl() != 0 { try!(write!(f, " tctl=0x{:x}", self.tctl()))}
        if self.sginak() != 0 { try!(write!(f, " sginak"))}
        if self.cginak() != 0 { try!(write!(f, " cginak"))}
        if self.sgonak() != 0 { try!(write!(f, " sgonak"))}
        if self.cgonak() != 0 { try!(write!(f, " cgonak"))}
        if self.poprgdne() != 0 { try!(write!(f, " poprgdne"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS device status register (OTG_FS_DSTS)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dsts(pub u32);
impl Dsts {
    #[doc="Suspend status"]
    #[inline] pub fn suspsts(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SUSPSTS != 0"]
    #[inline] pub fn test_suspsts(&self) -> bool {
        self.suspsts() != 0
    }

    #[doc="Sets the SUSPSTS field."]
    #[inline] pub fn set_suspsts<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Enumerated speed"]
    #[inline] pub fn enumspd(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x3) as u8) } // [2:1]
    }

    #[doc="Returns true if ENUMSPD != 0"]
    #[inline] pub fn test_enumspd(&self) -> bool {
        self.enumspd() != 0
    }

    #[doc="Sets the ENUMSPD field."]
    #[inline] pub fn set_enumspd<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Erratic error"]
    #[inline] pub fn eerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if EERR != 0"]
    #[inline] pub fn test_eerr(&self) -> bool {
        self.eerr() != 0
    }

    #[doc="Sets the EERR field."]
    #[inline] pub fn set_eerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Frame number of the received SOF"]
    #[inline] pub fn fnsof(&self) -> bits::U14 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3fff) as u16) } // [21:8]
    }

    #[doc="Returns true if FNSOF != 0"]
    #[inline] pub fn test_fnsof(&self) -> bool {
        self.fnsof() != 0
    }

    #[doc="Sets the FNSOF field."]
    #[inline] pub fn set_fnsof<V: Into<bits::U14>>(mut self, value: V) -> Self {
        let value: bits::U14 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3fff << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Dsts {
    #[inline]
    fn from(other: u32) -> Self {
         Dsts(other)
    }
}

impl ::core::fmt::Display for Dsts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dsts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.suspsts() != 0 { try!(write!(f, " suspsts"))}
        if self.enumspd() != 0 { try!(write!(f, " enumspd=0x{:x}", self.enumspd()))}
        if self.eerr() != 0 { try!(write!(f, " eerr"))}
        if self.fnsof() != 0 { try!(write!(f, " fnsof=0x{:x}", self.fnsof()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Diepmsk(pub u32);
impl Diepmsk {
    #[doc="Transfer completed interrupt mask"]
    #[inline] pub fn xfrcm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XFRCM != 0"]
    #[inline] pub fn test_xfrcm(&self) -> bool {
        self.xfrcm() != 0
    }

    #[doc="Sets the XFRCM field."]
    #[inline] pub fn set_xfrcm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Endpoint disabled interrupt mask"]
    #[inline] pub fn epdm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if EPDM != 0"]
    #[inline] pub fn test_epdm(&self) -> bool {
        self.epdm() != 0
    }

    #[doc="Sets the EPDM field."]
    #[inline] pub fn set_epdm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Timeout condition mask (Non-isochronous endpoints)"]
    #[inline] pub fn tom(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TOM != 0"]
    #[inline] pub fn test_tom(&self) -> bool {
        self.tom() != 0
    }

    #[doc="Sets the TOM field."]
    #[inline] pub fn set_tom<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="IN token received when TxFIFO empty mask"]
    #[inline] pub fn ittxfemsk(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ITTXFEMSK != 0"]
    #[inline] pub fn test_ittxfemsk(&self) -> bool {
        self.ittxfemsk() != 0
    }

    #[doc="Sets the ITTXFEMSK field."]
    #[inline] pub fn set_ittxfemsk<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="IN token received with EP mismatch mask"]
    #[inline] pub fn inepnmm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if INEPNMM != 0"]
    #[inline] pub fn test_inepnmm(&self) -> bool {
        self.inepnmm() != 0
    }

    #[doc="Sets the INEPNMM field."]
    #[inline] pub fn set_inepnmm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="IN endpoint NAK effective mask"]
    #[inline] pub fn inepnem(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if INEPNEM != 0"]
    #[inline] pub fn test_inepnem(&self) -> bool {
        self.inepnem() != 0
    }

    #[doc="Sets the INEPNEM field."]
    #[inline] pub fn set_inepnem<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u32> for Diepmsk {
    #[inline]
    fn from(other: u32) -> Self {
         Diepmsk(other)
    }
}

impl ::core::fmt::Display for Diepmsk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Diepmsk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrcm() != 0 { try!(write!(f, " xfrcm"))}
        if self.epdm() != 0 { try!(write!(f, " epdm"))}
        if self.tom() != 0 { try!(write!(f, " tom"))}
        if self.ittxfemsk() != 0 { try!(write!(f, " ittxfemsk"))}
        if self.inepnmm() != 0 { try!(write!(f, " inepnmm"))}
        if self.inepnem() != 0 { try!(write!(f, " inepnem"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Doepmsk(pub u32);
impl Doepmsk {
    #[doc="Transfer completed interrupt mask"]
    #[inline] pub fn xfrcm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XFRCM != 0"]
    #[inline] pub fn test_xfrcm(&self) -> bool {
        self.xfrcm() != 0
    }

    #[doc="Sets the XFRCM field."]
    #[inline] pub fn set_xfrcm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Endpoint disabled interrupt mask"]
    #[inline] pub fn epdm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if EPDM != 0"]
    #[inline] pub fn test_epdm(&self) -> bool {
        self.epdm() != 0
    }

    #[doc="Sets the EPDM field."]
    #[inline] pub fn set_epdm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="SETUP phase done mask"]
    #[inline] pub fn stupm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STUPM != 0"]
    #[inline] pub fn test_stupm(&self) -> bool {
        self.stupm() != 0
    }

    #[doc="Sets the STUPM field."]
    #[inline] pub fn set_stupm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="OUT token received when endpoint disabled mask"]
    #[inline] pub fn otepdm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if OTEPDM != 0"]
    #[inline] pub fn test_otepdm(&self) -> bool {
        self.otepdm() != 0
    }

    #[doc="Sets the OTEPDM field."]
    #[inline] pub fn set_otepdm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Doepmsk {
    #[inline]
    fn from(other: u32) -> Self {
         Doepmsk(other)
    }
}

impl ::core::fmt::Display for Doepmsk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Doepmsk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrcm() != 0 { try!(write!(f, " xfrcm"))}
        if self.epdm() != 0 { try!(write!(f, " epdm"))}
        if self.stupm() != 0 { try!(write!(f, " stupm"))}
        if self.otepdm() != 0 { try!(write!(f, " otepdm"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Daint(pub u32);
impl Daint {
    #[doc="IN endpoint interrupt bits"]
    #[inline] pub fn iepint(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if IEPINT != 0"]
    #[inline] pub fn test_iepint(&self) -> bool {
        self.iepint() != 0
    }

    #[doc="Sets the IEPINT field."]
    #[inline] pub fn set_iepint<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="OUT endpoint interrupt bits"]
    #[inline] pub fn oepint(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if OEPINT != 0"]
    #[inline] pub fn test_oepint(&self) -> bool {
        self.oepint() != 0
    }

    #[doc="Sets the OEPINT field."]
    #[inline] pub fn set_oepint<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Daint {
    #[inline]
    fn from(other: u32) -> Self {
         Daint(other)
    }
}

impl ::core::fmt::Display for Daint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Daint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.iepint() != 0 { try!(write!(f, " iepint=0x{:x}", self.iepint()))}
        if self.oepint() != 0 { try!(write!(f, " oepint=0x{:x}", self.oepint()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Daintmsk(pub u32);
impl Daintmsk {
    #[doc="IN EP interrupt mask bits"]
    #[inline] pub fn iepm(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if IEPM != 0"]
    #[inline] pub fn test_iepm(&self) -> bool {
        self.iepm() != 0
    }

    #[doc="Sets the IEPM field."]
    #[inline] pub fn set_iepm<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="OUT endpoint interrupt bits"]
    #[inline] pub fn oepint(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if OEPINT != 0"]
    #[inline] pub fn test_oepint(&self) -> bool {
        self.oepint() != 0
    }

    #[doc="Sets the OEPINT field."]
    #[inline] pub fn set_oepint<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Daintmsk {
    #[inline]
    fn from(other: u32) -> Self {
         Daintmsk(other)
    }
}

impl ::core::fmt::Display for Daintmsk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Daintmsk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.iepm() != 0 { try!(write!(f, " iepm=0x{:x}", self.iepm()))}
        if self.oepint() != 0 { try!(write!(f, " oepint=0x{:x}", self.oepint()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS device VBUS discharge time register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dvbusdis(pub u32);
impl Dvbusdis {
    #[doc="Device VBUS discharge time"]
    #[inline] pub fn vbusdt(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if VBUSDT != 0"]
    #[inline] pub fn test_vbusdt(&self) -> bool {
        self.vbusdt() != 0
    }

    #[doc="Sets the VBUSDT field."]
    #[inline] pub fn set_vbusdt<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dvbusdis {
    #[inline]
    fn from(other: u32) -> Self {
         Dvbusdis(other)
    }
}

impl ::core::fmt::Display for Dvbusdis {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dvbusdis {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.vbusdt() != 0 { try!(write!(f, " vbusdt=0x{:x}", self.vbusdt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS device VBUS pulsing time register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dvbuspulse(pub u32);
impl Dvbuspulse {
    #[doc="Device VBUS pulsing time"]
    #[inline] pub fn dvbusp(&self) -> bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if DVBUSP != 0"]
    #[inline] pub fn test_dvbusp(&self) -> bool {
        self.dvbusp() != 0
    }

    #[doc="Sets the DVBUSP field."]
    #[inline] pub fn set_dvbusp<V: Into<bits::U12>>(mut self, value: V) -> Self {
        let value: bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dvbuspulse {
    #[inline]
    fn from(other: u32) -> Self {
         Dvbuspulse(other)
    }
}

impl ::core::fmt::Display for Dvbuspulse {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dvbuspulse {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dvbusp() != 0 { try!(write!(f, " dvbusp=0x{:x}", self.dvbusp()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS device IN endpoint FIFO empty interrupt mask register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Diepempmsk(pub u32);
impl Diepempmsk {
    #[doc="IN EP Tx FIFO empty interrupt mask bits"]
    #[inline] pub fn ineptxfem(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if INEPTXFEM != 0"]
    #[inline] pub fn test_ineptxfem(&self) -> bool {
        self.ineptxfem() != 0
    }

    #[doc="Sets the INEPTXFEM field."]
    #[inline] pub fn set_ineptxfem<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Diepempmsk {
    #[inline]
    fn from(other: u32) -> Self {
         Diepempmsk(other)
    }
}

impl ::core::fmt::Display for Diepempmsk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Diepempmsk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ineptxfem() != 0 { try!(write!(f, " ineptxfem=0x{:x}", self.ineptxfem()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Diepctl0(pub u32);
impl Diepctl0 {
    #[doc="Maximum packet size"]
    #[inline] pub fn mpsiz(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if MPSIZ != 0"]
    #[inline] pub fn test_mpsiz(&self) -> bool {
        self.mpsiz() != 0
    }

    #[doc="Sets the MPSIZ field."]
    #[inline] pub fn set_mpsiz<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="USB active endpoint"]
    #[inline] pub fn usbaep(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if USBAEP != 0"]
    #[inline] pub fn test_usbaep(&self) -> bool {
        self.usbaep() != 0
    }

    #[doc="Sets the USBAEP field."]
    #[inline] pub fn set_usbaep<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="NAK status"]
    #[inline] pub fn naksts(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if NAKSTS != 0"]
    #[inline] pub fn test_naksts(&self) -> bool {
        self.naksts() != 0
    }

    #[doc="Sets the NAKSTS field."]
    #[inline] pub fn set_naksts<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Endpoint type"]
    #[inline] pub fn eptyp(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x3) as u8) } // [19:18]
    }

    #[doc="Returns true if EPTYP != 0"]
    #[inline] pub fn test_eptyp(&self) -> bool {
        self.eptyp() != 0
    }

    #[doc="Sets the EPTYP field."]
    #[inline] pub fn set_eptyp<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="STALL handshake"]
    #[inline] pub fn stall(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if STALL != 0"]
    #[inline] pub fn test_stall(&self) -> bool {
        self.stall() != 0
    }

    #[doc="Sets the STALL field."]
    #[inline] pub fn set_stall<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="TxFIFO number"]
    #[inline] pub fn txfnum(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0xf) as u8) } // [25:22]
    }

    #[doc="Returns true if TXFNUM != 0"]
    #[inline] pub fn test_txfnum(&self) -> bool {
        self.txfnum() != 0
    }

    #[doc="Sets the TXFNUM field."]
    #[inline] pub fn set_txfnum<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Clear NAK"]
    #[inline] pub fn cnak(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if CNAK != 0"]
    #[inline] pub fn test_cnak(&self) -> bool {
        self.cnak() != 0
    }

    #[doc="Sets the CNAK field."]
    #[inline] pub fn set_cnak<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Set NAK"]
    #[inline] pub fn snak(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if SNAK != 0"]
    #[inline] pub fn test_snak(&self) -> bool {
        self.snak() != 0
    }

    #[doc="Sets the SNAK field."]
    #[inline] pub fn set_snak<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Endpoint disable"]
    #[inline] pub fn epdis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if EPDIS != 0"]
    #[inline] pub fn test_epdis(&self) -> bool {
        self.epdis() != 0
    }

    #[doc="Sets the EPDIS field."]
    #[inline] pub fn set_epdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Endpoint enable"]
    #[inline] pub fn epena(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if EPENA != 0"]
    #[inline] pub fn test_epena(&self) -> bool {
        self.epena() != 0
    }

    #[doc="Sets the EPENA field."]
    #[inline] pub fn set_epena<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Diepctl0 {
    #[inline]
    fn from(other: u32) -> Self {
         Diepctl0(other)
    }
}

impl ::core::fmt::Display for Diepctl0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Diepctl0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mpsiz() != 0 { try!(write!(f, " mpsiz=0x{:x}", self.mpsiz()))}
        if self.usbaep() != 0 { try!(write!(f, " usbaep"))}
        if self.naksts() != 0 { try!(write!(f, " naksts"))}
        if self.eptyp() != 0 { try!(write!(f, " eptyp=0x{:x}", self.eptyp()))}
        if self.stall() != 0 { try!(write!(f, " stall"))}
        if self.txfnum() != 0 { try!(write!(f, " txfnum=0x{:x}", self.txfnum()))}
        if self.cnak() != 0 { try!(write!(f, " cnak"))}
        if self.snak() != 0 { try!(write!(f, " snak"))}
        if self.epdis() != 0 { try!(write!(f, " epdis"))}
        if self.epena() != 0 { try!(write!(f, " epena"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG device endpoint-n control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Diepctl(pub u32);
impl Diepctl {
    #[doc="EPENA"]
    #[inline] pub fn epena(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if EPENA != 0"]
    #[inline] pub fn test_epena(&self) -> bool {
        self.epena() != 0
    }

    #[doc="Sets the EPENA field."]
    #[inline] pub fn set_epena<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="EPDIS"]
    #[inline] pub fn epdis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if EPDIS != 0"]
    #[inline] pub fn test_epdis(&self) -> bool {
        self.epdis() != 0
    }

    #[doc="Sets the EPDIS field."]
    #[inline] pub fn set_epdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="SODDFRM/SD1PID"]
    #[inline] pub fn soddfrm_sd1pid(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if SODDFRM_SD1PID != 0"]
    #[inline] pub fn test_soddfrm_sd1pid(&self) -> bool {
        self.soddfrm_sd1pid() != 0
    }

    #[doc="Sets the SODDFRM_SD1PID field."]
    #[inline] pub fn set_soddfrm_sd1pid<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="SD0PID/SEVNFRM"]
    #[inline] pub fn sd0pid_sevnfrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if SD0PID_SEVNFRM != 0"]
    #[inline] pub fn test_sd0pid_sevnfrm(&self) -> bool {
        self.sd0pid_sevnfrm() != 0
    }

    #[doc="Sets the SD0PID_SEVNFRM field."]
    #[inline] pub fn set_sd0pid_sevnfrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="SNAK"]
    #[inline] pub fn snak(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if SNAK != 0"]
    #[inline] pub fn test_snak(&self) -> bool {
        self.snak() != 0
    }

    #[doc="Sets the SNAK field."]
    #[inline] pub fn set_snak<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="CNAK"]
    #[inline] pub fn cnak(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if CNAK != 0"]
    #[inline] pub fn test_cnak(&self) -> bool {
        self.cnak() != 0
    }

    #[doc="Sets the CNAK field."]
    #[inline] pub fn set_cnak<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="TXFNUM"]
    #[inline] pub fn txfnum(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0xf) as u8) } // [25:22]
    }

    #[doc="Returns true if TXFNUM != 0"]
    #[inline] pub fn test_txfnum(&self) -> bool {
        self.txfnum() != 0
    }

    #[doc="Sets the TXFNUM field."]
    #[inline] pub fn set_txfnum<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Stall"]
    #[inline] pub fn stall(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if Stall != 0"]
    #[inline] pub fn test_stall(&self) -> bool {
        self.stall() != 0
    }

    #[doc="Sets the Stall field."]
    #[inline] pub fn set_stall<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="EPTYP"]
    #[inline] pub fn eptyp(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x3) as u8) } // [19:18]
    }

    #[doc="Returns true if EPTYP != 0"]
    #[inline] pub fn test_eptyp(&self) -> bool {
        self.eptyp() != 0
    }

    #[doc="Sets the EPTYP field."]
    #[inline] pub fn set_eptyp<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="NAKSTS"]
    #[inline] pub fn naksts(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if NAKSTS != 0"]
    #[inline] pub fn test_naksts(&self) -> bool {
        self.naksts() != 0
    }

    #[doc="Sets the NAKSTS field."]
    #[inline] pub fn set_naksts<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="EONUM/DPID"]
    #[inline] pub fn eonum_dpid(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if EONUM_DPID != 0"]
    #[inline] pub fn test_eonum_dpid(&self) -> bool {
        self.eonum_dpid() != 0
    }

    #[doc="Sets the EONUM_DPID field."]
    #[inline] pub fn set_eonum_dpid<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="USBAEP"]
    #[inline] pub fn usbaep(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if USBAEP != 0"]
    #[inline] pub fn test_usbaep(&self) -> bool {
        self.usbaep() != 0
    }

    #[doc="Sets the USBAEP field."]
    #[inline] pub fn set_usbaep<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="MPSIZ"]
    #[inline] pub fn mpsiz(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
    }

    #[doc="Returns true if MPSIZ != 0"]
    #[inline] pub fn test_mpsiz(&self) -> bool {
        self.mpsiz() != 0
    }

    #[doc="Sets the MPSIZ field."]
    #[inline] pub fn set_mpsiz<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Diepctl {
    #[inline]
    fn from(other: u32) -> Self {
         Diepctl(other)
    }
}

impl ::core::fmt::Display for Diepctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Diepctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.epena() != 0 { try!(write!(f, " epena"))}
        if self.epdis() != 0 { try!(write!(f, " epdis"))}
        if self.soddfrm_sd1pid() != 0 { try!(write!(f, " soddfrm_sd1pid"))}
        if self.sd0pid_sevnfrm() != 0 { try!(write!(f, " sd0pid_sevnfrm"))}
        if self.snak() != 0 { try!(write!(f, " snak"))}
        if self.cnak() != 0 { try!(write!(f, " cnak"))}
        if self.txfnum() != 0 { try!(write!(f, " txfnum=0x{:x}", self.txfnum()))}
        if self.stall() != 0 { try!(write!(f, " stall"))}
        if self.eptyp() != 0 { try!(write!(f, " eptyp=0x{:x}", self.eptyp()))}
        if self.naksts() != 0 { try!(write!(f, " naksts"))}
        if self.eonum_dpid() != 0 { try!(write!(f, " eonum_dpid"))}
        if self.usbaep() != 0 { try!(write!(f, " usbaep"))}
        if self.mpsiz() != 0 { try!(write!(f, " mpsiz=0x{:x}", self.mpsiz()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="device endpoint-0 control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Doepctl0(pub u32);
impl Doepctl0 {
    #[doc="EPENA"]
    #[inline] pub fn epena(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if EPENA != 0"]
    #[inline] pub fn test_epena(&self) -> bool {
        self.epena() != 0
    }

    #[doc="Sets the EPENA field."]
    #[inline] pub fn set_epena<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="EPDIS"]
    #[inline] pub fn epdis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if EPDIS != 0"]
    #[inline] pub fn test_epdis(&self) -> bool {
        self.epdis() != 0
    }

    #[doc="Sets the EPDIS field."]
    #[inline] pub fn set_epdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="SNAK"]
    #[inline] pub fn snak(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if SNAK != 0"]
    #[inline] pub fn test_snak(&self) -> bool {
        self.snak() != 0
    }

    #[doc="Sets the SNAK field."]
    #[inline] pub fn set_snak<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="CNAK"]
    #[inline] pub fn cnak(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if CNAK != 0"]
    #[inline] pub fn test_cnak(&self) -> bool {
        self.cnak() != 0
    }

    #[doc="Sets the CNAK field."]
    #[inline] pub fn set_cnak<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Stall"]
    #[inline] pub fn stall(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if Stall != 0"]
    #[inline] pub fn test_stall(&self) -> bool {
        self.stall() != 0
    }

    #[doc="Sets the Stall field."]
    #[inline] pub fn set_stall<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="SNPM"]
    #[inline] pub fn snpm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if SNPM != 0"]
    #[inline] pub fn test_snpm(&self) -> bool {
        self.snpm() != 0
    }

    #[doc="Sets the SNPM field."]
    #[inline] pub fn set_snpm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="EPTYP"]
    #[inline] pub fn eptyp(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x3) as u8) } // [19:18]
    }

    #[doc="Returns true if EPTYP != 0"]
    #[inline] pub fn test_eptyp(&self) -> bool {
        self.eptyp() != 0
    }

    #[doc="Sets the EPTYP field."]
    #[inline] pub fn set_eptyp<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="NAKSTS"]
    #[inline] pub fn naksts(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if NAKSTS != 0"]
    #[inline] pub fn test_naksts(&self) -> bool {
        self.naksts() != 0
    }

    #[doc="Sets the NAKSTS field."]
    #[inline] pub fn set_naksts<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="USBAEP"]
    #[inline] pub fn usbaep(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if USBAEP != 0"]
    #[inline] pub fn test_usbaep(&self) -> bool {
        self.usbaep() != 0
    }

    #[doc="Sets the USBAEP field."]
    #[inline] pub fn set_usbaep<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="MPSIZ"]
    #[inline] pub fn mpsiz(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if MPSIZ != 0"]
    #[inline] pub fn test_mpsiz(&self) -> bool {
        self.mpsiz() != 0
    }

    #[doc="Sets the MPSIZ field."]
    #[inline] pub fn set_mpsiz<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Doepctl0 {
    #[inline]
    fn from(other: u32) -> Self {
         Doepctl0(other)
    }
}

impl ::core::fmt::Display for Doepctl0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Doepctl0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.epena() != 0 { try!(write!(f, " epena"))}
        if self.epdis() != 0 { try!(write!(f, " epdis"))}
        if self.snak() != 0 { try!(write!(f, " snak"))}
        if self.cnak() != 0 { try!(write!(f, " cnak"))}
        if self.stall() != 0 { try!(write!(f, " stall"))}
        if self.snpm() != 0 { try!(write!(f, " snpm"))}
        if self.eptyp() != 0 { try!(write!(f, " eptyp=0x{:x}", self.eptyp()))}
        if self.naksts() != 0 { try!(write!(f, " naksts"))}
        if self.usbaep() != 0 { try!(write!(f, " usbaep"))}
        if self.mpsiz() != 0 { try!(write!(f, " mpsiz=0x{:x}", self.mpsiz()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="device endpoint-n control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Doepctl(pub u32);
impl Doepctl {
    #[doc="EPENA"]
    #[inline] pub fn epena(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if EPENA != 0"]
    #[inline] pub fn test_epena(&self) -> bool {
        self.epena() != 0
    }

    #[doc="Sets the EPENA field."]
    #[inline] pub fn set_epena<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="EPDIS"]
    #[inline] pub fn epdis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if EPDIS != 0"]
    #[inline] pub fn test_epdis(&self) -> bool {
        self.epdis() != 0
    }

    #[doc="Sets the EPDIS field."]
    #[inline] pub fn set_epdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="SODDFRM"]
    #[inline] pub fn soddfrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if SODDFRM != 0"]
    #[inline] pub fn test_soddfrm(&self) -> bool {
        self.soddfrm() != 0
    }

    #[doc="Sets the SODDFRM field."]
    #[inline] pub fn set_soddfrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="SD0PID/SEVNFRM"]
    #[inline] pub fn sd0pid_sevnfrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if SD0PID_SEVNFRM != 0"]
    #[inline] pub fn test_sd0pid_sevnfrm(&self) -> bool {
        self.sd0pid_sevnfrm() != 0
    }

    #[doc="Sets the SD0PID_SEVNFRM field."]
    #[inline] pub fn set_sd0pid_sevnfrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="SNAK"]
    #[inline] pub fn snak(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if SNAK != 0"]
    #[inline] pub fn test_snak(&self) -> bool {
        self.snak() != 0
    }

    #[doc="Sets the SNAK field."]
    #[inline] pub fn set_snak<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="CNAK"]
    #[inline] pub fn cnak(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if CNAK != 0"]
    #[inline] pub fn test_cnak(&self) -> bool {
        self.cnak() != 0
    }

    #[doc="Sets the CNAK field."]
    #[inline] pub fn set_cnak<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Stall"]
    #[inline] pub fn stall(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if Stall != 0"]
    #[inline] pub fn test_stall(&self) -> bool {
        self.stall() != 0
    }

    #[doc="Sets the Stall field."]
    #[inline] pub fn set_stall<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="SNPM"]
    #[inline] pub fn snpm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if SNPM != 0"]
    #[inline] pub fn test_snpm(&self) -> bool {
        self.snpm() != 0
    }

    #[doc="Sets the SNPM field."]
    #[inline] pub fn set_snpm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="EPTYP"]
    #[inline] pub fn eptyp(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x3) as u8) } // [19:18]
    }

    #[doc="Returns true if EPTYP != 0"]
    #[inline] pub fn test_eptyp(&self) -> bool {
        self.eptyp() != 0
    }

    #[doc="Sets the EPTYP field."]
    #[inline] pub fn set_eptyp<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="NAKSTS"]
    #[inline] pub fn naksts(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if NAKSTS != 0"]
    #[inline] pub fn test_naksts(&self) -> bool {
        self.naksts() != 0
    }

    #[doc="Sets the NAKSTS field."]
    #[inline] pub fn set_naksts<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="EONUM/DPID"]
    #[inline] pub fn eonum_dpid(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if EONUM_DPID != 0"]
    #[inline] pub fn test_eonum_dpid(&self) -> bool {
        self.eonum_dpid() != 0
    }

    #[doc="Sets the EONUM_DPID field."]
    #[inline] pub fn set_eonum_dpid<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="USBAEP"]
    #[inline] pub fn usbaep(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if USBAEP != 0"]
    #[inline] pub fn test_usbaep(&self) -> bool {
        self.usbaep() != 0
    }

    #[doc="Sets the USBAEP field."]
    #[inline] pub fn set_usbaep<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="MPSIZ"]
    #[inline] pub fn mpsiz(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
    }

    #[doc="Returns true if MPSIZ != 0"]
    #[inline] pub fn test_mpsiz(&self) -> bool {
        self.mpsiz() != 0
    }

    #[doc="Sets the MPSIZ field."]
    #[inline] pub fn set_mpsiz<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Doepctl {
    #[inline]
    fn from(other: u32) -> Self {
         Doepctl(other)
    }
}

impl ::core::fmt::Display for Doepctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Doepctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.epena() != 0 { try!(write!(f, " epena"))}
        if self.epdis() != 0 { try!(write!(f, " epdis"))}
        if self.soddfrm() != 0 { try!(write!(f, " soddfrm"))}
        if self.sd0pid_sevnfrm() != 0 { try!(write!(f, " sd0pid_sevnfrm"))}
        if self.snak() != 0 { try!(write!(f, " snak"))}
        if self.cnak() != 0 { try!(write!(f, " cnak"))}
        if self.stall() != 0 { try!(write!(f, " stall"))}
        if self.snpm() != 0 { try!(write!(f, " snpm"))}
        if self.eptyp() != 0 { try!(write!(f, " eptyp=0x{:x}", self.eptyp()))}
        if self.naksts() != 0 { try!(write!(f, " naksts"))}
        if self.eonum_dpid() != 0 { try!(write!(f, " eonum_dpid"))}
        if self.usbaep() != 0 { try!(write!(f, " usbaep"))}
        if self.mpsiz() != 0 { try!(write!(f, " mpsiz=0x{:x}", self.mpsiz()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="device endpoint-x interrupt register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Diepint(pub u32);
impl Diepint {
    #[doc="TXFE"]
    #[inline] pub fn txfe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXFE != 0"]
    #[inline] pub fn test_txfe(&self) -> bool {
        self.txfe() != 0
    }

    #[doc="Sets the TXFE field."]
    #[inline] pub fn set_txfe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="INEPNE"]
    #[inline] pub fn inepne(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if INEPNE != 0"]
    #[inline] pub fn test_inepne(&self) -> bool {
        self.inepne() != 0
    }

    #[doc="Sets the INEPNE field."]
    #[inline] pub fn set_inepne<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="ITTXFE"]
    #[inline] pub fn ittxfe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ITTXFE != 0"]
    #[inline] pub fn test_ittxfe(&self) -> bool {
        self.ittxfe() != 0
    }

    #[doc="Sets the ITTXFE field."]
    #[inline] pub fn set_ittxfe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="TOC"]
    #[inline] pub fn toc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TOC != 0"]
    #[inline] pub fn test_toc(&self) -> bool {
        self.toc() != 0
    }

    #[doc="Sets the TOC field."]
    #[inline] pub fn set_toc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="EPDISD"]
    #[inline] pub fn epdisd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if EPDISD != 0"]
    #[inline] pub fn test_epdisd(&self) -> bool {
        self.epdisd() != 0
    }

    #[doc="Sets the EPDISD field."]
    #[inline] pub fn set_epdisd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="XFRC"]
    #[inline] pub fn xfrc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XFRC != 0"]
    #[inline] pub fn test_xfrc(&self) -> bool {
        self.xfrc() != 0
    }

    #[doc="Sets the XFRC field."]
    #[inline] pub fn set_xfrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Diepint {
    #[inline]
    fn from(other: u32) -> Self {
         Diepint(other)
    }
}

impl ::core::fmt::Display for Diepint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Diepint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txfe() != 0 { try!(write!(f, " txfe"))}
        if self.inepne() != 0 { try!(write!(f, " inepne"))}
        if self.ittxfe() != 0 { try!(write!(f, " ittxfe"))}
        if self.toc() != 0 { try!(write!(f, " toc"))}
        if self.epdisd() != 0 { try!(write!(f, " epdisd"))}
        if self.xfrc() != 0 { try!(write!(f, " xfrc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="device endpoint-n interrupt register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Doepint(pub u32);
impl Doepint {
    #[doc="B2BSTUP"]
    #[inline] pub fn b2bstup(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if B2BSTUP != 0"]
    #[inline] pub fn test_b2bstup(&self) -> bool {
        self.b2bstup() != 0
    }

    #[doc="Sets the B2BSTUP field."]
    #[inline] pub fn set_b2bstup<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="OTEPDIS"]
    #[inline] pub fn otepdis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if OTEPDIS != 0"]
    #[inline] pub fn test_otepdis(&self) -> bool {
        self.otepdis() != 0
    }

    #[doc="Sets the OTEPDIS field."]
    #[inline] pub fn set_otepdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="STUP"]
    #[inline] pub fn stup(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STUP != 0"]
    #[inline] pub fn test_stup(&self) -> bool {
        self.stup() != 0
    }

    #[doc="Sets the STUP field."]
    #[inline] pub fn set_stup<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="EPDISD"]
    #[inline] pub fn epdisd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if EPDISD != 0"]
    #[inline] pub fn test_epdisd(&self) -> bool {
        self.epdisd() != 0
    }

    #[doc="Sets the EPDISD field."]
    #[inline] pub fn set_epdisd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="XFRC"]
    #[inline] pub fn xfrc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XFRC != 0"]
    #[inline] pub fn test_xfrc(&self) -> bool {
        self.xfrc() != 0
    }

    #[doc="Sets the XFRC field."]
    #[inline] pub fn set_xfrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Doepint {
    #[inline]
    fn from(other: u32) -> Self {
         Doepint(other)
    }
}

impl ::core::fmt::Display for Doepint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Doepint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.b2bstup() != 0 { try!(write!(f, " b2bstup"))}
        if self.otepdis() != 0 { try!(write!(f, " otepdis"))}
        if self.stup() != 0 { try!(write!(f, " stup"))}
        if self.epdisd() != 0 { try!(write!(f, " epdisd"))}
        if self.xfrc() != 0 { try!(write!(f, " xfrc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="device endpoint-0 transfer size register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dieptsiz0(pub u32);
impl Dieptsiz0 {
    #[doc="Packet count"]
    #[inline] pub fn pktcnt(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x3) as u8) } // [20:19]
    }

    #[doc="Returns true if PKTCNT != 0"]
    #[inline] pub fn test_pktcnt(&self) -> bool {
        self.pktcnt() != 0
    }

    #[doc="Sets the PKTCNT field."]
    #[inline] pub fn set_pktcnt<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Transfer size"]
    #[inline] pub fn xfrsiz(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="Returns true if XFRSIZ != 0"]
    #[inline] pub fn test_xfrsiz(&self) -> bool {
        self.xfrsiz() != 0
    }

    #[doc="Sets the XFRSIZ field."]
    #[inline] pub fn set_xfrsiz<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dieptsiz0 {
    #[inline]
    fn from(other: u32) -> Self {
         Dieptsiz0(other)
    }
}

impl ::core::fmt::Display for Dieptsiz0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dieptsiz0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pktcnt() != 0 { try!(write!(f, " pktcnt=0x{:x}", self.pktcnt()))}
        if self.xfrsiz() != 0 { try!(write!(f, " xfrsiz=0x{:x}", self.xfrsiz()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="device OUT endpoint-0 transfer size register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Doeptsiz0(pub u32);
impl Doeptsiz0 {
    #[doc="SETUP packet count"]
    #[inline] pub fn stupcnt(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x3) as u8) } // [30:29]
    }

    #[doc="Returns true if STUPCNT != 0"]
    #[inline] pub fn test_stupcnt(&self) -> bool {
        self.stupcnt() != 0
    }

    #[doc="Sets the STUPCNT field."]
    #[inline] pub fn set_stupcnt<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Packet count"]
    #[inline] pub fn pktcnt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if PKTCNT != 0"]
    #[inline] pub fn test_pktcnt(&self) -> bool {
        self.pktcnt() != 0
    }

    #[doc="Sets the PKTCNT field."]
    #[inline] pub fn set_pktcnt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Transfer size"]
    #[inline] pub fn xfrsiz(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="Returns true if XFRSIZ != 0"]
    #[inline] pub fn test_xfrsiz(&self) -> bool {
        self.xfrsiz() != 0
    }

    #[doc="Sets the XFRSIZ field."]
    #[inline] pub fn set_xfrsiz<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Doeptsiz0 {
    #[inline]
    fn from(other: u32) -> Self {
         Doeptsiz0(other)
    }
}

impl ::core::fmt::Display for Doeptsiz0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Doeptsiz0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.stupcnt() != 0 { try!(write!(f, " stupcnt=0x{:x}", self.stupcnt()))}
        if self.pktcnt() != 0 { try!(write!(f, " pktcnt"))}
        if self.xfrsiz() != 0 { try!(write!(f, " xfrsiz=0x{:x}", self.xfrsiz()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="device endpoint-1 transfer size register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dieptsiz1(pub u32);
impl Dieptsiz1 {
    #[doc="Multi count"]
    #[inline] pub fn mcnt(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x3) as u8) } // [30:29]
    }

    #[doc="Returns true if MCNT != 0"]
    #[inline] pub fn test_mcnt(&self) -> bool {
        self.mcnt() != 0
    }

    #[doc="Sets the MCNT field."]
    #[inline] pub fn set_mcnt<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Packet count"]
    #[inline] pub fn pktcnt(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x3ff) as u16) } // [28:19]
    }

    #[doc="Returns true if PKTCNT != 0"]
    #[inline] pub fn test_pktcnt(&self) -> bool {
        self.pktcnt() != 0
    }

    #[doc="Sets the PKTCNT field."]
    #[inline] pub fn set_pktcnt<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Transfer size"]
    #[inline] pub fn xfrsiz(&self) -> bits::U19 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ffff) as u32) } // [18:0]
    }

    #[doc="Returns true if XFRSIZ != 0"]
    #[inline] pub fn test_xfrsiz(&self) -> bool {
        self.xfrsiz() != 0
    }

    #[doc="Sets the XFRSIZ field."]
    #[inline] pub fn set_xfrsiz<V: Into<bits::U19>>(mut self, value: V) -> Self {
        let value: bits::U19 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dieptsiz1 {
    #[inline]
    fn from(other: u32) -> Self {
         Dieptsiz1(other)
    }
}

impl ::core::fmt::Display for Dieptsiz1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dieptsiz1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mcnt() != 0 { try!(write!(f, " mcnt=0x{:x}", self.mcnt()))}
        if self.pktcnt() != 0 { try!(write!(f, " pktcnt=0x{:x}", self.pktcnt()))}
        if self.xfrsiz() != 0 { try!(write!(f, " xfrsiz=0x{:x}", self.xfrsiz()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="device endpoint-2 transfer size register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dieptsiz2(pub u32);
impl Dieptsiz2 {
    #[doc="Multi count"]
    #[inline] pub fn mcnt(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x3) as u8) } // [30:29]
    }

    #[doc="Returns true if MCNT != 0"]
    #[inline] pub fn test_mcnt(&self) -> bool {
        self.mcnt() != 0
    }

    #[doc="Sets the MCNT field."]
    #[inline] pub fn set_mcnt<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Packet count"]
    #[inline] pub fn pktcnt(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x3ff) as u16) } // [28:19]
    }

    #[doc="Returns true if PKTCNT != 0"]
    #[inline] pub fn test_pktcnt(&self) -> bool {
        self.pktcnt() != 0
    }

    #[doc="Sets the PKTCNT field."]
    #[inline] pub fn set_pktcnt<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Transfer size"]
    #[inline] pub fn xfrsiz(&self) -> bits::U19 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ffff) as u32) } // [18:0]
    }

    #[doc="Returns true if XFRSIZ != 0"]
    #[inline] pub fn test_xfrsiz(&self) -> bool {
        self.xfrsiz() != 0
    }

    #[doc="Sets the XFRSIZ field."]
    #[inline] pub fn set_xfrsiz<V: Into<bits::U19>>(mut self, value: V) -> Self {
        let value: bits::U19 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dieptsiz2 {
    #[inline]
    fn from(other: u32) -> Self {
         Dieptsiz2(other)
    }
}

impl ::core::fmt::Display for Dieptsiz2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dieptsiz2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mcnt() != 0 { try!(write!(f, " mcnt=0x{:x}", self.mcnt()))}
        if self.pktcnt() != 0 { try!(write!(f, " pktcnt=0x{:x}", self.pktcnt()))}
        if self.xfrsiz() != 0 { try!(write!(f, " xfrsiz=0x{:x}", self.xfrsiz()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="device endpoint-3 transfer size register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dieptsiz3(pub u32);
impl Dieptsiz3 {
    #[doc="Multi count"]
    #[inline] pub fn mcnt(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x3) as u8) } // [30:29]
    }

    #[doc="Returns true if MCNT != 0"]
    #[inline] pub fn test_mcnt(&self) -> bool {
        self.mcnt() != 0
    }

    #[doc="Sets the MCNT field."]
    #[inline] pub fn set_mcnt<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Packet count"]
    #[inline] pub fn pktcnt(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x3ff) as u16) } // [28:19]
    }

    #[doc="Returns true if PKTCNT != 0"]
    #[inline] pub fn test_pktcnt(&self) -> bool {
        self.pktcnt() != 0
    }

    #[doc="Sets the PKTCNT field."]
    #[inline] pub fn set_pktcnt<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Transfer size"]
    #[inline] pub fn xfrsiz(&self) -> bits::U19 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ffff) as u32) } // [18:0]
    }

    #[doc="Returns true if XFRSIZ != 0"]
    #[inline] pub fn test_xfrsiz(&self) -> bool {
        self.xfrsiz() != 0
    }

    #[doc="Sets the XFRSIZ field."]
    #[inline] pub fn set_xfrsiz<V: Into<bits::U19>>(mut self, value: V) -> Self {
        let value: bits::U19 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dieptsiz3 {
    #[inline]
    fn from(other: u32) -> Self {
         Dieptsiz3(other)
    }
}

impl ::core::fmt::Display for Dieptsiz3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dieptsiz3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mcnt() != 0 { try!(write!(f, " mcnt=0x{:x}", self.mcnt()))}
        if self.pktcnt() != 0 { try!(write!(f, " pktcnt=0x{:x}", self.pktcnt()))}
        if self.xfrsiz() != 0 { try!(write!(f, " xfrsiz=0x{:x}", self.xfrsiz()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS device IN endpoint transmit FIFO status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dtxfsts0(pub u32);
impl Dtxfsts0 {
    #[doc="IN endpoint TxFIFO space available"]
    #[inline] pub fn ineptfsav(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if INEPTFSAV != 0"]
    #[inline] pub fn test_ineptfsav(&self) -> bool {
        self.ineptfsav() != 0
    }

    #[doc="Sets the INEPTFSAV field."]
    #[inline] pub fn set_ineptfsav<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dtxfsts0 {
    #[inline]
    fn from(other: u32) -> Self {
         Dtxfsts0(other)
    }
}

impl ::core::fmt::Display for Dtxfsts0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dtxfsts0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ineptfsav() != 0 { try!(write!(f, " ineptfsav=0x{:x}", self.ineptfsav()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS device IN endpoint transmit FIFO status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dtxfsts1(pub u32);
impl Dtxfsts1 {
    #[doc="IN endpoint TxFIFO space available"]
    #[inline] pub fn ineptfsav(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if INEPTFSAV != 0"]
    #[inline] pub fn test_ineptfsav(&self) -> bool {
        self.ineptfsav() != 0
    }

    #[doc="Sets the INEPTFSAV field."]
    #[inline] pub fn set_ineptfsav<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dtxfsts1 {
    #[inline]
    fn from(other: u32) -> Self {
         Dtxfsts1(other)
    }
}

impl ::core::fmt::Display for Dtxfsts1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dtxfsts1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ineptfsav() != 0 { try!(write!(f, " ineptfsav=0x{:x}", self.ineptfsav()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS device IN endpoint transmit FIFO status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dtxfsts2(pub u32);
impl Dtxfsts2 {
    #[doc="IN endpoint TxFIFO space available"]
    #[inline] pub fn ineptfsav(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if INEPTFSAV != 0"]
    #[inline] pub fn test_ineptfsav(&self) -> bool {
        self.ineptfsav() != 0
    }

    #[doc="Sets the INEPTFSAV field."]
    #[inline] pub fn set_ineptfsav<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dtxfsts2 {
    #[inline]
    fn from(other: u32) -> Self {
         Dtxfsts2(other)
    }
}

impl ::core::fmt::Display for Dtxfsts2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dtxfsts2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ineptfsav() != 0 { try!(write!(f, " ineptfsav=0x{:x}", self.ineptfsav()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS device IN endpoint transmit FIFO status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dtxfsts3(pub u32);
impl Dtxfsts3 {
    #[doc="IN endpoint TxFIFO space available"]
    #[inline] pub fn ineptfsav(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if INEPTFSAV != 0"]
    #[inline] pub fn test_ineptfsav(&self) -> bool {
        self.ineptfsav() != 0
    }

    #[doc="Sets the INEPTFSAV field."]
    #[inline] pub fn set_ineptfsav<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dtxfsts3 {
    #[inline]
    fn from(other: u32) -> Self {
         Dtxfsts3(other)
    }
}

impl ::core::fmt::Display for Dtxfsts3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dtxfsts3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ineptfsav() != 0 { try!(write!(f, " ineptfsav=0x{:x}", self.ineptfsav()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="device OUT endpoint-1 transfer size register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Doeptsiz1(pub u32);
impl Doeptsiz1 {
    #[doc="Received data PID/SETUP packet count"]
    #[inline] pub fn rxdpid_stupcnt(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x3) as u8) } // [30:29]
    }

    #[doc="Returns true if RXDPID_STUPCNT != 0"]
    #[inline] pub fn test_rxdpid_stupcnt(&self) -> bool {
        self.rxdpid_stupcnt() != 0
    }

    #[doc="Sets the RXDPID_STUPCNT field."]
    #[inline] pub fn set_rxdpid_stupcnt<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Packet count"]
    #[inline] pub fn pktcnt(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x3ff) as u16) } // [28:19]
    }

    #[doc="Returns true if PKTCNT != 0"]
    #[inline] pub fn test_pktcnt(&self) -> bool {
        self.pktcnt() != 0
    }

    #[doc="Sets the PKTCNT field."]
    #[inline] pub fn set_pktcnt<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Transfer size"]
    #[inline] pub fn xfrsiz(&self) -> bits::U19 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ffff) as u32) } // [18:0]
    }

    #[doc="Returns true if XFRSIZ != 0"]
    #[inline] pub fn test_xfrsiz(&self) -> bool {
        self.xfrsiz() != 0
    }

    #[doc="Sets the XFRSIZ field."]
    #[inline] pub fn set_xfrsiz<V: Into<bits::U19>>(mut self, value: V) -> Self {
        let value: bits::U19 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Doeptsiz1 {
    #[inline]
    fn from(other: u32) -> Self {
         Doeptsiz1(other)
    }
}

impl ::core::fmt::Display for Doeptsiz1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Doeptsiz1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rxdpid_stupcnt() != 0 { try!(write!(f, " rxdpid_stupcnt=0x{:x}", self.rxdpid_stupcnt()))}
        if self.pktcnt() != 0 { try!(write!(f, " pktcnt=0x{:x}", self.pktcnt()))}
        if self.xfrsiz() != 0 { try!(write!(f, " xfrsiz=0x{:x}", self.xfrsiz()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="device OUT endpoint-2 transfer size register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Doeptsiz2(pub u32);
impl Doeptsiz2 {
    #[doc="Received data PID/SETUP packet count"]
    #[inline] pub fn rxdpid_stupcnt(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x3) as u8) } // [30:29]
    }

    #[doc="Returns true if RXDPID_STUPCNT != 0"]
    #[inline] pub fn test_rxdpid_stupcnt(&self) -> bool {
        self.rxdpid_stupcnt() != 0
    }

    #[doc="Sets the RXDPID_STUPCNT field."]
    #[inline] pub fn set_rxdpid_stupcnt<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Packet count"]
    #[inline] pub fn pktcnt(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x3ff) as u16) } // [28:19]
    }

    #[doc="Returns true if PKTCNT != 0"]
    #[inline] pub fn test_pktcnt(&self) -> bool {
        self.pktcnt() != 0
    }

    #[doc="Sets the PKTCNT field."]
    #[inline] pub fn set_pktcnt<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Transfer size"]
    #[inline] pub fn xfrsiz(&self) -> bits::U19 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ffff) as u32) } // [18:0]
    }

    #[doc="Returns true if XFRSIZ != 0"]
    #[inline] pub fn test_xfrsiz(&self) -> bool {
        self.xfrsiz() != 0
    }

    #[doc="Sets the XFRSIZ field."]
    #[inline] pub fn set_xfrsiz<V: Into<bits::U19>>(mut self, value: V) -> Self {
        let value: bits::U19 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Doeptsiz2 {
    #[inline]
    fn from(other: u32) -> Self {
         Doeptsiz2(other)
    }
}

impl ::core::fmt::Display for Doeptsiz2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Doeptsiz2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rxdpid_stupcnt() != 0 { try!(write!(f, " rxdpid_stupcnt=0x{:x}", self.rxdpid_stupcnt()))}
        if self.pktcnt() != 0 { try!(write!(f, " pktcnt=0x{:x}", self.pktcnt()))}
        if self.xfrsiz() != 0 { try!(write!(f, " xfrsiz=0x{:x}", self.xfrsiz()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="device OUT endpoint-3 transfer size register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Doeptsiz3(pub u32);
impl Doeptsiz3 {
    #[doc="Received data PID/SETUP packet count"]
    #[inline] pub fn rxdpid_stupcnt(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x3) as u8) } // [30:29]
    }

    #[doc="Returns true if RXDPID_STUPCNT != 0"]
    #[inline] pub fn test_rxdpid_stupcnt(&self) -> bool {
        self.rxdpid_stupcnt() != 0
    }

    #[doc="Sets the RXDPID_STUPCNT field."]
    #[inline] pub fn set_rxdpid_stupcnt<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Packet count"]
    #[inline] pub fn pktcnt(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x3ff) as u16) } // [28:19]
    }

    #[doc="Returns true if PKTCNT != 0"]
    #[inline] pub fn test_pktcnt(&self) -> bool {
        self.pktcnt() != 0
    }

    #[doc="Sets the PKTCNT field."]
    #[inline] pub fn set_pktcnt<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Transfer size"]
    #[inline] pub fn xfrsiz(&self) -> bits::U19 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ffff) as u32) } // [18:0]
    }

    #[doc="Returns true if XFRSIZ != 0"]
    #[inline] pub fn test_xfrsiz(&self) -> bool {
        self.xfrsiz() != 0
    }

    #[doc="Sets the XFRSIZ field."]
    #[inline] pub fn set_xfrsiz<V: Into<bits::U19>>(mut self, value: V) -> Self {
        let value: bits::U19 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Doeptsiz3 {
    #[inline]
    fn from(other: u32) -> Self {
         Doeptsiz3(other)
    }
}

impl ::core::fmt::Display for Doeptsiz3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Doeptsiz3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rxdpid_stupcnt() != 0 { try!(write!(f, " rxdpid_stupcnt=0x{:x}", self.rxdpid_stupcnt()))}
        if self.pktcnt() != 0 { try!(write!(f, " pktcnt=0x{:x}", self.pktcnt()))}
        if self.xfrsiz() != 0 { try!(write!(f, " xfrsiz=0x{:x}", self.xfrsiz()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG device endpoint-4 control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Diepctl4(pub u32);
impl Diepctl4 {
    #[doc="EPENA"]
    #[inline] pub fn epena(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if EPENA != 0"]
    #[inline] pub fn test_epena(&self) -> bool {
        self.epena() != 0
    }

    #[doc="Sets the EPENA field."]
    #[inline] pub fn set_epena<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="EPDIS"]
    #[inline] pub fn epdis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if EPDIS != 0"]
    #[inline] pub fn test_epdis(&self) -> bool {
        self.epdis() != 0
    }

    #[doc="Sets the EPDIS field."]
    #[inline] pub fn set_epdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="SODDFRM"]
    #[inline] pub fn soddfrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if SODDFRM != 0"]
    #[inline] pub fn test_soddfrm(&self) -> bool {
        self.soddfrm() != 0
    }

    #[doc="Sets the SODDFRM field."]
    #[inline] pub fn set_soddfrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="SD0PID/SEVNFRM"]
    #[inline] pub fn sd0pid_sevnfrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if SD0PID_SEVNFRM != 0"]
    #[inline] pub fn test_sd0pid_sevnfrm(&self) -> bool {
        self.sd0pid_sevnfrm() != 0
    }

    #[doc="Sets the SD0PID_SEVNFRM field."]
    #[inline] pub fn set_sd0pid_sevnfrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="SNAK"]
    #[inline] pub fn snak(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if SNAK != 0"]
    #[inline] pub fn test_snak(&self) -> bool {
        self.snak() != 0
    }

    #[doc="Sets the SNAK field."]
    #[inline] pub fn set_snak<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="CNAK"]
    #[inline] pub fn cnak(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if CNAK != 0"]
    #[inline] pub fn test_cnak(&self) -> bool {
        self.cnak() != 0
    }

    #[doc="Sets the CNAK field."]
    #[inline] pub fn set_cnak<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="TXFNUM"]
    #[inline] pub fn txfnum(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0xf) as u8) } // [25:22]
    }

    #[doc="Returns true if TXFNUM != 0"]
    #[inline] pub fn test_txfnum(&self) -> bool {
        self.txfnum() != 0
    }

    #[doc="Sets the TXFNUM field."]
    #[inline] pub fn set_txfnum<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Stall"]
    #[inline] pub fn stall(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if Stall != 0"]
    #[inline] pub fn test_stall(&self) -> bool {
        self.stall() != 0
    }

    #[doc="Sets the Stall field."]
    #[inline] pub fn set_stall<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="EPTYP"]
    #[inline] pub fn eptyp(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x3) as u8) } // [19:18]
    }

    #[doc="Returns true if EPTYP != 0"]
    #[inline] pub fn test_eptyp(&self) -> bool {
        self.eptyp() != 0
    }

    #[doc="Sets the EPTYP field."]
    #[inline] pub fn set_eptyp<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="NAKSTS"]
    #[inline] pub fn naksts(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if NAKSTS != 0"]
    #[inline] pub fn test_naksts(&self) -> bool {
        self.naksts() != 0
    }

    #[doc="Sets the NAKSTS field."]
    #[inline] pub fn set_naksts<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="EONUM/DPID"]
    #[inline] pub fn eonum_dpid(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if EONUM_DPID != 0"]
    #[inline] pub fn test_eonum_dpid(&self) -> bool {
        self.eonum_dpid() != 0
    }

    #[doc="Sets the EONUM_DPID field."]
    #[inline] pub fn set_eonum_dpid<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="USBAEP"]
    #[inline] pub fn usbaep(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if USBAEP != 0"]
    #[inline] pub fn test_usbaep(&self) -> bool {
        self.usbaep() != 0
    }

    #[doc="Sets the USBAEP field."]
    #[inline] pub fn set_usbaep<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="MPSIZ"]
    #[inline] pub fn mpsiz(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
    }

    #[doc="Returns true if MPSIZ != 0"]
    #[inline] pub fn test_mpsiz(&self) -> bool {
        self.mpsiz() != 0
    }

    #[doc="Sets the MPSIZ field."]
    #[inline] pub fn set_mpsiz<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Diepctl4 {
    #[inline]
    fn from(other: u32) -> Self {
         Diepctl4(other)
    }
}

impl ::core::fmt::Display for Diepctl4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Diepctl4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.epena() != 0 { try!(write!(f, " epena"))}
        if self.epdis() != 0 { try!(write!(f, " epdis"))}
        if self.soddfrm() != 0 { try!(write!(f, " soddfrm"))}
        if self.sd0pid_sevnfrm() != 0 { try!(write!(f, " sd0pid_sevnfrm"))}
        if self.snak() != 0 { try!(write!(f, " snak"))}
        if self.cnak() != 0 { try!(write!(f, " cnak"))}
        if self.txfnum() != 0 { try!(write!(f, " txfnum=0x{:x}", self.txfnum()))}
        if self.stall() != 0 { try!(write!(f, " stall"))}
        if self.eptyp() != 0 { try!(write!(f, " eptyp=0x{:x}", self.eptyp()))}
        if self.naksts() != 0 { try!(write!(f, " naksts"))}
        if self.eonum_dpid() != 0 { try!(write!(f, " eonum_dpid"))}
        if self.usbaep() != 0 { try!(write!(f, " usbaep"))}
        if self.mpsiz() != 0 { try!(write!(f, " mpsiz=0x{:x}", self.mpsiz()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="device endpoint-4 interrupt register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Diepint4(pub u32);
impl Diepint4 {
    #[doc="TXFE"]
    #[inline] pub fn txfe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXFE != 0"]
    #[inline] pub fn test_txfe(&self) -> bool {
        self.txfe() != 0
    }

    #[doc="Sets the TXFE field."]
    #[inline] pub fn set_txfe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="INEPNE"]
    #[inline] pub fn inepne(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if INEPNE != 0"]
    #[inline] pub fn test_inepne(&self) -> bool {
        self.inepne() != 0
    }

    #[doc="Sets the INEPNE field."]
    #[inline] pub fn set_inepne<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="ITTXFE"]
    #[inline] pub fn ittxfe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ITTXFE != 0"]
    #[inline] pub fn test_ittxfe(&self) -> bool {
        self.ittxfe() != 0
    }

    #[doc="Sets the ITTXFE field."]
    #[inline] pub fn set_ittxfe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="TOC"]
    #[inline] pub fn toc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TOC != 0"]
    #[inline] pub fn test_toc(&self) -> bool {
        self.toc() != 0
    }

    #[doc="Sets the TOC field."]
    #[inline] pub fn set_toc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="EPDISD"]
    #[inline] pub fn epdisd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if EPDISD != 0"]
    #[inline] pub fn test_epdisd(&self) -> bool {
        self.epdisd() != 0
    }

    #[doc="Sets the EPDISD field."]
    #[inline] pub fn set_epdisd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="XFRC"]
    #[inline] pub fn xfrc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XFRC != 0"]
    #[inline] pub fn test_xfrc(&self) -> bool {
        self.xfrc() != 0
    }

    #[doc="Sets the XFRC field."]
    #[inline] pub fn set_xfrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Diepint4 {
    #[inline]
    fn from(other: u32) -> Self {
         Diepint4(other)
    }
}

impl ::core::fmt::Display for Diepint4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Diepint4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txfe() != 0 { try!(write!(f, " txfe"))}
        if self.inepne() != 0 { try!(write!(f, " inepne"))}
        if self.ittxfe() != 0 { try!(write!(f, " ittxfe"))}
        if self.toc() != 0 { try!(write!(f, " toc"))}
        if self.epdisd() != 0 { try!(write!(f, " epdisd"))}
        if self.xfrc() != 0 { try!(write!(f, " xfrc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="device endpoint-4 transfer size register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dieptsiz4(pub u32);
impl Dieptsiz4 {
    #[doc="Multi count"]
    #[inline] pub fn mcnt(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x3) as u8) } // [30:29]
    }

    #[doc="Returns true if MCNT != 0"]
    #[inline] pub fn test_mcnt(&self) -> bool {
        self.mcnt() != 0
    }

    #[doc="Sets the MCNT field."]
    #[inline] pub fn set_mcnt<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Packet count"]
    #[inline] pub fn pktcnt(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x3ff) as u16) } // [28:19]
    }

    #[doc="Returns true if PKTCNT != 0"]
    #[inline] pub fn test_pktcnt(&self) -> bool {
        self.pktcnt() != 0
    }

    #[doc="Sets the PKTCNT field."]
    #[inline] pub fn set_pktcnt<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Transfer size"]
    #[inline] pub fn xfrsiz(&self) -> bits::U19 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ffff) as u32) } // [18:0]
    }

    #[doc="Returns true if XFRSIZ != 0"]
    #[inline] pub fn test_xfrsiz(&self) -> bool {
        self.xfrsiz() != 0
    }

    #[doc="Sets the XFRSIZ field."]
    #[inline] pub fn set_xfrsiz<V: Into<bits::U19>>(mut self, value: V) -> Self {
        let value: bits::U19 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dieptsiz4 {
    #[inline]
    fn from(other: u32) -> Self {
         Dieptsiz4(other)
    }
}

impl ::core::fmt::Display for Dieptsiz4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dieptsiz4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mcnt() != 0 { try!(write!(f, " mcnt=0x{:x}", self.mcnt()))}
        if self.pktcnt() != 0 { try!(write!(f, " pktcnt=0x{:x}", self.pktcnt()))}
        if self.xfrsiz() != 0 { try!(write!(f, " xfrsiz=0x{:x}", self.xfrsiz()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS device IN endpoint transmit FIFO status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dtxfsts4(pub u32);
impl Dtxfsts4 {
    #[doc="IN endpoint TxFIFO space available"]
    #[inline] pub fn ineptfsav(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if INEPTFSAV != 0"]
    #[inline] pub fn test_ineptfsav(&self) -> bool {
        self.ineptfsav() != 0
    }

    #[doc="Sets the INEPTFSAV field."]
    #[inline] pub fn set_ineptfsav<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dtxfsts4 {
    #[inline]
    fn from(other: u32) -> Self {
         Dtxfsts4(other)
    }
}

impl ::core::fmt::Display for Dtxfsts4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dtxfsts4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ineptfsav() != 0 { try!(write!(f, " ineptfsav=0x{:x}", self.ineptfsav()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG device endpoint-5 control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Diepctl5(pub u32);
impl Diepctl5 {
    #[doc="EPENA"]
    #[inline] pub fn epena(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if EPENA != 0"]
    #[inline] pub fn test_epena(&self) -> bool {
        self.epena() != 0
    }

    #[doc="Sets the EPENA field."]
    #[inline] pub fn set_epena<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="EPDIS"]
    #[inline] pub fn epdis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if EPDIS != 0"]
    #[inline] pub fn test_epdis(&self) -> bool {
        self.epdis() != 0
    }

    #[doc="Sets the EPDIS field."]
    #[inline] pub fn set_epdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="SODDFRM"]
    #[inline] pub fn soddfrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if SODDFRM != 0"]
    #[inline] pub fn test_soddfrm(&self) -> bool {
        self.soddfrm() != 0
    }

    #[doc="Sets the SODDFRM field."]
    #[inline] pub fn set_soddfrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="SD0PID/SEVNFRM"]
    #[inline] pub fn sd0pid_sevnfrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if SD0PID_SEVNFRM != 0"]
    #[inline] pub fn test_sd0pid_sevnfrm(&self) -> bool {
        self.sd0pid_sevnfrm() != 0
    }

    #[doc="Sets the SD0PID_SEVNFRM field."]
    #[inline] pub fn set_sd0pid_sevnfrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="SNAK"]
    #[inline] pub fn snak(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if SNAK != 0"]
    #[inline] pub fn test_snak(&self) -> bool {
        self.snak() != 0
    }

    #[doc="Sets the SNAK field."]
    #[inline] pub fn set_snak<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="CNAK"]
    #[inline] pub fn cnak(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if CNAK != 0"]
    #[inline] pub fn test_cnak(&self) -> bool {
        self.cnak() != 0
    }

    #[doc="Sets the CNAK field."]
    #[inline] pub fn set_cnak<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="TXFNUM"]
    #[inline] pub fn txfnum(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0xf) as u8) } // [25:22]
    }

    #[doc="Returns true if TXFNUM != 0"]
    #[inline] pub fn test_txfnum(&self) -> bool {
        self.txfnum() != 0
    }

    #[doc="Sets the TXFNUM field."]
    #[inline] pub fn set_txfnum<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Stall"]
    #[inline] pub fn stall(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if Stall != 0"]
    #[inline] pub fn test_stall(&self) -> bool {
        self.stall() != 0
    }

    #[doc="Sets the Stall field."]
    #[inline] pub fn set_stall<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="EPTYP"]
    #[inline] pub fn eptyp(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x3) as u8) } // [19:18]
    }

    #[doc="Returns true if EPTYP != 0"]
    #[inline] pub fn test_eptyp(&self) -> bool {
        self.eptyp() != 0
    }

    #[doc="Sets the EPTYP field."]
    #[inline] pub fn set_eptyp<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="NAKSTS"]
    #[inline] pub fn naksts(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if NAKSTS != 0"]
    #[inline] pub fn test_naksts(&self) -> bool {
        self.naksts() != 0
    }

    #[doc="Sets the NAKSTS field."]
    #[inline] pub fn set_naksts<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="EONUM/DPID"]
    #[inline] pub fn eonum_dpid(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if EONUM_DPID != 0"]
    #[inline] pub fn test_eonum_dpid(&self) -> bool {
        self.eonum_dpid() != 0
    }

    #[doc="Sets the EONUM_DPID field."]
    #[inline] pub fn set_eonum_dpid<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="USBAEP"]
    #[inline] pub fn usbaep(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if USBAEP != 0"]
    #[inline] pub fn test_usbaep(&self) -> bool {
        self.usbaep() != 0
    }

    #[doc="Sets the USBAEP field."]
    #[inline] pub fn set_usbaep<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="MPSIZ"]
    #[inline] pub fn mpsiz(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
    }

    #[doc="Returns true if MPSIZ != 0"]
    #[inline] pub fn test_mpsiz(&self) -> bool {
        self.mpsiz() != 0
    }

    #[doc="Sets the MPSIZ field."]
    #[inline] pub fn set_mpsiz<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Diepctl5 {
    #[inline]
    fn from(other: u32) -> Self {
         Diepctl5(other)
    }
}

impl ::core::fmt::Display for Diepctl5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Diepctl5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.epena() != 0 { try!(write!(f, " epena"))}
        if self.epdis() != 0 { try!(write!(f, " epdis"))}
        if self.soddfrm() != 0 { try!(write!(f, " soddfrm"))}
        if self.sd0pid_sevnfrm() != 0 { try!(write!(f, " sd0pid_sevnfrm"))}
        if self.snak() != 0 { try!(write!(f, " snak"))}
        if self.cnak() != 0 { try!(write!(f, " cnak"))}
        if self.txfnum() != 0 { try!(write!(f, " txfnum=0x{:x}", self.txfnum()))}
        if self.stall() != 0 { try!(write!(f, " stall"))}
        if self.eptyp() != 0 { try!(write!(f, " eptyp=0x{:x}", self.eptyp()))}
        if self.naksts() != 0 { try!(write!(f, " naksts"))}
        if self.eonum_dpid() != 0 { try!(write!(f, " eonum_dpid"))}
        if self.usbaep() != 0 { try!(write!(f, " usbaep"))}
        if self.mpsiz() != 0 { try!(write!(f, " mpsiz=0x{:x}", self.mpsiz()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="device endpoint-5 interrupt register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Diepint5(pub u32);
impl Diepint5 {
    #[doc="TXFE"]
    #[inline] pub fn txfe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXFE != 0"]
    #[inline] pub fn test_txfe(&self) -> bool {
        self.txfe() != 0
    }

    #[doc="Sets the TXFE field."]
    #[inline] pub fn set_txfe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="INEPNE"]
    #[inline] pub fn inepne(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if INEPNE != 0"]
    #[inline] pub fn test_inepne(&self) -> bool {
        self.inepne() != 0
    }

    #[doc="Sets the INEPNE field."]
    #[inline] pub fn set_inepne<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="ITTXFE"]
    #[inline] pub fn ittxfe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ITTXFE != 0"]
    #[inline] pub fn test_ittxfe(&self) -> bool {
        self.ittxfe() != 0
    }

    #[doc="Sets the ITTXFE field."]
    #[inline] pub fn set_ittxfe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="TOC"]
    #[inline] pub fn toc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TOC != 0"]
    #[inline] pub fn test_toc(&self) -> bool {
        self.toc() != 0
    }

    #[doc="Sets the TOC field."]
    #[inline] pub fn set_toc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="EPDISD"]
    #[inline] pub fn epdisd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if EPDISD != 0"]
    #[inline] pub fn test_epdisd(&self) -> bool {
        self.epdisd() != 0
    }

    #[doc="Sets the EPDISD field."]
    #[inline] pub fn set_epdisd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="XFRC"]
    #[inline] pub fn xfrc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XFRC != 0"]
    #[inline] pub fn test_xfrc(&self) -> bool {
        self.xfrc() != 0
    }

    #[doc="Sets the XFRC field."]
    #[inline] pub fn set_xfrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Diepint5 {
    #[inline]
    fn from(other: u32) -> Self {
         Diepint5(other)
    }
}

impl ::core::fmt::Display for Diepint5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Diepint5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txfe() != 0 { try!(write!(f, " txfe"))}
        if self.inepne() != 0 { try!(write!(f, " inepne"))}
        if self.ittxfe() != 0 { try!(write!(f, " ittxfe"))}
        if self.toc() != 0 { try!(write!(f, " toc"))}
        if self.epdisd() != 0 { try!(write!(f, " epdisd"))}
        if self.xfrc() != 0 { try!(write!(f, " xfrc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="device endpoint-5 transfer size register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dieptsiz55(pub u32);
impl Dieptsiz55 {
    #[doc="Multi count"]
    #[inline] pub fn mcnt(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x3) as u8) } // [30:29]
    }

    #[doc="Returns true if MCNT != 0"]
    #[inline] pub fn test_mcnt(&self) -> bool {
        self.mcnt() != 0
    }

    #[doc="Sets the MCNT field."]
    #[inline] pub fn set_mcnt<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Packet count"]
    #[inline] pub fn pktcnt(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x3ff) as u16) } // [28:19]
    }

    #[doc="Returns true if PKTCNT != 0"]
    #[inline] pub fn test_pktcnt(&self) -> bool {
        self.pktcnt() != 0
    }

    #[doc="Sets the PKTCNT field."]
    #[inline] pub fn set_pktcnt<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Transfer size"]
    #[inline] pub fn xfrsiz(&self) -> bits::U19 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ffff) as u32) } // [18:0]
    }

    #[doc="Returns true if XFRSIZ != 0"]
    #[inline] pub fn test_xfrsiz(&self) -> bool {
        self.xfrsiz() != 0
    }

    #[doc="Sets the XFRSIZ field."]
    #[inline] pub fn set_xfrsiz<V: Into<bits::U19>>(mut self, value: V) -> Self {
        let value: bits::U19 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dieptsiz55 {
    #[inline]
    fn from(other: u32) -> Self {
         Dieptsiz55(other)
    }
}

impl ::core::fmt::Display for Dieptsiz55 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dieptsiz55 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mcnt() != 0 { try!(write!(f, " mcnt=0x{:x}", self.mcnt()))}
        if self.pktcnt() != 0 { try!(write!(f, " pktcnt=0x{:x}", self.pktcnt()))}
        if self.xfrsiz() != 0 { try!(write!(f, " xfrsiz=0x{:x}", self.xfrsiz()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS device IN endpoint transmit FIFO status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dtxfsts55(pub u32);
impl Dtxfsts55 {
    #[doc="IN endpoint TxFIFO space available"]
    #[inline] pub fn ineptfsav(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if INEPTFSAV != 0"]
    #[inline] pub fn test_ineptfsav(&self) -> bool {
        self.ineptfsav() != 0
    }

    #[doc="Sets the INEPTFSAV field."]
    #[inline] pub fn set_ineptfsav<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dtxfsts55 {
    #[inline]
    fn from(other: u32) -> Self {
         Dtxfsts55(other)
    }
}

impl ::core::fmt::Display for Dtxfsts55 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dtxfsts55 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ineptfsav() != 0 { try!(write!(f, " ineptfsav=0x{:x}", self.ineptfsav()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="device endpoint-4 control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Doepctl4(pub u32);
impl Doepctl4 {
    #[doc="EPENA"]
    #[inline] pub fn epena(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if EPENA != 0"]
    #[inline] pub fn test_epena(&self) -> bool {
        self.epena() != 0
    }

    #[doc="Sets the EPENA field."]
    #[inline] pub fn set_epena<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="EPDIS"]
    #[inline] pub fn epdis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if EPDIS != 0"]
    #[inline] pub fn test_epdis(&self) -> bool {
        self.epdis() != 0
    }

    #[doc="Sets the EPDIS field."]
    #[inline] pub fn set_epdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="SODDFRM"]
    #[inline] pub fn soddfrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if SODDFRM != 0"]
    #[inline] pub fn test_soddfrm(&self) -> bool {
        self.soddfrm() != 0
    }

    #[doc="Sets the SODDFRM field."]
    #[inline] pub fn set_soddfrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="SD0PID/SEVNFRM"]
    #[inline] pub fn sd0pid_sevnfrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if SD0PID_SEVNFRM != 0"]
    #[inline] pub fn test_sd0pid_sevnfrm(&self) -> bool {
        self.sd0pid_sevnfrm() != 0
    }

    #[doc="Sets the SD0PID_SEVNFRM field."]
    #[inline] pub fn set_sd0pid_sevnfrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="SNAK"]
    #[inline] pub fn snak(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if SNAK != 0"]
    #[inline] pub fn test_snak(&self) -> bool {
        self.snak() != 0
    }

    #[doc="Sets the SNAK field."]
    #[inline] pub fn set_snak<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="CNAK"]
    #[inline] pub fn cnak(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if CNAK != 0"]
    #[inline] pub fn test_cnak(&self) -> bool {
        self.cnak() != 0
    }

    #[doc="Sets the CNAK field."]
    #[inline] pub fn set_cnak<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Stall"]
    #[inline] pub fn stall(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if Stall != 0"]
    #[inline] pub fn test_stall(&self) -> bool {
        self.stall() != 0
    }

    #[doc="Sets the Stall field."]
    #[inline] pub fn set_stall<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="SNPM"]
    #[inline] pub fn snpm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if SNPM != 0"]
    #[inline] pub fn test_snpm(&self) -> bool {
        self.snpm() != 0
    }

    #[doc="Sets the SNPM field."]
    #[inline] pub fn set_snpm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="EPTYP"]
    #[inline] pub fn eptyp(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x3) as u8) } // [19:18]
    }

    #[doc="Returns true if EPTYP != 0"]
    #[inline] pub fn test_eptyp(&self) -> bool {
        self.eptyp() != 0
    }

    #[doc="Sets the EPTYP field."]
    #[inline] pub fn set_eptyp<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="NAKSTS"]
    #[inline] pub fn naksts(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if NAKSTS != 0"]
    #[inline] pub fn test_naksts(&self) -> bool {
        self.naksts() != 0
    }

    #[doc="Sets the NAKSTS field."]
    #[inline] pub fn set_naksts<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="EONUM/DPID"]
    #[inline] pub fn eonum_dpid(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if EONUM_DPID != 0"]
    #[inline] pub fn test_eonum_dpid(&self) -> bool {
        self.eonum_dpid() != 0
    }

    #[doc="Sets the EONUM_DPID field."]
    #[inline] pub fn set_eonum_dpid<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="USBAEP"]
    #[inline] pub fn usbaep(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if USBAEP != 0"]
    #[inline] pub fn test_usbaep(&self) -> bool {
        self.usbaep() != 0
    }

    #[doc="Sets the USBAEP field."]
    #[inline] pub fn set_usbaep<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="MPSIZ"]
    #[inline] pub fn mpsiz(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
    }

    #[doc="Returns true if MPSIZ != 0"]
    #[inline] pub fn test_mpsiz(&self) -> bool {
        self.mpsiz() != 0
    }

    #[doc="Sets the MPSIZ field."]
    #[inline] pub fn set_mpsiz<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Doepctl4 {
    #[inline]
    fn from(other: u32) -> Self {
         Doepctl4(other)
    }
}

impl ::core::fmt::Display for Doepctl4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Doepctl4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.epena() != 0 { try!(write!(f, " epena"))}
        if self.epdis() != 0 { try!(write!(f, " epdis"))}
        if self.soddfrm() != 0 { try!(write!(f, " soddfrm"))}
        if self.sd0pid_sevnfrm() != 0 { try!(write!(f, " sd0pid_sevnfrm"))}
        if self.snak() != 0 { try!(write!(f, " snak"))}
        if self.cnak() != 0 { try!(write!(f, " cnak"))}
        if self.stall() != 0 { try!(write!(f, " stall"))}
        if self.snpm() != 0 { try!(write!(f, " snpm"))}
        if self.eptyp() != 0 { try!(write!(f, " eptyp=0x{:x}", self.eptyp()))}
        if self.naksts() != 0 { try!(write!(f, " naksts"))}
        if self.eonum_dpid() != 0 { try!(write!(f, " eonum_dpid"))}
        if self.usbaep() != 0 { try!(write!(f, " usbaep"))}
        if self.mpsiz() != 0 { try!(write!(f, " mpsiz=0x{:x}", self.mpsiz()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="device endpoint-4 interrupt register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Doepint4(pub u32);
impl Doepint4 {
    #[doc="B2BSTUP"]
    #[inline] pub fn b2bstup(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if B2BSTUP != 0"]
    #[inline] pub fn test_b2bstup(&self) -> bool {
        self.b2bstup() != 0
    }

    #[doc="Sets the B2BSTUP field."]
    #[inline] pub fn set_b2bstup<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="OTEPDIS"]
    #[inline] pub fn otepdis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if OTEPDIS != 0"]
    #[inline] pub fn test_otepdis(&self) -> bool {
        self.otepdis() != 0
    }

    #[doc="Sets the OTEPDIS field."]
    #[inline] pub fn set_otepdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="STUP"]
    #[inline] pub fn stup(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STUP != 0"]
    #[inline] pub fn test_stup(&self) -> bool {
        self.stup() != 0
    }

    #[doc="Sets the STUP field."]
    #[inline] pub fn set_stup<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="EPDISD"]
    #[inline] pub fn epdisd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if EPDISD != 0"]
    #[inline] pub fn test_epdisd(&self) -> bool {
        self.epdisd() != 0
    }

    #[doc="Sets the EPDISD field."]
    #[inline] pub fn set_epdisd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="XFRC"]
    #[inline] pub fn xfrc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XFRC != 0"]
    #[inline] pub fn test_xfrc(&self) -> bool {
        self.xfrc() != 0
    }

    #[doc="Sets the XFRC field."]
    #[inline] pub fn set_xfrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Doepint4 {
    #[inline]
    fn from(other: u32) -> Self {
         Doepint4(other)
    }
}

impl ::core::fmt::Display for Doepint4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Doepint4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.b2bstup() != 0 { try!(write!(f, " b2bstup"))}
        if self.otepdis() != 0 { try!(write!(f, " otepdis"))}
        if self.stup() != 0 { try!(write!(f, " stup"))}
        if self.epdisd() != 0 { try!(write!(f, " epdisd"))}
        if self.xfrc() != 0 { try!(write!(f, " xfrc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="device OUT endpoint-4 transfer size register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Doeptsiz4(pub u32);
impl Doeptsiz4 {
    #[doc="Received data PID/SETUP packet count"]
    #[inline] pub fn rxdpid_stupcnt(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x3) as u8) } // [30:29]
    }

    #[doc="Returns true if RXDPID_STUPCNT != 0"]
    #[inline] pub fn test_rxdpid_stupcnt(&self) -> bool {
        self.rxdpid_stupcnt() != 0
    }

    #[doc="Sets the RXDPID_STUPCNT field."]
    #[inline] pub fn set_rxdpid_stupcnt<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Packet count"]
    #[inline] pub fn pktcnt(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x3ff) as u16) } // [28:19]
    }

    #[doc="Returns true if PKTCNT != 0"]
    #[inline] pub fn test_pktcnt(&self) -> bool {
        self.pktcnt() != 0
    }

    #[doc="Sets the PKTCNT field."]
    #[inline] pub fn set_pktcnt<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Transfer size"]
    #[inline] pub fn xfrsiz(&self) -> bits::U19 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ffff) as u32) } // [18:0]
    }

    #[doc="Returns true if XFRSIZ != 0"]
    #[inline] pub fn test_xfrsiz(&self) -> bool {
        self.xfrsiz() != 0
    }

    #[doc="Sets the XFRSIZ field."]
    #[inline] pub fn set_xfrsiz<V: Into<bits::U19>>(mut self, value: V) -> Self {
        let value: bits::U19 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Doeptsiz4 {
    #[inline]
    fn from(other: u32) -> Self {
         Doeptsiz4(other)
    }
}

impl ::core::fmt::Display for Doeptsiz4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Doeptsiz4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rxdpid_stupcnt() != 0 { try!(write!(f, " rxdpid_stupcnt=0x{:x}", self.rxdpid_stupcnt()))}
        if self.pktcnt() != 0 { try!(write!(f, " pktcnt=0x{:x}", self.pktcnt()))}
        if self.xfrsiz() != 0 { try!(write!(f, " xfrsiz=0x{:x}", self.xfrsiz()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="device endpoint-5 control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Doepctl5(pub u32);
impl Doepctl5 {
    #[doc="EPENA"]
    #[inline] pub fn epena(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if EPENA != 0"]
    #[inline] pub fn test_epena(&self) -> bool {
        self.epena() != 0
    }

    #[doc="Sets the EPENA field."]
    #[inline] pub fn set_epena<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="EPDIS"]
    #[inline] pub fn epdis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if EPDIS != 0"]
    #[inline] pub fn test_epdis(&self) -> bool {
        self.epdis() != 0
    }

    #[doc="Sets the EPDIS field."]
    #[inline] pub fn set_epdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="SODDFRM"]
    #[inline] pub fn soddfrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if SODDFRM != 0"]
    #[inline] pub fn test_soddfrm(&self) -> bool {
        self.soddfrm() != 0
    }

    #[doc="Sets the SODDFRM field."]
    #[inline] pub fn set_soddfrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="SD0PID/SEVNFRM"]
    #[inline] pub fn sd0pid_sevnfrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if SD0PID_SEVNFRM != 0"]
    #[inline] pub fn test_sd0pid_sevnfrm(&self) -> bool {
        self.sd0pid_sevnfrm() != 0
    }

    #[doc="Sets the SD0PID_SEVNFRM field."]
    #[inline] pub fn set_sd0pid_sevnfrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="SNAK"]
    #[inline] pub fn snak(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if SNAK != 0"]
    #[inline] pub fn test_snak(&self) -> bool {
        self.snak() != 0
    }

    #[doc="Sets the SNAK field."]
    #[inline] pub fn set_snak<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="CNAK"]
    #[inline] pub fn cnak(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if CNAK != 0"]
    #[inline] pub fn test_cnak(&self) -> bool {
        self.cnak() != 0
    }

    #[doc="Sets the CNAK field."]
    #[inline] pub fn set_cnak<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Stall"]
    #[inline] pub fn stall(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if Stall != 0"]
    #[inline] pub fn test_stall(&self) -> bool {
        self.stall() != 0
    }

    #[doc="Sets the Stall field."]
    #[inline] pub fn set_stall<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="SNPM"]
    #[inline] pub fn snpm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if SNPM != 0"]
    #[inline] pub fn test_snpm(&self) -> bool {
        self.snpm() != 0
    }

    #[doc="Sets the SNPM field."]
    #[inline] pub fn set_snpm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="EPTYP"]
    #[inline] pub fn eptyp(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x3) as u8) } // [19:18]
    }

    #[doc="Returns true if EPTYP != 0"]
    #[inline] pub fn test_eptyp(&self) -> bool {
        self.eptyp() != 0
    }

    #[doc="Sets the EPTYP field."]
    #[inline] pub fn set_eptyp<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="NAKSTS"]
    #[inline] pub fn naksts(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if NAKSTS != 0"]
    #[inline] pub fn test_naksts(&self) -> bool {
        self.naksts() != 0
    }

    #[doc="Sets the NAKSTS field."]
    #[inline] pub fn set_naksts<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="EONUM/DPID"]
    #[inline] pub fn eonum_dpid(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if EONUM_DPID != 0"]
    #[inline] pub fn test_eonum_dpid(&self) -> bool {
        self.eonum_dpid() != 0
    }

    #[doc="Sets the EONUM_DPID field."]
    #[inline] pub fn set_eonum_dpid<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="USBAEP"]
    #[inline] pub fn usbaep(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if USBAEP != 0"]
    #[inline] pub fn test_usbaep(&self) -> bool {
        self.usbaep() != 0
    }

    #[doc="Sets the USBAEP field."]
    #[inline] pub fn set_usbaep<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="MPSIZ"]
    #[inline] pub fn mpsiz(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
    }

    #[doc="Returns true if MPSIZ != 0"]
    #[inline] pub fn test_mpsiz(&self) -> bool {
        self.mpsiz() != 0
    }

    #[doc="Sets the MPSIZ field."]
    #[inline] pub fn set_mpsiz<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Doepctl5 {
    #[inline]
    fn from(other: u32) -> Self {
         Doepctl5(other)
    }
}

impl ::core::fmt::Display for Doepctl5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Doepctl5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.epena() != 0 { try!(write!(f, " epena"))}
        if self.epdis() != 0 { try!(write!(f, " epdis"))}
        if self.soddfrm() != 0 { try!(write!(f, " soddfrm"))}
        if self.sd0pid_sevnfrm() != 0 { try!(write!(f, " sd0pid_sevnfrm"))}
        if self.snak() != 0 { try!(write!(f, " snak"))}
        if self.cnak() != 0 { try!(write!(f, " cnak"))}
        if self.stall() != 0 { try!(write!(f, " stall"))}
        if self.snpm() != 0 { try!(write!(f, " snpm"))}
        if self.eptyp() != 0 { try!(write!(f, " eptyp=0x{:x}", self.eptyp()))}
        if self.naksts() != 0 { try!(write!(f, " naksts"))}
        if self.eonum_dpid() != 0 { try!(write!(f, " eonum_dpid"))}
        if self.usbaep() != 0 { try!(write!(f, " usbaep"))}
        if self.mpsiz() != 0 { try!(write!(f, " mpsiz=0x{:x}", self.mpsiz()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="device endpoint-5 interrupt register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Doepint5(pub u32);
impl Doepint5 {
    #[doc="B2BSTUP"]
    #[inline] pub fn b2bstup(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if B2BSTUP != 0"]
    #[inline] pub fn test_b2bstup(&self) -> bool {
        self.b2bstup() != 0
    }

    #[doc="Sets the B2BSTUP field."]
    #[inline] pub fn set_b2bstup<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="OTEPDIS"]
    #[inline] pub fn otepdis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if OTEPDIS != 0"]
    #[inline] pub fn test_otepdis(&self) -> bool {
        self.otepdis() != 0
    }

    #[doc="Sets the OTEPDIS field."]
    #[inline] pub fn set_otepdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="STUP"]
    #[inline] pub fn stup(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STUP != 0"]
    #[inline] pub fn test_stup(&self) -> bool {
        self.stup() != 0
    }

    #[doc="Sets the STUP field."]
    #[inline] pub fn set_stup<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="EPDISD"]
    #[inline] pub fn epdisd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if EPDISD != 0"]
    #[inline] pub fn test_epdisd(&self) -> bool {
        self.epdisd() != 0
    }

    #[doc="Sets the EPDISD field."]
    #[inline] pub fn set_epdisd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="XFRC"]
    #[inline] pub fn xfrc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XFRC != 0"]
    #[inline] pub fn test_xfrc(&self) -> bool {
        self.xfrc() != 0
    }

    #[doc="Sets the XFRC field."]
    #[inline] pub fn set_xfrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Doepint5 {
    #[inline]
    fn from(other: u32) -> Self {
         Doepint5(other)
    }
}

impl ::core::fmt::Display for Doepint5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Doepint5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.b2bstup() != 0 { try!(write!(f, " b2bstup"))}
        if self.otepdis() != 0 { try!(write!(f, " otepdis"))}
        if self.stup() != 0 { try!(write!(f, " stup"))}
        if self.epdisd() != 0 { try!(write!(f, " epdisd"))}
        if self.xfrc() != 0 { try!(write!(f, " xfrc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="device OUT endpoint-5 transfer size register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Doeptsiz5(pub u32);
impl Doeptsiz5 {
    #[doc="Received data PID/SETUP packet count"]
    #[inline] pub fn rxdpid_stupcnt(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x3) as u8) } // [30:29]
    }

    #[doc="Returns true if RXDPID_STUPCNT != 0"]
    #[inline] pub fn test_rxdpid_stupcnt(&self) -> bool {
        self.rxdpid_stupcnt() != 0
    }

    #[doc="Sets the RXDPID_STUPCNT field."]
    #[inline] pub fn set_rxdpid_stupcnt<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Packet count"]
    #[inline] pub fn pktcnt(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x3ff) as u16) } // [28:19]
    }

    #[doc="Returns true if PKTCNT != 0"]
    #[inline] pub fn test_pktcnt(&self) -> bool {
        self.pktcnt() != 0
    }

    #[doc="Sets the PKTCNT field."]
    #[inline] pub fn set_pktcnt<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Transfer size"]
    #[inline] pub fn xfrsiz(&self) -> bits::U19 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ffff) as u32) } // [18:0]
    }

    #[doc="Returns true if XFRSIZ != 0"]
    #[inline] pub fn test_xfrsiz(&self) -> bool {
        self.xfrsiz() != 0
    }

    #[doc="Sets the XFRSIZ field."]
    #[inline] pub fn set_xfrsiz<V: Into<bits::U19>>(mut self, value: V) -> Self {
        let value: bits::U19 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Doeptsiz5 {
    #[inline]
    fn from(other: u32) -> Self {
         Doeptsiz5(other)
    }
}

impl ::core::fmt::Display for Doeptsiz5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Doeptsiz5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rxdpid_stupcnt() != 0 { try!(write!(f, " rxdpid_stupcnt=0x{:x}", self.rxdpid_stupcnt()))}
        if self.pktcnt() != 0 { try!(write!(f, " pktcnt=0x{:x}", self.pktcnt()))}
        if self.xfrsiz() != 0 { try!(write!(f, " xfrsiz=0x{:x}", self.xfrsiz()))}
        try!(write!(f, "]"));
        Ok(())
    }
}


