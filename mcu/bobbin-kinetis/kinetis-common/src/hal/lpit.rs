pub use bobbin_common::timer::*;
pub use chip::lpit::*;
pub use core::ops::Deref;

impl LpitPeriph {
    pub fn enabled(&self) -> bool {
        self.mcr().m_cen() != 0
    }
    pub fn set_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.with_mcr(|r| r.set_m_cen(value))
    }

    pub fn doze_enabled(&self) -> bool {
        self.mcr().doze_en() != 0
    }
    pub fn set_doze_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.with_mcr(|r| r.set_doze_en(value))
    }    

    pub fn ch_enabled(&self, index: usize) -> bool {
        self.tctrl(index).t_en() != 0
    }

    pub fn set_ch_enabled(&self, index: usize, value: bool) -> &Self {
        if value {
            self.set_setten(|r| r.set_set_t_en(index, 1))
        } else {
            self.set_clrten(|r| r.set_clr_t_en(index, 1))
        }
    }

    pub fn ch_value(&self, index: usize) -> u32 {
        self.tval(index).tmr_val().into()
    }
    pub fn set_ch_value(&self, index: usize, value: u32) -> &Self {
        self.set_tval(index, |r| r.set_tmr_val(value))
    }    

    pub fn ch_tie(&self, index: usize) -> bool {
        self.mier().tie(index) != 0
    }
    pub fn set_ch_tie(&self, index: usize, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.with_mier(|r| r.set_tie(index, value))
    }    

    pub fn ch_tif(&self, index: usize) -> bool {
        self.msr().tif(index) != 0
    }
    pub fn clr_ch_tif(&self, index: usize) -> &Self {
        self.set_msr(|r| r.set_tif(index, 1))
    }       
}

impl LpitCh {
    pub fn value(&self) -> u32 {
        self.periph.ch_value(self.index)
    }
    pub fn set_value(&self, value: u32) -> &Self {
        self.periph.set_ch_value(self.index, value);
        self
    }

    pub fn tie(&self) -> bool {
        self.periph.ch_tie(self.index)
    }
    pub fn set_tie(&self, value: bool) -> &Self {
        self.periph.set_ch_tie(self.index, value);
        self
    }

    pub fn tif(&self) -> bool {
        self.periph.ch_tif(self.index)
    }
    pub fn clr_tif(&self) -> &Self {
        self.periph.clr_ch_tif(self.index);
        self
    }
    pub fn wait_tif(&self) -> &Self {
        while !self.tif() {}
        self
    }
}

impl Start<u32> for LpitCh {
    fn start(&self, value: u32) -> &Self {       
        self.start_down(value)
    }
}


impl StartOnce<u32> for LpitCh {
    fn start_once(&self, value: u32) -> &Self {       
        self.start_down_once(value)
    }
}

impl StartDown<u32> for LpitCh {
    fn start_down(&self, value: u32) -> &Self {               
        self.periph
            .with_tctrl(self.index, |r| r.set_tsoi(0))        
            .set_ch_value(self.index, value)
            .clr_ch_tif(self.index)
            .set_ch_enabled(self.index, true);
        self
    }
}


impl StartDownOnce<u32> for LpitCh {
    fn start_down_once(&self, value: u32) -> &Self {       
        self.clr_tif();
        self.periph
            .with_tctrl(self.index, |r| r.set_tsoi(1))
            .set_ch_value(self.index, value)
            .clr_ch_tif(self.index)
            .set_ch_enabled(self.index, true);
        self
    }
}

impl Stop for LpitCh {
    fn stop(&self) -> &Self {
        self.periph.set_ch_enabled(self.index, false);
        self
    }
}

impl Period<u32> for LpitCh {
    fn period(&self) -> u32 {
        self.periph.ch_value(self.index)
    }
}

impl SetPeriod<u32> for LpitCh {
    fn set_period(&self, value: u32) -> &Self {
        self.periph.set_ch_value(self.index, value);
        self
    }
}

impl Counter<u32> for LpitCh {
    fn counter(&self) -> u32 {
        self.periph.ch_value(self.index)
    }
}

impl Timeout for LpitCh {
    fn test_timeout(&self) -> bool {
        self.tif()
    }

    fn clr_timeout(&self) -> &Self {
        self.clr_tif()
    }
}

impl Delay<u32> for LpitCh {
    fn delay(&self, value: u32) -> &Self {
        self
            .stop()
            .clr_timeout()
            .start_once(value)
            .wait_timeout()
    }
}
