pub use ::hal::clock::{self, Hz, ClockTree, DynamicClockTree, Clock};

pub const CLK_120MHZ: &Clk120MHz = &Clk120MHz {};
pub const CLK_60MHZ: &Clk60MHz = &Clk60MHz {};
pub const CLK_16MHZ: &Clk16MHz = &Clk16MHz {};
pub const CLK_32KHZ: &Clk32KHz = &Clk32KHz {};
pub const CLK: &DynamicClockTree = &DynamicClockTree {
    osc: Some(25_000_000),
    xosc: Some(32768),
};

//pub static mut CLK: Option<&'static ClockTree> = None;

pub struct Clk32KHz {}
impl Clk32KHz {
    pub fn configure(&self) {
        use ::chip::sysctl::SYSCTL;
        SYSCTL.with_rsclkcfg(|r| r.set_osysdiv(0).set_usepll(0).set_oscsrc(4));
    }
}

pub struct Clk16MHz {}
impl Clk16MHz {
    pub fn configure(&self) {
        use ::chip::sysctl::SYSCTL;
        SYSCTL.with_rsclkcfg(|r| r.set_osysdiv(0).set_usepll(0).set_oscsrc(0));
    }
}

pub struct Clk120MHz {}

impl Clk120MHz {
    pub fn configure(&self) {
        clock::set_clock(3, 96, 0, 4, 0);
    }
}

impl ClockTree for Clk120MHz {
    fn sysclk(&self) -> Hz {
        Some(120_000_000)
    }
}

pub struct Clk60MHz {}

impl Clk60MHz {
    pub fn configure(&self) {
        clock::set_clock(7, 96, 0, 4, 0);
    }
}

impl ClockTree for Clk60MHz {
    fn sysclk(&self) -> Hz {
        Some(60_000_000)
    }
}

unsafe impl Send for Clk120MHz {}

pub fn init() {
    CLK_120MHZ.configure();
}

pub fn clk() -> &'static ClockTree {
    CLK
}

pub fn sysclk_hz() -> u32 {
    clk().sysclk().expect("No clock specified")    
}