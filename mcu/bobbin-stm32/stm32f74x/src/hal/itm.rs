use core::fmt::{self, Arguments, Write};
use cortex_m;
use ::chip::dbg::DBG;

pub struct Port {}

impl Write for Port {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write_bytes(s.as_bytes());
        Ok(())
    }
}

pub fn write_byte(b: u8) {
    let stim = &cortex_m::peripheral::itm().stim[0];
    while !stim.is_fifo_ready() {}
    stim.write_u8(b);
}

pub fn write_bytes(bytes: &[u8]) {
    let mut bytes = bytes;
    let stim = &cortex_m::peripheral::itm().stim[0];
    while bytes.len() >= 4 {
        while !stim.is_fifo_ready() {}
        stim.write_u32((bytes[3] as u32) << 24 | (bytes[2] as u32) << 16 | (bytes[1] as u32) << 8 | bytes[0] as u32);
        bytes = &bytes[4..];
    }
    while bytes.len() >= 2 {
        while !stim.is_fifo_ready() {}
        stim.write_u16((bytes[1] as u16) << 8 | bytes[0] as u16);
        bytes = &bytes[2..];
    }
    while bytes.len() > 0 {
        while !stim.is_fifo_ready() {}
        stim.write_u8(bytes[0]);
        bytes = &bytes[1..];
    }    
}

pub fn write_fmt(args: Arguments) {
    Port {}.write_fmt(args).ok();
}

pub fn write_str(s: &str) {
    Port {}.write_str(s).ok();
}


pub fn init() {
    unsafe {
        let mut dbg = DBG;
        let dcb = cortex_m::peripheral::dcb_mut();
        let itm = cortex_m::peripheral::itm_mut();

        // DBGMCU: enable asynchronous tracing
        // NOTE(0b00) asynchronous mode
        dbg.with_cr(|r| r.set_trace_ioen(1).set_trace_mode(0b00));

        // DCB: enable the ITM
        let demcr = dcb.demcr.read();
        dcb.demcr.write({
            // Enable DWT and ITM
            const TRCENA: u32 = 1 << 24;

            demcr | TRCENA
        });

        // ITM: unlock the peripheral
        const KEY: u32 = 0xc5acce55;
        itm.lar.write(KEY);

        // ITM: enable the whole peripheral and assign an ID
        let tcr = itm.tcr.read();
        itm.tcr.write({
            // Enables the ITM
            const ITMENA: u32 = 1;
            // The ID of the ITM port. Anything different than 0 will do
            const TRACE_BUS_ID: u32 = 0b1 << 16;
            const TRACE_BUS_ID_MASK: u32 = 0b1111111 << 16;

            ((tcr | ITMENA) & !TRACE_BUS_ID_MASK) | TRACE_BUS_ID
        });

        itm.tpr.write(0xffffffff);

        // ITM: enable the stimulus port 0
        let ter = itm.ter[0].read();
        itm.ter[0].write({
            // Which port
            const N: u32 = 0;

            ter | 1 << N
        });    
    }
}