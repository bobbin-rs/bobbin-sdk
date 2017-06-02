use core::fmt::{self, Write};

use ::chip::sercom::{self, Sercom};

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


pub struct UsartDevice {
    sercom: Sercom,
}

pub fn device(sercom: Sercom) -> UsartDevice {
    UsartDevice { sercom: sercom }
}

impl UsartDevice {
    pub fn configure(&self, baud: u16, tx_pad: u8, rx_pad: u8) {
        let mut s = self.sercom.usart();
        unsafe {
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

            s.set_ctrla(sercom::usart::Ctrla(0)
                .set_txpo(tx_pad as u32)
                .set_rxpo(rx_pad as u32)
                .set_mode(0x1)
                .set_dord(1)
            );

            // Wait for synchronization
            while s.syncbusy().ctrlb() != 0 {}

            // Enable transmit and receive and set data size to 8 bits

            s.set_ctrlb(sercom::usart::Ctrlb(0)
                .set_rxen(1)
                .set_txen(1)
                .set_chsize(0)
            );

            // Load the baud value
            s.set_baud(sercom::usart::Baud(baud));

            // Wait for synchronization
            while s.syncbusy().enable() != 0 {}

            s.with_ctrla(|r| r.set_enable(1));
        }
    }

    pub fn write(&self, buf: &[u8]) {
        for b in buf.iter() {
            self.putc(*b);
        }
    }

    pub fn putc(&self, c: u8) {
        unsafe {
            let mut s = self.sercom.usart();
            while s.intflag().dre() == 0 {}
            s.set_data(sercom::usart::Data(0).set_data(c as u16));
        }
    }

    pub fn try_getc(&self) -> Option<u8> {
        unsafe {
            let s = self.sercom.usart();
            if s.intflag().rxc() != 0 {
                Some(s.data().data() as u8)
            } else {
                None
            }        
        }
    }    
}

impl Write for UsartDevice {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.putc(byte);
        }
        Ok(())
    }
}