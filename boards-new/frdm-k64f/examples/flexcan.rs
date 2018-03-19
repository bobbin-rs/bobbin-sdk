#![no_std]
#![no_main]

#[macro_use]
extern crate frdm_k64f as board;

use board::mcu::pin::*;
use board::mcu::flexcan::*;

// CAN0
// CAN0_TX - PTB18
// CAN0_RX - PTB19

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("FLEXCAN Test");    

    let c0 = CAN0;
    let c0_tx = PTB18;
    let c0_rx = PTB19;

    c0.gate_enable();
    c0_tx.port().gate_enable();
    c0_rx.port().gate_enable();

    c0_tx.connect_to(c0);
    c0_rx.connect_to(c0);

    
    let rx = c0.mbuf(0);
    let tx = c0.mbuf(1);

    // // Initialize the Module Configuration Register (CAN_MCR)
    // // Initialize the Control 1 Register (CAN_CTRL1) and optionally the CAN Bit Timing Register (CAN_CBT). 
    // // Initialize also the CAN FD CAN Bit Timing Register (CAN_FDCBT).
    // // Initialize the Message Buffers
    // // Initialize the Rx Individual Mask Registers (CAN_RXIMRn)
    // // Set required interrupt mask bits in the CAN_IMASK Registers (for all MB interrupts) and in CAN_CTRL1 / CAN_CTRL2 Registers (for Bus Off and Error interrupts)
    // // If Pretended Networking mode is enabled, configure the necessary registers for selective Wake Up
    // // Negate the HALT bit in CAN_MCR

    // Initialize the Module Configuration Register
    // • Enable the individual filtering per MB and reception queue features by setting the IRMQ bit
    // • Enable the warning interrupts by setting the WRNEN bit
    // • If required, disable frame self reception by setting the SRXDIS bit
    // • Enable the Rx FIFO by setting the RFEN bit
    // • Enable the abort mechanism by setting the AEN bit
    // • Enable the local priority feature by setting the LPRIOEN bit
    // • Initialize the Control Register
    // • Determine the bit timing parameters: PROPSEG, PSEG1, PSEG2, RJW • Determine the bit rate by programming the PRESDIV field
    // • Determine the internal arbitration mode (LBUF bit)
    // • Initialize the Message Buffers
    // • The Control and Status word of all Message Buffers must be initialized • If Rx FIFO was enabled, the ID filter table must be initialized
    // • Other entries in each Message Buffer should be initialized as required
    // • Initialize the Rx Individual Mask Registers
    // • Set required interrupt mask bits in the IMASK Registers (for all MB interrupts), in MCR Register for Wake-Up interrupt and in CTRL Register (for Bus Off and Error interrupts)
    // • Negate the HALT bit in MCR        

    dump_can(c0);

    c0.with_ctrl1(|r| r.set_clksrc(1));

    // // Check for Low Power Mode    

    if c0.mcr().test_lpmack() {
        // Enable Clock
        c0.with_mcr(|r| r.set_mdis(false).set_frz(false).set_halt(false));
        while c0.mcr().test_lpmack() {}
    }

    // Soft Reset
    {
        println!("Soft Reset...");
        c0.with_mcr(|r| r.set_softrst(true));
        while c0.mcr().test_softrst() {}
        println!("Soft Reset Complete");
    }

    // Clear FlexCAN Memory
    c0.clear_ram(16 * 4);
    // Set Freeze, Halt
    {
        println!("Enter Freeze Mode");
        c0.enter_freeze_mode();

    }

    // // Configure CAN Peripheral

    // // propseg: 0x04, phaseseg1: 0x07, phaseseg2: 0x01, predivider: 0x00, rjumpwidth: 1
    c0.with_ctrl1(|r| {
        r.set_propseg(0x4).set_pseg1(0x07).set_pseg2(0x01).set_presdiv(0).set_rjw(1)
    });

    // // Set Self Reception Disabled = False
    c0.with_mcr(|r| r
        .set_srxdis(false)
        .set_irmq(true)
    );
    // // Enable Individual Request Masking


    // // Set Loopback Mode = True
    // c0.set_lpb(true);
    c0.with_ctrl1(|r| r.set_lpb(false));

    // // Setup RX Mailbox

    rx.set_idmask(0x0);
    rx.set_code(Code::RxEmpty);    

    // Setup TX Mailbox
    tx.set_code(Code::TxInactive);


    dump_can(c0);
    // println!("RX: Code = {:?} DLC: {} ID: {:08x} TS: {:08x}", rx.code(), rx.dlc(), rx.id_std(), rx.time_stamp());
    // println!("TX: Code = {:?} DLC: {} ID: {:08x} TS: {:08x}", tx.code(), tx.dlc(), tx.id_std(), tx.time_stamp());

    // Enable CAN Peripheral
    println!("Exit Freeze Mode");
    // c0.exit_freeze_mode();
    c0.with_mcr(|r| r.set_frz(0).set_halt(0));
    while c0.mcr().test_frzack() {}
    dump_can(c0);

    println!("Loop");

    loop {
        println!("Tick...");

        // Transmit Message
        if let Code::TxInactive = tx.code() {
            // tx.set_code(Code::TxData);
            // tx.set_id_std(0x230);
            let tx_id = std_id(0x230);
            tx.write(tx_id, &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);
            let mut count = 0;
            while tx.code() != Code::TxInactive {
                if count == 500_000 {
                    println!("Timeout: code = {:?}", tx.code());
                    println!("{:?}", tx);
                    dump_can(c0);
                    panic!("Timeout");
                }
                count += 1;
            }
        }
        board::delay(500);
    }
}

fn dump_can(can: Can0) {
    println!("MCR: {:?}\nCTRL1: {:?}\nCTRL2: {:?}", can.mcr(), can.ctrl1(), can.ctrl2());
    println!("IFLAG1: {:?}\nESR1: {:?}\nESR2: {:?}\nECR: {:?}\nTIMER: {:?}", 
        can.iflag1(), can.esr1(), can.esr2(), can.ecr(), can.timer()
    );
}

