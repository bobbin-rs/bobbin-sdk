pub use bobbin_common::configure::*;
pub use bobbin_common::enabled::*;
pub use bobbin_common::can::*;

pub use ::chip::flexcan::*;

#[derive(Debug, Default)]
pub struct Config {
    ctrl1: Ctrl1,
}

impl Config {
    pub fn set_loopback(&self, value: bool) -> &Self {
        self.ctrl1 = self.ctrl1.set_lpb(1);
        self
    }
}


impl Configure<Config> for FlexcanPeriph {
    fn config(&self) -> Config {
        Config {
            ctrl1: Ctrl1(0),
        }
    }
    fn configure(&self, cfg: Config) -> &Self {
        self
            .set_ctrl1(|_| cfg.ctrl1)
    }
}

impl Enabled for FlexcanPeriph {
    fn enabled(&self) -> bool {        
        !self.mcr().test_mdis()
    }
    fn set_enabled(&self, value: bool) -> &Self {
        self.with_mcr(|r| r.set_mdis(!value))
    }
}
