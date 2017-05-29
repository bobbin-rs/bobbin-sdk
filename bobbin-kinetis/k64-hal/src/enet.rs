pub use ::chip::enet::*;
use ::chip::mpu::*;
use ::chip::mcg::MCG;
use ::driver::port::*;
use ::hal::sim;

use core::ptr;
use core::slice;
use core::cell::Cell;

pub const RX_COUNT: usize = 4;
pub const RX_BUFFER_SIZE: usize = 1520;
pub const TX_COUNT: usize = 4;
pub const TX_BUFFER_SIZE: usize = 1520;

#[repr(C)]
pub struct RxDesc {
    length: u16,
    control: u16,
    buffer: *mut u8,
}

#[repr(C)]
pub struct TxDesc {
    length: u16,
    control: u16,
    buffer: *mut u8,
}

pub type RxBuf = [u8; RX_BUFFER_SIZE];
pub type TxBuf = [u8; TX_BUFFER_SIZE];

static mut RX_DESCS: [RxDesc; RX_COUNT] = [
    RxDesc{length: 0, control: 0, buffer: 0 as *mut u8},
    RxDesc{length: 0, control: 0, buffer: 0 as *mut u8},
    RxDesc{length: 0, control: 0, buffer: 0 as *mut u8},
    RxDesc{length: 0, control: 0, buffer: 0 as *mut u8},
];
static mut TX_DESCS: [TxDesc; TX_COUNT] = [
    TxDesc{length: 0, control: 0, buffer: 0 as *mut u8},
    TxDesc{length: 0, control: 0, buffer: 0 as *mut u8},
    TxDesc{length: 0, control: 0, buffer: 0 as *mut u8},
    TxDesc{length: 0, control: 0, buffer: 0 as *mut u8},
];

static mut RX_BUFS: [RxBuf; RX_COUNT] = [
    [0u8; RX_BUFFER_SIZE],
    [0u8; RX_BUFFER_SIZE],
    [0u8; RX_BUFFER_SIZE],
    [0u8; RX_BUFFER_SIZE],
];

static mut TX_BUFS: [TxBuf; TX_COUNT] = [
    [0u8; TX_BUFFER_SIZE],
    [0u8; TX_BUFFER_SIZE],
    [0u8; TX_BUFFER_SIZE],
    [0u8; TX_BUFFER_SIZE],
];

const MAC_ADDR: [u8; 6] = [0x02, 0x01, 0x02, 0x03, 0x04, 0x05];

pub struct EnetDevice {
    enet: Enet,
    _rmii_mdc: Pin<AltFn>,
    _rmii_mdio: Pin<AltFn>,
    _rmii_crs_dv: Pin<AltFn>,
    _rmii_rxd0: Pin<AltFn>,
    _rmii_rxd1: Pin<AltFn>,
    _rmii_rxer: Pin<AltFn>,
    _rmii_txd0: Pin<AltFn>,
    _rmii_txd1: Pin<AltFn>,
    _rmii_txen: Pin<AltFn>,    
    rx_number: Cell<usize>,
    tx_number: Cell<usize>,
}

pub fn device(enet: Enet,
    rmii_mdc: Pin<AltFn>,
    rmii_mdio: Pin<AltFn>,
    rmii_crs_dv: Pin<AltFn>,
    rmii_rxd0: Pin<AltFn>,
    rmii_rxd1: Pin<AltFn>,
    rmii_rxer: Pin<AltFn>,
    rmii_txd0: Pin<AltFn>,
    rmii_txd1: Pin<AltFn>,
    rmii_txen: Pin<AltFn>,
    ) -> EnetDevice {
        EnetDevice {
            enet: enet,
            _rmii_mdc: rmii_mdc,
            _rmii_mdio: rmii_mdio,
            _rmii_crs_dv: rmii_crs_dv,
            _rmii_rxd0: rmii_rxd0,
            _rmii_rxd1: rmii_rxd1,
            _rmii_rxer: rmii_rxer,
            _rmii_txd0: rmii_txd0,
            _rmii_txd1: rmii_txd1,
            _rmii_txen: rmii_txen,
            rx_number: Cell::new(0),
            tx_number: Cell::new(0),
        }
}

impl EnetDevice {
    pub fn enable(&self) {    
        unsafe {
            self.mpu_disable();
            self.enet_enable();
            self.phy_init(0);
            self.phy_configure(0);
            //self.phy_dump(0);
            self.enet_init();
            //self.dump();
        }
    }

    pub fn set_loopback(&self, loopback: bool) {
        let loopback = if loopback { 1 } else { 0 };
        let mut enet = self.enet;
        unsafe {
            enet.set_rcr(Rcr(0)
                //.set_prom(1)
                .set_loop(loopback)        
            );
        }
    }

    unsafe fn enet_enable(&self) {
        sim::set_enet_enabled(self.enet, true);
    }

    unsafe fn mpu_disable(&self) {
        MPU.with_cesr(|r| r.set_vld(0));
    }

    unsafe fn enet_init(&self) {
        let mut enet = self.enet;
        enet.with_ecr(|r| r.set_reset(1));
        self.enet_set_tx_descs(&mut TX_DESCS, &mut TX_BUFS);
        self.enet_set_rx_descs(&mut RX_DESCS, &mut RX_BUFS);
        self.enet_init_mac();
    }

    unsafe fn enet_init_mac(&self) {
        let mut enet = self.enet;
        enet.set_rcr(Rcr(0)
            .set_mii_mode(1)
            .set_rmii_mode(1)
            .set_crcfwd(1)
            .set_max_fl(1518)
            .set_prom(1)
        );
        enet.set_tcr(Tcr(0)
            .set_fden(1)
        );
        enet.set_tacc(Tacc(0));
        enet.set_racc(Racc(0));
        enet.set_tfwr(Tfwr(0).set_strfwd(1));
        enet.set_rsfl(Rsfl(0));
        enet.set_tdsr(Tdsr(0).set_x_des_start((&TX_DESCS[0] as *const TxDesc as u32) >> 3));
        enet.set_rdsr(Rdsr(0).set_r_des_start((&RX_DESCS[0] as *const RxDesc as u32) >> 3));
        enet.set_mrbr(Mrbr(0).set_r_buf_size(RX_BUFFER_SIZE as u32 >> 4));
        self.enet_set_macaddr(&MAC_ADDR);

        if enet.mscr().0 & 0x7e != 0 {
            enet.set_mscr(Mscr(0).set_mii_speed(0x18).set_holdtime(1));
        }

        enet.with_ecr(|r| r.set_etheren(1).set_dbswp(1));
        enet.set_rdar(Rdar(0).set_rdar(1));                        
    }

    unsafe fn enet_set_macaddr(&self, addr: &[u8; 6]) {
        let mut enet = self.enet;
        let addr_lo: u32 = 
            (addr[0] as u32) << 24 |
            (addr[1] as u32) << 16 |
            (addr[2] as u32) << 8 |
            (addr[3] as u32) << 0;
        enet.set_palr(Palr(0).set_paddr1(addr_lo));
        let addr_hi: u32 = (addr[4] as u32) << 8 | (addr[5] as u32);
        enet.set_paur(Paur(0).set_paddr2(addr_hi));
    }

    unsafe fn enet_set_tx_descs(&self, tx_descs: &mut [TxDesc], tx_bufs: &mut [TxBuf]) {
        assert!(tx_descs.len() == tx_bufs.len());
        for i in 0..TX_COUNT {
            tx_descs[i].buffer = &mut tx_bufs[i][0] as *mut u8;
            tx_descs[i].length = 0;
            tx_descs[i].control = 1 << 10;
            if i == TX_COUNT -1 {
                tx_descs[i].control |= 1 << 13;
            }
        }
    }

    unsafe fn enet_set_rx_descs(&self, rx_descs: &mut [RxDesc], rx_bufs: &mut [RxBuf]) {
        assert!(rx_descs.len() == rx_bufs.len());
        for i in 0..RX_COUNT {
            rx_descs[i].buffer = &mut rx_bufs[i][0] as *mut u8;
            rx_descs[i].length = 0;
            rx_descs[i].control = 1 << 15;
            if i == RX_COUNT -1 {
                rx_descs[i].control |= 1 << 13;
            }
        }
    }

    unsafe fn phy_init(&self, phy: u32) {    
        let mut enet = self.enet;
        enet.set_mscr(Mscr(0).set_mii_speed(0x18).set_holdtime(1));
        self.phy_write(phy, 0, 1 << 15);
        while (self.phy_read(phy, 0) & 1 << 15) != 0 {}    
    }

    unsafe fn phy_configure(&self, phy: u32) {        
        // Set Auto-Negotiation Parameters
        self.phy_write(phy, 0x4, 1 << 8 | 1 << 7 | 1 << 6 | 1 << 5 | 1);
        // Trigger Auto-Negotiation
        self.phy_write(phy, 0x0, 1 << 12 | 1 << 9);
        // Wait until Auto-Negotiation Complete
        while (self.phy_read(phy, 0x1) & 1 << 5) == 0 {}
    }

    unsafe fn phy_write(&self, phy: u32, reg: u32, data: u32) {
        let mut enet = self.enet;    
        // Clear MII Interrupt
        enet.set_eir(Eir(0).set_mii(1));
        // Set Transfer
        enet.set_mmfr(Mmfr(0)
            .set_st(0b01)
            .set_op(0b01)
            .set_pa(phy as u32)
            .set_ra(reg as u32)                
            .set_ta(0b10)
            .set_data(data as u32)
        );
        // Wait until complete
        while enet.eir().mii() == 0 {}
        enet.set_eir(Eir(0).set_mii(1));
    }

    unsafe fn phy_read(&self, phy: u32, reg: u32) -> u16 {
        let mut enet = self.enet;
        // Clear MII Interrupt
        enet.set_eir(Eir(0).set_mii(1));
        // Set Transfer Type
        enet.set_mmfr(Mmfr(0)
            .set_st(0b01)
            .set_op(0b10)
            .set_pa(phy as u32)
            .set_ra(reg as u32)                
            .set_ta(0b10)
        );
        // Wait until complete
        while enet.eir().mii() == 0 {}
        enet.set_eir(Eir(0).set_mii(1));
        // Read Data
        enet.mmfr().data() as u16
    }

    #[allow(dead_code)]
    unsafe fn phy_dump(&self, phy: u32) {
        for i in [0u32, 1, 2, 3, 4, 5, 0x1e, 0x1f].iter() {
            trace!("P{:02x}: {:016b}", i, self.phy_read(phy, *i));
        }
    }

    #[allow(dead_code)]
    unsafe fn dump(&self) {
        trace!("C1:    0x{:02x}", MCG.c1().0);
        trace!("C2:    0x{:02x}", MCG.c2().0);
        trace!("C3:    0x{:02x}", MCG.c3().0);
        trace!("C4:    0x{:02x}", MCG.c4().0);
        trace!("C5:    0x{:02x}", MCG.c5().0);
        trace!("C6:    0x{:02x}", MCG.c6().0);
        trace!("C7:    0x{:02x}", MCG.c7().0);
        trace!("C8:    0x{:02x}", MCG.c8().0);
        trace!("S:     0x{:02x}", MCG.s().0);
        trace!("SC:    0x{:02x}", MCG.sc().0);

        trace!("ECR:   0x{:08x}", ENET.ecr().0);
        trace!("RCR:   0x{:08x}", ENET.rcr().0);
        trace!("TCR:   0x{:08x}", ENET.tcr().0);
        trace!("PALR:  0x{:08x}", ENET.palr().0);
        trace!("PAUR:  0x{:08x}", ENET.paur().0);
        trace!("RDSR:  0x{:08x}", ENET.rdsr().0);
        trace!("TDSR:  0x{:08x}", ENET.tdsr().0);
        trace!("MRBR:  0x{:08x}", ENET.mrbr().0);
        trace!("BDLEN:  0x{:04x}", ptr::read_volatile(&RX_DESCS[0].length as *const u16));
        trace!("BDCTR:  0x{:04x}", ptr::read_volatile(&RX_DESCS[0].control as *const u16));        
    }

    pub unsafe fn run(&self) {        
        let mut rx_number = 0;        
        loop {
            let mut rd = &mut RX_DESCS[rx_number];
            let mut control = ptr::read_volatile(&rd.control as *const u16);
        
            if (control & 1 << 15) == 0 {
                let len = ptr::read_volatile(&rd.length as *const u16);
                let b = slice::from_raw_parts(rd.buffer, len as usize);                
                trace!("{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x} > {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x} ({:02x}{:02x}) length {}",
                            b[6], b[7], b[8], b[9], b[10], b[11],
                            b[0], b[1], b[2], b[3], b[4], b[5],
                            b[12], b[13],
                            b.len() - 18,
                        );                
                control &= 1 << 13;
                control |= 1 << 15;
                ptr::write_volatile(&mut rd.control as *mut u16, control);
                if control & 1 << 13 != 0 {
                    rx_number = 0
                } else {
                    rx_number += 1;
                }
                ENET.set_rdar(Rdar(0).set_rdar(1));
            }
        }        
    }

    pub fn recv(&self, buf: &mut [u8]) -> usize {
        let mut enet = self.enet;
        unsafe {
            let mut rx_number = self.rx_number.get();            
            enet.with_eimr(|r| r.set_rxf(1));            
            enet.set_rdar(Rdar(0).set_rdar(1));                        
            if enet.eir().rxf() == 0 { return 0 }

            //trace!("eir: {:?}", self.enet.eir());
            enet.with_eimr(|r| r.set_rxb(1).set_rxf(1));

            let mut rd = &mut RX_DESCS[rx_number];
            let mut control = ptr::read_volatile(&rd.control as *const u16);            
            if (control & 1 << 15) == 0 {
                let len = ptr::read_volatile(&rd.length as *const u16);
                let rbuf = slice::from_raw_parts(rd.buffer, len as usize);                
                for i in 0..rbuf.len() {
                    buf[i] = rbuf[i];
                }
                control &= 1 << 13;
                control |= 1 << 15;
                ptr::write_volatile(&mut rd.control as *mut u16, control);
                if control & 1 << 13 != 0 {
                    rx_number = 0
                } else {
                    rx_number += 1;
                }                    
                self.rx_number.set(rx_number);
                enet.set_eir(Eir(0).set_rxf(0));
                len as usize               
            } else {
                0
            }
        }
    }

    pub fn send(&self, buf: &[u8]) -> usize {
        let mut enet = self.enet;
        unsafe {
            let mut tx_number = self.tx_number.get();
            let mut td = &mut TX_DESCS[tx_number];
            ptr::write_volatile(&mut td.length as *mut u16, buf.len() as u16);
            let wbuf = slice::from_raw_parts_mut(td.buffer, buf.len());
            for i in 0..buf.len() {
                wbuf[i] = buf[i];
            }
            let mut control = ptr::read_volatile(&td.control as *const u16);            
            control |= 1 << 15;
            control |= 1 << 11;
            control |= 1 << 10;
            ptr::write_volatile(&mut td.control as *mut u16, control);
            if control & 1 << 13 != 0 {
                tx_number = 0
            } else {
                tx_number += 1;
            }
            self.tx_number.set(tx_number);
            
            enet.with_eimr(|r| r.set_txf(1));
            enet.set_eir(Eir(0).set_txf(1));            
            enet.set_tdar(Tdar(0).set_tdar(1));
            while enet.eir().txf() == 0 {}
            enet.set_eir(Eir(0).set_txf(1));
            enet.with_eimr(|r| r.set_txb(1).set_txf(1));
            return buf.len()
        }
    }
}

// pub unsafe fn init_pins() {
//     use ::driver::port;
//     const AF_ENET: u8 = 4;

//     //rmii_rxer.set_pull(port::Pull::PullUp);
//     let rmii_rxd1 = port::pin(port::PORTA, 12).into_altfn(AF_ENET);
//     let rmii_rxd0 = port::pin(port::PORTA, 13).into_altfn(AF_ENET);
//     let rmii_crs_dv = port::pin(port::PORTA, 14).into_altfn(AF_ENET);
//     let rmii_txen = port::pin(port::PORTA, 15).into_altfn(AF_ENET);
//     let rmii_txd0 = port::pin(port::PORTA, 16).into_altfn(AF_ENET);
//     let rmii_txd1 = port::pin(port::PORTA, 17).into_altfn(AF_ENET);    
//     //let _ = port::pin(port::PORTA, 28).into_altfn(AF_ENET);    
//     let rmii_rxer = port::pin(port::PORTA, 5).into_altfn(AF_ENET);

//     //let _ = port::pin(port::PORTA, 18).into_analog();

//     //let rmii_txer = PTA28
//     //let rmii_rxclk = port::pin(port::PORTA, 18).into_altfn(AF_ENET);
//     // Check - PTA28 / PTA11 (RXCLK)

//     let rmii_mdio = port::pin(port::PORTB, 0).into_altfn(AF_ENET);  
//     let rmii_mdc = port::pin(port::PORTB, 1).into_altfn(AF_ENET);    
    
//     rmii_mdio.set_pull(port::Pull::PullUp);
//     rmii_mdio.set_ode(true);

    
//     // const AF_ENET: u32 = 4;

//     // // RMII - PTA5 / PTA12 / PTA13 / PTA14 / PTA15 / PTA16 / PTA17 / PTA28
//     // let port_a = port::PORT_A;
//     // port_a.set_enabled(1);
//     // for p in [12, 13, 14, 15, 16, 17, 28, 5].iter() {
//     //     port_a.set_mux(*p, AF_ENET)
//     // }    

//     // // MDIO / MDC - PTB0 / PTB1
//     // let port_b = port::PORT_B;
//     // port_b.set_enabled(1);
//     // for p in [0, 1].iter() {
//     //     port_b.set_mux(*p, AF_ENET);
//     // }
//     // port_b.set_pull_enabled(0, 1);
//     // port_b.set_pull_select(0, 1);
//     // port_b.set_open_drain_enabled(0, 1);


//     // // // IEEE588 TMR - PTC16 / PTC17 / PTC18 / PTC19
//     // let port_c = port::PORT_C;
//     // port_c.set_enabled(1);
//     // for p in [16, 17, 18, 19].iter() {
//     //     port_c.set_mux(*p, AF_ENET)
//     // }
        
// }    