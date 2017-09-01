//! Register map for EMAC0 peripheral
#[allow(unused_imports)] use bobbin_common::*;

periph!(EMAC0, Emac, 0x400ec000);

#[doc="Register map for EMAC0 peripheral"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Emac(pub usize);
impl Emac {
    #[doc="Get the *const pointer for the CFG register."]
    #[inline] pub fn cfg_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x0) as *const u32
    }

    #[doc="Get the *mut pointer for the CFG register."]
    #[inline] pub fn cfg_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x0) as *mut u32
    }

    #[doc="Read the CFG register."]
    #[inline] pub fn cfg(&self) -> Cfg { 
        unsafe {
            Cfg(read_volatile((self.0 + 0x0) as *const u32))
        }
    }

    #[doc="Write the CFG register."]
    #[inline] pub fn set_cfg<F: FnOnce(Cfg) -> Cfg>(&self, f: F) -> &Self {
        let value = f(Cfg(0));
        unsafe {
            write_volatile((self.0 + 0x0) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the CFG register."]
    #[inline] pub fn with_cfg<F: FnOnce(Cfg) -> Cfg>(&self, f: F) -> &Self {
        let tmp = self.cfg();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x0) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the FRAMEFLTR register."]
    #[inline] pub fn framefltr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x4) as *const u32
    }

    #[doc="Get the *mut pointer for the FRAMEFLTR register."]
    #[inline] pub fn framefltr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x4) as *mut u32
    }

    #[doc="Read the FRAMEFLTR register."]
    #[inline] pub fn framefltr(&self) -> Framefltr { 
        unsafe {
            Framefltr(read_volatile((self.0 + 0x4) as *const u32))
        }
    }

    #[doc="Write the FRAMEFLTR register."]
    #[inline] pub fn set_framefltr<F: FnOnce(Framefltr) -> Framefltr>(&self, f: F) -> &Self {
        let value = f(Framefltr(0));
        unsafe {
            write_volatile((self.0 + 0x4) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the FRAMEFLTR register."]
    #[inline] pub fn with_framefltr<F: FnOnce(Framefltr) -> Framefltr>(&self, f: F) -> &Self {
        let tmp = self.framefltr();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x4) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the HASHTBLH register."]
    #[inline] pub fn hashtblh_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x8) as *const u32
    }

    #[doc="Get the *mut pointer for the HASHTBLH register."]
    #[inline] pub fn hashtblh_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x8) as *mut u32
    }

    #[doc="Read the HASHTBLH register."]
    #[inline] pub fn hashtblh(&self) -> Hashtblh { 
        unsafe {
            Hashtblh(read_volatile((self.0 + 0x8) as *const u32))
        }
    }

    #[doc="Write the HASHTBLH register."]
    #[inline] pub fn set_hashtblh<F: FnOnce(Hashtblh) -> Hashtblh>(&self, f: F) -> &Self {
        let value = f(Hashtblh(0));
        unsafe {
            write_volatile((self.0 + 0x8) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the HASHTBLH register."]
    #[inline] pub fn with_hashtblh<F: FnOnce(Hashtblh) -> Hashtblh>(&self, f: F) -> &Self {
        let tmp = self.hashtblh();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x8) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the HASHTBLL register."]
    #[inline] pub fn hashtbll_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0xc) as *const u32
    }

    #[doc="Get the *mut pointer for the HASHTBLL register."]
    #[inline] pub fn hashtbll_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0xc) as *mut u32
    }

    #[doc="Read the HASHTBLL register."]
    #[inline] pub fn hashtbll(&self) -> Hashtbll { 
        unsafe {
            Hashtbll(read_volatile((self.0 + 0xc) as *const u32))
        }
    }

    #[doc="Write the HASHTBLL register."]
    #[inline] pub fn set_hashtbll<F: FnOnce(Hashtbll) -> Hashtbll>(&self, f: F) -> &Self {
        let value = f(Hashtbll(0));
        unsafe {
            write_volatile((self.0 + 0xc) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the HASHTBLL register."]
    #[inline] pub fn with_hashtbll<F: FnOnce(Hashtbll) -> Hashtbll>(&self, f: F) -> &Self {
        let tmp = self.hashtbll();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0xc) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the MIIADDR register."]
    #[inline] pub fn miiaddr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x10) as *const u32
    }

    #[doc="Get the *mut pointer for the MIIADDR register."]
    #[inline] pub fn miiaddr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x10) as *mut u32
    }

    #[doc="Read the MIIADDR register."]
    #[inline] pub fn miiaddr(&self) -> Miiaddr { 
        unsafe {
            Miiaddr(read_volatile((self.0 + 0x10) as *const u32))
        }
    }

    #[doc="Write the MIIADDR register."]
    #[inline] pub fn set_miiaddr<F: FnOnce(Miiaddr) -> Miiaddr>(&self, f: F) -> &Self {
        let value = f(Miiaddr(0));
        unsafe {
            write_volatile((self.0 + 0x10) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the MIIADDR register."]
    #[inline] pub fn with_miiaddr<F: FnOnce(Miiaddr) -> Miiaddr>(&self, f: F) -> &Self {
        let tmp = self.miiaddr();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x10) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the MIIDATA register."]
    #[inline] pub fn miidata_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x14) as *const u32
    }

    #[doc="Get the *mut pointer for the MIIDATA register."]
    #[inline] pub fn miidata_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x14) as *mut u32
    }

    #[doc="Read the MIIDATA register."]
    #[inline] pub fn miidata(&self) -> Miidata { 
        unsafe {
            Miidata(read_volatile((self.0 + 0x14) as *const u32))
        }
    }

    #[doc="Write the MIIDATA register."]
    #[inline] pub fn set_miidata<F: FnOnce(Miidata) -> Miidata>(&self, f: F) -> &Self {
        let value = f(Miidata(0));
        unsafe {
            write_volatile((self.0 + 0x14) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the MIIDATA register."]
    #[inline] pub fn with_miidata<F: FnOnce(Miidata) -> Miidata>(&self, f: F) -> &Self {
        let tmp = self.miidata();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x14) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the FLOWCTL register."]
    #[inline] pub fn flowctl_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x18) as *const u32
    }

    #[doc="Get the *mut pointer for the FLOWCTL register."]
    #[inline] pub fn flowctl_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x18) as *mut u32
    }

    #[doc="Read the FLOWCTL register."]
    #[inline] pub fn flowctl(&self) -> Flowctl { 
        unsafe {
            Flowctl(read_volatile((self.0 + 0x18) as *const u32))
        }
    }

    #[doc="Write the FLOWCTL register."]
    #[inline] pub fn set_flowctl<F: FnOnce(Flowctl) -> Flowctl>(&self, f: F) -> &Self {
        let value = f(Flowctl(0));
        unsafe {
            write_volatile((self.0 + 0x18) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the FLOWCTL register."]
    #[inline] pub fn with_flowctl<F: FnOnce(Flowctl) -> Flowctl>(&self, f: F) -> &Self {
        let tmp = self.flowctl();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x18) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the VLANTG register."]
    #[inline] pub fn vlantg_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x1c) as *const u32
    }

    #[doc="Get the *mut pointer for the VLANTG register."]
    #[inline] pub fn vlantg_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x1c) as *mut u32
    }

    #[doc="Read the VLANTG register."]
    #[inline] pub fn vlantg(&self) -> Vlantg { 
        unsafe {
            Vlantg(read_volatile((self.0 + 0x1c) as *const u32))
        }
    }

    #[doc="Write the VLANTG register."]
    #[inline] pub fn set_vlantg<F: FnOnce(Vlantg) -> Vlantg>(&self, f: F) -> &Self {
        let value = f(Vlantg(0));
        unsafe {
            write_volatile((self.0 + 0x1c) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the VLANTG register."]
    #[inline] pub fn with_vlantg<F: FnOnce(Vlantg) -> Vlantg>(&self, f: F) -> &Self {
        let tmp = self.vlantg();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x1c) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the STATUS register."]
    #[inline] pub fn status_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x24) as *const u32
    }

    #[doc="Get the *mut pointer for the STATUS register."]
    #[inline] pub fn status_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x24) as *mut u32
    }

    #[doc="Read the STATUS register."]
    #[inline] pub fn status(&self) -> Status { 
        unsafe {
            Status(read_volatile((self.0 + 0x24) as *const u32))
        }
    }

    #[doc="Write the STATUS register."]
    #[inline] pub fn set_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
        let value = f(Status(0));
        unsafe {
            write_volatile((self.0 + 0x24) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the STATUS register."]
    #[inline] pub fn with_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
        let tmp = self.status();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x24) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the RWUFF register."]
    #[inline] pub fn rwuff_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x28) as *const u32
    }

    #[doc="Get the *mut pointer for the RWUFF register."]
    #[inline] pub fn rwuff_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x28) as *mut u32
    }

    #[doc="Read the RWUFF register."]
    #[inline] pub fn rwuff(&self) -> Rwuff { 
        unsafe {
            Rwuff(read_volatile((self.0 + 0x28) as *const u32))
        }
    }

    #[doc="Write the RWUFF register."]
    #[inline] pub fn set_rwuff<F: FnOnce(Rwuff) -> Rwuff>(&self, f: F) -> &Self {
        let value = f(Rwuff(0));
        unsafe {
            write_volatile((self.0 + 0x28) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the RWUFF register."]
    #[inline] pub fn with_rwuff<F: FnOnce(Rwuff) -> Rwuff>(&self, f: F) -> &Self {
        let tmp = self.rwuff();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x28) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the PMTCTLSTAT register."]
    #[inline] pub fn pmtctlstat_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x2c) as *const u32
    }

    #[doc="Get the *mut pointer for the PMTCTLSTAT register."]
    #[inline] pub fn pmtctlstat_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x2c) as *mut u32
    }

    #[doc="Read the PMTCTLSTAT register."]
    #[inline] pub fn pmtctlstat(&self) -> Pmtctlstat { 
        unsafe {
            Pmtctlstat(read_volatile((self.0 + 0x2c) as *const u32))
        }
    }

    #[doc="Write the PMTCTLSTAT register."]
    #[inline] pub fn set_pmtctlstat<F: FnOnce(Pmtctlstat) -> Pmtctlstat>(&self, f: F) -> &Self {
        let value = f(Pmtctlstat(0));
        unsafe {
            write_volatile((self.0 + 0x2c) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the PMTCTLSTAT register."]
    #[inline] pub fn with_pmtctlstat<F: FnOnce(Pmtctlstat) -> Pmtctlstat>(&self, f: F) -> &Self {
        let tmp = self.pmtctlstat();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x2c) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the RIS register."]
    #[inline] pub fn ris_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x38) as *const u32
    }

    #[doc="Get the *mut pointer for the RIS register."]
    #[inline] pub fn ris_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x38) as *mut u32
    }

    #[doc="Read the RIS register."]
    #[inline] pub fn ris(&self) -> Ris { 
        unsafe {
            Ris(read_volatile((self.0 + 0x38) as *const u32))
        }
    }

    #[doc="Write the RIS register."]
    #[inline] pub fn set_ris<F: FnOnce(Ris) -> Ris>(&self, f: F) -> &Self {
        let value = f(Ris(0));
        unsafe {
            write_volatile((self.0 + 0x38) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the RIS register."]
    #[inline] pub fn with_ris<F: FnOnce(Ris) -> Ris>(&self, f: F) -> &Self {
        let tmp = self.ris();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x38) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the IM register."]
    #[inline] pub fn im_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x3c) as *const u32
    }

    #[doc="Get the *mut pointer for the IM register."]
    #[inline] pub fn im_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x3c) as *mut u32
    }

    #[doc="Read the IM register."]
    #[inline] pub fn im(&self) -> Im { 
        unsafe {
            Im(read_volatile((self.0 + 0x3c) as *const u32))
        }
    }

    #[doc="Write the IM register."]
    #[inline] pub fn set_im<F: FnOnce(Im) -> Im>(&self, f: F) -> &Self {
        let value = f(Im(0));
        unsafe {
            write_volatile((self.0 + 0x3c) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the IM register."]
    #[inline] pub fn with_im<F: FnOnce(Im) -> Im>(&self, f: F) -> &Self {
        let tmp = self.im();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x3c) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the ADDR0H register."]
    #[inline] pub fn addr0h_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x40) as *const u32
    }

    #[doc="Get the *mut pointer for the ADDR0H register."]
    #[inline] pub fn addr0h_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x40) as *mut u32
    }

    #[doc="Read the ADDR0H register."]
    #[inline] pub fn addr0h(&self) -> Addr0h { 
        unsafe {
            Addr0h(read_volatile((self.0 + 0x40) as *const u32))
        }
    }

    #[doc="Write the ADDR0H register."]
    #[inline] pub fn set_addr0h<F: FnOnce(Addr0h) -> Addr0h>(&self, f: F) -> &Self {
        let value = f(Addr0h(0));
        unsafe {
            write_volatile((self.0 + 0x40) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the ADDR0H register."]
    #[inline] pub fn with_addr0h<F: FnOnce(Addr0h) -> Addr0h>(&self, f: F) -> &Self {
        let tmp = self.addr0h();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x40) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the ADDR0L register."]
    #[inline] pub fn addr0l_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x44) as *const u32
    }

    #[doc="Get the *mut pointer for the ADDR0L register."]
    #[inline] pub fn addr0l_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x44) as *mut u32
    }

    #[doc="Read the ADDR0L register."]
    #[inline] pub fn addr0l(&self) -> Addr0l { 
        unsafe {
            Addr0l(read_volatile((self.0 + 0x44) as *const u32))
        }
    }

    #[doc="Write the ADDR0L register."]
    #[inline] pub fn set_addr0l<F: FnOnce(Addr0l) -> Addr0l>(&self, f: F) -> &Self {
        let value = f(Addr0l(0));
        unsafe {
            write_volatile((self.0 + 0x44) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the ADDR0L register."]
    #[inline] pub fn with_addr0l<F: FnOnce(Addr0l) -> Addr0l>(&self, f: F) -> &Self {
        let tmp = self.addr0l();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x44) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the ADDR1H register."]
    #[inline] pub fn addr1h_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x48) as *const u32
    }

    #[doc="Get the *mut pointer for the ADDR1H register."]
    #[inline] pub fn addr1h_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x48) as *mut u32
    }

    #[doc="Read the ADDR1H register."]
    #[inline] pub fn addr1h(&self) -> Addr1h { 
        unsafe {
            Addr1h(read_volatile((self.0 + 0x48) as *const u32))
        }
    }

    #[doc="Write the ADDR1H register."]
    #[inline] pub fn set_addr1h<F: FnOnce(Addr1h) -> Addr1h>(&self, f: F) -> &Self {
        let value = f(Addr1h(0));
        unsafe {
            write_volatile((self.0 + 0x48) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the ADDR1H register."]
    #[inline] pub fn with_addr1h<F: FnOnce(Addr1h) -> Addr1h>(&self, f: F) -> &Self {
        let tmp = self.addr1h();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x48) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the ADDR1L register."]
    #[inline] pub fn addr1l_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x4c) as *const u32
    }

    #[doc="Get the *mut pointer for the ADDR1L register."]
    #[inline] pub fn addr1l_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x4c) as *mut u32
    }

    #[doc="Read the ADDR1L register."]
    #[inline] pub fn addr1l(&self) -> Addr1l { 
        unsafe {
            Addr1l(read_volatile((self.0 + 0x4c) as *const u32))
        }
    }

    #[doc="Write the ADDR1L register."]
    #[inline] pub fn set_addr1l<F: FnOnce(Addr1l) -> Addr1l>(&self, f: F) -> &Self {
        let value = f(Addr1l(0));
        unsafe {
            write_volatile((self.0 + 0x4c) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the ADDR1L register."]
    #[inline] pub fn with_addr1l<F: FnOnce(Addr1l) -> Addr1l>(&self, f: F) -> &Self {
        let tmp = self.addr1l();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x4c) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the ADDR2H register."]
    #[inline] pub fn addr2h_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x50) as *const u32
    }

    #[doc="Get the *mut pointer for the ADDR2H register."]
    #[inline] pub fn addr2h_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x50) as *mut u32
    }

    #[doc="Read the ADDR2H register."]
    #[inline] pub fn addr2h(&self) -> Addr2h { 
        unsafe {
            Addr2h(read_volatile((self.0 + 0x50) as *const u32))
        }
    }

    #[doc="Write the ADDR2H register."]
    #[inline] pub fn set_addr2h<F: FnOnce(Addr2h) -> Addr2h>(&self, f: F) -> &Self {
        let value = f(Addr2h(0));
        unsafe {
            write_volatile((self.0 + 0x50) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the ADDR2H register."]
    #[inline] pub fn with_addr2h<F: FnOnce(Addr2h) -> Addr2h>(&self, f: F) -> &Self {
        let tmp = self.addr2h();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x50) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the ADDR2L register."]
    #[inline] pub fn addr2l_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x54) as *const u32
    }

    #[doc="Get the *mut pointer for the ADDR2L register."]
    #[inline] pub fn addr2l_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x54) as *mut u32
    }

    #[doc="Read the ADDR2L register."]
    #[inline] pub fn addr2l(&self) -> Addr2l { 
        unsafe {
            Addr2l(read_volatile((self.0 + 0x54) as *const u32))
        }
    }

    #[doc="Write the ADDR2L register."]
    #[inline] pub fn set_addr2l<F: FnOnce(Addr2l) -> Addr2l>(&self, f: F) -> &Self {
        let value = f(Addr2l(0));
        unsafe {
            write_volatile((self.0 + 0x54) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the ADDR2L register."]
    #[inline] pub fn with_addr2l<F: FnOnce(Addr2l) -> Addr2l>(&self, f: F) -> &Self {
        let tmp = self.addr2l();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x54) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the ADDR3H register."]
    #[inline] pub fn addr3h_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x58) as *const u32
    }

    #[doc="Get the *mut pointer for the ADDR3H register."]
    #[inline] pub fn addr3h_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x58) as *mut u32
    }

    #[doc="Read the ADDR3H register."]
    #[inline] pub fn addr3h(&self) -> Addr3h { 
        unsafe {
            Addr3h(read_volatile((self.0 + 0x58) as *const u32))
        }
    }

    #[doc="Write the ADDR3H register."]
    #[inline] pub fn set_addr3h<F: FnOnce(Addr3h) -> Addr3h>(&self, f: F) -> &Self {
        let value = f(Addr3h(0));
        unsafe {
            write_volatile((self.0 + 0x58) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the ADDR3H register."]
    #[inline] pub fn with_addr3h<F: FnOnce(Addr3h) -> Addr3h>(&self, f: F) -> &Self {
        let tmp = self.addr3h();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x58) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the ADDR3L register."]
    #[inline] pub fn addr3l_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x5c) as *const u32
    }

    #[doc="Get the *mut pointer for the ADDR3L register."]
    #[inline] pub fn addr3l_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x5c) as *mut u32
    }

    #[doc="Read the ADDR3L register."]
    #[inline] pub fn addr3l(&self) -> Addr3l { 
        unsafe {
            Addr3l(read_volatile((self.0 + 0x5c) as *const u32))
        }
    }

    #[doc="Write the ADDR3L register."]
    #[inline] pub fn set_addr3l<F: FnOnce(Addr3l) -> Addr3l>(&self, f: F) -> &Self {
        let value = f(Addr3l(0));
        unsafe {
            write_volatile((self.0 + 0x5c) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the ADDR3L register."]
    #[inline] pub fn with_addr3l<F: FnOnce(Addr3l) -> Addr3l>(&self, f: F) -> &Self {
        let tmp = self.addr3l();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x5c) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the WDOGTO register."]
    #[inline] pub fn wdogto_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0xdc) as *const u32
    }

    #[doc="Get the *mut pointer for the WDOGTO register."]
    #[inline] pub fn wdogto_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0xdc) as *mut u32
    }

    #[doc="Read the WDOGTO register."]
    #[inline] pub fn wdogto(&self) -> Wdogto { 
        unsafe {
            Wdogto(read_volatile((self.0 + 0xdc) as *const u32))
        }
    }

    #[doc="Write the WDOGTO register."]
    #[inline] pub fn set_wdogto<F: FnOnce(Wdogto) -> Wdogto>(&self, f: F) -> &Self {
        let value = f(Wdogto(0));
        unsafe {
            write_volatile((self.0 + 0xdc) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the WDOGTO register."]
    #[inline] pub fn with_wdogto<F: FnOnce(Wdogto) -> Wdogto>(&self, f: F) -> &Self {
        let tmp = self.wdogto();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0xdc) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the MMCCTRL register."]
    #[inline] pub fn mmcctrl_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x100) as *const u32
    }

    #[doc="Get the *mut pointer for the MMCCTRL register."]
    #[inline] pub fn mmcctrl_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x100) as *mut u32
    }

    #[doc="Read the MMCCTRL register."]
    #[inline] pub fn mmcctrl(&self) -> Mmcctrl { 
        unsafe {
            Mmcctrl(read_volatile((self.0 + 0x100) as *const u32))
        }
    }

    #[doc="Write the MMCCTRL register."]
    #[inline] pub fn set_mmcctrl<F: FnOnce(Mmcctrl) -> Mmcctrl>(&self, f: F) -> &Self {
        let value = f(Mmcctrl(0));
        unsafe {
            write_volatile((self.0 + 0x100) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the MMCCTRL register."]
    #[inline] pub fn with_mmcctrl<F: FnOnce(Mmcctrl) -> Mmcctrl>(&self, f: F) -> &Self {
        let tmp = self.mmcctrl();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x100) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the MMCRXRIS register."]
    #[inline] pub fn mmcrxris_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x104) as *const u32
    }

    #[doc="Get the *mut pointer for the MMCRXRIS register."]
    #[inline] pub fn mmcrxris_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x104) as *mut u32
    }

    #[doc="Read the MMCRXRIS register."]
    #[inline] pub fn mmcrxris(&self) -> Mmcrxris { 
        unsafe {
            Mmcrxris(read_volatile((self.0 + 0x104) as *const u32))
        }
    }

    #[doc="Write the MMCRXRIS register."]
    #[inline] pub fn set_mmcrxris<F: FnOnce(Mmcrxris) -> Mmcrxris>(&self, f: F) -> &Self {
        let value = f(Mmcrxris(0));
        unsafe {
            write_volatile((self.0 + 0x104) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the MMCRXRIS register."]
    #[inline] pub fn with_mmcrxris<F: FnOnce(Mmcrxris) -> Mmcrxris>(&self, f: F) -> &Self {
        let tmp = self.mmcrxris();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x104) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the MMCTXRIS register."]
    #[inline] pub fn mmctxris_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x108) as *const u32
    }

    #[doc="Get the *mut pointer for the MMCTXRIS register."]
    #[inline] pub fn mmctxris_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x108) as *mut u32
    }

    #[doc="Read the MMCTXRIS register."]
    #[inline] pub fn mmctxris(&self) -> Mmctxris { 
        unsafe {
            Mmctxris(read_volatile((self.0 + 0x108) as *const u32))
        }
    }

    #[doc="Write the MMCTXRIS register."]
    #[inline] pub fn set_mmctxris<F: FnOnce(Mmctxris) -> Mmctxris>(&self, f: F) -> &Self {
        let value = f(Mmctxris(0));
        unsafe {
            write_volatile((self.0 + 0x108) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the MMCTXRIS register."]
    #[inline] pub fn with_mmctxris<F: FnOnce(Mmctxris) -> Mmctxris>(&self, f: F) -> &Self {
        let tmp = self.mmctxris();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x108) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the MMCRXIM register."]
    #[inline] pub fn mmcrxim_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x10c) as *const u32
    }

    #[doc="Get the *mut pointer for the MMCRXIM register."]
    #[inline] pub fn mmcrxim_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x10c) as *mut u32
    }

    #[doc="Read the MMCRXIM register."]
    #[inline] pub fn mmcrxim(&self) -> Mmcrxim { 
        unsafe {
            Mmcrxim(read_volatile((self.0 + 0x10c) as *const u32))
        }
    }

    #[doc="Write the MMCRXIM register."]
    #[inline] pub fn set_mmcrxim<F: FnOnce(Mmcrxim) -> Mmcrxim>(&self, f: F) -> &Self {
        let value = f(Mmcrxim(0));
        unsafe {
            write_volatile((self.0 + 0x10c) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the MMCRXIM register."]
    #[inline] pub fn with_mmcrxim<F: FnOnce(Mmcrxim) -> Mmcrxim>(&self, f: F) -> &Self {
        let tmp = self.mmcrxim();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x10c) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the MMCTXIM register."]
    #[inline] pub fn mmctxim_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x110) as *const u32
    }

    #[doc="Get the *mut pointer for the MMCTXIM register."]
    #[inline] pub fn mmctxim_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x110) as *mut u32
    }

    #[doc="Read the MMCTXIM register."]
    #[inline] pub fn mmctxim(&self) -> Mmctxim { 
        unsafe {
            Mmctxim(read_volatile((self.0 + 0x110) as *const u32))
        }
    }

    #[doc="Write the MMCTXIM register."]
    #[inline] pub fn set_mmctxim<F: FnOnce(Mmctxim) -> Mmctxim>(&self, f: F) -> &Self {
        let value = f(Mmctxim(0));
        unsafe {
            write_volatile((self.0 + 0x110) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the MMCTXIM register."]
    #[inline] pub fn with_mmctxim<F: FnOnce(Mmctxim) -> Mmctxim>(&self, f: F) -> &Self {
        let tmp = self.mmctxim();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x110) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the TXCNTGB register."]
    #[inline] pub fn txcntgb_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x118) as *const u32
    }

    #[doc="Get the *mut pointer for the TXCNTGB register."]
    #[inline] pub fn txcntgb_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x118) as *mut u32
    }

    #[doc="Read the TXCNTGB register."]
    #[inline] pub fn txcntgb(&self) -> Txcntgb { 
        unsafe {
            Txcntgb(read_volatile((self.0 + 0x118) as *const u32))
        }
    }

    #[doc="Write the TXCNTGB register."]
    #[inline] pub fn set_txcntgb<F: FnOnce(Txcntgb) -> Txcntgb>(&self, f: F) -> &Self {
        let value = f(Txcntgb(0));
        unsafe {
            write_volatile((self.0 + 0x118) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the TXCNTGB register."]
    #[inline] pub fn with_txcntgb<F: FnOnce(Txcntgb) -> Txcntgb>(&self, f: F) -> &Self {
        let tmp = self.txcntgb();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x118) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the TXCNTSCOL register."]
    #[inline] pub fn txcntscol_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x14c) as *const u32
    }

    #[doc="Get the *mut pointer for the TXCNTSCOL register."]
    #[inline] pub fn txcntscol_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x14c) as *mut u32
    }

    #[doc="Read the TXCNTSCOL register."]
    #[inline] pub fn txcntscol(&self) -> Txcntscol { 
        unsafe {
            Txcntscol(read_volatile((self.0 + 0x14c) as *const u32))
        }
    }

    #[doc="Write the TXCNTSCOL register."]
    #[inline] pub fn set_txcntscol<F: FnOnce(Txcntscol) -> Txcntscol>(&self, f: F) -> &Self {
        let value = f(Txcntscol(0));
        unsafe {
            write_volatile((self.0 + 0x14c) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the TXCNTSCOL register."]
    #[inline] pub fn with_txcntscol<F: FnOnce(Txcntscol) -> Txcntscol>(&self, f: F) -> &Self {
        let tmp = self.txcntscol();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x14c) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the TXCNTMCOL register."]
    #[inline] pub fn txcntmcol_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x150) as *const u32
    }

    #[doc="Get the *mut pointer for the TXCNTMCOL register."]
    #[inline] pub fn txcntmcol_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x150) as *mut u32
    }

    #[doc="Read the TXCNTMCOL register."]
    #[inline] pub fn txcntmcol(&self) -> Txcntmcol { 
        unsafe {
            Txcntmcol(read_volatile((self.0 + 0x150) as *const u32))
        }
    }

    #[doc="Write the TXCNTMCOL register."]
    #[inline] pub fn set_txcntmcol<F: FnOnce(Txcntmcol) -> Txcntmcol>(&self, f: F) -> &Self {
        let value = f(Txcntmcol(0));
        unsafe {
            write_volatile((self.0 + 0x150) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the TXCNTMCOL register."]
    #[inline] pub fn with_txcntmcol<F: FnOnce(Txcntmcol) -> Txcntmcol>(&self, f: F) -> &Self {
        let tmp = self.txcntmcol();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x150) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the TXOCTCNTG register."]
    #[inline] pub fn txoctcntg_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x164) as *const u32
    }

    #[doc="Get the *mut pointer for the TXOCTCNTG register."]
    #[inline] pub fn txoctcntg_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x164) as *mut u32
    }

    #[doc="Read the TXOCTCNTG register."]
    #[inline] pub fn txoctcntg(&self) -> Txoctcntg { 
        unsafe {
            Txoctcntg(read_volatile((self.0 + 0x164) as *const u32))
        }
    }

    #[doc="Write the TXOCTCNTG register."]
    #[inline] pub fn set_txoctcntg<F: FnOnce(Txoctcntg) -> Txoctcntg>(&self, f: F) -> &Self {
        let value = f(Txoctcntg(0));
        unsafe {
            write_volatile((self.0 + 0x164) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the TXOCTCNTG register."]
    #[inline] pub fn with_txoctcntg<F: FnOnce(Txoctcntg) -> Txoctcntg>(&self, f: F) -> &Self {
        let tmp = self.txoctcntg();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x164) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the RXCNTGB register."]
    #[inline] pub fn rxcntgb_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x180) as *const u32
    }

    #[doc="Get the *mut pointer for the RXCNTGB register."]
    #[inline] pub fn rxcntgb_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x180) as *mut u32
    }

    #[doc="Read the RXCNTGB register."]
    #[inline] pub fn rxcntgb(&self) -> Rxcntgb { 
        unsafe {
            Rxcntgb(read_volatile((self.0 + 0x180) as *const u32))
        }
    }

    #[doc="Write the RXCNTGB register."]
    #[inline] pub fn set_rxcntgb<F: FnOnce(Rxcntgb) -> Rxcntgb>(&self, f: F) -> &Self {
        let value = f(Rxcntgb(0));
        unsafe {
            write_volatile((self.0 + 0x180) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the RXCNTGB register."]
    #[inline] pub fn with_rxcntgb<F: FnOnce(Rxcntgb) -> Rxcntgb>(&self, f: F) -> &Self {
        let tmp = self.rxcntgb();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x180) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the RXCNTCRCERR register."]
    #[inline] pub fn rxcntcrcerr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x194) as *const u32
    }

    #[doc="Get the *mut pointer for the RXCNTCRCERR register."]
    #[inline] pub fn rxcntcrcerr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x194) as *mut u32
    }

    #[doc="Read the RXCNTCRCERR register."]
    #[inline] pub fn rxcntcrcerr(&self) -> Rxcntcrcerr { 
        unsafe {
            Rxcntcrcerr(read_volatile((self.0 + 0x194) as *const u32))
        }
    }

    #[doc="Write the RXCNTCRCERR register."]
    #[inline] pub fn set_rxcntcrcerr<F: FnOnce(Rxcntcrcerr) -> Rxcntcrcerr>(&self, f: F) -> &Self {
        let value = f(Rxcntcrcerr(0));
        unsafe {
            write_volatile((self.0 + 0x194) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the RXCNTCRCERR register."]
    #[inline] pub fn with_rxcntcrcerr<F: FnOnce(Rxcntcrcerr) -> Rxcntcrcerr>(&self, f: F) -> &Self {
        let tmp = self.rxcntcrcerr();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x194) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the RXCNTALGNERR register."]
    #[inline] pub fn rxcntalgnerr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x198) as *const u32
    }

    #[doc="Get the *mut pointer for the RXCNTALGNERR register."]
    #[inline] pub fn rxcntalgnerr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x198) as *mut u32
    }

    #[doc="Read the RXCNTALGNERR register."]
    #[inline] pub fn rxcntalgnerr(&self) -> Rxcntalgnerr { 
        unsafe {
            Rxcntalgnerr(read_volatile((self.0 + 0x198) as *const u32))
        }
    }

    #[doc="Write the RXCNTALGNERR register."]
    #[inline] pub fn set_rxcntalgnerr<F: FnOnce(Rxcntalgnerr) -> Rxcntalgnerr>(&self, f: F) -> &Self {
        let value = f(Rxcntalgnerr(0));
        unsafe {
            write_volatile((self.0 + 0x198) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the RXCNTALGNERR register."]
    #[inline] pub fn with_rxcntalgnerr<F: FnOnce(Rxcntalgnerr) -> Rxcntalgnerr>(&self, f: F) -> &Self {
        let tmp = self.rxcntalgnerr();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x198) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the RXCNTGUNI register."]
    #[inline] pub fn rxcntguni_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x1c4) as *const u32
    }

    #[doc="Get the *mut pointer for the RXCNTGUNI register."]
    #[inline] pub fn rxcntguni_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x1c4) as *mut u32
    }

    #[doc="Read the RXCNTGUNI register."]
    #[inline] pub fn rxcntguni(&self) -> Rxcntguni { 
        unsafe {
            Rxcntguni(read_volatile((self.0 + 0x1c4) as *const u32))
        }
    }

    #[doc="Write the RXCNTGUNI register."]
    #[inline] pub fn set_rxcntguni<F: FnOnce(Rxcntguni) -> Rxcntguni>(&self, f: F) -> &Self {
        let value = f(Rxcntguni(0));
        unsafe {
            write_volatile((self.0 + 0x1c4) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the RXCNTGUNI register."]
    #[inline] pub fn with_rxcntguni<F: FnOnce(Rxcntguni) -> Rxcntguni>(&self, f: F) -> &Self {
        let tmp = self.rxcntguni();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x1c4) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the VLNINCREP register."]
    #[inline] pub fn vlnincrep_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x584) as *const u32
    }

    #[doc="Get the *mut pointer for the VLNINCREP register."]
    #[inline] pub fn vlnincrep_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x584) as *mut u32
    }

    #[doc="Read the VLNINCREP register."]
    #[inline] pub fn vlnincrep(&self) -> Vlnincrep { 
        unsafe {
            Vlnincrep(read_volatile((self.0 + 0x584) as *const u32))
        }
    }

    #[doc="Write the VLNINCREP register."]
    #[inline] pub fn set_vlnincrep<F: FnOnce(Vlnincrep) -> Vlnincrep>(&self, f: F) -> &Self {
        let value = f(Vlnincrep(0));
        unsafe {
            write_volatile((self.0 + 0x584) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the VLNINCREP register."]
    #[inline] pub fn with_vlnincrep<F: FnOnce(Vlnincrep) -> Vlnincrep>(&self, f: F) -> &Self {
        let tmp = self.vlnincrep();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x584) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the VLANHASH register."]
    #[inline] pub fn vlanhash_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x588) as *const u32
    }

    #[doc="Get the *mut pointer for the VLANHASH register."]
    #[inline] pub fn vlanhash_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x588) as *mut u32
    }

    #[doc="Read the VLANHASH register."]
    #[inline] pub fn vlanhash(&self) -> Vlanhash { 
        unsafe {
            Vlanhash(read_volatile((self.0 + 0x588) as *const u32))
        }
    }

    #[doc="Write the VLANHASH register."]
    #[inline] pub fn set_vlanhash<F: FnOnce(Vlanhash) -> Vlanhash>(&self, f: F) -> &Self {
        let value = f(Vlanhash(0));
        unsafe {
            write_volatile((self.0 + 0x588) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the VLANHASH register."]
    #[inline] pub fn with_vlanhash<F: FnOnce(Vlanhash) -> Vlanhash>(&self, f: F) -> &Self {
        let tmp = self.vlanhash();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x588) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the TIMSTCTRL register."]
    #[inline] pub fn timstctrl_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x700) as *const u32
    }

    #[doc="Get the *mut pointer for the TIMSTCTRL register."]
    #[inline] pub fn timstctrl_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x700) as *mut u32
    }

    #[doc="Read the TIMSTCTRL register."]
    #[inline] pub fn timstctrl(&self) -> Timstctrl { 
        unsafe {
            Timstctrl(read_volatile((self.0 + 0x700) as *const u32))
        }
    }

    #[doc="Write the TIMSTCTRL register."]
    #[inline] pub fn set_timstctrl<F: FnOnce(Timstctrl) -> Timstctrl>(&self, f: F) -> &Self {
        let value = f(Timstctrl(0));
        unsafe {
            write_volatile((self.0 + 0x700) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the TIMSTCTRL register."]
    #[inline] pub fn with_timstctrl<F: FnOnce(Timstctrl) -> Timstctrl>(&self, f: F) -> &Self {
        let tmp = self.timstctrl();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x700) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the SUBSECINC register."]
    #[inline] pub fn subsecinc_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x704) as *const u32
    }

    #[doc="Get the *mut pointer for the SUBSECINC register."]
    #[inline] pub fn subsecinc_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x704) as *mut u32
    }

    #[doc="Read the SUBSECINC register."]
    #[inline] pub fn subsecinc(&self) -> Subsecinc { 
        unsafe {
            Subsecinc(read_volatile((self.0 + 0x704) as *const u32))
        }
    }

    #[doc="Write the SUBSECINC register."]
    #[inline] pub fn set_subsecinc<F: FnOnce(Subsecinc) -> Subsecinc>(&self, f: F) -> &Self {
        let value = f(Subsecinc(0));
        unsafe {
            write_volatile((self.0 + 0x704) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the SUBSECINC register."]
    #[inline] pub fn with_subsecinc<F: FnOnce(Subsecinc) -> Subsecinc>(&self, f: F) -> &Self {
        let tmp = self.subsecinc();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x704) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the TIMSEC register."]
    #[inline] pub fn timsec_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x708) as *const u32
    }

    #[doc="Get the *mut pointer for the TIMSEC register."]
    #[inline] pub fn timsec_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x708) as *mut u32
    }

    #[doc="Read the TIMSEC register."]
    #[inline] pub fn timsec(&self) -> Timsec { 
        unsafe {
            Timsec(read_volatile((self.0 + 0x708) as *const u32))
        }
    }

    #[doc="Write the TIMSEC register."]
    #[inline] pub fn set_timsec<F: FnOnce(Timsec) -> Timsec>(&self, f: F) -> &Self {
        let value = f(Timsec(0));
        unsafe {
            write_volatile((self.0 + 0x708) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the TIMSEC register."]
    #[inline] pub fn with_timsec<F: FnOnce(Timsec) -> Timsec>(&self, f: F) -> &Self {
        let tmp = self.timsec();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x708) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the TIMNANO register."]
    #[inline] pub fn timnano_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x70c) as *const u32
    }

    #[doc="Get the *mut pointer for the TIMNANO register."]
    #[inline] pub fn timnano_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x70c) as *mut u32
    }

    #[doc="Read the TIMNANO register."]
    #[inline] pub fn timnano(&self) -> Timnano { 
        unsafe {
            Timnano(read_volatile((self.0 + 0x70c) as *const u32))
        }
    }

    #[doc="Write the TIMNANO register."]
    #[inline] pub fn set_timnano<F: FnOnce(Timnano) -> Timnano>(&self, f: F) -> &Self {
        let value = f(Timnano(0));
        unsafe {
            write_volatile((self.0 + 0x70c) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the TIMNANO register."]
    #[inline] pub fn with_timnano<F: FnOnce(Timnano) -> Timnano>(&self, f: F) -> &Self {
        let tmp = self.timnano();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x70c) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the TIMSECU register."]
    #[inline] pub fn timsecu_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x710) as *const u32
    }

    #[doc="Get the *mut pointer for the TIMSECU register."]
    #[inline] pub fn timsecu_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x710) as *mut u32
    }

    #[doc="Read the TIMSECU register."]
    #[inline] pub fn timsecu(&self) -> Timsecu { 
        unsafe {
            Timsecu(read_volatile((self.0 + 0x710) as *const u32))
        }
    }

    #[doc="Write the TIMSECU register."]
    #[inline] pub fn set_timsecu<F: FnOnce(Timsecu) -> Timsecu>(&self, f: F) -> &Self {
        let value = f(Timsecu(0));
        unsafe {
            write_volatile((self.0 + 0x710) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the TIMSECU register."]
    #[inline] pub fn with_timsecu<F: FnOnce(Timsecu) -> Timsecu>(&self, f: F) -> &Self {
        let tmp = self.timsecu();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x710) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the TIMNANOU register."]
    #[inline] pub fn timnanou_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x714) as *const u32
    }

    #[doc="Get the *mut pointer for the TIMNANOU register."]
    #[inline] pub fn timnanou_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x714) as *mut u32
    }

    #[doc="Read the TIMNANOU register."]
    #[inline] pub fn timnanou(&self) -> Timnanou { 
        unsafe {
            Timnanou(read_volatile((self.0 + 0x714) as *const u32))
        }
    }

    #[doc="Write the TIMNANOU register."]
    #[inline] pub fn set_timnanou<F: FnOnce(Timnanou) -> Timnanou>(&self, f: F) -> &Self {
        let value = f(Timnanou(0));
        unsafe {
            write_volatile((self.0 + 0x714) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the TIMNANOU register."]
    #[inline] pub fn with_timnanou<F: FnOnce(Timnanou) -> Timnanou>(&self, f: F) -> &Self {
        let tmp = self.timnanou();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x714) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the TIMADD register."]
    #[inline] pub fn timadd_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x718) as *const u32
    }

    #[doc="Get the *mut pointer for the TIMADD register."]
    #[inline] pub fn timadd_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x718) as *mut u32
    }

    #[doc="Read the TIMADD register."]
    #[inline] pub fn timadd(&self) -> Timadd { 
        unsafe {
            Timadd(read_volatile((self.0 + 0x718) as *const u32))
        }
    }

    #[doc="Write the TIMADD register."]
    #[inline] pub fn set_timadd<F: FnOnce(Timadd) -> Timadd>(&self, f: F) -> &Self {
        let value = f(Timadd(0));
        unsafe {
            write_volatile((self.0 + 0x718) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the TIMADD register."]
    #[inline] pub fn with_timadd<F: FnOnce(Timadd) -> Timadd>(&self, f: F) -> &Self {
        let tmp = self.timadd();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x718) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the TARGSEC register."]
    #[inline] pub fn targsec_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x71c) as *const u32
    }

    #[doc="Get the *mut pointer for the TARGSEC register."]
    #[inline] pub fn targsec_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x71c) as *mut u32
    }

    #[doc="Read the TARGSEC register."]
    #[inline] pub fn targsec(&self) -> Targsec { 
        unsafe {
            Targsec(read_volatile((self.0 + 0x71c) as *const u32))
        }
    }

    #[doc="Write the TARGSEC register."]
    #[inline] pub fn set_targsec<F: FnOnce(Targsec) -> Targsec>(&self, f: F) -> &Self {
        let value = f(Targsec(0));
        unsafe {
            write_volatile((self.0 + 0x71c) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the TARGSEC register."]
    #[inline] pub fn with_targsec<F: FnOnce(Targsec) -> Targsec>(&self, f: F) -> &Self {
        let tmp = self.targsec();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x71c) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the TARGNANO register."]
    #[inline] pub fn targnano_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x720) as *const u32
    }

    #[doc="Get the *mut pointer for the TARGNANO register."]
    #[inline] pub fn targnano_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x720) as *mut u32
    }

    #[doc="Read the TARGNANO register."]
    #[inline] pub fn targnano(&self) -> Targnano { 
        unsafe {
            Targnano(read_volatile((self.0 + 0x720) as *const u32))
        }
    }

    #[doc="Write the TARGNANO register."]
    #[inline] pub fn set_targnano<F: FnOnce(Targnano) -> Targnano>(&self, f: F) -> &Self {
        let value = f(Targnano(0));
        unsafe {
            write_volatile((self.0 + 0x720) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the TARGNANO register."]
    #[inline] pub fn with_targnano<F: FnOnce(Targnano) -> Targnano>(&self, f: F) -> &Self {
        let tmp = self.targnano();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x720) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the HWORDSEC register."]
    #[inline] pub fn hwordsec_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x724) as *const u32
    }

    #[doc="Get the *mut pointer for the HWORDSEC register."]
    #[inline] pub fn hwordsec_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x724) as *mut u32
    }

    #[doc="Read the HWORDSEC register."]
    #[inline] pub fn hwordsec(&self) -> Hwordsec { 
        unsafe {
            Hwordsec(read_volatile((self.0 + 0x724) as *const u32))
        }
    }

    #[doc="Write the HWORDSEC register."]
    #[inline] pub fn set_hwordsec<F: FnOnce(Hwordsec) -> Hwordsec>(&self, f: F) -> &Self {
        let value = f(Hwordsec(0));
        unsafe {
            write_volatile((self.0 + 0x724) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the HWORDSEC register."]
    #[inline] pub fn with_hwordsec<F: FnOnce(Hwordsec) -> Hwordsec>(&self, f: F) -> &Self {
        let tmp = self.hwordsec();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x724) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the TIMSTAT register."]
    #[inline] pub fn timstat_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x728) as *const u32
    }

    #[doc="Get the *mut pointer for the TIMSTAT register."]
    #[inline] pub fn timstat_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x728) as *mut u32
    }

    #[doc="Read the TIMSTAT register."]
    #[inline] pub fn timstat(&self) -> Timstat { 
        unsafe {
            Timstat(read_volatile((self.0 + 0x728) as *const u32))
        }
    }

    #[doc="Write the TIMSTAT register."]
    #[inline] pub fn set_timstat<F: FnOnce(Timstat) -> Timstat>(&self, f: F) -> &Self {
        let value = f(Timstat(0));
        unsafe {
            write_volatile((self.0 + 0x728) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the TIMSTAT register."]
    #[inline] pub fn with_timstat<F: FnOnce(Timstat) -> Timstat>(&self, f: F) -> &Self {
        let tmp = self.timstat();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x728) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the PPSCTRL register."]
    #[inline] pub fn ppsctrl_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x72c) as *const u32
    }

    #[doc="Get the *mut pointer for the PPSCTRL register."]
    #[inline] pub fn ppsctrl_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x72c) as *mut u32
    }

    #[doc="Read the PPSCTRL register."]
    #[inline] pub fn ppsctrl(&self) -> Ppsctrl { 
        unsafe {
            Ppsctrl(read_volatile((self.0 + 0x72c) as *const u32))
        }
    }

    #[doc="Write the PPSCTRL register."]
    #[inline] pub fn set_ppsctrl<F: FnOnce(Ppsctrl) -> Ppsctrl>(&self, f: F) -> &Self {
        let value = f(Ppsctrl(0));
        unsafe {
            write_volatile((self.0 + 0x72c) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the PPSCTRL register."]
    #[inline] pub fn with_ppsctrl<F: FnOnce(Ppsctrl) -> Ppsctrl>(&self, f: F) -> &Self {
        let tmp = self.ppsctrl();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x72c) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the PPS0INTVL register."]
    #[inline] pub fn pps0intvl_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x760) as *const u32
    }

    #[doc="Get the *mut pointer for the PPS0INTVL register."]
    #[inline] pub fn pps0intvl_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x760) as *mut u32
    }

    #[doc="Read the PPS0INTVL register."]
    #[inline] pub fn pps0intvl(&self) -> Pps0intvl { 
        unsafe {
            Pps0intvl(read_volatile((self.0 + 0x760) as *const u32))
        }
    }

    #[doc="Write the PPS0INTVL register."]
    #[inline] pub fn set_pps0intvl<F: FnOnce(Pps0intvl) -> Pps0intvl>(&self, f: F) -> &Self {
        let value = f(Pps0intvl(0));
        unsafe {
            write_volatile((self.0 + 0x760) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the PPS0INTVL register."]
    #[inline] pub fn with_pps0intvl<F: FnOnce(Pps0intvl) -> Pps0intvl>(&self, f: F) -> &Self {
        let tmp = self.pps0intvl();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x760) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the PPS0WIDTH register."]
    #[inline] pub fn pps0width_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x764) as *const u32
    }

    #[doc="Get the *mut pointer for the PPS0WIDTH register."]
    #[inline] pub fn pps0width_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x764) as *mut u32
    }

    #[doc="Read the PPS0WIDTH register."]
    #[inline] pub fn pps0width(&self) -> Pps0width { 
        unsafe {
            Pps0width(read_volatile((self.0 + 0x764) as *const u32))
        }
    }

    #[doc="Write the PPS0WIDTH register."]
    #[inline] pub fn set_pps0width<F: FnOnce(Pps0width) -> Pps0width>(&self, f: F) -> &Self {
        let value = f(Pps0width(0));
        unsafe {
            write_volatile((self.0 + 0x764) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the PPS0WIDTH register."]
    #[inline] pub fn with_pps0width<F: FnOnce(Pps0width) -> Pps0width>(&self, f: F) -> &Self {
        let tmp = self.pps0width();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x764) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the DMABUSMOD register."]
    #[inline] pub fn dmabusmod_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0xc00) as *const u32
    }

    #[doc="Get the *mut pointer for the DMABUSMOD register."]
    #[inline] pub fn dmabusmod_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0xc00) as *mut u32
    }

    #[doc="Read the DMABUSMOD register."]
    #[inline] pub fn dmabusmod(&self) -> Dmabusmod { 
        unsafe {
            Dmabusmod(read_volatile((self.0 + 0xc00) as *const u32))
        }
    }

    #[doc="Write the DMABUSMOD register."]
    #[inline] pub fn set_dmabusmod<F: FnOnce(Dmabusmod) -> Dmabusmod>(&self, f: F) -> &Self {
        let value = f(Dmabusmod(0));
        unsafe {
            write_volatile((self.0 + 0xc00) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the DMABUSMOD register."]
    #[inline] pub fn with_dmabusmod<F: FnOnce(Dmabusmod) -> Dmabusmod>(&self, f: F) -> &Self {
        let tmp = self.dmabusmod();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0xc00) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the TXPOLLD register."]
    #[inline] pub fn txpolld_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0xc04) as *const u32
    }

    #[doc="Get the *mut pointer for the TXPOLLD register."]
    #[inline] pub fn txpolld_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0xc04) as *mut u32
    }

    #[doc="Write the TXPOLLD register."]
    #[inline] pub fn set_txpolld<F: FnOnce(Txpolld) -> Txpolld>(&self, f: F) -> &Self {
        let value = f(Txpolld(0));
        unsafe {
            write_volatile((self.0 + 0xc04) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the RXPOLLD register."]
    #[inline] pub fn rxpolld_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0xc08) as *const u32
    }

    #[doc="Get the *mut pointer for the RXPOLLD register."]
    #[inline] pub fn rxpolld_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0xc08) as *mut u32
    }

    #[doc="Write the RXPOLLD register."]
    #[inline] pub fn set_rxpolld<F: FnOnce(Rxpolld) -> Rxpolld>(&self, f: F) -> &Self {
        let value = f(Rxpolld(0));
        unsafe {
            write_volatile((self.0 + 0xc08) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the RXDLADDR register."]
    #[inline] pub fn rxdladdr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0xc0c) as *const u32
    }

    #[doc="Get the *mut pointer for the RXDLADDR register."]
    #[inline] pub fn rxdladdr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0xc0c) as *mut u32
    }

    #[doc="Read the RXDLADDR register."]
    #[inline] pub fn rxdladdr(&self) -> Rxdladdr { 
        unsafe {
            Rxdladdr(read_volatile((self.0 + 0xc0c) as *const u32))
        }
    }

    #[doc="Write the RXDLADDR register."]
    #[inline] pub fn set_rxdladdr<F: FnOnce(Rxdladdr) -> Rxdladdr>(&self, f: F) -> &Self {
        let value = f(Rxdladdr(0));
        unsafe {
            write_volatile((self.0 + 0xc0c) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the RXDLADDR register."]
    #[inline] pub fn with_rxdladdr<F: FnOnce(Rxdladdr) -> Rxdladdr>(&self, f: F) -> &Self {
        let tmp = self.rxdladdr();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0xc0c) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the TXDLADDR register."]
    #[inline] pub fn txdladdr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0xc10) as *const u32
    }

    #[doc="Get the *mut pointer for the TXDLADDR register."]
    #[inline] pub fn txdladdr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0xc10) as *mut u32
    }

    #[doc="Read the TXDLADDR register."]
    #[inline] pub fn txdladdr(&self) -> Txdladdr { 
        unsafe {
            Txdladdr(read_volatile((self.0 + 0xc10) as *const u32))
        }
    }

    #[doc="Write the TXDLADDR register."]
    #[inline] pub fn set_txdladdr<F: FnOnce(Txdladdr) -> Txdladdr>(&self, f: F) -> &Self {
        let value = f(Txdladdr(0));
        unsafe {
            write_volatile((self.0 + 0xc10) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the TXDLADDR register."]
    #[inline] pub fn with_txdladdr<F: FnOnce(Txdladdr) -> Txdladdr>(&self, f: F) -> &Self {
        let tmp = self.txdladdr();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0xc10) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the DMARIS register."]
    #[inline] pub fn dmaris_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0xc14) as *const u32
    }

    #[doc="Get the *mut pointer for the DMARIS register."]
    #[inline] pub fn dmaris_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0xc14) as *mut u32
    }

    #[doc="Read the DMARIS register."]
    #[inline] pub fn dmaris(&self) -> Dmaris { 
        unsafe {
            Dmaris(read_volatile((self.0 + 0xc14) as *const u32))
        }
    }

    #[doc="Write the DMARIS register."]
    #[inline] pub fn set_dmaris<F: FnOnce(Dmaris) -> Dmaris>(&self, f: F) -> &Self {
        let value = f(Dmaris(0));
        unsafe {
            write_volatile((self.0 + 0xc14) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the DMARIS register."]
    #[inline] pub fn with_dmaris<F: FnOnce(Dmaris) -> Dmaris>(&self, f: F) -> &Self {
        let tmp = self.dmaris();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0xc14) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the DMAOPMODE register."]
    #[inline] pub fn dmaopmode_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0xc18) as *const u32
    }

    #[doc="Get the *mut pointer for the DMAOPMODE register."]
    #[inline] pub fn dmaopmode_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0xc18) as *mut u32
    }

    #[doc="Read the DMAOPMODE register."]
    #[inline] pub fn dmaopmode(&self) -> Dmaopmode { 
        unsafe {
            Dmaopmode(read_volatile((self.0 + 0xc18) as *const u32))
        }
    }

    #[doc="Write the DMAOPMODE register."]
    #[inline] pub fn set_dmaopmode<F: FnOnce(Dmaopmode) -> Dmaopmode>(&self, f: F) -> &Self {
        let value = f(Dmaopmode(0));
        unsafe {
            write_volatile((self.0 + 0xc18) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the DMAOPMODE register."]
    #[inline] pub fn with_dmaopmode<F: FnOnce(Dmaopmode) -> Dmaopmode>(&self, f: F) -> &Self {
        let tmp = self.dmaopmode();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0xc18) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the DMAIM register."]
    #[inline] pub fn dmaim_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0xc1c) as *const u32
    }

    #[doc="Get the *mut pointer for the DMAIM register."]
    #[inline] pub fn dmaim_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0xc1c) as *mut u32
    }

    #[doc="Read the DMAIM register."]
    #[inline] pub fn dmaim(&self) -> Dmaim { 
        unsafe {
            Dmaim(read_volatile((self.0 + 0xc1c) as *const u32))
        }
    }

    #[doc="Write the DMAIM register."]
    #[inline] pub fn set_dmaim<F: FnOnce(Dmaim) -> Dmaim>(&self, f: F) -> &Self {
        let value = f(Dmaim(0));
        unsafe {
            write_volatile((self.0 + 0xc1c) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the DMAIM register."]
    #[inline] pub fn with_dmaim<F: FnOnce(Dmaim) -> Dmaim>(&self, f: F) -> &Self {
        let tmp = self.dmaim();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0xc1c) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the MFBOC register."]
    #[inline] pub fn mfboc_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0xc20) as *const u32
    }

    #[doc="Get the *mut pointer for the MFBOC register."]
    #[inline] pub fn mfboc_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0xc20) as *mut u32
    }

    #[doc="Read the MFBOC register."]
    #[inline] pub fn mfboc(&self) -> Mfboc { 
        unsafe {
            Mfboc(read_volatile((self.0 + 0xc20) as *const u32))
        }
    }

    #[doc="Write the MFBOC register."]
    #[inline] pub fn set_mfboc<F: FnOnce(Mfboc) -> Mfboc>(&self, f: F) -> &Self {
        let value = f(Mfboc(0));
        unsafe {
            write_volatile((self.0 + 0xc20) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the MFBOC register."]
    #[inline] pub fn with_mfboc<F: FnOnce(Mfboc) -> Mfboc>(&self, f: F) -> &Self {
        let tmp = self.mfboc();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0xc20) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the RXINTWDT register."]
    #[inline] pub fn rxintwdt_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0xc24) as *const u32
    }

    #[doc="Get the *mut pointer for the RXINTWDT register."]
    #[inline] pub fn rxintwdt_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0xc24) as *mut u32
    }

    #[doc="Read the RXINTWDT register."]
    #[inline] pub fn rxintwdt(&self) -> Rxintwdt { 
        unsafe {
            Rxintwdt(read_volatile((self.0 + 0xc24) as *const u32))
        }
    }

    #[doc="Write the RXINTWDT register."]
    #[inline] pub fn set_rxintwdt<F: FnOnce(Rxintwdt) -> Rxintwdt>(&self, f: F) -> &Self {
        let value = f(Rxintwdt(0));
        unsafe {
            write_volatile((self.0 + 0xc24) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the RXINTWDT register."]
    #[inline] pub fn with_rxintwdt<F: FnOnce(Rxintwdt) -> Rxintwdt>(&self, f: F) -> &Self {
        let tmp = self.rxintwdt();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0xc24) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the HOSTXDESC register."]
    #[inline] pub fn hostxdesc_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0xc48) as *const u32
    }

    #[doc="Get the *mut pointer for the HOSTXDESC register."]
    #[inline] pub fn hostxdesc_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0xc48) as *mut u32
    }

    #[doc="Read the HOSTXDESC register."]
    #[inline] pub fn hostxdesc(&self) -> Hostxdesc { 
        unsafe {
            Hostxdesc(read_volatile((self.0 + 0xc48) as *const u32))
        }
    }

    #[doc="Write the HOSTXDESC register."]
    #[inline] pub fn set_hostxdesc<F: FnOnce(Hostxdesc) -> Hostxdesc>(&self, f: F) -> &Self {
        let value = f(Hostxdesc(0));
        unsafe {
            write_volatile((self.0 + 0xc48) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the HOSTXDESC register."]
    #[inline] pub fn with_hostxdesc<F: FnOnce(Hostxdesc) -> Hostxdesc>(&self, f: F) -> &Self {
        let tmp = self.hostxdesc();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0xc48) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the HOSRXDESC register."]
    #[inline] pub fn hosrxdesc_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0xc4c) as *const u32
    }

    #[doc="Get the *mut pointer for the HOSRXDESC register."]
    #[inline] pub fn hosrxdesc_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0xc4c) as *mut u32
    }

    #[doc="Read the HOSRXDESC register."]
    #[inline] pub fn hosrxdesc(&self) -> Hosrxdesc { 
        unsafe {
            Hosrxdesc(read_volatile((self.0 + 0xc4c) as *const u32))
        }
    }

    #[doc="Write the HOSRXDESC register."]
    #[inline] pub fn set_hosrxdesc<F: FnOnce(Hosrxdesc) -> Hosrxdesc>(&self, f: F) -> &Self {
        let value = f(Hosrxdesc(0));
        unsafe {
            write_volatile((self.0 + 0xc4c) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the HOSRXDESC register."]
    #[inline] pub fn with_hosrxdesc<F: FnOnce(Hosrxdesc) -> Hosrxdesc>(&self, f: F) -> &Self {
        let tmp = self.hosrxdesc();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0xc4c) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the HOSTXBA register."]
    #[inline] pub fn hostxba_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0xc50) as *const u32
    }

    #[doc="Get the *mut pointer for the HOSTXBA register."]
    #[inline] pub fn hostxba_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0xc50) as *mut u32
    }

    #[doc="Read the HOSTXBA register."]
    #[inline] pub fn hostxba(&self) -> Hostxba { 
        unsafe {
            Hostxba(read_volatile((self.0 + 0xc50) as *const u32))
        }
    }

    #[doc="Write the HOSTXBA register."]
    #[inline] pub fn set_hostxba<F: FnOnce(Hostxba) -> Hostxba>(&self, f: F) -> &Self {
        let value = f(Hostxba(0));
        unsafe {
            write_volatile((self.0 + 0xc50) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the HOSTXBA register."]
    #[inline] pub fn with_hostxba<F: FnOnce(Hostxba) -> Hostxba>(&self, f: F) -> &Self {
        let tmp = self.hostxba();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0xc50) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the HOSRXBA register."]
    #[inline] pub fn hosrxba_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0xc54) as *const u32
    }

    #[doc="Get the *mut pointer for the HOSRXBA register."]
    #[inline] pub fn hosrxba_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0xc54) as *mut u32
    }

    #[doc="Read the HOSRXBA register."]
    #[inline] pub fn hosrxba(&self) -> Hosrxba { 
        unsafe {
            Hosrxba(read_volatile((self.0 + 0xc54) as *const u32))
        }
    }

    #[doc="Write the HOSRXBA register."]
    #[inline] pub fn set_hosrxba<F: FnOnce(Hosrxba) -> Hosrxba>(&self, f: F) -> &Self {
        let value = f(Hosrxba(0));
        unsafe {
            write_volatile((self.0 + 0xc54) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the HOSRXBA register."]
    #[inline] pub fn with_hosrxba<F: FnOnce(Hosrxba) -> Hosrxba>(&self, f: F) -> &Self {
        let tmp = self.hosrxba();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0xc54) as *mut u32, value.0);
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

    #[doc="Get the *const pointer for the CC register."]
    #[inline] pub fn cc_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0xfc8) as *const u32
    }

    #[doc="Get the *mut pointer for the CC register."]
    #[inline] pub fn cc_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0xfc8) as *mut u32
    }

    #[doc="Read the CC register."]
    #[inline] pub fn cc(&self) -> Cc { 
        unsafe {
            Cc(read_volatile((self.0 + 0xfc8) as *const u32))
        }
    }

    #[doc="Write the CC register."]
    #[inline] pub fn set_cc<F: FnOnce(Cc) -> Cc>(&self, f: F) -> &Self {
        let value = f(Cc(0));
        unsafe {
            write_volatile((self.0 + 0xfc8) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the CC register."]
    #[inline] pub fn with_cc<F: FnOnce(Cc) -> Cc>(&self, f: F) -> &Self {
        let tmp = self.cc();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0xfc8) as *mut u32, value.0);
        }
        self
    }

}

#[doc="Ethernet MAC Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfg(pub u32);
impl Cfg {
    #[doc="Preamble Length for Transmit Frames"]
    #[inline] pub fn prelen(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Preamble Length for Transmit Frames"]
    #[inline] pub fn test_prelen(&self) -> bool {
        self.prelen != 0
    }

    #[doc="Preamble Length for Transmit Frames"]
    #[inline] pub fn set_prelen<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Receiver Enable"]
    #[inline] pub fn re(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Receiver Enable"]
    #[inline] pub fn test_re(&self) -> bool {
        self.re != 0
    }

    #[doc="Receiver Enable"]
    #[inline] pub fn set_re<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Transmitter Enable"]
    #[inline] pub fn te(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Transmitter Enable"]
    #[inline] pub fn test_te(&self) -> bool {
        self.te != 0
    }

    #[doc="Transmitter Enable"]
    #[inline] pub fn set_te<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Deferral Check"]
    #[inline] pub fn dc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Deferral Check"]
    #[inline] pub fn test_dc(&self) -> bool {
        self.dc != 0
    }

    #[doc="Deferral Check"]
    #[inline] pub fn set_dc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Back-Off Limit"]
    #[inline] pub fn bl(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="Back-Off Limit"]
    #[inline] pub fn test_bl(&self) -> bool {
        self.bl != 0
    }

    #[doc="Back-Off Limit"]
    #[inline] pub fn set_bl<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Automatic Pad or CRC Stripping"]
    #[inline] pub fn acs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Automatic Pad or CRC Stripping"]
    #[inline] pub fn test_acs(&self) -> bool {
        self.acs != 0
    }

    #[doc="Automatic Pad or CRC Stripping"]
    #[inline] pub fn set_acs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Disable Retry"]
    #[inline] pub fn dr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Disable Retry"]
    #[inline] pub fn test_dr(&self) -> bool {
        self.dr != 0
    }

    #[doc="Disable Retry"]
    #[inline] pub fn set_dr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Checksum Offload"]
    #[inline] pub fn ipc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Checksum Offload"]
    #[inline] pub fn test_ipc(&self) -> bool {
        self.ipc != 0
    }

    #[doc="Checksum Offload"]
    #[inline] pub fn set_ipc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Duplex Mode"]
    #[inline] pub fn dupm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Duplex Mode"]
    #[inline] pub fn test_dupm(&self) -> bool {
        self.dupm != 0
    }

    #[doc="Duplex Mode"]
    #[inline] pub fn set_dupm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Loopback Mode"]
    #[inline] pub fn loopbm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Loopback Mode"]
    #[inline] pub fn test_loopbm(&self) -> bool {
        self.loopbm != 0
    }

    #[doc="Loopback Mode"]
    #[inline] pub fn set_loopbm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Disable Receive Own"]
    #[inline] pub fn dro(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Disable Receive Own"]
    #[inline] pub fn test_dro(&self) -> bool {
        self.dro != 0
    }

    #[doc="Disable Receive Own"]
    #[inline] pub fn set_dro<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Speed"]
    #[inline] pub fn fes(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Speed"]
    #[inline] pub fn test_fes(&self) -> bool {
        self.fes != 0
    }

    #[doc="Speed"]
    #[inline] pub fn set_fes<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Port Select"]
    #[inline] pub fn ps(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Port Select"]
    #[inline] pub fn test_ps(&self) -> bool {
        self.ps != 0
    }

    #[doc="Port Select"]
    #[inline] pub fn set_ps<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Disable Carrier Sense During Transmission"]
    #[inline] pub fn discrs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Disable Carrier Sense During Transmission"]
    #[inline] pub fn test_discrs(&self) -> bool {
        self.discrs != 0
    }

    #[doc="Disable Carrier Sense During Transmission"]
    #[inline] pub fn set_discrs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Inter-Frame Gap (IFG)"]
    #[inline] pub fn ifg(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x7) as u8) } // [19:17]
    }

    #[doc="Inter-Frame Gap (IFG)"]
    #[inline] pub fn test_ifg(&self) -> bool {
        self.ifg != 0
    }

    #[doc="Inter-Frame Gap (IFG)"]
    #[inline] pub fn set_ifg<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Jumbo Frame Enable"]
    #[inline] pub fn jfen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Jumbo Frame Enable"]
    #[inline] pub fn test_jfen(&self) -> bool {
        self.jfen != 0
    }

    #[doc="Jumbo Frame Enable"]
    #[inline] pub fn set_jfen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Jabber Disable"]
    #[inline] pub fn jd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Jabber Disable"]
    #[inline] pub fn test_jd(&self) -> bool {
        self.jd != 0
    }

    #[doc="Jabber Disable"]
    #[inline] pub fn set_jd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Watchdog Disable"]
    #[inline] pub fn wddis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Watchdog Disable"]
    #[inline] pub fn test_wddis(&self) -> bool {
        self.wddis != 0
    }

    #[doc="Watchdog Disable"]
    #[inline] pub fn set_wddis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="CRC Stripping for Type Frames"]
    #[inline] pub fn cst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="CRC Stripping for Type Frames"]
    #[inline] pub fn test_cst(&self) -> bool {
        self.cst != 0
    }

    #[doc="CRC Stripping for Type Frames"]
    #[inline] pub fn set_cst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="IEEE 802"]
    #[inline] pub fn twokpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="IEEE 802"]
    #[inline] pub fn test_twokpen(&self) -> bool {
        self.twokpen != 0
    }

    #[doc="IEEE 802"]
    #[inline] pub fn set_twokpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

}

impl ::core::fmt::Display for Cfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.prelen() != 0 { try!(write!(f, " prelen=0x{:x}", self.prelen()))}
        if self.re() != 0 { try!(write!(f, " re"))}
        if self.te() != 0 { try!(write!(f, " te"))}
        if self.dc() != 0 { try!(write!(f, " dc"))}
        if self.bl() != 0 { try!(write!(f, " bl=0x{:x}", self.bl()))}
        if self.acs() != 0 { try!(write!(f, " acs"))}
        if self.dr() != 0 { try!(write!(f, " dr"))}
        if self.ipc() != 0 { try!(write!(f, " ipc"))}
        if self.dupm() != 0 { try!(write!(f, " dupm"))}
        if self.loopbm() != 0 { try!(write!(f, " loopbm"))}
        if self.dro() != 0 { try!(write!(f, " dro"))}
        if self.fes() != 0 { try!(write!(f, " fes"))}
        if self.ps() != 0 { try!(write!(f, " ps"))}
        if self.discrs() != 0 { try!(write!(f, " discrs"))}
        if self.ifg() != 0 { try!(write!(f, " ifg=0x{:x}", self.ifg()))}
        if self.jfen() != 0 { try!(write!(f, " jfen"))}
        if self.jd() != 0 { try!(write!(f, " jd"))}
        if self.wddis() != 0 { try!(write!(f, " wddis"))}
        if self.cst() != 0 { try!(write!(f, " cst"))}
        if self.twokpen() != 0 { try!(write!(f, " twokpen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Frame Filter"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Framefltr(pub u32);
impl Framefltr {
    #[doc="Promiscuous Mode"]
    #[inline] pub fn pr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Promiscuous Mode"]
    #[inline] pub fn test_pr(&self) -> bool {
        self.pr != 0
    }

    #[doc="Promiscuous Mode"]
    #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Hash Unicast"]
    #[inline] pub fn huc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Hash Unicast"]
    #[inline] pub fn test_huc(&self) -> bool {
        self.huc != 0
    }

    #[doc="Hash Unicast"]
    #[inline] pub fn set_huc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Hash Multicast"]
    #[inline] pub fn hmc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Hash Multicast"]
    #[inline] pub fn test_hmc(&self) -> bool {
        self.hmc != 0
    }

    #[doc="Hash Multicast"]
    #[inline] pub fn set_hmc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Destination Address (DA) Inverse Filtering"]
    #[inline] pub fn daif(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Destination Address (DA) Inverse Filtering"]
    #[inline] pub fn test_daif(&self) -> bool {
        self.daif != 0
    }

    #[doc="Destination Address (DA) Inverse Filtering"]
    #[inline] pub fn set_daif<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Pass All Multicast"]
    #[inline] pub fn pm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Pass All Multicast"]
    #[inline] pub fn test_pm(&self) -> bool {
        self.pm != 0
    }

    #[doc="Pass All Multicast"]
    #[inline] pub fn set_pm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Disable Broadcast Frames"]
    #[inline] pub fn dbf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Disable Broadcast Frames"]
    #[inline] pub fn test_dbf(&self) -> bool {
        self.dbf != 0
    }

    #[doc="Disable Broadcast Frames"]
    #[inline] pub fn set_dbf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Pass Control Frames"]
    #[inline] pub fn pcf(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Pass Control Frames"]
    #[inline] pub fn test_pcf(&self) -> bool {
        self.pcf != 0
    }

    #[doc="Pass Control Frames"]
    #[inline] pub fn set_pcf<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Source Address (SA) Inverse Filtering"]
    #[inline] pub fn saif(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Source Address (SA) Inverse Filtering"]
    #[inline] pub fn test_saif(&self) -> bool {
        self.saif != 0
    }

    #[doc="Source Address (SA) Inverse Filtering"]
    #[inline] pub fn set_saif<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Source Address Filter Enable"]
    #[inline] pub fn saf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Source Address Filter Enable"]
    #[inline] pub fn test_saf(&self) -> bool {
        self.saf != 0
    }

    #[doc="Source Address Filter Enable"]
    #[inline] pub fn set_saf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Hash or Perfect Filter"]
    #[inline] pub fn hpf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Hash or Perfect Filter"]
    #[inline] pub fn test_hpf(&self) -> bool {
        self.hpf != 0
    }

    #[doc="Hash or Perfect Filter"]
    #[inline] pub fn set_hpf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="VLAN Tag Filter Enable"]
    #[inline] pub fn vtfe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="VLAN Tag Filter Enable"]
    #[inline] pub fn test_vtfe(&self) -> bool {
        self.vtfe != 0
    }

    #[doc="VLAN Tag Filter Enable"]
    #[inline] pub fn set_vtfe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Receive All"]
    #[inline] pub fn ra(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Receive All"]
    #[inline] pub fn test_ra(&self) -> bool {
        self.ra != 0
    }

    #[doc="Receive All"]
    #[inline] pub fn set_ra<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl ::core::fmt::Display for Framefltr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Framefltr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pr() != 0 { try!(write!(f, " pr"))}
        if self.huc() != 0 { try!(write!(f, " huc"))}
        if self.hmc() != 0 { try!(write!(f, " hmc"))}
        if self.daif() != 0 { try!(write!(f, " daif"))}
        if self.pm() != 0 { try!(write!(f, " pm"))}
        if self.dbf() != 0 { try!(write!(f, " dbf"))}
        if self.pcf() != 0 { try!(write!(f, " pcf=0x{:x}", self.pcf()))}
        if self.saif() != 0 { try!(write!(f, " saif"))}
        if self.saf() != 0 { try!(write!(f, " saf"))}
        if self.hpf() != 0 { try!(write!(f, " hpf"))}
        if self.vtfe() != 0 { try!(write!(f, " vtfe"))}
        if self.ra() != 0 { try!(write!(f, " ra"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Hash Table High"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hashtblh(pub u32);
impl Hashtblh {
    #[doc="Hash Table High"]
    #[inline] pub fn hth(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Hash Table High"]
    #[inline] pub fn test_hth(&self) -> bool {
        self.hth != 0
    }

    #[doc="Hash Table High"]
    #[inline] pub fn set_hth<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Hashtblh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hashtblh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Hash Table Low"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hashtbll(pub u32);
impl Hashtbll {
    #[doc="Hash Table Low"]
    #[inline] pub fn htl(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Hash Table Low"]
    #[inline] pub fn test_htl(&self) -> bool {
        self.htl != 0
    }

    #[doc="Hash Table Low"]
    #[inline] pub fn set_htl<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Hashtbll {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hashtbll {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC MII Address"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Miiaddr(pub u32);
impl Miiaddr {
    #[doc="MII Busy"]
    #[inline] pub fn miib(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="MII Busy"]
    #[inline] pub fn test_miib(&self) -> bool {
        self.miib != 0
    }

    #[doc="MII Busy"]
    #[inline] pub fn set_miib<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="MII Write"]
    #[inline] pub fn miiw(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="MII Write"]
    #[inline] pub fn test_miiw(&self) -> bool {
        self.miiw != 0
    }

    #[doc="MII Write"]
    #[inline] pub fn set_miiw<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Clock Reference Frequency Selection"]
    #[inline] pub fn cr(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0xf) as u8) } // [5:2]
    }

    #[doc="Clock Reference Frequency Selection"]
    #[inline] pub fn test_cr(&self) -> bool {
        self.cr != 0
    }

    #[doc="Clock Reference Frequency Selection"]
    #[inline] pub fn set_cr<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="MII Register"]
    #[inline] pub fn mii(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1f) as u8) } // [10:6]
    }

    #[doc="MII Register"]
    #[inline] pub fn test_mii(&self) -> bool {
        self.mii != 0
    }

    #[doc="MII Register"]
    #[inline] pub fn set_mii<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Physical Layer Address"]
    #[inline] pub fn pla(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1f) as u8) } // [15:11]
    }

    #[doc="Physical Layer Address"]
    #[inline] pub fn test_pla(&self) -> bool {
        self.pla != 0
    }

    #[doc="Physical Layer Address"]
    #[inline] pub fn set_pla<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 11);
        self.0 |= value << 11;
        self
    }

}

impl ::core::fmt::Display for Miiaddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Miiaddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.miib() != 0 { try!(write!(f, " miib"))}
        if self.miiw() != 0 { try!(write!(f, " miiw"))}
        if self.cr() != 0 { try!(write!(f, " cr=0x{:x}", self.cr()))}
        if self.mii() != 0 { try!(write!(f, " mii=0x{:x}", self.mii()))}
        if self.pla() != 0 { try!(write!(f, " pla=0x{:x}", self.pla()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC MII Data Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Miidata(pub u32);
impl Miidata {
    #[doc="MII Data"]
    #[inline] pub fn data(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="MII Data"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data != 0
    }

    #[doc="MII Data"]
    #[inline] pub fn set_data<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Miidata {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Miidata {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.data() != 0 { try!(write!(f, " data=0x{:x}", self.data()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Flow Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Flowctl(pub u32);
impl Flowctl {
    #[doc="Flow Control Busy or Back-pressure Activate"]
    #[inline] pub fn fcbbpa(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Flow Control Busy or Back-pressure Activate"]
    #[inline] pub fn test_fcbbpa(&self) -> bool {
        self.fcbbpa != 0
    }

    #[doc="Flow Control Busy or Back-pressure Activate"]
    #[inline] pub fn set_fcbbpa<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transmit Flow Control Enable"]
    #[inline] pub fn tfe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Transmit Flow Control Enable"]
    #[inline] pub fn test_tfe(&self) -> bool {
        self.tfe != 0
    }

    #[doc="Transmit Flow Control Enable"]
    #[inline] pub fn set_tfe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Receive Flow Control Enable"]
    #[inline] pub fn rfe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Receive Flow Control Enable"]
    #[inline] pub fn test_rfe(&self) -> bool {
        self.rfe != 0
    }

    #[doc="Receive Flow Control Enable"]
    #[inline] pub fn set_rfe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Unicast Pause Frame Detect"]
    #[inline] pub fn up(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Unicast Pause Frame Detect"]
    #[inline] pub fn test_up(&self) -> bool {
        self.up != 0
    }

    #[doc="Unicast Pause Frame Detect"]
    #[inline] pub fn set_up<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Pause Low Threshold"]
    #[inline] pub fn plt(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Pause Low Threshold"]
    #[inline] pub fn test_plt(&self) -> bool {
        self.plt != 0
    }

    #[doc="Pause Low Threshold"]
    #[inline] pub fn set_plt<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Disable Zero-Quanta Pause"]
    #[inline] pub fn dzqp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Disable Zero-Quanta Pause"]
    #[inline] pub fn test_dzqp(&self) -> bool {
        self.dzqp != 0
    }

    #[doc="Disable Zero-Quanta Pause"]
    #[inline] pub fn set_dzqp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Pause Time"]
    #[inline] pub fn pt(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Pause Time"]
    #[inline] pub fn test_pt(&self) -> bool {
        self.pt != 0
    }

    #[doc="Pause Time"]
    #[inline] pub fn set_pt<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

}

impl ::core::fmt::Display for Flowctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Flowctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fcbbpa() != 0 { try!(write!(f, " fcbbpa"))}
        if self.tfe() != 0 { try!(write!(f, " tfe"))}
        if self.rfe() != 0 { try!(write!(f, " rfe"))}
        if self.up() != 0 { try!(write!(f, " up"))}
        if self.plt() != 0 { try!(write!(f, " plt=0x{:x}", self.plt()))}
        if self.dzqp() != 0 { try!(write!(f, " dzqp"))}
        if self.pt() != 0 { try!(write!(f, " pt=0x{:x}", self.pt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC VLAN Tag"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Vlantg(pub u32);
impl Vlantg {
    #[doc="VLAN Tag Identifier for Receive Frames"]
    #[inline] pub fn vl(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="VLAN Tag Identifier for Receive Frames"]
    #[inline] pub fn test_vl(&self) -> bool {
        self.vl != 0
    }

    #[doc="VLAN Tag Identifier for Receive Frames"]
    #[inline] pub fn set_vl<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Enable 12-Bit VLAN Tag Comparison"]
    #[inline] pub fn etv(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Enable 12-Bit VLAN Tag Comparison"]
    #[inline] pub fn test_etv(&self) -> bool {
        self.etv != 0
    }

    #[doc="Enable 12-Bit VLAN Tag Comparison"]
    #[inline] pub fn set_etv<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="VLAN Tag Inverse Match Enable"]
    #[inline] pub fn vtim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="VLAN Tag Inverse Match Enable"]
    #[inline] pub fn test_vtim(&self) -> bool {
        self.vtim != 0
    }

    #[doc="VLAN Tag Inverse Match Enable"]
    #[inline] pub fn set_vtim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Enable S-VLAN"]
    #[inline] pub fn esvl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Enable S-VLAN"]
    #[inline] pub fn test_esvl(&self) -> bool {
        self.esvl != 0
    }

    #[doc="Enable S-VLAN"]
    #[inline] pub fn set_esvl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="VLAN Tag Hash Table Match Enable"]
    #[inline] pub fn vthm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="VLAN Tag Hash Table Match Enable"]
    #[inline] pub fn test_vthm(&self) -> bool {
        self.vthm != 0
    }

    #[doc="VLAN Tag Hash Table Match Enable"]
    #[inline] pub fn set_vthm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

}

impl ::core::fmt::Display for Vlantg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Vlantg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.vl() != 0 { try!(write!(f, " vl=0x{:x}", self.vl()))}
        if self.etv() != 0 { try!(write!(f, " etv"))}
        if self.vtim() != 0 { try!(write!(f, " vtim"))}
        if self.esvl() != 0 { try!(write!(f, " esvl"))}
        if self.vthm() != 0 { try!(write!(f, " vthm"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u32);
impl Status {
    #[doc="MAC MII Receive Protocol Engine Status"]
    #[inline] pub fn rpe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="MAC MII Receive Protocol Engine Status"]
    #[inline] pub fn test_rpe(&self) -> bool {
        self.rpe != 0
    }

    #[doc="MAC MII Receive Protocol Engine Status"]
    #[inline] pub fn set_rpe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="MAC Receive Frame Controller FIFO Status"]
    #[inline] pub fn rfcfc(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x3) as u8) } // [2:1]
    }

    #[doc="MAC Receive Frame Controller FIFO Status"]
    #[inline] pub fn test_rfcfc(&self) -> bool {
        self.rfcfc != 0
    }

    #[doc="MAC Receive Frame Controller FIFO Status"]
    #[inline] pub fn set_rfcfc<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="TX/RX Controller RX FIFO Write Controller Active Status"]
    #[inline] pub fn rwc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="TX/RX Controller RX FIFO Write Controller Active Status"]
    #[inline] pub fn test_rwc(&self) -> bool {
        self.rwc != 0
    }

    #[doc="TX/RX Controller RX FIFO Write Controller Active Status"]
    #[inline] pub fn set_rwc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="TX/RX Controller Read Controller State"]
    #[inline] pub fn rrc(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="TX/RX Controller Read Controller State"]
    #[inline] pub fn test_rrc(&self) -> bool {
        self.rrc != 0
    }

    #[doc="TX/RX Controller Read Controller State"]
    #[inline] pub fn set_rrc<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="TX/RX Controller RX FIFO Fill-level Status"]
    #[inline] pub fn rxf(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="TX/RX Controller RX FIFO Fill-level Status"]
    #[inline] pub fn test_rxf(&self) -> bool {
        self.rxf != 0
    }

    #[doc="TX/RX Controller RX FIFO Fill-level Status"]
    #[inline] pub fn set_rxf<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="MAC MII Transmit Protocol Engine Status"]
    #[inline] pub fn tpe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="MAC MII Transmit Protocol Engine Status"]
    #[inline] pub fn test_tpe(&self) -> bool {
        self.tpe != 0
    }

    #[doc="MAC MII Transmit Protocol Engine Status"]
    #[inline] pub fn set_tpe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="MAC Transmit Frame Controller Status"]
    #[inline] pub fn tfc(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x3) as u8) } // [18:17]
    }

    #[doc="MAC Transmit Frame Controller Status"]
    #[inline] pub fn test_tfc(&self) -> bool {
        self.tfc != 0
    }

    #[doc="MAC Transmit Frame Controller Status"]
    #[inline] pub fn set_tfc<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="MAC Transmitter PAUSE"]
    #[inline] pub fn txpaused(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="MAC Transmitter PAUSE"]
    #[inline] pub fn test_txpaused(&self) -> bool {
        self.txpaused != 0
    }

    #[doc="MAC Transmitter PAUSE"]
    #[inline] pub fn set_txpaused<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="TX/RX Controller\'s TX FIFO Read Controller Status"]
    #[inline] pub fn trc(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
    }

    #[doc="TX/RX Controller\'s TX FIFO Read Controller Status"]
    #[inline] pub fn test_trc(&self) -> bool {
        self.trc != 0
    }

    #[doc="TX/RX Controller\'s TX FIFO Read Controller Status"]
    #[inline] pub fn set_trc<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="TX/RX Controller TX FIFO Write Controller Active Status"]
    #[inline] pub fn twc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="TX/RX Controller TX FIFO Write Controller Active Status"]
    #[inline] pub fn test_twc(&self) -> bool {
        self.twc != 0
    }

    #[doc="TX/RX Controller TX FIFO Write Controller Active Status"]
    #[inline] pub fn set_twc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="TX/RX Controller TX FIFO Not Empty Status"]
    #[inline] pub fn txfe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="TX/RX Controller TX FIFO Not Empty Status"]
    #[inline] pub fn test_txfe(&self) -> bool {
        self.txfe != 0
    }

    #[doc="TX/RX Controller TX FIFO Not Empty Status"]
    #[inline] pub fn set_txfe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="TX/RX Controller TX FIFO Full Status"]
    #[inline] pub fn txff(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="TX/RX Controller TX FIFO Full Status"]
    #[inline] pub fn test_txff(&self) -> bool {
        self.txff != 0
    }

    #[doc="TX/RX Controller TX FIFO Full Status"]
    #[inline] pub fn set_txff<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
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
        if self.rpe() != 0 { try!(write!(f, " rpe"))}
        if self.rfcfc() != 0 { try!(write!(f, " rfcfc=0x{:x}", self.rfcfc()))}
        if self.rwc() != 0 { try!(write!(f, " rwc"))}
        if self.rrc() != 0 { try!(write!(f, " rrc=0x{:x}", self.rrc()))}
        if self.rxf() != 0 { try!(write!(f, " rxf=0x{:x}", self.rxf()))}
        if self.tpe() != 0 { try!(write!(f, " tpe"))}
        if self.tfc() != 0 { try!(write!(f, " tfc=0x{:x}", self.tfc()))}
        if self.txpaused() != 0 { try!(write!(f, " txpaused"))}
        if self.trc() != 0 { try!(write!(f, " trc=0x{:x}", self.trc()))}
        if self.twc() != 0 { try!(write!(f, " twc"))}
        if self.txfe() != 0 { try!(write!(f, " txfe"))}
        if self.txff() != 0 { try!(write!(f, " txff"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Remote Wake-Up Frame Filter"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rwuff(pub u32);
impl Rwuff {
    #[doc="Remote Wake-Up Frame Filter"]
    #[inline] pub fn wakeupfil(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Remote Wake-Up Frame Filter"]
    #[inline] pub fn test_wakeupfil(&self) -> bool {
        self.wakeupfil != 0
    }

    #[doc="Remote Wake-Up Frame Filter"]
    #[inline] pub fn set_wakeupfil<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Rwuff {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rwuff {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC PMT Control and Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pmtctlstat(pub u32);
impl Pmtctlstat {
    #[doc="Power Down"]
    #[inline] pub fn pwrdwn(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Power Down"]
    #[inline] pub fn test_pwrdwn(&self) -> bool {
        self.pwrdwn != 0
    }

    #[doc="Power Down"]
    #[inline] pub fn set_pwrdwn<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Magic Packet Enable"]
    #[inline] pub fn mgkpkten(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Magic Packet Enable"]
    #[inline] pub fn test_mgkpkten(&self) -> bool {
        self.mgkpkten != 0
    }

    #[doc="Magic Packet Enable"]
    #[inline] pub fn set_mgkpkten<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Wake-Up Frame Enable"]
    #[inline] pub fn wupfren(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Wake-Up Frame Enable"]
    #[inline] pub fn test_wupfren(&self) -> bool {
        self.wupfren != 0
    }

    #[doc="Wake-Up Frame Enable"]
    #[inline] pub fn set_wupfren<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Magic Packet Received"]
    #[inline] pub fn mgkprx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Magic Packet Received"]
    #[inline] pub fn test_mgkprx(&self) -> bool {
        self.mgkprx != 0
    }

    #[doc="Magic Packet Received"]
    #[inline] pub fn set_mgkprx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Wake-Up Frame Received"]
    #[inline] pub fn wuprx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Wake-Up Frame Received"]
    #[inline] pub fn test_wuprx(&self) -> bool {
        self.wuprx != 0
    }

    #[doc="Wake-Up Frame Received"]
    #[inline] pub fn set_wuprx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Global Unicast"]
    #[inline] pub fn glblucast(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Global Unicast"]
    #[inline] pub fn test_glblucast(&self) -> bool {
        self.glblucast != 0
    }

    #[doc="Global Unicast"]
    #[inline] pub fn set_glblucast<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Remote Wake-Up FIFO Pointer"]
    #[inline] pub fn rwkptr(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Remote Wake-Up FIFO Pointer"]
    #[inline] pub fn test_rwkptr(&self) -> bool {
        self.rwkptr != 0
    }

    #[doc="Remote Wake-Up FIFO Pointer"]
    #[inline] pub fn set_rwkptr<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Wake-Up Frame Filter Register Pointer Reset"]
    #[inline] pub fn wupfrrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Wake-Up Frame Filter Register Pointer Reset"]
    #[inline] pub fn test_wupfrrst(&self) -> bool {
        self.wupfrrst != 0
    }

    #[doc="Wake-Up Frame Filter Register Pointer Reset"]
    #[inline] pub fn set_wupfrrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl ::core::fmt::Display for Pmtctlstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pmtctlstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pwrdwn() != 0 { try!(write!(f, " pwrdwn"))}
        if self.mgkpkten() != 0 { try!(write!(f, " mgkpkten"))}
        if self.wupfren() != 0 { try!(write!(f, " wupfren"))}
        if self.mgkprx() != 0 { try!(write!(f, " mgkprx"))}
        if self.wuprx() != 0 { try!(write!(f, " wuprx"))}
        if self.glblucast() != 0 { try!(write!(f, " glblucast"))}
        if self.rwkptr() != 0 { try!(write!(f, " rwkptr=0x{:x}", self.rwkptr()))}
        if self.wupfrrst() != 0 { try!(write!(f, " wupfrrst"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Raw Interrupt Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ris(pub u32);
impl Ris {
    #[doc="PMT Interrupt Status"]
    #[inline] pub fn pmt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="PMT Interrupt Status"]
    #[inline] pub fn test_pmt(&self) -> bool {
        self.pmt != 0
    }

    #[doc="PMT Interrupt Status"]
    #[inline] pub fn set_pmt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="MMC Interrupt Status"]
    #[inline] pub fn mmc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="MMC Interrupt Status"]
    #[inline] pub fn test_mmc(&self) -> bool {
        self.mmc != 0
    }

    #[doc="MMC Interrupt Status"]
    #[inline] pub fn set_mmc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="MMC Receive Interrupt Status"]
    #[inline] pub fn mmcrx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="MMC Receive Interrupt Status"]
    #[inline] pub fn test_mmcrx(&self) -> bool {
        self.mmcrx != 0
    }

    #[doc="MMC Receive Interrupt Status"]
    #[inline] pub fn set_mmcrx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="MMC Transmit Interrupt Status"]
    #[inline] pub fn mmctx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="MMC Transmit Interrupt Status"]
    #[inline] pub fn test_mmctx(&self) -> bool {
        self.mmctx != 0
    }

    #[doc="MMC Transmit Interrupt Status"]
    #[inline] pub fn set_mmctx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Timestamp Interrupt Status"]
    #[inline] pub fn ts(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Timestamp Interrupt Status"]
    #[inline] pub fn test_ts(&self) -> bool {
        self.ts != 0
    }

    #[doc="Timestamp Interrupt Status"]
    #[inline] pub fn set_ts<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

}

impl ::core::fmt::Display for Ris {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ris {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pmt() != 0 { try!(write!(f, " pmt"))}
        if self.mmc() != 0 { try!(write!(f, " mmc"))}
        if self.mmcrx() != 0 { try!(write!(f, " mmcrx"))}
        if self.mmctx() != 0 { try!(write!(f, " mmctx"))}
        if self.ts() != 0 { try!(write!(f, " ts"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Interrupt Mask"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Im(pub u32);
impl Im {
    #[doc="PMT Interrupt Mask"]
    #[inline] pub fn pmt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="PMT Interrupt Mask"]
    #[inline] pub fn test_pmt(&self) -> bool {
        self.pmt != 0
    }

    #[doc="PMT Interrupt Mask"]
    #[inline] pub fn set_pmt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Timestamp Interrupt Mask"]
    #[inline] pub fn tsi(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Timestamp Interrupt Mask"]
    #[inline] pub fn test_tsi(&self) -> bool {
        self.tsi != 0
    }

    #[doc="Timestamp Interrupt Mask"]
    #[inline] pub fn set_tsi<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

}

impl ::core::fmt::Display for Im {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Im {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pmt() != 0 { try!(write!(f, " pmt"))}
        if self.tsi() != 0 { try!(write!(f, " tsi"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Address 0 High"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Addr0h(pub u32);
impl Addr0h {
    #[doc="MAC Address0 [47:32]"]
    #[inline] pub fn addrhi(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="MAC Address0 [47:32]"]
    #[inline] pub fn test_addrhi(&self) -> bool {
        self.addrhi != 0
    }

    #[doc="MAC Address0 [47:32]"]
    #[inline] pub fn set_addrhi<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Address Enable"]
    #[inline] pub fn ae(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Address Enable"]
    #[inline] pub fn test_ae(&self) -> bool {
        self.ae != 0
    }

    #[doc="Address Enable"]
    #[inline] pub fn set_ae<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl ::core::fmt::Display for Addr0h {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Addr0h {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.addrhi() != 0 { try!(write!(f, " addrhi=0x{:x}", self.addrhi()))}
        if self.ae() != 0 { try!(write!(f, " ae"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Address 0 Low Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Addr0l(pub u32);
impl Addr0l {
    #[doc="MAC Address0 [31:0]"]
    #[inline] pub fn addrlo(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="MAC Address0 [31:0]"]
    #[inline] pub fn test_addrlo(&self) -> bool {
        self.addrlo != 0
    }

    #[doc="MAC Address0 [31:0]"]
    #[inline] pub fn set_addrlo<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Addr0l {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Addr0l {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Address 1 High"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Addr1h(pub u32);
impl Addr1h {
    #[doc="MAC Address1 [47:32]"]
    #[inline] pub fn addrhi(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="MAC Address1 [47:32]"]
    #[inline] pub fn test_addrhi(&self) -> bool {
        self.addrhi != 0
    }

    #[doc="MAC Address1 [47:32]"]
    #[inline] pub fn set_addrhi<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Mask Byte Control"]
    #[inline] pub fn mbc(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3f) as u8) } // [29:24]
    }

    #[doc="Mask Byte Control"]
    #[inline] pub fn test_mbc(&self) -> bool {
        self.mbc != 0
    }

    #[doc="Mask Byte Control"]
    #[inline] pub fn set_mbc<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Source Address"]
    #[inline] pub fn sa(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Source Address"]
    #[inline] pub fn test_sa(&self) -> bool {
        self.sa != 0
    }

    #[doc="Source Address"]
    #[inline] pub fn set_sa<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Address Enable"]
    #[inline] pub fn ae(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Address Enable"]
    #[inline] pub fn test_ae(&self) -> bool {
        self.ae != 0
    }

    #[doc="Address Enable"]
    #[inline] pub fn set_ae<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl ::core::fmt::Display for Addr1h {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Addr1h {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.addrhi() != 0 { try!(write!(f, " addrhi=0x{:x}", self.addrhi()))}
        if self.mbc() != 0 { try!(write!(f, " mbc=0x{:x}", self.mbc()))}
        if self.sa() != 0 { try!(write!(f, " sa"))}
        if self.ae() != 0 { try!(write!(f, " ae"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Address 1 Low"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Addr1l(pub u32);
impl Addr1l {
    #[doc="MAC Address1 [31:0]"]
    #[inline] pub fn addrlo(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="MAC Address1 [31:0]"]
    #[inline] pub fn test_addrlo(&self) -> bool {
        self.addrlo != 0
    }

    #[doc="MAC Address1 [31:0]"]
    #[inline] pub fn set_addrlo<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Addr1l {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Addr1l {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Address 2 High"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Addr2h(pub u32);
impl Addr2h {
    #[doc="MAC Address2 [47:32]"]
    #[inline] pub fn addrhi(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="MAC Address2 [47:32]"]
    #[inline] pub fn test_addrhi(&self) -> bool {
        self.addrhi != 0
    }

    #[doc="MAC Address2 [47:32]"]
    #[inline] pub fn set_addrhi<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Mask Byte Control"]
    #[inline] pub fn mbc(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3f) as u8) } // [29:24]
    }

    #[doc="Mask Byte Control"]
    #[inline] pub fn test_mbc(&self) -> bool {
        self.mbc != 0
    }

    #[doc="Mask Byte Control"]
    #[inline] pub fn set_mbc<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Source Address"]
    #[inline] pub fn sa(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Source Address"]
    #[inline] pub fn test_sa(&self) -> bool {
        self.sa != 0
    }

    #[doc="Source Address"]
    #[inline] pub fn set_sa<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Address Enable"]
    #[inline] pub fn ae(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Address Enable"]
    #[inline] pub fn test_ae(&self) -> bool {
        self.ae != 0
    }

    #[doc="Address Enable"]
    #[inline] pub fn set_ae<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl ::core::fmt::Display for Addr2h {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Addr2h {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.addrhi() != 0 { try!(write!(f, " addrhi=0x{:x}", self.addrhi()))}
        if self.mbc() != 0 { try!(write!(f, " mbc=0x{:x}", self.mbc()))}
        if self.sa() != 0 { try!(write!(f, " sa"))}
        if self.ae() != 0 { try!(write!(f, " ae"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Address 2 Low"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Addr2l(pub u32);
impl Addr2l {
    #[doc="MAC Address2 [31:0]"]
    #[inline] pub fn addrlo(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="MAC Address2 [31:0]"]
    #[inline] pub fn test_addrlo(&self) -> bool {
        self.addrlo != 0
    }

    #[doc="MAC Address2 [31:0]"]
    #[inline] pub fn set_addrlo<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Addr2l {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Addr2l {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Address 3 High"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Addr3h(pub u32);
impl Addr3h {
    #[doc="MAC Address3 [47:32]"]
    #[inline] pub fn addrhi(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="MAC Address3 [47:32]"]
    #[inline] pub fn test_addrhi(&self) -> bool {
        self.addrhi != 0
    }

    #[doc="MAC Address3 [47:32]"]
    #[inline] pub fn set_addrhi<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Mask Byte Control"]
    #[inline] pub fn mbc(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3f) as u8) } // [29:24]
    }

    #[doc="Mask Byte Control"]
    #[inline] pub fn test_mbc(&self) -> bool {
        self.mbc != 0
    }

    #[doc="Mask Byte Control"]
    #[inline] pub fn set_mbc<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Source Address"]
    #[inline] pub fn sa(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Source Address"]
    #[inline] pub fn test_sa(&self) -> bool {
        self.sa != 0
    }

    #[doc="Source Address"]
    #[inline] pub fn set_sa<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Address Enable"]
    #[inline] pub fn ae(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Address Enable"]
    #[inline] pub fn test_ae(&self) -> bool {
        self.ae != 0
    }

    #[doc="Address Enable"]
    #[inline] pub fn set_ae<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl ::core::fmt::Display for Addr3h {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Addr3h {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.addrhi() != 0 { try!(write!(f, " addrhi=0x{:x}", self.addrhi()))}
        if self.mbc() != 0 { try!(write!(f, " mbc=0x{:x}", self.mbc()))}
        if self.sa() != 0 { try!(write!(f, " sa"))}
        if self.ae() != 0 { try!(write!(f, " ae"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Address 3 Low"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Addr3l(pub u32);
impl Addr3l {
    #[doc="MAC Address3 [31:0]"]
    #[inline] pub fn addrlo(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="MAC Address3 [31:0]"]
    #[inline] pub fn test_addrlo(&self) -> bool {
        self.addrlo != 0
    }

    #[doc="MAC Address3 [31:0]"]
    #[inline] pub fn set_addrlo<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Addr3l {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Addr3l {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Watchdog Timeout"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Wdogto(pub u32);
impl Wdogto {
    #[doc="Watchdog Timeout"]
    #[inline] pub fn wto(&self) -> bits::U14 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3fff) as u16) } // [13:0]
    }

    #[doc="Watchdog Timeout"]
    #[inline] pub fn test_wto(&self) -> bool {
        self.wto != 0
    }

    #[doc="Watchdog Timeout"]
    #[inline] pub fn set_wto<V: Into<bits::U14>>(mut self, value: V) -> Self {
        let value: bits::U14 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3fff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Programmable Watchdog Enable"]
    #[inline] pub fn pwe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Programmable Watchdog Enable"]
    #[inline] pub fn test_pwe(&self) -> bool {
        self.pwe != 0
    }

    #[doc="Programmable Watchdog Enable"]
    #[inline] pub fn set_pwe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

}

impl ::core::fmt::Display for Wdogto {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Wdogto {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wto() != 0 { try!(write!(f, " wto=0x{:x}", self.wto()))}
        if self.pwe() != 0 { try!(write!(f, " pwe"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC MMC Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mmcctrl(pub u32);
impl Mmcctrl {
    #[doc="Counters Reset"]
    #[inline] pub fn cntrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Counters Reset"]
    #[inline] pub fn test_cntrst(&self) -> bool {
        self.cntrst != 0
    }

    #[doc="Counters Reset"]
    #[inline] pub fn set_cntrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Counters Stop Rollover"]
    #[inline] pub fn cntstpro(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Counters Stop Rollover"]
    #[inline] pub fn test_cntstpro(&self) -> bool {
        self.cntstpro != 0
    }

    #[doc="Counters Stop Rollover"]
    #[inline] pub fn set_cntstpro<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Reset on Read"]
    #[inline] pub fn rstonrd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Reset on Read"]
    #[inline] pub fn test_rstonrd(&self) -> bool {
        self.rstonrd != 0
    }

    #[doc="Reset on Read"]
    #[inline] pub fn set_rstonrd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="MMC Counter Freeze"]
    #[inline] pub fn cntfreez(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="MMC Counter Freeze"]
    #[inline] pub fn test_cntfreez(&self) -> bool {
        self.cntfreez != 0
    }

    #[doc="MMC Counter Freeze"]
    #[inline] pub fn set_cntfreez<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Counters Preset"]
    #[inline] pub fn cntprst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Counters Preset"]
    #[inline] pub fn test_cntprst(&self) -> bool {
        self.cntprst != 0
    }

    #[doc="Counters Preset"]
    #[inline] pub fn set_cntprst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Full/Half Preset Level Value"]
    #[inline] pub fn cntprstlvl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Full/Half Preset Level Value"]
    #[inline] pub fn test_cntprstlvl(&self) -> bool {
        self.cntprstlvl != 0
    }

    #[doc="Full/Half Preset Level Value"]
    #[inline] pub fn set_cntprstlvl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Update MMC Counters for Dropped Broadcast Frames"]
    #[inline] pub fn ucdbc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Update MMC Counters for Dropped Broadcast Frames"]
    #[inline] pub fn test_ucdbc(&self) -> bool {
        self.ucdbc != 0
    }

    #[doc="Update MMC Counters for Dropped Broadcast Frames"]
    #[inline] pub fn set_ucdbc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

}

impl ::core::fmt::Display for Mmcctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mmcctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cntrst() != 0 { try!(write!(f, " cntrst"))}
        if self.cntstpro() != 0 { try!(write!(f, " cntstpro"))}
        if self.rstonrd() != 0 { try!(write!(f, " rstonrd"))}
        if self.cntfreez() != 0 { try!(write!(f, " cntfreez"))}
        if self.cntprst() != 0 { try!(write!(f, " cntprst"))}
        if self.cntprstlvl() != 0 { try!(write!(f, " cntprstlvl"))}
        if self.ucdbc() != 0 { try!(write!(f, " ucdbc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC MMC Receive Raw Interrupt Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mmcrxris(pub u32);
impl Mmcrxris {
    #[doc="MMC Receive Good Bad Frame Counter Interrupt Status"]
    #[inline] pub fn gbf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="MMC Receive Good Bad Frame Counter Interrupt Status"]
    #[inline] pub fn test_gbf(&self) -> bool {
        self.gbf != 0
    }

    #[doc="MMC Receive Good Bad Frame Counter Interrupt Status"]
    #[inline] pub fn set_gbf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="MMC Receive CRC Error Frame Counter Interrupt Status"]
    #[inline] pub fn crcerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="MMC Receive CRC Error Frame Counter Interrupt Status"]
    #[inline] pub fn test_crcerr(&self) -> bool {
        self.crcerr != 0
    }

    #[doc="MMC Receive CRC Error Frame Counter Interrupt Status"]
    #[inline] pub fn set_crcerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="MMC Receive Alignment Error Frame Counter Interrupt Status"]
    #[inline] pub fn algnerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="MMC Receive Alignment Error Frame Counter Interrupt Status"]
    #[inline] pub fn test_algnerr(&self) -> bool {
        self.algnerr != 0
    }

    #[doc="MMC Receive Alignment Error Frame Counter Interrupt Status"]
    #[inline] pub fn set_algnerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="MMC Receive Unicast Good Frame Counter Interrupt Status"]
    #[inline] pub fn ucgf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="MMC Receive Unicast Good Frame Counter Interrupt Status"]
    #[inline] pub fn test_ucgf(&self) -> bool {
        self.ucgf != 0
    }

    #[doc="MMC Receive Unicast Good Frame Counter Interrupt Status"]
    #[inline] pub fn set_ucgf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

}

impl ::core::fmt::Display for Mmcrxris {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mmcrxris {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gbf() != 0 { try!(write!(f, " gbf"))}
        if self.crcerr() != 0 { try!(write!(f, " crcerr"))}
        if self.algnerr() != 0 { try!(write!(f, " algnerr"))}
        if self.ucgf() != 0 { try!(write!(f, " ucgf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC MMC Transmit Raw Interrupt Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mmctxris(pub u32);
impl Mmctxris {
    #[doc="MMC Transmit Good Bad Frame Counter Interrupt Status"]
    #[inline] pub fn gbf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="MMC Transmit Good Bad Frame Counter Interrupt Status"]
    #[inline] pub fn test_gbf(&self) -> bool {
        self.gbf != 0
    }

    #[doc="MMC Transmit Good Bad Frame Counter Interrupt Status"]
    #[inline] pub fn set_gbf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="MMC Transmit Single Collision Good Frame Counter Interrupt Status"]
    #[inline] pub fn scollgf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="MMC Transmit Single Collision Good Frame Counter Interrupt Status"]
    #[inline] pub fn test_scollgf(&self) -> bool {
        self.scollgf != 0
    }

    #[doc="MMC Transmit Single Collision Good Frame Counter Interrupt Status"]
    #[inline] pub fn set_scollgf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="MMC Transmit Multiple Collision Good Frame Counter Interrupt Status"]
    #[inline] pub fn mcollgf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="MMC Transmit Multiple Collision Good Frame Counter Interrupt Status"]
    #[inline] pub fn test_mcollgf(&self) -> bool {
        self.mcollgf != 0
    }

    #[doc="MMC Transmit Multiple Collision Good Frame Counter Interrupt Status"]
    #[inline] pub fn set_mcollgf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Octet Counter Interrupt Status"]
    #[inline] pub fn octcnt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Octet Counter Interrupt Status"]
    #[inline] pub fn test_octcnt(&self) -> bool {
        self.octcnt != 0
    }

    #[doc="Octet Counter Interrupt Status"]
    #[inline] pub fn set_octcnt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

}

impl ::core::fmt::Display for Mmctxris {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mmctxris {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gbf() != 0 { try!(write!(f, " gbf"))}
        if self.scollgf() != 0 { try!(write!(f, " scollgf"))}
        if self.mcollgf() != 0 { try!(write!(f, " mcollgf"))}
        if self.octcnt() != 0 { try!(write!(f, " octcnt"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC MMC Receive Interrupt Mask"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mmcrxim(pub u32);
impl Mmcrxim {
    #[doc="MMC Receive Good Bad Frame Counter Interrupt Mask"]
    #[inline] pub fn gbf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="MMC Receive Good Bad Frame Counter Interrupt Mask"]
    #[inline] pub fn test_gbf(&self) -> bool {
        self.gbf != 0
    }

    #[doc="MMC Receive Good Bad Frame Counter Interrupt Mask"]
    #[inline] pub fn set_gbf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="MMC Receive CRC Error Frame Counter Interrupt Mask"]
    #[inline] pub fn crcerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="MMC Receive CRC Error Frame Counter Interrupt Mask"]
    #[inline] pub fn test_crcerr(&self) -> bool {
        self.crcerr != 0
    }

    #[doc="MMC Receive CRC Error Frame Counter Interrupt Mask"]
    #[inline] pub fn set_crcerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="MMC Receive Alignment Error Frame Counter Interrupt Mask"]
    #[inline] pub fn algnerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="MMC Receive Alignment Error Frame Counter Interrupt Mask"]
    #[inline] pub fn test_algnerr(&self) -> bool {
        self.algnerr != 0
    }

    #[doc="MMC Receive Alignment Error Frame Counter Interrupt Mask"]
    #[inline] pub fn set_algnerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="MMC Receive Unicast Good Frame Counter Interrupt Mask"]
    #[inline] pub fn ucgf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="MMC Receive Unicast Good Frame Counter Interrupt Mask"]
    #[inline] pub fn test_ucgf(&self) -> bool {
        self.ucgf != 0
    }

    #[doc="MMC Receive Unicast Good Frame Counter Interrupt Mask"]
    #[inline] pub fn set_ucgf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

}

impl ::core::fmt::Display for Mmcrxim {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mmcrxim {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gbf() != 0 { try!(write!(f, " gbf"))}
        if self.crcerr() != 0 { try!(write!(f, " crcerr"))}
        if self.algnerr() != 0 { try!(write!(f, " algnerr"))}
        if self.ucgf() != 0 { try!(write!(f, " ucgf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC MMC Transmit Interrupt Mask"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mmctxim(pub u32);
impl Mmctxim {
    #[doc="MMC Transmit Good Bad Frame Counter Interrupt Mask"]
    #[inline] pub fn gbf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="MMC Transmit Good Bad Frame Counter Interrupt Mask"]
    #[inline] pub fn test_gbf(&self) -> bool {
        self.gbf != 0
    }

    #[doc="MMC Transmit Good Bad Frame Counter Interrupt Mask"]
    #[inline] pub fn set_gbf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="MMC Transmit Single Collision Good Frame Counter Interrupt Mask"]
    #[inline] pub fn scollgf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="MMC Transmit Single Collision Good Frame Counter Interrupt Mask"]
    #[inline] pub fn test_scollgf(&self) -> bool {
        self.scollgf != 0
    }

    #[doc="MMC Transmit Single Collision Good Frame Counter Interrupt Mask"]
    #[inline] pub fn set_scollgf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="MMC Transmit Multiple Collision Good Frame Counter Interrupt Mask"]
    #[inline] pub fn mcollgf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="MMC Transmit Multiple Collision Good Frame Counter Interrupt Mask"]
    #[inline] pub fn test_mcollgf(&self) -> bool {
        self.mcollgf != 0
    }

    #[doc="MMC Transmit Multiple Collision Good Frame Counter Interrupt Mask"]
    #[inline] pub fn set_mcollgf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="MMC Transmit Good Octet Counter Interrupt Mask"]
    #[inline] pub fn octcnt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="MMC Transmit Good Octet Counter Interrupt Mask"]
    #[inline] pub fn test_octcnt(&self) -> bool {
        self.octcnt != 0
    }

    #[doc="MMC Transmit Good Octet Counter Interrupt Mask"]
    #[inline] pub fn set_octcnt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

}

impl ::core::fmt::Display for Mmctxim {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mmctxim {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gbf() != 0 { try!(write!(f, " gbf"))}
        if self.scollgf() != 0 { try!(write!(f, " scollgf"))}
        if self.mcollgf() != 0 { try!(write!(f, " mcollgf"))}
        if self.octcnt() != 0 { try!(write!(f, " octcnt"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Transmit Frame Count for Good and Bad Frames"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Txcntgb(pub u32);
impl Txcntgb {
    #[doc="This field indicates the number of good and bad frames transmitted, exclusive of retried frames"]
    #[inline] pub fn txfrmgb(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="This field indicates the number of good and bad frames transmitted, exclusive of retried frames"]
    #[inline] pub fn test_txfrmgb(&self) -> bool {
        self.txfrmgb != 0
    }

    #[doc="This field indicates the number of good and bad frames transmitted, exclusive of retried frames"]
    #[inline] pub fn set_txfrmgb<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Txcntgb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Txcntgb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Transmit Frame Count for Frames Transmitted after Single Collision"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Txcntscol(pub u32);
impl Txcntscol {
    #[doc="This field indicates the number of successfully transmitted frames after a single collision in the half-duplex mode"]
    #[inline] pub fn txsnglcolg(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="This field indicates the number of successfully transmitted frames after a single collision in the half-duplex mode"]
    #[inline] pub fn test_txsnglcolg(&self) -> bool {
        self.txsnglcolg != 0
    }

    #[doc="This field indicates the number of successfully transmitted frames after a single collision in the half-duplex mode"]
    #[inline] pub fn set_txsnglcolg<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Txcntscol {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Txcntscol {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Transmit Frame Count for Frames Transmitted after Multiple Collisions"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Txcntmcol(pub u32);
impl Txcntmcol {
    #[doc="This field indicates the number of successfully transmitted frames after multiple collisions in the half-duplex mode"]
    #[inline] pub fn txmultcolg(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="This field indicates the number of successfully transmitted frames after multiple collisions in the half-duplex mode"]
    #[inline] pub fn test_txmultcolg(&self) -> bool {
        self.txmultcolg != 0
    }

    #[doc="This field indicates the number of successfully transmitted frames after multiple collisions in the half-duplex mode"]
    #[inline] pub fn set_txmultcolg<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Txcntmcol {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Txcntmcol {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Transmit Octet Count Good"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Txoctcntg(pub u32);
impl Txoctcntg {
    #[doc="This field indicates the number of bytes transmitted, exclusive of preamble, in good frames"]
    #[inline] pub fn txoctg(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="This field indicates the number of bytes transmitted, exclusive of preamble, in good frames"]
    #[inline] pub fn test_txoctg(&self) -> bool {
        self.txoctg != 0
    }

    #[doc="This field indicates the number of bytes transmitted, exclusive of preamble, in good frames"]
    #[inline] pub fn set_txoctg<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Txoctcntg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Txoctcntg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Receive Frame Count for Good and Bad Frames"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rxcntgb(pub u32);
impl Rxcntgb {
    #[doc="This field indicates the number of received good and bad frames"]
    #[inline] pub fn rxfrmgb(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="This field indicates the number of received good and bad frames"]
    #[inline] pub fn test_rxfrmgb(&self) -> bool {
        self.rxfrmgb != 0
    }

    #[doc="This field indicates the number of received good and bad frames"]
    #[inline] pub fn set_rxfrmgb<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Rxcntgb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rxcntgb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Receive Frame Count for CRC Error Frames"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rxcntcrcerr(pub u32);
impl Rxcntcrcerr {
    #[doc="This field indicates the number of frames received with CRC error"]
    #[inline] pub fn rxcrcerr(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="This field indicates the number of frames received with CRC error"]
    #[inline] pub fn test_rxcrcerr(&self) -> bool {
        self.rxcrcerr != 0
    }

    #[doc="This field indicates the number of frames received with CRC error"]
    #[inline] pub fn set_rxcrcerr<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Rxcntcrcerr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rxcntcrcerr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Receive Frame Count for Alignment Error Frames"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rxcntalgnerr(pub u32);
impl Rxcntalgnerr {
    #[doc="This field indicates the number of frames received with alignment (dribble) error"]
    #[inline] pub fn rxalgnerr(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="This field indicates the number of frames received with alignment (dribble) error"]
    #[inline] pub fn test_rxalgnerr(&self) -> bool {
        self.rxalgnerr != 0
    }

    #[doc="This field indicates the number of frames received with alignment (dribble) error"]
    #[inline] pub fn set_rxalgnerr<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Rxcntalgnerr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rxcntalgnerr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Receive Frame Count for Good Unicast Frames"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rxcntguni(pub u32);
impl Rxcntguni {
    #[doc="This field indicates the number of received good unicast frames"]
    #[inline] pub fn rxucastg(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="This field indicates the number of received good unicast frames"]
    #[inline] pub fn test_rxucastg(&self) -> bool {
        self.rxucastg != 0
    }

    #[doc="This field indicates the number of received good unicast frames"]
    #[inline] pub fn set_rxucastg<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Rxcntguni {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rxcntguni {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC VLAN Tag Inclusion or Replacement"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Vlnincrep(pub u32);
impl Vlnincrep {
    #[doc="VLAN Tag for Transmit Frames"]
    #[inline] pub fn vlt(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="VLAN Tag for Transmit Frames"]
    #[inline] pub fn test_vlt(&self) -> bool {
        self.vlt != 0
    }

    #[doc="VLAN Tag for Transmit Frames"]
    #[inline] pub fn set_vlt<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="VLAN Tag Control in Transmit Frames"]
    #[inline] pub fn vlc(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="VLAN Tag Control in Transmit Frames"]
    #[inline] pub fn test_vlc(&self) -> bool {
        self.vlc != 0
    }

    #[doc="VLAN Tag Control in Transmit Frames"]
    #[inline] pub fn set_vlc<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="VLAN Priority Control"]
    #[inline] pub fn vlp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="VLAN Priority Control"]
    #[inline] pub fn test_vlp(&self) -> bool {
        self.vlp != 0
    }

    #[doc="VLAN Priority Control"]
    #[inline] pub fn set_vlp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="C-VLAN or S-VLAN"]
    #[inline] pub fn csvl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="C-VLAN or S-VLAN"]
    #[inline] pub fn test_csvl(&self) -> bool {
        self.csvl != 0
    }

    #[doc="C-VLAN or S-VLAN"]
    #[inline] pub fn set_csvl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

}

impl ::core::fmt::Display for Vlnincrep {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Vlnincrep {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.vlt() != 0 { try!(write!(f, " vlt=0x{:x}", self.vlt()))}
        if self.vlc() != 0 { try!(write!(f, " vlc=0x{:x}", self.vlc()))}
        if self.vlp() != 0 { try!(write!(f, " vlp"))}
        if self.csvl() != 0 { try!(write!(f, " csvl"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC VLAN Hash Table"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Vlanhash(pub u32);
impl Vlanhash {
    #[doc="VLAN Hash Table"]
    #[inline] pub fn vlht(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="VLAN Hash Table"]
    #[inline] pub fn test_vlht(&self) -> bool {
        self.vlht != 0
    }

    #[doc="VLAN Hash Table"]
    #[inline] pub fn set_vlht<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Vlanhash {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Vlanhash {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.vlht() != 0 { try!(write!(f, " vlht=0x{:x}", self.vlht()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Timestamp Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Timstctrl(pub u32);
impl Timstctrl {
    #[doc="Timestamp Enable"]
    #[inline] pub fn tsen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Timestamp Enable"]
    #[inline] pub fn test_tsen(&self) -> bool {
        self.tsen != 0
    }

    #[doc="Timestamp Enable"]
    #[inline] pub fn set_tsen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Timestamp Fine or Coarse Update"]
    #[inline] pub fn tsfcupdt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Timestamp Fine or Coarse Update"]
    #[inline] pub fn test_tsfcupdt(&self) -> bool {
        self.tsfcupdt != 0
    }

    #[doc="Timestamp Fine or Coarse Update"]
    #[inline] pub fn set_tsfcupdt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Timestamp Initialize"]
    #[inline] pub fn tsinit(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Timestamp Initialize"]
    #[inline] pub fn test_tsinit(&self) -> bool {
        self.tsinit != 0
    }

    #[doc="Timestamp Initialize"]
    #[inline] pub fn set_tsinit<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Timestamp Update"]
    #[inline] pub fn tsupdt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Timestamp Update"]
    #[inline] pub fn test_tsupdt(&self) -> bool {
        self.tsupdt != 0
    }

    #[doc="Timestamp Update"]
    #[inline] pub fn set_tsupdt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Timestamp Interrupt Trigger Enable"]
    #[inline] pub fn inttrig(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Timestamp Interrupt Trigger Enable"]
    #[inline] pub fn test_inttrig(&self) -> bool {
        self.inttrig != 0
    }

    #[doc="Timestamp Interrupt Trigger Enable"]
    #[inline] pub fn set_inttrig<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Addend Register Update"]
    #[inline] pub fn addregup(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Addend Register Update"]
    #[inline] pub fn test_addregup(&self) -> bool {
        self.addregup != 0
    }

    #[doc="Addend Register Update"]
    #[inline] pub fn set_addregup<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Enable Timestamp For All Frames"]
    #[inline] pub fn allf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Enable Timestamp For All Frames"]
    #[inline] pub fn test_allf(&self) -> bool {
        self.allf != 0
    }

    #[doc="Enable Timestamp For All Frames"]
    #[inline] pub fn set_allf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Timestamp Digital or Binary Rollover Control"]
    #[inline] pub fn dgtlbin(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Timestamp Digital or Binary Rollover Control"]
    #[inline] pub fn test_dgtlbin(&self) -> bool {
        self.dgtlbin != 0
    }

    #[doc="Timestamp Digital or Binary Rollover Control"]
    #[inline] pub fn set_dgtlbin<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Enable PTP Packet Processing For Version 2 Format"]
    #[inline] pub fn ptpver2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Enable PTP Packet Processing For Version 2 Format"]
    #[inline] pub fn test_ptpver2(&self) -> bool {
        self.ptpver2 != 0
    }

    #[doc="Enable PTP Packet Processing For Version 2 Format"]
    #[inline] pub fn set_ptpver2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Enable Processing of PTP Over Ethernet Frames"]
    #[inline] pub fn ptpeth(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Enable Processing of PTP Over Ethernet Frames"]
    #[inline] pub fn test_ptpeth(&self) -> bool {
        self.ptpeth != 0
    }

    #[doc="Enable Processing of PTP Over Ethernet Frames"]
    #[inline] pub fn set_ptpeth<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Enable Processing of PTP Frames Sent Over IPv6-UDP"]
    #[inline] pub fn ptpipv6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Enable Processing of PTP Frames Sent Over IPv6-UDP"]
    #[inline] pub fn test_ptpipv6(&self) -> bool {
        self.ptpipv6 != 0
    }

    #[doc="Enable Processing of PTP Frames Sent Over IPv6-UDP"]
    #[inline] pub fn set_ptpipv6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Enable Processing of PTP Frames Sent over IPv4-UDP"]
    #[inline] pub fn ptpipv4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Enable Processing of PTP Frames Sent over IPv4-UDP"]
    #[inline] pub fn test_ptpipv4(&self) -> bool {
        self.ptpipv4 != 0
    }

    #[doc="Enable Processing of PTP Frames Sent over IPv4-UDP"]
    #[inline] pub fn set_ptpipv4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Enable Timestamp Snapshot for Event Messages"]
    #[inline] pub fn tsevnt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Enable Timestamp Snapshot for Event Messages"]
    #[inline] pub fn test_tsevnt(&self) -> bool {
        self.tsevnt != 0
    }

    #[doc="Enable Timestamp Snapshot for Event Messages"]
    #[inline] pub fn set_tsevnt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Enable Snapshot for Messages Relevant to Master"]
    #[inline] pub fn tsmast(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Enable Snapshot for Messages Relevant to Master"]
    #[inline] pub fn test_tsmast(&self) -> bool {
        self.tsmast != 0
    }

    #[doc="Enable Snapshot for Messages Relevant to Master"]
    #[inline] pub fn set_tsmast<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Select PTP packets for Taking Snapshots"]
    #[inline] pub fn selptp(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Select PTP packets for Taking Snapshots"]
    #[inline] pub fn test_selptp(&self) -> bool {
        self.selptp != 0
    }

    #[doc="Select PTP packets for Taking Snapshots"]
    #[inline] pub fn set_selptp<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Enable MAC address for PTP Frame Filtering"]
    #[inline] pub fn ptpfltr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Enable MAC address for PTP Frame Filtering"]
    #[inline] pub fn test_ptpfltr(&self) -> bool {
        self.ptpfltr != 0
    }

    #[doc="Enable MAC address for PTP Frame Filtering"]
    #[inline] pub fn set_ptpfltr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

}

impl ::core::fmt::Display for Timstctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Timstctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tsen() != 0 { try!(write!(f, " tsen"))}
        if self.tsfcupdt() != 0 { try!(write!(f, " tsfcupdt"))}
        if self.tsinit() != 0 { try!(write!(f, " tsinit"))}
        if self.tsupdt() != 0 { try!(write!(f, " tsupdt"))}
        if self.inttrig() != 0 { try!(write!(f, " inttrig"))}
        if self.addregup() != 0 { try!(write!(f, " addregup"))}
        if self.allf() != 0 { try!(write!(f, " allf"))}
        if self.dgtlbin() != 0 { try!(write!(f, " dgtlbin"))}
        if self.ptpver2() != 0 { try!(write!(f, " ptpver2"))}
        if self.ptpeth() != 0 { try!(write!(f, " ptpeth"))}
        if self.ptpipv6() != 0 { try!(write!(f, " ptpipv6"))}
        if self.ptpipv4() != 0 { try!(write!(f, " ptpipv4"))}
        if self.tsevnt() != 0 { try!(write!(f, " tsevnt"))}
        if self.tsmast() != 0 { try!(write!(f, " tsmast"))}
        if self.selptp() != 0 { try!(write!(f, " selptp=0x{:x}", self.selptp()))}
        if self.ptpfltr() != 0 { try!(write!(f, " ptpfltr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Sub-Second Increment"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Subsecinc(pub u32);
impl Subsecinc {
    #[doc="Sub-second Increment Value"]
    #[inline] pub fn ssinc(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Sub-second Increment Value"]
    #[inline] pub fn test_ssinc(&self) -> bool {
        self.ssinc != 0
    }

    #[doc="Sub-second Increment Value"]
    #[inline] pub fn set_ssinc<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Subsecinc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Subsecinc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ssinc() != 0 { try!(write!(f, " ssinc=0x{:x}", self.ssinc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC System Time - Seconds"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Timsec(pub u32);
impl Timsec {
    #[doc="Timestamp Second"]
    #[inline] pub fn tss(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Timestamp Second"]
    #[inline] pub fn test_tss(&self) -> bool {
        self.tss != 0
    }

    #[doc="Timestamp Second"]
    #[inline] pub fn set_tss<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Timsec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Timsec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC System Time - Nanoseconds"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Timnano(pub u32);
impl Timnano {
    #[doc="Timestamp Sub-Seconds"]
    #[inline] pub fn tsss(&self) -> bits::U31 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7fffffff) as u32) } // [30:0]
    }

    #[doc="Timestamp Sub-Seconds"]
    #[inline] pub fn test_tsss(&self) -> bool {
        self.tsss != 0
    }

    #[doc="Timestamp Sub-Seconds"]
    #[inline] pub fn set_tsss<V: Into<bits::U31>>(mut self, value: V) -> Self {
        let value: bits::U31 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7fffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Timnano {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Timnano {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tsss() != 0 { try!(write!(f, " tsss=0x{:x}", self.tsss()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC System Time - Seconds Update"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Timsecu(pub u32);
impl Timsecu {
    #[doc="Timestamp Second"]
    #[inline] pub fn tss(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Timestamp Second"]
    #[inline] pub fn test_tss(&self) -> bool {
        self.tss != 0
    }

    #[doc="Timestamp Second"]
    #[inline] pub fn set_tss<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Timsecu {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Timsecu {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC System Time - Nanoseconds Update"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Timnanou(pub u32);
impl Timnanou {
    #[doc="Timestamp Sub-Second"]
    #[inline] pub fn tsss(&self) -> bits::U31 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7fffffff) as u32) } // [30:0]
    }

    #[doc="Timestamp Sub-Second"]
    #[inline] pub fn test_tsss(&self) -> bool {
        self.tsss != 0
    }

    #[doc="Timestamp Sub-Second"]
    #[inline] pub fn set_tsss<V: Into<bits::U31>>(mut self, value: V) -> Self {
        let value: bits::U31 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7fffffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Add or subtract time"]
    #[inline] pub fn addsub(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Add or subtract time"]
    #[inline] pub fn test_addsub(&self) -> bool {
        self.addsub != 0
    }

    #[doc="Add or subtract time"]
    #[inline] pub fn set_addsub<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl ::core::fmt::Display for Timnanou {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Timnanou {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tsss() != 0 { try!(write!(f, " tsss=0x{:x}", self.tsss()))}
        if self.addsub() != 0 { try!(write!(f, " addsub"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Timestamp Addend"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Timadd(pub u32);
impl Timadd {
    #[doc="Timestamp Addend Register"]
    #[inline] pub fn tsar(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Timestamp Addend Register"]
    #[inline] pub fn test_tsar(&self) -> bool {
        self.tsar != 0
    }

    #[doc="Timestamp Addend Register"]
    #[inline] pub fn set_tsar<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Timadd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Timadd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Target Time Seconds"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Targsec(pub u32);
impl Targsec {
    #[doc="Target Time Seconds Register"]
    #[inline] pub fn tstr(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Target Time Seconds Register"]
    #[inline] pub fn test_tstr(&self) -> bool {
        self.tstr != 0
    }

    #[doc="Target Time Seconds Register"]
    #[inline] pub fn set_tstr<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Targsec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Targsec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Target Time Nanoseconds"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Targnano(pub u32);
impl Targnano {
    #[doc="Target Timestamp Low Register"]
    #[inline] pub fn ttslo(&self) -> bits::U31 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7fffffff) as u32) } // [30:0]
    }

    #[doc="Target Timestamp Low Register"]
    #[inline] pub fn test_ttslo(&self) -> bool {
        self.ttslo != 0
    }

    #[doc="Target Timestamp Low Register"]
    #[inline] pub fn set_ttslo<V: Into<bits::U31>>(mut self, value: V) -> Self {
        let value: bits::U31 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7fffffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Target Time Register Busy"]
    #[inline] pub fn trgtbusy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Target Time Register Busy"]
    #[inline] pub fn test_trgtbusy(&self) -> bool {
        self.trgtbusy != 0
    }

    #[doc="Target Time Register Busy"]
    #[inline] pub fn set_trgtbusy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl ::core::fmt::Display for Targnano {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Targnano {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ttslo() != 0 { try!(write!(f, " ttslo=0x{:x}", self.ttslo()))}
        if self.trgtbusy() != 0 { try!(write!(f, " trgtbusy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC System Time-Higher Word Seconds"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hwordsec(pub u32);
impl Hwordsec {
    #[doc="Target Timestamp Higher Word Register"]
    #[inline] pub fn tshwr(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Target Timestamp Higher Word Register"]
    #[inline] pub fn test_tshwr(&self) -> bool {
        self.tshwr != 0
    }

    #[doc="Target Timestamp Higher Word Register"]
    #[inline] pub fn set_tshwr<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Hwordsec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hwordsec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tshwr() != 0 { try!(write!(f, " tshwr=0x{:x}", self.tshwr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Timestamp Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Timstat(pub u32);
impl Timstat {
    #[doc="Timestamp Seconds Overflow"]
    #[inline] pub fn tssovf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Timestamp Seconds Overflow"]
    #[inline] pub fn test_tssovf(&self) -> bool {
        self.tssovf != 0
    }

    #[doc="Timestamp Seconds Overflow"]
    #[inline] pub fn set_tssovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Timestamp Target Time Reached"]
    #[inline] pub fn tstargt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Timestamp Target Time Reached"]
    #[inline] pub fn test_tstargt(&self) -> bool {
        self.tstargt != 0
    }

    #[doc="Timestamp Target Time Reached"]
    #[inline] pub fn set_tstargt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl ::core::fmt::Display for Timstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Timstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tssovf() != 0 { try!(write!(f, " tssovf"))}
        if self.tstargt() != 0 { try!(write!(f, " tstargt"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC PPS Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ppsctrl(pub u32);
impl Ppsctrl {
    #[doc="EN0PPS Output Frequency Control (PPSCTRL) or Command Control (PPSCMD)"]
    #[inline] pub fn ppsctrl(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="EN0PPS Output Frequency Control (PPSCTRL) or Command Control (PPSCMD)"]
    #[inline] pub fn test_ppsctrl(&self) -> bool {
        self.ppsctrl != 0
    }

    #[doc="EN0PPS Output Frequency Control (PPSCTRL) or Command Control (PPSCMD)"]
    #[inline] pub fn set_ppsctrl<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Flexible PPS Output Mode Enable"]
    #[inline] pub fn ppsen0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Flexible PPS Output Mode Enable"]
    #[inline] pub fn test_ppsen0(&self) -> bool {
        self.ppsen0 != 0
    }

    #[doc="Flexible PPS Output Mode Enable"]
    #[inline] pub fn set_ppsen0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Target Time Register Mode for PPS0 Output"]
    #[inline] pub fn trgmods0(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="Target Time Register Mode for PPS0 Output"]
    #[inline] pub fn test_trgmods0(&self) -> bool {
        self.trgmods0 != 0
    }

    #[doc="Target Time Register Mode for PPS0 Output"]
    #[inline] pub fn set_trgmods0<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

}

impl ::core::fmt::Display for Ppsctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ppsctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ppsctrl() != 0 { try!(write!(f, " ppsctrl=0x{:x}", self.ppsctrl()))}
        if self.ppsen0() != 0 { try!(write!(f, " ppsen0"))}
        if self.trgmods0() != 0 { try!(write!(f, " trgmods0=0x{:x}", self.trgmods0()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC PPS0 Interval"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pps0intvl(pub u32);
impl Pps0intvl {
    #[doc="PPS0 Output Signal Interval"]
    #[inline] pub fn pps0int(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="PPS0 Output Signal Interval"]
    #[inline] pub fn test_pps0int(&self) -> bool {
        self.pps0int != 0
    }

    #[doc="PPS0 Output Signal Interval"]
    #[inline] pub fn set_pps0int<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Pps0intvl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pps0intvl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC PPS0 Width"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pps0width(pub u32);
impl Pps0width {
    #[doc="EN0PPS Output Signal Width"]
    #[inline] pub fn emac_pps0width(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="EN0PPS Output Signal Width"]
    #[inline] pub fn test_emac_pps0width(&self) -> bool {
        self.emac_pps0width != 0
    }

    #[doc="EN0PPS Output Signal Width"]
    #[inline] pub fn set_emac_pps0width<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Pps0width {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pps0width {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC DMA Bus Mode"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dmabusmod(pub u32);
impl Dmabusmod {
    #[doc="DMA Software Reset"]
    #[inline] pub fn swr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="DMA Software Reset"]
    #[inline] pub fn test_swr(&self) -> bool {
        self.swr != 0
    }

    #[doc="DMA Software Reset"]
    #[inline] pub fn set_swr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="DMA Arbitration Scheme"]
    #[inline] pub fn da(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="DMA Arbitration Scheme"]
    #[inline] pub fn test_da(&self) -> bool {
        self.da != 0
    }

    #[doc="DMA Arbitration Scheme"]
    #[inline] pub fn set_da<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Descriptor Skip Length"]
    #[inline] pub fn dsl(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1f) as u8) } // [6:2]
    }

    #[doc="Descriptor Skip Length"]
    #[inline] pub fn test_dsl(&self) -> bool {
        self.dsl != 0
    }

    #[doc="Descriptor Skip Length"]
    #[inline] pub fn set_dsl<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Alternate Descriptor Size"]
    #[inline] pub fn atds(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Alternate Descriptor Size"]
    #[inline] pub fn test_atds(&self) -> bool {
        self.atds != 0
    }

    #[doc="Alternate Descriptor Size"]
    #[inline] pub fn set_atds<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Programmable Burst Length"]
    #[inline] pub fn pbl(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3f) as u8) } // [13:8]
    }

    #[doc="Programmable Burst Length"]
    #[inline] pub fn test_pbl(&self) -> bool {
        self.pbl != 0
    }

    #[doc="Programmable Burst Length"]
    #[inline] pub fn set_pbl<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Priority Ratio"]
    #[inline] pub fn pr(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x3) as u8) } // [15:14]
    }

    #[doc="Priority Ratio"]
    #[inline] pub fn test_pr(&self) -> bool {
        self.pr != 0
    }

    #[doc="Priority Ratio"]
    #[inline] pub fn set_pr<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Fixed Burst"]
    #[inline] pub fn fb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Fixed Burst"]
    #[inline] pub fn test_fb(&self) -> bool {
        self.fb != 0
    }

    #[doc="Fixed Burst"]
    #[inline] pub fn set_fb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="RX DMA Programmable Burst Length (PBL)"]
    #[inline] pub fn rpbl(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x3f) as u8) } // [22:17]
    }

    #[doc="RX DMA Programmable Burst Length (PBL)"]
    #[inline] pub fn test_rpbl(&self) -> bool {
        self.rpbl != 0
    }

    #[doc="RX DMA Programmable Burst Length (PBL)"]
    #[inline] pub fn set_rpbl<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Use Separate Programmable Burst Length (PBL)"]
    #[inline] pub fn usp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Use Separate Programmable Burst Length (PBL)"]
    #[inline] pub fn test_usp(&self) -> bool {
        self.usp != 0
    }

    #[doc="Use Separate Programmable Burst Length (PBL)"]
    #[inline] pub fn set_usp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="8 x Programmable Burst Length (PBL) Mode"]
    #[inline] pub fn _8xpbl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="8 x Programmable Burst Length (PBL) Mode"]
    #[inline] pub fn test_8xpbl(&self) -> bool {
        self._8xpbl != 0
    }

    #[doc="8 x Programmable Burst Length (PBL) Mode"]
    #[inline] pub fn set_8xpbl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Address Aligned Beats"]
    #[inline] pub fn aal(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Address Aligned Beats"]
    #[inline] pub fn test_aal(&self) -> bool {
        self.aal != 0
    }

    #[doc="Address Aligned Beats"]
    #[inline] pub fn set_aal<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Mixed Burst"]
    #[inline] pub fn mb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Mixed Burst"]
    #[inline] pub fn test_mb(&self) -> bool {
        self.mb != 0
    }

    #[doc="Mixed Burst"]
    #[inline] pub fn set_mb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Transmit Priority"]
    #[inline] pub fn txpr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Transmit Priority"]
    #[inline] pub fn test_txpr(&self) -> bool {
        self.txpr != 0
    }

    #[doc="Transmit Priority"]
    #[inline] pub fn set_txpr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Rebuild Burst"]
    #[inline] pub fn rib(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Rebuild Burst"]
    #[inline] pub fn test_rib(&self) -> bool {
        self.rib != 0
    }

    #[doc="Rebuild Burst"]
    #[inline] pub fn set_rib<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl ::core::fmt::Display for Dmabusmod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dmabusmod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.swr() != 0 { try!(write!(f, " swr"))}
        if self.da() != 0 { try!(write!(f, " da"))}
        if self.dsl() != 0 { try!(write!(f, " dsl=0x{:x}", self.dsl()))}
        if self.atds() != 0 { try!(write!(f, " atds"))}
        if self.pbl() != 0 { try!(write!(f, " pbl=0x{:x}", self.pbl()))}
        if self.pr() != 0 { try!(write!(f, " pr=0x{:x}", self.pr()))}
        if self.fb() != 0 { try!(write!(f, " fb"))}
        if self.rpbl() != 0 { try!(write!(f, " rpbl=0x{:x}", self.rpbl()))}
        if self.usp() != 0 { try!(write!(f, " usp"))}
        if self._8xpbl() != 0 { try!(write!(f, " _8xpbl"))}
        if self.aal() != 0 { try!(write!(f, " aal"))}
        if self.mb() != 0 { try!(write!(f, " mb"))}
        if self.txpr() != 0 { try!(write!(f, " txpr"))}
        if self.rib() != 0 { try!(write!(f, " rib"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Transmit Poll Demand"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Txpolld(pub u32);
impl Txpolld {
    #[doc="Transmit Poll Demand"]
    #[inline] pub fn tpd(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Transmit Poll Demand"]
    #[inline] pub fn test_tpd(&self) -> bool {
        self.tpd != 0
    }

    #[doc="Transmit Poll Demand"]
    #[inline] pub fn set_tpd<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Txpolld {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Txpolld {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Receive Poll Demand"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rxpolld(pub u32);
impl Rxpolld {
    #[doc="Receive Poll Demand"]
    #[inline] pub fn rpd(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Receive Poll Demand"]
    #[inline] pub fn test_rpd(&self) -> bool {
        self.rpd != 0
    }

    #[doc="Receive Poll Demand"]
    #[inline] pub fn set_rpd<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Rxpolld {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rxpolld {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Receive Descriptor List Address"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rxdladdr(pub u32);
impl Rxdladdr {
    #[doc="Start of Receive List"]
    #[inline] pub fn strxlist(&self) -> bits::U30 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3fffffff) as u32) } // [31:2]
    }

    #[doc="Start of Receive List"]
    #[inline] pub fn test_strxlist(&self) -> bool {
        self.strxlist != 0
    }

    #[doc="Start of Receive List"]
    #[inline] pub fn set_strxlist<V: Into<bits::U30>>(mut self, value: V) -> Self {
        let value: bits::U30 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3fffffff << 2);
        self.0 |= value << 2;
        self
    }

}

impl ::core::fmt::Display for Rxdladdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rxdladdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.strxlist() != 0 { try!(write!(f, " strxlist=0x{:x}", self.strxlist()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Transmit Descriptor List Address"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Txdladdr(pub u32);
impl Txdladdr {
    #[doc="Start of Transmit List Base Address"]
    #[inline] pub fn txdladdr(&self) -> bits::U30 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3fffffff) as u32) } // [31:2]
    }

    #[doc="Start of Transmit List Base Address"]
    #[inline] pub fn test_txdladdr(&self) -> bool {
        self.txdladdr != 0
    }

    #[doc="Start of Transmit List Base Address"]
    #[inline] pub fn set_txdladdr<V: Into<bits::U30>>(mut self, value: V) -> Self {
        let value: bits::U30 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3fffffff << 2);
        self.0 |= value << 2;
        self
    }

}

impl ::core::fmt::Display for Txdladdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Txdladdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txdladdr() != 0 { try!(write!(f, " txdladdr=0x{:x}", self.txdladdr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC DMA Interrupt Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dmaris(pub u32);
impl Dmaris {
    #[doc="Transmit Interrupt"]
    #[inline] pub fn ti(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Transmit Interrupt"]
    #[inline] pub fn test_ti(&self) -> bool {
        self.ti != 0
    }

    #[doc="Transmit Interrupt"]
    #[inline] pub fn set_ti<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transmit Process Stopped"]
    #[inline] pub fn tps(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Transmit Process Stopped"]
    #[inline] pub fn test_tps(&self) -> bool {
        self.tps != 0
    }

    #[doc="Transmit Process Stopped"]
    #[inline] pub fn set_tps<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Transmit Buffer Unavailable"]
    #[inline] pub fn tu(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Transmit Buffer Unavailable"]
    #[inline] pub fn test_tu(&self) -> bool {
        self.tu != 0
    }

    #[doc="Transmit Buffer Unavailable"]
    #[inline] pub fn set_tu<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Transmit Jabber Timeout"]
    #[inline] pub fn tjt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Transmit Jabber Timeout"]
    #[inline] pub fn test_tjt(&self) -> bool {
        self.tjt != 0
    }

    #[doc="Transmit Jabber Timeout"]
    #[inline] pub fn set_tjt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Receive Overflow"]
    #[inline] pub fn ovf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Receive Overflow"]
    #[inline] pub fn test_ovf(&self) -> bool {
        self.ovf != 0
    }

    #[doc="Receive Overflow"]
    #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Transmit Underflow"]
    #[inline] pub fn unf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Transmit Underflow"]
    #[inline] pub fn test_unf(&self) -> bool {
        self.unf != 0
    }

    #[doc="Transmit Underflow"]
    #[inline] pub fn set_unf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Receive Interrupt"]
    #[inline] pub fn ri(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Receive Interrupt"]
    #[inline] pub fn test_ri(&self) -> bool {
        self.ri != 0
    }

    #[doc="Receive Interrupt"]
    #[inline] pub fn set_ri<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Receive Buffer Unavailable"]
    #[inline] pub fn ru(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Receive Buffer Unavailable"]
    #[inline] pub fn test_ru(&self) -> bool {
        self.ru != 0
    }

    #[doc="Receive Buffer Unavailable"]
    #[inline] pub fn set_ru<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Receive Process Stopped"]
    #[inline] pub fn rps(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Receive Process Stopped"]
    #[inline] pub fn test_rps(&self) -> bool {
        self.rps != 0
    }

    #[doc="Receive Process Stopped"]
    #[inline] pub fn set_rps<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Receive Watchdog Timeout"]
    #[inline] pub fn rwt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Receive Watchdog Timeout"]
    #[inline] pub fn test_rwt(&self) -> bool {
        self.rwt != 0
    }

    #[doc="Receive Watchdog Timeout"]
    #[inline] pub fn set_rwt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Early Transmit Interrupt"]
    #[inline] pub fn eti(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Early Transmit Interrupt"]
    #[inline] pub fn test_eti(&self) -> bool {
        self.eti != 0
    }

    #[doc="Early Transmit Interrupt"]
    #[inline] pub fn set_eti<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Fatal Bus Error Interrupt"]
    #[inline] pub fn fbi(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Fatal Bus Error Interrupt"]
    #[inline] pub fn test_fbi(&self) -> bool {
        self.fbi != 0
    }

    #[doc="Fatal Bus Error Interrupt"]
    #[inline] pub fn set_fbi<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Early Receive Interrupt"]
    #[inline] pub fn eri(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Early Receive Interrupt"]
    #[inline] pub fn test_eri(&self) -> bool {
        self.eri != 0
    }

    #[doc="Early Receive Interrupt"]
    #[inline] pub fn set_eri<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Abnormal Interrupt Summary"]
    #[inline] pub fn ais(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Abnormal Interrupt Summary"]
    #[inline] pub fn test_ais(&self) -> bool {
        self.ais != 0
    }

    #[doc="Abnormal Interrupt Summary"]
    #[inline] pub fn set_ais<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Normal Interrupt Summary"]
    #[inline] pub fn nis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Normal Interrupt Summary"]
    #[inline] pub fn test_nis(&self) -> bool {
        self.nis != 0
    }

    #[doc="Normal Interrupt Summary"]
    #[inline] pub fn set_nis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Received Process State"]
    #[inline] pub fn rs(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x7) as u8) } // [19:17]
    }

    #[doc="Received Process State"]
    #[inline] pub fn test_rs(&self) -> bool {
        self.rs != 0
    }

    #[doc="Received Process State"]
    #[inline] pub fn set_rs<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Transmit Process State"]
    #[inline] pub fn ts(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x7) as u8) } // [22:20]
    }

    #[doc="Transmit Process State"]
    #[inline] pub fn test_ts(&self) -> bool {
        self.ts != 0
    }

    #[doc="Transmit Process State"]
    #[inline] pub fn set_ts<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Access Error"]
    #[inline] pub fn ae(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x7) as u8) } // [25:23]
    }

    #[doc="Access Error"]
    #[inline] pub fn test_ae(&self) -> bool {
        self.ae != 0
    }

    #[doc="Access Error"]
    #[inline] pub fn set_ae<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="MAC MMC Interrupt"]
    #[inline] pub fn mmc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="MAC MMC Interrupt"]
    #[inline] pub fn test_mmc(&self) -> bool {
        self.mmc != 0
    }

    #[doc="MAC MMC Interrupt"]
    #[inline] pub fn set_mmc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="MAC PMT Interrupt Status"]
    #[inline] pub fn pmt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="MAC PMT Interrupt Status"]
    #[inline] pub fn test_pmt(&self) -> bool {
        self.pmt != 0
    }

    #[doc="MAC PMT Interrupt Status"]
    #[inline] pub fn set_pmt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Timestamp Trigger Interrupt Status"]
    #[inline] pub fn tt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Timestamp Trigger Interrupt Status"]
    #[inline] pub fn test_tt(&self) -> bool {
        self.tt != 0
    }

    #[doc="Timestamp Trigger Interrupt Status"]
    #[inline] pub fn set_tt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

}

impl ::core::fmt::Display for Dmaris {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dmaris {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ti() != 0 { try!(write!(f, " ti"))}
        if self.tps() != 0 { try!(write!(f, " tps"))}
        if self.tu() != 0 { try!(write!(f, " tu"))}
        if self.tjt() != 0 { try!(write!(f, " tjt"))}
        if self.ovf() != 0 { try!(write!(f, " ovf"))}
        if self.unf() != 0 { try!(write!(f, " unf"))}
        if self.ri() != 0 { try!(write!(f, " ri"))}
        if self.ru() != 0 { try!(write!(f, " ru"))}
        if self.rps() != 0 { try!(write!(f, " rps"))}
        if self.rwt() != 0 { try!(write!(f, " rwt"))}
        if self.eti() != 0 { try!(write!(f, " eti"))}
        if self.fbi() != 0 { try!(write!(f, " fbi"))}
        if self.eri() != 0 { try!(write!(f, " eri"))}
        if self.ais() != 0 { try!(write!(f, " ais"))}
        if self.nis() != 0 { try!(write!(f, " nis"))}
        if self.rs() != 0 { try!(write!(f, " rs=0x{:x}", self.rs()))}
        if self.ts() != 0 { try!(write!(f, " ts=0x{:x}", self.ts()))}
        if self.ae() != 0 { try!(write!(f, " ae=0x{:x}", self.ae()))}
        if self.mmc() != 0 { try!(write!(f, " mmc"))}
        if self.pmt() != 0 { try!(write!(f, " pmt"))}
        if self.tt() != 0 { try!(write!(f, " tt"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC DMA Operation Mode"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dmaopmode(pub u32);
impl Dmaopmode {
    #[doc="Start or Stop Receive"]
    #[inline] pub fn sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Start or Stop Receive"]
    #[inline] pub fn test_sr(&self) -> bool {
        self.sr != 0
    }

    #[doc="Start or Stop Receive"]
    #[inline] pub fn set_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Operate on Second Frame"]
    #[inline] pub fn osf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Operate on Second Frame"]
    #[inline] pub fn test_osf(&self) -> bool {
        self.osf != 0
    }

    #[doc="Operate on Second Frame"]
    #[inline] pub fn set_osf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Receive Threshold Control"]
    #[inline] pub fn rtc(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
    }

    #[doc="Receive Threshold Control"]
    #[inline] pub fn test_rtc(&self) -> bool {
        self.rtc != 0
    }

    #[doc="Receive Threshold Control"]
    #[inline] pub fn set_rtc<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Drop Giant Frame Enable"]
    #[inline] pub fn dgf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Drop Giant Frame Enable"]
    #[inline] pub fn test_dgf(&self) -> bool {
        self.dgf != 0
    }

    #[doc="Drop Giant Frame Enable"]
    #[inline] pub fn set_dgf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Forward Undersized Good Frames"]
    #[inline] pub fn fuf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Forward Undersized Good Frames"]
    #[inline] pub fn test_fuf(&self) -> bool {
        self.fuf != 0
    }

    #[doc="Forward Undersized Good Frames"]
    #[inline] pub fn set_fuf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Forward Error Frames"]
    #[inline] pub fn fef(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Forward Error Frames"]
    #[inline] pub fn test_fef(&self) -> bool {
        self.fef != 0
    }

    #[doc="Forward Error Frames"]
    #[inline] pub fn set_fef<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Start or Stop Transmission Command"]
    #[inline] pub fn st(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Start or Stop Transmission Command"]
    #[inline] pub fn test_st(&self) -> bool {
        self.st != 0
    }

    #[doc="Start or Stop Transmission Command"]
    #[inline] pub fn set_st<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Transmit Threshold Control"]
    #[inline] pub fn ttc(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x7) as u8) } // [16:14]
    }

    #[doc="Transmit Threshold Control"]
    #[inline] pub fn test_ttc(&self) -> bool {
        self.ttc != 0
    }

    #[doc="Transmit Threshold Control"]
    #[inline] pub fn set_ttc<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Flush Transmit FIFO"]
    #[inline] pub fn ftf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Flush Transmit FIFO"]
    #[inline] pub fn test_ftf(&self) -> bool {
        self.ftf != 0
    }

    #[doc="Flush Transmit FIFO"]
    #[inline] pub fn set_ftf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Transmit Store and Forward"]
    #[inline] pub fn tsf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Transmit Store and Forward"]
    #[inline] pub fn test_tsf(&self) -> bool {
        self.tsf != 0
    }

    #[doc="Transmit Store and Forward"]
    #[inline] pub fn set_tsf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Disable Flushing of Received Frames"]
    #[inline] pub fn dff(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Disable Flushing of Received Frames"]
    #[inline] pub fn test_dff(&self) -> bool {
        self.dff != 0
    }

    #[doc="Disable Flushing of Received Frames"]
    #[inline] pub fn set_dff<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Receive Store and Forward"]
    #[inline] pub fn rsf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Receive Store and Forward"]
    #[inline] pub fn test_rsf(&self) -> bool {
        self.rsf != 0
    }

    #[doc="Receive Store and Forward"]
    #[inline] pub fn set_rsf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Disable Dropping of TCP/IP Checksum Error Frames"]
    #[inline] pub fn dt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Disable Dropping of TCP/IP Checksum Error Frames"]
    #[inline] pub fn test_dt(&self) -> bool {
        self.dt != 0
    }

    #[doc="Disable Dropping of TCP/IP Checksum Error Frames"]
    #[inline] pub fn set_dt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

}

impl ::core::fmt::Display for Dmaopmode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dmaopmode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sr() != 0 { try!(write!(f, " sr"))}
        if self.osf() != 0 { try!(write!(f, " osf"))}
        if self.rtc() != 0 { try!(write!(f, " rtc=0x{:x}", self.rtc()))}
        if self.dgf() != 0 { try!(write!(f, " dgf"))}
        if self.fuf() != 0 { try!(write!(f, " fuf"))}
        if self.fef() != 0 { try!(write!(f, " fef"))}
        if self.st() != 0 { try!(write!(f, " st"))}
        if self.ttc() != 0 { try!(write!(f, " ttc=0x{:x}", self.ttc()))}
        if self.ftf() != 0 { try!(write!(f, " ftf"))}
        if self.tsf() != 0 { try!(write!(f, " tsf"))}
        if self.dff() != 0 { try!(write!(f, " dff"))}
        if self.rsf() != 0 { try!(write!(f, " rsf"))}
        if self.dt() != 0 { try!(write!(f, " dt"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC DMA Interrupt Mask Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dmaim(pub u32);
impl Dmaim {
    #[doc="Transmit Interrupt Enable"]
    #[inline] pub fn tie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Transmit Interrupt Enable"]
    #[inline] pub fn test_tie(&self) -> bool {
        self.tie != 0
    }

    #[doc="Transmit Interrupt Enable"]
    #[inline] pub fn set_tie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transmit Stopped Enable"]
    #[inline] pub fn tse(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Transmit Stopped Enable"]
    #[inline] pub fn test_tse(&self) -> bool {
        self.tse != 0
    }

    #[doc="Transmit Stopped Enable"]
    #[inline] pub fn set_tse<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Transmit Buffer Unvailable Enable"]
    #[inline] pub fn tue(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Transmit Buffer Unvailable Enable"]
    #[inline] pub fn test_tue(&self) -> bool {
        self.tue != 0
    }

    #[doc="Transmit Buffer Unvailable Enable"]
    #[inline] pub fn set_tue<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Transmit Jabber Timeout Enable"]
    #[inline] pub fn tje(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Transmit Jabber Timeout Enable"]
    #[inline] pub fn test_tje(&self) -> bool {
        self.tje != 0
    }

    #[doc="Transmit Jabber Timeout Enable"]
    #[inline] pub fn set_tje<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Overflow Interrupt Enable"]
    #[inline] pub fn ove(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Overflow Interrupt Enable"]
    #[inline] pub fn test_ove(&self) -> bool {
        self.ove != 0
    }

    #[doc="Overflow Interrupt Enable"]
    #[inline] pub fn set_ove<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Underflow Interrupt Enable"]
    #[inline] pub fn une(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Underflow Interrupt Enable"]
    #[inline] pub fn test_une(&self) -> bool {
        self.une != 0
    }

    #[doc="Underflow Interrupt Enable"]
    #[inline] pub fn set_une<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Receive Interrupt Enable"]
    #[inline] pub fn rie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Receive Interrupt Enable"]
    #[inline] pub fn test_rie(&self) -> bool {
        self.rie != 0
    }

    #[doc="Receive Interrupt Enable"]
    #[inline] pub fn set_rie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Receive Buffer Unavailable Enable"]
    #[inline] pub fn rue(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Receive Buffer Unavailable Enable"]
    #[inline] pub fn test_rue(&self) -> bool {
        self.rue != 0
    }

    #[doc="Receive Buffer Unavailable Enable"]
    #[inline] pub fn set_rue<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Receive Stopped Enable"]
    #[inline] pub fn rse(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Receive Stopped Enable"]
    #[inline] pub fn test_rse(&self) -> bool {
        self.rse != 0
    }

    #[doc="Receive Stopped Enable"]
    #[inline] pub fn set_rse<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Receive Watchdog Timeout Enable"]
    #[inline] pub fn rwe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Receive Watchdog Timeout Enable"]
    #[inline] pub fn test_rwe(&self) -> bool {
        self.rwe != 0
    }

    #[doc="Receive Watchdog Timeout Enable"]
    #[inline] pub fn set_rwe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Early Transmit Interrupt Enable"]
    #[inline] pub fn ete(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Early Transmit Interrupt Enable"]
    #[inline] pub fn test_ete(&self) -> bool {
        self.ete != 0
    }

    #[doc="Early Transmit Interrupt Enable"]
    #[inline] pub fn set_ete<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Fatal Bus Error Enable"]
    #[inline] pub fn fbe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Fatal Bus Error Enable"]
    #[inline] pub fn test_fbe(&self) -> bool {
        self.fbe != 0
    }

    #[doc="Fatal Bus Error Enable"]
    #[inline] pub fn set_fbe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Early Receive Interrupt Enable"]
    #[inline] pub fn ere(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Early Receive Interrupt Enable"]
    #[inline] pub fn test_ere(&self) -> bool {
        self.ere != 0
    }

    #[doc="Early Receive Interrupt Enable"]
    #[inline] pub fn set_ere<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Abnormal Interrupt Summary Enable"]
    #[inline] pub fn aie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Abnormal Interrupt Summary Enable"]
    #[inline] pub fn test_aie(&self) -> bool {
        self.aie != 0
    }

    #[doc="Abnormal Interrupt Summary Enable"]
    #[inline] pub fn set_aie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Normal Interrupt Summary Enable"]
    #[inline] pub fn nie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Normal Interrupt Summary Enable"]
    #[inline] pub fn test_nie(&self) -> bool {
        self.nie != 0
    }

    #[doc="Normal Interrupt Summary Enable"]
    #[inline] pub fn set_nie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

}

impl ::core::fmt::Display for Dmaim {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dmaim {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tie() != 0 { try!(write!(f, " tie"))}
        if self.tse() != 0 { try!(write!(f, " tse"))}
        if self.tue() != 0 { try!(write!(f, " tue"))}
        if self.tje() != 0 { try!(write!(f, " tje"))}
        if self.ove() != 0 { try!(write!(f, " ove"))}
        if self.une() != 0 { try!(write!(f, " une"))}
        if self.rie() != 0 { try!(write!(f, " rie"))}
        if self.rue() != 0 { try!(write!(f, " rue"))}
        if self.rse() != 0 { try!(write!(f, " rse"))}
        if self.rwe() != 0 { try!(write!(f, " rwe"))}
        if self.ete() != 0 { try!(write!(f, " ete"))}
        if self.fbe() != 0 { try!(write!(f, " fbe"))}
        if self.ere() != 0 { try!(write!(f, " ere"))}
        if self.aie() != 0 { try!(write!(f, " aie"))}
        if self.nie() != 0 { try!(write!(f, " nie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Missed Frame and Buffer Overflow Counter"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mfboc(pub u32);
impl Mfboc {
    #[doc="Missed Frame Counter"]
    #[inline] pub fn misfrmcnt(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Missed Frame Counter"]
    #[inline] pub fn test_misfrmcnt(&self) -> bool {
        self.misfrmcnt != 0
    }

    #[doc="Missed Frame Counter"]
    #[inline] pub fn set_misfrmcnt<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Overflow bit for Missed Frame Counter"]
    #[inline] pub fn miscntovf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Overflow bit for Missed Frame Counter"]
    #[inline] pub fn test_miscntovf(&self) -> bool {
        self.miscntovf != 0
    }

    #[doc="Overflow bit for Missed Frame Counter"]
    #[inline] pub fn set_miscntovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Overflow Frame Counter"]
    #[inline] pub fn ovffrmcnt(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x7ff) as u16) } // [27:17]
    }

    #[doc="Overflow Frame Counter"]
    #[inline] pub fn test_ovffrmcnt(&self) -> bool {
        self.ovffrmcnt != 0
    }

    #[doc="Overflow Frame Counter"]
    #[inline] pub fn set_ovffrmcnt<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Overflow Bit for FIFO Overflow Counter"]
    #[inline] pub fn ovfcntovf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Overflow Bit for FIFO Overflow Counter"]
    #[inline] pub fn test_ovfcntovf(&self) -> bool {
        self.ovfcntovf != 0
    }

    #[doc="Overflow Bit for FIFO Overflow Counter"]
    #[inline] pub fn set_ovfcntovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

}

impl ::core::fmt::Display for Mfboc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mfboc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.misfrmcnt() != 0 { try!(write!(f, " misfrmcnt=0x{:x}", self.misfrmcnt()))}
        if self.miscntovf() != 0 { try!(write!(f, " miscntovf"))}
        if self.ovffrmcnt() != 0 { try!(write!(f, " ovffrmcnt=0x{:x}", self.ovffrmcnt()))}
        if self.ovfcntovf() != 0 { try!(write!(f, " ovfcntovf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Receive Interrupt Watchdog Timer"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rxintwdt(pub u32);
impl Rxintwdt {
    #[doc="Receive Interrupt Watchdog Timer Count"]
    #[inline] pub fn riwt(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Receive Interrupt Watchdog Timer Count"]
    #[inline] pub fn test_riwt(&self) -> bool {
        self.riwt != 0
    }

    #[doc="Receive Interrupt Watchdog Timer Count"]
    #[inline] pub fn set_riwt<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Rxintwdt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rxintwdt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.riwt() != 0 { try!(write!(f, " riwt=0x{:x}", self.riwt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Current Host Transmit Descriptor"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hostxdesc(pub u32);
impl Hostxdesc {
    #[doc="Host Transmit Descriptor Address Pointer"]
    #[inline] pub fn curtxdesc(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Host Transmit Descriptor Address Pointer"]
    #[inline] pub fn test_curtxdesc(&self) -> bool {
        self.curtxdesc != 0
    }

    #[doc="Host Transmit Descriptor Address Pointer"]
    #[inline] pub fn set_curtxdesc<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Hostxdesc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hostxdesc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Current Host Receive Descriptor"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hosrxdesc(pub u32);
impl Hosrxdesc {
    #[doc="Host Receive Descriptor Address Pointer"]
    #[inline] pub fn currxdesc(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Host Receive Descriptor Address Pointer"]
    #[inline] pub fn test_currxdesc(&self) -> bool {
        self.currxdesc != 0
    }

    #[doc="Host Receive Descriptor Address Pointer"]
    #[inline] pub fn set_currxdesc<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Hosrxdesc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hosrxdesc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Current Host Transmit Buffer Address"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hostxba(pub u32);
impl Hostxba {
    #[doc="Host Transmit Buffer Address Pointer"]
    #[inline] pub fn curtxbufa(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Host Transmit Buffer Address Pointer"]
    #[inline] pub fn test_curtxbufa(&self) -> bool {
        self.curtxbufa != 0
    }

    #[doc="Host Transmit Buffer Address Pointer"]
    #[inline] pub fn set_curtxbufa<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Hostxba {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hostxba {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Current Host Receive Buffer Address"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hosrxba(pub u32);
impl Hosrxba {
    #[doc="Host Receive Buffer Address Pointer"]
    #[inline] pub fn currxbufa(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Host Receive Buffer Address Pointer"]
    #[inline] pub fn test_currxbufa(&self) -> bool {
        self.currxbufa != 0
    }

    #[doc="Host Receive Buffer Address Pointer"]
    #[inline] pub fn set_currxbufa<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Hosrxba {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hosrxba {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Peripheral Property Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pp(pub u32);
impl Pp {
    #[doc="Ethernet PHY Type"]
    #[inline] pub fn phytype(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Ethernet PHY Type"]
    #[inline] pub fn test_phytype(&self) -> bool {
        self.phytype != 0
    }

    #[doc="Ethernet PHY Type"]
    #[inline] pub fn set_phytype<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Ethernet MAC Type"]
    #[inline] pub fn mactype(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Ethernet MAC Type"]
    #[inline] pub fn test_mactype(&self) -> bool {
        self.mactype != 0
    }

    #[doc="Ethernet MAC Type"]
    #[inline] pub fn set_mactype<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
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
        if self.phytype() != 0 { try!(write!(f, " phytype=0x{:x}", self.phytype()))}
        if self.mactype() != 0 { try!(write!(f, " mactype=0x{:x}", self.mactype()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Peripheral Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pc(pub u32);
impl Pc {
    #[doc="Ethernet PHY Hold"]
    #[inline] pub fn phyhold(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Ethernet PHY Hold"]
    #[inline] pub fn test_phyhold(&self) -> bool {
        self.phyhold != 0
    }

    #[doc="Ethernet PHY Hold"]
    #[inline] pub fn set_phyhold<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Auto Negotiation Mode"]
    #[inline] pub fn anmode(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x3) as u8) } // [2:1]
    }

    #[doc="Auto Negotiation Mode"]
    #[inline] pub fn test_anmode(&self) -> bool {
        self.anmode != 0
    }

    #[doc="Auto Negotiation Mode"]
    #[inline] pub fn set_anmode<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Auto Negotiation Enable"]
    #[inline] pub fn anen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Auto Negotiation Enable"]
    #[inline] pub fn test_anen(&self) -> bool {
        self.anen != 0
    }

    #[doc="Auto Negotiation Enable"]
    #[inline] pub fn set_anen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Fast Auto Negotiation Select"]
    #[inline] pub fn fastansel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Fast Auto Negotiation Select"]
    #[inline] pub fn test_fastansel(&self) -> bool {
        self.fastansel != 0
    }

    #[doc="Fast Auto Negotiation Select"]
    #[inline] pub fn set_fastansel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Fast Auto Negotiation Enable"]
    #[inline] pub fn fastanen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Fast Auto Negotiation Enable"]
    #[inline] pub fn test_fastanen(&self) -> bool {
        self.fastanen != 0
    }

    #[doc="Fast Auto Negotiation Enable"]
    #[inline] pub fn set_fastanen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Extended Full Duplex Ability"]
    #[inline] pub fn extfd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Extended Full Duplex Ability"]
    #[inline] pub fn test_extfd(&self) -> bool {
        self.extfd != 0
    }

    #[doc="Extended Full Duplex Ability"]
    #[inline] pub fn set_extfd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="FAST Link-Up in Parallel Detect"]
    #[inline] pub fn fastlupd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="FAST Link-Up in Parallel Detect"]
    #[inline] pub fn test_fastlupd(&self) -> bool {
        self.fastlupd != 0
    }

    #[doc="FAST Link-Up in Parallel Detect"]
    #[inline] pub fn set_fastlupd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Fast RXDV Detection"]
    #[inline] pub fn fastrxdv(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Fast RXDV Detection"]
    #[inline] pub fn test_fastrxdv(&self) -> bool {
        self.fastrxdv != 0
    }

    #[doc="Fast RXDV Detection"]
    #[inline] pub fn set_fastrxdv<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="MDIX Enable"]
    #[inline] pub fn mdixen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="MDIX Enable"]
    #[inline] pub fn test_mdixen(&self) -> bool {
        self.mdixen != 0
    }

    #[doc="MDIX Enable"]
    #[inline] pub fn set_mdixen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Fast Auto MDI-X"]
    #[inline] pub fn fastmdix(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Fast Auto MDI-X"]
    #[inline] pub fn test_fastmdix(&self) -> bool {
        self.fastmdix != 0
    }

    #[doc="Fast Auto MDI-X"]
    #[inline] pub fn set_fastmdix<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Robust Auto MDI-X"]
    #[inline] pub fn rbstmdix(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Robust Auto MDI-X"]
    #[inline] pub fn test_rbstmdix(&self) -> bool {
        self.rbstmdix != 0
    }

    #[doc="Robust Auto MDI-X"]
    #[inline] pub fn set_rbstmdix<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="MDI Swap"]
    #[inline] pub fn mdiswap(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="MDI Swap"]
    #[inline] pub fn test_mdiswap(&self) -> bool {
        self.mdiswap != 0
    }

    #[doc="MDI Swap"]
    #[inline] pub fn set_mdiswap<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Polarity Swap"]
    #[inline] pub fn polswap(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Polarity Swap"]
    #[inline] pub fn test_polswap(&self) -> bool {
        self.polswap != 0
    }

    #[doc="Polarity Swap"]
    #[inline] pub fn set_polswap<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Fast Link Down Mode"]
    #[inline] pub fn fastldmode(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1f) as u8) } // [19:15]
    }

    #[doc="Fast Link Down Mode"]
    #[inline] pub fn test_fastldmode(&self) -> bool {
        self.fastldmode != 0
    }

    #[doc="Fast Link Down Mode"]
    #[inline] pub fn set_fastldmode<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="TDR Auto Run"]
    #[inline] pub fn tdrrun(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="TDR Auto Run"]
    #[inline] pub fn test_tdrrun(&self) -> bool {
        self.tdrrun != 0
    }

    #[doc="TDR Auto Run"]
    #[inline] pub fn set_tdrrun<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Link Loss Recovery"]
    #[inline] pub fn lrr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Link Loss Recovery"]
    #[inline] pub fn test_lrr(&self) -> bool {
        self.lrr != 0
    }

    #[doc="Link Loss Recovery"]
    #[inline] pub fn set_lrr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Isolate MII in Link Loss"]
    #[inline] pub fn isomiill(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Isolate MII in Link Loss"]
    #[inline] pub fn test_isomiill(&self) -> bool {
        self.isomiill != 0
    }

    #[doc="Isolate MII in Link Loss"]
    #[inline] pub fn set_isomiill<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="RXER Detection During Idle"]
    #[inline] pub fn rxeridle(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="RXER Detection During Idle"]
    #[inline] pub fn test_rxeridle(&self) -> bool {
        self.rxeridle != 0
    }

    #[doc="RXER Detection During Idle"]
    #[inline] pub fn set_rxeridle<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Odd Nibble TXER Detection Disable"]
    #[inline] pub fn nibdetdis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Odd Nibble TXER Detection Disable"]
    #[inline] pub fn test_nibdetdis(&self) -> bool {
        self.nibdetdis != 0
    }

    #[doc="Odd Nibble TXER Detection Disable"]
    #[inline] pub fn set_nibdetdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="PHY Soft Restart"]
    #[inline] pub fn digrestart(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="PHY Soft Restart"]
    #[inline] pub fn test_digrestart(&self) -> bool {
        self.digrestart != 0
    }

    #[doc="PHY Soft Restart"]
    #[inline] pub fn set_digrestart<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Ethernet Interface Select"]
    #[inline] pub fn pintfs(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x7) as u8) } // [30:28]
    }

    #[doc="Ethernet Interface Select"]
    #[inline] pub fn test_pintfs(&self) -> bool {
        self.pintfs != 0
    }

    #[doc="Ethernet Interface Select"]
    #[inline] pub fn set_pintfs<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="PHY Select"]
    #[inline] pub fn phyext(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="PHY Select"]
    #[inline] pub fn test_phyext(&self) -> bool {
        self.phyext != 0
    }

    #[doc="PHY Select"]
    #[inline] pub fn set_phyext<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
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
        if self.phyhold() != 0 { try!(write!(f, " phyhold"))}
        if self.anmode() != 0 { try!(write!(f, " anmode=0x{:x}", self.anmode()))}
        if self.anen() != 0 { try!(write!(f, " anen"))}
        if self.fastansel() != 0 { try!(write!(f, " fastansel=0x{:x}", self.fastansel()))}
        if self.fastanen() != 0 { try!(write!(f, " fastanen"))}
        if self.extfd() != 0 { try!(write!(f, " extfd"))}
        if self.fastlupd() != 0 { try!(write!(f, " fastlupd"))}
        if self.fastrxdv() != 0 { try!(write!(f, " fastrxdv"))}
        if self.mdixen() != 0 { try!(write!(f, " mdixen"))}
        if self.fastmdix() != 0 { try!(write!(f, " fastmdix"))}
        if self.rbstmdix() != 0 { try!(write!(f, " rbstmdix"))}
        if self.mdiswap() != 0 { try!(write!(f, " mdiswap"))}
        if self.polswap() != 0 { try!(write!(f, " polswap"))}
        if self.fastldmode() != 0 { try!(write!(f, " fastldmode=0x{:x}", self.fastldmode()))}
        if self.tdrrun() != 0 { try!(write!(f, " tdrrun"))}
        if self.lrr() != 0 { try!(write!(f, " lrr"))}
        if self.isomiill() != 0 { try!(write!(f, " isomiill"))}
        if self.rxeridle() != 0 { try!(write!(f, " rxeridle"))}
        if self.nibdetdis() != 0 { try!(write!(f, " nibdetdis"))}
        if self.digrestart() != 0 { try!(write!(f, " digrestart"))}
        if self.pintfs() != 0 { try!(write!(f, " pintfs=0x{:x}", self.pintfs()))}
        if self.phyext() != 0 { try!(write!(f, " phyext"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC Clock Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cc(pub u32);
impl Cc {
    #[doc="LED Polarity Control"]
    #[inline] pub fn pol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="LED Polarity Control"]
    #[inline] pub fn test_pol(&self) -> bool {
        self.pol != 0
    }

    #[doc="LED Polarity Control"]
    #[inline] pub fn set_pol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="PTP Clock Reference Enable"]
    #[inline] pub fn ptpcen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="PTP Clock Reference Enable"]
    #[inline] pub fn test_ptpcen(&self) -> bool {
        self.ptpcen != 0
    }

    #[doc="PTP Clock Reference Enable"]
    #[inline] pub fn set_ptpcen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

}

impl ::core::fmt::Display for Cc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pol() != 0 { try!(write!(f, " pol"))}
        if self.ptpcen() != 0 { try!(write!(f, " ptpcen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}


