pub use bobbin_common::serial::*;
pub use chip::sercom::*;
pub use super::pm::PmEnabled;

// NOTE: Before usage, power up and set clocks

// pm::set_sercom_enabled(sercom, true);

// // Set GCLK_GEN0 as source for SERCOM

// gclk::GCLK.set_clkctrl(gclk::Clkctrl(0)
//     .set_id(0x14 + sercom_index(sercom))
//     .set_gen(0x0)
//     .set_clken(1)
// );
// // Wait for synchronization
// while gclk::GCLK.status().syncbusy() != 0 {}


pub trait Usart {
    fn configure(&self, baud: u16, tx_pad: u8, rx_pad: u8) -> &Self;
    fn set_baud(&self, baud: u16) -> &Self;
    fn set_enabled(&self, bool) -> &Self;
    // fn write(&self, buf: &[u8]) -> &Self;
}

impl<T> Usart for Periph<T> {
    fn configure(&self, baud: u16, tx_pad: u8, rx_pad: u8) -> &Self {
        let s = self.usart();

        // Before Use: Power up SERCOM
        // Before Use: Set SERCOM Clock

        // UART Initialization

        // Wait for synchronization
        while s.syncbusy().enable() != 0 {}

        // Disable the UART module
        s.with_ctrla(|r| r.set_enable(0));

        // Wait for synchronization
        while s.syncbusy().swrst() != 0 {}
        
        // Software Reset
        s.with_ctrla(|r| r.set_swrst(1));
    
        // Wait for synchronization
        while s.ctrla().swrst() != 0 {}

        // Wait for synchronization
        while s.syncbusy().swrst() != 0 || s.syncbusy().enable() != 0 {}

        // Update the UART pad settings, mode and data order settings

        s.set_ctrla(|r| r
            .set_txpo(tx_pad as u32)
            .set_rxpo(rx_pad as u32)
            .set_mode(0x1)
            .set_dord(1)
        );

        // Wait for synchronization
        while s.syncbusy().ctrlb() != 0 {}

        // Enable transmit and receive and set data size to 8 bits

        s.set_ctrlb(|r| r
            .set_rxen(1)
            .set_txen(1)
            .set_chsize(0)
        );

        // Load the baud value
        self.set_baud(baud);
        self
    }

    fn set_baud(&self, value: u16) -> &Self {
        let s = self.usart();
        s.set_baud(|_| usart::Baud(value));
        while s.syncbusy().enable() != 0 {}
        self
    }

    fn set_enabled(&self, value: bool) -> &Self {
        let s = self.usart();
        s.with_ctrla(|r| r.set_enable(value));
        self
    }

    // fn write(&self, buf: &[u8]) -> &Self {
    //     for b in buf.iter() {
    //         self.putc(*b);
    //     }
    //     self
    // }
}

impl<T> SerialTx<u8> for Periph<T> {    
    fn can_tx(&self) -> bool {
        self.usart().intflag().dre() != 0
    }

    fn tx(&self, c: u8) -> &Self {
        self.usart().set_data(|r| r.set_data(c));
        self
    }
}

impl<T> SerialRx<u8> for Periph<T> {
    fn can_rx(&self) -> bool {
        self.usart().intflag().rxc() != 0
    }

    fn rx(&self) -> u8 {
        self.usart().data().data().value() as u8
    }
}