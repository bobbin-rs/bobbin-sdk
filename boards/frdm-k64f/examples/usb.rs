#![no_std]
#![no_main]
#![feature(repr_align, attr_literals)]

#[macro_use]
extern crate frdm_k64f as board;

use board::hal::sim::*;
use board::hal::usb::*;
use board::chip::mpu::*;

// use board::hal::clock::*;
// use board::clock::CLK;

// #[derive(Debug, Default, Clone, Copy)]
// pub struct BufferDescriptor {
//     desc: u32,
//     addr: u32,
// }

pub const ENDPOINT_BUF_SIZE: usize = 64;

#[repr(align(512))]
pub struct BufferDescriptorTable([BufferDesc; 8]);

impl BufferDescriptorTable {
    pub fn index(&self, index: usize, tx: bool, odd: bool) -> usize {
        (index * 4) + if tx { 2 } else { 0 } + if odd { 1 } else { 0 }
    }

    pub fn bd(&self, index: usize, tx: bool, odd: bool) -> &BufferDesc {
        &self.0[self.index(index, tx, odd)]
    }


    pub fn bd_mut(&mut self, index: usize, tx: bool, odd: bool) -> &mut BufferDesc {
        &mut self.0[self.index(index, tx, odd)]
    }    
}


static mut EP_BUFFERS: [[u8; ENDPOINT_BUF_SIZE]; 4] = [[0u8; ENDPOINT_BUF_SIZE]; 4];
static mut BDT: BufferDescriptorTable = BufferDescriptorTable([ BufferDesc([0u8; 8]); 8]);


#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Running USB");
    board::delay(100);
    

    // DISABLE MPU    
    MPU.with_cesr(|r| r.set_vld(0));

    let usb = USB0;

    // Setup USB Clocks

    println!("Setting up USB Clocks");
    usb.sim_enable();

    println!("Resetting USB");
    usb.with_usbtrc0(|r| r.set_usbreset(1));
    while usb.usbtrc0().test_usbreset() {}

    println!("Setting Clock Source");

    SIM.with_sopt2(|r| r
        .set_pllfllsel(0b11)
        .set_usbsrc(1)
    );

    println!("Enabling IRC");
    usb.with_clk_recover_irc_en(|r| r.set_irc_en(1));

    println!("Enabling Clock Recovery");
    usb.with_clk_recover_ctrl(|r| r.set_clock_recover_en(1));

    assert_eq!(usb.idcomp().nid(), 0b111011);
    assert_eq!(usb.rev().rev(), 0b00110011);


    println!("Setting up driver");
    let irq = usb.irq_usb();
    let usb = UsbDriver::new(usb);
    usb.enable_irq(&irq);

    // println!("Resetting USB");
    // usb.reset();
    
    println!("Enabling usb");

    unsafe {
        usb.set_bdt(&mut BDT as *mut BufferDescriptorTable as u32);
    }
    // usb.dump_bdt();

    usb.enable();
    // usb.dump();
    println!("Done with Init");
    loop {
        board::delay(1000);
    }
}

use board::common::{Irq, Poll};
use board::cortexm::hal::nvic;
use board::cortexm::hal::scb::SCB;

use core::cell::Cell;
// use core::marker::PhantomData;

pub struct UsbDriver {
    usb: UsbPeriph,    
    addr: Cell<u8>,
}

unsafe impl Sync for UsbDriver {}
unsafe impl Send for UsbDriver {}

impl UsbDriver {
    pub fn new<U: Into<UsbPeriph>>(usb: U) -> Self {
        UsbDriver {
            usb: usb.into(),
            addr: Cell::new(0),
        }
    }

    pub fn addr(&self) -> u8 {
        self.addr.get()
    }

    pub fn set_addr(&self, value: u8) {
        self.addr.set(value)
    }

    pub fn enable_irq<I: Irq>(&self, irq: &I) {        
        nvic::set_enabled(irq.irq_num() as usize, false);
        SCB.set_irq_handler(irq.irq_num() as usize, None);
        SCB.set_irq_handler(irq.irq_num() as usize, Some(irq.wrap(self)));
        nvic::set_enabled(irq.irq_num() as usize, true);
    }

    pub fn reset(&self) {
        self.usb.with_usbtrc0(|r| r.set_usbreset(1));
        while self.usb.usbtrc0().test_usbreset() {}
    }

    pub fn dump(&self) {
        println!("STAT:       {:?}", self.usb.stat());
        println!("CTL:        {:?}", self.usb.ctl());
        println!("USBCTRL:    {:?}", self.usb.usbctrl());
        println!("OBSERVE:    {:?}", self.usb.observe());
        println!("CONTROL:    {:?}", self.usb.control());
        println!("USBTRC0:    {:?}", self.usb.usbtrc0());
        println!("RECOVER_CTRL:   {:?}", self.usb.clk_recover_ctrl());
        println!("RECOVER_IRC_EN: {:?}", self.usb.clk_recover_irc_en());
        
    }

    pub fn dump_istat(&self) {
        println!("ISTAT:    {:?}", self.usb.istat());
        println!("ERRSTAT:  {:?}", self.usb.errstat());
        println!("OTGISTAT: {:?}", self.usb.otgistat());
        println!("USBTRC0:  {:?}", self.usb.usbtrc0());
    }


    pub fn set_bdt(&self, bdt: u32) {
        println!("set_bdt: {:08x}", bdt);
        self.usb.set_bdtpage1(|_| Bdtpage1((bdt >> 8) as u8));
        self.usb.set_bdtpage2(|_| Bdtpage2((bdt >> 16) as u8));
        self.usb.set_bdtpage3(|_| Bdtpage3((bdt >> 24) as u8));
        // USB0->BDTPAGE1 = ((uint32_t) buf_desc_table) >> 8;  //bits 15-9
        // USB0->BDTPAGE2 = ((uint32_t) buf_desc_table) >> 16; //bits 23-16
        // USB0->BDTPAGE3 = ((uint32_t) buf_desc_table) >> 24; //bits 31-24
    }
    pub fn bdt_base(&self) -> u32 {
        (self.usb.bdtpage1().0 as u32) << 8 |
        (self.usb.bdtpage2().0 as u32) << 16 |
        (self.usb.bdtpage3().0 as u32) << 24
    }

    pub fn dump_bdt(&self) {
        println!("BDTPAGE1: {:?}", self.usb.bdtpage1());
        println!("BDTPAGE2: {:?}", self.usb.bdtpage2());
        println!("BDTPAGE3: {:?}", self.usb.bdtpage3());
        println!("BDT:      0x{:08x}", self.bdt_base());
    }

    pub fn enable(&self) {
        // USB Enable
        self.usb.with_ctl(|r| r.set_usbensofen(1));
        // Leave Suspend, Disable Weak Pulldowns
        self.usb.with_usbctrl(|r| r.set_susp(0).set_pde(0));
        // Enable USB Reset Interrupt
        self.usb.with_inten(|r| r.set_usbrsten(1));
        // Enable Resistor Pullup for USB Full Speed
        self.usb.with_control(|r| r.set_dppullupnonotg(1));
    }
}

impl Poll for UsbDriver {
    fn poll(&self) {
        let istat = self.usb.istat();
        // println!("{:?}", istat);
        // for i in 0..4 {
        //     println!("{}: {:?}", i, unsafe { BDT.0[i].bdesc().own() });
        // }        
        // println!("{:?} {:?} {:?}", istat, self.usb.stat(), self.usb.errstat());
        // println!("CTL   {:?}", self.usb.ctl());
        // println!("USBCTL   {:?}", self.usb.usbctrl());

        if istat.test_error() {
            println!("ERROR: {:?}", self.usb.errstat());
            self.usb.set_errstat(|_| Errstat(0xff));
            self.usb.set_istat(|r| r.set_error(1));
        }

        if istat.test_usbrst() {
            self.usb.with_ctl(|r| r.set_oddrst(1));
            // self.usb.with_ctl(|r| r.set_oddrst(1));

            unsafe {
                for i in 0..8 {
                    BDT.0[i].set_bdesc(|_| Bdesc(0));
                    BDT.0[i].set_baddr(|_| Baddr(0));
                }

                for ep in 0..2 {

                    let b_even = &EP_BUFFERS[(ep * 2)] as *const u8 as u32;
                    let b_odd = &EP_BUFFERS[(ep * 2) + 1] as *const u8 as u32;
                    // println!("{} {} {:02x} {:02x}", ep * 2, (ep * 2) + 1, b_even, b_odd);

                    BDT.bd_mut(ep, false, false)
                        .set_bdesc(|r| r.set_own(1).set_bc(ENDPOINT_BUF_SIZE).set_data01(0).set_dts(1));
                    BDT.bd_mut(ep, false, false)
                        .set_baddr(|r| r.set_addr(b_even));
                    // println!("{:?}", BDT.bd_mut(ep, false, false).baddr());
                    BDT.bd_mut(ep, false, true)
                        .set_bdesc(|r| r.set_own(1).set_bc(ENDPOINT_BUF_SIZE).set_data01(1).set_dts(1));
                    BDT.bd_mut(ep, false, true)
                        .set_baddr(|r| r.set_addr(b_odd));                        
                    // println!("e{:?}", BDT.bd_mut(ep, false, true).baddr());
                    self.usb.with_endpt(ep, |r| r.set_eprxen(1).set_eptxen(1).set_ephshk(1));
                    // println!("ENDPT[{}]: {:?}", ep, self.usb.endpt(ep));
                }
            }
            // Clear All Interrupts
            self.usb.set_errstat(|_| Errstat(0xff));
            self.usb.set_istat(|_| Istat(0xff));

            // Set address to zero
            self.usb.set_addr(|r| r.set_addr(0));

            // Enable interrupts
            self.usb.set_erren(|_| Erren(0xff));
            self.usb.with_inten(|r| r
                .set_usbrsten(1)
                .set_erroren(1)
                .set_softoken(1)
                .set_tokdneen(1)
                .set_sleepen(1)
                .set_stallen(1)
            );
            // unsafe {
            //     println!("BDT: {:p}", &BDT);
            //     for i in 0..8 {
            //         let bd = BDT.0[i];
            //         println!("BDT[{}] {:?} {:?}", i, bd.bdesc(), bd.baddr());
            //     }
            // }

            // for i in 0..2 {
            //     for (n, b) in buffers[i].iter().enumerate() {
            //         if n % 8 == 0 { print!("\n"); }
            //         print!("{:02x}", b);
            //     }
            //     println!("");
            // }            
            // println!("USBRST");
            println!("----");
        }
        if istat.test_softok() {
            self.usb.set_istat(|r| r.set_softok(1));
            // for i in 0..4 {
            //     let bd = unsafe { BDT.0[i].bdesc() };
            //     println!("{}: {:?}", i, bd);
            // }
            // println!("SOFTOK {:?} {:?}", self.usb.addr().addr(), self.usb.stat());
        }
        if istat.test_tokdne() {
            let stat = self.usb.stat();
            // println!("TOKDNE {:?}", stat);
            let ep = stat.endp().value();
            let tx = stat.test_tx();
            let odd = stat.test_odd();
            unsafe {
                let bd = BDT.bd_mut(ep as usize, tx, odd);
                println!("BD: {:?} {:?}", bd.bdesc(), bd.baddr());
                match bd.bdesc().tok_pid().value() {
                    0x1 => {
                        println!("TOK_OUT");
                    },
                    0x9 => {
                        println!("TOK_IN: {:?}", stat);
                        println!("ADDR => 0x{:02x}", self.addr());
                        println!("CTL: {:?}", self.usb.ctl());
                        self.usb.set_addr(|r| r.set_addr(self.addr()));
                    },
                    0x5 => {
                        println!("TOK_SOF");
                    }
                    0xd => {
                        println!("TOK_SETUP: {:?}", stat);
                        let index = BDT.index(ep as usize, tx, odd);
                        // println!("index: {}", index);
                        let ep_buf = EP_BUFFERS[index];
                        let t = UsbSetup([
                            ep_buf[0],
                            ep_buf[1],
                            ep_buf[2],
                            ep_buf[3],
                            ep_buf[4],
                            ep_buf[5],
                            ep_buf[6],
                            ep_buf[7],
                        ]);
                        // println!("ADDR: {:?}", self.usb.addr());
                        // println!("TOK_SETUP: {:?}", t);
                        // println!("REQUEST_TYPE: {:?}", t.request_type());
                        // println!("REQUEST:      {:?}", t.request());
                        // println!("VALUE:        {:?}", t.value());
                        // println!("INDEX:        {:?}", t.index());
                        // println!("LENGTH:       {:?}", t.length());
                        println!("REQ: {:02x}{:02x}", t.request().0, t.request_type().0);
                        let mut data_len = 0;
                        match (t.request().0, t.request_type().0) {
                            (0x05, 0x00) => {
                                println!(" => SET_ADDRESS");
                                self.set_addr(t.value().value().into());                                
                            },
                            (0x06, 0x80) => {
                                println!(" => GET_DESCRIPTOR");
                                println!("VALUE:        {:?}", t.value());
                                println!("INDEX:        {:?}", t.index());
                                println!("LENGTH:       {:?}", t.length());

                            let device_descriptor = [
                                0x12, // bLength
                                0x01, // bDescriptorType
                                0x01, // bcdUSB (lo)
                                0x01, // bcdUSB (hi)
                                0x00, // bDeviceClass
                                0x00, // bDeviceSubClass
                                0x00, // bDeviceProtocol
                                0x40, // bMaxPacketSize
                                0xad, // idVendor (lo)
                                0xde, // idVendor (hi)
                                0xef, // idProduct (lo)
                                0xbe, // idProduct (hi)
                                0x00, // bcdDevice (lo)
                                0x00, // bcdDevice (hi)
                                0x01, // iManufacturer "ACME Inc."
                                0x02, // iProduct "Demo Device"
                                0x03, // iSerial "00000000"
                                0x01, // bNumConfigurations
                            ];                            
                            println!("desc_len: {}", device_descriptor.len());
                            },
                            _ => {},
                        }

                        let tx_bd = BDT.bd_mut(ep as usize, true, odd);
                        tx_bd.set_bdesc(|r| r.set_bc(data_len).set_own(1).set_data01(1));



                        self.usb.with_ctl(|r| r.set_txsuspendtokenbusy(0));
                    },
                    _ => {},
                }
                if !stat.test_tx() {
                    bd.set_bdesc(|r| r.set_own(1).set_bc(ENDPOINT_BUF_SIZE).set_data01(0).set_dts(1));
                }
                // Send response packet


                // for i in 0..4 {
                //     for (n, b) in EP_BUFFERS[i].iter().enumerate() {
                //         if n % 8 == 0 { print!("\n"); }
                //         print!("{:02x}", b);
                //     }
                //     println!("");
                // }                            
            }
            println!("tokdne done");
            self.usb.set_istat(|r| r.set_tokdne(1));

        }
        if istat.test_sleep() {
            self.usb.set_istat(|r| r.set_sleep(1));
            println!("SLEEP: {:?}", self.usb.stat());
        }

        if istat.test_stall() {
            self.usb.set_istat(|r| r.set_stall(1));
        }
    }
}