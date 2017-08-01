pub use ::chip::lpspi::*;

pub enum Prescale {
    Div1 = 0b000,
    Div2 = 0b001,
    Div4 = 0b010,
    Div8 = 0b011,
    Div16 = 0b100,
    Div32 = 0b101,
    Div64 = 0b110,
    Div128 = 0b111,
}

#[derive(Debug)]
pub struct Config {
    cfgr0: Cfgr0,
    cfgr1: Cfgr1,
    ccr: Ccr,
    fcr: Fcr,
}

impl Default for Config {
    fn default() -> Config {
        Config { 
            cfgr0: Cfgr0(0),
            cfgr1: Cfgr1(0),
            ccr: Ccr(0),
            fcr: Fcr(0),
        }
    }
}

impl Config {
    // CFGR0

    pub fn rdmo(mut self, value: bool) -> Self {
        let value = if value { 1 } else { 0 };
        self.cfgr0 = self.cfgr0.set_rdmo(value);
        self
    }

    pub fn cirfifo(mut self, value: bool) -> Self {
        let value = if value { 1 } else { 0 };
        self.cfgr0 = self.cfgr0.set_cirfifo(value);
        self
    }

    pub fn hrsel(mut self, value: bool) -> Self {
        let value = if value { 1 } else { 0 };
        self.cfgr0 = self.cfgr0.set_hrsel(value);
        self
    }

    pub fn hrpol(mut self, value: bool) -> Self {
        let value = if value { 1 } else { 0 };
        self.cfgr0 = self.cfgr0.set_hrpol(value);
        self
    }

    pub fn hren(mut self, value: bool) -> Self {
        let value = if value { 1 } else { 0 };
        self.cfgr0 = self.cfgr0.set_hren(value);
        self
    }

    // CFGR1

    pub fn pcscfg(mut self, value: bool) -> Self {
        let value = if value { 1 } else { 0 };
        self.cfgr1 = self.cfgr1.set_pcscfg(value);
        self
    }

    pub fn outcfg(mut self, value: bool) -> Self {
        let value = if value { 1 } else { 0 };
        self.cfgr1 = self.cfgr1.set_outcfg(value);
        self
    }

    pub fn pincfg(mut self, value: bool) -> Self {
        let value = if value { 1 } else { 0 };
        self.cfgr1 = self.cfgr1.set_pincfg(value);
        self
    }

    pub fn matcfg(mut self, value: bool) -> Self {
        let value = if value { 1 } else { 0 };
        self.cfgr1 = self.cfgr1.set_matcfg(value);
        self
    }

    pub fn pcspol(mut self, value: bool) -> Self {
        let value = if value { 1 } else { 0 };
        self.cfgr1 = self.cfgr1.set_pcspol(value);
        self
    }

    pub fn nostall(mut self, value: bool) -> Self {
        let value = if value { 1 } else { 0 };
        self.cfgr1 = self.cfgr1.set_nostall(value);
        self
    }

    pub fn autopcs(mut self, value: bool) -> Self {
        let value = if value { 1 } else { 0 };
        self.cfgr1 = self.cfgr1.set_autopcs(value);
        self
    }    

    pub fn sample(mut self, value: bool) -> Self {
        let value = if value { 1 } else { 0 };
        self.cfgr1 = self.cfgr1.set_sample(value);
        self
    }

    pub fn master(mut self, value: bool) -> Self {
        let value = if value { 1 } else { 0 };
        self.cfgr1 = self.cfgr1.set_master(value);
        self
    }       

    // CCR

    pub fn sckpcs(mut self, value: u8) -> Self {
        self.ccr = self.ccr.set_sckpcs(value as u32);
        self
    }       

    pub fn pcssck(mut self, value: u8) -> Self {
        self.ccr = self.ccr.set_pcssck(value as u32);
        self
    }    

    pub fn dbt(mut self, value: u8) -> Self {
        self.ccr = self.ccr.set_dbt(value as u32);
        self
    }    

    pub fn sckdiv(mut self, value: u8) -> Self {
        self.ccr = self.ccr.set_sckdiv(value as u32);
        self
    }       

    // FCR

    pub fn rxwater(mut self, value: u8) -> Self {
        self.fcr = self.fcr.set_rxwater(value as u32);
        self
    }

    pub fn txwater(mut self, value: u8) -> Self {
        self.fcr = self.fcr.set_txwater(value as u32);
        self
    }
}

pub struct Target {
    lpspi: Lpspi,
    tcr: Tcr
}

impl Target {
    pub fn cpol(mut self, value: bool) -> Self {
        let value = if value { 1 } else { 0 };
        self.tcr = self.tcr.set_cpol(value);
        self
    }

    pub fn cpha(mut self, value: bool) -> Self {
        let value = if value { 1 } else { 0 };
        self.tcr = self.tcr.set_cpha(value);
        self
    }

    pub fn prescale(mut self, value: u8) -> Self {
        self.tcr = self.tcr.set_prescale(value as u32);
        self
    }

    pub fn pcs(mut self, value: u8) -> Self {
        self.tcr = self.tcr.set_pcs(value as u32);
        self
    }

    pub fn lsbf(mut self, value: bool) -> Self {
        let value = if value { 1 } else { 0 };
        self.tcr = self.tcr.set_lsbf(value);
        self
    }

    pub fn bysw(mut self, value: bool) -> Self {
        let value = if value { 1 } else { 0 };
        self.tcr = self.tcr.set_bysw(value);
        self
    }

    pub fn cont(mut self, value: bool) -> Self {
        let value = if value { 1 } else { 0 };
        self.tcr = self.tcr.set_cont(value);
        self
    }

    pub fn contc(mut self, value: bool) -> Self {
        let value = if value { 1 } else { 0 };
        self.tcr = self.tcr.set_contc(value);
        self
    }

    pub fn rxmsk(mut self, value: bool) -> Self {
        let value = if value { 1 } else { 0 };
        self.tcr = self.tcr.set_rxmsk(value);
        self
    }

    pub fn txmsk(mut self, value: bool) -> Self {
        let value = if value { 1 } else { 0 };
        self.tcr = self.tcr.set_txmsk(value);
        self
    }

    pub fn width(mut self, value: u8) -> Self {
        self.tcr = self.tcr.set_width(value as u32);
        self
    }

    pub fn framesz(mut self, value: u8) -> Self {
        self.tcr = self.tcr.set_framesz(value as u32);
        self
    }

    pub fn configure(&self) {
        let mut lpspi = self.lpspi;
        unsafe {
            lpspi.set_tcr(Tcr(self.tcr.0))
        }
    }        

    pub fn send(&self, data: u16) {
        self.configure();
        let mut lpspi = self.lpspi;
        unsafe {
            while lpspi.sr().tdf() == 0 {}
            lpspi.set_tdr(Tdr(0).set_data(data as u32));            
        }
    }

    pub fn recv(&self) -> u16 {
        self.configure();
        let lpspi = self.lpspi;
        unsafe {
            while lpspi.sr().rdf() == 0 {}
            lpspi.rdr().data() as u16
        }        
    }

}

pub fn device(lpspi: Lpspi) -> LpspiDevice {
    LpspiDevice { lpspi: lpspi }
}

pub struct LpspiDevice {
    pub lpspi: Lpspi
}

impl LpspiDevice {
    pub fn configure(&self, cfg: Config) {
        let mut lpspi = self.lpspi;
        unsafe {
            lpspi.set_cfgr0(Cfgr0(cfg.cfgr0.0));
            lpspi.set_cfgr1(Cfgr1(cfg.cfgr1.0));
            lpspi.set_ccr(Ccr(cfg.ccr.0));
            lpspi.set_fcr(Fcr(cfg.fcr.0));
        }        
    }

    pub fn target(&self) -> Target {
        Target {
            lpspi: self.lpspi,
            tcr: Tcr(0),
        }
    }

    // CR

    pub fn reset_receive_fifo(&self) {
        let mut lpspi = self.lpspi;
        unsafe {
            lpspi.with_cr(|r| r.set_rrf(1));
        }        
    }

    pub fn reset_transmit_fifo(&self) {
        let mut lpspi = self.lpspi;
        unsafe {
            lpspi.with_cr(|r| r.set_rtf(1));
        }        
    }    

    pub fn set_reset(&self, value: bool) {
        let value = if value { 1 } else { 0 };
        let mut lpspi = self.lpspi;
        unsafe {
            lpspi.with_cr(|r| r.set_rst(value));
        }
    }

    pub fn set_enabled(&self, value: bool) {
        let value = if value { 1 } else { 0 };
        let mut lpspi = self.lpspi;
        unsafe {
            lpspi.with_cr(|r| r.set_men(value));
        }
    }

    // SR

    pub fn mbf(&self) -> bool {
        let lpspi = self.lpspi;
        unsafe {
            lpspi.sr().mbf() != 0
        }
    }

    // DMF

    pub fn dmf(&self) -> bool {
        let lpspi = self.lpspi;
        unsafe {
            lpspi.sr().dmf() != 0
        }
    }

    pub fn clr_dmf(&self) {
        let mut lpspi = self.lpspi;
        unsafe {
            lpspi.set_sr(Sr(0).set_dmf(1));
        }
    }

    // _REF

    pub fn _ref(&self) -> bool {
        let lpspi = self.lpspi;
        unsafe {
            lpspi.sr()._ref() != 0
        }
    }

    pub fn clr_ref(&self) {
        let mut lpspi = self.lpspi;
        unsafe {
            lpspi.set_sr(Sr(0).set_ref(1));
        }
    }

    // TEF

    pub fn tef(&self) -> bool {
        let lpspi = self.lpspi;
        unsafe {
            lpspi.sr().tef() != 0
        }
    }

    pub fn clr_tef(&self) {
        let mut lpspi = self.lpspi;
        unsafe {
            lpspi.set_sr(Sr(0).set_tef(1));
        }
    }

    // TCF

    pub fn tcf(&self) -> bool {
        let lpspi = self.lpspi;
        unsafe {
            lpspi.sr().tcf() != 0
        }
    }

    pub fn clr_tcf(&self) {
        let mut lpspi = self.lpspi;
        unsafe {
            lpspi.set_sr(Sr(0).set_tcf(1));
        }
    }

    // FCF

    pub fn fcf(&self) -> bool {
        let lpspi = self.lpspi;
        unsafe {
            lpspi.sr().fcf() != 0
        }
    }

    pub fn clr_fcf(&self) {
        let mut lpspi = self.lpspi;
        unsafe {
            lpspi.set_sr(Sr(0).set_fcf(1));
        }
    }

    // WCF 

    pub fn wcf(&self) -> bool {
        let lpspi = self.lpspi;
        unsafe {
            lpspi.sr().wcf() != 0
        }
    }

    pub fn clr_wcf(&self) {
        let mut lpspi = self.lpspi;
        unsafe {
            lpspi.set_sr(Sr(0).set_wcf(1));
        }
    }                

    // RDF 

    pub fn rdf(&self) -> bool {
        let lpspi = self.lpspi;
        unsafe {
            lpspi.sr().rdf() != 0
        }
    }

    // TDF

    pub fn tdf(&self) -> bool {
        let lpspi = self.lpspi;
        unsafe {
            lpspi.sr().tdf() != 0
        }
    }
    
    // IER

    // DMIE

    pub fn dmie(&self) -> bool {
        let lpspi = self.lpspi;
        unsafe {
            lpspi.ier().dmie() != 0
        }
    }    

    pub fn clr_dmie(&self) {
        let mut lpspi = self.lpspi;
        unsafe {
            lpspi.set_ier(Ier(0).set_dmie(1));
        }
    }    

    // REIE

    pub fn reie(&self) -> bool {
        let lpspi = self.lpspi;
        unsafe {
            lpspi.ier().reie() != 0
        }
    }    

    pub fn clr_reie(&self) {
        let mut lpspi = self.lpspi;
        unsafe {
            lpspi.set_ier(Ier(0).set_reie(1));
        }
    }    

    // TEIE

    pub fn teie(&self) -> bool {
        let lpspi = self.lpspi;
        unsafe {
            lpspi.ier().teie() != 0
        }
    }    

    pub fn clr_teie(&self) {
        let mut lpspi = self.lpspi;
        unsafe {
            lpspi.set_ier(Ier(0).set_teie(1));
        }
    }    

    // TCIE

    pub fn tcie(&self) -> bool {
        let lpspi = self.lpspi;
        unsafe {
            lpspi.ier().tcie() != 0
        }
    }    

    pub fn clr_tcie(&self) {
        let mut lpspi = self.lpspi;
        unsafe {
            lpspi.set_ier(Ier(0).set_tcie(1));
        }
    }    

    // FCIE

    pub fn fcie(&self) -> bool {
        let lpspi = self.lpspi;
        unsafe {
            lpspi.ier().fcie() != 0
        }
    }    

    pub fn clr_fcie(&self) {
        let mut lpspi = self.lpspi;
        unsafe {
            lpspi.set_ier(Ier(0).set_fcie(1));
        }
    }    

    // WCIE

    pub fn wcie(&self) -> bool {
        let lpspi = self.lpspi;
        unsafe {
            lpspi.ier().wcie() != 0
        }
    }    

    pub fn clr_wcie(&self) {
        let mut lpspi = self.lpspi;
        unsafe {
            lpspi.set_ier(Ier(0).set_wcie(1));
        }
    }    

    // RDIE

    pub fn rdie(&self) -> bool {
        let lpspi = self.lpspi;
        unsafe {
            lpspi.ier().rdie() != 0
        }
    }    

    pub fn clr_rdie(&self) {
        let mut lpspi = self.lpspi;
        unsafe {
            lpspi.set_ier(Ier(0).set_rdie(1));
        }
    }    

    // TDIE

    pub fn tdie(&self) -> bool {
        let lpspi = self.lpspi;
        unsafe {
            lpspi.ier().tdie() != 0
        }
    }    

    pub fn clr_tdie(&self) {
        let mut lpspi = self.lpspi;
        unsafe {
            lpspi.set_ier(Ier(0).set_tdie(1));
        }
    }

}
