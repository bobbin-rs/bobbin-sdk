#![no_std]
#![no_main]

#[macro_use]
extern crate evb_s32k144 as board;

use board::hal::lpspi;
use board::uja1169::Mode;
use board::chip::can::{Can};
use board::hal::can::Code;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("CAN Ping Test");

    let l1 = board::spi::lpspi1();
    l1.set_enabled(false);    
    
    l1.configure(lpspi::Config::default()
        .master(true)
        .sckpcs(4)
        .pcssck(9)
        .dbt(8)
        .sckdiv(8)
        .txwater(3)        
    );
    l1.set_enabled(true);
    let t = l1.target()
        .cpha(true)
        .prescale(2)
        .pcs(3)
        .framesz(15);
    let u = board::uja1169::device(t);
    let r = u.reg();
    println!("ids:    {:?}", r.ids());
    println!("mc:     {:?}", r.mc());
    println!("ms:     {:?}", r.ms());
    println!("wds:    {:?}", r.wds());
    println!("sc:     {:?}", r.sc());
    println!("sbccc:  {:?}", r.sbccc());
    println!("mptnvs: {:?}", r.mtpnvs());
    println!("forced_normal: {}", u.is_forced_normal_mode());
    println!("software_development: {}", u.is_software_development_mode());
    println!("mode: {:?}", u.mode());
    println!("");

    if u.is_forced_normal_mode() {
        println!("Updating board NVM to software dev mode");
        u.write_nvm([0x00, 0x04]);
        println!("Reset device to continue");
        loop {}
    }
    println!("Changing to Normal mode");
    u.set_mode(Mode::Normal);
    println!("Mode: {:?}", u.mode());

    // Turn on V2 regulator
    r.with_rc(|r| r.set_v2c_vextc(0b11));

    // // Transceiver to Active mode

    r.with_canc(|r| r.set_cmc(0b01));

    println!("CAN STATUS");
    println!("0x0 WDS:     {:?}", r.wds());
    println!("0x1 MC:      {:?}", r.mc());
    println!("0x10 RC:     {:?}", r.rc());
    println!("0x1b SS:     {:?}", r.ss());
    println!("0x20 CANC:   {:?}", r.canc());
    println!("0x22 TS:     {:?}", r.ts());



    let c0 = board::can::can0();
    let rx = c0.mbuf(0);
    let tx = c0.mbuf(1);

    // Initialize the Module Configuration Register (CAN_MCR)
    // Initialize the Control 1 Register (CAN_CTRL1) and optionally the CAN Bit Timing Register (CAN_CBT). 
    // Initialize also the CAN FD CAN Bit Timing Register (CAN_FDCBT).
    // Initialize the Message Buffers
    // Initialize the Rx Individual Mask Registers (CAN_RXIMRn)
    // Set required interrupt mask bits in the CAN_IMASK Registers (for all MB interrupts) and in CAN_CTRL1 / CAN_CTRL2 Registers (for Bus Off and Error interrupts)
    // If Pretended Networking mode is enabled, configure the necessary registers for selective Wake Up
    // Negate the HALT bit in CAN_MCR

        

    //dump_can(c0.can);

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
            r.set_propseg(0x6).set_pseg1(0x03).set_pseg2(0x03).set_presdiv(0).set_rjw(3).set_smp(1)
        });
    }

    // Set Self Reception Disabled = False
    c0.set_srxdis(false);

    // Enable Individual Request Masking
    c0.set_irmq(true);

    // Set Loopback Mode = True
    c0.set_lpb(false);

    // Setup RX Mailbox

    rx.set_idmask(0);
    tx.set_id_std(0);
    rx.set_code(Code::RxEmpty);

    // Setup TX Mailbox
    tx.set_code(Code::TxInactive);


    dump_can(c0.can);
    println!("RX: Code = {:?} DLC: {} ID: {:08x} TS: {:08x}", rx.code(), rx.dlc(), rx.id_std(), rx.time_stamp());
    println!("TX: Code = {:?} DLC: {} ID: {:08x} TS: {:08x}", tx.code(), tx.dlc(), tx.id_std(), tx.time_stamp());

    // Enable CAN Peripheral
    println!("Exit Freeze Mode");
    c0.exit_freeze_mode();

    // Setup timer
    let timer = board::timer::lpit1();
    timer.set_value(40_000 * 500);
    timer.clr_tif();
    timer.set_tie(true);
    timer.set_enabled(true);    
    loop {
        if timer.tif() {
            timer.clr_tif();
            //dump_can(c0.can);
            // println!("CAN STATUS");
            // println!("0x0 WDS:     {:?}", r.wds());
            // println!("0x1 MC:      {:?}", r.mc());
            // println!("0x10 RC:     {:?}", r.rc());
            // println!("0x1b SS:     {:?}", r.ss());
            // println!("TX: Code = {:?} DLC: {} ID: {:08x} TS: {:08x}", tx.code(), tx.dlc(), tx.id_std(), tx.time_stamp());
            // Transmit Message
            // if let Code::TxInactive = tx.code() {
                //tx.set_id_std(0x7E0);
                tx.set_id_std(0x7df);
                //tx.write(&[0x02, 0x01, 0x0c]);
                tx.write(&[0x02, 0x01, 0x0c, 0x55, 0x55, 0x55, 0x55, 0x55]);
                //tx.write(&[0x02, 0x01, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x00]);
                // println!("TX: Code = {:?} DLC: {} ID: {:08x} TS: {:08x}", tx.code(), tx.dlc(), tx.id_std(), tx.time_stamp());
            // }
            
        }

        if rx.flag() {
            //println!("RX: Code = {:?} DLC: {} ID: {:08x} TS: {:08x}", rx.code(), rx.dlc(), rx.id_std(), rx.time_stamp());
            let mut buf = [0u8; 16];
            let n = rx.read(&mut buf);
            //println!("RX: {:?} {:?}", rx.mb8h0(), rx.mb8h1());
            print!("< {:04x}: {:08x}", rx.time_stamp(), rx.id_std());
            for i in 0..n {
                print!(" {:02x}", buf[i]);
            }
            println!("");
            rx.set_id_std(0);
            //rx.set_code(Code::RxEmpty);            
            let _ = c0.timer();
            rx.clr_flag();
        }
        if tx.flag() {
            tx.clr_flag();
            let mut buf = [0u8; 16];
            let n = tx.read(&mut buf);            
            //println!("TX: {:?} {:?}", tx.mb8h0(), tx.mb8h1());
            print!("> {:04x}: {:08x}", tx.time_stamp(), tx.id_std());
            for i in 0..n {
                print!(" {:02x}", buf[i]);
            }            
            println!("");
            //println!("TX: Code = {:?} DLC: {} ID: {:08x} TS: {:08x}", tx.code(), tx.dlc(), tx.id_std(), tx.time_stamp());
        }
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

