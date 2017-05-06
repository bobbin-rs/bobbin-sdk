#![no_std]
#![no_main]

#[macro_use]
extern crate evb_s32k144 as board;

use board::chip::can::{Can, Esr1};
use board::hal::can::Code;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("FLEXCAN Test");

    let c0 = board::can::can0();

    // Initialize the Module Configuration Register (CAN_MCR)
    // Initialize the Control 1 Register (CAN_CTRL1) and optionally the CAN Bit Timing Register (CAN_CBT). 
    // Initialize also the CAN FD CAN Bit Timing Register (CAN_FDCBT).
    // Initialize the Message Buffers
    // Initialize the Rx Individual Mask Registers (CAN_RXIMRn)
    // Set required interrupt mask bits in the CAN_IMASK Registers (for all MB interrupts) and in CAN_CTRL1 / CAN_CTRL2 Registers (for Bus Off and Error interrupts)
    // If Pretended Networking mode is enabled, configure the necessary registers for selective Wake Up
    // Negate the HALT bit in CAN_MCR

        

    dump_can(c0.can);

    // Check for Low Power Mode    

    if c0.lpmack() {
        // Enable Clock
        c0.set_mdis(false);
        c0.set_frz(false);
        c0.set_halt(false);
        while c0.lpmack() {}
    }

    // Soft Reset
    {
        println!("Soft Reset...");
        c0.set_softrst(true);
        while c0.softrst() {}
        println!("Soft Reset Complete");
    }

    // Clear FlexCAN Memory
    c0.clear_ram();
    // Set Freeze, Halt
    {
        println!("Enter Freeze Mode");
        c0.enter_freeze_mode();

    }

    // Configure CAN Peripheral

    // propseg: 0x04, phaseseg1: 0x07, phaseseg2: 0x01, predivider: 0x00, rjumpwidth: 1

    unsafe {
        let mut can = c0.can;        
        can.with_ctrl1(|r| {
            r.set_propseg(0x4).set_pseg1(0x07).set_pseg2(0x01).set_presdiv(0).set_rjw(1)
        });
    }

    // Set Self Reception Disabled = False
    c0.set_srxdis(false);
    // Enable Individual Request Masking
    c0.set_irmq(true);

    // Set Loopback Mode = True
    c0.set_lpb(true);

    // Setup RX Mailbox

    c0.set_mi(0, 0x0);
    c0.set_id(0, 0x123 << 18); 
    c0.set_code(0, Code::RxEmpty);

    // Setup TX Mailbox
    c0.set_code(1, Code::TxInactive);


    dump_can(c0.can);
    println!("RX: Code = {:?} DLC: {} ID: {:08x} TS: {:08x}", c0.code(0), c0.dlc(0), c0.id(0), c0.time_stamp(0));
    println!("TX: Code = {:?} DLC: {} ID: {:08x} TS: {:08x}", c0.code(1), c0.dlc(1), c0.id(1), c0.time_stamp(1));

    // Enable CAN Peripheral
    println!("Exit Freeze Mode");
    c0.exit_freeze_mode();

    // Setup timer
    let timer = board::timer::lpit1();
    timer.set_value(40_000 * 500);
    timer.clr_tif();
    timer.set_tie(true);
    timer.set_enabled(true);    
    let mut i = 0;
    loop {
        if timer.tif() {
            timer.clr_tif();
            println!("Tick...");

            // Transmit Message
            if let Code::TxInactive = c0.code(1) {
                c0.set_code(1, Code::TxData);
                c0.set_id(1, i << 18); 
                //c0.set_id(1, 0x123 << 0); 
                c0.set_dlc(1, 0x8);
            }
            i += 1;
            
        }

        if c0.bufi(0) {
            c0.clr_bufi(0);
            println!("RX: Code = {:?} DLC: {} ID: {:08x} TS: {:08x}", c0.code(0), c0.dlc(0), c0.id(0), c0.time_stamp(0));
            //c0.set_id(0, 0x123 << 18);
            c0.set_id(0, 0);
            c0.set_code(0, Code::RxEmpty);
            
        }
        if c0.bufi(1) {
            c0.clr_bufi(1);
            println!("TX: Code = {:?} DLC: {} ID: {:08x} TS: {:08x}", c0.code(1), c0.dlc(1), c0.id(1), c0.time_stamp(1));
        }
        

        // If message available, print out message
    }
}

fn dump_can(can: Can) {
    unsafe {
        println!("MCR: {:?}\nCTRL1: {:?}\nCTRL2: {:?}", can.mcr(), can.ctrl1(), can.ctrl2());
        println!("IFLAG1: {:?}\nESR1: {:?}\nESR2: {:?}\nECR: {:?}\nTIMER: {:?}", 
            can.iflag1(), can.esr1(), can.esr2(), can.ecr(), can.timer()
        );
    }
}

