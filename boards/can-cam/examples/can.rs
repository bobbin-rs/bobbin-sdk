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

    println!("Clock: {} Baud: {} Prescaler: {}", can_clk, can_baud, brp);

    can.set_btr(|r| r
        .set_lbkm(1) // Loopback Mode
        .set_sjw(sjw) // Resynchronization Jump Width
        .set_ts2(ts2) // Time Segment 2
        .set_ts1(ts1) // Time Segment 1
        .set_brp(brp) // Baud Rate Prescaler
    );

    println!("BTR: {:?}", can.btr());

    // Setup Configuration

    println!("MCR: {:?}", can.mcr());

    // Enter Normal Mode

    println!("Enter Normal Mode");

    can.with_mcr(|r| r.set_inrq(0).set_sleep(0));
    while can.msr().test_inak() {}

    println!("In Normal Mode");

    board::delay(1000);

    println!("Enter Sleep Mode");
    can.with_mcr(|r| r.set_inrq(0).set_sleep(1));
    while !can.msr().test_slak() {}
    println!("In Sleep Mode");

    println!("Done");

    loop {}
}

