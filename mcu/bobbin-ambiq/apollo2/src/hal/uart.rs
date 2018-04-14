pub use bobbin_common::configure::*;
pub use bobbin_common::enabled::*;
pub use bobbin_common::hal::serial::*;
pub use ::chip::uart::*;

impl UartPeriph {
    pub fn init(&self) -> &Self {
        // Enable 24MHz Clock
        self.with_cr(|r| r.set_clksel(0x01).set_clken(1));
        
        // Set baud dividers
        // FUART/(16·BR) = IBRD + FBRD
        // 24_000_000 / (16 * 115_200) = IBRD + FBRD
        // IBRD = 13, FBRD = 0

        self.set_ibrd(|r| r.set_divint(6));
        self.set_fbrd(|r| r.set_divfrac(33));        
        // Enable FIFO and set 8 bit word length
        self.with_lcrh(|r| r.set_fen(1).set_wlen(0x3));
        // Enable TX and RX
        self.with_cr(|r| r.set_rxe(1).set_txe(1));
        self
    }
}

impl Enabled for UartPeriph {
    fn enabled(&self) -> bool {
        self.cr().test_uarten()
    }

    fn set_enabled(&self, value: bool) -> &Self {
        self.with_cr(|r| r.set_uarten(value))
    }
}


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