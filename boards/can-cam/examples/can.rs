#![no_std]
#![no_main]

#[macro_use]
extern crate can_cam as board;

use board::hal::gpio::*;
use board::hal::can::*;
use board::hal::clock::*;
use board::clock::CLK;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    board::delay(100);
    println!("CAN Test");

    let can = CAN1;
    let can_tx = PD1;
    let can_rx = PD0;

    can.rcc_enable();
    can_tx.port().rcc_enable();
    can_rx.port().rcc_enable();

    println!("RCC Enabled");

    can_tx.mode_can_tx(&can).open_drain();
    can_rx.mode_can_rx(&can).open_drain();
    
    println!("Pins Configured");

    // Master Reset

    println!("Master Reset");

    can.with_mcr(|r| r.set_reset(1));
    while can.mcr().test_reset() {}

    println!("Reset Complete");

    // Assert that CAN is in sleep mode after reset

    assert!(can.msr().test_slak(), "CAN not in sleep mode");

    // Enter Initialization Mode

    println!("Enter Initialization Mode");

    can.with_mcr(|r| r.set_inrq(1).set_sleep(0));
    while !can.msr().test_inak() {}

    println!("In Initialization Mode");

    // Setup BTR Register

    let can_clk = can.clock(&CLK).unwrap();
    let can_baud = 500_000;

    // Use 16 time quanta per bit

    let can_prescale = can_clk / (16 * can_baud);
    let brp = can_prescale - 1;
    let ts1 = 12 - 1;
    let ts2 = 3 - 1;
    let sjw = 4 - 1;

    let lbkm = true;
    let silm = false;

    println!("Clock: {} Baud: {} Prescaler: {}", can_clk, can_baud, brp);

    can.set_btr(|r| r
        .set_silm(silm) // Silent Mode
        .set_lbkm(lbkm) // Loopback Mode
        .set_sjw(sjw) // Resynchronization Jump Width
        .set_ts2(ts2) // Time Segment 2
        .set_ts1(ts1) // Time Segment 1
        .set_brp(brp) // Baud Rate Prescaler
    );

    println!("BTR: {:?}", can.btr());
    println!("MCR: {:?}", can.mcr());

    // Setup Filter Registers

    can.with_fmr(|r| r.set_finit(1));

    // Activate Filter 0
    // Set Mask - all zeros
    // Set Identifier - zero

    can.set_fm1r(|r| r.set_fbm(0, 0)); // Identifier Mask Mode
    can.set_fs1r(|r| r.set_fsc(0, 1)); // Single 32 Bit Scale
    can.set_ffa1r(|r| r.set_ffa(0, 0)); // Filter Assigned to FIFO
    can.set_fa1r(|r| r.set_fact(0, 1)); // Filter Active

    can.set_fr0(0, |r| r.set_fb(0x00000000)); // Set ID to 0
    can.set_fr1(0, |r| r.set_fb(0x00000000)); // Set Mask to 0

    can.with_fmr(|r| r.set_finit(0));

    println!("Enter Normal Mode");

    can.with_mcr(|r| r.set_inrq(0).set_sleep(0));
    while can.msr().test_inak() {}

    assert!(!can.msr().test_inak(), "Still in Inactive Mode");
    assert!(!can.msr().test_slak(), "Still in Sleep Mode");

    // Enter Normal Mode

    let id = 0x0123;
    let dlc = 8;
    let data = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];

    let mut ticks = 100;
    let mut n = 0;
    loop {
        if can.rfr(0).test_fmp() {
            println!("> RIR: {:?}", can.rir(0));
            println!("> RDTR: {:?}", can.rdtr(0));
            println!("> RDLR: {:?}", can.rdlr(0));
            println!("> RDHR: {:?}", can.rdhr(0));            
            can.with_rfr(0, |r| r.set_rfom(1));
        }
        // if let Some(msg) = recv(&can) {
        //     println!("< {:?}", msg);
        // }

        if n > 0 {
            n -= 1            
        } else {
            can.set_tdhr(0, |r| r.set_data4(data[4]).set_data5(data[5]).set_data6(data[6]).set_data7(data[7]));            
            can.set_tdlr(0, |r| r.set_data0(data[0]).set_data1(data[1]).set_data2(data[2]).set_data3(data[3]));
            can.set_tdtr(0, |r| r.set_dlc(dlc));
            can.set_tir(0, |r| r.set_stid(id).set_ide(0).set_txrq(1));
            println!("> TIR: {:?}", can.tir(0));
            println!("> TDTR: {:?}", can.tdtr(0));
            println!("> TDLR: {:?}", can.tdlr(0));
            println!("> TDHR: {:?}", can.tdhr(0)); 

            while can.tir(0).test_txrq() {}
            println!("> TX Done");

            n = ticks;
        }
        board::delay(10);
    }



    println!("In Normal Mode");

    board::delay(1000);

    println!("Enter Sleep Mode");
    can.with_mcr(|r| r.set_inrq(0).set_sleep(1));
    while !can.msr().test_slak() {}
    println!("In Sleep Mode");

    println!("Done");

    loop {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Id {
    Std(u16),
    Ext(u32),    
}

#[derive(Debug)]
struct Message {
    id: Id,
    dlc: u8,
    data: [u8; 8],
}

impl Message {
    pub fn new(id: Id, data: &[u8]) -> Message {
        assert!(data.len() < 8);
        let mut d = [0u8; 8];
        &d[..data.len()].copy_from_slice(data);
        Message {
            id: id,
            dlc: data.len() as u8,
            data: d,
        }        
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn data(&self) -> &[u8] {
        &self.data[..self.dlc as usize]
    }
}

