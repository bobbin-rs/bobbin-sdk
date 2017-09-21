pub use ::chip::emac::*;
//use ::driver::gpio::{Pin, AltFn};
use sysctl;
use core::fmt;
use core::ptr;
use core::cell::Cell;
use bit_field::BitField;

const TX_BUFFER_SIZE: usize = 1536;
const RX_BUFFER_SIZE: usize = 1536;
const RX_DESC_COUNT: usize = 4;
const TX_DESC_COUNT: usize = 2;

type TxDescs = [TxDesc; TX_DESC_COUNT];
type RxDescs = [RxDesc; RX_DESC_COUNT];
type TxBuf = [u8; TX_BUFFER_SIZE];
type RxBuf = [u8; RX_BUFFER_SIZE];
type TxBufs = [TxBuf; TX_DESC_COUNT];
type RxBufs = [RxBuf; RX_DESC_COUNT];

pub static mut TX_DESCS: TxDescs = [TX_DESC_EMPTY; TX_DESC_COUNT];
pub static mut RX_DESCS: RxDescs = [RX_DESC_EMPTY; RX_DESC_COUNT];
pub static mut TX_BUFS: TxBufs = [[0u8; TX_BUFFER_SIZE]; TX_DESC_COUNT];
pub static mut RX_BUFS: RxBufs = [[0u8; RX_BUFFER_SIZE]; RX_DESC_COUNT];

pub static mut TX_INDEX: usize = 0;
pub static mut RX_INDEX: usize = 0;

pub struct EmacDevice {
    emac: Emac,
    tx_index: Cell<usize>,
    rx_index: Cell<usize>,
}

pub fn device(emac: Emac) -> EmacDevice {
    EmacDevice { emac: emac, tx_index: Cell::new(0), rx_index: Cell::new(0) }
}

impl EmacDevice {
    pub fn init(&self) {
        unsafe {
            let mut emac = self.emac;

            // Enable MAC and wait until ready
            sysctl::set_emac_enabled(emac, true);
            while !sysctl::emac_ready() {}

            // Set MII CR
            
            // self.set_fes(true); // 100
            // self.set_dupm(true); // Full Duplex
            // self.set_ra(true);

            // Enable PHY and wait until ready
            sysctl::set_ephy_enabled(true);
            sysctl::set_ephy_power(true);
            while !sysctl::ephy_ready() {}

            // Wait for MAC to come out of reset
            while emac.dmabusmod().swr() != 0 {}

            // Initiate a PHY Reset and wait until ready
            self.write_phy(0, 0x000, 1 << 15);
            while (self.read_phy(0, 0x000) & 1 << 15) != 0 {}

            // Wait again until ready
            while !sysctl::ephy_ready() {}

            // Wait until link is up
            while (self.read_phy(0, 0x001) & 1 << 2) == 0 {}
            trace!("link up!");

            self.init_descriptors();

            // Enable Store and Forward            
            emac.with_dmaopmode(|r| r.set_rsf(1).set_tsf(1));  

            // Set Extended Descriptors
            emac.with_dmabusmod(|r| r.set_atds(1));          

            // Strip CRC of Ether frames
            emac.with_cfg(|r| r.set_cst(1));

            // Receive all  
            emac.with_framefltr(|r| r.set_ra(1).set_pr(1));

            // Disable MMC interrupts

            emac.with_mmcrxim(|r| { r
                .set_gbf(0)
                .set_crcerr(0)
                .set_algnerr(0)
                .set_ucgf(0)
            });

            emac.with_mmctxim(|r| { r
                .set_gbf(0)
                .set_scollgf(0)
                .set_mcollgf(0)
                .set_octcnt(0)
            });
            emac.set_dmaris(Dmaris(0).set_ti(1).set_tu(1).set_ri(1));
            for _ in 0..1000 { asm!("nop") }
        }
        
        self.set_rx_enabled(true);
        self.set_tx_enabled(true);
    }

    pub fn set_loopback_mode(&self, value: bool) {
        let value = if value { 1 } else { 0 };
        let mut emac = self.emac;
        unsafe {
            emac.with_cfg(|r| r.set_loopbm(value));
        }
    }


    fn init_descriptors(&self) {
        unsafe {
            for i in 0..TX_DESC_COUNT {
                let next_desc = if i < TX_DESC_COUNT-1 {
                    &TX_DESCS[i+1]
                } else {
                    &TX_DESCS[0]
                };
                let mut td = &mut TX_DESCS[i];
                td.set_chained(true);
                td.set_buffer_1(&mut TX_BUFS[i]);
                td.set_next(next_desc);                
            }
            self.set_tx_desc(&TX_DESCS[0]);
            for i in 0..RX_DESC_COUNT {
                let next_desc = if i < RX_DESC_COUNT - 1 {
                    &RX_DESCS[i+1]
                } else {
                    &RX_DESCS[0]
                };
                //let mut rd = &mut RX_DESCS[i];
                let mut rd = ptr::read_volatile(&mut RX_DESCS[i] as *mut RxDesc);
                rd.set_owned(true);
                rd.set_chained(true);
                rd.set_buffer_1(&mut RX_BUFS[i]);
                rd.set_next(next_desc);
                
                ptr::write_volatile(&mut RX_DESCS[i] as *mut RxDesc, rd);                
                
            }
            self.set_rx_desc(&RX_DESCS[0]);

            // for i in 0..TX_DESC_COUNT {
            //     trace!("{} - {:?}", i, &TX_DESCS[i]);
            // }
            // for i in 0..RX_DESC_COUNT {
            //     trace!("{} - {:?}", i, &RX_DESCS[i]);
            // }
        }        
    }

    pub fn set_extended_descriptors(&self, value: bool) {
        let value = if value { 1 } else { 0 };
        unsafe {
            let mut emac = self.emac;
            emac.with_dmabusmod(|r| r.set_atds(value));
        }
    }

    pub fn set_rxdladdr(&self, value: u32) {
        unsafe {
            let mut emac = self.emac;
            emac.set_rxdladdr(Rxdladdr(0).set_strxlist(value >> 2));
        }        
    }

    pub fn set_txdladdr(&self, value: u32) {
        unsafe {
            let mut emac = self.emac;
            emac.set_txdladdr(Txdladdr(0).set_txdladdr(value >> 2));
        }        
    }    

    pub fn set_rx_desc(&self, desc: &RxDesc) {
        self.set_rxdladdr(desc as *const RxDesc as u32)
    }

    pub fn set_tx_desc(&self, desc: &TxDesc) {
        self.set_txdladdr(desc as *const TxDesc as u32)
    }

    pub fn set_rx_enabled(&self, value: bool) {
        let value = if value { 1 } else { 0 };
        unsafe {
            let mut emac = self.emac;
            emac.with_dmaopmode(|r| r.set_sr(value));
            emac.with_cfg(|r| r.set_re(value));
        }
    }

    pub fn set_tx_enabled(&self, value: bool) {
        let value = if value { 1 } else { 0 };
        unsafe {
            let mut emac = self.emac;
            emac.with_dmaopmode(|r| r.set_st(value));
            emac.with_cfg(|r| r.set_te(value));
        }
    }
    

    pub fn poll_rx(&self) {
        let mut emac = self.emac;
        unsafe { emac.set_rxpolld(Rxpolld(0).set_rpd(1)); }
    }

    pub fn poll_tx(&self) {
        let mut emac = self.emac;
        unsafe { emac.set_txpolld(Txpolld(0).set_tpd(1)); }
    }

    pub fn read_phy(&self, phy: u8, reg: u8) -> u16 {
        unsafe {
            let mut emac = self.emac;

            // Wait until not busy
            while emac.miiaddr().miib() != 0 {}

            emac.with_miiaddr(|r| 
                r.set_cr(1 as u32)
                    .set_pla(phy as u32)
                    .set_mii(reg as u32)
                    .set_miiw(0)
                    .set_miib(1)
            );

            // Wait until not busy
            while emac.miiaddr().miib() != 0 {}

            emac.miidata().data() as u16
        }
    }

    pub fn write_phy(&self, phy: u8, reg: u8, value: u16) {
        unsafe {
            let mut emac = self.emac;

            // Wait until not busy
            while emac.miiaddr().miib() != 0 {}

            emac.set_miidata(Miidata(0).set_data(value as u32));

            emac.with_miiaddr(|r| 
                r.set_cr(1 as u32)
                    .set_pla(phy as u32)
                    .set_mii(reg as u32)
                    .set_miiw(1)
                    .set_miib(1)
            );

            // Wait until not busy
            while emac.miiaddr().miib() != 0 {}
        }
    }        

    pub fn recv(&self, buf: &mut [u8]) -> usize {        
        self.poll_rx();        
        unsafe {
            // let emac = self.emac;
            // if emac.dmaris().ri() == 0 {
            //     return 0
            // }
            // //trace!("Receive interrupt!: {:?}", emac.dmaris());            
            // emac.set_dmaris(Dmaris(0).set_ri(1));


            let mut rx_index = self.rx_index.get();

            let mut rd = ptr::read_volatile(&mut RX_DESCS[rx_index] as *mut RxDesc);
            if rd.0[0] & 1 << 31 != 0 {
                return 0;
            }


            // for i in 0..RX_DESC_COUNT {
            //     let mut rd = ptr::read_volatile(&mut RX_DESCS[i] as *mut RxDesc);
            //     trace!("rd[{}]: owned? {}", i, rd.owned());
            // }            
            // trace!("rx_index: {}", rx_index);            
            // trace!("{}: {:?}", rx_index, &RX_DESCS[rx_index]);
            // trace!("HOSRXDESC: {:?}", emac.hosrxdesc());
            // trace!("HOSRXBA: {:?}", emac.hosrxba());


            let rb = rd.buffer_1();
            let len = rd.frame_length(); 

            // trace!("RB: {:p}, len: {}", rb, len);

            // if !rd.is_last() {
            //     trace!("ignoring: is_last is false");
            //     return 0;
            // }

            // trace!("rxdladdr: {:?}", self.emac.rxdladdr());
            // for i in 0..8 {
            //     trace!("{}: {:p} - 0x{:?}", i, &rd.0[i], rd.0[i]);
            // }                  

            // trace!("{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x} > {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x} ({:02x}{:02x}) length {}",
            //     rb[6], rb[7], rb[8], rb[9], rb[10], rb[11], 
            //     rb[0], rb[1], rb[2], rb[3], rb[4], rb[5],
            //     rb[12], rb[13],
            //     len,
            // );
            
            for i in 0..len {
                //trace!("{}: {:02x}", i, rb[i]);
                buf[i] = rb[i];
            }
            rd.set_owned(true);
            ptr::write_volatile(&mut RX_DESCS[rx_index] as *mut RxDesc, rd);
            rx_index = (rx_index + 1) % RX_DESC_COUNT;
            self.rx_index.set(rx_index);

            // for i in 0..RX_DESC_COUNT {
            //     let mut rd = ptr::read_volatile(&mut RX_DESCS[i] as *mut RxDesc);
            //     trace!("rd[{}]: owned? {}", i, rd.owned());
            // }

            len
        }
    }
    pub fn send(&self, buf: &[u8]) -> usize {
        unsafe {
            let mut emac = self.emac;
            let mut tx_index = self.tx_index.get();
            let mut td = &mut TX_DESCS[tx_index];
            if td.owned() {
                return 0;
            }
            //trace!("sending {}", buf.len());
            td.set_buffer_1_len(buf.len());
            let mut tb = td.buffer_1_mut();
            for i in 0..buf.len() {
                tb[i] = buf[i];
            }            

            td.set_first(true);
            td.set_last(true);
            td.set_owned(true);
            td.set_interrupt(true);            

            tx_index = (tx_index + 1) % TX_DESC_COUNT;
            self.tx_index.set(tx_index);
            //for _ in 0..100 { asm!("nop")}
            // trace!("STATUS: {:?} RIS: {:?} DMARIS: {:?}",
            //     self.emac.status(), 
            //     self.emac.ris(),
            //     self.emac.dmaris(),
            // );
            emac.set_dmaris(Dmaris(0).set_tu(1).set_tps(1).set_ti(1));
            
            self.poll_tx();
            //while emac.dmaris().ti() == 0 {}
            
            // trace!("txdladdr: {:?}", self.emac.txdladdr());
            // for i in 0..8 {
            //     trace!("{}: {:p} - 0x{:?}", i, &td.0[i], td.0[i]);
            // }
            // trace!("Waiting for interrupt");
            
            // let mut c = 0;
            // loop {
            //     if c == 10_000_000 {
            //         c = 0;
            //         trace!("STATUS: {:?} RIS: {:?} DMARIS: {:?}",
            //             self.emac.status(), 
            //             self.emac.ris(),
            //             self.emac.dmaris(),
            //         );
            //     }
            //     if self.emac.dmaris().ts() == 0x6 {
            //         trace!("underflow");
            //         break;
            //     }
            //     if self.emac.dmaris().tps() != 0 {
            //         trace!("stopped");
            //         break;
            //     }
            //     if self.emac.dmaris().ti() != 0 {
            //         break;
            //     }
            // }
            
        }
        buf.len()
    }

}


pub const TX_DESC_EMPTY: TxDesc = TxDesc([0u32; 8]);
pub const RX_DESC_EMPTY: RxDesc = RxDesc([0u32; 8]);

#[derive(Default, Clone, Copy)]
pub struct TxDesc([u32; 8]);
#[derive(Default, Clone, Copy)]
pub struct RxDesc([u32; 8]);

impl TxDesc {
    pub fn owned(&self) -> bool {
        self.periph[0].get_bit(31)
    }
    pub fn set_owned(&mut self, value: bool) {
        self.periph[0].set_bit(31, value);
    }

    pub fn set_interrupt(&mut self, value: bool) {
        self.periph[0].set_bit(30, value);
    }    

    pub fn is_last(&self) -> bool {
        self.periph[0].get_bit(29)
    }    

    pub fn set_last(&mut self, value: bool) {
        self.periph[0].set_bit(29, value);
    }

    pub fn is_first(&self) -> bool {
        self.periph[0].get_bit(28)
    }    

    pub fn set_first(&mut self, value: bool) {
        self.periph[0].set_bit(28, value);
    }    

    pub fn set_disable_crc(&mut self, value: bool) {
        self.periph[0].set_bit(27, value);
    }    

    pub fn is_end_of_ring(&self) -> bool {
        self.periph[0].get_bit(21)
    }

    pub fn set_end_of_ring(&mut self, value: bool) {
        self.periph[0].set_bit(21, value);
    }

    pub fn is_chained(&self) -> bool {
        self.periph[0].get_bit(20)
    }

    pub fn set_chained(&mut self, value: bool) {
        self.periph[0].set_bit(20, value);
    }

    pub fn buffer_1(&self) -> &'static [u8] {
        let len = self.periph[1].get_range(0..13) as usize;
        let ptr = self.periph[2] as *const u8;        
        unsafe { ::core::slice::from_raw_parts(ptr, len) }
    }

    pub fn buffer_1_mut(&self) -> &'static mut [u8] {
        let len = self.periph[1].get_range(0..13) as usize;
        let ptr = self.periph[2] as *mut  u8;        
        unsafe { ::core::slice::from_raw_parts_mut(ptr, len) }
    }    

    pub fn set_buffer_1(&mut self, buf: &'static mut [u8]) {
        assert!(buf.len() & !0x1fff == 0);        
        self.periph[1].set_range(0..13, buf.len() as u32);
        self.periph[2] = buf.as_mut_ptr() as u32;        
    }

    pub fn set_buffer_1_len(&mut self, len: usize) {
        assert!(len & !0x1fff == 0);        
        self.periph[1].set_range(0..13, len as u32);
    }

    pub fn buffer_2(&self) -> &'static [u8] {
        let len = self.periph[1].get_range(16..29) as usize;
        let ptr = self.periph[3] as *const u8;        
        unsafe { ::core::slice::from_raw_parts(ptr, len) }        
    }

    pub fn buffer_2_mut(&self) -> &'static mut [u8] {
        let len = self.periph[1].get_range(16..29) as usize;
        let ptr = self.periph[3] as *mut u8;        
        unsafe { ::core::slice::from_raw_parts_mut(ptr, len) }        
    }

    pub fn set_buffer_2(&mut self, buf: &'static mut [u8]) {
        assert!(buf.len() & !0x1fff == 0);        
        self.periph[1].set_range(16..29, buf.len() as u32);
        self.periph[3] = buf.as_mut_ptr() as u32;        
    }

    pub fn set_buffer_2_len(&mut self, len: usize) {
        assert!(len & !0x1fff == 0);
        self.periph[1].set_range(16..29, len as u32);
    }


    pub fn next(&self) -> &TxDesc {
        unsafe { &*(self.periph[3] as *const TxDesc) }
    }

    pub fn set_next(&mut self, next: &TxDesc) {
        self.periph[3] = next as *const TxDesc as u32;
    }
}

impl RxDesc {

    pub fn owned(&self) -> bool {
        self.periph[0].get_bit(31)
    }

    pub fn set_owned(&mut self, value: bool) {
        self.periph[0].set_bit(31, value);
    }

    pub fn frame_length(&self) -> usize {
        self.periph[0].get_range(16..30) as usize
    }

    pub fn set_frame_length(&mut self, value: usize) {
        self.periph[0].set_range(16..30, value as u32);
    }

    pub fn is_first(&self) -> bool {
        self.periph[0].get_bit(9)
    }

    pub fn is_last(&self) -> bool {
        self.periph[0].get_bit(10)
    }

    pub fn is_chained(&self) -> bool {
        self.periph[1].get_bit(14)
    }

    pub fn set_chained(&mut self, value: bool) {
        self.periph[1].set_bit(14, value);
    }

    pub fn is_end_of_ring(&self) -> bool {
        self.periph[1].get_bit(15)
    }

    pub fn set_end_of_ring(&mut self, value: bool) {
        self.periph[1].set_bit(15, value);
    }

    pub fn buffer_1(&self) -> &'static [u8] {
        let len = self.periph[1].get_range(0..13) as usize;
        let ptr = self.periph[2] as *const u8;        
        unsafe { ::core::slice::from_raw_parts(ptr, len) }
    }

    pub fn buffer_1_mut(&self) -> &'static mut [u8] {
        let len = self.periph[1].get_range(0..13) as usize;
        let ptr = self.periph[2] as *mut  u8;        
        unsafe { ::core::slice::from_raw_parts_mut(ptr, len) }
    }    

    pub fn set_buffer_1(&mut self, buf: &'static mut [u8]) {
        assert!(buf.len() & !0x1fff == 0);        
        self.periph[1].set_range(0..13, buf.len() as u32);
        self.periph[2] = buf.as_mut_ptr() as u32;        
    }

    pub fn buffer_2(&self) -> &'static [u8] {
        let len = self.periph[1].get_range(16..29) as usize;
        let ptr = self.periph[3] as *const u8;        
        unsafe { ::core::slice::from_raw_parts(ptr, len) }        
    }

    pub fn buffer_2_mut(&self) -> &'static mut [u8] {
        let len = self.periph[1].get_range(16..29) as usize;
        let ptr = self.periph[3] as *mut u8;        
        unsafe { ::core::slice::from_raw_parts_mut(ptr, len) }        
    }

    pub fn set_buffer_2(&mut self, buf: &'static mut [u8]) {
        assert!(buf.len() & !0x1fff == 0);        
        self.periph[1].set_range(16..29, buf.len() as u32);
        self.periph[3] = buf.as_mut_ptr() as u32;        
    }

    pub fn next(&self) -> &RxDesc {
        unsafe { &*(self.periph[3] as *const RxDesc) }
    }

    pub fn set_next(&mut self, next: &RxDesc) {
        self.periph[3] = next as *const RxDesc as u32;
    }    
}

impl fmt::Debug for TxDesc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "TxDesc @ {:p}\n", self));
        try!(write!(f, "  0: {:08x}\n", self.periph[0]));
        try!(write!(f, "  1: {:08x}\n", self.periph[1]));
        try!(write!(f, "  2: {:08x}\n", self.periph[2]));
        try!(write!(f, "  3: {:08x}\n", self.periph[3]));
        try!(write!(f, "  4: {:08x}\n", self.periph[4]));
        try!(write!(f, "  5: {:08x}\n", self.periph[5]));
        try!(write!(f, "  6: {:08x}\n", self.periph[6]));
        try!(write!(f, "  7: {:08x}\n", self.periph[7]));
        Ok(())
    }
}

impl fmt::Debug for RxDesc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "RxDesc @ {:p}\n", self));
        try!(write!(f, "  0: {:08x}\n", self.periph[0]));
        try!(write!(f, "  1: {:08x}\n", self.periph[1]));
        try!(write!(f, "  2: {:08x}\n", self.periph[2]));
        try!(write!(f, "  3: {:08x}\n", self.periph[3]));
        try!(write!(f, "  4: {:08x}\n", self.periph[4]));
        try!(write!(f, "  5: {:08x}\n", self.periph[5]));
        try!(write!(f, "  6: {:08x}\n", self.periph[6]));
        try!(write!(f, "  7: {:08x}\n", self.periph[7]));
        Ok(())
    }
}