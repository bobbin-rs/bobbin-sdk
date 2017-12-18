#[allow(unused_imports)] use bobbin_common::*;

periph!( CAN1, Can1, _CAN1, CanPeriph, 0x40006400);
periph!( CAN2, Can2, _CAN2, CanPeriph, 0x40006800);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="CAN Peripheral"]
pub struct CanPeriph(pub usize); 

impl super::sig::Signal<super::sig::Can1Tx> for Can1 {}
impl super::sig::SignalCanTx<super::sig::Can1Tx> for Can1 {}
impl super::sig::Signal<super::sig::Can1Rx> for Can1 {}
impl super::sig::SignalCanRx<super::sig::Can1Rx> for Can1 {}

impl super::sig::Signal<super::sig::Can2Tx> for Can2 {}
impl super::sig::SignalCanTx<super::sig::Can2Tx> for Can2 {}
impl super::sig::Signal<super::sig::Can2Rx> for Can2 {}
impl super::sig::SignalCanRx<super::sig::Can2Rx> for Can2 {}


impl CanPeriph {
    #[doc="Get the *mut pointer for the MCR register."]
    #[inline] pub fn mcr_mut(&self) -> *mut Mcr { 
        (self.0 + 0x0) as *mut Mcr
    }

    #[doc="Get the *const pointer for the MCR register."]
    #[inline] pub fn mcr_ptr(&self) -> *const Mcr { 
           self.mcr_mut()
    }

    #[doc="Read the MCR register."]
    #[inline] pub fn mcr(&self) -> Mcr { 
        unsafe {
            read_volatile(self.mcr_ptr())
        }
    }

    #[doc="Write the MCR register."]
    #[inline] pub fn set_mcr<F: FnOnce(Mcr) -> Mcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mcr_mut(), f(Mcr(0)));
        }
        self
    }

    #[doc="Modify the MCR register."]
    #[inline] pub fn with_mcr<F: FnOnce(Mcr) -> Mcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mcr_mut(), f(self.mcr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MSR register."]
    #[inline] pub fn msr_mut(&self) -> *mut Msr { 
        (self.0 + 0x4) as *mut Msr
    }

    #[doc="Get the *const pointer for the MSR register."]
    #[inline] pub fn msr_ptr(&self) -> *const Msr { 
           self.msr_mut()
    }

    #[doc="Read the MSR register."]
    #[inline] pub fn msr(&self) -> Msr { 
        unsafe {
            read_volatile(self.msr_ptr())
        }
    }

    #[doc="Write the MSR register."]
    #[inline] pub fn set_msr<F: FnOnce(Msr) -> Msr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.msr_mut(), f(Msr(0)));
        }
        self
    }

    #[doc="Modify the MSR register."]
    #[inline] pub fn with_msr<F: FnOnce(Msr) -> Msr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.msr_mut(), f(self.msr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TSR register."]
    #[inline] pub fn tsr_mut(&self) -> *mut Tsr { 
        (self.0 + 0x8) as *mut Tsr
    }

    #[doc="Get the *const pointer for the TSR register."]
    #[inline] pub fn tsr_ptr(&self) -> *const Tsr { 
           self.tsr_mut()
    }

    #[doc="Read the TSR register."]
    #[inline] pub fn tsr(&self) -> Tsr { 
        unsafe {
            read_volatile(self.tsr_ptr())
        }
    }

    #[doc="Write the TSR register."]
    #[inline] pub fn set_tsr<F: FnOnce(Tsr) -> Tsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tsr_mut(), f(Tsr(0)));
        }
        self
    }

    #[doc="Modify the TSR register."]
    #[inline] pub fn with_tsr<F: FnOnce(Tsr) -> Tsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tsr_mut(), f(self.tsr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the RF0R register."]
    #[inline] pub fn rf0r_mut(&self) -> *mut Rf0r { 
        (self.0 + 0xc) as *mut Rf0r
    }

    #[doc="Get the *const pointer for the RF0R register."]
    #[inline] pub fn rf0r_ptr(&self) -> *const Rf0r { 
           self.rf0r_mut()
    }

    #[doc="Read the RF0R register."]
    #[inline] pub fn rf0r(&self) -> Rf0r { 
        unsafe {
            read_volatile(self.rf0r_ptr())
        }
    }

    #[doc="Write the RF0R register."]
    #[inline] pub fn set_rf0r<F: FnOnce(Rf0r) -> Rf0r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.rf0r_mut(), f(Rf0r(0)));
        }
        self
    }

    #[doc="Modify the RF0R register."]
    #[inline] pub fn with_rf0r<F: FnOnce(Rf0r) -> Rf0r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.rf0r_mut(), f(self.rf0r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the RF1R register."]
    #[inline] pub fn rf1r_mut(&self) -> *mut Rf1r { 
        (self.0 + 0x10) as *mut Rf1r
    }

    #[doc="Get the *const pointer for the RF1R register."]
    #[inline] pub fn rf1r_ptr(&self) -> *const Rf1r { 
           self.rf1r_mut()
    }

    #[doc="Read the RF1R register."]
    #[inline] pub fn rf1r(&self) -> Rf1r { 
        unsafe {
            read_volatile(self.rf1r_ptr())
        }
    }

    #[doc="Write the RF1R register."]
    #[inline] pub fn set_rf1r<F: FnOnce(Rf1r) -> Rf1r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.rf1r_mut(), f(Rf1r(0)));
        }
        self
    }

    #[doc="Modify the RF1R register."]
    #[inline] pub fn with_rf1r<F: FnOnce(Rf1r) -> Rf1r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.rf1r_mut(), f(self.rf1r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IER register."]
    #[inline] pub fn ier_mut(&self) -> *mut Ier { 
        (self.0 + 0x14) as *mut Ier
    }

    #[doc="Get the *const pointer for the IER register."]
    #[inline] pub fn ier_ptr(&self) -> *const Ier { 
           self.ier_mut()
    }

    #[doc="Read the IER register."]
    #[inline] pub fn ier(&self) -> Ier { 
        unsafe {
            read_volatile(self.ier_ptr())
        }
    }

    #[doc="Write the IER register."]
    #[inline] pub fn set_ier<F: FnOnce(Ier) -> Ier>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ier_mut(), f(Ier(0)));
        }
        self
    }

    #[doc="Modify the IER register."]
    #[inline] pub fn with_ier<F: FnOnce(Ier) -> Ier>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ier_mut(), f(self.ier()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ESR register."]
    #[inline] pub fn esr_mut(&self) -> *mut Esr { 
        (self.0 + 0x18) as *mut Esr
    }

    #[doc="Get the *const pointer for the ESR register."]
    #[inline] pub fn esr_ptr(&self) -> *const Esr { 
           self.esr_mut()
    }

    #[doc="Read the ESR register."]
    #[inline] pub fn esr(&self) -> Esr { 
        unsafe {
            read_volatile(self.esr_ptr())
        }
    }

    #[doc="Write the ESR register."]
    #[inline] pub fn set_esr<F: FnOnce(Esr) -> Esr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.esr_mut(), f(Esr(0)));
        }
        self
    }

    #[doc="Modify the ESR register."]
    #[inline] pub fn with_esr<F: FnOnce(Esr) -> Esr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.esr_mut(), f(self.esr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the BTR register."]
    #[inline] pub fn btr_mut(&self) -> *mut Btr { 
        (self.0 + 0x1c) as *mut Btr
    }

    #[doc="Get the *const pointer for the BTR register."]
    #[inline] pub fn btr_ptr(&self) -> *const Btr { 
           self.btr_mut()
    }

    #[doc="Read the BTR register."]
    #[inline] pub fn btr(&self) -> Btr { 
        unsafe {
            read_volatile(self.btr_ptr())
        }
    }

    #[doc="Write the BTR register."]
    #[inline] pub fn set_btr<F: FnOnce(Btr) -> Btr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.btr_mut(), f(Btr(0)));
        }
        self
    }

    #[doc="Modify the BTR register."]
    #[inline] pub fn with_btr<F: FnOnce(Btr) -> Btr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.btr_mut(), f(self.btr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TI0R register."]
    #[inline] pub fn ti0r_mut(&self) -> *mut Ti0r { 
        (self.0 + 0x180) as *mut Ti0r
    }

    #[doc="Get the *const pointer for the TI0R register."]
    #[inline] pub fn ti0r_ptr(&self) -> *const Ti0r { 
           self.ti0r_mut()
    }

    #[doc="Read the TI0R register."]
    #[inline] pub fn ti0r(&self) -> Ti0r { 
        unsafe {
            read_volatile(self.ti0r_ptr())
        }
    }

    #[doc="Write the TI0R register."]
    #[inline] pub fn set_ti0r<F: FnOnce(Ti0r) -> Ti0r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ti0r_mut(), f(Ti0r(0)));
        }
        self
    }

    #[doc="Modify the TI0R register."]
    #[inline] pub fn with_ti0r<F: FnOnce(Ti0r) -> Ti0r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ti0r_mut(), f(self.ti0r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TDT0R register."]
    #[inline] pub fn tdt0r_mut(&self) -> *mut Tdt0r { 
        (self.0 + 0x184) as *mut Tdt0r
    }

    #[doc="Get the *const pointer for the TDT0R register."]
    #[inline] pub fn tdt0r_ptr(&self) -> *const Tdt0r { 
           self.tdt0r_mut()
    }

    #[doc="Read the TDT0R register."]
    #[inline] pub fn tdt0r(&self) -> Tdt0r { 
        unsafe {
            read_volatile(self.tdt0r_ptr())
        }
    }

    #[doc="Write the TDT0R register."]
    #[inline] pub fn set_tdt0r<F: FnOnce(Tdt0r) -> Tdt0r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tdt0r_mut(), f(Tdt0r(0)));
        }
        self
    }

    #[doc="Modify the TDT0R register."]
    #[inline] pub fn with_tdt0r<F: FnOnce(Tdt0r) -> Tdt0r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tdt0r_mut(), f(self.tdt0r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TDL0R register."]
    #[inline] pub fn tdl0r_mut(&self) -> *mut Tdl0r { 
        (self.0 + 0x188) as *mut Tdl0r
    }

    #[doc="Get the *const pointer for the TDL0R register."]
    #[inline] pub fn tdl0r_ptr(&self) -> *const Tdl0r { 
           self.tdl0r_mut()
    }

    #[doc="Read the TDL0R register."]
    #[inline] pub fn tdl0r(&self) -> Tdl0r { 
        unsafe {
            read_volatile(self.tdl0r_ptr())
        }
    }

    #[doc="Write the TDL0R register."]
    #[inline] pub fn set_tdl0r<F: FnOnce(Tdl0r) -> Tdl0r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tdl0r_mut(), f(Tdl0r(0)));
        }
        self
    }

    #[doc="Modify the TDL0R register."]
    #[inline] pub fn with_tdl0r<F: FnOnce(Tdl0r) -> Tdl0r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tdl0r_mut(), f(self.tdl0r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TDH0R register."]
    #[inline] pub fn tdh0r_mut(&self) -> *mut Tdh0r { 
        (self.0 + 0x18c) as *mut Tdh0r
    }

    #[doc="Get the *const pointer for the TDH0R register."]
    #[inline] pub fn tdh0r_ptr(&self) -> *const Tdh0r { 
           self.tdh0r_mut()
    }

    #[doc="Read the TDH0R register."]
    #[inline] pub fn tdh0r(&self) -> Tdh0r { 
        unsafe {
            read_volatile(self.tdh0r_ptr())
        }
    }

    #[doc="Write the TDH0R register."]
    #[inline] pub fn set_tdh0r<F: FnOnce(Tdh0r) -> Tdh0r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tdh0r_mut(), f(Tdh0r(0)));
        }
        self
    }

    #[doc="Modify the TDH0R register."]
    #[inline] pub fn with_tdh0r<F: FnOnce(Tdh0r) -> Tdh0r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tdh0r_mut(), f(self.tdh0r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TI1R register."]
    #[inline] pub fn ti1r_mut(&self) -> *mut Ti1r { 
        (self.0 + 0x190) as *mut Ti1r
    }

    #[doc="Get the *const pointer for the TI1R register."]
    #[inline] pub fn ti1r_ptr(&self) -> *const Ti1r { 
           self.ti1r_mut()
    }

    #[doc="Read the TI1R register."]
    #[inline] pub fn ti1r(&self) -> Ti1r { 
        unsafe {
            read_volatile(self.ti1r_ptr())
        }
    }

    #[doc="Write the TI1R register."]
    #[inline] pub fn set_ti1r<F: FnOnce(Ti1r) -> Ti1r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ti1r_mut(), f(Ti1r(0)));
        }
        self
    }

    #[doc="Modify the TI1R register."]
    #[inline] pub fn with_ti1r<F: FnOnce(Ti1r) -> Ti1r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ti1r_mut(), f(self.ti1r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TDT1R register."]
    #[inline] pub fn tdt1r_mut(&self) -> *mut Tdt1r { 
        (self.0 + 0x194) as *mut Tdt1r
    }

    #[doc="Get the *const pointer for the TDT1R register."]
    #[inline] pub fn tdt1r_ptr(&self) -> *const Tdt1r { 
           self.tdt1r_mut()
    }

    #[doc="Read the TDT1R register."]
    #[inline] pub fn tdt1r(&self) -> Tdt1r { 
        unsafe {
            read_volatile(self.tdt1r_ptr())
        }
    }

    #[doc="Write the TDT1R register."]
    #[inline] pub fn set_tdt1r<F: FnOnce(Tdt1r) -> Tdt1r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tdt1r_mut(), f(Tdt1r(0)));
        }
        self
    }

    #[doc="Modify the TDT1R register."]
    #[inline] pub fn with_tdt1r<F: FnOnce(Tdt1r) -> Tdt1r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tdt1r_mut(), f(self.tdt1r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TDL1R register."]
    #[inline] pub fn tdl1r_mut(&self) -> *mut Tdl1r { 
        (self.0 + 0x198) as *mut Tdl1r
    }

    #[doc="Get the *const pointer for the TDL1R register."]
    #[inline] pub fn tdl1r_ptr(&self) -> *const Tdl1r { 
           self.tdl1r_mut()
    }

    #[doc="Read the TDL1R register."]
    #[inline] pub fn tdl1r(&self) -> Tdl1r { 
        unsafe {
            read_volatile(self.tdl1r_ptr())
        }
    }

    #[doc="Write the TDL1R register."]
    #[inline] pub fn set_tdl1r<F: FnOnce(Tdl1r) -> Tdl1r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tdl1r_mut(), f(Tdl1r(0)));
        }
        self
    }

    #[doc="Modify the TDL1R register."]
    #[inline] pub fn with_tdl1r<F: FnOnce(Tdl1r) -> Tdl1r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tdl1r_mut(), f(self.tdl1r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TDH1R register."]
    #[inline] pub fn tdh1r_mut(&self) -> *mut Tdh1r { 
        (self.0 + 0x19c) as *mut Tdh1r
    }

    #[doc="Get the *const pointer for the TDH1R register."]
    #[inline] pub fn tdh1r_ptr(&self) -> *const Tdh1r { 
           self.tdh1r_mut()
    }

    #[doc="Read the TDH1R register."]
    #[inline] pub fn tdh1r(&self) -> Tdh1r { 
        unsafe {
            read_volatile(self.tdh1r_ptr())
        }
    }

    #[doc="Write the TDH1R register."]
    #[inline] pub fn set_tdh1r<F: FnOnce(Tdh1r) -> Tdh1r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tdh1r_mut(), f(Tdh1r(0)));
        }
        self
    }

    #[doc="Modify the TDH1R register."]
    #[inline] pub fn with_tdh1r<F: FnOnce(Tdh1r) -> Tdh1r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tdh1r_mut(), f(self.tdh1r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TI2R register."]
    #[inline] pub fn ti2r_mut(&self) -> *mut Ti2r { 
        (self.0 + 0x1a0) as *mut Ti2r
    }

    #[doc="Get the *const pointer for the TI2R register."]
    #[inline] pub fn ti2r_ptr(&self) -> *const Ti2r { 
           self.ti2r_mut()
    }

    #[doc="Read the TI2R register."]
    #[inline] pub fn ti2r(&self) -> Ti2r { 
        unsafe {
            read_volatile(self.ti2r_ptr())
        }
    }

    #[doc="Write the TI2R register."]
    #[inline] pub fn set_ti2r<F: FnOnce(Ti2r) -> Ti2r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ti2r_mut(), f(Ti2r(0)));
        }
        self
    }

    #[doc="Modify the TI2R register."]
    #[inline] pub fn with_ti2r<F: FnOnce(Ti2r) -> Ti2r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ti2r_mut(), f(self.ti2r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TDT2R register."]
    #[inline] pub fn tdt2r_mut(&self) -> *mut Tdt2r { 
        (self.0 + 0x1a4) as *mut Tdt2r
    }

    #[doc="Get the *const pointer for the TDT2R register."]
    #[inline] pub fn tdt2r_ptr(&self) -> *const Tdt2r { 
           self.tdt2r_mut()
    }

    #[doc="Read the TDT2R register."]
    #[inline] pub fn tdt2r(&self) -> Tdt2r { 
        unsafe {
            read_volatile(self.tdt2r_ptr())
        }
    }

    #[doc="Write the TDT2R register."]
    #[inline] pub fn set_tdt2r<F: FnOnce(Tdt2r) -> Tdt2r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tdt2r_mut(), f(Tdt2r(0)));
        }
        self
    }

    #[doc="Modify the TDT2R register."]
    #[inline] pub fn with_tdt2r<F: FnOnce(Tdt2r) -> Tdt2r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tdt2r_mut(), f(self.tdt2r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TDL2R register."]
    #[inline] pub fn tdl2r_mut(&self) -> *mut Tdl2r { 
        (self.0 + 0x1a8) as *mut Tdl2r
    }

    #[doc="Get the *const pointer for the TDL2R register."]
    #[inline] pub fn tdl2r_ptr(&self) -> *const Tdl2r { 
           self.tdl2r_mut()
    }

    #[doc="Read the TDL2R register."]
    #[inline] pub fn tdl2r(&self) -> Tdl2r { 
        unsafe {
            read_volatile(self.tdl2r_ptr())
        }
    }

    #[doc="Write the TDL2R register."]
    #[inline] pub fn set_tdl2r<F: FnOnce(Tdl2r) -> Tdl2r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tdl2r_mut(), f(Tdl2r(0)));
        }
        self
    }

    #[doc="Modify the TDL2R register."]
    #[inline] pub fn with_tdl2r<F: FnOnce(Tdl2r) -> Tdl2r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tdl2r_mut(), f(self.tdl2r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TDH2R register."]
    #[inline] pub fn tdh2r_mut(&self) -> *mut Tdh2r { 
        (self.0 + 0x1ac) as *mut Tdh2r
    }

    #[doc="Get the *const pointer for the TDH2R register."]
    #[inline] pub fn tdh2r_ptr(&self) -> *const Tdh2r { 
           self.tdh2r_mut()
    }

    #[doc="Read the TDH2R register."]
    #[inline] pub fn tdh2r(&self) -> Tdh2r { 
        unsafe {
            read_volatile(self.tdh2r_ptr())
        }
    }

    #[doc="Write the TDH2R register."]
    #[inline] pub fn set_tdh2r<F: FnOnce(Tdh2r) -> Tdh2r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tdh2r_mut(), f(Tdh2r(0)));
        }
        self
    }

    #[doc="Modify the TDH2R register."]
    #[inline] pub fn with_tdh2r<F: FnOnce(Tdh2r) -> Tdh2r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tdh2r_mut(), f(self.tdh2r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the RI0R register."]
    #[inline] pub fn ri0r_mut(&self) -> *mut Ri0r { 
        (self.0 + 0x1b0) as *mut Ri0r
    }

    #[doc="Get the *const pointer for the RI0R register."]
    #[inline] pub fn ri0r_ptr(&self) -> *const Ri0r { 
           self.ri0r_mut()
    }

    #[doc="Read the RI0R register."]
    #[inline] pub fn ri0r(&self) -> Ri0r { 
        unsafe {
            read_volatile(self.ri0r_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RDT0R register."]
    #[inline] pub fn rdt0r_mut(&self) -> *mut Rdt0r { 
        (self.0 + 0x1b4) as *mut Rdt0r
    }

    #[doc="Get the *const pointer for the RDT0R register."]
    #[inline] pub fn rdt0r_ptr(&self) -> *const Rdt0r { 
           self.rdt0r_mut()
    }

    #[doc="Read the RDT0R register."]
    #[inline] pub fn rdt0r(&self) -> Rdt0r { 
        unsafe {
            read_volatile(self.rdt0r_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RDL0R register."]
    #[inline] pub fn rdl0r_mut(&self) -> *mut Rdl0r { 
        (self.0 + 0x1b8) as *mut Rdl0r
    }

    #[doc="Get the *const pointer for the RDL0R register."]
    #[inline] pub fn rdl0r_ptr(&self) -> *const Rdl0r { 
           self.rdl0r_mut()
    }

    #[doc="Read the RDL0R register."]
    #[inline] pub fn rdl0r(&self) -> Rdl0r { 
        unsafe {
            read_volatile(self.rdl0r_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RDH0R register."]
    #[inline] pub fn rdh0r_mut(&self) -> *mut Rdh0r { 
        (self.0 + 0x1bc) as *mut Rdh0r
    }

    #[doc="Get the *const pointer for the RDH0R register."]
    #[inline] pub fn rdh0r_ptr(&self) -> *const Rdh0r { 
           self.rdh0r_mut()
    }

    #[doc="Read the RDH0R register."]
    #[inline] pub fn rdh0r(&self) -> Rdh0r { 
        unsafe {
            read_volatile(self.rdh0r_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RI1R register."]
    #[inline] pub fn ri1r_mut(&self) -> *mut Ri1r { 
        (self.0 + 0x1c0) as *mut Ri1r
    }

    #[doc="Get the *const pointer for the RI1R register."]
    #[inline] pub fn ri1r_ptr(&self) -> *const Ri1r { 
           self.ri1r_mut()
    }

    #[doc="Read the RI1R register."]
    #[inline] pub fn ri1r(&self) -> Ri1r { 
        unsafe {
            read_volatile(self.ri1r_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RDT1R register."]
    #[inline] pub fn rdt1r_mut(&self) -> *mut Rdt1r { 
        (self.0 + 0x1c4) as *mut Rdt1r
    }

    #[doc="Get the *const pointer for the RDT1R register."]
    #[inline] pub fn rdt1r_ptr(&self) -> *const Rdt1r { 
           self.rdt1r_mut()
    }

    #[doc="Read the RDT1R register."]
    #[inline] pub fn rdt1r(&self) -> Rdt1r { 
        unsafe {
            read_volatile(self.rdt1r_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RDL1R register."]
    #[inline] pub fn rdl1r_mut(&self) -> *mut Rdl1r { 
        (self.0 + 0x1c8) as *mut Rdl1r
    }

    #[doc="Get the *const pointer for the RDL1R register."]
    #[inline] pub fn rdl1r_ptr(&self) -> *const Rdl1r { 
           self.rdl1r_mut()
    }

    #[doc="Read the RDL1R register."]
    #[inline] pub fn rdl1r(&self) -> Rdl1r { 
        unsafe {
            read_volatile(self.rdl1r_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RDH1R register."]
    #[inline] pub fn rdh1r_mut(&self) -> *mut Rdh1r { 
        (self.0 + 0x1cc) as *mut Rdh1r
    }

    #[doc="Get the *const pointer for the RDH1R register."]
    #[inline] pub fn rdh1r_ptr(&self) -> *const Rdh1r { 
           self.rdh1r_mut()
    }

    #[doc="Read the RDH1R register."]
    #[inline] pub fn rdh1r(&self) -> Rdh1r { 
        unsafe {
            read_volatile(self.rdh1r_ptr())
        }
    }

    #[doc="Get the *mut pointer for the FMR register."]
    #[inline] pub fn fmr_mut(&self) -> *mut Fmr { 
        (self.0 + 0x200) as *mut Fmr
    }

    #[doc="Get the *const pointer for the FMR register."]
    #[inline] pub fn fmr_ptr(&self) -> *const Fmr { 
           self.fmr_mut()
    }

    #[doc="Read the FMR register."]
    #[inline] pub fn fmr(&self) -> Fmr { 
        unsafe {
            read_volatile(self.fmr_ptr())
        }
    }

    #[doc="Write the FMR register."]
    #[inline] pub fn set_fmr<F: FnOnce(Fmr) -> Fmr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fmr_mut(), f(Fmr(0)));
        }
        self
    }

    #[doc="Modify the FMR register."]
    #[inline] pub fn with_fmr<F: FnOnce(Fmr) -> Fmr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fmr_mut(), f(self.fmr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FM1R register."]
    #[inline] pub fn fm1r_mut(&self) -> *mut Fm1r { 
        (self.0 + 0x204) as *mut Fm1r
    }

    #[doc="Get the *const pointer for the FM1R register."]
    #[inline] pub fn fm1r_ptr(&self) -> *const Fm1r { 
           self.fm1r_mut()
    }

    #[doc="Read the FM1R register."]
    #[inline] pub fn fm1r(&self) -> Fm1r { 
        unsafe {
            read_volatile(self.fm1r_ptr())
        }
    }

    #[doc="Write the FM1R register."]
    #[inline] pub fn set_fm1r<F: FnOnce(Fm1r) -> Fm1r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fm1r_mut(), f(Fm1r(0)));
        }
        self
    }

    #[doc="Modify the FM1R register."]
    #[inline] pub fn with_fm1r<F: FnOnce(Fm1r) -> Fm1r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fm1r_mut(), f(self.fm1r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FS1R register."]
    #[inline] pub fn fs1r_mut(&self) -> *mut Fs1r { 
        (self.0 + 0x20c) as *mut Fs1r
    }

    #[doc="Get the *const pointer for the FS1R register."]
    #[inline] pub fn fs1r_ptr(&self) -> *const Fs1r { 
           self.fs1r_mut()
    }

    #[doc="Read the FS1R register."]
    #[inline] pub fn fs1r(&self) -> Fs1r { 
        unsafe {
            read_volatile(self.fs1r_ptr())
        }
    }

    #[doc="Write the FS1R register."]
    #[inline] pub fn set_fs1r<F: FnOnce(Fs1r) -> Fs1r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fs1r_mut(), f(Fs1r(0)));
        }
        self
    }

    #[doc="Modify the FS1R register."]
    #[inline] pub fn with_fs1r<F: FnOnce(Fs1r) -> Fs1r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fs1r_mut(), f(self.fs1r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FFA1R register."]
    #[inline] pub fn ffa1r_mut(&self) -> *mut Ffa1r { 
        (self.0 + 0x214) as *mut Ffa1r
    }

    #[doc="Get the *const pointer for the FFA1R register."]
    #[inline] pub fn ffa1r_ptr(&self) -> *const Ffa1r { 
           self.ffa1r_mut()
    }

    #[doc="Read the FFA1R register."]
    #[inline] pub fn ffa1r(&self) -> Ffa1r { 
        unsafe {
            read_volatile(self.ffa1r_ptr())
        }
    }

    #[doc="Write the FFA1R register."]
    #[inline] pub fn set_ffa1r<F: FnOnce(Ffa1r) -> Ffa1r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ffa1r_mut(), f(Ffa1r(0)));
        }
        self
    }

    #[doc="Modify the FFA1R register."]
    #[inline] pub fn with_ffa1r<F: FnOnce(Ffa1r) -> Ffa1r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ffa1r_mut(), f(self.ffa1r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FA1R register."]
    #[inline] pub fn fa1r_mut(&self) -> *mut Fa1r { 
        (self.0 + 0x21c) as *mut Fa1r
    }

    #[doc="Get the *const pointer for the FA1R register."]
    #[inline] pub fn fa1r_ptr(&self) -> *const Fa1r { 
           self.fa1r_mut()
    }

    #[doc="Read the FA1R register."]
    #[inline] pub fn fa1r(&self) -> Fa1r { 
        unsafe {
            read_volatile(self.fa1r_ptr())
        }
    }

    #[doc="Write the FA1R register."]
    #[inline] pub fn set_fa1r<F: FnOnce(Fa1r) -> Fa1r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fa1r_mut(), f(Fa1r(0)));
        }
        self
    }

    #[doc="Modify the FA1R register."]
    #[inline] pub fn with_fa1r<F: FnOnce(Fa1r) -> Fa1r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fa1r_mut(), f(self.fa1r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FR register."]
    #[inline] pub fn fr_mut(&self) -> *mut Fr { 
        (self.0 + 0x240) as *mut Fr
    }

    #[doc="Get the *const pointer for the FR register."]
    #[inline] pub fn fr_ptr(&self) -> *const Fr { 
           self.fr_mut()
    }

    #[doc="Read the FR register."]
    #[inline] pub fn fr(&self) -> Fr { 
        unsafe {
            read_volatile(self.fr_ptr())
        }
    }

    #[doc="Write the FR register."]
    #[inline] pub fn set_fr<F: FnOnce(Fr) -> Fr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fr_mut(), f(Fr(0)));
        }
        self
    }

    #[doc="Modify the FR register."]
    #[inline] pub fn with_fr<F: FnOnce(Fr) -> Fr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fr_mut(), f(self.fr()));
        }
        self
    }

}

#[doc="master control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mcr(pub u32);
impl Mcr {
    #[doc="DBF"]
    #[inline] pub fn dbf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if DBF != 0"]
    #[inline] pub fn test_dbf(&self) -> bool {
        self.dbf() != 0
    }

    #[doc="Sets the DBF field."]
    #[inline] pub fn set_dbf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="RESET"]
    #[inline] pub fn _reset(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if RESET != 0"]
    #[inline] pub fn test_reset(&self) -> bool {
        self._reset() != 0
    }

    #[doc="Sets the RESET field."]
    #[inline] pub fn set_reset<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="TTCM"]
    #[inline] pub fn ttcm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TTCM != 0"]
    #[inline] pub fn test_ttcm(&self) -> bool {
        self.ttcm() != 0
    }

    #[doc="Sets the TTCM field."]
    #[inline] pub fn set_ttcm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="ABOM"]
    #[inline] pub fn abom(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if ABOM != 0"]
    #[inline] pub fn test_abom(&self) -> bool {
        self.abom() != 0
    }

    #[doc="Sets the ABOM field."]
    #[inline] pub fn set_abom<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="AWUM"]
    #[inline] pub fn awum(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if AWUM != 0"]
    #[inline] pub fn test_awum(&self) -> bool {
        self.awum() != 0
    }

    #[doc="Sets the AWUM field."]
    #[inline] pub fn set_awum<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="NART"]
    #[inline] pub fn nart(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NART != 0"]
    #[inline] pub fn test_nart(&self) -> bool {
        self.nart() != 0
    }

    #[doc="Sets the NART field."]
    #[inline] pub fn set_nart<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="RFLM"]
    #[inline] pub fn rflm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RFLM != 0"]
    #[inline] pub fn test_rflm(&self) -> bool {
        self.rflm() != 0
    }

    #[doc="Sets the RFLM field."]
    #[inline] pub fn set_rflm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="TXFP"]
    #[inline] pub fn txfp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TXFP != 0"]
    #[inline] pub fn test_txfp(&self) -> bool {
        self.txfp() != 0
    }

    #[doc="Sets the TXFP field."]
    #[inline] pub fn set_txfp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="SLEEP"]
    #[inline] pub fn sleep(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SLEEP != 0"]
    #[inline] pub fn test_sleep(&self) -> bool {
        self.sleep() != 0
    }

    #[doc="Sets the SLEEP field."]
    #[inline] pub fn set_sleep<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="INRQ"]
    #[inline] pub fn inrq(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if INRQ != 0"]
    #[inline] pub fn test_inrq(&self) -> bool {
        self.inrq() != 0
    }

    #[doc="Sets the INRQ field."]
    #[inline] pub fn set_inrq<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Mcr {
    #[inline]
    fn from(other: u32) -> Self {
         Mcr(other)
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
        if self.dbf() != 0 { try!(write!(f, " dbf"))}
        if self._reset() != 0 { try!(write!(f, " _reset"))}
        if self.ttcm() != 0 { try!(write!(f, " ttcm"))}
        if self.abom() != 0 { try!(write!(f, " abom"))}
        if self.awum() != 0 { try!(write!(f, " awum"))}
        if self.nart() != 0 { try!(write!(f, " nart"))}
        if self.rflm() != 0 { try!(write!(f, " rflm"))}
        if self.txfp() != 0 { try!(write!(f, " txfp"))}
        if self.sleep() != 0 { try!(write!(f, " sleep"))}
        if self.inrq() != 0 { try!(write!(f, " inrq"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="master status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Msr(pub u32);
impl Msr {
    #[doc="RX"]
    #[inline] pub fn rx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if RX != 0"]
    #[inline] pub fn test_rx(&self) -> bool {
        self.rx() != 0
    }

    #[doc="Sets the RX field."]
    #[inline] pub fn set_rx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="SAMP"]
    #[inline] pub fn samp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if SAMP != 0"]
    #[inline] pub fn test_samp(&self) -> bool {
        self.samp() != 0
    }

    #[doc="Sets the SAMP field."]
    #[inline] pub fn set_samp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="RXM"]
    #[inline] pub fn rxm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if RXM != 0"]
    #[inline] pub fn test_rxm(&self) -> bool {
        self.rxm() != 0
    }

    #[doc="Sets the RXM field."]
    #[inline] pub fn set_rxm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="TXM"]
    #[inline] pub fn txm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if TXM != 0"]
    #[inline] pub fn test_txm(&self) -> bool {
        self.txm() != 0
    }

    #[doc="Sets the TXM field."]
    #[inline] pub fn set_txm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="SLAKI"]
    #[inline] pub fn slaki(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if SLAKI != 0"]
    #[inline] pub fn test_slaki(&self) -> bool {
        self.slaki() != 0
    }

    #[doc="Sets the SLAKI field."]
    #[inline] pub fn set_slaki<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="WKUI"]
    #[inline] pub fn wkui(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if WKUI != 0"]
    #[inline] pub fn test_wkui(&self) -> bool {
        self.wkui() != 0
    }

    #[doc="Sets the WKUI field."]
    #[inline] pub fn set_wkui<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="ERRI"]
    #[inline] pub fn erri(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ERRI != 0"]
    #[inline] pub fn test_erri(&self) -> bool {
        self.erri() != 0
    }

    #[doc="Sets the ERRI field."]
    #[inline] pub fn set_erri<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="SLAK"]
    #[inline] pub fn slak(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SLAK != 0"]
    #[inline] pub fn test_slak(&self) -> bool {
        self.slak() != 0
    }

    #[doc="Sets the SLAK field."]
    #[inline] pub fn set_slak<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="INAK"]
    #[inline] pub fn inak(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if INAK != 0"]
    #[inline] pub fn test_inak(&self) -> bool {
        self.inak() != 0
    }

    #[doc="Sets the INAK field."]
    #[inline] pub fn set_inak<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Msr {
    #[inline]
    fn from(other: u32) -> Self {
         Msr(other)
    }
}

impl ::core::fmt::Display for Msr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Msr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rx() != 0 { try!(write!(f, " rx"))}
        if self.samp() != 0 { try!(write!(f, " samp"))}
        if self.rxm() != 0 { try!(write!(f, " rxm"))}
        if self.txm() != 0 { try!(write!(f, " txm"))}
        if self.slaki() != 0 { try!(write!(f, " slaki"))}
        if self.wkui() != 0 { try!(write!(f, " wkui"))}
        if self.erri() != 0 { try!(write!(f, " erri"))}
        if self.slak() != 0 { try!(write!(f, " slak"))}
        if self.inak() != 0 { try!(write!(f, " inak"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="transmit status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tsr(pub u32);
impl Tsr {
    #[doc="Lowest priority flag for mailbox 2"]
    #[inline] pub fn low2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOW2 != 0"]
    #[inline] pub fn test_low2(&self) -> bool {
        self.low2() != 0
    }

    #[doc="Sets the LOW2 field."]
    #[inline] pub fn set_low2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Lowest priority flag for mailbox 1"]
    #[inline] pub fn low1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if LOW1 != 0"]
    #[inline] pub fn test_low1(&self) -> bool {
        self.low1() != 0
    }

    #[doc="Sets the LOW1 field."]
    #[inline] pub fn set_low1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Lowest priority flag for mailbox 0"]
    #[inline] pub fn low0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if LOW0 != 0"]
    #[inline] pub fn test_low0(&self) -> bool {
        self.low0() != 0
    }

    #[doc="Sets the LOW0 field."]
    #[inline] pub fn set_low0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Lowest priority flag for mailbox 2"]
    #[inline] pub fn tme2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if TME2 != 0"]
    #[inline] pub fn test_tme2(&self) -> bool {
        self.tme2() != 0
    }

    #[doc="Sets the TME2 field."]
    #[inline] pub fn set_tme2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Lowest priority flag for mailbox 1"]
    #[inline] pub fn tme1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if TME1 != 0"]
    #[inline] pub fn test_tme1(&self) -> bool {
        self.tme1() != 0
    }

    #[doc="Sets the TME1 field."]
    #[inline] pub fn set_tme1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Lowest priority flag for mailbox 0"]
    #[inline] pub fn tme0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if TME0 != 0"]
    #[inline] pub fn test_tme0(&self) -> bool {
        self.tme0() != 0
    }

    #[doc="Sets the TME0 field."]
    #[inline] pub fn set_tme0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="CODE"]
    #[inline] pub fn code(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3) as u8) } // [25:24]
    }

    #[doc="Returns true if CODE != 0"]
    #[inline] pub fn test_code(&self) -> bool {
        self.code() != 0
    }

    #[doc="Sets the CODE field."]
    #[inline] pub fn set_code<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="ABRQ2"]
    #[inline] pub fn abrq2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if ABRQ2 != 0"]
    #[inline] pub fn test_abrq2(&self) -> bool {
        self.abrq2() != 0
    }

    #[doc="Sets the ABRQ2 field."]
    #[inline] pub fn set_abrq2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="TERR2"]
    #[inline] pub fn terr2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if TERR2 != 0"]
    #[inline] pub fn test_terr2(&self) -> bool {
        self.terr2() != 0
    }

    #[doc="Sets the TERR2 field."]
    #[inline] pub fn set_terr2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="ALST2"]
    #[inline] pub fn alst2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if ALST2 != 0"]
    #[inline] pub fn test_alst2(&self) -> bool {
        self.alst2() != 0
    }

    #[doc="Sets the ALST2 field."]
    #[inline] pub fn set_alst2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="TXOK2"]
    #[inline] pub fn txok2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if TXOK2 != 0"]
    #[inline] pub fn test_txok2(&self) -> bool {
        self.txok2() != 0
    }

    #[doc="Sets the TXOK2 field."]
    #[inline] pub fn set_txok2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="RQCP2"]
    #[inline] pub fn rqcp2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if RQCP2 != 0"]
    #[inline] pub fn test_rqcp2(&self) -> bool {
        self.rqcp2() != 0
    }

    #[doc="Sets the RQCP2 field."]
    #[inline] pub fn set_rqcp2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="ABRQ1"]
    #[inline] pub fn abrq1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if ABRQ1 != 0"]
    #[inline] pub fn test_abrq1(&self) -> bool {
        self.abrq1() != 0
    }

    #[doc="Sets the ABRQ1 field."]
    #[inline] pub fn set_abrq1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="TERR1"]
    #[inline] pub fn terr1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if TERR1 != 0"]
    #[inline] pub fn test_terr1(&self) -> bool {
        self.terr1() != 0
    }

    #[doc="Sets the TERR1 field."]
    #[inline] pub fn set_terr1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="ALST1"]
    #[inline] pub fn alst1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if ALST1 != 0"]
    #[inline] pub fn test_alst1(&self) -> bool {
        self.alst1() != 0
    }

    #[doc="Sets the ALST1 field."]
    #[inline] pub fn set_alst1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="TXOK1"]
    #[inline] pub fn txok1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if TXOK1 != 0"]
    #[inline] pub fn test_txok1(&self) -> bool {
        self.txok1() != 0
    }

    #[doc="Sets the TXOK1 field."]
    #[inline] pub fn set_txok1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="RQCP1"]
    #[inline] pub fn rqcp1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if RQCP1 != 0"]
    #[inline] pub fn test_rqcp1(&self) -> bool {
        self.rqcp1() != 0
    }

    #[doc="Sets the RQCP1 field."]
    #[inline] pub fn set_rqcp1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="ABRQ0"]
    #[inline] pub fn abrq0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ABRQ0 != 0"]
    #[inline] pub fn test_abrq0(&self) -> bool {
        self.abrq0() != 0
    }

    #[doc="Sets the ABRQ0 field."]
    #[inline] pub fn set_abrq0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="TERR0"]
    #[inline] pub fn terr0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TERR0 != 0"]
    #[inline] pub fn test_terr0(&self) -> bool {
        self.terr0() != 0
    }

    #[doc="Sets the TERR0 field."]
    #[inline] pub fn set_terr0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="ALST0"]
    #[inline] pub fn alst0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ALST0 != 0"]
    #[inline] pub fn test_alst0(&self) -> bool {
        self.alst0() != 0
    }

    #[doc="Sets the ALST0 field."]
    #[inline] pub fn set_alst0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="TXOK0"]
    #[inline] pub fn txok0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TXOK0 != 0"]
    #[inline] pub fn test_txok0(&self) -> bool {
        self.txok0() != 0
    }

    #[doc="Sets the TXOK0 field."]
    #[inline] pub fn set_txok0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="RQCP0"]
    #[inline] pub fn rqcp0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RQCP0 != 0"]
    #[inline] pub fn test_rqcp0(&self) -> bool {
        self.rqcp0() != 0
    }

    #[doc="Sets the RQCP0 field."]
    #[inline] pub fn set_rqcp0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tsr {
    #[inline]
    fn from(other: u32) -> Self {
         Tsr(other)
    }
}

impl ::core::fmt::Display for Tsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.low2() != 0 { try!(write!(f, " low2"))}
        if self.low1() != 0 { try!(write!(f, " low1"))}
        if self.low0() != 0 { try!(write!(f, " low0"))}
        if self.tme2() != 0 { try!(write!(f, " tme2"))}
        if self.tme1() != 0 { try!(write!(f, " tme1"))}
        if self.tme0() != 0 { try!(write!(f, " tme0"))}
        if self.code() != 0 { try!(write!(f, " code=0x{:x}", self.code()))}
        if self.abrq2() != 0 { try!(write!(f, " abrq2"))}
        if self.terr2() != 0 { try!(write!(f, " terr2"))}
        if self.alst2() != 0 { try!(write!(f, " alst2"))}
        if self.txok2() != 0 { try!(write!(f, " txok2"))}
        if self.rqcp2() != 0 { try!(write!(f, " rqcp2"))}
        if self.abrq1() != 0 { try!(write!(f, " abrq1"))}
        if self.terr1() != 0 { try!(write!(f, " terr1"))}
        if self.alst1() != 0 { try!(write!(f, " alst1"))}
        if self.txok1() != 0 { try!(write!(f, " txok1"))}
        if self.rqcp1() != 0 { try!(write!(f, " rqcp1"))}
        if self.abrq0() != 0 { try!(write!(f, " abrq0"))}
        if self.terr0() != 0 { try!(write!(f, " terr0"))}
        if self.alst0() != 0 { try!(write!(f, " alst0"))}
        if self.txok0() != 0 { try!(write!(f, " txok0"))}
        if self.rqcp0() != 0 { try!(write!(f, " rqcp0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="receive FIFO 0 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rf0r(pub u32);
impl Rf0r {
    #[doc="RFOM0"]
    #[inline] pub fn rfom0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RFOM0 != 0"]
    #[inline] pub fn test_rfom0(&self) -> bool {
        self.rfom0() != 0
    }

    #[doc="Sets the RFOM0 field."]
    #[inline] pub fn set_rfom0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="FOVR0"]
    #[inline] pub fn fovr0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if FOVR0 != 0"]
    #[inline] pub fn test_fovr0(&self) -> bool {
        self.fovr0() != 0
    }

    #[doc="Sets the FOVR0 field."]
    #[inline] pub fn set_fovr0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="FULL0"]
    #[inline] pub fn full0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FULL0 != 0"]
    #[inline] pub fn test_full0(&self) -> bool {
        self.full0() != 0
    }

    #[doc="Sets the FULL0 field."]
    #[inline] pub fn set_full0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="FMP0"]
    #[inline] pub fn fmp0(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if FMP0 != 0"]
    #[inline] pub fn test_fmp0(&self) -> bool {
        self.fmp0() != 0
    }

    #[doc="Sets the FMP0 field."]
    #[inline] pub fn set_fmp0<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rf0r {
    #[inline]
    fn from(other: u32) -> Self {
         Rf0r(other)
    }
}

impl ::core::fmt::Display for Rf0r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rf0r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rfom0() != 0 { try!(write!(f, " rfom0"))}
        if self.fovr0() != 0 { try!(write!(f, " fovr0"))}
        if self.full0() != 0 { try!(write!(f, " full0"))}
        if self.fmp0() != 0 { try!(write!(f, " fmp0=0x{:x}", self.fmp0()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="receive FIFO 1 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rf1r(pub u32);
impl Rf1r {
    #[doc="RFOM1"]
    #[inline] pub fn rfom1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RFOM1 != 0"]
    #[inline] pub fn test_rfom1(&self) -> bool {
        self.rfom1() != 0
    }

    #[doc="Sets the RFOM1 field."]
    #[inline] pub fn set_rfom1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="FOVR1"]
    #[inline] pub fn fovr1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if FOVR1 != 0"]
    #[inline] pub fn test_fovr1(&self) -> bool {
        self.fovr1() != 0
    }

    #[doc="Sets the FOVR1 field."]
    #[inline] pub fn set_fovr1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="FULL1"]
    #[inline] pub fn full1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FULL1 != 0"]
    #[inline] pub fn test_full1(&self) -> bool {
        self.full1() != 0
    }

    #[doc="Sets the FULL1 field."]
    #[inline] pub fn set_full1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="FMP1"]
    #[inline] pub fn fmp1(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if FMP1 != 0"]
    #[inline] pub fn test_fmp1(&self) -> bool {
        self.fmp1() != 0
    }

    #[doc="Sets the FMP1 field."]
    #[inline] pub fn set_fmp1<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rf1r {
    #[inline]
    fn from(other: u32) -> Self {
         Rf1r(other)
    }
}

impl ::core::fmt::Display for Rf1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rf1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rfom1() != 0 { try!(write!(f, " rfom1"))}
        if self.fovr1() != 0 { try!(write!(f, " fovr1"))}
        if self.full1() != 0 { try!(write!(f, " full1"))}
        if self.fmp1() != 0 { try!(write!(f, " fmp1=0x{:x}", self.fmp1()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="interrupt enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ier(pub u32);
impl Ier {
    #[doc="SLKIE"]
    #[inline] pub fn slkie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if SLKIE != 0"]
    #[inline] pub fn test_slkie(&self) -> bool {
        self.slkie() != 0
    }

    #[doc="Sets the SLKIE field."]
    #[inline] pub fn set_slkie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="WKUIE"]
    #[inline] pub fn wkuie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if WKUIE != 0"]
    #[inline] pub fn test_wkuie(&self) -> bool {
        self.wkuie() != 0
    }

    #[doc="Sets the WKUIE field."]
    #[inline] pub fn set_wkuie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="ERRIE"]
    #[inline] pub fn errie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if ERRIE != 0"]
    #[inline] pub fn test_errie(&self) -> bool {
        self.errie() != 0
    }

    #[doc="Sets the ERRIE field."]
    #[inline] pub fn set_errie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="LECIE"]
    #[inline] pub fn lecie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if LECIE != 0"]
    #[inline] pub fn test_lecie(&self) -> bool {
        self.lecie() != 0
    }

    #[doc="Sets the LECIE field."]
    #[inline] pub fn set_lecie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="BOFIE"]
    #[inline] pub fn bofie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if BOFIE != 0"]
    #[inline] pub fn test_bofie(&self) -> bool {
        self.bofie() != 0
    }

    #[doc="Sets the BOFIE field."]
    #[inline] pub fn set_bofie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="EPVIE"]
    #[inline] pub fn epvie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if EPVIE != 0"]
    #[inline] pub fn test_epvie(&self) -> bool {
        self.epvie() != 0
    }

    #[doc="Sets the EPVIE field."]
    #[inline] pub fn set_epvie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="EWGIE"]
    #[inline] pub fn ewgie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if EWGIE != 0"]
    #[inline] pub fn test_ewgie(&self) -> bool {
        self.ewgie() != 0
    }

    #[doc="Sets the EWGIE field."]
    #[inline] pub fn set_ewgie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="FOVIE1"]
    #[inline] pub fn fovie1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if FOVIE1 != 0"]
    #[inline] pub fn test_fovie1(&self) -> bool {
        self.fovie1() != 0
    }

    #[doc="Sets the FOVIE1 field."]
    #[inline] pub fn set_fovie1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="FFIE1"]
    #[inline] pub fn ffie1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if FFIE1 != 0"]
    #[inline] pub fn test_ffie1(&self) -> bool {
        self.ffie1() != 0
    }

    #[doc="Sets the FFIE1 field."]
    #[inline] pub fn set_ffie1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="FMPIE1"]
    #[inline] pub fn fmpie1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if FMPIE1 != 0"]
    #[inline] pub fn test_fmpie1(&self) -> bool {
        self.fmpie1() != 0
    }

    #[doc="Sets the FMPIE1 field."]
    #[inline] pub fn set_fmpie1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="FOVIE0"]
    #[inline] pub fn fovie0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FOVIE0 != 0"]
    #[inline] pub fn test_fovie0(&self) -> bool {
        self.fovie0() != 0
    }

    #[doc="Sets the FOVIE0 field."]
    #[inline] pub fn set_fovie0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="FFIE0"]
    #[inline] pub fn ffie0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if FFIE0 != 0"]
    #[inline] pub fn test_ffie0(&self) -> bool {
        self.ffie0() != 0
    }

    #[doc="Sets the FFIE0 field."]
    #[inline] pub fn set_ffie0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="FMPIE0"]
    #[inline] pub fn fmpie0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if FMPIE0 != 0"]
    #[inline] pub fn test_fmpie0(&self) -> bool {
        self.fmpie0() != 0
    }

    #[doc="Sets the FMPIE0 field."]
    #[inline] pub fn set_fmpie0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="TMEIE"]
    #[inline] pub fn tmeie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TMEIE != 0"]
    #[inline] pub fn test_tmeie(&self) -> bool {
        self.tmeie() != 0
    }

    #[doc="Sets the TMEIE field."]
    #[inline] pub fn set_tmeie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ier {
    #[inline]
    fn from(other: u32) -> Self {
         Ier(other)
    }
}

impl ::core::fmt::Display for Ier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.slkie() != 0 { try!(write!(f, " slkie"))}
        if self.wkuie() != 0 { try!(write!(f, " wkuie"))}
        if self.errie() != 0 { try!(write!(f, " errie"))}
        if self.lecie() != 0 { try!(write!(f, " lecie"))}
        if self.bofie() != 0 { try!(write!(f, " bofie"))}
        if self.epvie() != 0 { try!(write!(f, " epvie"))}
        if self.ewgie() != 0 { try!(write!(f, " ewgie"))}
        if self.fovie1() != 0 { try!(write!(f, " fovie1"))}
        if self.ffie1() != 0 { try!(write!(f, " ffie1"))}
        if self.fmpie1() != 0 { try!(write!(f, " fmpie1"))}
        if self.fovie0() != 0 { try!(write!(f, " fovie0"))}
        if self.ffie0() != 0 { try!(write!(f, " ffie0"))}
        if self.fmpie0() != 0 { try!(write!(f, " fmpie0"))}
        if self.tmeie() != 0 { try!(write!(f, " tmeie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="interrupt enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Esr(pub u32);
impl Esr {
    #[doc="REC"]
    #[inline] pub fn rec(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if REC != 0"]
    #[inline] pub fn test_rec(&self) -> bool {
        self.rec() != 0
    }

    #[doc="Sets the REC field."]
    #[inline] pub fn set_rec<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="TEC"]
    #[inline] pub fn tec(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if TEC != 0"]
    #[inline] pub fn test_tec(&self) -> bool {
        self.tec() != 0
    }

    #[doc="Sets the TEC field."]
    #[inline] pub fn set_tec<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="LEC"]
    #[inline] pub fn lec(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if LEC != 0"]
    #[inline] pub fn test_lec(&self) -> bool {
        self.lec() != 0
    }

    #[doc="Sets the LEC field."]
    #[inline] pub fn set_lec<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="BOFF"]
    #[inline] pub fn boff(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if BOFF != 0"]
    #[inline] pub fn test_boff(&self) -> bool {
        self.boff() != 0
    }

    #[doc="Sets the BOFF field."]
    #[inline] pub fn set_boff<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="EPVF"]
    #[inline] pub fn epvf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if EPVF != 0"]
    #[inline] pub fn test_epvf(&self) -> bool {
        self.epvf() != 0
    }

    #[doc="Sets the EPVF field."]
    #[inline] pub fn set_epvf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="EWGF"]
    #[inline] pub fn ewgf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EWGF != 0"]
    #[inline] pub fn test_ewgf(&self) -> bool {
        self.ewgf() != 0
    }

    #[doc="Sets the EWGF field."]
    #[inline] pub fn set_ewgf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Esr {
    #[inline]
    fn from(other: u32) -> Self {
         Esr(other)
    }
}

impl ::core::fmt::Display for Esr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Esr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rec() != 0 { try!(write!(f, " rec=0x{:x}", self.rec()))}
        if self.tec() != 0 { try!(write!(f, " tec=0x{:x}", self.tec()))}
        if self.lec() != 0 { try!(write!(f, " lec=0x{:x}", self.lec()))}
        if self.boff() != 0 { try!(write!(f, " boff"))}
        if self.epvf() != 0 { try!(write!(f, " epvf"))}
        if self.ewgf() != 0 { try!(write!(f, " ewgf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="bit timing register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Btr(pub u32);
impl Btr {
    #[doc="SILM"]
    #[inline] pub fn silm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if SILM != 0"]
    #[inline] pub fn test_silm(&self) -> bool {
        self.silm() != 0
    }

    #[doc="Sets the SILM field."]
    #[inline] pub fn set_silm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="LBKM"]
    #[inline] pub fn lbkm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if LBKM != 0"]
    #[inline] pub fn test_lbkm(&self) -> bool {
        self.lbkm() != 0
    }

    #[doc="Sets the LBKM field."]
    #[inline] pub fn set_lbkm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="SJW"]
    #[inline] pub fn sjw(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3) as u8) } // [25:24]
    }

    #[doc="Returns true if SJW != 0"]
    #[inline] pub fn test_sjw(&self) -> bool {
        self.sjw() != 0
    }

    #[doc="Sets the SJW field."]
    #[inline] pub fn set_sjw<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="TS2"]
    #[inline] pub fn ts2(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x7) as u8) } // [22:20]
    }

    #[doc="Returns true if TS2 != 0"]
    #[inline] pub fn test_ts2(&self) -> bool {
        self.ts2() != 0
    }

    #[doc="Sets the TS2 field."]
    #[inline] pub fn set_ts2<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="TS1"]
    #[inline] pub fn ts1(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if TS1 != 0"]
    #[inline] pub fn test_ts1(&self) -> bool {
        self.ts1() != 0
    }

    #[doc="Sets the TS1 field."]
    #[inline] pub fn set_ts1<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="BRP"]
    #[inline] pub fn brp(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="Returns true if BRP != 0"]
    #[inline] pub fn test_brp(&self) -> bool {
        self.brp() != 0
    }

    #[doc="Sets the BRP field."]
    #[inline] pub fn set_brp<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Btr {
    #[inline]
    fn from(other: u32) -> Self {
         Btr(other)
    }
}

impl ::core::fmt::Display for Btr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Btr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.silm() != 0 { try!(write!(f, " silm"))}
        if self.lbkm() != 0 { try!(write!(f, " lbkm"))}
        if self.sjw() != 0 { try!(write!(f, " sjw=0x{:x}", self.sjw()))}
        if self.ts2() != 0 { try!(write!(f, " ts2=0x{:x}", self.ts2()))}
        if self.ts1() != 0 { try!(write!(f, " ts1=0x{:x}", self.ts1()))}
        if self.brp() != 0 { try!(write!(f, " brp=0x{:x}", self.brp()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="TX mailbox identifier register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ti0r(pub u32);
impl Ti0r {
    #[doc="STID"]
    #[inline] pub fn stid(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x7ff) as u16) } // [31:21]
    }

    #[doc="Returns true if STID != 0"]
    #[inline] pub fn test_stid(&self) -> bool {
        self.stid() != 0
    }

    #[doc="Sets the STID field."]
    #[inline] pub fn set_stid<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="EXID"]
    #[inline] pub fn exid(&self) -> bits::U18 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3ffff) as u32) } // [20:3]
    }

    #[doc="Returns true if EXID != 0"]
    #[inline] pub fn test_exid(&self) -> bool {
        self.exid() != 0
    }

    #[doc="Sets the EXID field."]
    #[inline] pub fn set_exid<V: Into<bits::U18>>(mut self, value: V) -> Self {
        let value: bits::U18 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ffff << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="IDE"]
    #[inline] pub fn ide(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if IDE != 0"]
    #[inline] pub fn test_ide(&self) -> bool {
        self.ide() != 0
    }

    #[doc="Sets the IDE field."]
    #[inline] pub fn set_ide<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="RTR"]
    #[inline] pub fn rtr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RTR != 0"]
    #[inline] pub fn test_rtr(&self) -> bool {
        self.rtr() != 0
    }

    #[doc="Sets the RTR field."]
    #[inline] pub fn set_rtr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="TXRQ"]
    #[inline] pub fn txrq(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TXRQ != 0"]
    #[inline] pub fn test_txrq(&self) -> bool {
        self.txrq() != 0
    }

    #[doc="Sets the TXRQ field."]
    #[inline] pub fn set_txrq<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ti0r {
    #[inline]
    fn from(other: u32) -> Self {
         Ti0r(other)
    }
}

impl ::core::fmt::Display for Ti0r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ti0r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.stid() != 0 { try!(write!(f, " stid=0x{:x}", self.stid()))}
        if self.exid() != 0 { try!(write!(f, " exid=0x{:x}", self.exid()))}
        if self.ide() != 0 { try!(write!(f, " ide"))}
        if self.rtr() != 0 { try!(write!(f, " rtr"))}
        if self.txrq() != 0 { try!(write!(f, " txrq"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="mailbox data length control and time stamp register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tdt0r(pub u32);
impl Tdt0r {
    #[doc="TIME"]
    #[inline] pub fn time(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if TIME != 0"]
    #[inline] pub fn test_time(&self) -> bool {
        self.time() != 0
    }

    #[doc="Sets the TIME field."]
    #[inline] pub fn set_time<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="TGT"]
    #[inline] pub fn tgt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if TGT != 0"]
    #[inline] pub fn test_tgt(&self) -> bool {
        self.tgt() != 0
    }

    #[doc="Sets the TGT field."]
    #[inline] pub fn set_tgt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DLC"]
    #[inline] pub fn dlc(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if DLC != 0"]
    #[inline] pub fn test_dlc(&self) -> bool {
        self.dlc() != 0
    }

    #[doc="Sets the DLC field."]
    #[inline] pub fn set_dlc<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tdt0r {
    #[inline]
    fn from(other: u32) -> Self {
         Tdt0r(other)
    }
}

impl ::core::fmt::Display for Tdt0r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tdt0r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.time() != 0 { try!(write!(f, " time=0x{:x}", self.time()))}
        if self.tgt() != 0 { try!(write!(f, " tgt"))}
        if self.dlc() != 0 { try!(write!(f, " dlc=0x{:x}", self.dlc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="mailbox data low register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tdl0r(pub u32);
impl Tdl0r {
    #[doc="DATA3"]
    #[inline] pub fn data3(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if DATA3 != 0"]
    #[inline] pub fn test_data3(&self) -> bool {
        self.data3() != 0
    }

    #[doc="Sets the DATA3 field."]
    #[inline] pub fn set_data3<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="DATA2"]
    #[inline] pub fn data2(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if DATA2 != 0"]
    #[inline] pub fn test_data2(&self) -> bool {
        self.data2() != 0
    }

    #[doc="Sets the DATA2 field."]
    #[inline] pub fn set_data2<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="DATA1"]
    #[inline] pub fn data1(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if DATA1 != 0"]
    #[inline] pub fn test_data1(&self) -> bool {
        self.data1() != 0
    }

    #[doc="Sets the DATA1 field."]
    #[inline] pub fn set_data1<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DATA0"]
    #[inline] pub fn data0(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DATA0 != 0"]
    #[inline] pub fn test_data0(&self) -> bool {
        self.data0() != 0
    }

    #[doc="Sets the DATA0 field."]
    #[inline] pub fn set_data0<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tdl0r {
    #[inline]
    fn from(other: u32) -> Self {
         Tdl0r(other)
    }
}

impl ::core::fmt::Display for Tdl0r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tdl0r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.data3() != 0 { try!(write!(f, " data3=0x{:x}", self.data3()))}
        if self.data2() != 0 { try!(write!(f, " data2=0x{:x}", self.data2()))}
        if self.data1() != 0 { try!(write!(f, " data1=0x{:x}", self.data1()))}
        if self.data0() != 0 { try!(write!(f, " data0=0x{:x}", self.data0()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="mailbox data high register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tdh0r(pub u32);
impl Tdh0r {
    #[doc="DATA7"]
    #[inline] pub fn data7(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if DATA7 != 0"]
    #[inline] pub fn test_data7(&self) -> bool {
        self.data7() != 0
    }

    #[doc="Sets the DATA7 field."]
    #[inline] pub fn set_data7<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="DATA6"]
    #[inline] pub fn data6(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if DATA6 != 0"]
    #[inline] pub fn test_data6(&self) -> bool {
        self.data6() != 0
    }

    #[doc="Sets the DATA6 field."]
    #[inline] pub fn set_data6<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="DATA5"]
    #[inline] pub fn data5(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if DATA5 != 0"]
    #[inline] pub fn test_data5(&self) -> bool {
        self.data5() != 0
    }

    #[doc="Sets the DATA5 field."]
    #[inline] pub fn set_data5<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DATA4"]
    #[inline] pub fn data4(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DATA4 != 0"]
    #[inline] pub fn test_data4(&self) -> bool {
        self.data4() != 0
    }

    #[doc="Sets the DATA4 field."]
    #[inline] pub fn set_data4<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tdh0r {
    #[inline]
    fn from(other: u32) -> Self {
         Tdh0r(other)
    }
}

impl ::core::fmt::Display for Tdh0r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tdh0r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.data7() != 0 { try!(write!(f, " data7=0x{:x}", self.data7()))}
        if self.data6() != 0 { try!(write!(f, " data6=0x{:x}", self.data6()))}
        if self.data5() != 0 { try!(write!(f, " data5=0x{:x}", self.data5()))}
        if self.data4() != 0 { try!(write!(f, " data4=0x{:x}", self.data4()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="mailbox identifier register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ti1r(pub u32);
impl Ti1r {
    #[doc="STID"]
    #[inline] pub fn stid(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x7ff) as u16) } // [31:21]
    }

    #[doc="Returns true if STID != 0"]
    #[inline] pub fn test_stid(&self) -> bool {
        self.stid() != 0
    }

    #[doc="Sets the STID field."]
    #[inline] pub fn set_stid<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="EXID"]
    #[inline] pub fn exid(&self) -> bits::U18 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3ffff) as u32) } // [20:3]
    }

    #[doc="Returns true if EXID != 0"]
    #[inline] pub fn test_exid(&self) -> bool {
        self.exid() != 0
    }

    #[doc="Sets the EXID field."]
    #[inline] pub fn set_exid<V: Into<bits::U18>>(mut self, value: V) -> Self {
        let value: bits::U18 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ffff << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="IDE"]
    #[inline] pub fn ide(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if IDE != 0"]
    #[inline] pub fn test_ide(&self) -> bool {
        self.ide() != 0
    }

    #[doc="Sets the IDE field."]
    #[inline] pub fn set_ide<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="RTR"]
    #[inline] pub fn rtr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RTR != 0"]
    #[inline] pub fn test_rtr(&self) -> bool {
        self.rtr() != 0
    }

    #[doc="Sets the RTR field."]
    #[inline] pub fn set_rtr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="TXRQ"]
    #[inline] pub fn txrq(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TXRQ != 0"]
    #[inline] pub fn test_txrq(&self) -> bool {
        self.txrq() != 0
    }

    #[doc="Sets the TXRQ field."]
    #[inline] pub fn set_txrq<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ti1r {
    #[inline]
    fn from(other: u32) -> Self {
         Ti1r(other)
    }
}

impl ::core::fmt::Display for Ti1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ti1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.stid() != 0 { try!(write!(f, " stid=0x{:x}", self.stid()))}
        if self.exid() != 0 { try!(write!(f, " exid=0x{:x}", self.exid()))}
        if self.ide() != 0 { try!(write!(f, " ide"))}
        if self.rtr() != 0 { try!(write!(f, " rtr"))}
        if self.txrq() != 0 { try!(write!(f, " txrq"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="mailbox data length control and time stamp register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tdt1r(pub u32);
impl Tdt1r {
    #[doc="TIME"]
    #[inline] pub fn time(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if TIME != 0"]
    #[inline] pub fn test_time(&self) -> bool {
        self.time() != 0
    }

    #[doc="Sets the TIME field."]
    #[inline] pub fn set_time<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="TGT"]
    #[inline] pub fn tgt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if TGT != 0"]
    #[inline] pub fn test_tgt(&self) -> bool {
        self.tgt() != 0
    }

    #[doc="Sets the TGT field."]
    #[inline] pub fn set_tgt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DLC"]
    #[inline] pub fn dlc(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if DLC != 0"]
    #[inline] pub fn test_dlc(&self) -> bool {
        self.dlc() != 0
    }

    #[doc="Sets the DLC field."]
    #[inline] pub fn set_dlc<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tdt1r {
    #[inline]
    fn from(other: u32) -> Self {
         Tdt1r(other)
    }
}

impl ::core::fmt::Display for Tdt1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tdt1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.time() != 0 { try!(write!(f, " time=0x{:x}", self.time()))}
        if self.tgt() != 0 { try!(write!(f, " tgt"))}
        if self.dlc() != 0 { try!(write!(f, " dlc=0x{:x}", self.dlc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="mailbox data low register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tdl1r(pub u32);
impl Tdl1r {
    #[doc="DATA3"]
    #[inline] pub fn data3(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if DATA3 != 0"]
    #[inline] pub fn test_data3(&self) -> bool {
        self.data3() != 0
    }

    #[doc="Sets the DATA3 field."]
    #[inline] pub fn set_data3<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="DATA2"]
    #[inline] pub fn data2(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if DATA2 != 0"]
    #[inline] pub fn test_data2(&self) -> bool {
        self.data2() != 0
    }

    #[doc="Sets the DATA2 field."]
    #[inline] pub fn set_data2<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="DATA1"]
    #[inline] pub fn data1(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if DATA1 != 0"]
    #[inline] pub fn test_data1(&self) -> bool {
        self.data1() != 0
    }

    #[doc="Sets the DATA1 field."]
    #[inline] pub fn set_data1<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DATA0"]
    #[inline] pub fn data0(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DATA0 != 0"]
    #[inline] pub fn test_data0(&self) -> bool {
        self.data0() != 0
    }

    #[doc="Sets the DATA0 field."]
    #[inline] pub fn set_data0<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tdl1r {
    #[inline]
    fn from(other: u32) -> Self {
         Tdl1r(other)
    }
}

impl ::core::fmt::Display for Tdl1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tdl1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.data3() != 0 { try!(write!(f, " data3=0x{:x}", self.data3()))}
        if self.data2() != 0 { try!(write!(f, " data2=0x{:x}", self.data2()))}
        if self.data1() != 0 { try!(write!(f, " data1=0x{:x}", self.data1()))}
        if self.data0() != 0 { try!(write!(f, " data0=0x{:x}", self.data0()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="mailbox data high register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tdh1r(pub u32);
impl Tdh1r {
    #[doc="DATA7"]
    #[inline] pub fn data7(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if DATA7 != 0"]
    #[inline] pub fn test_data7(&self) -> bool {
        self.data7() != 0
    }

    #[doc="Sets the DATA7 field."]
    #[inline] pub fn set_data7<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="DATA6"]
    #[inline] pub fn data6(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if DATA6 != 0"]
    #[inline] pub fn test_data6(&self) -> bool {
        self.data6() != 0
    }

    #[doc="Sets the DATA6 field."]
    #[inline] pub fn set_data6<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="DATA5"]
    #[inline] pub fn data5(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if DATA5 != 0"]
    #[inline] pub fn test_data5(&self) -> bool {
        self.data5() != 0
    }

    #[doc="Sets the DATA5 field."]
    #[inline] pub fn set_data5<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DATA4"]
    #[inline] pub fn data4(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DATA4 != 0"]
    #[inline] pub fn test_data4(&self) -> bool {
        self.data4() != 0
    }

    #[doc="Sets the DATA4 field."]
    #[inline] pub fn set_data4<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tdh1r {
    #[inline]
    fn from(other: u32) -> Self {
         Tdh1r(other)
    }
}

impl ::core::fmt::Display for Tdh1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tdh1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.data7() != 0 { try!(write!(f, " data7=0x{:x}", self.data7()))}
        if self.data6() != 0 { try!(write!(f, " data6=0x{:x}", self.data6()))}
        if self.data5() != 0 { try!(write!(f, " data5=0x{:x}", self.data5()))}
        if self.data4() != 0 { try!(write!(f, " data4=0x{:x}", self.data4()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="mailbox identifier register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ti2r(pub u32);
impl Ti2r {
    #[doc="STID"]
    #[inline] pub fn stid(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x7ff) as u16) } // [31:21]
    }

    #[doc="Returns true if STID != 0"]
    #[inline] pub fn test_stid(&self) -> bool {
        self.stid() != 0
    }

    #[doc="Sets the STID field."]
    #[inline] pub fn set_stid<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="EXID"]
    #[inline] pub fn exid(&self) -> bits::U18 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3ffff) as u32) } // [20:3]
    }

    #[doc="Returns true if EXID != 0"]
    #[inline] pub fn test_exid(&self) -> bool {
        self.exid() != 0
    }

    #[doc="Sets the EXID field."]
    #[inline] pub fn set_exid<V: Into<bits::U18>>(mut self, value: V) -> Self {
        let value: bits::U18 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ffff << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="IDE"]
    #[inline] pub fn ide(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if IDE != 0"]
    #[inline] pub fn test_ide(&self) -> bool {
        self.ide() != 0
    }

    #[doc="Sets the IDE field."]
    #[inline] pub fn set_ide<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="RTR"]
    #[inline] pub fn rtr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RTR != 0"]
    #[inline] pub fn test_rtr(&self) -> bool {
        self.rtr() != 0
    }

    #[doc="Sets the RTR field."]
    #[inline] pub fn set_rtr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="TXRQ"]
    #[inline] pub fn txrq(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TXRQ != 0"]
    #[inline] pub fn test_txrq(&self) -> bool {
        self.txrq() != 0
    }

    #[doc="Sets the TXRQ field."]
    #[inline] pub fn set_txrq<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ti2r {
    #[inline]
    fn from(other: u32) -> Self {
         Ti2r(other)
    }
}

impl ::core::fmt::Display for Ti2r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ti2r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.stid() != 0 { try!(write!(f, " stid=0x{:x}", self.stid()))}
        if self.exid() != 0 { try!(write!(f, " exid=0x{:x}", self.exid()))}
        if self.ide() != 0 { try!(write!(f, " ide"))}
        if self.rtr() != 0 { try!(write!(f, " rtr"))}
        if self.txrq() != 0 { try!(write!(f, " txrq"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="mailbox data length control and time stamp register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tdt2r(pub u32);
impl Tdt2r {
    #[doc="TIME"]
    #[inline] pub fn time(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if TIME != 0"]
    #[inline] pub fn test_time(&self) -> bool {
        self.time() != 0
    }

    #[doc="Sets the TIME field."]
    #[inline] pub fn set_time<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="TGT"]
    #[inline] pub fn tgt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if TGT != 0"]
    #[inline] pub fn test_tgt(&self) -> bool {
        self.tgt() != 0
    }

    #[doc="Sets the TGT field."]
    #[inline] pub fn set_tgt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DLC"]
    #[inline] pub fn dlc(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if DLC != 0"]
    #[inline] pub fn test_dlc(&self) -> bool {
        self.dlc() != 0
    }

    #[doc="Sets the DLC field."]
    #[inline] pub fn set_dlc<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tdt2r {
    #[inline]
    fn from(other: u32) -> Self {
         Tdt2r(other)
    }
}

impl ::core::fmt::Display for Tdt2r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tdt2r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.time() != 0 { try!(write!(f, " time=0x{:x}", self.time()))}
        if self.tgt() != 0 { try!(write!(f, " tgt"))}
        if self.dlc() != 0 { try!(write!(f, " dlc=0x{:x}", self.dlc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="mailbox data low register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tdl2r(pub u32);
impl Tdl2r {
    #[doc="DATA3"]
    #[inline] pub fn data3(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if DATA3 != 0"]
    #[inline] pub fn test_data3(&self) -> bool {
        self.data3() != 0
    }

    #[doc="Sets the DATA3 field."]
    #[inline] pub fn set_data3<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="DATA2"]
    #[inline] pub fn data2(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if DATA2 != 0"]
    #[inline] pub fn test_data2(&self) -> bool {
        self.data2() != 0
    }

    #[doc="Sets the DATA2 field."]
    #[inline] pub fn set_data2<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="DATA1"]
    #[inline] pub fn data1(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if DATA1 != 0"]
    #[inline] pub fn test_data1(&self) -> bool {
        self.data1() != 0
    }

    #[doc="Sets the DATA1 field."]
    #[inline] pub fn set_data1<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DATA0"]
    #[inline] pub fn data0(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DATA0 != 0"]
    #[inline] pub fn test_data0(&self) -> bool {
        self.data0() != 0
    }

    #[doc="Sets the DATA0 field."]
    #[inline] pub fn set_data0<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tdl2r {
    #[inline]
    fn from(other: u32) -> Self {
         Tdl2r(other)
    }
}

impl ::core::fmt::Display for Tdl2r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tdl2r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.data3() != 0 { try!(write!(f, " data3=0x{:x}", self.data3()))}
        if self.data2() != 0 { try!(write!(f, " data2=0x{:x}", self.data2()))}
        if self.data1() != 0 { try!(write!(f, " data1=0x{:x}", self.data1()))}
        if self.data0() != 0 { try!(write!(f, " data0=0x{:x}", self.data0()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="mailbox data high register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tdh2r(pub u32);
impl Tdh2r {
    #[doc="DATA7"]
    #[inline] pub fn data7(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if DATA7 != 0"]
    #[inline] pub fn test_data7(&self) -> bool {
        self.data7() != 0
    }

    #[doc="Sets the DATA7 field."]
    #[inline] pub fn set_data7<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="DATA6"]
    #[inline] pub fn data6(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if DATA6 != 0"]
    #[inline] pub fn test_data6(&self) -> bool {
        self.data6() != 0
    }

    #[doc="Sets the DATA6 field."]
    #[inline] pub fn set_data6<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="DATA5"]
    #[inline] pub fn data5(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if DATA5 != 0"]
    #[inline] pub fn test_data5(&self) -> bool {
        self.data5() != 0
    }

    #[doc="Sets the DATA5 field."]
    #[inline] pub fn set_data5<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DATA4"]
    #[inline] pub fn data4(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DATA4 != 0"]
    #[inline] pub fn test_data4(&self) -> bool {
        self.data4() != 0
    }

    #[doc="Sets the DATA4 field."]
    #[inline] pub fn set_data4<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tdh2r {
    #[inline]
    fn from(other: u32) -> Self {
         Tdh2r(other)
    }
}

impl ::core::fmt::Display for Tdh2r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tdh2r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.data7() != 0 { try!(write!(f, " data7=0x{:x}", self.data7()))}
        if self.data6() != 0 { try!(write!(f, " data6=0x{:x}", self.data6()))}
        if self.data5() != 0 { try!(write!(f, " data5=0x{:x}", self.data5()))}
        if self.data4() != 0 { try!(write!(f, " data4=0x{:x}", self.data4()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="receive FIFO mailbox identifier register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ri0r(pub u32);
impl Ri0r {
    #[doc="STID"]
    #[inline] pub fn stid(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x7ff) as u16) } // [31:21]
    }

    #[doc="Returns true if STID != 0"]
    #[inline] pub fn test_stid(&self) -> bool {
        self.stid() != 0
    }

    #[doc="Sets the STID field."]
    #[inline] pub fn set_stid<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="EXID"]
    #[inline] pub fn exid(&self) -> bits::U18 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3ffff) as u32) } // [20:3]
    }

    #[doc="Returns true if EXID != 0"]
    #[inline] pub fn test_exid(&self) -> bool {
        self.exid() != 0
    }

    #[doc="Sets the EXID field."]
    #[inline] pub fn set_exid<V: Into<bits::U18>>(mut self, value: V) -> Self {
        let value: bits::U18 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ffff << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="IDE"]
    #[inline] pub fn ide(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if IDE != 0"]
    #[inline] pub fn test_ide(&self) -> bool {
        self.ide() != 0
    }

    #[doc="Sets the IDE field."]
    #[inline] pub fn set_ide<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="RTR"]
    #[inline] pub fn rtr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RTR != 0"]
    #[inline] pub fn test_rtr(&self) -> bool {
        self.rtr() != 0
    }

    #[doc="Sets the RTR field."]
    #[inline] pub fn set_rtr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Ri0r {
    #[inline]
    fn from(other: u32) -> Self {
         Ri0r(other)
    }
}

impl ::core::fmt::Display for Ri0r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ri0r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.stid() != 0 { try!(write!(f, " stid=0x{:x}", self.stid()))}
        if self.exid() != 0 { try!(write!(f, " exid=0x{:x}", self.exid()))}
        if self.ide() != 0 { try!(write!(f, " ide"))}
        if self.rtr() != 0 { try!(write!(f, " rtr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="mailbox data high register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rdt0r(pub u32);
impl Rdt0r {
    #[doc="TIME"]
    #[inline] pub fn time(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if TIME != 0"]
    #[inline] pub fn test_time(&self) -> bool {
        self.time() != 0
    }

    #[doc="Sets the TIME field."]
    #[inline] pub fn set_time<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="FMI"]
    #[inline] pub fn fmi(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if FMI != 0"]
    #[inline] pub fn test_fmi(&self) -> bool {
        self.fmi() != 0
    }

    #[doc="Sets the FMI field."]
    #[inline] pub fn set_fmi<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DLC"]
    #[inline] pub fn dlc(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if DLC != 0"]
    #[inline] pub fn test_dlc(&self) -> bool {
        self.dlc() != 0
    }

    #[doc="Sets the DLC field."]
    #[inline] pub fn set_dlc<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rdt0r {
    #[inline]
    fn from(other: u32) -> Self {
         Rdt0r(other)
    }
}

impl ::core::fmt::Display for Rdt0r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rdt0r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.time() != 0 { try!(write!(f, " time=0x{:x}", self.time()))}
        if self.fmi() != 0 { try!(write!(f, " fmi=0x{:x}", self.fmi()))}
        if self.dlc() != 0 { try!(write!(f, " dlc=0x{:x}", self.dlc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="mailbox data high register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rdl0r(pub u32);
impl Rdl0r {
    #[doc="DATA3"]
    #[inline] pub fn data3(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if DATA3 != 0"]
    #[inline] pub fn test_data3(&self) -> bool {
        self.data3() != 0
    }

    #[doc="Sets the DATA3 field."]
    #[inline] pub fn set_data3<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="DATA2"]
    #[inline] pub fn data2(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if DATA2 != 0"]
    #[inline] pub fn test_data2(&self) -> bool {
        self.data2() != 0
    }

    #[doc="Sets the DATA2 field."]
    #[inline] pub fn set_data2<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="DATA1"]
    #[inline] pub fn data1(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if DATA1 != 0"]
    #[inline] pub fn test_data1(&self) -> bool {
        self.data1() != 0
    }

    #[doc="Sets the DATA1 field."]
    #[inline] pub fn set_data1<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DATA0"]
    #[inline] pub fn data0(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DATA0 != 0"]
    #[inline] pub fn test_data0(&self) -> bool {
        self.data0() != 0
    }

    #[doc="Sets the DATA0 field."]
    #[inline] pub fn set_data0<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rdl0r {
    #[inline]
    fn from(other: u32) -> Self {
         Rdl0r(other)
    }
}

impl ::core::fmt::Display for Rdl0r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rdl0r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.data3() != 0 { try!(write!(f, " data3=0x{:x}", self.data3()))}
        if self.data2() != 0 { try!(write!(f, " data2=0x{:x}", self.data2()))}
        if self.data1() != 0 { try!(write!(f, " data1=0x{:x}", self.data1()))}
        if self.data0() != 0 { try!(write!(f, " data0=0x{:x}", self.data0()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="receive FIFO mailbox data high register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rdh0r(pub u32);
impl Rdh0r {
    #[doc="DATA7"]
    #[inline] pub fn data7(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if DATA7 != 0"]
    #[inline] pub fn test_data7(&self) -> bool {
        self.data7() != 0
    }

    #[doc="Sets the DATA7 field."]
    #[inline] pub fn set_data7<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="DATA6"]
    #[inline] pub fn data6(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if DATA6 != 0"]
    #[inline] pub fn test_data6(&self) -> bool {
        self.data6() != 0
    }

    #[doc="Sets the DATA6 field."]
    #[inline] pub fn set_data6<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="DATA5"]
    #[inline] pub fn data5(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if DATA5 != 0"]
    #[inline] pub fn test_data5(&self) -> bool {
        self.data5() != 0
    }

    #[doc="Sets the DATA5 field."]
    #[inline] pub fn set_data5<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DATA4"]
    #[inline] pub fn data4(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DATA4 != 0"]
    #[inline] pub fn test_data4(&self) -> bool {
        self.data4() != 0
    }

    #[doc="Sets the DATA4 field."]
    #[inline] pub fn set_data4<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rdh0r {
    #[inline]
    fn from(other: u32) -> Self {
         Rdh0r(other)
    }
}

impl ::core::fmt::Display for Rdh0r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rdh0r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.data7() != 0 { try!(write!(f, " data7=0x{:x}", self.data7()))}
        if self.data6() != 0 { try!(write!(f, " data6=0x{:x}", self.data6()))}
        if self.data5() != 0 { try!(write!(f, " data5=0x{:x}", self.data5()))}
        if self.data4() != 0 { try!(write!(f, " data4=0x{:x}", self.data4()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="mailbox data high register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ri1r(pub u32);
impl Ri1r {
    #[doc="STID"]
    #[inline] pub fn stid(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x7ff) as u16) } // [31:21]
    }

    #[doc="Returns true if STID != 0"]
    #[inline] pub fn test_stid(&self) -> bool {
        self.stid() != 0
    }

    #[doc="Sets the STID field."]
    #[inline] pub fn set_stid<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="EXID"]
    #[inline] pub fn exid(&self) -> bits::U18 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3ffff) as u32) } // [20:3]
    }

    #[doc="Returns true if EXID != 0"]
    #[inline] pub fn test_exid(&self) -> bool {
        self.exid() != 0
    }

    #[doc="Sets the EXID field."]
    #[inline] pub fn set_exid<V: Into<bits::U18>>(mut self, value: V) -> Self {
        let value: bits::U18 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ffff << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="IDE"]
    #[inline] pub fn ide(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if IDE != 0"]
    #[inline] pub fn test_ide(&self) -> bool {
        self.ide() != 0
    }

    #[doc="Sets the IDE field."]
    #[inline] pub fn set_ide<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="RTR"]
    #[inline] pub fn rtr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RTR != 0"]
    #[inline] pub fn test_rtr(&self) -> bool {
        self.rtr() != 0
    }

    #[doc="Sets the RTR field."]
    #[inline] pub fn set_rtr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Ri1r {
    #[inline]
    fn from(other: u32) -> Self {
         Ri1r(other)
    }
}

impl ::core::fmt::Display for Ri1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ri1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.stid() != 0 { try!(write!(f, " stid=0x{:x}", self.stid()))}
        if self.exid() != 0 { try!(write!(f, " exid=0x{:x}", self.exid()))}
        if self.ide() != 0 { try!(write!(f, " ide"))}
        if self.rtr() != 0 { try!(write!(f, " rtr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="mailbox data high register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rdt1r(pub u32);
impl Rdt1r {
    #[doc="TIME"]
    #[inline] pub fn time(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if TIME != 0"]
    #[inline] pub fn test_time(&self) -> bool {
        self.time() != 0
    }

    #[doc="Sets the TIME field."]
    #[inline] pub fn set_time<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="FMI"]
    #[inline] pub fn fmi(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if FMI != 0"]
    #[inline] pub fn test_fmi(&self) -> bool {
        self.fmi() != 0
    }

    #[doc="Sets the FMI field."]
    #[inline] pub fn set_fmi<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DLC"]
    #[inline] pub fn dlc(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if DLC != 0"]
    #[inline] pub fn test_dlc(&self) -> bool {
        self.dlc() != 0
    }

    #[doc="Sets the DLC field."]
    #[inline] pub fn set_dlc<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rdt1r {
    #[inline]
    fn from(other: u32) -> Self {
         Rdt1r(other)
    }
}

impl ::core::fmt::Display for Rdt1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rdt1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.time() != 0 { try!(write!(f, " time=0x{:x}", self.time()))}
        if self.fmi() != 0 { try!(write!(f, " fmi=0x{:x}", self.fmi()))}
        if self.dlc() != 0 { try!(write!(f, " dlc=0x{:x}", self.dlc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="mailbox data high register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rdl1r(pub u32);
impl Rdl1r {
    #[doc="DATA3"]
    #[inline] pub fn data3(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if DATA3 != 0"]
    #[inline] pub fn test_data3(&self) -> bool {
        self.data3() != 0
    }

    #[doc="Sets the DATA3 field."]
    #[inline] pub fn set_data3<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="DATA2"]
    #[inline] pub fn data2(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if DATA2 != 0"]
    #[inline] pub fn test_data2(&self) -> bool {
        self.data2() != 0
    }

    #[doc="Sets the DATA2 field."]
    #[inline] pub fn set_data2<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="DATA1"]
    #[inline] pub fn data1(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if DATA1 != 0"]
    #[inline] pub fn test_data1(&self) -> bool {
        self.data1() != 0
    }

    #[doc="Sets the DATA1 field."]
    #[inline] pub fn set_data1<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DATA0"]
    #[inline] pub fn data0(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DATA0 != 0"]
    #[inline] pub fn test_data0(&self) -> bool {
        self.data0() != 0
    }

    #[doc="Sets the DATA0 field."]
    #[inline] pub fn set_data0<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rdl1r {
    #[inline]
    fn from(other: u32) -> Self {
         Rdl1r(other)
    }
}

impl ::core::fmt::Display for Rdl1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rdl1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.data3() != 0 { try!(write!(f, " data3=0x{:x}", self.data3()))}
        if self.data2() != 0 { try!(write!(f, " data2=0x{:x}", self.data2()))}
        if self.data1() != 0 { try!(write!(f, " data1=0x{:x}", self.data1()))}
        if self.data0() != 0 { try!(write!(f, " data0=0x{:x}", self.data0()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="mailbox data high register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rdh1r(pub u32);
impl Rdh1r {
    #[doc="DATA7"]
    #[inline] pub fn data7(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if DATA7 != 0"]
    #[inline] pub fn test_data7(&self) -> bool {
        self.data7() != 0
    }

    #[doc="Sets the DATA7 field."]
    #[inline] pub fn set_data7<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="DATA6"]
    #[inline] pub fn data6(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if DATA6 != 0"]
    #[inline] pub fn test_data6(&self) -> bool {
        self.data6() != 0
    }

    #[doc="Sets the DATA6 field."]
    #[inline] pub fn set_data6<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="DATA5"]
    #[inline] pub fn data5(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if DATA5 != 0"]
    #[inline] pub fn test_data5(&self) -> bool {
        self.data5() != 0
    }

    #[doc="Sets the DATA5 field."]
    #[inline] pub fn set_data5<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DATA4"]
    #[inline] pub fn data4(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DATA4 != 0"]
    #[inline] pub fn test_data4(&self) -> bool {
        self.data4() != 0
    }

    #[doc="Sets the DATA4 field."]
    #[inline] pub fn set_data4<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rdh1r {
    #[inline]
    fn from(other: u32) -> Self {
         Rdh1r(other)
    }
}

impl ::core::fmt::Display for Rdh1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rdh1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.data7() != 0 { try!(write!(f, " data7=0x{:x}", self.data7()))}
        if self.data6() != 0 { try!(write!(f, " data6=0x{:x}", self.data6()))}
        if self.data5() != 0 { try!(write!(f, " data5=0x{:x}", self.data5()))}
        if self.data4() != 0 { try!(write!(f, " data4=0x{:x}", self.data4()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="filter master register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fmr(pub u32);
impl Fmr {
    #[doc="CAN2SB"]
    #[inline] pub fn can2sb(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3f) as u8) } // [13:8]
    }

    #[doc="Returns true if CAN2SB != 0"]
    #[inline] pub fn test_can2sb(&self) -> bool {
        self.can2sb() != 0
    }

    #[doc="Sets the CAN2SB field."]
    #[inline] pub fn set_can2sb<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="FINIT"]
    #[inline] pub fn finit(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FINIT != 0"]
    #[inline] pub fn test_finit(&self) -> bool {
        self.finit() != 0
    }

    #[doc="Sets the FINIT field."]
    #[inline] pub fn set_finit<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Fmr {
    #[inline]
    fn from(other: u32) -> Self {
         Fmr(other)
    }
}

impl ::core::fmt::Display for Fmr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fmr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.can2sb() != 0 { try!(write!(f, " can2sb=0x{:x}", self.can2sb()))}
        if self.finit() != 0 { try!(write!(f, " finit"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="filter mode register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fm1r(pub u32);
impl Fm1r {
    #[doc="Filter mode"]
    #[inline] pub fn fbm0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FBM0 != 0"]
    #[inline] pub fn test_fbm0(&self) -> bool {
        self.fbm0() != 0
    }

    #[doc="Sets the FBM0 field."]
    #[inline] pub fn set_fbm0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Filter mode"]
    #[inline] pub fn fbm1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if FBM1 != 0"]
    #[inline] pub fn test_fbm1(&self) -> bool {
        self.fbm1() != 0
    }

    #[doc="Sets the FBM1 field."]
    #[inline] pub fn set_fbm1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Filter mode"]
    #[inline] pub fn fbm2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if FBM2 != 0"]
    #[inline] pub fn test_fbm2(&self) -> bool {
        self.fbm2() != 0
    }

    #[doc="Sets the FBM2 field."]
    #[inline] pub fn set_fbm2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Filter mode"]
    #[inline] pub fn fbm3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FBM3 != 0"]
    #[inline] pub fn test_fbm3(&self) -> bool {
        self.fbm3() != 0
    }

    #[doc="Sets the FBM3 field."]
    #[inline] pub fn set_fbm3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Filter mode"]
    #[inline] pub fn fbm4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if FBM4 != 0"]
    #[inline] pub fn test_fbm4(&self) -> bool {
        self.fbm4() != 0
    }

    #[doc="Sets the FBM4 field."]
    #[inline] pub fn set_fbm4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Filter mode"]
    #[inline] pub fn fbm5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if FBM5 != 0"]
    #[inline] pub fn test_fbm5(&self) -> bool {
        self.fbm5() != 0
    }

    #[doc="Sets the FBM5 field."]
    #[inline] pub fn set_fbm5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Filter mode"]
    #[inline] pub fn fbm6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if FBM6 != 0"]
    #[inline] pub fn test_fbm6(&self) -> bool {
        self.fbm6() != 0
    }

    #[doc="Sets the FBM6 field."]
    #[inline] pub fn set_fbm6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Filter mode"]
    #[inline] pub fn fbm7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if FBM7 != 0"]
    #[inline] pub fn test_fbm7(&self) -> bool {
        self.fbm7() != 0
    }

    #[doc="Sets the FBM7 field."]
    #[inline] pub fn set_fbm7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Filter mode"]
    #[inline] pub fn fbm8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if FBM8 != 0"]
    #[inline] pub fn test_fbm8(&self) -> bool {
        self.fbm8() != 0
    }

    #[doc="Sets the FBM8 field."]
    #[inline] pub fn set_fbm8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Filter mode"]
    #[inline] pub fn fbm9(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FBM9 != 0"]
    #[inline] pub fn test_fbm9(&self) -> bool {
        self.fbm9() != 0
    }

    #[doc="Sets the FBM9 field."]
    #[inline] pub fn set_fbm9<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Filter mode"]
    #[inline] pub fn fbm10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if FBM10 != 0"]
    #[inline] pub fn test_fbm10(&self) -> bool {
        self.fbm10() != 0
    }

    #[doc="Sets the FBM10 field."]
    #[inline] pub fn set_fbm10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Filter mode"]
    #[inline] pub fn fbm11(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if FBM11 != 0"]
    #[inline] pub fn test_fbm11(&self) -> bool {
        self.fbm11() != 0
    }

    #[doc="Sets the FBM11 field."]
    #[inline] pub fn set_fbm11<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Filter mode"]
    #[inline] pub fn fbm12(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if FBM12 != 0"]
    #[inline] pub fn test_fbm12(&self) -> bool {
        self.fbm12() != 0
    }

    #[doc="Sets the FBM12 field."]
    #[inline] pub fn set_fbm12<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Filter mode"]
    #[inline] pub fn fbm13(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if FBM13 != 0"]
    #[inline] pub fn test_fbm13(&self) -> bool {
        self.fbm13() != 0
    }

    #[doc="Sets the FBM13 field."]
    #[inline] pub fn set_fbm13<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Filter mode"]
    #[inline] pub fn fbm14(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if FBM14 != 0"]
    #[inline] pub fn test_fbm14(&self) -> bool {
        self.fbm14() != 0
    }

    #[doc="Sets the FBM14 field."]
    #[inline] pub fn set_fbm14<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Filter mode"]
    #[inline] pub fn fbm15(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if FBM15 != 0"]
    #[inline] pub fn test_fbm15(&self) -> bool {
        self.fbm15() != 0
    }

    #[doc="Sets the FBM15 field."]
    #[inline] pub fn set_fbm15<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Filter mode"]
    #[inline] pub fn fbm16(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if FBM16 != 0"]
    #[inline] pub fn test_fbm16(&self) -> bool {
        self.fbm16() != 0
    }

    #[doc="Sets the FBM16 field."]
    #[inline] pub fn set_fbm16<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Filter mode"]
    #[inline] pub fn fbm17(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if FBM17 != 0"]
    #[inline] pub fn test_fbm17(&self) -> bool {
        self.fbm17() != 0
    }

    #[doc="Sets the FBM17 field."]
    #[inline] pub fn set_fbm17<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Filter mode"]
    #[inline] pub fn fbm18(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if FBM18 != 0"]
    #[inline] pub fn test_fbm18(&self) -> bool {
        self.fbm18() != 0
    }

    #[doc="Sets the FBM18 field."]
    #[inline] pub fn set_fbm18<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Filter mode"]
    #[inline] pub fn fbm19(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if FBM19 != 0"]
    #[inline] pub fn test_fbm19(&self) -> bool {
        self.fbm19() != 0
    }

    #[doc="Sets the FBM19 field."]
    #[inline] pub fn set_fbm19<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Filter mode"]
    #[inline] pub fn fbm20(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if FBM20 != 0"]
    #[inline] pub fn test_fbm20(&self) -> bool {
        self.fbm20() != 0
    }

    #[doc="Sets the FBM20 field."]
    #[inline] pub fn set_fbm20<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Filter mode"]
    #[inline] pub fn fbm21(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if FBM21 != 0"]
    #[inline] pub fn test_fbm21(&self) -> bool {
        self.fbm21() != 0
    }

    #[doc="Sets the FBM21 field."]
    #[inline] pub fn set_fbm21<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Filter mode"]
    #[inline] pub fn fbm22(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if FBM22 != 0"]
    #[inline] pub fn test_fbm22(&self) -> bool {
        self.fbm22() != 0
    }

    #[doc="Sets the FBM22 field."]
    #[inline] pub fn set_fbm22<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Filter mode"]
    #[inline] pub fn fbm23(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if FBM23 != 0"]
    #[inline] pub fn test_fbm23(&self) -> bool {
        self.fbm23() != 0
    }

    #[doc="Sets the FBM23 field."]
    #[inline] pub fn set_fbm23<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Filter mode"]
    #[inline] pub fn fbm24(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if FBM24 != 0"]
    #[inline] pub fn test_fbm24(&self) -> bool {
        self.fbm24() != 0
    }

    #[doc="Sets the FBM24 field."]
    #[inline] pub fn set_fbm24<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Filter mode"]
    #[inline] pub fn fbm25(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if FBM25 != 0"]
    #[inline] pub fn test_fbm25(&self) -> bool {
        self.fbm25() != 0
    }

    #[doc="Sets the FBM25 field."]
    #[inline] pub fn set_fbm25<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Filter mode"]
    #[inline] pub fn fbm26(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if FBM26 != 0"]
    #[inline] pub fn test_fbm26(&self) -> bool {
        self.fbm26() != 0
    }

    #[doc="Sets the FBM26 field."]
    #[inline] pub fn set_fbm26<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Filter mode"]
    #[inline] pub fn fbm27(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if FBM27 != 0"]
    #[inline] pub fn test_fbm27(&self) -> bool {
        self.fbm27() != 0
    }

    #[doc="Sets the FBM27 field."]
    #[inline] pub fn set_fbm27<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

}

impl From<u32> for Fm1r {
    #[inline]
    fn from(other: u32) -> Self {
         Fm1r(other)
    }
}

impl ::core::fmt::Display for Fm1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fm1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fbm0() != 0 { try!(write!(f, " fbm0"))}
        if self.fbm1() != 0 { try!(write!(f, " fbm1"))}
        if self.fbm2() != 0 { try!(write!(f, " fbm2"))}
        if self.fbm3() != 0 { try!(write!(f, " fbm3"))}
        if self.fbm4() != 0 { try!(write!(f, " fbm4"))}
        if self.fbm5() != 0 { try!(write!(f, " fbm5"))}
        if self.fbm6() != 0 { try!(write!(f, " fbm6"))}
        if self.fbm7() != 0 { try!(write!(f, " fbm7"))}
        if self.fbm8() != 0 { try!(write!(f, " fbm8"))}
        if self.fbm9() != 0 { try!(write!(f, " fbm9"))}
        if self.fbm10() != 0 { try!(write!(f, " fbm10"))}
        if self.fbm11() != 0 { try!(write!(f, " fbm11"))}
        if self.fbm12() != 0 { try!(write!(f, " fbm12"))}
        if self.fbm13() != 0 { try!(write!(f, " fbm13"))}
        if self.fbm14() != 0 { try!(write!(f, " fbm14"))}
        if self.fbm15() != 0 { try!(write!(f, " fbm15"))}
        if self.fbm16() != 0 { try!(write!(f, " fbm16"))}
        if self.fbm17() != 0 { try!(write!(f, " fbm17"))}
        if self.fbm18() != 0 { try!(write!(f, " fbm18"))}
        if self.fbm19() != 0 { try!(write!(f, " fbm19"))}
        if self.fbm20() != 0 { try!(write!(f, " fbm20"))}
        if self.fbm21() != 0 { try!(write!(f, " fbm21"))}
        if self.fbm22() != 0 { try!(write!(f, " fbm22"))}
        if self.fbm23() != 0 { try!(write!(f, " fbm23"))}
        if self.fbm24() != 0 { try!(write!(f, " fbm24"))}
        if self.fbm25() != 0 { try!(write!(f, " fbm25"))}
        if self.fbm26() != 0 { try!(write!(f, " fbm26"))}
        if self.fbm27() != 0 { try!(write!(f, " fbm27"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="filter scale register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fs1r(pub u32);
impl Fs1r {
    #[doc="Filter scale configuration"]
    #[inline] pub fn fsc0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FSC0 != 0"]
    #[inline] pub fn test_fsc0(&self) -> bool {
        self.fsc0() != 0
    }

    #[doc="Sets the FSC0 field."]
    #[inline] pub fn set_fsc0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Filter scale configuration"]
    #[inline] pub fn fsc1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if FSC1 != 0"]
    #[inline] pub fn test_fsc1(&self) -> bool {
        self.fsc1() != 0
    }

    #[doc="Sets the FSC1 field."]
    #[inline] pub fn set_fsc1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Filter scale configuration"]
    #[inline] pub fn fsc2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if FSC2 != 0"]
    #[inline] pub fn test_fsc2(&self) -> bool {
        self.fsc2() != 0
    }

    #[doc="Sets the FSC2 field."]
    #[inline] pub fn set_fsc2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Filter scale configuration"]
    #[inline] pub fn fsc3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FSC3 != 0"]
    #[inline] pub fn test_fsc3(&self) -> bool {
        self.fsc3() != 0
    }

    #[doc="Sets the FSC3 field."]
    #[inline] pub fn set_fsc3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Filter scale configuration"]
    #[inline] pub fn fsc4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if FSC4 != 0"]
    #[inline] pub fn test_fsc4(&self) -> bool {
        self.fsc4() != 0
    }

    #[doc="Sets the FSC4 field."]
    #[inline] pub fn set_fsc4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Filter scale configuration"]
    #[inline] pub fn fsc5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if FSC5 != 0"]
    #[inline] pub fn test_fsc5(&self) -> bool {
        self.fsc5() != 0
    }

    #[doc="Sets the FSC5 field."]
    #[inline] pub fn set_fsc5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Filter scale configuration"]
    #[inline] pub fn fsc6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if FSC6 != 0"]
    #[inline] pub fn test_fsc6(&self) -> bool {
        self.fsc6() != 0
    }

    #[doc="Sets the FSC6 field."]
    #[inline] pub fn set_fsc6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Filter scale configuration"]
    #[inline] pub fn fsc7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if FSC7 != 0"]
    #[inline] pub fn test_fsc7(&self) -> bool {
        self.fsc7() != 0
    }

    #[doc="Sets the FSC7 field."]
    #[inline] pub fn set_fsc7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Filter scale configuration"]
    #[inline] pub fn fsc8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if FSC8 != 0"]
    #[inline] pub fn test_fsc8(&self) -> bool {
        self.fsc8() != 0
    }

    #[doc="Sets the FSC8 field."]
    #[inline] pub fn set_fsc8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Filter scale configuration"]
    #[inline] pub fn fsc9(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FSC9 != 0"]
    #[inline] pub fn test_fsc9(&self) -> bool {
        self.fsc9() != 0
    }

    #[doc="Sets the FSC9 field."]
    #[inline] pub fn set_fsc9<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Filter scale configuration"]
    #[inline] pub fn fsc10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if FSC10 != 0"]
    #[inline] pub fn test_fsc10(&self) -> bool {
        self.fsc10() != 0
    }

    #[doc="Sets the FSC10 field."]
    #[inline] pub fn set_fsc10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Filter scale configuration"]
    #[inline] pub fn fsc11(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if FSC11 != 0"]
    #[inline] pub fn test_fsc11(&self) -> bool {
        self.fsc11() != 0
    }

    #[doc="Sets the FSC11 field."]
    #[inline] pub fn set_fsc11<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Filter scale configuration"]
    #[inline] pub fn fsc12(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if FSC12 != 0"]
    #[inline] pub fn test_fsc12(&self) -> bool {
        self.fsc12() != 0
    }

    #[doc="Sets the FSC12 field."]
    #[inline] pub fn set_fsc12<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Filter scale configuration"]
    #[inline] pub fn fsc13(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if FSC13 != 0"]
    #[inline] pub fn test_fsc13(&self) -> bool {
        self.fsc13() != 0
    }

    #[doc="Sets the FSC13 field."]
    #[inline] pub fn set_fsc13<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Filter scale configuration"]
    #[inline] pub fn fsc14(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if FSC14 != 0"]
    #[inline] pub fn test_fsc14(&self) -> bool {
        self.fsc14() != 0
    }

    #[doc="Sets the FSC14 field."]
    #[inline] pub fn set_fsc14<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Filter scale configuration"]
    #[inline] pub fn fsc15(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if FSC15 != 0"]
    #[inline] pub fn test_fsc15(&self) -> bool {
        self.fsc15() != 0
    }

    #[doc="Sets the FSC15 field."]
    #[inline] pub fn set_fsc15<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Filter scale configuration"]
    #[inline] pub fn fsc16(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if FSC16 != 0"]
    #[inline] pub fn test_fsc16(&self) -> bool {
        self.fsc16() != 0
    }

    #[doc="Sets the FSC16 field."]
    #[inline] pub fn set_fsc16<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Filter scale configuration"]
    #[inline] pub fn fsc17(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if FSC17 != 0"]
    #[inline] pub fn test_fsc17(&self) -> bool {
        self.fsc17() != 0
    }

    #[doc="Sets the FSC17 field."]
    #[inline] pub fn set_fsc17<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Filter scale configuration"]
    #[inline] pub fn fsc18(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if FSC18 != 0"]
    #[inline] pub fn test_fsc18(&self) -> bool {
        self.fsc18() != 0
    }

    #[doc="Sets the FSC18 field."]
    #[inline] pub fn set_fsc18<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Filter scale configuration"]
    #[inline] pub fn fsc19(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if FSC19 != 0"]
    #[inline] pub fn test_fsc19(&self) -> bool {
        self.fsc19() != 0
    }

    #[doc="Sets the FSC19 field."]
    #[inline] pub fn set_fsc19<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Filter scale configuration"]
    #[inline] pub fn fsc20(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if FSC20 != 0"]
    #[inline] pub fn test_fsc20(&self) -> bool {
        self.fsc20() != 0
    }

    #[doc="Sets the FSC20 field."]
    #[inline] pub fn set_fsc20<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Filter scale configuration"]
    #[inline] pub fn fsc21(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if FSC21 != 0"]
    #[inline] pub fn test_fsc21(&self) -> bool {
        self.fsc21() != 0
    }

    #[doc="Sets the FSC21 field."]
    #[inline] pub fn set_fsc21<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Filter scale configuration"]
    #[inline] pub fn fsc22(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if FSC22 != 0"]
    #[inline] pub fn test_fsc22(&self) -> bool {
        self.fsc22() != 0
    }

    #[doc="Sets the FSC22 field."]
    #[inline] pub fn set_fsc22<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Filter scale configuration"]
    #[inline] pub fn fsc23(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if FSC23 != 0"]
    #[inline] pub fn test_fsc23(&self) -> bool {
        self.fsc23() != 0
    }

    #[doc="Sets the FSC23 field."]
    #[inline] pub fn set_fsc23<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Filter scale configuration"]
    #[inline] pub fn fsc24(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if FSC24 != 0"]
    #[inline] pub fn test_fsc24(&self) -> bool {
        self.fsc24() != 0
    }

    #[doc="Sets the FSC24 field."]
    #[inline] pub fn set_fsc24<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Filter scale configuration"]
    #[inline] pub fn fsc25(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if FSC25 != 0"]
    #[inline] pub fn test_fsc25(&self) -> bool {
        self.fsc25() != 0
    }

    #[doc="Sets the FSC25 field."]
    #[inline] pub fn set_fsc25<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Filter scale configuration"]
    #[inline] pub fn fsc26(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if FSC26 != 0"]
    #[inline] pub fn test_fsc26(&self) -> bool {
        self.fsc26() != 0
    }

    #[doc="Sets the FSC26 field."]
    #[inline] pub fn set_fsc26<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Filter scale configuration"]
    #[inline] pub fn fsc27(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if FSC27 != 0"]
    #[inline] pub fn test_fsc27(&self) -> bool {
        self.fsc27() != 0
    }

    #[doc="Sets the FSC27 field."]
    #[inline] pub fn set_fsc27<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

}

impl From<u32> for Fs1r {
    #[inline]
    fn from(other: u32) -> Self {
         Fs1r(other)
    }
}

impl ::core::fmt::Display for Fs1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fs1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fsc0() != 0 { try!(write!(f, " fsc0"))}
        if self.fsc1() != 0 { try!(write!(f, " fsc1"))}
        if self.fsc2() != 0 { try!(write!(f, " fsc2"))}
        if self.fsc3() != 0 { try!(write!(f, " fsc3"))}
        if self.fsc4() != 0 { try!(write!(f, " fsc4"))}
        if self.fsc5() != 0 { try!(write!(f, " fsc5"))}
        if self.fsc6() != 0 { try!(write!(f, " fsc6"))}
        if self.fsc7() != 0 { try!(write!(f, " fsc7"))}
        if self.fsc8() != 0 { try!(write!(f, " fsc8"))}
        if self.fsc9() != 0 { try!(write!(f, " fsc9"))}
        if self.fsc10() != 0 { try!(write!(f, " fsc10"))}
        if self.fsc11() != 0 { try!(write!(f, " fsc11"))}
        if self.fsc12() != 0 { try!(write!(f, " fsc12"))}
        if self.fsc13() != 0 { try!(write!(f, " fsc13"))}
        if self.fsc14() != 0 { try!(write!(f, " fsc14"))}
        if self.fsc15() != 0 { try!(write!(f, " fsc15"))}
        if self.fsc16() != 0 { try!(write!(f, " fsc16"))}
        if self.fsc17() != 0 { try!(write!(f, " fsc17"))}
        if self.fsc18() != 0 { try!(write!(f, " fsc18"))}
        if self.fsc19() != 0 { try!(write!(f, " fsc19"))}
        if self.fsc20() != 0 { try!(write!(f, " fsc20"))}
        if self.fsc21() != 0 { try!(write!(f, " fsc21"))}
        if self.fsc22() != 0 { try!(write!(f, " fsc22"))}
        if self.fsc23() != 0 { try!(write!(f, " fsc23"))}
        if self.fsc24() != 0 { try!(write!(f, " fsc24"))}
        if self.fsc25() != 0 { try!(write!(f, " fsc25"))}
        if self.fsc26() != 0 { try!(write!(f, " fsc26"))}
        if self.fsc27() != 0 { try!(write!(f, " fsc27"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="filter FIFO assignment register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ffa1r(pub u32);
impl Ffa1r {
    #[doc="Filter FIFO assignment for filter 0"]
    #[inline] pub fn ffa0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FFA0 != 0"]
    #[inline] pub fn test_ffa0(&self) -> bool {
        self.ffa0() != 0
    }

    #[doc="Sets the FFA0 field."]
    #[inline] pub fn set_ffa0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Filter FIFO assignment for filter 1"]
    #[inline] pub fn ffa1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if FFA1 != 0"]
    #[inline] pub fn test_ffa1(&self) -> bool {
        self.ffa1() != 0
    }

    #[doc="Sets the FFA1 field."]
    #[inline] pub fn set_ffa1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Filter FIFO assignment for filter 2"]
    #[inline] pub fn ffa2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if FFA2 != 0"]
    #[inline] pub fn test_ffa2(&self) -> bool {
        self.ffa2() != 0
    }

    #[doc="Sets the FFA2 field."]
    #[inline] pub fn set_ffa2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Filter FIFO assignment for filter 3"]
    #[inline] pub fn ffa3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FFA3 != 0"]
    #[inline] pub fn test_ffa3(&self) -> bool {
        self.ffa3() != 0
    }

    #[doc="Sets the FFA3 field."]
    #[inline] pub fn set_ffa3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Filter FIFO assignment for filter 4"]
    #[inline] pub fn ffa4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if FFA4 != 0"]
    #[inline] pub fn test_ffa4(&self) -> bool {
        self.ffa4() != 0
    }

    #[doc="Sets the FFA4 field."]
    #[inline] pub fn set_ffa4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Filter FIFO assignment for filter 5"]
    #[inline] pub fn ffa5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if FFA5 != 0"]
    #[inline] pub fn test_ffa5(&self) -> bool {
        self.ffa5() != 0
    }

    #[doc="Sets the FFA5 field."]
    #[inline] pub fn set_ffa5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Filter FIFO assignment for filter 6"]
    #[inline] pub fn ffa6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if FFA6 != 0"]
    #[inline] pub fn test_ffa6(&self) -> bool {
        self.ffa6() != 0
    }

    #[doc="Sets the FFA6 field."]
    #[inline] pub fn set_ffa6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Filter FIFO assignment for filter 7"]
    #[inline] pub fn ffa7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if FFA7 != 0"]
    #[inline] pub fn test_ffa7(&self) -> bool {
        self.ffa7() != 0
    }

    #[doc="Sets the FFA7 field."]
    #[inline] pub fn set_ffa7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Filter FIFO assignment for filter 8"]
    #[inline] pub fn ffa8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if FFA8 != 0"]
    #[inline] pub fn test_ffa8(&self) -> bool {
        self.ffa8() != 0
    }

    #[doc="Sets the FFA8 field."]
    #[inline] pub fn set_ffa8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Filter FIFO assignment for filter 9"]
    #[inline] pub fn ffa9(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FFA9 != 0"]
    #[inline] pub fn test_ffa9(&self) -> bool {
        self.ffa9() != 0
    }

    #[doc="Sets the FFA9 field."]
    #[inline] pub fn set_ffa9<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Filter FIFO assignment for filter 10"]
    #[inline] pub fn ffa10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if FFA10 != 0"]
    #[inline] pub fn test_ffa10(&self) -> bool {
        self.ffa10() != 0
    }

    #[doc="Sets the FFA10 field."]
    #[inline] pub fn set_ffa10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Filter FIFO assignment for filter 11"]
    #[inline] pub fn ffa11(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if FFA11 != 0"]
    #[inline] pub fn test_ffa11(&self) -> bool {
        self.ffa11() != 0
    }

    #[doc="Sets the FFA11 field."]
    #[inline] pub fn set_ffa11<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Filter FIFO assignment for filter 12"]
    #[inline] pub fn ffa12(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if FFA12 != 0"]
    #[inline] pub fn test_ffa12(&self) -> bool {
        self.ffa12() != 0
    }

    #[doc="Sets the FFA12 field."]
    #[inline] pub fn set_ffa12<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Filter FIFO assignment for filter 13"]
    #[inline] pub fn ffa13(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if FFA13 != 0"]
    #[inline] pub fn test_ffa13(&self) -> bool {
        self.ffa13() != 0
    }

    #[doc="Sets the FFA13 field."]
    #[inline] pub fn set_ffa13<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Filter FIFO assignment for filter 14"]
    #[inline] pub fn ffa14(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if FFA14 != 0"]
    #[inline] pub fn test_ffa14(&self) -> bool {
        self.ffa14() != 0
    }

    #[doc="Sets the FFA14 field."]
    #[inline] pub fn set_ffa14<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Filter FIFO assignment for filter 15"]
    #[inline] pub fn ffa15(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if FFA15 != 0"]
    #[inline] pub fn test_ffa15(&self) -> bool {
        self.ffa15() != 0
    }

    #[doc="Sets the FFA15 field."]
    #[inline] pub fn set_ffa15<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Filter FIFO assignment for filter 16"]
    #[inline] pub fn ffa16(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if FFA16 != 0"]
    #[inline] pub fn test_ffa16(&self) -> bool {
        self.ffa16() != 0
    }

    #[doc="Sets the FFA16 field."]
    #[inline] pub fn set_ffa16<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Filter FIFO assignment for filter 17"]
    #[inline] pub fn ffa17(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if FFA17 != 0"]
    #[inline] pub fn test_ffa17(&self) -> bool {
        self.ffa17() != 0
    }

    #[doc="Sets the FFA17 field."]
    #[inline] pub fn set_ffa17<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Filter FIFO assignment for filter 18"]
    #[inline] pub fn ffa18(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if FFA18 != 0"]
    #[inline] pub fn test_ffa18(&self) -> bool {
        self.ffa18() != 0
    }

    #[doc="Sets the FFA18 field."]
    #[inline] pub fn set_ffa18<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Filter FIFO assignment for filter 19"]
    #[inline] pub fn ffa19(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if FFA19 != 0"]
    #[inline] pub fn test_ffa19(&self) -> bool {
        self.ffa19() != 0
    }

    #[doc="Sets the FFA19 field."]
    #[inline] pub fn set_ffa19<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Filter FIFO assignment for filter 20"]
    #[inline] pub fn ffa20(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if FFA20 != 0"]
    #[inline] pub fn test_ffa20(&self) -> bool {
        self.ffa20() != 0
    }

    #[doc="Sets the FFA20 field."]
    #[inline] pub fn set_ffa20<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Filter FIFO assignment for filter 21"]
    #[inline] pub fn ffa21(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if FFA21 != 0"]
    #[inline] pub fn test_ffa21(&self) -> bool {
        self.ffa21() != 0
    }

    #[doc="Sets the FFA21 field."]
    #[inline] pub fn set_ffa21<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Filter FIFO assignment for filter 22"]
    #[inline] pub fn ffa22(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if FFA22 != 0"]
    #[inline] pub fn test_ffa22(&self) -> bool {
        self.ffa22() != 0
    }

    #[doc="Sets the FFA22 field."]
    #[inline] pub fn set_ffa22<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Filter FIFO assignment for filter 23"]
    #[inline] pub fn ffa23(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if FFA23 != 0"]
    #[inline] pub fn test_ffa23(&self) -> bool {
        self.ffa23() != 0
    }

    #[doc="Sets the FFA23 field."]
    #[inline] pub fn set_ffa23<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Filter FIFO assignment for filter 24"]
    #[inline] pub fn ffa24(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if FFA24 != 0"]
    #[inline] pub fn test_ffa24(&self) -> bool {
        self.ffa24() != 0
    }

    #[doc="Sets the FFA24 field."]
    #[inline] pub fn set_ffa24<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Filter FIFO assignment for filter 25"]
    #[inline] pub fn ffa25(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if FFA25 != 0"]
    #[inline] pub fn test_ffa25(&self) -> bool {
        self.ffa25() != 0
    }

    #[doc="Sets the FFA25 field."]
    #[inline] pub fn set_ffa25<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Filter FIFO assignment for filter 26"]
    #[inline] pub fn ffa26(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if FFA26 != 0"]
    #[inline] pub fn test_ffa26(&self) -> bool {
        self.ffa26() != 0
    }

    #[doc="Sets the FFA26 field."]
    #[inline] pub fn set_ffa26<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Filter FIFO assignment for filter 27"]
    #[inline] pub fn ffa27(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if FFA27 != 0"]
    #[inline] pub fn test_ffa27(&self) -> bool {
        self.ffa27() != 0
    }

    #[doc="Sets the FFA27 field."]
    #[inline] pub fn set_ffa27<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

}

impl From<u32> for Ffa1r {
    #[inline]
    fn from(other: u32) -> Self {
         Ffa1r(other)
    }
}

impl ::core::fmt::Display for Ffa1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ffa1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ffa0() != 0 { try!(write!(f, " ffa0"))}
        if self.ffa1() != 0 { try!(write!(f, " ffa1"))}
        if self.ffa2() != 0 { try!(write!(f, " ffa2"))}
        if self.ffa3() != 0 { try!(write!(f, " ffa3"))}
        if self.ffa4() != 0 { try!(write!(f, " ffa4"))}
        if self.ffa5() != 0 { try!(write!(f, " ffa5"))}
        if self.ffa6() != 0 { try!(write!(f, " ffa6"))}
        if self.ffa7() != 0 { try!(write!(f, " ffa7"))}
        if self.ffa8() != 0 { try!(write!(f, " ffa8"))}
        if self.ffa9() != 0 { try!(write!(f, " ffa9"))}
        if self.ffa10() != 0 { try!(write!(f, " ffa10"))}
        if self.ffa11() != 0 { try!(write!(f, " ffa11"))}
        if self.ffa12() != 0 { try!(write!(f, " ffa12"))}
        if self.ffa13() != 0 { try!(write!(f, " ffa13"))}
        if self.ffa14() != 0 { try!(write!(f, " ffa14"))}
        if self.ffa15() != 0 { try!(write!(f, " ffa15"))}
        if self.ffa16() != 0 { try!(write!(f, " ffa16"))}
        if self.ffa17() != 0 { try!(write!(f, " ffa17"))}
        if self.ffa18() != 0 { try!(write!(f, " ffa18"))}
        if self.ffa19() != 0 { try!(write!(f, " ffa19"))}
        if self.ffa20() != 0 { try!(write!(f, " ffa20"))}
        if self.ffa21() != 0 { try!(write!(f, " ffa21"))}
        if self.ffa22() != 0 { try!(write!(f, " ffa22"))}
        if self.ffa23() != 0 { try!(write!(f, " ffa23"))}
        if self.ffa24() != 0 { try!(write!(f, " ffa24"))}
        if self.ffa25() != 0 { try!(write!(f, " ffa25"))}
        if self.ffa26() != 0 { try!(write!(f, " ffa26"))}
        if self.ffa27() != 0 { try!(write!(f, " ffa27"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="filter activation register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fa1r(pub u32);
impl Fa1r {
    #[doc="Filter active"]
    #[inline] pub fn fact0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FACT0 != 0"]
    #[inline] pub fn test_fact0(&self) -> bool {
        self.fact0() != 0
    }

    #[doc="Sets the FACT0 field."]
    #[inline] pub fn set_fact0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Filter active"]
    #[inline] pub fn fact1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if FACT1 != 0"]
    #[inline] pub fn test_fact1(&self) -> bool {
        self.fact1() != 0
    }

    #[doc="Sets the FACT1 field."]
    #[inline] pub fn set_fact1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Filter active"]
    #[inline] pub fn fact2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if FACT2 != 0"]
    #[inline] pub fn test_fact2(&self) -> bool {
        self.fact2() != 0
    }

    #[doc="Sets the FACT2 field."]
    #[inline] pub fn set_fact2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Filter active"]
    #[inline] pub fn fact3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FACT3 != 0"]
    #[inline] pub fn test_fact3(&self) -> bool {
        self.fact3() != 0
    }

    #[doc="Sets the FACT3 field."]
    #[inline] pub fn set_fact3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Filter active"]
    #[inline] pub fn fact4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if FACT4 != 0"]
    #[inline] pub fn test_fact4(&self) -> bool {
        self.fact4() != 0
    }

    #[doc="Sets the FACT4 field."]
    #[inline] pub fn set_fact4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Filter active"]
    #[inline] pub fn fact5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if FACT5 != 0"]
    #[inline] pub fn test_fact5(&self) -> bool {
        self.fact5() != 0
    }

    #[doc="Sets the FACT5 field."]
    #[inline] pub fn set_fact5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Filter active"]
    #[inline] pub fn fact6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if FACT6 != 0"]
    #[inline] pub fn test_fact6(&self) -> bool {
        self.fact6() != 0
    }

    #[doc="Sets the FACT6 field."]
    #[inline] pub fn set_fact6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Filter active"]
    #[inline] pub fn fact7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if FACT7 != 0"]
    #[inline] pub fn test_fact7(&self) -> bool {
        self.fact7() != 0
    }

    #[doc="Sets the FACT7 field."]
    #[inline] pub fn set_fact7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Filter active"]
    #[inline] pub fn fact8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if FACT8 != 0"]
    #[inline] pub fn test_fact8(&self) -> bool {
        self.fact8() != 0
    }

    #[doc="Sets the FACT8 field."]
    #[inline] pub fn set_fact8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Filter active"]
    #[inline] pub fn fact9(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FACT9 != 0"]
    #[inline] pub fn test_fact9(&self) -> bool {
        self.fact9() != 0
    }

    #[doc="Sets the FACT9 field."]
    #[inline] pub fn set_fact9<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Filter active"]
    #[inline] pub fn fact10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if FACT10 != 0"]
    #[inline] pub fn test_fact10(&self) -> bool {
        self.fact10() != 0
    }

    #[doc="Sets the FACT10 field."]
    #[inline] pub fn set_fact10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Filter active"]
    #[inline] pub fn fact11(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if FACT11 != 0"]
    #[inline] pub fn test_fact11(&self) -> bool {
        self.fact11() != 0
    }

    #[doc="Sets the FACT11 field."]
    #[inline] pub fn set_fact11<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Filter active"]
    #[inline] pub fn fact12(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if FACT12 != 0"]
    #[inline] pub fn test_fact12(&self) -> bool {
        self.fact12() != 0
    }

    #[doc="Sets the FACT12 field."]
    #[inline] pub fn set_fact12<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Filter active"]
    #[inline] pub fn fact13(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if FACT13 != 0"]
    #[inline] pub fn test_fact13(&self) -> bool {
        self.fact13() != 0
    }

    #[doc="Sets the FACT13 field."]
    #[inline] pub fn set_fact13<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Filter active"]
    #[inline] pub fn fact14(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if FACT14 != 0"]
    #[inline] pub fn test_fact14(&self) -> bool {
        self.fact14() != 0
    }

    #[doc="Sets the FACT14 field."]
    #[inline] pub fn set_fact14<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Filter active"]
    #[inline] pub fn fact15(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if FACT15 != 0"]
    #[inline] pub fn test_fact15(&self) -> bool {
        self.fact15() != 0
    }

    #[doc="Sets the FACT15 field."]
    #[inline] pub fn set_fact15<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Filter active"]
    #[inline] pub fn fact16(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if FACT16 != 0"]
    #[inline] pub fn test_fact16(&self) -> bool {
        self.fact16() != 0
    }

    #[doc="Sets the FACT16 field."]
    #[inline] pub fn set_fact16<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Filter active"]
    #[inline] pub fn fact17(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if FACT17 != 0"]
    #[inline] pub fn test_fact17(&self) -> bool {
        self.fact17() != 0
    }

    #[doc="Sets the FACT17 field."]
    #[inline] pub fn set_fact17<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Filter active"]
    #[inline] pub fn fact18(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if FACT18 != 0"]
    #[inline] pub fn test_fact18(&self) -> bool {
        self.fact18() != 0
    }

    #[doc="Sets the FACT18 field."]
    #[inline] pub fn set_fact18<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Filter active"]
    #[inline] pub fn fact19(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if FACT19 != 0"]
    #[inline] pub fn test_fact19(&self) -> bool {
        self.fact19() != 0
    }

    #[doc="Sets the FACT19 field."]
    #[inline] pub fn set_fact19<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Filter active"]
    #[inline] pub fn fact20(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if FACT20 != 0"]
    #[inline] pub fn test_fact20(&self) -> bool {
        self.fact20() != 0
    }

    #[doc="Sets the FACT20 field."]
    #[inline] pub fn set_fact20<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Filter active"]
    #[inline] pub fn fact21(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if FACT21 != 0"]
    #[inline] pub fn test_fact21(&self) -> bool {
        self.fact21() != 0
    }

    #[doc="Sets the FACT21 field."]
    #[inline] pub fn set_fact21<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Filter active"]
    #[inline] pub fn fact22(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if FACT22 != 0"]
    #[inline] pub fn test_fact22(&self) -> bool {
        self.fact22() != 0
    }

    #[doc="Sets the FACT22 field."]
    #[inline] pub fn set_fact22<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Filter active"]
    #[inline] pub fn fact23(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if FACT23 != 0"]
    #[inline] pub fn test_fact23(&self) -> bool {
        self.fact23() != 0
    }

    #[doc="Sets the FACT23 field."]
    #[inline] pub fn set_fact23<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Filter active"]
    #[inline] pub fn fact24(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if FACT24 != 0"]
    #[inline] pub fn test_fact24(&self) -> bool {
        self.fact24() != 0
    }

    #[doc="Sets the FACT24 field."]
    #[inline] pub fn set_fact24<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Filter active"]
    #[inline] pub fn fact25(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if FACT25 != 0"]
    #[inline] pub fn test_fact25(&self) -> bool {
        self.fact25() != 0
    }

    #[doc="Sets the FACT25 field."]
    #[inline] pub fn set_fact25<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Filter active"]
    #[inline] pub fn fact26(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if FACT26 != 0"]
    #[inline] pub fn test_fact26(&self) -> bool {
        self.fact26() != 0
    }

    #[doc="Sets the FACT26 field."]
    #[inline] pub fn set_fact26<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Filter active"]
    #[inline] pub fn fact27(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if FACT27 != 0"]
    #[inline] pub fn test_fact27(&self) -> bool {
        self.fact27() != 0
    }

    #[doc="Sets the FACT27 field."]
    #[inline] pub fn set_fact27<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

}

impl From<u32> for Fa1r {
    #[inline]
    fn from(other: u32) -> Self {
         Fa1r(other)
    }
}

impl ::core::fmt::Display for Fa1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fa1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fact0() != 0 { try!(write!(f, " fact0"))}
        if self.fact1() != 0 { try!(write!(f, " fact1"))}
        if self.fact2() != 0 { try!(write!(f, " fact2"))}
        if self.fact3() != 0 { try!(write!(f, " fact3"))}
        if self.fact4() != 0 { try!(write!(f, " fact4"))}
        if self.fact5() != 0 { try!(write!(f, " fact5"))}
        if self.fact6() != 0 { try!(write!(f, " fact6"))}
        if self.fact7() != 0 { try!(write!(f, " fact7"))}
        if self.fact8() != 0 { try!(write!(f, " fact8"))}
        if self.fact9() != 0 { try!(write!(f, " fact9"))}
        if self.fact10() != 0 { try!(write!(f, " fact10"))}
        if self.fact11() != 0 { try!(write!(f, " fact11"))}
        if self.fact12() != 0 { try!(write!(f, " fact12"))}
        if self.fact13() != 0 { try!(write!(f, " fact13"))}
        if self.fact14() != 0 { try!(write!(f, " fact14"))}
        if self.fact15() != 0 { try!(write!(f, " fact15"))}
        if self.fact16() != 0 { try!(write!(f, " fact16"))}
        if self.fact17() != 0 { try!(write!(f, " fact17"))}
        if self.fact18() != 0 { try!(write!(f, " fact18"))}
        if self.fact19() != 0 { try!(write!(f, " fact19"))}
        if self.fact20() != 0 { try!(write!(f, " fact20"))}
        if self.fact21() != 0 { try!(write!(f, " fact21"))}
        if self.fact22() != 0 { try!(write!(f, " fact22"))}
        if self.fact23() != 0 { try!(write!(f, " fact23"))}
        if self.fact24() != 0 { try!(write!(f, " fact24"))}
        if self.fact25() != 0 { try!(write!(f, " fact25"))}
        if self.fact26() != 0 { try!(write!(f, " fact26"))}
        if self.fact27() != 0 { try!(write!(f, " fact27"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Filter Register Registers 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fr(pub u32);
impl Fr {
    #[doc="Filter bit"]
    #[inline] pub fn fb(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if FB != 0"]
    #[inline] pub fn test_fb(&self) -> bool {
        self.fb() != 0
    }

    #[doc="Sets the FB field."]
    #[inline] pub fn set_fb<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Fr {
    #[inline]
    fn from(other: u32) -> Self {
         Fr(other)
    }
}

impl ::core::fmt::Display for Fr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}


