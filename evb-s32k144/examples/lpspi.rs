#![no_std]
#![no_main]

#[macro_use]
extern crate evb_s32k144 as board;

use board::hal::lpspi::*;
use board::uja1169::Mode;
use board::hal::pcc::*;
use board::hal::port::*;

// NOTE: Board must be powered by 12V to use UJA1169
// Without power, all registers will read 0xff

// SPLLDIV2 = 40MHz
// Prescale = Divide by 4 => 10MHz
// SCKDIV = 8 => Divide by 10 => 1MHz


#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("LPSPI Test");

    pub const SCK: Ptb14 = PTB14;
    pub const MOSI: Ptb15 = PTB15;
    pub const MISO: Ptb16 = PTB16;
    pub const PCS3: Ptb17 = PTB17;

    pub const SPI: Lpspi1 = LPSPI1;

    SCK.port().pcc_set_enabled(true);
    SCK.mode_spi_sck(&SPI);

    MISO.port().pcc_set_enabled(true);
    MISO.mode_spi_sout(&SPI);

    MOSI.port().pcc_set_enabled(true);
    MOSI.mode_spi_sin(&SPI);

    PCS3.port().pcc_set_enabled(true);
    PCS3.mode_spi_pcs3(&SPI);

    let l1 = SPI;
    l1.pcc_set_clock_source(ClockSource::SPLLDIV2).pcc_set_enabled(true);
    l1.set_enabled(false);    
    
    l1.configure(Config::default()
        .set_master(true)
        .set_clock_config(
            8,
            8,
            9,
            4
        )
        // .sckpcs(4)
        // .pcssck(9)
        // .dbt(8)
        // .sckdiv(8)
        // .txwater(3)        
    );
    l1.with_fcr(|r| r.set_txwater(3));

    l1.set_enabled(true);
    let t = l1.target()
        .cpha(true)
        .prescale(2)
        .pcs(3)
        .framesz(15);


    t.configure();

    // let s = l1;
    // println!("CR:     {:?}", s.cr());
    // println!("SR:     {:?}", s.sr());
    // println!("CFGR0:  {:?}", s.cfgr0());
    // println!("CFGR1:  {:?}", s.cfgr1());
    // println!("CCR:    {:?}", s.ccr());
    // println!("FCR:    {:?}", s.fcr());
    // println!("FSR:    {:?}", s.fsr());
    // println!("TCR:    {:?}", s.tcr());
    // println!("RSR:    {:?}", s.rsr());
    // println!("");
    let u = board::uja1169::device(t);
    let r = u.reg();

    let ids = r.ids().ids();
    if ids == 0xff {
        println!("*** IDS = 0xff, device may not have 12v power. ***");
        loop {}
    }

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
    println!("Changing to Normal mode");
    u.set_mode(Mode::Normal);
    println!("mode: {:?}", u.mode());

    println!("CAN STATUS");
    println!("0x20 CANC:   {:?}", r.canc());
    println!("0x22 TS:     {:?}", r.ts());
    println!("0x23 TEE:    {:?}", r.tee());
    println!("0x26 DR:     {:?}", r.dr());
    for i in 0..4 {
        println!("0x{:02x} ID{}     {:?}", 0x27+i, i, r.id(i));
    }
    for i in 0..4 {
        println!("0x{:02x} M{}      {:?}", 0x2b+i, i, r.m(i));
    }
    println!("0x2F FC     {:?}", r.fc());
    for i in 0..8 {
        println!("0x{:02x} DM{}     {:?}", 0x68+i, i, r.dm(i));
    }
    println!("DONE");
    
    loop {}
}