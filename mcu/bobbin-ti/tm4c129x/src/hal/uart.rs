pub use bobbin_common::configure::*;
pub use bobbin_common::enabled::*;
pub use bobbin_common::hal::serial::*;

pub use chip::uart::*;
pub use super::sysctl::SysctlEnabled;

use bobbin_common::bits::*;

#[derive(Debug)]
pub struct Config {
    ibrd: Ibrd,
    fbrd: Fbrd,
    lcrh: Lcrh,
    ctl: Ctl,
}

impl Default for Config {
    #[inline]
    fn default() -> Self {
        Config {
            ibrd: Ibrd(0),
            fbrd: Fbrd(0),
            lcrh: Lcrh(0).set_wlen(0x3).set_fen(1), // 8 bit, FIFO enabled
            ctl: Ctl(0).set_hse(1), // High speed clock
        }
    }
}

impl Config {
    pub fn set_baud(mut self, baud_hz: u32, clock_hz: u32) -> Self {
        let baud_div = ((8 * clock_hz) / baud_hz) + 1;
        let baud_int = baud_div / 64;
        let baud_frac = baud_div % 64;
        self.ibrd = self.ibrd.set_divint(baud_int);
        self.fbrd = self.fbrd.set_divfrac(baud_frac);
        self
    }
    pub fn set_wlen(mut self, wlen: U2) -> Self {
        self.lcrh = self.lcrh.set_wlen(wlen);
        self
    }
    pub fn set_fen(mut self, fen: U1) -> Self {
        self.lcrh = self.lcrh.set_fen(fen);
        self
    }
}


impl Configure<Config> for UartPeriph {
    fn config(&self) -> Config {
        Config {
            ibrd: self.ibrd(),
            fbrd: self.fbrd(),
            lcrh: self.lcrh(),
            ctl: self.ctl(),
        }        
    }

    fn configure(&self, cfg: Config) -> &Self {
        self
            .disable()
            .set_ibrd(|_| cfg.ibrd)
            .set_fbrd(|_| cfg.fbrd)
            .set_lcrh(|_| cfg.lcrh)
            .set_ctl(|_| cfg.ctl)
    }
}

impl Enabled for UartPeriph {
    fn enabled(&self) -> bool {
        self.ctl().uarten() != 0
    }

    fn set_enabled(&self, value: bool) -> &Self {
        self.with_ctl(|r| r
            .set_uarten(value)
            .set_rxe(value)
            .set_txe(value))
    }
}

// impl UartPeriph {
//     pub fn configure(&self, baud_hz: u32, sysclk_hz: u32) -> &Self {
//         let baud_div = ((8 * sysclk_hz) / baud_hz) + 1;
//         let baud_int = baud_div / 64;
//         let baud_frac = baud_div % 64;

//         self.with_ctl(|r| r.set_uarten(0));
//         self.with_ibrd(|r| r.set_divint(baud_int));
//         self.with_fbrd(|r| r.set_divfrac(baud_frac));
//         self.with_lcrh(|r| r.set_wlen(0x3).set_fen(1));
//         self.with_ctl(|r| r.set_hse(1).set_rxe(1).set_txe(1));        
//         self
//     }
// }


impl SerialTx<u8> for UartPeriph  {
    fn can_tx(&self) -> bool {
        self.fr().txff() == 0
    }

    fn tx(&self, c: u8) -> &Self {
        self.set_dr(|r| r.set_data(c))
    }
}

impl SerialRx<u8> for UartPeriph {
    fn can_rx(&self) -> bool {
        self.fr().rxfe() == 0
    }

    fn rx(&self) -> u8 {
        self.dr().data().value()
    }
}