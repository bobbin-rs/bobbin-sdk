pub use ::hal::clock::{self, Hz, ClockTree, Clock};

pub const CLK_120MHZ: &Clk120MHz = &Clk120MHz {};
pub const CLK_60MHZ: &Clk60MHz = &Clk60MHz {};

pub static mut CLK: Option<&'static ClockTree> = None;

pub struct Clk120MHz {}

impl Clk120MHz {
    pub fn configure(&self) {
        clock::set_clock(3, 96, 0, 4, 0);
        unsafe {
            CLK = Some(CLK_120MHZ);
        }
    }
}

impl ClockTree for Clk120MHz {
    fn mosc(&self) -> Hz {
        Some(25_000_000)
    }
    fn rtosc(&self) -> Hz {
        Some(32768)
    }
    fn sysclk(&self) -> Hz {
        Some(120_000_000)
    }
}

pub struct Clk60MHz {}

impl Clk60MHz {
    pub fn configure(&self) {
        clock::set_clock(7, 96, 0, 4, 0);
        unsafe {
            CLK = Some(CLK_60MHZ);
        }
    }
}

impl ClockTree for Clk60MHz {
    fn mosc(&self) -> Hz {
        Some(25_000_000)
    }
    fn rtosc(&self) -> Hz {
        Some(32768)
    }
    fn sysclk(&self) -> Hz {
        Some(60_000_000)
    }
}

unsafe impl Send for Clk120MHz {}

pub fn init() {
    CLK_120MHZ.configure();
}

pub fn clk() -> &'static ClockTree {
    unsafe { CLK.unwrap() }
}

pub fn sysclk_hz() -> u32 {
    clk().sysclk().expect("No clock specified")    
}