use hal::rcc;
use hal::gpio;
use chip::gpio::*;
use chip::syscfg;
use chip::ethernet::{self, ETHERNET_MAC};
use chip::ethernet_dma::{self, ETHERNET_DMA};

// Pins used for Ethernet
// PHY: LAN8720

// PB11 - TXEN            - ETH_RMII_TX_EN
// PB12 - TXD0            - ETH_RMII_TXD0
// PB13 - TXD1            - ETH_RMII_TXD1
// PC4  - RXD0/MODE0      - ETH_RMII_RXD0
// PC5  - RXD1/MODE1      - ETH_RMII_RXD1
// PA7  - CRS_DV/MODE2    - ETH_RMII_CRS_DV
// PA2  - MDIO            - ETH_MDIO
// PC1  - MDC             - ETH_MDC
// PA1  - NINT/REFCLKO    - ETH_RMII_REF_CLK
// PE2  - NRST            - ETH_MII_TXD3

pub const ENET_0: Enet = Enet::Enet0;
pub const PHY: u8 = 0;

const TX_DESC_SIZE: usize = 8;
const RX_DESC_SIZE: usize = 8;
const TX_BUFFER_SIZE: usize = 1536;
const RX_BUFFER_SIZE: usize = 1536;
const RX_DESC_COUNT: usize = 8;
const TX_DESC_COUNT: usize = 8;

type TxDesc = [u32; TX_DESC_SIZE];
type RxDesc = [u32; RX_DESC_SIZE];
type TxDescs = [TxDesc; TX_DESC_COUNT];
type RxDescs = [RxDesc; RX_DESC_COUNT];
type TxBuf = [u8; TX_BUFFER_SIZE];
type RxBuf = [u8; RX_BUFFER_SIZE];
type TxBufs = [TxBuf; TX_DESC_COUNT];
type RxBufs = [TxBuf; RX_DESC_COUNT];

pub static mut TX_DESCS: TxDescs = [[0u32; TX_DESC_SIZE]; TX_DESC_COUNT];
pub static mut RX_DESCS: RxDescs = [[0u32; RX_DESC_SIZE]; RX_DESC_COUNT];
pub static mut TX_BUFS: TxBufs = [[0u8; TX_BUFFER_SIZE]; TX_DESC_COUNT];
pub static mut RX_BUFS: RxBufs = [[0u8; RX_BUFFER_SIZE]; RX_DESC_COUNT];

pub static mut TX_INDEX: usize = 0;
pub static mut RX_INDEX: usize = 0;

pub enum Enet {
    Enet0,
}

impl Enet {

    pub fn init(&self) {
        trace!("ETH: init()");
    
        self.ports_init();
        self.clocks_init();        
        self.phy_init();
        self.dma_init();
        self.mac_init();
        self.desc_init();
        self.enable();

        trace!("ETH: init() complete");
    }

    fn clocks_init(&self) {
        trace!("ETH: clocks_init()");

        // Disable Ethernet Clocks
        rcc::set_ethmac_enabled(false);
        rcc::set_ethmactx_enabled(false);
        rcc::set_ethmacrx_enabled(false);
        rcc::set_ethmacptp_enabled(false);

        // Enable SYSCFG Clock
        rcc::set_syscfg_enabled(true);

        // Set RMII Mode
        unsafe { 
            syscfg::SYSCFG.with_pmc(|r| r.set_mii_rmii_sel(1)); 
        }
        
        // Enable Ethernet Clocks
        rcc::set_ethmac_enabled(true);
        rcc::set_ethmactx_enabled(true);
        rcc::set_ethmacrx_enabled(true);
        rcc::set_ethmacptp_enabled(true);        

        // Enable DMA Clocks
        rcc::set_dma1_enabled(true);
        rcc::set_dma2_enabled(true);

        // Ethernet Reset
        unsafe {       
            ETHERNET_DMA.set_dmabmr(ethernet_dma::Dmabmr(0).set_sr(1));
            while ETHERNET_DMA.dmabmr().sr() != 0 {}
        }
    }    

    fn ports_init(&self) {
        trace!("ETH: ports_init()");

        let port_pins = [
            (GPIOA, 1), (GPIOA, 2), (GPIOA, 7),
            (GPIOB, 11), (GPIOB, 12), (GPIOB, 13),
            (GPIOC, 1), (GPIOC, 4), (GPIOC, 5),
        ];

        // Enable GPIO Clocks ABCE
        rcc::set_gpio_enabled(GPIOA, true);
        rcc::set_gpio_enabled(GPIOB, true);
        rcc::set_gpio_enabled(GPIOC, true);

        for &(port, pin) in port_pins.iter() {
             // GPIO Mode = AF
            gpio::set_mode(port, pin, 0b10);
            // GPIO AF = AF_ETH
            gpio::set_af(port, pin, 11);
            // GPIO PullUp / Pulldown Disabled
            gpio::set_pupd(port, pin, 0b00);
            // GPIO Speed = High
            gpio::set_ospeed(port, pin, 0b10);
        }
    }

    fn phy_init(&self) {
        trace!("ETH: phy_init()");
        // PHY assert NSRT on PE2

        if true {
            // Enable GPIOE
            rcc::set_gpio_enabled(GPIOE, true);

            // GPIO PE2 active low
            gpio::set_mode(GPIOE, 2, 0b01);

            // NRST Low (turn off PHY)
            gpio::set_odr(GPIOE, 2, false);

            // NRST High (turn on PHY)
            gpio::set_odr(GPIOE, 2, true);
        }

        unsafe {
            // Setup SMI Clock (168mhz system clock)    
            ETHERNET_MAC.set_macmiiar(ethernet::Macmiiar(0).set_cr(0b100));
            while ETHERNET_MAC.macmiiar().mb() != 0 {}            
        }

        // PHY Reset
        self.phy_write(PHY, 0, 1 << 15);
        while (self.phy_read(PHY, 0) & 1 << 15) != 0 {} 

        // Enable Auto-Negotiate
        self.phy_write(PHY, 0, 1 << 12);
        while (self.phy_read(PHY, 1) & 1 << 5) == 0 {}
        
    }

    fn phy_read(&self, phy: u8, reg: u8) -> u16 {
        unsafe {
            ETHERNET_MAC.set_macmiiar(ethernet::Macmiiar(0)
                .set_cr(0b100 as u32)
                .set_pa(phy as u32)
                .set_mr(reg as u32)
                .set_mw(0)
                .set_mb(1)
            );
            while ETHERNET_MAC.macmiiar().mb() != 0 {}
            ETHERNET_MAC.macmiidr().td() as u16
        }
    }

    fn phy_write(&self, phy: u8, reg: u8, value: u16) {
        unsafe {            
            ETHERNET_MAC.set_macmiidr(ethernet::Macmiidr(0).set_td(value as u32));
            ETHERNET_MAC.set_macmiiar(ethernet::Macmiiar(0)
                .set_cr(0b100 as u32)
                .set_pa(phy as u32)
                .set_mr(reg as u32)
                .set_mw(1)
                .set_mb(1)
            );
            while ETHERNET_MAC.macmiiar().mb() != 0 {}
        }
    }

    pub fn phy_dump(&self) {    
        for x in 0..7 {
            trace!("{:02}: {:04x}  ", x, self.phy_read(PHY, x));
        }
        trace!("\n");
        for x in 16..19 {
            trace!("{:02}: {:04x}  ", x, self.phy_read(PHY, x));
        }
        trace!("\n");
        for x in 26..32 {
            trace!("{:02}: {:04x}  ", x, self.phy_read(PHY, x));
        }
        trace!("\n");
    }

    fn dma_init(&self) {
        unsafe {
            trace!("ETH: dma_init()");
            // Disable checksum error frames, Receive Storage and Forward, Disable Flushing Received Frames,
            // Transmit Storage and Forward, Operate on Second Frame
            // DMAOMR = DTCEFD | RSF | DFRF | TSF | FEF | OSF
            ETHERNET_DMA.set_dmaomr(ethernet_dma::Dmaomr(0)
                .set_dtcefd(1)
                .set_rsf(1)
                .set_dfrf(1)
                .set_tsf(1)
                .set_fef(1)
                .set_osf(1)
            );


            // Address Aligned Beats, Fixed Burst, RX Beats = 32, Programmable Burst Length = 32,
            // PM = 2:1, Use Separate PBL 
            // DMABMR = AAB | FB | 32 << RDP_SHIFT | 32 << PBL_SHIFT | PM_2_1 | USP
            ETHERNET_DMA.set_dmabmr(ethernet_dma::Dmabmr(0)
                .set_aab(1)
                .set_fb(1)
                .set_rdp(32)
                .set_pbl(32)
                .set_rtpr(0b01)
                .set_usp(1)
            );

            // Enable DMA interrupts
            // NISE (Normal Interrupts) | RIE (Receive Interrupt) | TIE (Transmit Interrupt)
            ETHERNET_DMA.set_dmaier(ethernet_dma::Dmaier(0)
                .set_nise(1)
                .set_rie(1)
                .set_tie(1)
            );                            
        }
    }

    fn mac_init(&self) {
        unsafe {
            trace!("ETH: mac_init()");
            // Strip CRC for Type Frames, Fast Ethernet, Duplex Mode, Auto Pad / CRC Stripping, Retry Disable
            // MACCR = CSTF | FES | DM | APCS | RD        
            ETHERNET_MAC.set_maccr(ethernet::Maccr(0)
                //.set_cstf(1)
                .set_fes(1)
                .set_dm(1)
                .set_apcs(1)
                .set_rd(1)
            );

            // Receive All / Promiscuous Mode
            // MACFFR = RA | PM
            ETHERNET_MAC.set_macffr(ethernet::Macffr(0)
                //.set_ra(1)
                //.set_pm(1)
            );

            // Clear Hash Table High Register
            // MACHTHR = 0
            ETHERNET_MAC.set_machthr(ethernet::Machthr(0));

            // Clear Hash Table Low Register
            // MACHTLR = 0
            ETHERNET_MAC.set_machtlr(ethernet::Machtlr(0));
            
            // Set Pause Time to 0x100       
            // MACFCR = 0x100 << PT_SHIFT
            ETHERNET_MAC.set_macfcr(ethernet::Macfcr(0).set_pt(0x100));

            // Set VLAN tag register to 0
            // MACVLANTR = 0
            ETHERNET_MAC.set_macvlantr(ethernet::Macvlantr(0));
        }
    }

    fn desc_init(&self) {
        unsafe {
            for i in 0..TX_DESC_COUNT {
                let next_desc = if i < TX_DESC_COUNT-1 {
                    &TX_DESCS[i+1] as *const TxDesc as u32
                } else {
                    &TX_DESCS[0] as *const TxDesc as u32
                };
                let mut td = &mut TX_DESCS[i];
                td[0] = 1 << 20;
                td[1] = 0;
                td[2] = &TX_BUFS[i] as *const u8 as u32;
                td[3] = next_desc;
            }

            for i in 0..RX_DESC_COUNT {
                let next_desc = if i < RX_DESC_COUNT - 1 {
                    &RX_DESCS[i+1] as *const RxDesc as u32
                } else {
                    &RX_DESCS[0] as *const RxDesc as u32
                };
                let mut rd = &mut RX_DESCS[i];
                rd[0] = 1 << 31;
                rd[1] = 1 << 14 | (RX_BUFFER_SIZE as u32 & 0x1fff);;
                rd[2] = &RX_BUFS[i] as *const u8 as u32;
                rd[3] = next_desc;
            } 
            ETHERNET_DMA.set_dmatdlar(ethernet_dma::Dmatdlar(&TX_DESCS[0] as *const TxDesc as u32));
            ETHERNET_DMA.set_dmardlar(ethernet_dma::Dmardlar(&RX_DESCS[0] as *const RxDesc as u32));
        }
    }

    fn enable(&self) {
        unsafe {
            // Enable Transmit
            ETHERNET_MAC.with_maccr(|r| r.set_te(1));
            // Flush Transmit FIFO
            ETHERNET_DMA.with_dmaomr(|r| r.set_ftf(1));
            // Enable Receive
            ETHERNET_MAC.with_maccr(|r| r.set_re(1));      

            // DMA Start Transmit
            ETHERNET_DMA.with_dmaomr(|r| r.set_st(1));
            // DMA Start Receive        
            ETHERNET_DMA.with_dmaomr(|r| r.set_sr(1));
        }
    }

    pub fn recv_ready(&self) -> bool {
        unsafe {
            let rd = &RX_DESCS[RX_INDEX];
            rd[0] & 1 << 31 == 0
        }
    }

    pub fn recv_read(&self) -> (&'static RxBuf, usize) {
        unsafe {
            let rd = &RX_DESCS[RX_INDEX];
            let rb = &RX_BUFS[RX_INDEX];
            let len: usize = (rd[0] as usize >> 16) & 0x1fff;
            (rb, len)
        }
    }

    pub fn recv_done(&self) {
        unsafe {            
            let rd = &mut RX_DESCS[RX_INDEX];
            rd[0] |= 1 << 31;
            // if RBUS
            if ETHERNET_DMA.dmasr().rbus() != 0 {
                ETHERNET_DMA.set_dmasr(ethernet_dma::Dmasr(0).set_rbus(1));
                ETHERNET_DMA.set_dmarpdr(ethernet_dma::Dmarpdr(1));
            }  
            RX_INDEX = (RX_INDEX + 1) % RX_DESC_COUNT;
        }      
    }

    pub fn send_ready(&self) -> bool {
        true
    }

    pub fn send_read(&self) -> &'static mut TxBuf {
        unsafe {
            &mut TX_BUFS[TX_INDEX]
        }    
    }

    pub fn send_done(&self, len: usize) {
        unsafe {
            let td = &mut TX_DESCS[TX_INDEX];

            // Set length in descriptor
            td[1] = (len & 0x1fff) as u32;

            // OWN | IC | LS | FS
            td[0] |= 1 << 31 | 1 << 30 | 1 << 29 | 1 << 28;
            // if TBUS
            if ETHERNET_DMA.dmasr().tbus() != 0 {
                ETHERNET_DMA.set_dmasr(ethernet_dma::Dmasr(0).set_tbus(1));
                ETHERNET_DMA.set_dmatpdr(ethernet_dma::Dmatpdr(1));
            }
            while td[0] & 1 << 31 != 0 {}
            TX_INDEX = (TX_INDEX + 1) % TX_DESC_COUNT;
        }
    }
    
}