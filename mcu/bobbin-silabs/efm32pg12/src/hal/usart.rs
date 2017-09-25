pub use bobbin_common::serial::*;
pub use ::chip::usart::*;
pub use ::hal::cmu::CmuEnabled;

impl UsartPeriph {
    pub fn set_baud(&self, clock: u32, baud: u32) -> &Self {
        // Comment from emlib driver
        /* 
        * We want to use integer division to avoid forcing in float division
        * utils, and yet keep rounding effect errors to a minimum.
        *
        * CLKDIV in asynchronous mode is given by:
        *
        * CLKDIV = 256 * (fHFPERCLK/(oversample * br) - 1)
        * or
        * CLKDIV = (256 * fHFPERCLK)/(oversample * br) - 256
        *
        * The basic problem with integer division in the above formula is that
        * the dividend (256 * fHFPERCLK) may become higher than max 32 bit
        * integer. Yet, we want to evaluate dividend first before dividing in
        * order to get as small rounding effects as possible. We do not want
        * to make too harsh restrictions on max fHFPERCLK value either.
        *
        * One can possibly factorize 256 and oversample/br. However,
        * since the last 6 or 3 bits of CLKDIV are don't care, we can base our
        * integer arithmetic on the below formula
        *
        * CLKDIV / 64 = (4 * fHFPERCLK)/(oversample * br) - 4 (3 bits dont care)
        * or
        * CLKDIV / 8  = (32 * fHFPERCLK)/(oversample * br) - 32 (6 bits dont care)
        *
        * and calculate 1/64 of CLKDIV first. This allows for fHFPERCLK
        * up to 1GHz without overflowing a 32 bit value!
        */

        // NOTE: The comment assumes you are setting the CLKDIV register directly.
        // In this case we are setting CLKDIV.div, which requires shifting right by 3.
        // Alternatively, we could set CLKDIV directly.

        let oversample = 16;    

        let clkdiv  = 4 * clock + (oversample * baud) / 2;
        let clkdiv = clkdiv / (oversample * baud);
        let clkdiv = clkdiv - 4;
        let clkdiv = clkdiv * 64;    

        self.set_clkdiv(|r| r.set_div(clkdiv >> 3));        
        self
    }
}

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