pub use bobbin_common::hal::spi::*;
pub use ::chip::sercom::*;
use ::chip::gclk;

impl SercomPeriph {
    pub fn init_spi(&self, baud: u8, dopo: u8, dipo: u8) {
        let s = self.spi();

        // Set GCLK_GEN0 as source for SERCOM

        gclk::GCLK.set_clkctrl(|_| gclk::Clkctrl(0)
            .set_id(0x14 + self.index())
            .set_gen(0x0)
            .set_clken(1)
        );

        // Wait for synchronization
        while gclk::GCLK.status().syncbusy() != 0 {}

        // Disable the SERCOM module
        s.with_ctrla(|r| r.set_enable(0));

        // Wait for synchronization
        while s.syncbusy().swrst() != 0 {}
        
        // Software Reset
        s.with_ctrla(|r| r.set_swrst(1));
    
        // Wait for synchronization
        while s.ctrla().swrst() != 0 {}

        // Wait for synchronization
        while s.syncbusy().swrst() != 0 || s.syncbusy().enable() != 0 {}
        
        // 1. Select SPI mode in master / slave operation in the Operating Mode bit group in the CTRLA register (CTRLA.MODE= 0x2 or 0x3 ).
        // 2. Select transfer mode for the Clock Polarity bit and the Clock Phase bit in the CTRLA register (CTRLA.CPOL and CTRLA.CPHA) if desired.
        // 3. Select the Frame Format value in the CTRLA register (CTRLA.FORM).
        // 4. Configure the Data In Pinout field in the Control A register (CTRLA.DIPO) for SERCOM pads of the receiver.
        // 5. Configure the Data Out Pinout bit group in the Control A register (CTRLA.DOPO) for SERCOM pads of the transmitter.
        // 6. Select the Character Size value in the CTRLB register (CTRLB.CHSIZE).
        // 7. Write the Data Order bit in the CTRLA register (CTRLA.DORD) for data direction.

        // MODE=0x3 (Master)
        // CPOL=0x0
        // CPHA=0x0
        // DOPO=0x1 (MOSI Pad 2, SCK Pad 3)
        // DIPO=0x0 (MISO PAD 0)
        // DORD=0x0 (MSB First)

        // CHSIZE=0x0 (8 bit)

        s.with_ctrla(|r| r.set_mode(0x3).set_cpol(0x0).set_cpha(0x0).set_dopo(dopo).set_dipo(dipo).set_dord(0x0));
        s.with_ctrlb(|r| r.set_chsize(0x0));

        // 8. If the SPI is used in master mode:
        // 8.1. Select the desired baud rate by writing to the Baud register (BAUD).

        // #define SERCOM_FREQ_REF 48000000
        // uint8_t SERCOM::calculateBaudrateSynchronous(uint32_t baudrate)
        // {
        //   return SERCOM_FREQ_REF / (2 * baudrate) - 1;
        // }

        // Ex: 1mhz baud rate, br = 48_000_000 / (2 * 1_000_000) - 1 = 23;

        //let br: u8 = 48_000_000 / (2 * baud_rate) - 1;

        s.set_baud(|_| spi::Baud(0).set_baud(baud));                    
    }
    
    pub fn set_baud(&self, baud: u8) {
        {
            let s = self.spi();
            s.set_baud(|_| spi::Baud(0).set_baud(baud));            
        }
    }

    pub fn set_lsbfirst(&self, value: bool) {
        let value = if value { 1 } else { 0 };
        {            
            let s = self.spi();
            s.with_ctrla(|r| r.set_dord(value));
        }
    }

    pub fn set_rxen(&self, value: bool) {
        let value = if value { 1 } else { 0 };
        {
            let s = self.spi();
            s.with_ctrlb(|r| r.set_rxen(value));
            while s.syncbusy().ctrlb() != 0 {}
        }        
    }

    pub fn set_enabled(&self, value: bool) {
        let value = if value { 1 } else { 0 };
        {
            let s = self.spi();
            s.with_ctrla(|r| r.set_enable(value));
            while s.syncbusy().enable() != 0 {}
        }        
    }   
    pub fn xfer(&self, out: u8) -> u8 {
        let s = self.spi();
        while s.intflag().dre() == 0 {}
        s.set_data(|_| spi::Data(0).set_data(out));
        while s.intflag().rxc() == 0 {}
        s.data().data().value() as u8
    }
    pub fn spi_transfer(&self, bytes_out: &[u8], bytes_in: &mut [u8]) {
        for i in 0..bytes_out.len() {
            bytes_in[i] = self.xfer(bytes_out[i]);
        }
    }
}


// impl SpiTransfer for SercomPeriph {
//     fn transfer(&self, bytes_out: &[u8], bytes_in: &mut [u8]) {
//         for i in 0..bytes_out.len() {
//             bytes_in[i] = self.xfer(bytes_out[i]);
//         }
//     }
// }



// impl SpiTransfer for SercomPeriph {
//     fn transfer(sercom: Sercom,  {
//         unimplemented!()
//         // unsafe {
//         //     let mut s = sercom.spi();
//         //     while s.intflag().dre() == 0 {}
//         //     s.set_data(spi::Data(0).set_data(out as u32));
//         //     while s.intflag().rxc() == 0 {}
//         //     s.data().data() as u8
//         // }
//     }    
// }