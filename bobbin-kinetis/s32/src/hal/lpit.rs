pub use ::chip::lpit::*;
use ::chip::irq::*;
use nvic;

pub enum TriggerSelect {
    Trigger0 = 0b00,
    Trigger1 = 0b01,
    Trigger2 = 0b10,
    Trigger3 = 0b11,
}

pub enum TriggerSource {
    External = 0,
    Internal = 1,
}

pub enum Mode {
    Periodic32 = 0b00,
    Periodic16Dual = 0b01,
    TriggerAccumulator = 0b10,
    TriggerInputCapture = 0b11,
}

#[derive(Clone)]
pub struct Timer {
    lpit: Lpit,
    channel: usize,
}
pub fn timer(lpit: Lpit, channel: usize) -> Timer {
    let mut lpit = lpit;
    unsafe { 
        lpit.with_mcr(|r| r.set_m_cen(1)); 
    }
    Timer { lpit: lpit, channel: channel }
}

impl Timer {
    pub fn lpit(&self) -> Lpit {
        self.lpit
    }

    pub fn channel(&self) -> usize {
        self.channel
    }

    pub fn doze_enabled(&self) -> bool {
        let lpit = self.lpit;
        unsafe {
            lpit.mcr().doze_en() != 0
        }
    }

    pub fn set_doze_enabled(&self, value: bool) {
        let value = if value { 1 } else { 0 };
        let mut lpit = self.lpit;
        unsafe {
            lpit.with_mcr(|r| r.set_doze_en(value))
        }
    }

    pub fn tif(&self) -> bool {
        let lpit = self.lpit;
        unsafe {
            lpit.msr().tif(self.channel) != 0
        }
    }

    pub fn clr_tif(&self) {
        let mut lpit = self.lpit;
        unsafe {
            lpit.set_msr(Msr(0).set_tif(self.channel, 1))
        }
    }  

    pub fn tie(&self) -> bool {
        let lpit = self.lpit;
        unsafe {
            lpit.mier().tie(self.channel) != 0
        }
    }

    pub fn set_tie(&self, value: bool) {
        let value = if value { 1 } else { 0 };
        let mut lpit = self.lpit;
        unsafe {
            lpit.with_mier(|r| r.set_tie(self.channel, value))
        }
    }

    pub fn value(&self) -> u32 {
        let lpit = self.lpit;
        unsafe {
            lpit.tval(self.channel).tmr_val()
        }
    }

    pub fn set_value(&self, value: u32) {
        let mut lpit = self.lpit;
        unsafe {
            lpit.set_tval(self.channel, Tval(0).set_tmr_val(value))
        }
    }    

    pub fn cur_value(&self) -> u32 {
        let lpit = self.lpit;
        unsafe {
            lpit.cval(self.channel).tmr_cur_val()
        }
    }    

    pub fn trg_sel(&self) -> TriggerSelect {
        let lpit = self.lpit;
        unsafe {
            match lpit.tctrl(self.channel).trg_sel() {
                0b00 => TriggerSelect::Trigger0,
                0b01 => TriggerSelect::Trigger1,
                0b10 => TriggerSelect::Trigger2,
                0b11 => TriggerSelect::Trigger3,                
                _ => panic!("Invalid TriggerSelect value")
            }
        }
    }

    pub fn set_trg_sel(&self, value: TriggerSelect) {
        let mut lpit = self.lpit;
        unsafe {
            lpit.with_tctrl(self.channel, |r| r.set_trg_sel(value as u32))
        }
    }

    pub fn trg_src(&self) -> TriggerSource {
        let lpit = self.lpit;
        unsafe {
            match lpit.tctrl(self.channel).trg_src() {
                0b0 => TriggerSource::External,
                0b1 => TriggerSource::Internal,
                _ => panic!("Invalid TriggerSource value")
            }
        }
    }

    pub fn set_trg_src(&self, value: TriggerSource) {
        let mut lpit = self.lpit;
        unsafe {
            lpit.with_tctrl(self.channel, |r| r.set_trg_src(value as u32))
        }
    }    

    pub fn trot(&self) -> bool {
        let lpit = self.lpit;
        unsafe {
            lpit.tctrl(self.channel).trot() != 0
        }
    }    
    pub fn set_trot(&self, value: bool) {
        let value = if value { 1 } else { 0 };
        let mut lpit = self.lpit;
        unsafe {
            lpit.with_tctrl(self.channel, |r| r.set_trot(value))
        }
    }       

    pub fn tsoi(&self) -> bool {
        let lpit = self.lpit;
        unsafe {
            lpit.tctrl(self.channel).tsoi() != 0
        }
    }    
    pub fn set_tsoi(&self, value: bool) {
        let value = if value { 1 } else { 0 };
        let mut lpit = self.lpit;
        unsafe {
            lpit.with_tctrl(self.channel, |r| r.set_tsoi(value))
        }
    }    

    pub fn tsot(&self) -> bool {
        let lpit = self.lpit;
        unsafe {
            lpit.tctrl(self.channel).tsot() != 0
        }
    }    
    pub fn set_tsot(&self, value: bool) {
        let value = if value { 1 } else { 0 };
        let mut lpit = self.lpit;
        unsafe {
            lpit.with_tctrl(self.channel, |r| r.set_tsot(value))
        }
    }               

    pub fn mode(&self) -> Mode {
        let lpit = self.lpit;
        unsafe {
            match lpit.tctrl(self.channel).mode() {
                0b00 => Mode::Periodic32,
                0b01 => Mode::Periodic16Dual,
                0b10 => Mode::TriggerAccumulator,
                0b11 => Mode::TriggerInputCapture,
                _ => panic!("Invalid Mode value")
            }
        }
    }

    pub fn set_mode(&self, value: Mode) {
        let mut lpit = self.lpit;
        unsafe {
            lpit.with_tctrl(self.channel, |r| r.set_mode(value as u32))
        }
    }        

    pub fn chain(&self) -> bool {
        let lpit = self.lpit;
        unsafe {
            lpit.tctrl(self.channel).chain() != 0
        }
    }

    pub fn set_chain(&self, value: bool) {
        assert!(!(self.channel == 0 && value), "Channel 0 may not be chained");
        let value = if value { 1 } else { 0 };
        let mut lpit = self.lpit;
        unsafe {
            lpit.with_tctrl(self.channel, |r| r.set_chain(value))
        }
    }   

    pub fn enabled(&self) -> bool {
        let lpit = self.lpit;
        unsafe {
            lpit.tctrl(self.channel).t_en() != 0
        }
    }

    pub fn set_enabled(&self, value: bool) {
        let mut lpit = self.lpit;
        unsafe {
            if value {
                lpit.set_setten(Setten(0).set_set_t_en(self.channel, 1))
            } else {
                lpit.set_clrten(Clrten(0).set_clr_t_en(self.channel, 1))
            }
            
        }        
    }

    pub fn irq(&self) -> Irq {
        match self.channel {
            0 => IRQ_LPIT0_CH0,
            1 => IRQ_LPIT0_CH1,
            2 => IRQ_LPIT0_CH2,
            3 => IRQ_LPIT0_CH3,
            _ => panic!("No IRQ for Channel {}", self.channel)
        }
    }
    
    pub fn set_handler(&self, handler: Option<unsafe extern "C" fn()>) {
        self.irq().set_handler(handler);        
        nvic::set_enabled(self.irq().0, handler.is_some())
    }
}