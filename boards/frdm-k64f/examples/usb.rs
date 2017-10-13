#![no_std]
#![no_main]
#![feature(repr_align, attr_literals)]

#[macro_use]
extern crate frdm_k64f as board;

use board::hal::sim::*;
use board::hal::usb::*;
// use board::hal::clock::*;
// use board::clock::CLK;

#[derive(Debug, Default, Clone, Copy)]
pub struct BufferDescriptor {
    desc: u32,
    addr: u32,
}

#[repr(align(512))]
pub struct BufferDescriptorTable([BufferDescriptor; 16]);

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Running USB");
    board::delay(100);
    
    let mut bdt = BufferDescriptorTable([BufferDescriptor::default() ; 16]);

    println!("BDT: {:p}", &bdt);

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

    usb.set_bdt(&mut bdt as *mut BufferDescriptorTable as u32);
    usb.dump_bdt();

    usb.enable();
    usb.dump();
    println!("Done with Init");
    loop {
        board::delay(1000);
    }
}

use board::common::{Irq, Poll};
use board::cortexm::hal::nvic;
use board::cortexm::hal::scb::SCB;

// use core::cell::Cell;
// use core::marker::PhantomData;

pub struct UsbDriver {
    usb: UsbPeriph,    
}

unsafe impl Sync for UsbDriver {}
unsafe impl Send for UsbDriver {}

impl UsbDriver {
    pub fn new<U: Into<UsbPeriph>>(usb: U) -> Self {
        UsbDriver {
            usb: usb.into(),
        }
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
        println!("{:?}", istat);

        if istat.test_usbrst() {
            self.usb.set_istat(|r| r.set_usbrst(1));
            // println!("USBRST");
        }

        board::delay(100);
    }
}