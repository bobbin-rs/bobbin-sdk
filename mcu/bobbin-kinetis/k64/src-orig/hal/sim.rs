pub use ::chip::sim::*;

pub trait SimExt {
    fn enabled<P: En>(&self, p: &P) -> bool;
    fn set_enabled<P: En>(&self, p: &P, value: bool) -> &Self;
}

impl SimExt for Sim {
    fn enabled<P: En>(&self, p: &P) -> bool {
        self.en(p) != 0
    }
    fn set_enabled<P: En>(&self, p: &P, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.set_en(p, value);
        self
    }
    
}

pub fn set_enabled<P: En>(p: &P, value: bool) {
    SIM.set_enabled(p, value);
}

pub fn enable<P: En>(p: &P) {
    SIM.set_enabled(p, true);
}

pub fn disable<P: En>(p: &P) {
    SIM.set_enabled(p, false);
}

pub trait SimEnabled {
    fn sim_enabled(&self) -> bool;
    fn sim_set_enabled(&self, value: bool) -> &Self;
    fn sim_enable(&self) -> &Self { self.sim_set_enabled(true); self }
    fn sim_disable(&self) -> &Self { self.sim_set_enabled(false); self }
}

impl<P> SimEnabled for P where P: En {
    fn sim_enabled(&self) -> bool {
        self.en() != 0
    }
    fn sim_set_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.set_en(value);
        self
    }
}

fn to_u8_be(v: u32) -> [u8; 4] {
    [
        (v >> 24) as u8,
        (v >> 16) as u8,
        (v >> 8) as u8,
        (v >> 0) as u8,
    ]
}

fn to_hex(v: u8) -> [u8; 2] {
    pub const HEX: &[u8] = b"0123456789abcdef";
    [
        HEX[((v >> 4) & 0xf) as usize],
        HEX[((v >> 0) & 0xf) as usize],
    ]    
}

impl Sim {
    pub fn uid(&self) -> [u32; 4] {
        [self.uidh().0, self.uidmh().0, self.uidml().0, self.uidl().0]
    }

    pub fn write_uid(&self, buf: &mut[u8]) -> usize {
        assert!(buf.len() >= 32);
        let uid = self.uid();
        for i in 0..4 {
            let v = to_u8_be(uid[i]);
            for j in 0..4 {
                let h = to_hex(v[j]);
                for k in 0..2 {
                    buf[i * 8 + j * 2 + k] = h[k];
                }
                
            }        
        }
        32
    }

}