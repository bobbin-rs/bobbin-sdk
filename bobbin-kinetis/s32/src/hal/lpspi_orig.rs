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

    fn rdmo(mut self, value: bool) -> Self {
        self.cfgr0 = self.cfgr0.set_rdmo(value);
        self
    }

    fn cirfifo(mut self, value: bool) -> Self {
        self.cfgr0 = self.cfgr0.set_cirfifo(value);
        self
    }

    fn hrsel(mut self, value: bool) -> Self {
        self.cfgr0 = self.cfgr0.set_hrsel(value);
        self
    }

    fn hrpol(mut self, value: bool) -> Self {
        self.cfgr0 = self.cfgr0.set_hrpol(value);
        self
    }

    fn hren(mut self, value: bool) -> Self {
        self.cfgr0 = self.cfgr0.set_hren(value);
        self
    }

    // CFGR1

    fn pcscfg(mut self, value: bool) -> Self {
        self.cfgr1 = self.cfgr1.set_pcscfg(value);
        self
    }

    fn outcfg(mut self, value: bool) -> Self {
        self.cfgr1 = self.cfgr1.set_outcfg(value);
        self
    }

    fn pincfg(mut self, value: bool) -> Self {
        self.cfgr1 = self.cfgr1.set_pincfg(value);
        self
    }

    fn matcfg(mut self, value: bool) -> Self {
        self.cfgr1 = self.cfgr1.set_matcfg(value);
        self
    }

    fn pcspol(mut self, value: bool) -> Self {
        self.cfgr1 = self.cfgr1.set_pcspol(value);
        self
    }

    fn nostall(mut self, value: bool) -> Self {
        self.cfgr1 = self.cfgr1.set_nostall(value);
        self
    }

    fn autopcs(mut self, value: bool) -> Self {
        self.cfgr1 = self.cfgr1.set_autopcs(value);
        self
    }    

    fn sample(mut self, value: bool) -> Self {
        self.cfgr1 = self.cfgr1.set_sample(value);
        self
    }

    fn master(mut self, value: bool) -> Self {
        self.cfgr1 = self.cfgr1.set_master(value);
        self
    }       

    // CCR

    fn sckpcs(mut self, value: u8) -> Self {
        self.ccr = self.ccr.set_sckpcs(value as u32);
        self
    }       

    fn pcssck(mut self, value: u8) -> Self {
        self.ccr = self.ccr.set_pcssck(value as u32);
        self
    }    

    fn dbt(mut self, value: u8) -> Self {
        self.ccr = self.ccr.set_dbt(value as u32);
        self
    }    

    fn sckdiv(mut self, value: u8) -> Self {
        self.ccr = self.ccr.set_sckdiv(value as u32);
        self
    }       

    // FCR

    fn rxwater(mut self, value: u8) -> Self {
        self.fcr = self.fcr.set_rxwater(value as u32);
        self
    }

    fn txwater(mut self, value: u8) -> Self {
        self.fcr = self.fcr.set_txwater(value as u32);
        self
    }
}

pub struct Target {
    lpspi: Lpspi,
    tcr: Tcr
}

impl Target {
    fn cpol(mut self, value: bool) -> Self {
        self.tcr = self.tcr.set_cpol(value);
        self
    }

    fn cpha(mut self, value: bool) -> Self {
        self.tcr = self.tcr.set_cpha(value);
        self
    }

    fn prescale(mut self, value: u8) -> Self {
        self.tcr = self.tcr.set_prescale(value);
        self
    }

    fn pcs(mut self, value: u8) -> Self {
        self.tcr = self.tcr.set_pcs(value);
        self
    }

    fn lsbf(mut self, value: bool) -> Self {
        self.tcr = self.tcr.set_lsbf(value);
        self
    }

    fn bysw(mut self, value: bool) -> Self {
        self.tcr = self.tcr.set_bysw(value);
        self
    }

    fn cont(mut self, value: bool) -> Self {
        self.tcr = self.tcr.set_cont(value);
        self
    }

    fn contc(mut self, value: bool) -> Self {
        self.tcr = self.tcr.set_contc(value);
        self
    }

    fn rxmsk(mut self, value: bool) -> Self {
        self.tcr = self.tcr.set_rxmsk(value);
        self
    }

    fn txmsk(mut self, value: bool) -> Self {
        self.tcr = self.tcr.set_txmsk(value);
        self
    }

    fn width(mut self, value: u8) -> Self {
        self.tcr = self.tcr.set_width(value);
        self
    }

    fn framesz(mut self, value: u8) -> Self {
        self.tcr = self.tcr.set_framesz(value);
        self
    }

    fn configure(&self) {

        self.set_tcr(|_| self.tcr)
    }        

    fn send(&self, data: u16) {
        self.configure();

        while self.sr().tdf() == 0 {}
        self.set_tdr(|r| r.set_data(data as u32));            
    }

    fn recv(&self) -> u16 {
        self.configure();
        let lpspi = self.lpspi;
        while self.sr().rdf() == 0 {}
        self.rdr().data() as u16
    }

}

fn device(lpspi: Lpspi) -> LpspiDevice {
    LpspiDevice { lpspi: lpspi }
}

pub struct LpspiDevice {
    pub lpspi: Lpspi
}

pub trait LpspiExt {
    fn configure(&self, cfg: Config) -> &Self;

    // CR

    fn reset_receive_fifo(&self) -> &Self;
    fn reset_transmit_fifo(&self) -> &Self;
    fn set_reset(&self, value: bool) -> &Self;
    fn set_enabled(&self, value: bool) -> &Self;

    // SR

    fn mbf(&self) -> bool;

    // DMF

    fn dmf(&self) -> bool;
    fn clr_dmf(&self) -> &Self;

    // _REF

    fn _ref(&self) -> bool;
    fn clr_ref(&self) -> &Self;

    // TEF

    fn tef(&self) -> bool;
    fn clr_tef(&self) -> &Self;

    // TCF

    fn tcf(&self) -> bool;
    fn clr_tcf(&self) -> &Self;

    // FCF

    fn fcf(&self) -> bool;
    fn clr_fcf(&self) -> &Self;

    // WCF 

    fn wcf(&self) -> bool;
    fn clr_wcf(&self) -> &Self;

    // RDF 

    fn rdf(&self) -> bool;

    // TDF

    fn tdf(&self) -> bool;
    
    // IER

    // DMIE

    fn dmie(&self) -> bool;
    fn clr_dmie(&self) -> &Self;

    // REIE

    fn reie(&self) -> bool;
    fn clr_reie(&self) -> &Self;

    // TEIE

    fn teie(&self) -> bool;
    fn clr_teie(&self) -> &Self;

    // TCIE

    fn tcie(&self) -> bool;
    fn clr_tcie(&self) -> &Self;

    // FCIE

    fn fcie(&self) -> bool;
    fn clr_fcie(&self) -> &Self;

    // WCIE

    fn wcie(&self) -> bool; -> &Self;    
    fn clr_wcie(&self) -> &Self;

    // RDIE

    fn rdie(&self) -> bool;
    fn clr_rdie(&self) -> &Self;

    // TDIE

    fn tdie(&self) -> bool;
    fn clr_tdie(&self) -> &Self;
}

impl<T> LpspiExt for Periph<T> {
    fn configure(&self, cfg: Config) {
        self.set_cfgr0(cfg.cfgr0);
        self.set_cfgr1(cfg.cfgr1);
        self.set_ccr(cfg.ccr);
        self.set_fcr(cfg.fcr);
    }

    // CR

    fn reset_receive_fifo(&self) -> &Self {
        self.with_cr(|r| r.set_rrf(1));
    }

    fn reset_transmit_fifo(&self) -> &Self {
        self.with_cr(|r| r.set_rtf(1));
    }    

    fn set_reset(&self, value: bool) -> &Self {
        self.with_cr(|r| r.set_rst(value));
    }

    fn set_enabled(&self, value: bool) -> &Self {
        self.with_cr(|r| r.set_men(value));
    }

    // SR

    fn mbf(&self) -> bool {
        self.sr().mbf() != 0
    }

    // DMF

    fn dmf(&self) -> bool {
        self.sr().dmf() != 0
    }

    fn clr_dmf(&self) -> &Self {
        self.set_sr(|r| r.set_dmf(1));
    }

    // _REF

    fn _ref(&self) -> bool {
        self.sr()._ref() != 0
    }

    fn clr_ref(&self) -> &Self {
        self.set_sr(|r| r.set_ref(1));
    }

    // TEF

    fn tef(&self) -> bool {
        self.sr().tef() != 0
    }

    fn clr_tef(&self) -> &Self {
        self.set_sr(|r| r.set_tef(1));
    }

    // TCF

    fn tcf(&self) -> bool {
        self.sr().tcf() != 0
    }

    fn clr_tcf(&self) -> &Self {
        self.set_sr(|r| r.set_tcf(1));
    }

    // FCF

    fn fcf(&self) -> bool {
        self.sr().fcf() != 0
    }

    fn clr_fcf(&self) -> &Self {
        self.set_sr(|r| r.set_fcf(1));
    }

    // WCF 

    fn wcf(&self) -> bool {
        self.sr().wcf() != 0
    }

    fn clr_wcf(&self) -> &Self {
        self.set_sr(|r| r.set_wcf(1));
    }                

    // RDF 

    fn rdf(&self) -> bool {
        self.sr().rdf() != 0
    }

    // TDF

    fn tdf(&self) -> bool {
        self.sr().tdf() != 0
    }
    
    // IER

    // DMIE

    fn dmie(&self) -> bool {
        self.ier().dmie() != 0
    }    

    fn clr_dmie(&self) -> &Self {
        self.set_ier(|r| r.set_dmie(1));
    }    

    // REIE

    fn reie(&self) -> bool {
        self.ier().reie() != 0
    }    

    fn clr_reie(&self) -> &Self {
        self.set_ier(|r| r.set_reie(1));
    }    

    // TEIE

    fn teie(&self) -> bool {
        self.ier().teie() != 0
    }    

    fn clr_teie(&self) -> &Self {
        self.set_ier(|r| r.set_teie(1));
    }    

    // TCIE

    fn tcie(&self) -> bool {
        self.ier().tcie() != 0
    }    

    fn clr_tcie(&self) -> &Self {
        self.set_ier(|r| r.set_tcie(1));
    }    

    // FCIE

    fn fcie(&self) -> bool {
        self.ier().fcie() != 0
    }    

    fn clr_fcie(&self) -> &Self {
        self.set_ier(|r| r.set_fcie(1));
    }    

    // WCIE

    fn wcie(&self) -> bool {
        self.ier().wcie() != 0
    }    

    fn clr_wcie(&self) -> &Self {
        self.set_ier(|r| r.set_wcie(1));
    }    

    // RDIE

    fn rdie(&self) -> bool {
        self.ier().rdie() != 0
    }    

    fn clr_rdie(&self) -> &Self {
        self.set_ier(|r| r.set_rdie(1));
    }    

    // TDIE

    fn tdie(&self) -> bool {
        self.ier().tdie() != 0
    }    

    fn clr_tdie(&self) -> &Self {
        self.set_ier(|r| r.set_tdie(1));
    }
}
