use chip::lpit::*;

pub trait LpitExt {
    fn enabled(&self) -> bool;
    fn set_enabled(&self, value: bool) -> &Self;

    fn doze_enabled(&self) -> bool;
    fn set_doze_enabled(&self, value: bool) -> &Self;

    fn ch_enabled(&self, index: usize) -> bool;
    fn set_ch_enabled(&self, index: usize, value: bool) -> &Self;

    fn ch_value(&self, index: usize) -> u32;
    fn set_ch_value(&self, index: usize, value: u32) -> &Self;

    fn ch_tie(&self, index: usize) -> bool;
    fn set_ch_tie(&self, index: usize, value: bool) -> &Self;

    fn ch_tif(&self, index: usize) -> bool;
    fn clr_ch_tif(&self, index: usize) -> &Self;    
}

impl LpitExt for LpitImpl {
    fn enabled(&self) -> bool {
        self.mcr().m_cen() != 0
    }
    fn set_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.with_mcr(|r| r.set_m_cen(value))
    }

    fn doze_enabled(&self) -> bool {
        self.mcr().doze_en() != 0
    }
    fn set_doze_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.with_mcr(|r| r.set_doze_en(value))
    }    

    fn ch_enabled(&self, index: usize) -> bool {
        self.tctrl(index).t_en() != 0
    }

    fn set_ch_enabled(&self, index: usize, value: bool) -> &Self {
        if value {
            self.set_setten(Setten(0).set_set_t_en(index, 1))
        } else {
            self.set_clrten(Clrten(0).set_clr_t_en(index, 1))
        }
    }

    fn ch_value(&self, index: usize) -> u32 {
        self.tval(index).tmr_val()
    }
    fn set_ch_value(&self, index: usize, value: u32) -> &Self {
        self.set_tval(index, Tval(0).set_tmr_val(value))
    }    

    fn ch_tie(&self, index: usize) -> bool {
        self.mier().tie(index) != 0
    }
    fn set_ch_tie(&self, index: usize, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.with_mier(|r| r.set_tie(index, value))
    }    

    fn ch_tif(&self, index: usize) -> bool {
        self.msr().tif(index) != 0
    }
    fn clr_ch_tif(&self, index: usize) -> &Self {
        self.set_msr(Msr(0).set_tif(index, 1))
    }       
}