//! Cryptographic processor

#[allow(unused_imports)] use ::bobbin_common::*;

#[doc="Cryptographic processor"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CrypPeriph(pub usize);
impl CrypPeriph {
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

    #[doc="Get the *mut pointer for the SR register."]
    #[inline] pub fn sr_mut(&self) -> *mut Sr { 
        (self.0 + 0x4) as *mut Sr
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

    #[doc="Get the *mut pointer for the DIN register."]
    #[inline] pub fn din_mut(&self) -> *mut Din { 
        (self.0 + 0x8) as *mut Din
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

    #[doc="Write the DIN register."]
    #[inline] pub fn set_din<F: FnOnce(Din) -> Din>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.din_mut(), f(Din(0)));
        }
        self
    }

    #[doc="Modify the DIN register."]
    #[inline] pub fn with_din<F: FnOnce(Din) -> Din>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.din_mut(), f(self.din()));
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

    #[doc="Get the *mut pointer for the DMACR register."]
    #[inline] pub fn dmacr_mut(&self) -> *mut Dmacr { 
        (self.0 + 0x10) as *mut Dmacr
    }

    #[doc="Get the *const pointer for the DMACR register."]
    #[inline] pub fn dmacr_ptr(&self) -> *const Dmacr { 
           self.dmacr_mut()
    }

    #[doc="Read the DMACR register."]
    #[inline] pub fn dmacr(&self) -> Dmacr { 
        unsafe {
            read_volatile(self.dmacr_ptr())
        }
    }

    #[doc="Write the DMACR register."]
    #[inline] pub fn set_dmacr<F: FnOnce(Dmacr) -> Dmacr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dmacr_mut(), f(Dmacr(0)));
        }
        self
    }

    #[doc="Modify the DMACR register."]
    #[inline] pub fn with_dmacr<F: FnOnce(Dmacr) -> Dmacr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dmacr_mut(), f(self.dmacr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IMSCR register."]
    #[inline] pub fn imscr_mut(&self) -> *mut Imscr { 
        (self.0 + 0x14) as *mut Imscr
    }

    #[doc="Get the *const pointer for the IMSCR register."]
    #[inline] pub fn imscr_ptr(&self) -> *const Imscr { 
           self.imscr_mut()
    }

    #[doc="Read the IMSCR register."]
    #[inline] pub fn imscr(&self) -> Imscr { 
        unsafe {
            read_volatile(self.imscr_ptr())
        }
    }

    #[doc="Write the IMSCR register."]
    #[inline] pub fn set_imscr<F: FnOnce(Imscr) -> Imscr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.imscr_mut(), f(Imscr(0)));
        }
        self
    }

    #[doc="Modify the IMSCR register."]
    #[inline] pub fn with_imscr<F: FnOnce(Imscr) -> Imscr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.imscr_mut(), f(self.imscr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the RISR register."]
    #[inline] pub fn risr_mut(&self) -> *mut Risr { 
        (self.0 + 0x18) as *mut Risr
    }

    #[doc="Get the *const pointer for the RISR register."]
    #[inline] pub fn risr_ptr(&self) -> *const Risr { 
           self.risr_mut()
    }

    #[doc="Read the RISR register."]
    #[inline] pub fn risr(&self) -> Risr { 
        unsafe {
            read_volatile(self.risr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the MISR register."]
    #[inline] pub fn misr_mut(&self) -> *mut Misr { 
        (self.0 + 0x1c) as *mut Misr
    }

    #[doc="Get the *const pointer for the MISR register."]
    #[inline] pub fn misr_ptr(&self) -> *const Misr { 
           self.misr_mut()
    }

    #[doc="Read the MISR register."]
    #[inline] pub fn misr(&self) -> Misr { 
        unsafe {
            read_volatile(self.misr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the K0LR register."]
    #[inline] pub fn k0lr_mut(&self) -> *mut K0lr { 
        (self.0 + 0x20) as *mut K0lr
    }

    #[doc="Get the *const pointer for the K0LR register."]
    #[inline] pub fn k0lr_ptr(&self) -> *const K0lr { 
           self.k0lr_mut()
    }

    #[doc="Write the K0LR register."]
    #[inline] pub fn set_k0lr<F: FnOnce(K0lr) -> K0lr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.k0lr_mut(), f(K0lr(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the K0RR register."]
    #[inline] pub fn k0rr_mut(&self) -> *mut K0rr { 
        (self.0 + 0x24) as *mut K0rr
    }

    #[doc="Get the *const pointer for the K0RR register."]
    #[inline] pub fn k0rr_ptr(&self) -> *const K0rr { 
           self.k0rr_mut()
    }

    #[doc="Write the K0RR register."]
    #[inline] pub fn set_k0rr<F: FnOnce(K0rr) -> K0rr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.k0rr_mut(), f(K0rr(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the K1LR register."]
    #[inline] pub fn k1lr_mut(&self) -> *mut K1lr { 
        (self.0 + 0x28) as *mut K1lr
    }

    #[doc="Get the *const pointer for the K1LR register."]
    #[inline] pub fn k1lr_ptr(&self) -> *const K1lr { 
           self.k1lr_mut()
    }

    #[doc="Write the K1LR register."]
    #[inline] pub fn set_k1lr<F: FnOnce(K1lr) -> K1lr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.k1lr_mut(), f(K1lr(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the K1RR register."]
    #[inline] pub fn k1rr_mut(&self) -> *mut K1rr { 
        (self.0 + 0x2c) as *mut K1rr
    }

    #[doc="Get the *const pointer for the K1RR register."]
    #[inline] pub fn k1rr_ptr(&self) -> *const K1rr { 
           self.k1rr_mut()
    }

    #[doc="Write the K1RR register."]
    #[inline] pub fn set_k1rr<F: FnOnce(K1rr) -> K1rr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.k1rr_mut(), f(K1rr(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the K2LR register."]
    #[inline] pub fn k2lr_mut(&self) -> *mut K2lr { 
        (self.0 + 0x30) as *mut K2lr
    }

    #[doc="Get the *const pointer for the K2LR register."]
    #[inline] pub fn k2lr_ptr(&self) -> *const K2lr { 
           self.k2lr_mut()
    }

    #[doc="Write the K2LR register."]
    #[inline] pub fn set_k2lr<F: FnOnce(K2lr) -> K2lr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.k2lr_mut(), f(K2lr(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the K2RR register."]
    #[inline] pub fn k2rr_mut(&self) -> *mut K2rr { 
        (self.0 + 0x34) as *mut K2rr
    }

    #[doc="Get the *const pointer for the K2RR register."]
    #[inline] pub fn k2rr_ptr(&self) -> *const K2rr { 
           self.k2rr_mut()
    }

    #[doc="Write the K2RR register."]
    #[inline] pub fn set_k2rr<F: FnOnce(K2rr) -> K2rr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.k2rr_mut(), f(K2rr(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the K3LR register."]
    #[inline] pub fn k3lr_mut(&self) -> *mut K3lr { 
        (self.0 + 0x38) as *mut K3lr
    }

    #[doc="Get the *const pointer for the K3LR register."]
    #[inline] pub fn k3lr_ptr(&self) -> *const K3lr { 
           self.k3lr_mut()
    }

    #[doc="Write the K3LR register."]
    #[inline] pub fn set_k3lr<F: FnOnce(K3lr) -> K3lr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.k3lr_mut(), f(K3lr(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the K3RR register."]
    #[inline] pub fn k3rr_mut(&self) -> *mut K3rr { 
        (self.0 + 0x3c) as *mut K3rr
    }

    #[doc="Get the *const pointer for the K3RR register."]
    #[inline] pub fn k3rr_ptr(&self) -> *const K3rr { 
           self.k3rr_mut()
    }

    #[doc="Write the K3RR register."]
    #[inline] pub fn set_k3rr<F: FnOnce(K3rr) -> K3rr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.k3rr_mut(), f(K3rr(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the IV0LR register."]
    #[inline] pub fn iv0lr_mut(&self) -> *mut Iv0lr { 
        (self.0 + 0x40) as *mut Iv0lr
    }

    #[doc="Get the *const pointer for the IV0LR register."]
    #[inline] pub fn iv0lr_ptr(&self) -> *const Iv0lr { 
           self.iv0lr_mut()
    }

    #[doc="Read the IV0LR register."]
    #[inline] pub fn iv0lr(&self) -> Iv0lr { 
        unsafe {
            read_volatile(self.iv0lr_ptr())
        }
    }

    #[doc="Write the IV0LR register."]
    #[inline] pub fn set_iv0lr<F: FnOnce(Iv0lr) -> Iv0lr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.iv0lr_mut(), f(Iv0lr(0)));
        }
        self
    }

    #[doc="Modify the IV0LR register."]
    #[inline] pub fn with_iv0lr<F: FnOnce(Iv0lr) -> Iv0lr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.iv0lr_mut(), f(self.iv0lr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IV0RR register."]
    #[inline] pub fn iv0rr_mut(&self) -> *mut Iv0rr { 
        (self.0 + 0x44) as *mut Iv0rr
    }

    #[doc="Get the *const pointer for the IV0RR register."]
    #[inline] pub fn iv0rr_ptr(&self) -> *const Iv0rr { 
           self.iv0rr_mut()
    }

    #[doc="Read the IV0RR register."]
    #[inline] pub fn iv0rr(&self) -> Iv0rr { 
        unsafe {
            read_volatile(self.iv0rr_ptr())
        }
    }

    #[doc="Write the IV0RR register."]
    #[inline] pub fn set_iv0rr<F: FnOnce(Iv0rr) -> Iv0rr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.iv0rr_mut(), f(Iv0rr(0)));
        }
        self
    }

    #[doc="Modify the IV0RR register."]
    #[inline] pub fn with_iv0rr<F: FnOnce(Iv0rr) -> Iv0rr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.iv0rr_mut(), f(self.iv0rr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IV1LR register."]
    #[inline] pub fn iv1lr_mut(&self) -> *mut Iv1lr { 
        (self.0 + 0x48) as *mut Iv1lr
    }

    #[doc="Get the *const pointer for the IV1LR register."]
    #[inline] pub fn iv1lr_ptr(&self) -> *const Iv1lr { 
           self.iv1lr_mut()
    }

    #[doc="Read the IV1LR register."]
    #[inline] pub fn iv1lr(&self) -> Iv1lr { 
        unsafe {
            read_volatile(self.iv1lr_ptr())
        }
    }

    #[doc="Write the IV1LR register."]
    #[inline] pub fn set_iv1lr<F: FnOnce(Iv1lr) -> Iv1lr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.iv1lr_mut(), f(Iv1lr(0)));
        }
        self
    }

    #[doc="Modify the IV1LR register."]
    #[inline] pub fn with_iv1lr<F: FnOnce(Iv1lr) -> Iv1lr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.iv1lr_mut(), f(self.iv1lr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IV1RR register."]
    #[inline] pub fn iv1rr_mut(&self) -> *mut Iv1rr { 
        (self.0 + 0x4c) as *mut Iv1rr
    }

    #[doc="Get the *const pointer for the IV1RR register."]
    #[inline] pub fn iv1rr_ptr(&self) -> *const Iv1rr { 
           self.iv1rr_mut()
    }

    #[doc="Read the IV1RR register."]
    #[inline] pub fn iv1rr(&self) -> Iv1rr { 
        unsafe {
            read_volatile(self.iv1rr_ptr())
        }
    }

    #[doc="Write the IV1RR register."]
    #[inline] pub fn set_iv1rr<F: FnOnce(Iv1rr) -> Iv1rr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.iv1rr_mut(), f(Iv1rr(0)));
        }
        self
    }

    #[doc="Modify the IV1RR register."]
    #[inline] pub fn with_iv1rr<F: FnOnce(Iv1rr) -> Iv1rr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.iv1rr_mut(), f(self.iv1rr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSGCMCCM0R register."]
    #[inline] pub fn csgcmccm0r_mut(&self) -> *mut Csgcmccm0r { 
        (self.0 + 0x50) as *mut Csgcmccm0r
    }

    #[doc="Get the *const pointer for the CSGCMCCM0R register."]
    #[inline] pub fn csgcmccm0r_ptr(&self) -> *const Csgcmccm0r { 
           self.csgcmccm0r_mut()
    }

    #[doc="Read the CSGCMCCM0R register."]
    #[inline] pub fn csgcmccm0r(&self) -> Csgcmccm0r { 
        unsafe {
            read_volatile(self.csgcmccm0r_ptr())
        }
    }

    #[doc="Write the CSGCMCCM0R register."]
    #[inline] pub fn set_csgcmccm0r<F: FnOnce(Csgcmccm0r) -> Csgcmccm0r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csgcmccm0r_mut(), f(Csgcmccm0r(0)));
        }
        self
    }

    #[doc="Modify the CSGCMCCM0R register."]
    #[inline] pub fn with_csgcmccm0r<F: FnOnce(Csgcmccm0r) -> Csgcmccm0r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csgcmccm0r_mut(), f(self.csgcmccm0r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSGCMCCM1R register."]
    #[inline] pub fn csgcmccm1r_mut(&self) -> *mut Csgcmccm1r { 
        (self.0 + 0x54) as *mut Csgcmccm1r
    }

    #[doc="Get the *const pointer for the CSGCMCCM1R register."]
    #[inline] pub fn csgcmccm1r_ptr(&self) -> *const Csgcmccm1r { 
           self.csgcmccm1r_mut()
    }

    #[doc="Read the CSGCMCCM1R register."]
    #[inline] pub fn csgcmccm1r(&self) -> Csgcmccm1r { 
        unsafe {
            read_volatile(self.csgcmccm1r_ptr())
        }
    }

    #[doc="Write the CSGCMCCM1R register."]
    #[inline] pub fn set_csgcmccm1r<F: FnOnce(Csgcmccm1r) -> Csgcmccm1r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csgcmccm1r_mut(), f(Csgcmccm1r(0)));
        }
        self
    }

    #[doc="Modify the CSGCMCCM1R register."]
    #[inline] pub fn with_csgcmccm1r<F: FnOnce(Csgcmccm1r) -> Csgcmccm1r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csgcmccm1r_mut(), f(self.csgcmccm1r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSGCMCCM2R register."]
    #[inline] pub fn csgcmccm2r_mut(&self) -> *mut Csgcmccm2r { 
        (self.0 + 0x58) as *mut Csgcmccm2r
    }

    #[doc="Get the *const pointer for the CSGCMCCM2R register."]
    #[inline] pub fn csgcmccm2r_ptr(&self) -> *const Csgcmccm2r { 
           self.csgcmccm2r_mut()
    }

    #[doc="Read the CSGCMCCM2R register."]
    #[inline] pub fn csgcmccm2r(&self) -> Csgcmccm2r { 
        unsafe {
            read_volatile(self.csgcmccm2r_ptr())
        }
    }

    #[doc="Write the CSGCMCCM2R register."]
    #[inline] pub fn set_csgcmccm2r<F: FnOnce(Csgcmccm2r) -> Csgcmccm2r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csgcmccm2r_mut(), f(Csgcmccm2r(0)));
        }
        self
    }

    #[doc="Modify the CSGCMCCM2R register."]
    #[inline] pub fn with_csgcmccm2r<F: FnOnce(Csgcmccm2r) -> Csgcmccm2r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csgcmccm2r_mut(), f(self.csgcmccm2r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSGCMCCM3R register."]
    #[inline] pub fn csgcmccm3r_mut(&self) -> *mut Csgcmccm3r { 
        (self.0 + 0x5c) as *mut Csgcmccm3r
    }

    #[doc="Get the *const pointer for the CSGCMCCM3R register."]
    #[inline] pub fn csgcmccm3r_ptr(&self) -> *const Csgcmccm3r { 
           self.csgcmccm3r_mut()
    }

    #[doc="Read the CSGCMCCM3R register."]
    #[inline] pub fn csgcmccm3r(&self) -> Csgcmccm3r { 
        unsafe {
            read_volatile(self.csgcmccm3r_ptr())
        }
    }

    #[doc="Write the CSGCMCCM3R register."]
    #[inline] pub fn set_csgcmccm3r<F: FnOnce(Csgcmccm3r) -> Csgcmccm3r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csgcmccm3r_mut(), f(Csgcmccm3r(0)));
        }
        self
    }

    #[doc="Modify the CSGCMCCM3R register."]
    #[inline] pub fn with_csgcmccm3r<F: FnOnce(Csgcmccm3r) -> Csgcmccm3r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csgcmccm3r_mut(), f(self.csgcmccm3r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSGCMCCM4R register."]
    #[inline] pub fn csgcmccm4r_mut(&self) -> *mut Csgcmccm4r { 
        (self.0 + 0x60) as *mut Csgcmccm4r
    }

    #[doc="Get the *const pointer for the CSGCMCCM4R register."]
    #[inline] pub fn csgcmccm4r_ptr(&self) -> *const Csgcmccm4r { 
           self.csgcmccm4r_mut()
    }

    #[doc="Read the CSGCMCCM4R register."]
    #[inline] pub fn csgcmccm4r(&self) -> Csgcmccm4r { 
        unsafe {
            read_volatile(self.csgcmccm4r_ptr())
        }
    }

    #[doc="Write the CSGCMCCM4R register."]
    #[inline] pub fn set_csgcmccm4r<F: FnOnce(Csgcmccm4r) -> Csgcmccm4r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csgcmccm4r_mut(), f(Csgcmccm4r(0)));
        }
        self
    }

    #[doc="Modify the CSGCMCCM4R register."]
    #[inline] pub fn with_csgcmccm4r<F: FnOnce(Csgcmccm4r) -> Csgcmccm4r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csgcmccm4r_mut(), f(self.csgcmccm4r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSGCMCCM5R register."]
    #[inline] pub fn csgcmccm5r_mut(&self) -> *mut Csgcmccm5r { 
        (self.0 + 0x64) as *mut Csgcmccm5r
    }

    #[doc="Get the *const pointer for the CSGCMCCM5R register."]
    #[inline] pub fn csgcmccm5r_ptr(&self) -> *const Csgcmccm5r { 
           self.csgcmccm5r_mut()
    }

    #[doc="Read the CSGCMCCM5R register."]
    #[inline] pub fn csgcmccm5r(&self) -> Csgcmccm5r { 
        unsafe {
            read_volatile(self.csgcmccm5r_ptr())
        }
    }

    #[doc="Write the CSGCMCCM5R register."]
    #[inline] pub fn set_csgcmccm5r<F: FnOnce(Csgcmccm5r) -> Csgcmccm5r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csgcmccm5r_mut(), f(Csgcmccm5r(0)));
        }
        self
    }

    #[doc="Modify the CSGCMCCM5R register."]
    #[inline] pub fn with_csgcmccm5r<F: FnOnce(Csgcmccm5r) -> Csgcmccm5r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csgcmccm5r_mut(), f(self.csgcmccm5r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSGCMCCM6R register."]
    #[inline] pub fn csgcmccm6r_mut(&self) -> *mut Csgcmccm6r { 
        (self.0 + 0x68) as *mut Csgcmccm6r
    }

    #[doc="Get the *const pointer for the CSGCMCCM6R register."]
    #[inline] pub fn csgcmccm6r_ptr(&self) -> *const Csgcmccm6r { 
           self.csgcmccm6r_mut()
    }

    #[doc="Read the CSGCMCCM6R register."]
    #[inline] pub fn csgcmccm6r(&self) -> Csgcmccm6r { 
        unsafe {
            read_volatile(self.csgcmccm6r_ptr())
        }
    }

    #[doc="Write the CSGCMCCM6R register."]
    #[inline] pub fn set_csgcmccm6r<F: FnOnce(Csgcmccm6r) -> Csgcmccm6r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csgcmccm6r_mut(), f(Csgcmccm6r(0)));
        }
        self
    }

    #[doc="Modify the CSGCMCCM6R register."]
    #[inline] pub fn with_csgcmccm6r<F: FnOnce(Csgcmccm6r) -> Csgcmccm6r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csgcmccm6r_mut(), f(self.csgcmccm6r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSGCMCCM7R register."]
    #[inline] pub fn csgcmccm7r_mut(&self) -> *mut Csgcmccm7r { 
        (self.0 + 0x6c) as *mut Csgcmccm7r
    }

    #[doc="Get the *const pointer for the CSGCMCCM7R register."]
    #[inline] pub fn csgcmccm7r_ptr(&self) -> *const Csgcmccm7r { 
           self.csgcmccm7r_mut()
    }

    #[doc="Read the CSGCMCCM7R register."]
    #[inline] pub fn csgcmccm7r(&self) -> Csgcmccm7r { 
        unsafe {
            read_volatile(self.csgcmccm7r_ptr())
        }
    }

    #[doc="Write the CSGCMCCM7R register."]
    #[inline] pub fn set_csgcmccm7r<F: FnOnce(Csgcmccm7r) -> Csgcmccm7r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csgcmccm7r_mut(), f(Csgcmccm7r(0)));
        }
        self
    }

    #[doc="Modify the CSGCMCCM7R register."]
    #[inline] pub fn with_csgcmccm7r<F: FnOnce(Csgcmccm7r) -> Csgcmccm7r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csgcmccm7r_mut(), f(self.csgcmccm7r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSGCM0R register."]
    #[inline] pub fn csgcm0r_mut(&self) -> *mut Csgcm0r { 
        (self.0 + 0x70) as *mut Csgcm0r
    }

    #[doc="Get the *const pointer for the CSGCM0R register."]
    #[inline] pub fn csgcm0r_ptr(&self) -> *const Csgcm0r { 
           self.csgcm0r_mut()
    }

    #[doc="Read the CSGCM0R register."]
    #[inline] pub fn csgcm0r(&self) -> Csgcm0r { 
        unsafe {
            read_volatile(self.csgcm0r_ptr())
        }
    }

    #[doc="Write the CSGCM0R register."]
    #[inline] pub fn set_csgcm0r<F: FnOnce(Csgcm0r) -> Csgcm0r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csgcm0r_mut(), f(Csgcm0r(0)));
        }
        self
    }

    #[doc="Modify the CSGCM0R register."]
    #[inline] pub fn with_csgcm0r<F: FnOnce(Csgcm0r) -> Csgcm0r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csgcm0r_mut(), f(self.csgcm0r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSGCM1R register."]
    #[inline] pub fn csgcm1r_mut(&self) -> *mut Csgcm1r { 
        (self.0 + 0x74) as *mut Csgcm1r
    }

    #[doc="Get the *const pointer for the CSGCM1R register."]
    #[inline] pub fn csgcm1r_ptr(&self) -> *const Csgcm1r { 
           self.csgcm1r_mut()
    }

    #[doc="Read the CSGCM1R register."]
    #[inline] pub fn csgcm1r(&self) -> Csgcm1r { 
        unsafe {
            read_volatile(self.csgcm1r_ptr())
        }
    }

    #[doc="Write the CSGCM1R register."]
    #[inline] pub fn set_csgcm1r<F: FnOnce(Csgcm1r) -> Csgcm1r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csgcm1r_mut(), f(Csgcm1r(0)));
        }
        self
    }

    #[doc="Modify the CSGCM1R register."]
    #[inline] pub fn with_csgcm1r<F: FnOnce(Csgcm1r) -> Csgcm1r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csgcm1r_mut(), f(self.csgcm1r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSGCM2R register."]
    #[inline] pub fn csgcm2r_mut(&self) -> *mut Csgcm2r { 
        (self.0 + 0x78) as *mut Csgcm2r
    }

    #[doc="Get the *const pointer for the CSGCM2R register."]
    #[inline] pub fn csgcm2r_ptr(&self) -> *const Csgcm2r { 
           self.csgcm2r_mut()
    }

    #[doc="Read the CSGCM2R register."]
    #[inline] pub fn csgcm2r(&self) -> Csgcm2r { 
        unsafe {
            read_volatile(self.csgcm2r_ptr())
        }
    }

    #[doc="Write the CSGCM2R register."]
    #[inline] pub fn set_csgcm2r<F: FnOnce(Csgcm2r) -> Csgcm2r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csgcm2r_mut(), f(Csgcm2r(0)));
        }
        self
    }

    #[doc="Modify the CSGCM2R register."]
    #[inline] pub fn with_csgcm2r<F: FnOnce(Csgcm2r) -> Csgcm2r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csgcm2r_mut(), f(self.csgcm2r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSGCM3R register."]
    #[inline] pub fn csgcm3r_mut(&self) -> *mut Csgcm3r { 
        (self.0 + 0x7c) as *mut Csgcm3r
    }

    #[doc="Get the *const pointer for the CSGCM3R register."]
    #[inline] pub fn csgcm3r_ptr(&self) -> *const Csgcm3r { 
           self.csgcm3r_mut()
    }

    #[doc="Read the CSGCM3R register."]
    #[inline] pub fn csgcm3r(&self) -> Csgcm3r { 
        unsafe {
            read_volatile(self.csgcm3r_ptr())
        }
    }

    #[doc="Write the CSGCM3R register."]
    #[inline] pub fn set_csgcm3r<F: FnOnce(Csgcm3r) -> Csgcm3r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csgcm3r_mut(), f(Csgcm3r(0)));
        }
        self
    }

    #[doc="Modify the CSGCM3R register."]
    #[inline] pub fn with_csgcm3r<F: FnOnce(Csgcm3r) -> Csgcm3r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csgcm3r_mut(), f(self.csgcm3r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSGCM4R register."]
    #[inline] pub fn csgcm4r_mut(&self) -> *mut Csgcm4r { 
        (self.0 + 0x80) as *mut Csgcm4r
    }

    #[doc="Get the *const pointer for the CSGCM4R register."]
    #[inline] pub fn csgcm4r_ptr(&self) -> *const Csgcm4r { 
           self.csgcm4r_mut()
    }

    #[doc="Read the CSGCM4R register."]
    #[inline] pub fn csgcm4r(&self) -> Csgcm4r { 
        unsafe {
            read_volatile(self.csgcm4r_ptr())
        }
    }

    #[doc="Write the CSGCM4R register."]
    #[inline] pub fn set_csgcm4r<F: FnOnce(Csgcm4r) -> Csgcm4r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csgcm4r_mut(), f(Csgcm4r(0)));
        }
        self
    }

    #[doc="Modify the CSGCM4R register."]
    #[inline] pub fn with_csgcm4r<F: FnOnce(Csgcm4r) -> Csgcm4r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csgcm4r_mut(), f(self.csgcm4r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSGCM5R register."]
    #[inline] pub fn csgcm5r_mut(&self) -> *mut Csgcm5r { 
        (self.0 + 0x84) as *mut Csgcm5r
    }

    #[doc="Get the *const pointer for the CSGCM5R register."]
    #[inline] pub fn csgcm5r_ptr(&self) -> *const Csgcm5r { 
           self.csgcm5r_mut()
    }

    #[doc="Read the CSGCM5R register."]
    #[inline] pub fn csgcm5r(&self) -> Csgcm5r { 
        unsafe {
            read_volatile(self.csgcm5r_ptr())
        }
    }

    #[doc="Write the CSGCM5R register."]
    #[inline] pub fn set_csgcm5r<F: FnOnce(Csgcm5r) -> Csgcm5r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csgcm5r_mut(), f(Csgcm5r(0)));
        }
        self
    }

    #[doc="Modify the CSGCM5R register."]
    #[inline] pub fn with_csgcm5r<F: FnOnce(Csgcm5r) -> Csgcm5r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csgcm5r_mut(), f(self.csgcm5r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSGCM6R register."]
    #[inline] pub fn csgcm6r_mut(&self) -> *mut Csgcm6r { 
        (self.0 + 0x88) as *mut Csgcm6r
    }

    #[doc="Get the *const pointer for the CSGCM6R register."]
    #[inline] pub fn csgcm6r_ptr(&self) -> *const Csgcm6r { 
           self.csgcm6r_mut()
    }

    #[doc="Read the CSGCM6R register."]
    #[inline] pub fn csgcm6r(&self) -> Csgcm6r { 
        unsafe {
            read_volatile(self.csgcm6r_ptr())
        }
    }

    #[doc="Write the CSGCM6R register."]
    #[inline] pub fn set_csgcm6r<F: FnOnce(Csgcm6r) -> Csgcm6r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csgcm6r_mut(), f(Csgcm6r(0)));
        }
        self
    }

    #[doc="Modify the CSGCM6R register."]
    #[inline] pub fn with_csgcm6r<F: FnOnce(Csgcm6r) -> Csgcm6r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csgcm6r_mut(), f(self.csgcm6r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSGCM7R register."]
    #[inline] pub fn csgcm7r_mut(&self) -> *mut Csgcm7r { 
        (self.0 + 0x8c) as *mut Csgcm7r
    }

    #[doc="Get the *const pointer for the CSGCM7R register."]
    #[inline] pub fn csgcm7r_ptr(&self) -> *const Csgcm7r { 
           self.csgcm7r_mut()
    }

    #[doc="Read the CSGCM7R register."]
    #[inline] pub fn csgcm7r(&self) -> Csgcm7r { 
        unsafe {
            read_volatile(self.csgcm7r_ptr())
        }
    }

    #[doc="Write the CSGCM7R register."]
    #[inline] pub fn set_csgcm7r<F: FnOnce(Csgcm7r) -> Csgcm7r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csgcm7r_mut(), f(Csgcm7r(0)));
        }
        self
    }

    #[doc="Modify the CSGCM7R register."]
    #[inline] pub fn with_csgcm7r<F: FnOnce(Csgcm7r) -> Csgcm7r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csgcm7r_mut(), f(self.csgcm7r()));
        }
        self
    }

}

#[doc="control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="Algorithm direction"]
    #[inline] pub fn algodir(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ALGODIR != 0"]
    #[inline] pub fn test_algodir(&self) -> bool {
        self.algodir() != 0
    }

    #[doc="Sets the ALGODIR field."]
    #[inline] pub fn set_algodir<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Algorithm mode"]
    #[inline] pub fn algomode0(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x7) as u8) } // [5:3]
    }

    #[doc="Returns true if ALGOMODE0 != 0"]
    #[inline] pub fn test_algomode0(&self) -> bool {
        self.algomode0() != 0
    }

    #[doc="Sets the ALGOMODE0 field."]
    #[inline] pub fn set_algomode0<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Data type selection"]
    #[inline] pub fn datatype(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if DATATYPE != 0"]
    #[inline] pub fn test_datatype(&self) -> bool {
        self.datatype() != 0
    }

    #[doc="Sets the DATATYPE field."]
    #[inline] pub fn set_datatype<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Key size selection (AES mode only)"]
    #[inline] pub fn keysize(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if KEYSIZE != 0"]
    #[inline] pub fn test_keysize(&self) -> bool {
        self.keysize() != 0
    }

    #[doc="Sets the KEYSIZE field."]
    #[inline] pub fn set_keysize<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="FIFO flush"]
    #[inline] pub fn fflush(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if FFLUSH != 0"]
    #[inline] pub fn test_fflush(&self) -> bool {
        self.fflush() != 0
    }

    #[doc="Sets the FFLUSH field."]
    #[inline] pub fn set_fflush<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Cryptographic processor enable"]
    #[inline] pub fn crypen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CRYPEN != 0"]
    #[inline] pub fn test_crypen(&self) -> bool {
        self.crypen() != 0
    }

    #[doc="Sets the CRYPEN field."]
    #[inline] pub fn set_crypen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="GCM_CCMPH"]
    #[inline] pub fn gcm_ccmph(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if GCM_CCMPH != 0"]
    #[inline] pub fn test_gcm_ccmph(&self) -> bool {
        self.gcm_ccmph() != 0
    }

    #[doc="Sets the GCM_CCMPH field."]
    #[inline] pub fn set_gcm_ccmph<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="ALGOMODE"]
    #[inline] pub fn algomode3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if ALGOMODE3 != 0"]
    #[inline] pub fn test_algomode3(&self) -> bool {
        self.algomode3() != 0
    }

    #[doc="Sets the ALGOMODE3 field."]
    #[inline] pub fn set_algomode3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
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
        if self.algodir() != 0 { try!(write!(f, " algodir"))}
        if self.algomode0() != 0 { try!(write!(f, " algomode0=0x{:x}", self.algomode0()))}
        if self.datatype() != 0 { try!(write!(f, " datatype=0x{:x}", self.datatype()))}
        if self.keysize() != 0 { try!(write!(f, " keysize=0x{:x}", self.keysize()))}
        if self.fflush() != 0 { try!(write!(f, " fflush"))}
        if self.crypen() != 0 { try!(write!(f, " crypen"))}
        if self.gcm_ccmph() != 0 { try!(write!(f, " gcm_ccmph=0x{:x}", self.gcm_ccmph()))}
        if self.algomode3() != 0 { try!(write!(f, " algomode3"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc="Busy bit"]
    #[inline] pub fn busy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if BUSY != 0"]
    #[inline] pub fn test_busy(&self) -> bool {
        self.busy() != 0
    }

    #[doc="Sets the BUSY field."]
    #[inline] pub fn set_busy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Output FIFO full"]
    #[inline] pub fn offu(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if OFFU != 0"]
    #[inline] pub fn test_offu(&self) -> bool {
        self.offu() != 0
    }

    #[doc="Sets the OFFU field."]
    #[inline] pub fn set_offu<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Output FIFO not empty"]
    #[inline] pub fn ofne(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if OFNE != 0"]
    #[inline] pub fn test_ofne(&self) -> bool {
        self.ofne() != 0
    }

    #[doc="Sets the OFNE field."]
    #[inline] pub fn set_ofne<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Input FIFO not full"]
    #[inline] pub fn ifnf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if IFNF != 0"]
    #[inline] pub fn test_ifnf(&self) -> bool {
        self.ifnf() != 0
    }

    #[doc="Sets the IFNF field."]
    #[inline] pub fn set_ifnf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Input FIFO empty"]
    #[inline] pub fn ifem(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if IFEM != 0"]
    #[inline] pub fn test_ifem(&self) -> bool {
        self.ifem() != 0
    }

    #[doc="Sets the IFEM field."]
    #[inline] pub fn set_ifem<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
        if self.busy() != 0 { try!(write!(f, " busy"))}
        if self.offu() != 0 { try!(write!(f, " offu"))}
        if self.ofne() != 0 { try!(write!(f, " ofne"))}
        if self.ifnf() != 0 { try!(write!(f, " ifnf"))}
        if self.ifem() != 0 { try!(write!(f, " ifem"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="data input register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Din(pub u32);
impl Din {
    #[doc="Data input"]
    #[inline] pub fn datain(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if DATAIN != 0"]
    #[inline] pub fn test_datain(&self) -> bool {
        self.datain() != 0
    }

    #[doc="Sets the DATAIN field."]
    #[inline] pub fn set_datain<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
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
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="data output register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dout(pub u32);
impl Dout {
    #[doc="Data output"]
    #[inline] pub fn dataout(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if DATAOUT != 0"]
    #[inline] pub fn test_dataout(&self) -> bool {
        self.dataout() != 0
    }

    #[doc="Sets the DATAOUT field."]
    #[inline] pub fn set_dataout<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
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
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dmacr(pub u32);
impl Dmacr {
    #[doc="DMA output enable"]
    #[inline] pub fn doen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DOEN != 0"]
    #[inline] pub fn test_doen(&self) -> bool {
        self.doen() != 0
    }

    #[doc="Sets the DOEN field."]
    #[inline] pub fn set_doen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="DMA input enable"]
    #[inline] pub fn dien(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DIEN != 0"]
    #[inline] pub fn test_dien(&self) -> bool {
        self.dien() != 0
    }

    #[doc="Sets the DIEN field."]
    #[inline] pub fn set_dien<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dmacr {
    #[inline]
    fn from(other: u32) -> Self {
         Dmacr(other)
    }
}

impl ::core::fmt::Display for Dmacr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dmacr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.doen() != 0 { try!(write!(f, " doen"))}
        if self.dien() != 0 { try!(write!(f, " dien"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="interrupt mask set/clear register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Imscr(pub u32);
impl Imscr {
    #[doc="Output FIFO service interrupt mask"]
    #[inline] pub fn outim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if OUTIM != 0"]
    #[inline] pub fn test_outim(&self) -> bool {
        self.outim() != 0
    }

    #[doc="Sets the OUTIM field."]
    #[inline] pub fn set_outim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Input FIFO service interrupt mask"]
    #[inline] pub fn inim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if INIM != 0"]
    #[inline] pub fn test_inim(&self) -> bool {
        self.inim() != 0
    }

    #[doc="Sets the INIM field."]
    #[inline] pub fn set_inim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Imscr {
    #[inline]
    fn from(other: u32) -> Self {
         Imscr(other)
    }
}

impl ::core::fmt::Display for Imscr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Imscr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.outim() != 0 { try!(write!(f, " outim"))}
        if self.inim() != 0 { try!(write!(f, " inim"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="raw interrupt status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Risr(pub u32);
impl Risr {
    #[doc="Output FIFO service raw interrupt status"]
    #[inline] pub fn outris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if OUTRIS != 0"]
    #[inline] pub fn test_outris(&self) -> bool {
        self.outris() != 0
    }

    #[doc="Sets the OUTRIS field."]
    #[inline] pub fn set_outris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Input FIFO service raw interrupt status"]
    #[inline] pub fn inris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if INRIS != 0"]
    #[inline] pub fn test_inris(&self) -> bool {
        self.inris() != 0
    }

    #[doc="Sets the INRIS field."]
    #[inline] pub fn set_inris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Risr {
    #[inline]
    fn from(other: u32) -> Self {
         Risr(other)
    }
}

impl ::core::fmt::Display for Risr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Risr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.outris() != 0 { try!(write!(f, " outris"))}
        if self.inris() != 0 { try!(write!(f, " inris"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="masked interrupt status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Misr(pub u32);
impl Misr {
    #[doc="Output FIFO service masked interrupt status"]
    #[inline] pub fn outmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if OUTMIS != 0"]
    #[inline] pub fn test_outmis(&self) -> bool {
        self.outmis() != 0
    }

    #[doc="Sets the OUTMIS field."]
    #[inline] pub fn set_outmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Input FIFO service masked interrupt status"]
    #[inline] pub fn inmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if INMIS != 0"]
    #[inline] pub fn test_inmis(&self) -> bool {
        self.inmis() != 0
    }

    #[doc="Sets the INMIS field."]
    #[inline] pub fn set_inmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Misr {
    #[inline]
    fn from(other: u32) -> Self {
         Misr(other)
    }
}

impl ::core::fmt::Display for Misr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Misr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.outmis() != 0 { try!(write!(f, " outmis"))}
        if self.inmis() != 0 { try!(write!(f, " inmis"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="key registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct K0lr(pub u32);
impl K0lr {
    #[doc="b224"]
    #[inline] pub fn b224(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if b224 != 0"]
    #[inline] pub fn test_b224(&self) -> bool {
        self.b224() != 0
    }

    #[doc="Sets the b224 field."]
    #[inline] pub fn set_b224<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="b225"]
    #[inline] pub fn b225(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if b225 != 0"]
    #[inline] pub fn test_b225(&self) -> bool {
        self.b225() != 0
    }

    #[doc="Sets the b225 field."]
    #[inline] pub fn set_b225<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="b226"]
    #[inline] pub fn b226(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if b226 != 0"]
    #[inline] pub fn test_b226(&self) -> bool {
        self.b226() != 0
    }

    #[doc="Sets the b226 field."]
    #[inline] pub fn set_b226<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="b227"]
    #[inline] pub fn b227(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if b227 != 0"]
    #[inline] pub fn test_b227(&self) -> bool {
        self.b227() != 0
    }

    #[doc="Sets the b227 field."]
    #[inline] pub fn set_b227<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="b228"]
    #[inline] pub fn b228(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if b228 != 0"]
    #[inline] pub fn test_b228(&self) -> bool {
        self.b228() != 0
    }

    #[doc="Sets the b228 field."]
    #[inline] pub fn set_b228<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="b229"]
    #[inline] pub fn b229(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if b229 != 0"]
    #[inline] pub fn test_b229(&self) -> bool {
        self.b229() != 0
    }

    #[doc="Sets the b229 field."]
    #[inline] pub fn set_b229<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="b230"]
    #[inline] pub fn b230(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if b230 != 0"]
    #[inline] pub fn test_b230(&self) -> bool {
        self.b230() != 0
    }

    #[doc="Sets the b230 field."]
    #[inline] pub fn set_b230<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="b231"]
    #[inline] pub fn b231(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if b231 != 0"]
    #[inline] pub fn test_b231(&self) -> bool {
        self.b231() != 0
    }

    #[doc="Sets the b231 field."]
    #[inline] pub fn set_b231<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="b232"]
    #[inline] pub fn b232(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if b232 != 0"]
    #[inline] pub fn test_b232(&self) -> bool {
        self.b232() != 0
    }

    #[doc="Sets the b232 field."]
    #[inline] pub fn set_b232<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="b233"]
    #[inline] pub fn b233(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if b233 != 0"]
    #[inline] pub fn test_b233(&self) -> bool {
        self.b233() != 0
    }

    #[doc="Sets the b233 field."]
    #[inline] pub fn set_b233<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="b234"]
    #[inline] pub fn b234(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if b234 != 0"]
    #[inline] pub fn test_b234(&self) -> bool {
        self.b234() != 0
    }

    #[doc="Sets the b234 field."]
    #[inline] pub fn set_b234<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="b235"]
    #[inline] pub fn b235(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if b235 != 0"]
    #[inline] pub fn test_b235(&self) -> bool {
        self.b235() != 0
    }

    #[doc="Sets the b235 field."]
    #[inline] pub fn set_b235<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="b236"]
    #[inline] pub fn b236(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if b236 != 0"]
    #[inline] pub fn test_b236(&self) -> bool {
        self.b236() != 0
    }

    #[doc="Sets the b236 field."]
    #[inline] pub fn set_b236<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="b237"]
    #[inline] pub fn b237(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if b237 != 0"]
    #[inline] pub fn test_b237(&self) -> bool {
        self.b237() != 0
    }

    #[doc="Sets the b237 field."]
    #[inline] pub fn set_b237<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="b238"]
    #[inline] pub fn b238(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if b238 != 0"]
    #[inline] pub fn test_b238(&self) -> bool {
        self.b238() != 0
    }

    #[doc="Sets the b238 field."]
    #[inline] pub fn set_b238<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="b239"]
    #[inline] pub fn b239(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if b239 != 0"]
    #[inline] pub fn test_b239(&self) -> bool {
        self.b239() != 0
    }

    #[doc="Sets the b239 field."]
    #[inline] pub fn set_b239<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="b240"]
    #[inline] pub fn b240(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if b240 != 0"]
    #[inline] pub fn test_b240(&self) -> bool {
        self.b240() != 0
    }

    #[doc="Sets the b240 field."]
    #[inline] pub fn set_b240<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="b241"]
    #[inline] pub fn b241(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if b241 != 0"]
    #[inline] pub fn test_b241(&self) -> bool {
        self.b241() != 0
    }

    #[doc="Sets the b241 field."]
    #[inline] pub fn set_b241<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="b242"]
    #[inline] pub fn b242(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if b242 != 0"]
    #[inline] pub fn test_b242(&self) -> bool {
        self.b242() != 0
    }

    #[doc="Sets the b242 field."]
    #[inline] pub fn set_b242<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="b243"]
    #[inline] pub fn b243(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if b243 != 0"]
    #[inline] pub fn test_b243(&self) -> bool {
        self.b243() != 0
    }

    #[doc="Sets the b243 field."]
    #[inline] pub fn set_b243<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="b244"]
    #[inline] pub fn b244(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if b244 != 0"]
    #[inline] pub fn test_b244(&self) -> bool {
        self.b244() != 0
    }

    #[doc="Sets the b244 field."]
    #[inline] pub fn set_b244<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="b245"]
    #[inline] pub fn b245(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if b245 != 0"]
    #[inline] pub fn test_b245(&self) -> bool {
        self.b245() != 0
    }

    #[doc="Sets the b245 field."]
    #[inline] pub fn set_b245<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="b246"]
    #[inline] pub fn b246(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if b246 != 0"]
    #[inline] pub fn test_b246(&self) -> bool {
        self.b246() != 0
    }

    #[doc="Sets the b246 field."]
    #[inline] pub fn set_b246<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="b247"]
    #[inline] pub fn b247(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if b247 != 0"]
    #[inline] pub fn test_b247(&self) -> bool {
        self.b247() != 0
    }

    #[doc="Sets the b247 field."]
    #[inline] pub fn set_b247<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="b248"]
    #[inline] pub fn b248(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if b248 != 0"]
    #[inline] pub fn test_b248(&self) -> bool {
        self.b248() != 0
    }

    #[doc="Sets the b248 field."]
    #[inline] pub fn set_b248<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="b249"]
    #[inline] pub fn b249(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if b249 != 0"]
    #[inline] pub fn test_b249(&self) -> bool {
        self.b249() != 0
    }

    #[doc="Sets the b249 field."]
    #[inline] pub fn set_b249<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="b250"]
    #[inline] pub fn b250(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if b250 != 0"]
    #[inline] pub fn test_b250(&self) -> bool {
        self.b250() != 0
    }

    #[doc="Sets the b250 field."]
    #[inline] pub fn set_b250<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="b251"]
    #[inline] pub fn b251(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if b251 != 0"]
    #[inline] pub fn test_b251(&self) -> bool {
        self.b251() != 0
    }

    #[doc="Sets the b251 field."]
    #[inline] pub fn set_b251<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="b252"]
    #[inline] pub fn b252(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if b252 != 0"]
    #[inline] pub fn test_b252(&self) -> bool {
        self.b252() != 0
    }

    #[doc="Sets the b252 field."]
    #[inline] pub fn set_b252<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="b253"]
    #[inline] pub fn b253(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if b253 != 0"]
    #[inline] pub fn test_b253(&self) -> bool {
        self.b253() != 0
    }

    #[doc="Sets the b253 field."]
    #[inline] pub fn set_b253<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="b254"]
    #[inline] pub fn b254(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if b254 != 0"]
    #[inline] pub fn test_b254(&self) -> bool {
        self.b254() != 0
    }

    #[doc="Sets the b254 field."]
    #[inline] pub fn set_b254<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="b255"]
    #[inline] pub fn b255(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if b255 != 0"]
    #[inline] pub fn test_b255(&self) -> bool {
        self.b255() != 0
    }

    #[doc="Sets the b255 field."]
    #[inline] pub fn set_b255<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for K0lr {
    #[inline]
    fn from(other: u32) -> Self {
         K0lr(other)
    }
}

impl ::core::fmt::Display for K0lr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for K0lr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.b224() != 0 { try!(write!(f, " b224"))}
        if self.b225() != 0 { try!(write!(f, " b225"))}
        if self.b226() != 0 { try!(write!(f, " b226"))}
        if self.b227() != 0 { try!(write!(f, " b227"))}
        if self.b228() != 0 { try!(write!(f, " b228"))}
        if self.b229() != 0 { try!(write!(f, " b229"))}
        if self.b230() != 0 { try!(write!(f, " b230"))}
        if self.b231() != 0 { try!(write!(f, " b231"))}
        if self.b232() != 0 { try!(write!(f, " b232"))}
        if self.b233() != 0 { try!(write!(f, " b233"))}
        if self.b234() != 0 { try!(write!(f, " b234"))}
        if self.b235() != 0 { try!(write!(f, " b235"))}
        if self.b236() != 0 { try!(write!(f, " b236"))}
        if self.b237() != 0 { try!(write!(f, " b237"))}
        if self.b238() != 0 { try!(write!(f, " b238"))}
        if self.b239() != 0 { try!(write!(f, " b239"))}
        if self.b240() != 0 { try!(write!(f, " b240"))}
        if self.b241() != 0 { try!(write!(f, " b241"))}
        if self.b242() != 0 { try!(write!(f, " b242"))}
        if self.b243() != 0 { try!(write!(f, " b243"))}
        if self.b244() != 0 { try!(write!(f, " b244"))}
        if self.b245() != 0 { try!(write!(f, " b245"))}
        if self.b246() != 0 { try!(write!(f, " b246"))}
        if self.b247() != 0 { try!(write!(f, " b247"))}
        if self.b248() != 0 { try!(write!(f, " b248"))}
        if self.b249() != 0 { try!(write!(f, " b249"))}
        if self.b250() != 0 { try!(write!(f, " b250"))}
        if self.b251() != 0 { try!(write!(f, " b251"))}
        if self.b252() != 0 { try!(write!(f, " b252"))}
        if self.b253() != 0 { try!(write!(f, " b253"))}
        if self.b254() != 0 { try!(write!(f, " b254"))}
        if self.b255() != 0 { try!(write!(f, " b255"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="key registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct K0rr(pub u32);
impl K0rr {
    #[doc="b192"]
    #[inline] pub fn b192(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if b192 != 0"]
    #[inline] pub fn test_b192(&self) -> bool {
        self.b192() != 0
    }

    #[doc="Sets the b192 field."]
    #[inline] pub fn set_b192<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="b193"]
    #[inline] pub fn b193(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if b193 != 0"]
    #[inline] pub fn test_b193(&self) -> bool {
        self.b193() != 0
    }

    #[doc="Sets the b193 field."]
    #[inline] pub fn set_b193<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="b194"]
    #[inline] pub fn b194(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if b194 != 0"]
    #[inline] pub fn test_b194(&self) -> bool {
        self.b194() != 0
    }

    #[doc="Sets the b194 field."]
    #[inline] pub fn set_b194<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="b195"]
    #[inline] pub fn b195(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if b195 != 0"]
    #[inline] pub fn test_b195(&self) -> bool {
        self.b195() != 0
    }

    #[doc="Sets the b195 field."]
    #[inline] pub fn set_b195<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="b196"]
    #[inline] pub fn b196(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if b196 != 0"]
    #[inline] pub fn test_b196(&self) -> bool {
        self.b196() != 0
    }

    #[doc="Sets the b196 field."]
    #[inline] pub fn set_b196<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="b197"]
    #[inline] pub fn b197(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if b197 != 0"]
    #[inline] pub fn test_b197(&self) -> bool {
        self.b197() != 0
    }

    #[doc="Sets the b197 field."]
    #[inline] pub fn set_b197<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="b198"]
    #[inline] pub fn b198(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if b198 != 0"]
    #[inline] pub fn test_b198(&self) -> bool {
        self.b198() != 0
    }

    #[doc="Sets the b198 field."]
    #[inline] pub fn set_b198<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="b199"]
    #[inline] pub fn b199(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if b199 != 0"]
    #[inline] pub fn test_b199(&self) -> bool {
        self.b199() != 0
    }

    #[doc="Sets the b199 field."]
    #[inline] pub fn set_b199<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="b200"]
    #[inline] pub fn b200(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if b200 != 0"]
    #[inline] pub fn test_b200(&self) -> bool {
        self.b200() != 0
    }

    #[doc="Sets the b200 field."]
    #[inline] pub fn set_b200<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="b201"]
    #[inline] pub fn b201(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if b201 != 0"]
    #[inline] pub fn test_b201(&self) -> bool {
        self.b201() != 0
    }

    #[doc="Sets the b201 field."]
    #[inline] pub fn set_b201<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="b202"]
    #[inline] pub fn b202(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if b202 != 0"]
    #[inline] pub fn test_b202(&self) -> bool {
        self.b202() != 0
    }

    #[doc="Sets the b202 field."]
    #[inline] pub fn set_b202<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="b203"]
    #[inline] pub fn b203(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if b203 != 0"]
    #[inline] pub fn test_b203(&self) -> bool {
        self.b203() != 0
    }

    #[doc="Sets the b203 field."]
    #[inline] pub fn set_b203<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="b204"]
    #[inline] pub fn b204(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if b204 != 0"]
    #[inline] pub fn test_b204(&self) -> bool {
        self.b204() != 0
    }

    #[doc="Sets the b204 field."]
    #[inline] pub fn set_b204<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="b205"]
    #[inline] pub fn b205(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if b205 != 0"]
    #[inline] pub fn test_b205(&self) -> bool {
        self.b205() != 0
    }

    #[doc="Sets the b205 field."]
    #[inline] pub fn set_b205<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="b206"]
    #[inline] pub fn b206(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if b206 != 0"]
    #[inline] pub fn test_b206(&self) -> bool {
        self.b206() != 0
    }

    #[doc="Sets the b206 field."]
    #[inline] pub fn set_b206<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="b207"]
    #[inline] pub fn b207(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if b207 != 0"]
    #[inline] pub fn test_b207(&self) -> bool {
        self.b207() != 0
    }

    #[doc="Sets the b207 field."]
    #[inline] pub fn set_b207<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="b208"]
    #[inline] pub fn b208(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if b208 != 0"]
    #[inline] pub fn test_b208(&self) -> bool {
        self.b208() != 0
    }

    #[doc="Sets the b208 field."]
    #[inline] pub fn set_b208<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="b209"]
    #[inline] pub fn b209(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if b209 != 0"]
    #[inline] pub fn test_b209(&self) -> bool {
        self.b209() != 0
    }

    #[doc="Sets the b209 field."]
    #[inline] pub fn set_b209<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="b210"]
    #[inline] pub fn b210(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if b210 != 0"]
    #[inline] pub fn test_b210(&self) -> bool {
        self.b210() != 0
    }

    #[doc="Sets the b210 field."]
    #[inline] pub fn set_b210<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="b211"]
    #[inline] pub fn b211(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if b211 != 0"]
    #[inline] pub fn test_b211(&self) -> bool {
        self.b211() != 0
    }

    #[doc="Sets the b211 field."]
    #[inline] pub fn set_b211<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="b212"]
    #[inline] pub fn b212(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if b212 != 0"]
    #[inline] pub fn test_b212(&self) -> bool {
        self.b212() != 0
    }

    #[doc="Sets the b212 field."]
    #[inline] pub fn set_b212<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="b213"]
    #[inline] pub fn b213(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if b213 != 0"]
    #[inline] pub fn test_b213(&self) -> bool {
        self.b213() != 0
    }

    #[doc="Sets the b213 field."]
    #[inline] pub fn set_b213<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="b214"]
    #[inline] pub fn b214(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if b214 != 0"]
    #[inline] pub fn test_b214(&self) -> bool {
        self.b214() != 0
    }

    #[doc="Sets the b214 field."]
    #[inline] pub fn set_b214<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="b215"]
    #[inline] pub fn b215(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if b215 != 0"]
    #[inline] pub fn test_b215(&self) -> bool {
        self.b215() != 0
    }

    #[doc="Sets the b215 field."]
    #[inline] pub fn set_b215<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="b216"]
    #[inline] pub fn b216(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if b216 != 0"]
    #[inline] pub fn test_b216(&self) -> bool {
        self.b216() != 0
    }

    #[doc="Sets the b216 field."]
    #[inline] pub fn set_b216<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="b217"]
    #[inline] pub fn b217(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if b217 != 0"]
    #[inline] pub fn test_b217(&self) -> bool {
        self.b217() != 0
    }

    #[doc="Sets the b217 field."]
    #[inline] pub fn set_b217<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="b218"]
    #[inline] pub fn b218(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if b218 != 0"]
    #[inline] pub fn test_b218(&self) -> bool {
        self.b218() != 0
    }

    #[doc="Sets the b218 field."]
    #[inline] pub fn set_b218<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="b219"]
    #[inline] pub fn b219(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if b219 != 0"]
    #[inline] pub fn test_b219(&self) -> bool {
        self.b219() != 0
    }

    #[doc="Sets the b219 field."]
    #[inline] pub fn set_b219<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="b220"]
    #[inline] pub fn b220(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if b220 != 0"]
    #[inline] pub fn test_b220(&self) -> bool {
        self.b220() != 0
    }

    #[doc="Sets the b220 field."]
    #[inline] pub fn set_b220<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="b221"]
    #[inline] pub fn b221(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if b221 != 0"]
    #[inline] pub fn test_b221(&self) -> bool {
        self.b221() != 0
    }

    #[doc="Sets the b221 field."]
    #[inline] pub fn set_b221<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="b222"]
    #[inline] pub fn b222(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if b222 != 0"]
    #[inline] pub fn test_b222(&self) -> bool {
        self.b222() != 0
    }

    #[doc="Sets the b222 field."]
    #[inline] pub fn set_b222<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="b223"]
    #[inline] pub fn b223(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if b223 != 0"]
    #[inline] pub fn test_b223(&self) -> bool {
        self.b223() != 0
    }

    #[doc="Sets the b223 field."]
    #[inline] pub fn set_b223<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for K0rr {
    #[inline]
    fn from(other: u32) -> Self {
         K0rr(other)
    }
}

impl ::core::fmt::Display for K0rr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for K0rr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.b192() != 0 { try!(write!(f, " b192"))}
        if self.b193() != 0 { try!(write!(f, " b193"))}
        if self.b194() != 0 { try!(write!(f, " b194"))}
        if self.b195() != 0 { try!(write!(f, " b195"))}
        if self.b196() != 0 { try!(write!(f, " b196"))}
        if self.b197() != 0 { try!(write!(f, " b197"))}
        if self.b198() != 0 { try!(write!(f, " b198"))}
        if self.b199() != 0 { try!(write!(f, " b199"))}
        if self.b200() != 0 { try!(write!(f, " b200"))}
        if self.b201() != 0 { try!(write!(f, " b201"))}
        if self.b202() != 0 { try!(write!(f, " b202"))}
        if self.b203() != 0 { try!(write!(f, " b203"))}
        if self.b204() != 0 { try!(write!(f, " b204"))}
        if self.b205() != 0 { try!(write!(f, " b205"))}
        if self.b206() != 0 { try!(write!(f, " b206"))}
        if self.b207() != 0 { try!(write!(f, " b207"))}
        if self.b208() != 0 { try!(write!(f, " b208"))}
        if self.b209() != 0 { try!(write!(f, " b209"))}
        if self.b210() != 0 { try!(write!(f, " b210"))}
        if self.b211() != 0 { try!(write!(f, " b211"))}
        if self.b212() != 0 { try!(write!(f, " b212"))}
        if self.b213() != 0 { try!(write!(f, " b213"))}
        if self.b214() != 0 { try!(write!(f, " b214"))}
        if self.b215() != 0 { try!(write!(f, " b215"))}
        if self.b216() != 0 { try!(write!(f, " b216"))}
        if self.b217() != 0 { try!(write!(f, " b217"))}
        if self.b218() != 0 { try!(write!(f, " b218"))}
        if self.b219() != 0 { try!(write!(f, " b219"))}
        if self.b220() != 0 { try!(write!(f, " b220"))}
        if self.b221() != 0 { try!(write!(f, " b221"))}
        if self.b222() != 0 { try!(write!(f, " b222"))}
        if self.b223() != 0 { try!(write!(f, " b223"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="key registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct K1lr(pub u32);
impl K1lr {
    #[doc="b160"]
    #[inline] pub fn b160(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if b160 != 0"]
    #[inline] pub fn test_b160(&self) -> bool {
        self.b160() != 0
    }

    #[doc="Sets the b160 field."]
    #[inline] pub fn set_b160<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="b161"]
    #[inline] pub fn b161(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if b161 != 0"]
    #[inline] pub fn test_b161(&self) -> bool {
        self.b161() != 0
    }

    #[doc="Sets the b161 field."]
    #[inline] pub fn set_b161<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="b162"]
    #[inline] pub fn b162(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if b162 != 0"]
    #[inline] pub fn test_b162(&self) -> bool {
        self.b162() != 0
    }

    #[doc="Sets the b162 field."]
    #[inline] pub fn set_b162<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="b163"]
    #[inline] pub fn b163(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if b163 != 0"]
    #[inline] pub fn test_b163(&self) -> bool {
        self.b163() != 0
    }

    #[doc="Sets the b163 field."]
    #[inline] pub fn set_b163<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="b164"]
    #[inline] pub fn b164(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if b164 != 0"]
    #[inline] pub fn test_b164(&self) -> bool {
        self.b164() != 0
    }

    #[doc="Sets the b164 field."]
    #[inline] pub fn set_b164<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="b165"]
    #[inline] pub fn b165(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if b165 != 0"]
    #[inline] pub fn test_b165(&self) -> bool {
        self.b165() != 0
    }

    #[doc="Sets the b165 field."]
    #[inline] pub fn set_b165<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="b166"]
    #[inline] pub fn b166(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if b166 != 0"]
    #[inline] pub fn test_b166(&self) -> bool {
        self.b166() != 0
    }

    #[doc="Sets the b166 field."]
    #[inline] pub fn set_b166<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="b167"]
    #[inline] pub fn b167(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if b167 != 0"]
    #[inline] pub fn test_b167(&self) -> bool {
        self.b167() != 0
    }

    #[doc="Sets the b167 field."]
    #[inline] pub fn set_b167<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="b168"]
    #[inline] pub fn b168(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if b168 != 0"]
    #[inline] pub fn test_b168(&self) -> bool {
        self.b168() != 0
    }

    #[doc="Sets the b168 field."]
    #[inline] pub fn set_b168<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="b169"]
    #[inline] pub fn b169(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if b169 != 0"]
    #[inline] pub fn test_b169(&self) -> bool {
        self.b169() != 0
    }

    #[doc="Sets the b169 field."]
    #[inline] pub fn set_b169<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="b170"]
    #[inline] pub fn b170(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if b170 != 0"]
    #[inline] pub fn test_b170(&self) -> bool {
        self.b170() != 0
    }

    #[doc="Sets the b170 field."]
    #[inline] pub fn set_b170<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="b171"]
    #[inline] pub fn b171(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if b171 != 0"]
    #[inline] pub fn test_b171(&self) -> bool {
        self.b171() != 0
    }

    #[doc="Sets the b171 field."]
    #[inline] pub fn set_b171<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="b172"]
    #[inline] pub fn b172(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if b172 != 0"]
    #[inline] pub fn test_b172(&self) -> bool {
        self.b172() != 0
    }

    #[doc="Sets the b172 field."]
    #[inline] pub fn set_b172<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="b173"]
    #[inline] pub fn b173(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if b173 != 0"]
    #[inline] pub fn test_b173(&self) -> bool {
        self.b173() != 0
    }

    #[doc="Sets the b173 field."]
    #[inline] pub fn set_b173<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="b174"]
    #[inline] pub fn b174(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if b174 != 0"]
    #[inline] pub fn test_b174(&self) -> bool {
        self.b174() != 0
    }

    #[doc="Sets the b174 field."]
    #[inline] pub fn set_b174<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="b175"]
    #[inline] pub fn b175(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if b175 != 0"]
    #[inline] pub fn test_b175(&self) -> bool {
        self.b175() != 0
    }

    #[doc="Sets the b175 field."]
    #[inline] pub fn set_b175<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="b176"]
    #[inline] pub fn b176(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if b176 != 0"]
    #[inline] pub fn test_b176(&self) -> bool {
        self.b176() != 0
    }

    #[doc="Sets the b176 field."]
    #[inline] pub fn set_b176<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="b177"]
    #[inline] pub fn b177(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if b177 != 0"]
    #[inline] pub fn test_b177(&self) -> bool {
        self.b177() != 0
    }

    #[doc="Sets the b177 field."]
    #[inline] pub fn set_b177<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="b178"]
    #[inline] pub fn b178(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if b178 != 0"]
    #[inline] pub fn test_b178(&self) -> bool {
        self.b178() != 0
    }

    #[doc="Sets the b178 field."]
    #[inline] pub fn set_b178<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="b179"]
    #[inline] pub fn b179(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if b179 != 0"]
    #[inline] pub fn test_b179(&self) -> bool {
        self.b179() != 0
    }

    #[doc="Sets the b179 field."]
    #[inline] pub fn set_b179<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="b180"]
    #[inline] pub fn b180(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if b180 != 0"]
    #[inline] pub fn test_b180(&self) -> bool {
        self.b180() != 0
    }

    #[doc="Sets the b180 field."]
    #[inline] pub fn set_b180<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="b181"]
    #[inline] pub fn b181(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if b181 != 0"]
    #[inline] pub fn test_b181(&self) -> bool {
        self.b181() != 0
    }

    #[doc="Sets the b181 field."]
    #[inline] pub fn set_b181<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="b182"]
    #[inline] pub fn b182(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if b182 != 0"]
    #[inline] pub fn test_b182(&self) -> bool {
        self.b182() != 0
    }

    #[doc="Sets the b182 field."]
    #[inline] pub fn set_b182<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="b183"]
    #[inline] pub fn b183(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if b183 != 0"]
    #[inline] pub fn test_b183(&self) -> bool {
        self.b183() != 0
    }

    #[doc="Sets the b183 field."]
    #[inline] pub fn set_b183<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="b184"]
    #[inline] pub fn b184(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if b184 != 0"]
    #[inline] pub fn test_b184(&self) -> bool {
        self.b184() != 0
    }

    #[doc="Sets the b184 field."]
    #[inline] pub fn set_b184<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="b185"]
    #[inline] pub fn b185(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if b185 != 0"]
    #[inline] pub fn test_b185(&self) -> bool {
        self.b185() != 0
    }

    #[doc="Sets the b185 field."]
    #[inline] pub fn set_b185<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="b186"]
    #[inline] pub fn b186(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if b186 != 0"]
    #[inline] pub fn test_b186(&self) -> bool {
        self.b186() != 0
    }

    #[doc="Sets the b186 field."]
    #[inline] pub fn set_b186<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="b187"]
    #[inline] pub fn b187(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if b187 != 0"]
    #[inline] pub fn test_b187(&self) -> bool {
        self.b187() != 0
    }

    #[doc="Sets the b187 field."]
    #[inline] pub fn set_b187<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="b188"]
    #[inline] pub fn b188(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if b188 != 0"]
    #[inline] pub fn test_b188(&self) -> bool {
        self.b188() != 0
    }

    #[doc="Sets the b188 field."]
    #[inline] pub fn set_b188<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="b189"]
    #[inline] pub fn b189(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if b189 != 0"]
    #[inline] pub fn test_b189(&self) -> bool {
        self.b189() != 0
    }

    #[doc="Sets the b189 field."]
    #[inline] pub fn set_b189<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="b190"]
    #[inline] pub fn b190(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if b190 != 0"]
    #[inline] pub fn test_b190(&self) -> bool {
        self.b190() != 0
    }

    #[doc="Sets the b190 field."]
    #[inline] pub fn set_b190<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="b191"]
    #[inline] pub fn b191(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if b191 != 0"]
    #[inline] pub fn test_b191(&self) -> bool {
        self.b191() != 0
    }

    #[doc="Sets the b191 field."]
    #[inline] pub fn set_b191<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for K1lr {
    #[inline]
    fn from(other: u32) -> Self {
         K1lr(other)
    }
}

impl ::core::fmt::Display for K1lr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for K1lr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.b160() != 0 { try!(write!(f, " b160"))}
        if self.b161() != 0 { try!(write!(f, " b161"))}
        if self.b162() != 0 { try!(write!(f, " b162"))}
        if self.b163() != 0 { try!(write!(f, " b163"))}
        if self.b164() != 0 { try!(write!(f, " b164"))}
        if self.b165() != 0 { try!(write!(f, " b165"))}
        if self.b166() != 0 { try!(write!(f, " b166"))}
        if self.b167() != 0 { try!(write!(f, " b167"))}
        if self.b168() != 0 { try!(write!(f, " b168"))}
        if self.b169() != 0 { try!(write!(f, " b169"))}
        if self.b170() != 0 { try!(write!(f, " b170"))}
        if self.b171() != 0 { try!(write!(f, " b171"))}
        if self.b172() != 0 { try!(write!(f, " b172"))}
        if self.b173() != 0 { try!(write!(f, " b173"))}
        if self.b174() != 0 { try!(write!(f, " b174"))}
        if self.b175() != 0 { try!(write!(f, " b175"))}
        if self.b176() != 0 { try!(write!(f, " b176"))}
        if self.b177() != 0 { try!(write!(f, " b177"))}
        if self.b178() != 0 { try!(write!(f, " b178"))}
        if self.b179() != 0 { try!(write!(f, " b179"))}
        if self.b180() != 0 { try!(write!(f, " b180"))}
        if self.b181() != 0 { try!(write!(f, " b181"))}
        if self.b182() != 0 { try!(write!(f, " b182"))}
        if self.b183() != 0 { try!(write!(f, " b183"))}
        if self.b184() != 0 { try!(write!(f, " b184"))}
        if self.b185() != 0 { try!(write!(f, " b185"))}
        if self.b186() != 0 { try!(write!(f, " b186"))}
        if self.b187() != 0 { try!(write!(f, " b187"))}
        if self.b188() != 0 { try!(write!(f, " b188"))}
        if self.b189() != 0 { try!(write!(f, " b189"))}
        if self.b190() != 0 { try!(write!(f, " b190"))}
        if self.b191() != 0 { try!(write!(f, " b191"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="key registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct K1rr(pub u32);
impl K1rr {
    #[doc="b128"]
    #[inline] pub fn b128(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if b128 != 0"]
    #[inline] pub fn test_b128(&self) -> bool {
        self.b128() != 0
    }

    #[doc="Sets the b128 field."]
    #[inline] pub fn set_b128<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="b129"]
    #[inline] pub fn b129(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if b129 != 0"]
    #[inline] pub fn test_b129(&self) -> bool {
        self.b129() != 0
    }

    #[doc="Sets the b129 field."]
    #[inline] pub fn set_b129<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="b130"]
    #[inline] pub fn b130(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if b130 != 0"]
    #[inline] pub fn test_b130(&self) -> bool {
        self.b130() != 0
    }

    #[doc="Sets the b130 field."]
    #[inline] pub fn set_b130<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="b131"]
    #[inline] pub fn b131(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if b131 != 0"]
    #[inline] pub fn test_b131(&self) -> bool {
        self.b131() != 0
    }

    #[doc="Sets the b131 field."]
    #[inline] pub fn set_b131<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="b132"]
    #[inline] pub fn b132(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if b132 != 0"]
    #[inline] pub fn test_b132(&self) -> bool {
        self.b132() != 0
    }

    #[doc="Sets the b132 field."]
    #[inline] pub fn set_b132<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="b133"]
    #[inline] pub fn b133(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if b133 != 0"]
    #[inline] pub fn test_b133(&self) -> bool {
        self.b133() != 0
    }

    #[doc="Sets the b133 field."]
    #[inline] pub fn set_b133<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="b134"]
    #[inline] pub fn b134(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if b134 != 0"]
    #[inline] pub fn test_b134(&self) -> bool {
        self.b134() != 0
    }

    #[doc="Sets the b134 field."]
    #[inline] pub fn set_b134<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="b135"]
    #[inline] pub fn b135(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if b135 != 0"]
    #[inline] pub fn test_b135(&self) -> bool {
        self.b135() != 0
    }

    #[doc="Sets the b135 field."]
    #[inline] pub fn set_b135<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="b136"]
    #[inline] pub fn b136(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if b136 != 0"]
    #[inline] pub fn test_b136(&self) -> bool {
        self.b136() != 0
    }

    #[doc="Sets the b136 field."]
    #[inline] pub fn set_b136<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="b137"]
    #[inline] pub fn b137(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if b137 != 0"]
    #[inline] pub fn test_b137(&self) -> bool {
        self.b137() != 0
    }

    #[doc="Sets the b137 field."]
    #[inline] pub fn set_b137<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="b138"]
    #[inline] pub fn b138(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if b138 != 0"]
    #[inline] pub fn test_b138(&self) -> bool {
        self.b138() != 0
    }

    #[doc="Sets the b138 field."]
    #[inline] pub fn set_b138<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="b139"]
    #[inline] pub fn b139(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if b139 != 0"]
    #[inline] pub fn test_b139(&self) -> bool {
        self.b139() != 0
    }

    #[doc="Sets the b139 field."]
    #[inline] pub fn set_b139<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="b140"]
    #[inline] pub fn b140(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if b140 != 0"]
    #[inline] pub fn test_b140(&self) -> bool {
        self.b140() != 0
    }

    #[doc="Sets the b140 field."]
    #[inline] pub fn set_b140<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="b141"]
    #[inline] pub fn b141(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if b141 != 0"]
    #[inline] pub fn test_b141(&self) -> bool {
        self.b141() != 0
    }

    #[doc="Sets the b141 field."]
    #[inline] pub fn set_b141<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="b142"]
    #[inline] pub fn b142(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if b142 != 0"]
    #[inline] pub fn test_b142(&self) -> bool {
        self.b142() != 0
    }

    #[doc="Sets the b142 field."]
    #[inline] pub fn set_b142<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="b143"]
    #[inline] pub fn b143(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if b143 != 0"]
    #[inline] pub fn test_b143(&self) -> bool {
        self.b143() != 0
    }

    #[doc="Sets the b143 field."]
    #[inline] pub fn set_b143<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="b144"]
    #[inline] pub fn b144(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if b144 != 0"]
    #[inline] pub fn test_b144(&self) -> bool {
        self.b144() != 0
    }

    #[doc="Sets the b144 field."]
    #[inline] pub fn set_b144<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="b145"]
    #[inline] pub fn b145(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if b145 != 0"]
    #[inline] pub fn test_b145(&self) -> bool {
        self.b145() != 0
    }

    #[doc="Sets the b145 field."]
    #[inline] pub fn set_b145<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="b146"]
    #[inline] pub fn b146(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if b146 != 0"]
    #[inline] pub fn test_b146(&self) -> bool {
        self.b146() != 0
    }

    #[doc="Sets the b146 field."]
    #[inline] pub fn set_b146<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="b147"]
    #[inline] pub fn b147(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if b147 != 0"]
    #[inline] pub fn test_b147(&self) -> bool {
        self.b147() != 0
    }

    #[doc="Sets the b147 field."]
    #[inline] pub fn set_b147<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="b148"]
    #[inline] pub fn b148(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if b148 != 0"]
    #[inline] pub fn test_b148(&self) -> bool {
        self.b148() != 0
    }

    #[doc="Sets the b148 field."]
    #[inline] pub fn set_b148<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="b149"]
    #[inline] pub fn b149(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if b149 != 0"]
    #[inline] pub fn test_b149(&self) -> bool {
        self.b149() != 0
    }

    #[doc="Sets the b149 field."]
    #[inline] pub fn set_b149<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="b150"]
    #[inline] pub fn b150(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if b150 != 0"]
    #[inline] pub fn test_b150(&self) -> bool {
        self.b150() != 0
    }

    #[doc="Sets the b150 field."]
    #[inline] pub fn set_b150<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="b151"]
    #[inline] pub fn b151(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if b151 != 0"]
    #[inline] pub fn test_b151(&self) -> bool {
        self.b151() != 0
    }

    #[doc="Sets the b151 field."]
    #[inline] pub fn set_b151<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="b152"]
    #[inline] pub fn b152(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if b152 != 0"]
    #[inline] pub fn test_b152(&self) -> bool {
        self.b152() != 0
    }

    #[doc="Sets the b152 field."]
    #[inline] pub fn set_b152<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="b153"]
    #[inline] pub fn b153(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if b153 != 0"]
    #[inline] pub fn test_b153(&self) -> bool {
        self.b153() != 0
    }

    #[doc="Sets the b153 field."]
    #[inline] pub fn set_b153<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="b154"]
    #[inline] pub fn b154(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if b154 != 0"]
    #[inline] pub fn test_b154(&self) -> bool {
        self.b154() != 0
    }

    #[doc="Sets the b154 field."]
    #[inline] pub fn set_b154<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="b155"]
    #[inline] pub fn b155(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if b155 != 0"]
    #[inline] pub fn test_b155(&self) -> bool {
        self.b155() != 0
    }

    #[doc="Sets the b155 field."]
    #[inline] pub fn set_b155<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="b156"]
    #[inline] pub fn b156(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if b156 != 0"]
    #[inline] pub fn test_b156(&self) -> bool {
        self.b156() != 0
    }

    #[doc="Sets the b156 field."]
    #[inline] pub fn set_b156<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="b157"]
    #[inline] pub fn b157(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if b157 != 0"]
    #[inline] pub fn test_b157(&self) -> bool {
        self.b157() != 0
    }

    #[doc="Sets the b157 field."]
    #[inline] pub fn set_b157<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="b158"]
    #[inline] pub fn b158(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if b158 != 0"]
    #[inline] pub fn test_b158(&self) -> bool {
        self.b158() != 0
    }

    #[doc="Sets the b158 field."]
    #[inline] pub fn set_b158<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="b159"]
    #[inline] pub fn b159(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if b159 != 0"]
    #[inline] pub fn test_b159(&self) -> bool {
        self.b159() != 0
    }

    #[doc="Sets the b159 field."]
    #[inline] pub fn set_b159<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for K1rr {
    #[inline]
    fn from(other: u32) -> Self {
         K1rr(other)
    }
}

impl ::core::fmt::Display for K1rr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for K1rr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.b128() != 0 { try!(write!(f, " b128"))}
        if self.b129() != 0 { try!(write!(f, " b129"))}
        if self.b130() != 0 { try!(write!(f, " b130"))}
        if self.b131() != 0 { try!(write!(f, " b131"))}
        if self.b132() != 0 { try!(write!(f, " b132"))}
        if self.b133() != 0 { try!(write!(f, " b133"))}
        if self.b134() != 0 { try!(write!(f, " b134"))}
        if self.b135() != 0 { try!(write!(f, " b135"))}
        if self.b136() != 0 { try!(write!(f, " b136"))}
        if self.b137() != 0 { try!(write!(f, " b137"))}
        if self.b138() != 0 { try!(write!(f, " b138"))}
        if self.b139() != 0 { try!(write!(f, " b139"))}
        if self.b140() != 0 { try!(write!(f, " b140"))}
        if self.b141() != 0 { try!(write!(f, " b141"))}
        if self.b142() != 0 { try!(write!(f, " b142"))}
        if self.b143() != 0 { try!(write!(f, " b143"))}
        if self.b144() != 0 { try!(write!(f, " b144"))}
        if self.b145() != 0 { try!(write!(f, " b145"))}
        if self.b146() != 0 { try!(write!(f, " b146"))}
        if self.b147() != 0 { try!(write!(f, " b147"))}
        if self.b148() != 0 { try!(write!(f, " b148"))}
        if self.b149() != 0 { try!(write!(f, " b149"))}
        if self.b150() != 0 { try!(write!(f, " b150"))}
        if self.b151() != 0 { try!(write!(f, " b151"))}
        if self.b152() != 0 { try!(write!(f, " b152"))}
        if self.b153() != 0 { try!(write!(f, " b153"))}
        if self.b154() != 0 { try!(write!(f, " b154"))}
        if self.b155() != 0 { try!(write!(f, " b155"))}
        if self.b156() != 0 { try!(write!(f, " b156"))}
        if self.b157() != 0 { try!(write!(f, " b157"))}
        if self.b158() != 0 { try!(write!(f, " b158"))}
        if self.b159() != 0 { try!(write!(f, " b159"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="key registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct K2lr(pub u32);
impl K2lr {
    #[doc="b96"]
    #[inline] pub fn b96(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if b96 != 0"]
    #[inline] pub fn test_b96(&self) -> bool {
        self.b96() != 0
    }

    #[doc="Sets the b96 field."]
    #[inline] pub fn set_b96<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="b97"]
    #[inline] pub fn b97(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if b97 != 0"]
    #[inline] pub fn test_b97(&self) -> bool {
        self.b97() != 0
    }

    #[doc="Sets the b97 field."]
    #[inline] pub fn set_b97<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="b98"]
    #[inline] pub fn b98(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if b98 != 0"]
    #[inline] pub fn test_b98(&self) -> bool {
        self.b98() != 0
    }

    #[doc="Sets the b98 field."]
    #[inline] pub fn set_b98<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="b99"]
    #[inline] pub fn b99(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if b99 != 0"]
    #[inline] pub fn test_b99(&self) -> bool {
        self.b99() != 0
    }

    #[doc="Sets the b99 field."]
    #[inline] pub fn set_b99<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="b100"]
    #[inline] pub fn b100(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if b100 != 0"]
    #[inline] pub fn test_b100(&self) -> bool {
        self.b100() != 0
    }

    #[doc="Sets the b100 field."]
    #[inline] pub fn set_b100<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="b101"]
    #[inline] pub fn b101(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if b101 != 0"]
    #[inline] pub fn test_b101(&self) -> bool {
        self.b101() != 0
    }

    #[doc="Sets the b101 field."]
    #[inline] pub fn set_b101<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="b102"]
    #[inline] pub fn b102(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if b102 != 0"]
    #[inline] pub fn test_b102(&self) -> bool {
        self.b102() != 0
    }

    #[doc="Sets the b102 field."]
    #[inline] pub fn set_b102<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="b103"]
    #[inline] pub fn b103(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if b103 != 0"]
    #[inline] pub fn test_b103(&self) -> bool {
        self.b103() != 0
    }

    #[doc="Sets the b103 field."]
    #[inline] pub fn set_b103<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="b104"]
    #[inline] pub fn b104(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if b104 != 0"]
    #[inline] pub fn test_b104(&self) -> bool {
        self.b104() != 0
    }

    #[doc="Sets the b104 field."]
    #[inline] pub fn set_b104<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="b105"]
    #[inline] pub fn b105(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if b105 != 0"]
    #[inline] pub fn test_b105(&self) -> bool {
        self.b105() != 0
    }

    #[doc="Sets the b105 field."]
    #[inline] pub fn set_b105<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="b106"]
    #[inline] pub fn b106(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if b106 != 0"]
    #[inline] pub fn test_b106(&self) -> bool {
        self.b106() != 0
    }

    #[doc="Sets the b106 field."]
    #[inline] pub fn set_b106<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="b107"]
    #[inline] pub fn b107(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if b107 != 0"]
    #[inline] pub fn test_b107(&self) -> bool {
        self.b107() != 0
    }

    #[doc="Sets the b107 field."]
    #[inline] pub fn set_b107<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="b108"]
    #[inline] pub fn b108(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if b108 != 0"]
    #[inline] pub fn test_b108(&self) -> bool {
        self.b108() != 0
    }

    #[doc="Sets the b108 field."]
    #[inline] pub fn set_b108<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="b109"]
    #[inline] pub fn b109(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if b109 != 0"]
    #[inline] pub fn test_b109(&self) -> bool {
        self.b109() != 0
    }

    #[doc="Sets the b109 field."]
    #[inline] pub fn set_b109<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="b110"]
    #[inline] pub fn b110(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if b110 != 0"]
    #[inline] pub fn test_b110(&self) -> bool {
        self.b110() != 0
    }

    #[doc="Sets the b110 field."]
    #[inline] pub fn set_b110<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="b111"]
    #[inline] pub fn b111(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if b111 != 0"]
    #[inline] pub fn test_b111(&self) -> bool {
        self.b111() != 0
    }

    #[doc="Sets the b111 field."]
    #[inline] pub fn set_b111<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="b112"]
    #[inline] pub fn b112(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if b112 != 0"]
    #[inline] pub fn test_b112(&self) -> bool {
        self.b112() != 0
    }

    #[doc="Sets the b112 field."]
    #[inline] pub fn set_b112<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="b113"]
    #[inline] pub fn b113(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if b113 != 0"]
    #[inline] pub fn test_b113(&self) -> bool {
        self.b113() != 0
    }

    #[doc="Sets the b113 field."]
    #[inline] pub fn set_b113<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="b114"]
    #[inline] pub fn b114(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if b114 != 0"]
    #[inline] pub fn test_b114(&self) -> bool {
        self.b114() != 0
    }

    #[doc="Sets the b114 field."]
    #[inline] pub fn set_b114<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="b115"]
    #[inline] pub fn b115(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if b115 != 0"]
    #[inline] pub fn test_b115(&self) -> bool {
        self.b115() != 0
    }

    #[doc="Sets the b115 field."]
    #[inline] pub fn set_b115<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="b116"]
    #[inline] pub fn b116(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if b116 != 0"]
    #[inline] pub fn test_b116(&self) -> bool {
        self.b116() != 0
    }

    #[doc="Sets the b116 field."]
    #[inline] pub fn set_b116<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="b117"]
    #[inline] pub fn b117(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if b117 != 0"]
    #[inline] pub fn test_b117(&self) -> bool {
        self.b117() != 0
    }

    #[doc="Sets the b117 field."]
    #[inline] pub fn set_b117<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="b118"]
    #[inline] pub fn b118(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if b118 != 0"]
    #[inline] pub fn test_b118(&self) -> bool {
        self.b118() != 0
    }

    #[doc="Sets the b118 field."]
    #[inline] pub fn set_b118<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="b119"]
    #[inline] pub fn b119(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if b119 != 0"]
    #[inline] pub fn test_b119(&self) -> bool {
        self.b119() != 0
    }

    #[doc="Sets the b119 field."]
    #[inline] pub fn set_b119<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="b120"]
    #[inline] pub fn b120(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if b120 != 0"]
    #[inline] pub fn test_b120(&self) -> bool {
        self.b120() != 0
    }

    #[doc="Sets the b120 field."]
    #[inline] pub fn set_b120<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="b121"]
    #[inline] pub fn b121(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if b121 != 0"]
    #[inline] pub fn test_b121(&self) -> bool {
        self.b121() != 0
    }

    #[doc="Sets the b121 field."]
    #[inline] pub fn set_b121<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="b122"]
    #[inline] pub fn b122(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if b122 != 0"]
    #[inline] pub fn test_b122(&self) -> bool {
        self.b122() != 0
    }

    #[doc="Sets the b122 field."]
    #[inline] pub fn set_b122<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="b123"]
    #[inline] pub fn b123(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if b123 != 0"]
    #[inline] pub fn test_b123(&self) -> bool {
        self.b123() != 0
    }

    #[doc="Sets the b123 field."]
    #[inline] pub fn set_b123<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="b124"]
    #[inline] pub fn b124(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if b124 != 0"]
    #[inline] pub fn test_b124(&self) -> bool {
        self.b124() != 0
    }

    #[doc="Sets the b124 field."]
    #[inline] pub fn set_b124<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="b125"]
    #[inline] pub fn b125(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if b125 != 0"]
    #[inline] pub fn test_b125(&self) -> bool {
        self.b125() != 0
    }

    #[doc="Sets the b125 field."]
    #[inline] pub fn set_b125<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="b126"]
    #[inline] pub fn b126(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if b126 != 0"]
    #[inline] pub fn test_b126(&self) -> bool {
        self.b126() != 0
    }

    #[doc="Sets the b126 field."]
    #[inline] pub fn set_b126<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="b127"]
    #[inline] pub fn b127(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if b127 != 0"]
    #[inline] pub fn test_b127(&self) -> bool {
        self.b127() != 0
    }

    #[doc="Sets the b127 field."]
    #[inline] pub fn set_b127<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for K2lr {
    #[inline]
    fn from(other: u32) -> Self {
         K2lr(other)
    }
}

impl ::core::fmt::Display for K2lr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for K2lr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.b96() != 0 { try!(write!(f, " b96"))}
        if self.b97() != 0 { try!(write!(f, " b97"))}
        if self.b98() != 0 { try!(write!(f, " b98"))}
        if self.b99() != 0 { try!(write!(f, " b99"))}
        if self.b100() != 0 { try!(write!(f, " b100"))}
        if self.b101() != 0 { try!(write!(f, " b101"))}
        if self.b102() != 0 { try!(write!(f, " b102"))}
        if self.b103() != 0 { try!(write!(f, " b103"))}
        if self.b104() != 0 { try!(write!(f, " b104"))}
        if self.b105() != 0 { try!(write!(f, " b105"))}
        if self.b106() != 0 { try!(write!(f, " b106"))}
        if self.b107() != 0 { try!(write!(f, " b107"))}
        if self.b108() != 0 { try!(write!(f, " b108"))}
        if self.b109() != 0 { try!(write!(f, " b109"))}
        if self.b110() != 0 { try!(write!(f, " b110"))}
        if self.b111() != 0 { try!(write!(f, " b111"))}
        if self.b112() != 0 { try!(write!(f, " b112"))}
        if self.b113() != 0 { try!(write!(f, " b113"))}
        if self.b114() != 0 { try!(write!(f, " b114"))}
        if self.b115() != 0 { try!(write!(f, " b115"))}
        if self.b116() != 0 { try!(write!(f, " b116"))}
        if self.b117() != 0 { try!(write!(f, " b117"))}
        if self.b118() != 0 { try!(write!(f, " b118"))}
        if self.b119() != 0 { try!(write!(f, " b119"))}
        if self.b120() != 0 { try!(write!(f, " b120"))}
        if self.b121() != 0 { try!(write!(f, " b121"))}
        if self.b122() != 0 { try!(write!(f, " b122"))}
        if self.b123() != 0 { try!(write!(f, " b123"))}
        if self.b124() != 0 { try!(write!(f, " b124"))}
        if self.b125() != 0 { try!(write!(f, " b125"))}
        if self.b126() != 0 { try!(write!(f, " b126"))}
        if self.b127() != 0 { try!(write!(f, " b127"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="key registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct K2rr(pub u32);
impl K2rr {
    #[doc="b64"]
    #[inline] pub fn b64(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if b64 != 0"]
    #[inline] pub fn test_b64(&self) -> bool {
        self.b64() != 0
    }

    #[doc="Sets the b64 field."]
    #[inline] pub fn set_b64<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="b65"]
    #[inline] pub fn b65(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if b65 != 0"]
    #[inline] pub fn test_b65(&self) -> bool {
        self.b65() != 0
    }

    #[doc="Sets the b65 field."]
    #[inline] pub fn set_b65<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="b66"]
    #[inline] pub fn b66(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if b66 != 0"]
    #[inline] pub fn test_b66(&self) -> bool {
        self.b66() != 0
    }

    #[doc="Sets the b66 field."]
    #[inline] pub fn set_b66<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="b67"]
    #[inline] pub fn b67(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if b67 != 0"]
    #[inline] pub fn test_b67(&self) -> bool {
        self.b67() != 0
    }

    #[doc="Sets the b67 field."]
    #[inline] pub fn set_b67<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="b68"]
    #[inline] pub fn b68(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if b68 != 0"]
    #[inline] pub fn test_b68(&self) -> bool {
        self.b68() != 0
    }

    #[doc="Sets the b68 field."]
    #[inline] pub fn set_b68<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="b69"]
    #[inline] pub fn b69(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if b69 != 0"]
    #[inline] pub fn test_b69(&self) -> bool {
        self.b69() != 0
    }

    #[doc="Sets the b69 field."]
    #[inline] pub fn set_b69<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="b70"]
    #[inline] pub fn b70(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if b70 != 0"]
    #[inline] pub fn test_b70(&self) -> bool {
        self.b70() != 0
    }

    #[doc="Sets the b70 field."]
    #[inline] pub fn set_b70<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="b71"]
    #[inline] pub fn b71(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if b71 != 0"]
    #[inline] pub fn test_b71(&self) -> bool {
        self.b71() != 0
    }

    #[doc="Sets the b71 field."]
    #[inline] pub fn set_b71<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="b72"]
    #[inline] pub fn b72(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if b72 != 0"]
    #[inline] pub fn test_b72(&self) -> bool {
        self.b72() != 0
    }

    #[doc="Sets the b72 field."]
    #[inline] pub fn set_b72<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="b73"]
    #[inline] pub fn b73(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if b73 != 0"]
    #[inline] pub fn test_b73(&self) -> bool {
        self.b73() != 0
    }

    #[doc="Sets the b73 field."]
    #[inline] pub fn set_b73<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="b74"]
    #[inline] pub fn b74(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if b74 != 0"]
    #[inline] pub fn test_b74(&self) -> bool {
        self.b74() != 0
    }

    #[doc="Sets the b74 field."]
    #[inline] pub fn set_b74<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="b75"]
    #[inline] pub fn b75(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if b75 != 0"]
    #[inline] pub fn test_b75(&self) -> bool {
        self.b75() != 0
    }

    #[doc="Sets the b75 field."]
    #[inline] pub fn set_b75<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="b76"]
    #[inline] pub fn b76(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if b76 != 0"]
    #[inline] pub fn test_b76(&self) -> bool {
        self.b76() != 0
    }

    #[doc="Sets the b76 field."]
    #[inline] pub fn set_b76<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="b77"]
    #[inline] pub fn b77(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if b77 != 0"]
    #[inline] pub fn test_b77(&self) -> bool {
        self.b77() != 0
    }

    #[doc="Sets the b77 field."]
    #[inline] pub fn set_b77<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="b78"]
    #[inline] pub fn b78(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if b78 != 0"]
    #[inline] pub fn test_b78(&self) -> bool {
        self.b78() != 0
    }

    #[doc="Sets the b78 field."]
    #[inline] pub fn set_b78<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="b79"]
    #[inline] pub fn b79(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if b79 != 0"]
    #[inline] pub fn test_b79(&self) -> bool {
        self.b79() != 0
    }

    #[doc="Sets the b79 field."]
    #[inline] pub fn set_b79<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="b80"]
    #[inline] pub fn b80(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if b80 != 0"]
    #[inline] pub fn test_b80(&self) -> bool {
        self.b80() != 0
    }

    #[doc="Sets the b80 field."]
    #[inline] pub fn set_b80<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="b81"]
    #[inline] pub fn b81(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if b81 != 0"]
    #[inline] pub fn test_b81(&self) -> bool {
        self.b81() != 0
    }

    #[doc="Sets the b81 field."]
    #[inline] pub fn set_b81<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="b82"]
    #[inline] pub fn b82(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if b82 != 0"]
    #[inline] pub fn test_b82(&self) -> bool {
        self.b82() != 0
    }

    #[doc="Sets the b82 field."]
    #[inline] pub fn set_b82<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="b83"]
    #[inline] pub fn b83(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if b83 != 0"]
    #[inline] pub fn test_b83(&self) -> bool {
        self.b83() != 0
    }

    #[doc="Sets the b83 field."]
    #[inline] pub fn set_b83<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="b84"]
    #[inline] pub fn b84(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if b84 != 0"]
    #[inline] pub fn test_b84(&self) -> bool {
        self.b84() != 0
    }

    #[doc="Sets the b84 field."]
    #[inline] pub fn set_b84<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="b85"]
    #[inline] pub fn b85(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if b85 != 0"]
    #[inline] pub fn test_b85(&self) -> bool {
        self.b85() != 0
    }

    #[doc="Sets the b85 field."]
    #[inline] pub fn set_b85<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="b86"]
    #[inline] pub fn b86(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if b86 != 0"]
    #[inline] pub fn test_b86(&self) -> bool {
        self.b86() != 0
    }

    #[doc="Sets the b86 field."]
    #[inline] pub fn set_b86<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="b87"]
    #[inline] pub fn b87(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if b87 != 0"]
    #[inline] pub fn test_b87(&self) -> bool {
        self.b87() != 0
    }

    #[doc="Sets the b87 field."]
    #[inline] pub fn set_b87<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="b88"]
    #[inline] pub fn b88(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if b88 != 0"]
    #[inline] pub fn test_b88(&self) -> bool {
        self.b88() != 0
    }

    #[doc="Sets the b88 field."]
    #[inline] pub fn set_b88<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="b89"]
    #[inline] pub fn b89(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if b89 != 0"]
    #[inline] pub fn test_b89(&self) -> bool {
        self.b89() != 0
    }

    #[doc="Sets the b89 field."]
    #[inline] pub fn set_b89<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="b90"]
    #[inline] pub fn b90(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if b90 != 0"]
    #[inline] pub fn test_b90(&self) -> bool {
        self.b90() != 0
    }

    #[doc="Sets the b90 field."]
    #[inline] pub fn set_b90<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="b91"]
    #[inline] pub fn b91(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if b91 != 0"]
    #[inline] pub fn test_b91(&self) -> bool {
        self.b91() != 0
    }

    #[doc="Sets the b91 field."]
    #[inline] pub fn set_b91<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="b92"]
    #[inline] pub fn b92(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if b92 != 0"]
    #[inline] pub fn test_b92(&self) -> bool {
        self.b92() != 0
    }

    #[doc="Sets the b92 field."]
    #[inline] pub fn set_b92<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="b93"]
    #[inline] pub fn b93(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if b93 != 0"]
    #[inline] pub fn test_b93(&self) -> bool {
        self.b93() != 0
    }

    #[doc="Sets the b93 field."]
    #[inline] pub fn set_b93<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="b94"]
    #[inline] pub fn b94(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if b94 != 0"]
    #[inline] pub fn test_b94(&self) -> bool {
        self.b94() != 0
    }

    #[doc="Sets the b94 field."]
    #[inline] pub fn set_b94<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="b95"]
    #[inline] pub fn b95(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if b95 != 0"]
    #[inline] pub fn test_b95(&self) -> bool {
        self.b95() != 0
    }

    #[doc="Sets the b95 field."]
    #[inline] pub fn set_b95<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for K2rr {
    #[inline]
    fn from(other: u32) -> Self {
         K2rr(other)
    }
}

impl ::core::fmt::Display for K2rr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for K2rr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.b64() != 0 { try!(write!(f, " b64"))}
        if self.b65() != 0 { try!(write!(f, " b65"))}
        if self.b66() != 0 { try!(write!(f, " b66"))}
        if self.b67() != 0 { try!(write!(f, " b67"))}
        if self.b68() != 0 { try!(write!(f, " b68"))}
        if self.b69() != 0 { try!(write!(f, " b69"))}
        if self.b70() != 0 { try!(write!(f, " b70"))}
        if self.b71() != 0 { try!(write!(f, " b71"))}
        if self.b72() != 0 { try!(write!(f, " b72"))}
        if self.b73() != 0 { try!(write!(f, " b73"))}
        if self.b74() != 0 { try!(write!(f, " b74"))}
        if self.b75() != 0 { try!(write!(f, " b75"))}
        if self.b76() != 0 { try!(write!(f, " b76"))}
        if self.b77() != 0 { try!(write!(f, " b77"))}
        if self.b78() != 0 { try!(write!(f, " b78"))}
        if self.b79() != 0 { try!(write!(f, " b79"))}
        if self.b80() != 0 { try!(write!(f, " b80"))}
        if self.b81() != 0 { try!(write!(f, " b81"))}
        if self.b82() != 0 { try!(write!(f, " b82"))}
        if self.b83() != 0 { try!(write!(f, " b83"))}
        if self.b84() != 0 { try!(write!(f, " b84"))}
        if self.b85() != 0 { try!(write!(f, " b85"))}
        if self.b86() != 0 { try!(write!(f, " b86"))}
        if self.b87() != 0 { try!(write!(f, " b87"))}
        if self.b88() != 0 { try!(write!(f, " b88"))}
        if self.b89() != 0 { try!(write!(f, " b89"))}
        if self.b90() != 0 { try!(write!(f, " b90"))}
        if self.b91() != 0 { try!(write!(f, " b91"))}
        if self.b92() != 0 { try!(write!(f, " b92"))}
        if self.b93() != 0 { try!(write!(f, " b93"))}
        if self.b94() != 0 { try!(write!(f, " b94"))}
        if self.b95() != 0 { try!(write!(f, " b95"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="key registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct K3lr(pub u32);
impl K3lr {
    #[doc="b32"]
    #[inline] pub fn b32(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if b32 != 0"]
    #[inline] pub fn test_b32(&self) -> bool {
        self.b32() != 0
    }

    #[doc="Sets the b32 field."]
    #[inline] pub fn set_b32<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="b33"]
    #[inline] pub fn b33(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if b33 != 0"]
    #[inline] pub fn test_b33(&self) -> bool {
        self.b33() != 0
    }

    #[doc="Sets the b33 field."]
    #[inline] pub fn set_b33<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="b34"]
    #[inline] pub fn b34(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if b34 != 0"]
    #[inline] pub fn test_b34(&self) -> bool {
        self.b34() != 0
    }

    #[doc="Sets the b34 field."]
    #[inline] pub fn set_b34<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="b35"]
    #[inline] pub fn b35(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if b35 != 0"]
    #[inline] pub fn test_b35(&self) -> bool {
        self.b35() != 0
    }

    #[doc="Sets the b35 field."]
    #[inline] pub fn set_b35<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="b36"]
    #[inline] pub fn b36(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if b36 != 0"]
    #[inline] pub fn test_b36(&self) -> bool {
        self.b36() != 0
    }

    #[doc="Sets the b36 field."]
    #[inline] pub fn set_b36<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="b37"]
    #[inline] pub fn b37(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if b37 != 0"]
    #[inline] pub fn test_b37(&self) -> bool {
        self.b37() != 0
    }

    #[doc="Sets the b37 field."]
    #[inline] pub fn set_b37<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="b38"]
    #[inline] pub fn b38(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if b38 != 0"]
    #[inline] pub fn test_b38(&self) -> bool {
        self.b38() != 0
    }

    #[doc="Sets the b38 field."]
    #[inline] pub fn set_b38<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="b39"]
    #[inline] pub fn b39(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if b39 != 0"]
    #[inline] pub fn test_b39(&self) -> bool {
        self.b39() != 0
    }

    #[doc="Sets the b39 field."]
    #[inline] pub fn set_b39<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="b40"]
    #[inline] pub fn b40(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if b40 != 0"]
    #[inline] pub fn test_b40(&self) -> bool {
        self.b40() != 0
    }

    #[doc="Sets the b40 field."]
    #[inline] pub fn set_b40<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="b41"]
    #[inline] pub fn b41(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if b41 != 0"]
    #[inline] pub fn test_b41(&self) -> bool {
        self.b41() != 0
    }

    #[doc="Sets the b41 field."]
    #[inline] pub fn set_b41<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="b42"]
    #[inline] pub fn b42(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if b42 != 0"]
    #[inline] pub fn test_b42(&self) -> bool {
        self.b42() != 0
    }

    #[doc="Sets the b42 field."]
    #[inline] pub fn set_b42<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="b43"]
    #[inline] pub fn b43(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if b43 != 0"]
    #[inline] pub fn test_b43(&self) -> bool {
        self.b43() != 0
    }

    #[doc="Sets the b43 field."]
    #[inline] pub fn set_b43<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="b44"]
    #[inline] pub fn b44(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if b44 != 0"]
    #[inline] pub fn test_b44(&self) -> bool {
        self.b44() != 0
    }

    #[doc="Sets the b44 field."]
    #[inline] pub fn set_b44<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="b45"]
    #[inline] pub fn b45(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if b45 != 0"]
    #[inline] pub fn test_b45(&self) -> bool {
        self.b45() != 0
    }

    #[doc="Sets the b45 field."]
    #[inline] pub fn set_b45<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="b46"]
    #[inline] pub fn b46(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if b46 != 0"]
    #[inline] pub fn test_b46(&self) -> bool {
        self.b46() != 0
    }

    #[doc="Sets the b46 field."]
    #[inline] pub fn set_b46<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="b47"]
    #[inline] pub fn b47(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if b47 != 0"]
    #[inline] pub fn test_b47(&self) -> bool {
        self.b47() != 0
    }

    #[doc="Sets the b47 field."]
    #[inline] pub fn set_b47<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="b48"]
    #[inline] pub fn b48(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if b48 != 0"]
    #[inline] pub fn test_b48(&self) -> bool {
        self.b48() != 0
    }

    #[doc="Sets the b48 field."]
    #[inline] pub fn set_b48<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="b49"]
    #[inline] pub fn b49(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if b49 != 0"]
    #[inline] pub fn test_b49(&self) -> bool {
        self.b49() != 0
    }

    #[doc="Sets the b49 field."]
    #[inline] pub fn set_b49<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="b50"]
    #[inline] pub fn b50(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if b50 != 0"]
    #[inline] pub fn test_b50(&self) -> bool {
        self.b50() != 0
    }

    #[doc="Sets the b50 field."]
    #[inline] pub fn set_b50<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="b51"]
    #[inline] pub fn b51(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if b51 != 0"]
    #[inline] pub fn test_b51(&self) -> bool {
        self.b51() != 0
    }

    #[doc="Sets the b51 field."]
    #[inline] pub fn set_b51<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="b52"]
    #[inline] pub fn b52(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if b52 != 0"]
    #[inline] pub fn test_b52(&self) -> bool {
        self.b52() != 0
    }

    #[doc="Sets the b52 field."]
    #[inline] pub fn set_b52<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="b53"]
    #[inline] pub fn b53(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if b53 != 0"]
    #[inline] pub fn test_b53(&self) -> bool {
        self.b53() != 0
    }

    #[doc="Sets the b53 field."]
    #[inline] pub fn set_b53<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="b54"]
    #[inline] pub fn b54(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if b54 != 0"]
    #[inline] pub fn test_b54(&self) -> bool {
        self.b54() != 0
    }

    #[doc="Sets the b54 field."]
    #[inline] pub fn set_b54<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="b55"]
    #[inline] pub fn b55(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if b55 != 0"]
    #[inline] pub fn test_b55(&self) -> bool {
        self.b55() != 0
    }

    #[doc="Sets the b55 field."]
    #[inline] pub fn set_b55<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="b56"]
    #[inline] pub fn b56(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if b56 != 0"]
    #[inline] pub fn test_b56(&self) -> bool {
        self.b56() != 0
    }

    #[doc="Sets the b56 field."]
    #[inline] pub fn set_b56<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="b57"]
    #[inline] pub fn b57(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if b57 != 0"]
    #[inline] pub fn test_b57(&self) -> bool {
        self.b57() != 0
    }

    #[doc="Sets the b57 field."]
    #[inline] pub fn set_b57<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="b58"]
    #[inline] pub fn b58(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if b58 != 0"]
    #[inline] pub fn test_b58(&self) -> bool {
        self.b58() != 0
    }

    #[doc="Sets the b58 field."]
    #[inline] pub fn set_b58<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="b59"]
    #[inline] pub fn b59(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if b59 != 0"]
    #[inline] pub fn test_b59(&self) -> bool {
        self.b59() != 0
    }

    #[doc="Sets the b59 field."]
    #[inline] pub fn set_b59<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="b60"]
    #[inline] pub fn b60(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if b60 != 0"]
    #[inline] pub fn test_b60(&self) -> bool {
        self.b60() != 0
    }

    #[doc="Sets the b60 field."]
    #[inline] pub fn set_b60<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="b61"]
    #[inline] pub fn b61(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if b61 != 0"]
    #[inline] pub fn test_b61(&self) -> bool {
        self.b61() != 0
    }

    #[doc="Sets the b61 field."]
    #[inline] pub fn set_b61<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="b62"]
    #[inline] pub fn b62(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if b62 != 0"]
    #[inline] pub fn test_b62(&self) -> bool {
        self.b62() != 0
    }

    #[doc="Sets the b62 field."]
    #[inline] pub fn set_b62<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="b63"]
    #[inline] pub fn b63(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if b63 != 0"]
    #[inline] pub fn test_b63(&self) -> bool {
        self.b63() != 0
    }

    #[doc="Sets the b63 field."]
    #[inline] pub fn set_b63<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for K3lr {
    #[inline]
    fn from(other: u32) -> Self {
         K3lr(other)
    }
}

impl ::core::fmt::Display for K3lr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for K3lr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.b32() != 0 { try!(write!(f, " b32"))}
        if self.b33() != 0 { try!(write!(f, " b33"))}
        if self.b34() != 0 { try!(write!(f, " b34"))}
        if self.b35() != 0 { try!(write!(f, " b35"))}
        if self.b36() != 0 { try!(write!(f, " b36"))}
        if self.b37() != 0 { try!(write!(f, " b37"))}
        if self.b38() != 0 { try!(write!(f, " b38"))}
        if self.b39() != 0 { try!(write!(f, " b39"))}
        if self.b40() != 0 { try!(write!(f, " b40"))}
        if self.b41() != 0 { try!(write!(f, " b41"))}
        if self.b42() != 0 { try!(write!(f, " b42"))}
        if self.b43() != 0 { try!(write!(f, " b43"))}
        if self.b44() != 0 { try!(write!(f, " b44"))}
        if self.b45() != 0 { try!(write!(f, " b45"))}
        if self.b46() != 0 { try!(write!(f, " b46"))}
        if self.b47() != 0 { try!(write!(f, " b47"))}
        if self.b48() != 0 { try!(write!(f, " b48"))}
        if self.b49() != 0 { try!(write!(f, " b49"))}
        if self.b50() != 0 { try!(write!(f, " b50"))}
        if self.b51() != 0 { try!(write!(f, " b51"))}
        if self.b52() != 0 { try!(write!(f, " b52"))}
        if self.b53() != 0 { try!(write!(f, " b53"))}
        if self.b54() != 0 { try!(write!(f, " b54"))}
        if self.b55() != 0 { try!(write!(f, " b55"))}
        if self.b56() != 0 { try!(write!(f, " b56"))}
        if self.b57() != 0 { try!(write!(f, " b57"))}
        if self.b58() != 0 { try!(write!(f, " b58"))}
        if self.b59() != 0 { try!(write!(f, " b59"))}
        if self.b60() != 0 { try!(write!(f, " b60"))}
        if self.b61() != 0 { try!(write!(f, " b61"))}
        if self.b62() != 0 { try!(write!(f, " b62"))}
        if self.b63() != 0 { try!(write!(f, " b63"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="key registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct K3rr(pub u32);
impl K3rr {
    #[doc="b0"]
    #[inline] pub fn b0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if b0 != 0"]
    #[inline] pub fn test_b0(&self) -> bool {
        self.b0() != 0
    }

    #[doc="Sets the b0 field."]
    #[inline] pub fn set_b0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="b1"]
    #[inline] pub fn b1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if b1 != 0"]
    #[inline] pub fn test_b1(&self) -> bool {
        self.b1() != 0
    }

    #[doc="Sets the b1 field."]
    #[inline] pub fn set_b1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="b2"]
    #[inline] pub fn b2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if b2 != 0"]
    #[inline] pub fn test_b2(&self) -> bool {
        self.b2() != 0
    }

    #[doc="Sets the b2 field."]
    #[inline] pub fn set_b2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="b3"]
    #[inline] pub fn b3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if b3 != 0"]
    #[inline] pub fn test_b3(&self) -> bool {
        self.b3() != 0
    }

    #[doc="Sets the b3 field."]
    #[inline] pub fn set_b3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="b4"]
    #[inline] pub fn b4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if b4 != 0"]
    #[inline] pub fn test_b4(&self) -> bool {
        self.b4() != 0
    }

    #[doc="Sets the b4 field."]
    #[inline] pub fn set_b4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="b5"]
    #[inline] pub fn b5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if b5 != 0"]
    #[inline] pub fn test_b5(&self) -> bool {
        self.b5() != 0
    }

    #[doc="Sets the b5 field."]
    #[inline] pub fn set_b5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="b6"]
    #[inline] pub fn b6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if b6 != 0"]
    #[inline] pub fn test_b6(&self) -> bool {
        self.b6() != 0
    }

    #[doc="Sets the b6 field."]
    #[inline] pub fn set_b6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="b7"]
    #[inline] pub fn b7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if b7 != 0"]
    #[inline] pub fn test_b7(&self) -> bool {
        self.b7() != 0
    }

    #[doc="Sets the b7 field."]
    #[inline] pub fn set_b7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="b8"]
    #[inline] pub fn b8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if b8 != 0"]
    #[inline] pub fn test_b8(&self) -> bool {
        self.b8() != 0
    }

    #[doc="Sets the b8 field."]
    #[inline] pub fn set_b8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="b9"]
    #[inline] pub fn b9(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if b9 != 0"]
    #[inline] pub fn test_b9(&self) -> bool {
        self.b9() != 0
    }

    #[doc="Sets the b9 field."]
    #[inline] pub fn set_b9<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="b10"]
    #[inline] pub fn b10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if b10 != 0"]
    #[inline] pub fn test_b10(&self) -> bool {
        self.b10() != 0
    }

    #[doc="Sets the b10 field."]
    #[inline] pub fn set_b10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="b11"]
    #[inline] pub fn b11(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if b11 != 0"]
    #[inline] pub fn test_b11(&self) -> bool {
        self.b11() != 0
    }

    #[doc="Sets the b11 field."]
    #[inline] pub fn set_b11<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="b12"]
    #[inline] pub fn b12(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if b12 != 0"]
    #[inline] pub fn test_b12(&self) -> bool {
        self.b12() != 0
    }

    #[doc="Sets the b12 field."]
    #[inline] pub fn set_b12<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="b13"]
    #[inline] pub fn b13(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if b13 != 0"]
    #[inline] pub fn test_b13(&self) -> bool {
        self.b13() != 0
    }

    #[doc="Sets the b13 field."]
    #[inline] pub fn set_b13<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="b14"]
    #[inline] pub fn b14(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if b14 != 0"]
    #[inline] pub fn test_b14(&self) -> bool {
        self.b14() != 0
    }

    #[doc="Sets the b14 field."]
    #[inline] pub fn set_b14<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="b15"]
    #[inline] pub fn b15(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if b15 != 0"]
    #[inline] pub fn test_b15(&self) -> bool {
        self.b15() != 0
    }

    #[doc="Sets the b15 field."]
    #[inline] pub fn set_b15<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="b16"]
    #[inline] pub fn b16(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if b16 != 0"]
    #[inline] pub fn test_b16(&self) -> bool {
        self.b16() != 0
    }

    #[doc="Sets the b16 field."]
    #[inline] pub fn set_b16<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="b17"]
    #[inline] pub fn b17(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if b17 != 0"]
    #[inline] pub fn test_b17(&self) -> bool {
        self.b17() != 0
    }

    #[doc="Sets the b17 field."]
    #[inline] pub fn set_b17<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="b18"]
    #[inline] pub fn b18(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if b18 != 0"]
    #[inline] pub fn test_b18(&self) -> bool {
        self.b18() != 0
    }

    #[doc="Sets the b18 field."]
    #[inline] pub fn set_b18<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="b19"]
    #[inline] pub fn b19(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if b19 != 0"]
    #[inline] pub fn test_b19(&self) -> bool {
        self.b19() != 0
    }

    #[doc="Sets the b19 field."]
    #[inline] pub fn set_b19<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="b20"]
    #[inline] pub fn b20(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if b20 != 0"]
    #[inline] pub fn test_b20(&self) -> bool {
        self.b20() != 0
    }

    #[doc="Sets the b20 field."]
    #[inline] pub fn set_b20<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="b21"]
    #[inline] pub fn b21(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if b21 != 0"]
    #[inline] pub fn test_b21(&self) -> bool {
        self.b21() != 0
    }

    #[doc="Sets the b21 field."]
    #[inline] pub fn set_b21<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="b22"]
    #[inline] pub fn b22(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if b22 != 0"]
    #[inline] pub fn test_b22(&self) -> bool {
        self.b22() != 0
    }

    #[doc="Sets the b22 field."]
    #[inline] pub fn set_b22<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="b23"]
    #[inline] pub fn b23(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if b23 != 0"]
    #[inline] pub fn test_b23(&self) -> bool {
        self.b23() != 0
    }

    #[doc="Sets the b23 field."]
    #[inline] pub fn set_b23<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="b24"]
    #[inline] pub fn b24(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if b24 != 0"]
    #[inline] pub fn test_b24(&self) -> bool {
        self.b24() != 0
    }

    #[doc="Sets the b24 field."]
    #[inline] pub fn set_b24<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="b25"]
    #[inline] pub fn b25(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if b25 != 0"]
    #[inline] pub fn test_b25(&self) -> bool {
        self.b25() != 0
    }

    #[doc="Sets the b25 field."]
    #[inline] pub fn set_b25<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="b26"]
    #[inline] pub fn b26(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if b26 != 0"]
    #[inline] pub fn test_b26(&self) -> bool {
        self.b26() != 0
    }

    #[doc="Sets the b26 field."]
    #[inline] pub fn set_b26<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="b27"]
    #[inline] pub fn b27(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if b27 != 0"]
    #[inline] pub fn test_b27(&self) -> bool {
        self.b27() != 0
    }

    #[doc="Sets the b27 field."]
    #[inline] pub fn set_b27<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="b28"]
    #[inline] pub fn b28(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if b28 != 0"]
    #[inline] pub fn test_b28(&self) -> bool {
        self.b28() != 0
    }

    #[doc="Sets the b28 field."]
    #[inline] pub fn set_b28<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="b29"]
    #[inline] pub fn b29(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if b29 != 0"]
    #[inline] pub fn test_b29(&self) -> bool {
        self.b29() != 0
    }

    #[doc="Sets the b29 field."]
    #[inline] pub fn set_b29<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="b30"]
    #[inline] pub fn b30(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if b30 != 0"]
    #[inline] pub fn test_b30(&self) -> bool {
        self.b30() != 0
    }

    #[doc="Sets the b30 field."]
    #[inline] pub fn set_b30<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="b31"]
    #[inline] pub fn b31(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if b31 != 0"]
    #[inline] pub fn test_b31(&self) -> bool {
        self.b31() != 0
    }

    #[doc="Sets the b31 field."]
    #[inline] pub fn set_b31<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for K3rr {
    #[inline]
    fn from(other: u32) -> Self {
         K3rr(other)
    }
}

impl ::core::fmt::Display for K3rr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for K3rr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.b0() != 0 { try!(write!(f, " b0"))}
        if self.b1() != 0 { try!(write!(f, " b1"))}
        if self.b2() != 0 { try!(write!(f, " b2"))}
        if self.b3() != 0 { try!(write!(f, " b3"))}
        if self.b4() != 0 { try!(write!(f, " b4"))}
        if self.b5() != 0 { try!(write!(f, " b5"))}
        if self.b6() != 0 { try!(write!(f, " b6"))}
        if self.b7() != 0 { try!(write!(f, " b7"))}
        if self.b8() != 0 { try!(write!(f, " b8"))}
        if self.b9() != 0 { try!(write!(f, " b9"))}
        if self.b10() != 0 { try!(write!(f, " b10"))}
        if self.b11() != 0 { try!(write!(f, " b11"))}
        if self.b12() != 0 { try!(write!(f, " b12"))}
        if self.b13() != 0 { try!(write!(f, " b13"))}
        if self.b14() != 0 { try!(write!(f, " b14"))}
        if self.b15() != 0 { try!(write!(f, " b15"))}
        if self.b16() != 0 { try!(write!(f, " b16"))}
        if self.b17() != 0 { try!(write!(f, " b17"))}
        if self.b18() != 0 { try!(write!(f, " b18"))}
        if self.b19() != 0 { try!(write!(f, " b19"))}
        if self.b20() != 0 { try!(write!(f, " b20"))}
        if self.b21() != 0 { try!(write!(f, " b21"))}
        if self.b22() != 0 { try!(write!(f, " b22"))}
        if self.b23() != 0 { try!(write!(f, " b23"))}
        if self.b24() != 0 { try!(write!(f, " b24"))}
        if self.b25() != 0 { try!(write!(f, " b25"))}
        if self.b26() != 0 { try!(write!(f, " b26"))}
        if self.b27() != 0 { try!(write!(f, " b27"))}
        if self.b28() != 0 { try!(write!(f, " b28"))}
        if self.b29() != 0 { try!(write!(f, " b29"))}
        if self.b30() != 0 { try!(write!(f, " b30"))}
        if self.b31() != 0 { try!(write!(f, " b31"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="initialization vector registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Iv0lr(pub u32);
impl Iv0lr {
    #[doc="IV31"]
    #[inline] pub fn iv31(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if IV31 != 0"]
    #[inline] pub fn test_iv31(&self) -> bool {
        self.iv31() != 0
    }

    #[doc="Sets the IV31 field."]
    #[inline] pub fn set_iv31<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="IV30"]
    #[inline] pub fn iv30(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if IV30 != 0"]
    #[inline] pub fn test_iv30(&self) -> bool {
        self.iv30() != 0
    }

    #[doc="Sets the IV30 field."]
    #[inline] pub fn set_iv30<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="IV29"]
    #[inline] pub fn iv29(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if IV29 != 0"]
    #[inline] pub fn test_iv29(&self) -> bool {
        self.iv29() != 0
    }

    #[doc="Sets the IV29 field."]
    #[inline] pub fn set_iv29<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="IV28"]
    #[inline] pub fn iv28(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if IV28 != 0"]
    #[inline] pub fn test_iv28(&self) -> bool {
        self.iv28() != 0
    }

    #[doc="Sets the IV28 field."]
    #[inline] pub fn set_iv28<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="IV27"]
    #[inline] pub fn iv27(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if IV27 != 0"]
    #[inline] pub fn test_iv27(&self) -> bool {
        self.iv27() != 0
    }

    #[doc="Sets the IV27 field."]
    #[inline] pub fn set_iv27<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="IV26"]
    #[inline] pub fn iv26(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if IV26 != 0"]
    #[inline] pub fn test_iv26(&self) -> bool {
        self.iv26() != 0
    }

    #[doc="Sets the IV26 field."]
    #[inline] pub fn set_iv26<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="IV25"]
    #[inline] pub fn iv25(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if IV25 != 0"]
    #[inline] pub fn test_iv25(&self) -> bool {
        self.iv25() != 0
    }

    #[doc="Sets the IV25 field."]
    #[inline] pub fn set_iv25<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="IV24"]
    #[inline] pub fn iv24(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if IV24 != 0"]
    #[inline] pub fn test_iv24(&self) -> bool {
        self.iv24() != 0
    }

    #[doc="Sets the IV24 field."]
    #[inline] pub fn set_iv24<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="IV23"]
    #[inline] pub fn iv23(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if IV23 != 0"]
    #[inline] pub fn test_iv23(&self) -> bool {
        self.iv23() != 0
    }

    #[doc="Sets the IV23 field."]
    #[inline] pub fn set_iv23<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="IV22"]
    #[inline] pub fn iv22(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if IV22 != 0"]
    #[inline] pub fn test_iv22(&self) -> bool {
        self.iv22() != 0
    }

    #[doc="Sets the IV22 field."]
    #[inline] pub fn set_iv22<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="IV21"]
    #[inline] pub fn iv21(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if IV21 != 0"]
    #[inline] pub fn test_iv21(&self) -> bool {
        self.iv21() != 0
    }

    #[doc="Sets the IV21 field."]
    #[inline] pub fn set_iv21<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="IV20"]
    #[inline] pub fn iv20(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if IV20 != 0"]
    #[inline] pub fn test_iv20(&self) -> bool {
        self.iv20() != 0
    }

    #[doc="Sets the IV20 field."]
    #[inline] pub fn set_iv20<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="IV19"]
    #[inline] pub fn iv19(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if IV19 != 0"]
    #[inline] pub fn test_iv19(&self) -> bool {
        self.iv19() != 0
    }

    #[doc="Sets the IV19 field."]
    #[inline] pub fn set_iv19<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="IV18"]
    #[inline] pub fn iv18(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if IV18 != 0"]
    #[inline] pub fn test_iv18(&self) -> bool {
        self.iv18() != 0
    }

    #[doc="Sets the IV18 field."]
    #[inline] pub fn set_iv18<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="IV17"]
    #[inline] pub fn iv17(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if IV17 != 0"]
    #[inline] pub fn test_iv17(&self) -> bool {
        self.iv17() != 0
    }

    #[doc="Sets the IV17 field."]
    #[inline] pub fn set_iv17<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="IV16"]
    #[inline] pub fn iv16(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if IV16 != 0"]
    #[inline] pub fn test_iv16(&self) -> bool {
        self.iv16() != 0
    }

    #[doc="Sets the IV16 field."]
    #[inline] pub fn set_iv16<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="IV15"]
    #[inline] pub fn iv15(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if IV15 != 0"]
    #[inline] pub fn test_iv15(&self) -> bool {
        self.iv15() != 0
    }

    #[doc="Sets the IV15 field."]
    #[inline] pub fn set_iv15<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="IV14"]
    #[inline] pub fn iv14(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if IV14 != 0"]
    #[inline] pub fn test_iv14(&self) -> bool {
        self.iv14() != 0
    }

    #[doc="Sets the IV14 field."]
    #[inline] pub fn set_iv14<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="IV13"]
    #[inline] pub fn iv13(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if IV13 != 0"]
    #[inline] pub fn test_iv13(&self) -> bool {
        self.iv13() != 0
    }

    #[doc="Sets the IV13 field."]
    #[inline] pub fn set_iv13<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="IV12"]
    #[inline] pub fn iv12(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if IV12 != 0"]
    #[inline] pub fn test_iv12(&self) -> bool {
        self.iv12() != 0
    }

    #[doc="Sets the IV12 field."]
    #[inline] pub fn set_iv12<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="IV11"]
    #[inline] pub fn iv11(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if IV11 != 0"]
    #[inline] pub fn test_iv11(&self) -> bool {
        self.iv11() != 0
    }

    #[doc="Sets the IV11 field."]
    #[inline] pub fn set_iv11<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="IV10"]
    #[inline] pub fn iv10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if IV10 != 0"]
    #[inline] pub fn test_iv10(&self) -> bool {
        self.iv10() != 0
    }

    #[doc="Sets the IV10 field."]
    #[inline] pub fn set_iv10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="IV9"]
    #[inline] pub fn iv9(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if IV9 != 0"]
    #[inline] pub fn test_iv9(&self) -> bool {
        self.iv9() != 0
    }

    #[doc="Sets the IV9 field."]
    #[inline] pub fn set_iv9<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="IV8"]
    #[inline] pub fn iv8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if IV8 != 0"]
    #[inline] pub fn test_iv8(&self) -> bool {
        self.iv8() != 0
    }

    #[doc="Sets the IV8 field."]
    #[inline] pub fn set_iv8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="IV7"]
    #[inline] pub fn iv7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if IV7 != 0"]
    #[inline] pub fn test_iv7(&self) -> bool {
        self.iv7() != 0
    }

    #[doc="Sets the IV7 field."]
    #[inline] pub fn set_iv7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="IV6"]
    #[inline] pub fn iv6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if IV6 != 0"]
    #[inline] pub fn test_iv6(&self) -> bool {
        self.iv6() != 0
    }

    #[doc="Sets the IV6 field."]
    #[inline] pub fn set_iv6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="IV5"]
    #[inline] pub fn iv5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if IV5 != 0"]
    #[inline] pub fn test_iv5(&self) -> bool {
        self.iv5() != 0
    }

    #[doc="Sets the IV5 field."]
    #[inline] pub fn set_iv5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="IV4"]
    #[inline] pub fn iv4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if IV4 != 0"]
    #[inline] pub fn test_iv4(&self) -> bool {
        self.iv4() != 0
    }

    #[doc="Sets the IV4 field."]
    #[inline] pub fn set_iv4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="IV3"]
    #[inline] pub fn iv3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if IV3 != 0"]
    #[inline] pub fn test_iv3(&self) -> bool {
        self.iv3() != 0
    }

    #[doc="Sets the IV3 field."]
    #[inline] pub fn set_iv3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="IV2"]
    #[inline] pub fn iv2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if IV2 != 0"]
    #[inline] pub fn test_iv2(&self) -> bool {
        self.iv2() != 0
    }

    #[doc="Sets the IV2 field."]
    #[inline] pub fn set_iv2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="IV1"]
    #[inline] pub fn iv1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if IV1 != 0"]
    #[inline] pub fn test_iv1(&self) -> bool {
        self.iv1() != 0
    }

    #[doc="Sets the IV1 field."]
    #[inline] pub fn set_iv1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="IV0"]
    #[inline] pub fn iv0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if IV0 != 0"]
    #[inline] pub fn test_iv0(&self) -> bool {
        self.iv0() != 0
    }

    #[doc="Sets the IV0 field."]
    #[inline] pub fn set_iv0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Iv0lr {
    #[inline]
    fn from(other: u32) -> Self {
         Iv0lr(other)
    }
}

impl ::core::fmt::Display for Iv0lr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Iv0lr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.iv31() != 0 { try!(write!(f, " iv31"))}
        if self.iv30() != 0 { try!(write!(f, " iv30"))}
        if self.iv29() != 0 { try!(write!(f, " iv29"))}
        if self.iv28() != 0 { try!(write!(f, " iv28"))}
        if self.iv27() != 0 { try!(write!(f, " iv27"))}
        if self.iv26() != 0 { try!(write!(f, " iv26"))}
        if self.iv25() != 0 { try!(write!(f, " iv25"))}
        if self.iv24() != 0 { try!(write!(f, " iv24"))}
        if self.iv23() != 0 { try!(write!(f, " iv23"))}
        if self.iv22() != 0 { try!(write!(f, " iv22"))}
        if self.iv21() != 0 { try!(write!(f, " iv21"))}
        if self.iv20() != 0 { try!(write!(f, " iv20"))}
        if self.iv19() != 0 { try!(write!(f, " iv19"))}
        if self.iv18() != 0 { try!(write!(f, " iv18"))}
        if self.iv17() != 0 { try!(write!(f, " iv17"))}
        if self.iv16() != 0 { try!(write!(f, " iv16"))}
        if self.iv15() != 0 { try!(write!(f, " iv15"))}
        if self.iv14() != 0 { try!(write!(f, " iv14"))}
        if self.iv13() != 0 { try!(write!(f, " iv13"))}
        if self.iv12() != 0 { try!(write!(f, " iv12"))}
        if self.iv11() != 0 { try!(write!(f, " iv11"))}
        if self.iv10() != 0 { try!(write!(f, " iv10"))}
        if self.iv9() != 0 { try!(write!(f, " iv9"))}
        if self.iv8() != 0 { try!(write!(f, " iv8"))}
        if self.iv7() != 0 { try!(write!(f, " iv7"))}
        if self.iv6() != 0 { try!(write!(f, " iv6"))}
        if self.iv5() != 0 { try!(write!(f, " iv5"))}
        if self.iv4() != 0 { try!(write!(f, " iv4"))}
        if self.iv3() != 0 { try!(write!(f, " iv3"))}
        if self.iv2() != 0 { try!(write!(f, " iv2"))}
        if self.iv1() != 0 { try!(write!(f, " iv1"))}
        if self.iv0() != 0 { try!(write!(f, " iv0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="initialization vector registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Iv0rr(pub u32);
impl Iv0rr {
    #[doc="IV63"]
    #[inline] pub fn iv63(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if IV63 != 0"]
    #[inline] pub fn test_iv63(&self) -> bool {
        self.iv63() != 0
    }

    #[doc="Sets the IV63 field."]
    #[inline] pub fn set_iv63<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="IV62"]
    #[inline] pub fn iv62(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if IV62 != 0"]
    #[inline] pub fn test_iv62(&self) -> bool {
        self.iv62() != 0
    }

    #[doc="Sets the IV62 field."]
    #[inline] pub fn set_iv62<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="IV61"]
    #[inline] pub fn iv61(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if IV61 != 0"]
    #[inline] pub fn test_iv61(&self) -> bool {
        self.iv61() != 0
    }

    #[doc="Sets the IV61 field."]
    #[inline] pub fn set_iv61<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="IV60"]
    #[inline] pub fn iv60(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if IV60 != 0"]
    #[inline] pub fn test_iv60(&self) -> bool {
        self.iv60() != 0
    }

    #[doc="Sets the IV60 field."]
    #[inline] pub fn set_iv60<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="IV59"]
    #[inline] pub fn iv59(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if IV59 != 0"]
    #[inline] pub fn test_iv59(&self) -> bool {
        self.iv59() != 0
    }

    #[doc="Sets the IV59 field."]
    #[inline] pub fn set_iv59<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="IV58"]
    #[inline] pub fn iv58(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if IV58 != 0"]
    #[inline] pub fn test_iv58(&self) -> bool {
        self.iv58() != 0
    }

    #[doc="Sets the IV58 field."]
    #[inline] pub fn set_iv58<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="IV57"]
    #[inline] pub fn iv57(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if IV57 != 0"]
    #[inline] pub fn test_iv57(&self) -> bool {
        self.iv57() != 0
    }

    #[doc="Sets the IV57 field."]
    #[inline] pub fn set_iv57<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="IV56"]
    #[inline] pub fn iv56(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if IV56 != 0"]
    #[inline] pub fn test_iv56(&self) -> bool {
        self.iv56() != 0
    }

    #[doc="Sets the IV56 field."]
    #[inline] pub fn set_iv56<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="IV55"]
    #[inline] pub fn iv55(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if IV55 != 0"]
    #[inline] pub fn test_iv55(&self) -> bool {
        self.iv55() != 0
    }

    #[doc="Sets the IV55 field."]
    #[inline] pub fn set_iv55<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="IV54"]
    #[inline] pub fn iv54(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if IV54 != 0"]
    #[inline] pub fn test_iv54(&self) -> bool {
        self.iv54() != 0
    }

    #[doc="Sets the IV54 field."]
    #[inline] pub fn set_iv54<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="IV53"]
    #[inline] pub fn iv53(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if IV53 != 0"]
    #[inline] pub fn test_iv53(&self) -> bool {
        self.iv53() != 0
    }

    #[doc="Sets the IV53 field."]
    #[inline] pub fn set_iv53<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="IV52"]
    #[inline] pub fn iv52(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if IV52 != 0"]
    #[inline] pub fn test_iv52(&self) -> bool {
        self.iv52() != 0
    }

    #[doc="Sets the IV52 field."]
    #[inline] pub fn set_iv52<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="IV51"]
    #[inline] pub fn iv51(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if IV51 != 0"]
    #[inline] pub fn test_iv51(&self) -> bool {
        self.iv51() != 0
    }

    #[doc="Sets the IV51 field."]
    #[inline] pub fn set_iv51<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="IV50"]
    #[inline] pub fn iv50(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if IV50 != 0"]
    #[inline] pub fn test_iv50(&self) -> bool {
        self.iv50() != 0
    }

    #[doc="Sets the IV50 field."]
    #[inline] pub fn set_iv50<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="IV49"]
    #[inline] pub fn iv49(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if IV49 != 0"]
    #[inline] pub fn test_iv49(&self) -> bool {
        self.iv49() != 0
    }

    #[doc="Sets the IV49 field."]
    #[inline] pub fn set_iv49<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="IV48"]
    #[inline] pub fn iv48(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if IV48 != 0"]
    #[inline] pub fn test_iv48(&self) -> bool {
        self.iv48() != 0
    }

    #[doc="Sets the IV48 field."]
    #[inline] pub fn set_iv48<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="IV47"]
    #[inline] pub fn iv47(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if IV47 != 0"]
    #[inline] pub fn test_iv47(&self) -> bool {
        self.iv47() != 0
    }

    #[doc="Sets the IV47 field."]
    #[inline] pub fn set_iv47<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="IV46"]
    #[inline] pub fn iv46(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if IV46 != 0"]
    #[inline] pub fn test_iv46(&self) -> bool {
        self.iv46() != 0
    }

    #[doc="Sets the IV46 field."]
    #[inline] pub fn set_iv46<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="IV45"]
    #[inline] pub fn iv45(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if IV45 != 0"]
    #[inline] pub fn test_iv45(&self) -> bool {
        self.iv45() != 0
    }

    #[doc="Sets the IV45 field."]
    #[inline] pub fn set_iv45<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="IV44"]
    #[inline] pub fn iv44(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if IV44 != 0"]
    #[inline] pub fn test_iv44(&self) -> bool {
        self.iv44() != 0
    }

    #[doc="Sets the IV44 field."]
    #[inline] pub fn set_iv44<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="IV43"]
    #[inline] pub fn iv43(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if IV43 != 0"]
    #[inline] pub fn test_iv43(&self) -> bool {
        self.iv43() != 0
    }

    #[doc="Sets the IV43 field."]
    #[inline] pub fn set_iv43<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="IV42"]
    #[inline] pub fn iv42(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if IV42 != 0"]
    #[inline] pub fn test_iv42(&self) -> bool {
        self.iv42() != 0
    }

    #[doc="Sets the IV42 field."]
    #[inline] pub fn set_iv42<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="IV41"]
    #[inline] pub fn iv41(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if IV41 != 0"]
    #[inline] pub fn test_iv41(&self) -> bool {
        self.iv41() != 0
    }

    #[doc="Sets the IV41 field."]
    #[inline] pub fn set_iv41<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="IV40"]
    #[inline] pub fn iv40(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if IV40 != 0"]
    #[inline] pub fn test_iv40(&self) -> bool {
        self.iv40() != 0
    }

    #[doc="Sets the IV40 field."]
    #[inline] pub fn set_iv40<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="IV39"]
    #[inline] pub fn iv39(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if IV39 != 0"]
    #[inline] pub fn test_iv39(&self) -> bool {
        self.iv39() != 0
    }

    #[doc="Sets the IV39 field."]
    #[inline] pub fn set_iv39<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="IV38"]
    #[inline] pub fn iv38(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if IV38 != 0"]
    #[inline] pub fn test_iv38(&self) -> bool {
        self.iv38() != 0
    }

    #[doc="Sets the IV38 field."]
    #[inline] pub fn set_iv38<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="IV37"]
    #[inline] pub fn iv37(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if IV37 != 0"]
    #[inline] pub fn test_iv37(&self) -> bool {
        self.iv37() != 0
    }

    #[doc="Sets the IV37 field."]
    #[inline] pub fn set_iv37<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="IV36"]
    #[inline] pub fn iv36(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if IV36 != 0"]
    #[inline] pub fn test_iv36(&self) -> bool {
        self.iv36() != 0
    }

    #[doc="Sets the IV36 field."]
    #[inline] pub fn set_iv36<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="IV35"]
    #[inline] pub fn iv35(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if IV35 != 0"]
    #[inline] pub fn test_iv35(&self) -> bool {
        self.iv35() != 0
    }

    #[doc="Sets the IV35 field."]
    #[inline] pub fn set_iv35<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="IV34"]
    #[inline] pub fn iv34(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if IV34 != 0"]
    #[inline] pub fn test_iv34(&self) -> bool {
        self.iv34() != 0
    }

    #[doc="Sets the IV34 field."]
    #[inline] pub fn set_iv34<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="IV33"]
    #[inline] pub fn iv33(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if IV33 != 0"]
    #[inline] pub fn test_iv33(&self) -> bool {
        self.iv33() != 0
    }

    #[doc="Sets the IV33 field."]
    #[inline] pub fn set_iv33<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="IV32"]
    #[inline] pub fn iv32(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if IV32 != 0"]
    #[inline] pub fn test_iv32(&self) -> bool {
        self.iv32() != 0
    }

    #[doc="Sets the IV32 field."]
    #[inline] pub fn set_iv32<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Iv0rr {
    #[inline]
    fn from(other: u32) -> Self {
         Iv0rr(other)
    }
}

impl ::core::fmt::Display for Iv0rr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Iv0rr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.iv63() != 0 { try!(write!(f, " iv63"))}
        if self.iv62() != 0 { try!(write!(f, " iv62"))}
        if self.iv61() != 0 { try!(write!(f, " iv61"))}
        if self.iv60() != 0 { try!(write!(f, " iv60"))}
        if self.iv59() != 0 { try!(write!(f, " iv59"))}
        if self.iv58() != 0 { try!(write!(f, " iv58"))}
        if self.iv57() != 0 { try!(write!(f, " iv57"))}
        if self.iv56() != 0 { try!(write!(f, " iv56"))}
        if self.iv55() != 0 { try!(write!(f, " iv55"))}
        if self.iv54() != 0 { try!(write!(f, " iv54"))}
        if self.iv53() != 0 { try!(write!(f, " iv53"))}
        if self.iv52() != 0 { try!(write!(f, " iv52"))}
        if self.iv51() != 0 { try!(write!(f, " iv51"))}
        if self.iv50() != 0 { try!(write!(f, " iv50"))}
        if self.iv49() != 0 { try!(write!(f, " iv49"))}
        if self.iv48() != 0 { try!(write!(f, " iv48"))}
        if self.iv47() != 0 { try!(write!(f, " iv47"))}
        if self.iv46() != 0 { try!(write!(f, " iv46"))}
        if self.iv45() != 0 { try!(write!(f, " iv45"))}
        if self.iv44() != 0 { try!(write!(f, " iv44"))}
        if self.iv43() != 0 { try!(write!(f, " iv43"))}
        if self.iv42() != 0 { try!(write!(f, " iv42"))}
        if self.iv41() != 0 { try!(write!(f, " iv41"))}
        if self.iv40() != 0 { try!(write!(f, " iv40"))}
        if self.iv39() != 0 { try!(write!(f, " iv39"))}
        if self.iv38() != 0 { try!(write!(f, " iv38"))}
        if self.iv37() != 0 { try!(write!(f, " iv37"))}
        if self.iv36() != 0 { try!(write!(f, " iv36"))}
        if self.iv35() != 0 { try!(write!(f, " iv35"))}
        if self.iv34() != 0 { try!(write!(f, " iv34"))}
        if self.iv33() != 0 { try!(write!(f, " iv33"))}
        if self.iv32() != 0 { try!(write!(f, " iv32"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="initialization vector registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Iv1lr(pub u32);
impl Iv1lr {
    #[doc="IV95"]
    #[inline] pub fn iv95(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if IV95 != 0"]
    #[inline] pub fn test_iv95(&self) -> bool {
        self.iv95() != 0
    }

    #[doc="Sets the IV95 field."]
    #[inline] pub fn set_iv95<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="IV94"]
    #[inline] pub fn iv94(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if IV94 != 0"]
    #[inline] pub fn test_iv94(&self) -> bool {
        self.iv94() != 0
    }

    #[doc="Sets the IV94 field."]
    #[inline] pub fn set_iv94<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="IV93"]
    #[inline] pub fn iv93(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if IV93 != 0"]
    #[inline] pub fn test_iv93(&self) -> bool {
        self.iv93() != 0
    }

    #[doc="Sets the IV93 field."]
    #[inline] pub fn set_iv93<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="IV92"]
    #[inline] pub fn iv92(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if IV92 != 0"]
    #[inline] pub fn test_iv92(&self) -> bool {
        self.iv92() != 0
    }

    #[doc="Sets the IV92 field."]
    #[inline] pub fn set_iv92<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="IV91"]
    #[inline] pub fn iv91(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if IV91 != 0"]
    #[inline] pub fn test_iv91(&self) -> bool {
        self.iv91() != 0
    }

    #[doc="Sets the IV91 field."]
    #[inline] pub fn set_iv91<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="IV90"]
    #[inline] pub fn iv90(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if IV90 != 0"]
    #[inline] pub fn test_iv90(&self) -> bool {
        self.iv90() != 0
    }

    #[doc="Sets the IV90 field."]
    #[inline] pub fn set_iv90<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="IV89"]
    #[inline] pub fn iv89(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if IV89 != 0"]
    #[inline] pub fn test_iv89(&self) -> bool {
        self.iv89() != 0
    }

    #[doc="Sets the IV89 field."]
    #[inline] pub fn set_iv89<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="IV88"]
    #[inline] pub fn iv88(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if IV88 != 0"]
    #[inline] pub fn test_iv88(&self) -> bool {
        self.iv88() != 0
    }

    #[doc="Sets the IV88 field."]
    #[inline] pub fn set_iv88<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="IV87"]
    #[inline] pub fn iv87(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if IV87 != 0"]
    #[inline] pub fn test_iv87(&self) -> bool {
        self.iv87() != 0
    }

    #[doc="Sets the IV87 field."]
    #[inline] pub fn set_iv87<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="IV86"]
    #[inline] pub fn iv86(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if IV86 != 0"]
    #[inline] pub fn test_iv86(&self) -> bool {
        self.iv86() != 0
    }

    #[doc="Sets the IV86 field."]
    #[inline] pub fn set_iv86<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="IV85"]
    #[inline] pub fn iv85(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if IV85 != 0"]
    #[inline] pub fn test_iv85(&self) -> bool {
        self.iv85() != 0
    }

    #[doc="Sets the IV85 field."]
    #[inline] pub fn set_iv85<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="IV84"]
    #[inline] pub fn iv84(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if IV84 != 0"]
    #[inline] pub fn test_iv84(&self) -> bool {
        self.iv84() != 0
    }

    #[doc="Sets the IV84 field."]
    #[inline] pub fn set_iv84<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="IV83"]
    #[inline] pub fn iv83(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if IV83 != 0"]
    #[inline] pub fn test_iv83(&self) -> bool {
        self.iv83() != 0
    }

    #[doc="Sets the IV83 field."]
    #[inline] pub fn set_iv83<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="IV82"]
    #[inline] pub fn iv82(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if IV82 != 0"]
    #[inline] pub fn test_iv82(&self) -> bool {
        self.iv82() != 0
    }

    #[doc="Sets the IV82 field."]
    #[inline] pub fn set_iv82<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="IV81"]
    #[inline] pub fn iv81(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if IV81 != 0"]
    #[inline] pub fn test_iv81(&self) -> bool {
        self.iv81() != 0
    }

    #[doc="Sets the IV81 field."]
    #[inline] pub fn set_iv81<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="IV80"]
    #[inline] pub fn iv80(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if IV80 != 0"]
    #[inline] pub fn test_iv80(&self) -> bool {
        self.iv80() != 0
    }

    #[doc="Sets the IV80 field."]
    #[inline] pub fn set_iv80<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="IV79"]
    #[inline] pub fn iv79(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if IV79 != 0"]
    #[inline] pub fn test_iv79(&self) -> bool {
        self.iv79() != 0
    }

    #[doc="Sets the IV79 field."]
    #[inline] pub fn set_iv79<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="IV78"]
    #[inline] pub fn iv78(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if IV78 != 0"]
    #[inline] pub fn test_iv78(&self) -> bool {
        self.iv78() != 0
    }

    #[doc="Sets the IV78 field."]
    #[inline] pub fn set_iv78<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="IV77"]
    #[inline] pub fn iv77(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if IV77 != 0"]
    #[inline] pub fn test_iv77(&self) -> bool {
        self.iv77() != 0
    }

    #[doc="Sets the IV77 field."]
    #[inline] pub fn set_iv77<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="IV76"]
    #[inline] pub fn iv76(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if IV76 != 0"]
    #[inline] pub fn test_iv76(&self) -> bool {
        self.iv76() != 0
    }

    #[doc="Sets the IV76 field."]
    #[inline] pub fn set_iv76<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="IV75"]
    #[inline] pub fn iv75(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if IV75 != 0"]
    #[inline] pub fn test_iv75(&self) -> bool {
        self.iv75() != 0
    }

    #[doc="Sets the IV75 field."]
    #[inline] pub fn set_iv75<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="IV74"]
    #[inline] pub fn iv74(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if IV74 != 0"]
    #[inline] pub fn test_iv74(&self) -> bool {
        self.iv74() != 0
    }

    #[doc="Sets the IV74 field."]
    #[inline] pub fn set_iv74<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="IV73"]
    #[inline] pub fn iv73(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if IV73 != 0"]
    #[inline] pub fn test_iv73(&self) -> bool {
        self.iv73() != 0
    }

    #[doc="Sets the IV73 field."]
    #[inline] pub fn set_iv73<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="IV72"]
    #[inline] pub fn iv72(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if IV72 != 0"]
    #[inline] pub fn test_iv72(&self) -> bool {
        self.iv72() != 0
    }

    #[doc="Sets the IV72 field."]
    #[inline] pub fn set_iv72<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="IV71"]
    #[inline] pub fn iv71(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if IV71 != 0"]
    #[inline] pub fn test_iv71(&self) -> bool {
        self.iv71() != 0
    }

    #[doc="Sets the IV71 field."]
    #[inline] pub fn set_iv71<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="IV70"]
    #[inline] pub fn iv70(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if IV70 != 0"]
    #[inline] pub fn test_iv70(&self) -> bool {
        self.iv70() != 0
    }

    #[doc="Sets the IV70 field."]
    #[inline] pub fn set_iv70<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="IV69"]
    #[inline] pub fn iv69(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if IV69 != 0"]
    #[inline] pub fn test_iv69(&self) -> bool {
        self.iv69() != 0
    }

    #[doc="Sets the IV69 field."]
    #[inline] pub fn set_iv69<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="IV68"]
    #[inline] pub fn iv68(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if IV68 != 0"]
    #[inline] pub fn test_iv68(&self) -> bool {
        self.iv68() != 0
    }

    #[doc="Sets the IV68 field."]
    #[inline] pub fn set_iv68<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="IV67"]
    #[inline] pub fn iv67(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if IV67 != 0"]
    #[inline] pub fn test_iv67(&self) -> bool {
        self.iv67() != 0
    }

    #[doc="Sets the IV67 field."]
    #[inline] pub fn set_iv67<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="IV66"]
    #[inline] pub fn iv66(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if IV66 != 0"]
    #[inline] pub fn test_iv66(&self) -> bool {
        self.iv66() != 0
    }

    #[doc="Sets the IV66 field."]
    #[inline] pub fn set_iv66<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="IV65"]
    #[inline] pub fn iv65(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if IV65 != 0"]
    #[inline] pub fn test_iv65(&self) -> bool {
        self.iv65() != 0
    }

    #[doc="Sets the IV65 field."]
    #[inline] pub fn set_iv65<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="IV64"]
    #[inline] pub fn iv64(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if IV64 != 0"]
    #[inline] pub fn test_iv64(&self) -> bool {
        self.iv64() != 0
    }

    #[doc="Sets the IV64 field."]
    #[inline] pub fn set_iv64<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Iv1lr {
    #[inline]
    fn from(other: u32) -> Self {
         Iv1lr(other)
    }
}

impl ::core::fmt::Display for Iv1lr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Iv1lr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.iv95() != 0 { try!(write!(f, " iv95"))}
        if self.iv94() != 0 { try!(write!(f, " iv94"))}
        if self.iv93() != 0 { try!(write!(f, " iv93"))}
        if self.iv92() != 0 { try!(write!(f, " iv92"))}
        if self.iv91() != 0 { try!(write!(f, " iv91"))}
        if self.iv90() != 0 { try!(write!(f, " iv90"))}
        if self.iv89() != 0 { try!(write!(f, " iv89"))}
        if self.iv88() != 0 { try!(write!(f, " iv88"))}
        if self.iv87() != 0 { try!(write!(f, " iv87"))}
        if self.iv86() != 0 { try!(write!(f, " iv86"))}
        if self.iv85() != 0 { try!(write!(f, " iv85"))}
        if self.iv84() != 0 { try!(write!(f, " iv84"))}
        if self.iv83() != 0 { try!(write!(f, " iv83"))}
        if self.iv82() != 0 { try!(write!(f, " iv82"))}
        if self.iv81() != 0 { try!(write!(f, " iv81"))}
        if self.iv80() != 0 { try!(write!(f, " iv80"))}
        if self.iv79() != 0 { try!(write!(f, " iv79"))}
        if self.iv78() != 0 { try!(write!(f, " iv78"))}
        if self.iv77() != 0 { try!(write!(f, " iv77"))}
        if self.iv76() != 0 { try!(write!(f, " iv76"))}
        if self.iv75() != 0 { try!(write!(f, " iv75"))}
        if self.iv74() != 0 { try!(write!(f, " iv74"))}
        if self.iv73() != 0 { try!(write!(f, " iv73"))}
        if self.iv72() != 0 { try!(write!(f, " iv72"))}
        if self.iv71() != 0 { try!(write!(f, " iv71"))}
        if self.iv70() != 0 { try!(write!(f, " iv70"))}
        if self.iv69() != 0 { try!(write!(f, " iv69"))}
        if self.iv68() != 0 { try!(write!(f, " iv68"))}
        if self.iv67() != 0 { try!(write!(f, " iv67"))}
        if self.iv66() != 0 { try!(write!(f, " iv66"))}
        if self.iv65() != 0 { try!(write!(f, " iv65"))}
        if self.iv64() != 0 { try!(write!(f, " iv64"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="initialization vector registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Iv1rr(pub u32);
impl Iv1rr {
    #[doc="IV127"]
    #[inline] pub fn iv127(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if IV127 != 0"]
    #[inline] pub fn test_iv127(&self) -> bool {
        self.iv127() != 0
    }

    #[doc="Sets the IV127 field."]
    #[inline] pub fn set_iv127<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="IV126"]
    #[inline] pub fn iv126(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if IV126 != 0"]
    #[inline] pub fn test_iv126(&self) -> bool {
        self.iv126() != 0
    }

    #[doc="Sets the IV126 field."]
    #[inline] pub fn set_iv126<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="IV125"]
    #[inline] pub fn iv125(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if IV125 != 0"]
    #[inline] pub fn test_iv125(&self) -> bool {
        self.iv125() != 0
    }

    #[doc="Sets the IV125 field."]
    #[inline] pub fn set_iv125<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="IV124"]
    #[inline] pub fn iv124(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if IV124 != 0"]
    #[inline] pub fn test_iv124(&self) -> bool {
        self.iv124() != 0
    }

    #[doc="Sets the IV124 field."]
    #[inline] pub fn set_iv124<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="IV123"]
    #[inline] pub fn iv123(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if IV123 != 0"]
    #[inline] pub fn test_iv123(&self) -> bool {
        self.iv123() != 0
    }

    #[doc="Sets the IV123 field."]
    #[inline] pub fn set_iv123<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="IV122"]
    #[inline] pub fn iv122(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if IV122 != 0"]
    #[inline] pub fn test_iv122(&self) -> bool {
        self.iv122() != 0
    }

    #[doc="Sets the IV122 field."]
    #[inline] pub fn set_iv122<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="IV121"]
    #[inline] pub fn iv121(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if IV121 != 0"]
    #[inline] pub fn test_iv121(&self) -> bool {
        self.iv121() != 0
    }

    #[doc="Sets the IV121 field."]
    #[inline] pub fn set_iv121<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="IV120"]
    #[inline] pub fn iv120(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if IV120 != 0"]
    #[inline] pub fn test_iv120(&self) -> bool {
        self.iv120() != 0
    }

    #[doc="Sets the IV120 field."]
    #[inline] pub fn set_iv120<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="IV119"]
    #[inline] pub fn iv119(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if IV119 != 0"]
    #[inline] pub fn test_iv119(&self) -> bool {
        self.iv119() != 0
    }

    #[doc="Sets the IV119 field."]
    #[inline] pub fn set_iv119<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="IV118"]
    #[inline] pub fn iv118(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if IV118 != 0"]
    #[inline] pub fn test_iv118(&self) -> bool {
        self.iv118() != 0
    }

    #[doc="Sets the IV118 field."]
    #[inline] pub fn set_iv118<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="IV117"]
    #[inline] pub fn iv117(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if IV117 != 0"]
    #[inline] pub fn test_iv117(&self) -> bool {
        self.iv117() != 0
    }

    #[doc="Sets the IV117 field."]
    #[inline] pub fn set_iv117<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="IV116"]
    #[inline] pub fn iv116(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if IV116 != 0"]
    #[inline] pub fn test_iv116(&self) -> bool {
        self.iv116() != 0
    }

    #[doc="Sets the IV116 field."]
    #[inline] pub fn set_iv116<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="IV115"]
    #[inline] pub fn iv115(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if IV115 != 0"]
    #[inline] pub fn test_iv115(&self) -> bool {
        self.iv115() != 0
    }

    #[doc="Sets the IV115 field."]
    #[inline] pub fn set_iv115<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="IV114"]
    #[inline] pub fn iv114(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if IV114 != 0"]
    #[inline] pub fn test_iv114(&self) -> bool {
        self.iv114() != 0
    }

    #[doc="Sets the IV114 field."]
    #[inline] pub fn set_iv114<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="IV113"]
    #[inline] pub fn iv113(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if IV113 != 0"]
    #[inline] pub fn test_iv113(&self) -> bool {
        self.iv113() != 0
    }

    #[doc="Sets the IV113 field."]
    #[inline] pub fn set_iv113<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="IV112"]
    #[inline] pub fn iv112(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if IV112 != 0"]
    #[inline] pub fn test_iv112(&self) -> bool {
        self.iv112() != 0
    }

    #[doc="Sets the IV112 field."]
    #[inline] pub fn set_iv112<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="IV111"]
    #[inline] pub fn iv111(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if IV111 != 0"]
    #[inline] pub fn test_iv111(&self) -> bool {
        self.iv111() != 0
    }

    #[doc="Sets the IV111 field."]
    #[inline] pub fn set_iv111<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="IV110"]
    #[inline] pub fn iv110(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if IV110 != 0"]
    #[inline] pub fn test_iv110(&self) -> bool {
        self.iv110() != 0
    }

    #[doc="Sets the IV110 field."]
    #[inline] pub fn set_iv110<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="IV109"]
    #[inline] pub fn iv109(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if IV109 != 0"]
    #[inline] pub fn test_iv109(&self) -> bool {
        self.iv109() != 0
    }

    #[doc="Sets the IV109 field."]
    #[inline] pub fn set_iv109<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="IV108"]
    #[inline] pub fn iv108(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if IV108 != 0"]
    #[inline] pub fn test_iv108(&self) -> bool {
        self.iv108() != 0
    }

    #[doc="Sets the IV108 field."]
    #[inline] pub fn set_iv108<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="IV107"]
    #[inline] pub fn iv107(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if IV107 != 0"]
    #[inline] pub fn test_iv107(&self) -> bool {
        self.iv107() != 0
    }

    #[doc="Sets the IV107 field."]
    #[inline] pub fn set_iv107<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="IV106"]
    #[inline] pub fn iv106(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if IV106 != 0"]
    #[inline] pub fn test_iv106(&self) -> bool {
        self.iv106() != 0
    }

    #[doc="Sets the IV106 field."]
    #[inline] pub fn set_iv106<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="IV105"]
    #[inline] pub fn iv105(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if IV105 != 0"]
    #[inline] pub fn test_iv105(&self) -> bool {
        self.iv105() != 0
    }

    #[doc="Sets the IV105 field."]
    #[inline] pub fn set_iv105<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="IV104"]
    #[inline] pub fn iv104(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if IV104 != 0"]
    #[inline] pub fn test_iv104(&self) -> bool {
        self.iv104() != 0
    }

    #[doc="Sets the IV104 field."]
    #[inline] pub fn set_iv104<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="IV103"]
    #[inline] pub fn iv103(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if IV103 != 0"]
    #[inline] pub fn test_iv103(&self) -> bool {
        self.iv103() != 0
    }

    #[doc="Sets the IV103 field."]
    #[inline] pub fn set_iv103<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="IV102"]
    #[inline] pub fn iv102(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if IV102 != 0"]
    #[inline] pub fn test_iv102(&self) -> bool {
        self.iv102() != 0
    }

    #[doc="Sets the IV102 field."]
    #[inline] pub fn set_iv102<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="IV101"]
    #[inline] pub fn iv101(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if IV101 != 0"]
    #[inline] pub fn test_iv101(&self) -> bool {
        self.iv101() != 0
    }

    #[doc="Sets the IV101 field."]
    #[inline] pub fn set_iv101<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="IV100"]
    #[inline] pub fn iv100(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if IV100 != 0"]
    #[inline] pub fn test_iv100(&self) -> bool {
        self.iv100() != 0
    }

    #[doc="Sets the IV100 field."]
    #[inline] pub fn set_iv100<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="IV99"]
    #[inline] pub fn iv99(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if IV99 != 0"]
    #[inline] pub fn test_iv99(&self) -> bool {
        self.iv99() != 0
    }

    #[doc="Sets the IV99 field."]
    #[inline] pub fn set_iv99<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="IV98"]
    #[inline] pub fn iv98(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if IV98 != 0"]
    #[inline] pub fn test_iv98(&self) -> bool {
        self.iv98() != 0
    }

    #[doc="Sets the IV98 field."]
    #[inline] pub fn set_iv98<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="IV97"]
    #[inline] pub fn iv97(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if IV97 != 0"]
    #[inline] pub fn test_iv97(&self) -> bool {
        self.iv97() != 0
    }

    #[doc="Sets the IV97 field."]
    #[inline] pub fn set_iv97<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="IV96"]
    #[inline] pub fn iv96(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if IV96 != 0"]
    #[inline] pub fn test_iv96(&self) -> bool {
        self.iv96() != 0
    }

    #[doc="Sets the IV96 field."]
    #[inline] pub fn set_iv96<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Iv1rr {
    #[inline]
    fn from(other: u32) -> Self {
         Iv1rr(other)
    }
}

impl ::core::fmt::Display for Iv1rr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Iv1rr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.iv127() != 0 { try!(write!(f, " iv127"))}
        if self.iv126() != 0 { try!(write!(f, " iv126"))}
        if self.iv125() != 0 { try!(write!(f, " iv125"))}
        if self.iv124() != 0 { try!(write!(f, " iv124"))}
        if self.iv123() != 0 { try!(write!(f, " iv123"))}
        if self.iv122() != 0 { try!(write!(f, " iv122"))}
        if self.iv121() != 0 { try!(write!(f, " iv121"))}
        if self.iv120() != 0 { try!(write!(f, " iv120"))}
        if self.iv119() != 0 { try!(write!(f, " iv119"))}
        if self.iv118() != 0 { try!(write!(f, " iv118"))}
        if self.iv117() != 0 { try!(write!(f, " iv117"))}
        if self.iv116() != 0 { try!(write!(f, " iv116"))}
        if self.iv115() != 0 { try!(write!(f, " iv115"))}
        if self.iv114() != 0 { try!(write!(f, " iv114"))}
        if self.iv113() != 0 { try!(write!(f, " iv113"))}
        if self.iv112() != 0 { try!(write!(f, " iv112"))}
        if self.iv111() != 0 { try!(write!(f, " iv111"))}
        if self.iv110() != 0 { try!(write!(f, " iv110"))}
        if self.iv109() != 0 { try!(write!(f, " iv109"))}
        if self.iv108() != 0 { try!(write!(f, " iv108"))}
        if self.iv107() != 0 { try!(write!(f, " iv107"))}
        if self.iv106() != 0 { try!(write!(f, " iv106"))}
        if self.iv105() != 0 { try!(write!(f, " iv105"))}
        if self.iv104() != 0 { try!(write!(f, " iv104"))}
        if self.iv103() != 0 { try!(write!(f, " iv103"))}
        if self.iv102() != 0 { try!(write!(f, " iv102"))}
        if self.iv101() != 0 { try!(write!(f, " iv101"))}
        if self.iv100() != 0 { try!(write!(f, " iv100"))}
        if self.iv99() != 0 { try!(write!(f, " iv99"))}
        if self.iv98() != 0 { try!(write!(f, " iv98"))}
        if self.iv97() != 0 { try!(write!(f, " iv97"))}
        if self.iv96() != 0 { try!(write!(f, " iv96"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csgcmccm0r(pub u32);
impl Csgcmccm0r {
    #[doc="CSGCMCCM0R"]
    #[inline] pub fn csgcmccm0r(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSGCMCCM0R != 0"]
    #[inline] pub fn test_csgcmccm0r(&self) -> bool {
        self.csgcmccm0r() != 0
    }

    #[doc="Sets the CSGCMCCM0R field."]
    #[inline] pub fn set_csgcmccm0r<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csgcmccm0r {
    #[inline]
    fn from(other: u32) -> Self {
         Csgcmccm0r(other)
    }
}

impl ::core::fmt::Display for Csgcmccm0r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csgcmccm0r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csgcmccm1r(pub u32);
impl Csgcmccm1r {
    #[doc="CSGCMCCM1R"]
    #[inline] pub fn csgcmccm1r(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSGCMCCM1R != 0"]
    #[inline] pub fn test_csgcmccm1r(&self) -> bool {
        self.csgcmccm1r() != 0
    }

    #[doc="Sets the CSGCMCCM1R field."]
    #[inline] pub fn set_csgcmccm1r<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csgcmccm1r {
    #[inline]
    fn from(other: u32) -> Self {
         Csgcmccm1r(other)
    }
}

impl ::core::fmt::Display for Csgcmccm1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csgcmccm1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csgcmccm2r(pub u32);
impl Csgcmccm2r {
    #[doc="CSGCMCCM2R"]
    #[inline] pub fn csgcmccm2r(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSGCMCCM2R != 0"]
    #[inline] pub fn test_csgcmccm2r(&self) -> bool {
        self.csgcmccm2r() != 0
    }

    #[doc="Sets the CSGCMCCM2R field."]
    #[inline] pub fn set_csgcmccm2r<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csgcmccm2r {
    #[inline]
    fn from(other: u32) -> Self {
         Csgcmccm2r(other)
    }
}

impl ::core::fmt::Display for Csgcmccm2r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csgcmccm2r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csgcmccm3r(pub u32);
impl Csgcmccm3r {
    #[doc="CSGCMCCM3R"]
    #[inline] pub fn csgcmccm3r(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSGCMCCM3R != 0"]
    #[inline] pub fn test_csgcmccm3r(&self) -> bool {
        self.csgcmccm3r() != 0
    }

    #[doc="Sets the CSGCMCCM3R field."]
    #[inline] pub fn set_csgcmccm3r<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csgcmccm3r {
    #[inline]
    fn from(other: u32) -> Self {
         Csgcmccm3r(other)
    }
}

impl ::core::fmt::Display for Csgcmccm3r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csgcmccm3r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csgcmccm4r(pub u32);
impl Csgcmccm4r {
    #[doc="CSGCMCCM4R"]
    #[inline] pub fn csgcmccm4r(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSGCMCCM4R != 0"]
    #[inline] pub fn test_csgcmccm4r(&self) -> bool {
        self.csgcmccm4r() != 0
    }

    #[doc="Sets the CSGCMCCM4R field."]
    #[inline] pub fn set_csgcmccm4r<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csgcmccm4r {
    #[inline]
    fn from(other: u32) -> Self {
         Csgcmccm4r(other)
    }
}

impl ::core::fmt::Display for Csgcmccm4r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csgcmccm4r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csgcmccm5r(pub u32);
impl Csgcmccm5r {
    #[doc="CSGCMCCM5R"]
    #[inline] pub fn csgcmccm5r(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSGCMCCM5R != 0"]
    #[inline] pub fn test_csgcmccm5r(&self) -> bool {
        self.csgcmccm5r() != 0
    }

    #[doc="Sets the CSGCMCCM5R field."]
    #[inline] pub fn set_csgcmccm5r<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csgcmccm5r {
    #[inline]
    fn from(other: u32) -> Self {
         Csgcmccm5r(other)
    }
}

impl ::core::fmt::Display for Csgcmccm5r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csgcmccm5r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csgcmccm6r(pub u32);
impl Csgcmccm6r {
    #[doc="CSGCMCCM6R"]
    #[inline] pub fn csgcmccm6r(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSGCMCCM6R != 0"]
    #[inline] pub fn test_csgcmccm6r(&self) -> bool {
        self.csgcmccm6r() != 0
    }

    #[doc="Sets the CSGCMCCM6R field."]
    #[inline] pub fn set_csgcmccm6r<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csgcmccm6r {
    #[inline]
    fn from(other: u32) -> Self {
         Csgcmccm6r(other)
    }
}

impl ::core::fmt::Display for Csgcmccm6r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csgcmccm6r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csgcmccm7r(pub u32);
impl Csgcmccm7r {
    #[doc="CSGCMCCM7R"]
    #[inline] pub fn csgcmccm7r(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSGCMCCM7R != 0"]
    #[inline] pub fn test_csgcmccm7r(&self) -> bool {
        self.csgcmccm7r() != 0
    }

    #[doc="Sets the CSGCMCCM7R field."]
    #[inline] pub fn set_csgcmccm7r<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csgcmccm7r {
    #[inline]
    fn from(other: u32) -> Self {
         Csgcmccm7r(other)
    }
}

impl ::core::fmt::Display for Csgcmccm7r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csgcmccm7r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csgcm0r(pub u32);
impl Csgcm0r {
    #[doc="CSGCM0R"]
    #[inline] pub fn csgcm0r(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSGCM0R != 0"]
    #[inline] pub fn test_csgcm0r(&self) -> bool {
        self.csgcm0r() != 0
    }

    #[doc="Sets the CSGCM0R field."]
    #[inline] pub fn set_csgcm0r<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csgcm0r {
    #[inline]
    fn from(other: u32) -> Self {
         Csgcm0r(other)
    }
}

impl ::core::fmt::Display for Csgcm0r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csgcm0r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csgcm1r(pub u32);
impl Csgcm1r {
    #[doc="CSGCM1R"]
    #[inline] pub fn csgcm1r(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSGCM1R != 0"]
    #[inline] pub fn test_csgcm1r(&self) -> bool {
        self.csgcm1r() != 0
    }

    #[doc="Sets the CSGCM1R field."]
    #[inline] pub fn set_csgcm1r<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csgcm1r {
    #[inline]
    fn from(other: u32) -> Self {
         Csgcm1r(other)
    }
}

impl ::core::fmt::Display for Csgcm1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csgcm1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csgcm2r(pub u32);
impl Csgcm2r {
    #[doc="CSGCM2R"]
    #[inline] pub fn csgcm2r(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSGCM2R != 0"]
    #[inline] pub fn test_csgcm2r(&self) -> bool {
        self.csgcm2r() != 0
    }

    #[doc="Sets the CSGCM2R field."]
    #[inline] pub fn set_csgcm2r<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csgcm2r {
    #[inline]
    fn from(other: u32) -> Self {
         Csgcm2r(other)
    }
}

impl ::core::fmt::Display for Csgcm2r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csgcm2r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csgcm3r(pub u32);
impl Csgcm3r {
    #[doc="CSGCM3R"]
    #[inline] pub fn csgcm3r(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSGCM3R != 0"]
    #[inline] pub fn test_csgcm3r(&self) -> bool {
        self.csgcm3r() != 0
    }

    #[doc="Sets the CSGCM3R field."]
    #[inline] pub fn set_csgcm3r<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csgcm3r {
    #[inline]
    fn from(other: u32) -> Self {
         Csgcm3r(other)
    }
}

impl ::core::fmt::Display for Csgcm3r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csgcm3r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csgcm4r(pub u32);
impl Csgcm4r {
    #[doc="CSGCM4R"]
    #[inline] pub fn csgcm4r(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSGCM4R != 0"]
    #[inline] pub fn test_csgcm4r(&self) -> bool {
        self.csgcm4r() != 0
    }

    #[doc="Sets the CSGCM4R field."]
    #[inline] pub fn set_csgcm4r<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csgcm4r {
    #[inline]
    fn from(other: u32) -> Self {
         Csgcm4r(other)
    }
}

impl ::core::fmt::Display for Csgcm4r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csgcm4r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csgcm5r(pub u32);
impl Csgcm5r {
    #[doc="CSGCM5R"]
    #[inline] pub fn csgcm5r(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSGCM5R != 0"]
    #[inline] pub fn test_csgcm5r(&self) -> bool {
        self.csgcm5r() != 0
    }

    #[doc="Sets the CSGCM5R field."]
    #[inline] pub fn set_csgcm5r<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csgcm5r {
    #[inline]
    fn from(other: u32) -> Self {
         Csgcm5r(other)
    }
}

impl ::core::fmt::Display for Csgcm5r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csgcm5r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csgcm6r(pub u32);
impl Csgcm6r {
    #[doc="CSGCM6R"]
    #[inline] pub fn csgcm6r(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSGCM6R != 0"]
    #[inline] pub fn test_csgcm6r(&self) -> bool {
        self.csgcm6r() != 0
    }

    #[doc="Sets the CSGCM6R field."]
    #[inline] pub fn set_csgcm6r<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csgcm6r {
    #[inline]
    fn from(other: u32) -> Self {
         Csgcm6r(other)
    }
}

impl ::core::fmt::Display for Csgcm6r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csgcm6r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csgcm7r(pub u32);
impl Csgcm7r {
    #[doc="CSGCM7R"]
    #[inline] pub fn csgcm7r(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSGCM7R != 0"]
    #[inline] pub fn test_csgcm7r(&self) -> bool {
        self.csgcm7r() != 0
    }

    #[doc="Sets the CSGCM7R field."]
    #[inline] pub fn set_csgcm7r<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csgcm7r {
    #[inline]
    fn from(other: u32) -> Self {
         Csgcm7r(other)
    }
}

impl ::core::fmt::Display for Csgcm7r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csgcm7r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

