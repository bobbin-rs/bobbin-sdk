pub use bobbin_common::serial::*;
pub use ::chip::usart::*;
pub use ::hal::cmu::CmuEnabled;

impl SerialTx<u8> for UsartPeriph  {
    fn can_tx(&self) -> bool {
        self._if().txbl() != 0
    }

    fn tx(&self, c: u8) -> &Self {
        self.set_txdata(|r| r.set_txdata(c))
    }
}

impl SerialRx<u8> for UsartPeriph {
    fn can_rx(&self) -> bool {
        self._if().rxdatav() == 0
    }

    fn rx(&self) -> u8 {
        self.rxdata().rxdata().value()
    }
}