pub use ::chip::crc::*;

#[derive(Debug, Clone, Copy)]
pub enum PolySize {
    Bits32 = 0b00,
    Bits16 = 0b01,
    Bits8 = 0b10,
    Bits7 = 0b11,
}

pub struct Config {
    pub rev_out: bool,
    pub rev_in: bool,
    pub polysize: PolySize,
    pub poly: u32,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            rev_out: false,
            rev_in: false,
            polysize: PolySize::Bits32,
            poly: 0x04C11DB7,
        }
    }
}

impl Config {
    pub fn bits_32() -> Config {
        Config::default()
    }
    pub fn bits_16() -> Config {
        Config::default().with_polysize(PolySize::Bits16)
    }
    pub fn with_polysize(mut self, polysize: PolySize) -> Config {
        self.polysize = polysize;
        self
    }
    pub fn with_poly(mut self, poly: u32) -> Config {
        self.poly = poly;
        self
    }
}

pub fn configure(mut crc: Crc, cfg: &Config) {
    unsafe {
        crc.with_cr(|r| r
            .set_rev_out(if cfg.rev_out { 1 } else { 0 })
            .set_rev_in(if cfg.rev_in { 1 } else { 0 })
            .set_polysize(cfg.polysize as u32)
        );
        crc.set_pol(Pol(0).set_pol(cfg.poly));
    }
}

pub fn initialize(mut crc: Crc, value: u32) {
    unsafe {
        crc.set_init(Init(0).set_init(value));
        crc.with_cr(|r| r.set_reset(1));
    }
}

pub fn write(mut crc: Crc, value: u32) {
    unsafe {
        crc.set_dr(Dr(0).set_dr(value))
    }
}

pub fn read(crc: Crc) -> u32 {
    unsafe {
        crc.dr().dr()
    }
}