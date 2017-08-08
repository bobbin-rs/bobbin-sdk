pub use chip::edma::*;

pub trait EdmaChExt {
    fn attr(&self) -> TcdAttr;
    fn set_attr(&self, TcdAttr) -> &Self;
    fn with_attr<F: FnOnce(TcdAttr) -> TcdAttr>(&self, F) -> &Self;

    fn csr(&self) -> TcdCsr;
    fn set_csr(&self, TcdCsr) -> &Self;
    fn with_csr<F: FnOnce(TcdCsr) -> TcdCsr>(&self, F) -> &Self;

    fn saddr(&self) -> u32;
    fn set_saddr(&self, u32) -> &Self;
    
    fn soff(&self) -> i16;
    fn set_soff(&self, i16) -> &Self;

    fn slast(&self) -> i32;
    fn set_slast(&self, i32) -> &Self;    

    fn daddr(&self) -> u32;
    fn set_daddr(&self, u32) -> &Self;

    fn doff(&self) -> i16;
    fn set_doff(&self, i16) -> &Self;

    fn nbytes(&self) -> usize;
    fn set_nbytes(&self, usize) -> &Self;

    fn citer(&self) -> u16;
    fn set_citer(&self, u16) -> &Self;

    fn dlastsga(&self) -> u32;
    fn set_dlastsga(&self, u32) -> &Self;

    fn biter(&self) -> u16;
    fn set_biter(&self, u16) -> &Self;
    
    fn is_enabled(&self) -> bool;
    fn set_enabled(&self) -> &Self;
    fn clr_enabled(&self) -> &Self;

    fn is_done(&self) -> bool;
    fn clr_done(&self) -> &Self;

    fn is_active(&self) -> bool;

    fn set_start(&self) -> &Self;

    fn is_err(&self) -> bool;
    fn clr_err(&self) -> &Self;

    fn is_int(&self) -> bool;
    fn clr_int(&self) -> &Self;

    fn is_hrs(&self) -> bool;
}

impl<P, T> EdmaChExt for Channel<P, T> {
    fn attr(&self) -> TcdAttr {
        self.periph.tcd_attr(self.index)
    }
    fn set_attr(&self, value: TcdAttr) -> &Self {
        self.periph.set_tcd_attr(self.index, |_| value);
        self
    }
    fn with_attr<F: FnOnce(TcdAttr) -> TcdAttr>(&self, f: F) -> &Self {
        self.periph.with_tcd_attr(self.index, f);
        self
    }

    fn csr(&self) -> TcdCsr {
        self.periph.tcd_csr(self.index)
    }
    fn set_csr(&self, value: TcdCsr) -> &Self {
        self.periph.set_tcd_csr(self.index, |_| value);
        self
    }
    fn with_csr<F: FnOnce(TcdCsr) -> TcdCsr>(&self, f: F) -> &Self {
        self.periph.with_tcd_csr(self.index, f);
        self
    }

    fn saddr(&self) -> u32 {
        self.periph.tcd_saddr(self.index).saddr().into()
    }

    fn set_saddr(&self, value: u32) -> &Self {
        self.periph.set_tcd_saddr(self.index, |r| r.set_saddr(value));
        self
    }

    fn soff(&self) -> i16 {
        let soff: u16 = self.periph.tcd_soff(self.index).soff().into();
        soff as i16
    }

    fn set_soff(&self, value: i16) -> &Self {
        self.periph.set_tcd_soff(self.index, |r| r.set_soff(value as u16));
        self
    }

    fn slast(&self) -> i32 {
        let slast: u32 = self.periph.tcd_slast(self.index).slast().into();
        slast as i32
    }

    fn set_slast(&self, value: i32) -> &Self {
        self.periph.set_tcd_slast(self.index, |r| r.set_slast(value as u32));
        self
    }    

    fn daddr(&self) -> u32 {
        self.periph.tcd_daddr(self.index).daddr().into()
    }

    fn set_daddr(&self, value: u32) -> &Self {
        self.periph.set_tcd_daddr(self.index, |r| r.set_daddr(value));
        self
    }

    fn doff(&self) -> i16 {
        let doff: u16 = self.periph.tcd_doff(self.index).doff().into();
        doff as i16
    }

    fn set_doff(&self, value: i16) -> &Self {
        self.periph.set_tcd_doff(self.index, |r| r.set_doff(value as u16));
        self
    }

    fn nbytes(&self) -> usize {
        self.periph.tcd_nbytes_mlno(self.index).nbytes().into()
    }

    fn set_nbytes(&self, value: usize) -> &Self {
        self.periph.set_tcd_nbytes_mlno(self.index, |r| r.set_nbytes(value as u32));
        self
    }

    fn citer(&self) -> u16 {
        self.periph.tcd_citer_elinkno(self.index).citer().into()
    }
    fn set_citer(&self, value: u16) -> &Self {
        self.periph.set_tcd_citer_elinkno(self.index, |r| r.set_citer(value));
        self
    }


    fn dlastsga(&self) -> u32 {
        self.periph.tcd_dlastsga(self.index).dlastsga().into()
    }
    fn set_dlastsga(&self, value: u32) -> &Self {
        self.periph.set_tcd_dlastsga(self.index, |r| r.set_dlastsga(value));
        self
    }


    fn biter(&self) -> u16 {
        self.periph.tcd_biter_elinkno(self.index).biter().into()
    }
    fn set_biter(&self, value: u16) -> &Self {
        self.periph.set_tcd_biter_elinkno(self.index, |r| r.set_biter(value));
        self
    }

    fn is_enabled(&self) -> bool {
        self.periph.erq().erq(self.index) != 0
    }
    fn set_enabled(&self) -> &Self {
        self.periph.set_serq(|r| r.set_serq(self.index as u8));
        self
    }
    fn clr_enabled(&self) -> &Self {
        self.periph.set_cerq(|r| r.set_cerq(self.index as u8));
        self
    }

    fn is_done(&self) -> bool {
        self.csr().done() != 0
    }

    fn clr_done(&self) -> &Self {
        self.periph.set_cdne(|r| r.set_cdne(self.index as u8));
        self        
    }

    fn is_active(&self) -> bool {
        self.csr().active() != 0
    }
    fn set_start(&self) -> &Self {
        self.periph.set_ssrt(|r| r.set_ssrt(self.index as u8));
        self                
    }

    fn is_err(&self) -> bool {
        self.periph.err().err(self.index) != 0
    }
    fn clr_err(&self) -> &Self {
        self.periph.set_cerr(|r| r.set_cerr(self.index as u8));
        self                
    }

    fn is_int(&self) -> bool {
        self.periph.int().int(self.index) != 0
    }
    fn clr_int(&self) -> &Self {
        self.periph.set_cint(|r| r.set_cint(self.index as u8));
        self                        
    }

    fn is_hrs(&self) -> bool {
        self.periph.hrs().hrs(self.index) != 0
    }

}