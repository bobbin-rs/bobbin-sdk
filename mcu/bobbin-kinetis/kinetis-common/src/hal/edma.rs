pub use chip::edma::*;
pub use bobbin_common::Channel;

impl EdmaCh {
    pub fn attr(&self) -> TcdAttr {
        self.periph.tcd_attr(self.index)
    }
    pub fn set_attr(&self, value: TcdAttr) -> &Self {
        self.periph.set_tcd_attr(self.index, |_| value);
        self
    }
    pub fn with_attr<F: FnOnce(TcdAttr) -> TcdAttr>(&self, f: F) -> &Self {
        self.periph.with_tcd_attr(self.index, f);
        self
    }

    pub fn csr(&self) -> TcdCsr {
        self.periph.tcd_csr(self.index)
    }
    pub fn set_csr(&self, value: TcdCsr) -> &Self {
        self.periph.set_tcd_csr(self.index, |_| value);
        self
    }
    pub fn with_csr<F: FnOnce(TcdCsr) -> TcdCsr>(&self, f: F) -> &Self {
        self.periph.with_tcd_csr(self.index, f);
        self
    }

    pub fn saddr(&self) -> u32 {
        self.periph.tcd_saddr(self.index).saddr().into()
    }

    pub fn set_saddr(&self, value: u32) -> &Self {
        self.periph.set_tcd_saddr(self.index, |r| r.set_saddr(value));
        self
    }

    pub fn soff(&self) -> i16 {
        let soff: u16 = self.periph.tcd_soff(self.index).soff().into();
        soff as i16
    }

    pub fn set_soff(&self, value: i16) -> &Self {
        self.periph.set_tcd_soff(self.index, |r| r.set_soff(value as u16));
        self
    }

    pub fn slast(&self) -> i32 {
        let slast: u32 = self.periph.tcd_slast(self.index).slast().into();
        slast as i32
    }

    pub fn set_slast(&self, value: i32) -> &Self {
        self.periph.set_tcd_slast(self.index, |r| r.set_slast(value as u32));
        self
    }    

    pub fn daddr(&self) -> u32 {
        self.periph.tcd_daddr(self.index).daddr().into()
    }

    pub fn set_daddr(&self, value: u32) -> &Self {
        self.periph.set_tcd_daddr(self.index, |r| r.set_daddr(value));
        self
    }

    pub fn doff(&self) -> i16 {
        let doff: u16 = self.periph.tcd_doff(self.index).doff().into();
        doff as i16
    }

    pub fn set_doff(&self, value: i16) -> &Self {
        self.periph.set_tcd_doff(self.index, |r| r.set_doff(value as u16));
        self
    }

    pub fn nbytes(&self) -> usize {
        self.periph.tcd_nbytes_mlno(self.index).nbytes().into()
    }

    pub fn set_nbytes(&self, value: usize) -> &Self {
        self.periph.set_tcd_nbytes_mlno(self.index, |r| r.set_nbytes(value as u32));
        self
    }

    pub fn citer(&self) -> u16 {
        self.periph.tcd_citer_elinkno(self.index).citer().into()
    }
    pub fn set_citer(&self, value: u16) -> &Self {
        self.periph.set_tcd_citer_elinkno(self.index, |r| r.set_citer(value));
        self
    }


    pub fn dlastsga(&self) -> u32 {
        self.periph.tcd_dlastsga(self.index).dlastsga().into()
    }
    pub fn set_dlastsga(&self, value: u32) -> &Self {
        self.periph.set_tcd_dlastsga(self.index, |r| r.set_dlastsga(value));
        self
    }


    pub fn biter(&self) -> u16 {
        self.periph.tcd_biter_elinkno(self.index).biter().into()
    }
    pub fn set_biter(&self, value: u16) -> &Self {
        self.periph.set_tcd_biter_elinkno(self.index, |r| r.set_biter(value));
        self
    }

    pub fn is_enabled(&self) -> bool {
        self.periph.erq().erq(self.index) != 0
    }
    pub fn set_enabled(&self) -> &Self {
        self.periph.set_serq(|r| r.set_serq(self.index as u8));
        self
    }
    pub fn clr_enabled(&self) -> &Self {
        self.periph.set_cerq(|r| r.set_cerq(self.index as u8));
        self
    }

    pub fn is_done(&self) -> bool {
        self.csr().done() != 0
    }

    pub fn clr_done(&self) -> &Self {
        self.periph.set_cdne(|r| r.set_cdne(self.index as u8));
        self        
    }

    pub fn is_active(&self) -> bool {
        self.csr().active() != 0
    }
    pub fn set_start(&self) -> &Self {
        self.periph.set_ssrt(|r| r.set_ssrt(self.index as u8));
        self                
    }

    pub fn is_err(&self) -> bool {
        self.periph.err().err(self.index) != 0
    }
    pub fn clr_err(&self) -> &Self {
        self.periph.set_cerr(|r| r.set_cerr(self.index as u8));
        self                
    }

    pub fn is_int(&self) -> bool {
        self.periph.int().int(self.index) != 0
    }
    pub fn clr_int(&self) -> &Self {
        self.periph.set_cint(|r| r.set_cint(self.index as u8));
        self                        
    }

    pub fn is_hrs(&self) -> bool {
        self.periph.hrs().hrs(self.index) != 0
    }

}