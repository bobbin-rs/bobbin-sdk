#![no_std]
#![no_main]
#![allow(dead_code)]

#[macro_use]
extern crate frdm_k64f as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("[start] Running tests for frdm-k64f");
    test_crc();
    test_gpio();
    test_systick();
    test_ftm();
    test_pit();
    test_lptmr();
    test_adc();
    test_dma();
    test_irq();
    test_uart();
    test_i2c();
    test_spi();
    test_flexcan();
    println!("[done] All tests passed");
    loop {}
}

fn test_crc() {
    use board::common::crc::*;
    use board::hal::crc::*;

    println!("# Setting up CRC");    

    CRC.sim_enable();

    let tot = 1;
    let totr = 2;
    let fxor = 1;
    
    CRC
        .set_ctrl(|r| r.set_tcrc(1).set_tot(tot).set_totr(totr).set_fxor(fxor))
        .set_poly(CRC_32);
        
    let v: u32 = CRC.init(CRC_32_START).write(0x12345678u32).read();
    assert_eq!(v, 0x4A090E98);

    let msg = b"Hello World";
    let v: u32 = CRC.init(CRC_32_START).write(&msg[..]).read();
    assert_eq!(v, 0x4A17B156);

    // NOTE: These tests require updating the read() function to
    // byte swap 16 bit values, which may be incorrect. Add
    // further tests with other CRC specifications.

    let tot = 1;
    let totr = 1;
    let fxor = 0;

    // println!("{} {} {}", tot, totr,fxor);
    CRC
        .set_ctrl(|r| r.set_tcrc(0).set_tot(tot).set_totr(totr).set_fxor(fxor))
        .set_poly(0x8005u16);

    let v: u16 = CRC.init(0u16).write(0x1234u32).read();
    // println!("{:04x} {:04x}", v, 0x770D);
    assert_eq!(v, 0x770d);


    let v: u16 = CRC.init(0u16).write(&msg[..]).read();
    // println!("{:04x} {:04x}", v, 0x3EEB);
    assert_eq!(v, 0x3eeb);

    let v: u16 = CRC.init(0u16).write(&b"123456789"[..]).read();
    // println!("{:04x} {:04x}", v, 0x3EEB);
    assert_eq!(v, 0xBB3D);

    // assert_eq!(v, 0x3EEB);
    println!("[pass] CRC OK");

}

/// Jumper PTC16(D0) to PTC17(D1)
fn test_gpio() {
    use board::hal::port::*;
    use board::hal::gpio::*;

    let port = PORTC;
    let port_out = PTC16;
    let port_in = PTC17;
    let gpio_out = port_out.gpio_pin();
    let gpio_in = port_in.gpio_pin();

    port.sim_enable();
    port_out.set_mux_gpio();
    port_in.set_mux_gpio();

    gpio_out.set_dir_output();
    gpio_in.set_dir_input();

    gpio_out.set_output(false);
    assert_eq!(gpio_in.input(), false);

    gpio_out.set_output(true);
    assert_eq!(gpio_in.input(), true);

    gpio_out.toggle_output();
    assert_eq!(gpio_in.input(), false);

    gpio_out.toggle_output();
    assert_eq!(gpio_in.input(), true);

    port_out.set_mux_disabled();
    port_in.set_mux_disabled();

    println!("[pass] GPIO OK");

}

fn test_systick() {
    use board::hal::systick::*;

    println!("# Testing SYSTICK");
    test_systick(&SYSTICK, ClockSource::Internal);
    println!("[pass] SYSTICK OK");
}

fn test_ftm() {
    use board::hal::ftm::*;

    println!("# Testing FTM");

    let tim = FTM0;    
    tim.sim_enable();

    test_ftm(&tim);
    
    tim.sim_disable();

    println!("[pass] FTM OK");
}

fn test_pit() {
    use board::hal::pit::*;

    println!("# Testing PIT");

    let tim = PIT;
    let tim_ch = PIT_CH0;

    tim.sim_enable();

    test_pit(&tim_ch);

    tim.sim_disable();

    println!("[pass] PIT OK");
}

fn test_lptmr() {
    use board::hal::lptmr::*;

    println!("# Testing LPTMR");

    let tim = LPTMR0;
    tim.sim_enable();
    tim.with_csr(|r| r.set_ten(0));
    tim.with_psr(|r| r.set_pbyp(1).set_pcs(0b11));

    test_lptmr(&tim);

    tim.sim_disable();

    println!("[pass] LPTMR OK");
}

fn test_adc() {
    use board::common::bits::*;
    use board::hal::adc::*;

    let adc = ADC0;
    let ch0 = ADC0_TEMP;    
    let ch1 = ADC0_BANDGAP;
    let ch2 = ADC0_REFSH;
    let ch3 = ADC0_REFSL;

    adc.sim_enable();
    let v: U12 = ch0.analog_read();
    println!("# ADC0_TEMP:    {}", v);
    // Arbitrary bounds - find formula
    // assert!(v.value() > 550 && v.value() < 650);

    let v: U12 = ch1.analog_read();
    println!("# ADC0_BANDGAP: {}", v);
    // Arbitrary bounds - find formula
    // assert!(v.value() > 500 && v.value() < 700);

    let v: U12 = ch2.analog_read();
    println!("# ADC0_REFSH:   {}", v);
    assert!(v.value() == 4095);

    let v: U12 = ch3.analog_read();
    println!("# ADC0_REFSL:   {}", v);
    assert!(v.value() == 0);

    adc.sim_disable();

    println!("[pass] ADC OK");
}

fn test_dma() {
    use board::hal::edma::*;

    let mut src = [0u8; 32];
    let mut dst = [0u8; 32];

    for (i, s) in src.iter_mut().enumerate() {
        *s = i as u8;
    }

    let d = DMA;
    let ch = DMA0;

    d.set_tcd_citer_elinkno(ch.index(), |_| TcdCiterElinkno(0x0001));
    d.set_tcd_biter_elinkno(ch.index(), |_| TcdBiterElinkno(0x0001));
    d.set_tcd_nbytes_mlno(ch.index(), |_| TcdNbytesMlno(32));

    d.set_tcd_saddr(ch.index(), |_| TcdSaddr(0).set_saddr(src.as_ptr() as u32));
    d.set_tcd_soff(ch.index(), |_| TcdSoff(0).set_soff(0x1));
    d.with_tcd_attr(ch.index(), |r| r.set_ssize(0));
    d.set_tcd_slast(ch.index(), |_| TcdSlast(0));
    


    d.set_tcd_daddr(ch.index(), |_| TcdDaddr(0).set_daddr(dst.as_mut_ptr() as u32));
    d.set_tcd_doff(ch.index(), |_| TcdDoff(0).set_doff(0x1));
    d.with_tcd_attr(ch.index(), |r| r.set_dsize(0).set_dmod(0).set_smod(0));
    d.set_tcd_dlastsga(ch.index(), |_| TcdDlastsga(0));
    

    d.with_tcd_csr(ch.index(), |r| r.set_intmajor(0).set_inthalf(0).set_majorlinkch(0).set_majorelink(0));

    d.with_tcd_csr(ch.index(), |r| r.set_start(1));
    loop {
        let err = d.err();
        if err.err(ch.index()) != 0 {
            println!("[fail] DMA_ES: {:?}", d.es());

            break;
        }
        let csr = d.tcd_csr(ch.index());
        if csr.done() != 0 {
            break;
        }        
    }

    assert_eq!(src, dst);
    println!("[pass] DMA OK");
}

/// Jumper PTC16(D0) to PTC17(D1)
fn test_irq() {
    use board::hal::port::*;
    use board::hal::gpio::*;

    let port = PORTC;
    let port_out = PTC16;
    let port_in = PTC17;
    let gpio_out = port_out.gpio_pin();
    let gpio_in = port_in.gpio_pin();

    port.sim_enable();
    port_out.set_mux_gpio();
    port_in.set_mux_gpio();

    gpio_out.set_dir_output().set_output(true);
    gpio_in.set_dir_input();

    port_in.set_irqc(board::hal::port::InterruptConfig::IrqEitherEdge);


    // Falling Edge
    port_in.clr_isf();
    assert!(!port_in.isf());   
    gpio_out.set_output(false);
    assert!(port_in.isf());   

    // Rising Edge
    port_in.clr_isf();
    assert!(!port_in.isf());   
    gpio_out.set_output(true);
    assert!(port_in.isf());   

    println!("[pass] IRQ OK");
}

fn test_uart() {
    use board::hal::uart::*;
    use board::hal::clock::Clock;
    use board::clock::CLK;

    let uart_baud = 115_200;
    let uart = UART1;

    uart.sim_enable();

    // println!("# UART Clock: {}", uart.clock(&CLK).expect("No bus clock"));
    let baud_div = uart.clock(&CLK).expect("No bus clock") / (16 * uart_baud);
    uart
        .set_config(|c| c.set_baud_divisor(baud_div as u16))
        .with_c1(|r| r.set_loops(true))
        .enable();

    let src = b"Hello World\r\n";
    let mut dst = [0u8; 13];

    assert!(src.len() == dst.len());
    for i in 0..src.len() {
        uart.wait_tx().tx(src[i]);
        dst[i] = uart.wait_rx().rx();
    }
    assert_eq!(src, &dst);
    println!("[pass] UART OK");
}

// // NOTE: Board must be powered by 12V to use UJA1169
// // Without power, all registers will read 0xff

// // SPLLDIV2 = 40MHz
// // Prescale = Divide by 4 => 10MHz
// // SCKDIV = 8 => Divide by 10 => 1MHz
// fn test_lpspi() {
//     use board::hal::lpspi::*;
//     use board::hal::pcc::*;
//     use board::hal::port::*;


//     pub const SCK: Ptb14 = PTB14;
//     pub const MOSI: Ptb15 = PTB15;
//     pub const MISO: Ptb16 = PTB16;
//     pub const PCS3: Ptb17 = PTB17;

//     pub const SPI: Lpspi1 = LPSPI1;

//     SCK.port().pcc_set_enabled(true);
//     SCK.mode_spi_sck(&SPI);

//     MISO.port().pcc_set_enabled(true);
//     MISO.mode_spi_sout(&SPI);

//     MOSI.port().pcc_set_enabled(true);
//     MOSI.mode_spi_sin(&SPI);

//     PCS3.port().pcc_set_enabled(true);
//     PCS3.mode_spi_pcs3(&SPI);

//     let l1 = SPI;
//     l1.pcc_set_clock_source(ClockSource::SPLLDIV2).pcc_set_enabled(true);
//     l1.set_enabled(false);    
    
//     l1.configure(Config::default()
//         .set_master(true)
//         .set_clock_config(
//             8, // SCKDIV
//             8, // DBT
//             9, // PCSSCK
//             4  // SCKPCS
//         )
//     );
//     l1.with_fcr(|r| r.set_txwater(3));

//     l1.set_enabled(true);
//     let t = l1.target()
//         .cpha(true)
//         .prescale(2)
//         .pcs(3)
//         .framesz(15);


//     t.configure();

//     let u = board::uja1169::device(t);
//     let r = u.reg();
//     let ids = r.ids().ids();
//     if ids == 0xff {
//         panic!("*** IDS = 0xff, device may not have 12v power. ***");
//     }
//     assert_eq!(ids, 0xEF);

//     println!("[pass] LPSPI OK");
// }

fn test_flexcan() {
    use board::hal::flexcan::*;

    // NOTE - Check message buffer size limits for this chip

    // Initialize the Module Configuration Register (CAN_MCR)
    // Initialize the Control 1 Register (CAN_CTRL1) and optionally the CAN Bit Timing Register (CAN_CBT). 
    // Initialize also the CAN FD CAN Bit Timing Register (CAN_FDCBT).
    // Initialize the Message Buffers
    // Initialize the Rx Individual Mask Registers (CAN_RXIMRn)
    // Set required interrupt mask bits in the CAN_IMASK Registers (for all MB interrupts) and in CAN_CTRL1 / CAN_CTRL2 Registers (for Bus Off and Error interrupts)
    // If Pretended Networking mode is enabled, configure the necessary registers for selective Wake Up
    // Negate the HALT bit in CAN_MCR

    // println!("# FlexCan Test Start");

    let can = CAN0;
    let rx = can.mbuf(0);
    let tx = can.mbuf(1);
    can.sim_enable();
    can.enable();
    {
        if can.mcr().test_lpmack() {
            // Enable Clock
            can.with_mcr(|r| r.set_frz(false).set_halt(false));
            while can.mcr().test_lpmack() {}
        }
        can.with_mcr(|r| r.set_softrst(true));
        while can.mcr().test_softrst() {}
        // println!("# Clear RAM (MAXMB={})", can.mcr().maxmb());
        can.clear_ram(16);        
        can.with_mcr(|r| r.set_frz(true).set_halt(true));
        can.with_mcr(|r| r
            .set_irmq(true) // Individual Rx Masking and Queue Enable
        );
        can.with_ctrl1(|r| r
            .set_lpb(true) // Loopback Enabled
            .set_propseg(0x6) // Propagation Segment = 7
            .set_pseg1(0x03) // Phase Segment 1 = 4
            .set_pseg2(0x03) // Phase Segment 2 = 4
            .set_presdiv(0) // Prescale Divider = 1
            .set_rjw(3) // Resync Jump Width = 4
            .set_smp(1) // Samples = 3
        );
        rx.set_idmask(0x1fffffff);
        rx.set_idmask(0x1fffffff);
    }
    {
        can.with_mcr(|r| r.set_frz(false).set_halt(false));
    }
    for _ in 0..10 {
        let id = CanId::Ext(ExtendedId(0x1234));

        // println!("# Prepare Rx Mailbox");
        rx.set_id(CanId::Ext(ExtendedId(0x1234)));
        rx.set_code(Code::RxEmpty);

        // println!("# Write Packet");
        let buf_out = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        tx.write(id, &buf_out);
        // println!("# > {:?} {:?}", id, buf_out);

        // println!("# Wait for Tx Flag");
        while !tx.flag() {}
        tx.clr_flag();

        // println!("TX: {:?}", tx);
        // println!("# Wait for Rx Flag");
        while !rx.flag() {}
        rx.clr_flag();
        let mut buf_in = [0u8; 16];
        let (id_rx, n) = rx.read(&mut buf_in);
        // println!("# < {:?} {:?}", id, &buf_in[..n]);
        assert_eq!(id_rx, id);
        assert_eq!(&buf_out[..], &buf_in[..n]);
    }
    can.disable();
    can.sim_disable();

    println!("[pass] FLEXCAN OK")
}

fn test_i2c() {
    use board::hal::i2c::*;
    use board::hal::port::*;

    pub const I2CADDR: u8 = 0x1D;

    let port = PORTE;
    let port_scl = PTE24;
    let port_sda = PTE25;

    let i2c = I2C0;

    port.sim_enable();

    port_scl.set_mux_gpio();
    port_sda.set_mux_gpio();

    port_scl.mode_i2c_scl(&i2c).set_pull_none().set_ode(true);
    port_sda.mode_i2c_sda(&i2c).set_pull_none().set_ode(true);

    i2c.sim_enable();

    let mult = 0;
    let icr = 0x1c;
    let addr = 0x1d;

    i2c.init(mult, icr);

    // // Write 1
    // i2c.with_tx(|i| {
    //     i.write(addr, &[0x0d]);
    // });

    // // Write 2
    // i2c.with_tx(|i| {
    //     i.write(addr, &[0x0d, 0x0e]);
    // });

    // // Write 3
    // i2c.with_tx(|i| {
    //     i.write(addr, &[0x0d, 0x0e, 0x0f]);
    // });
   
    // // Read 1
    // let mut buf: [u8; 1] = [0u8; 1];
    // i2c.with_tx(|i| {
    //     i.read(addr, &mut buf);
    // });

    // // Read 2
    // let mut buf: [u8; 2] = [0u8; 2];
    // i2c.with_tx(|i| {
    //     i.read(addr, &mut buf);
    // });

    // // Read 3
    // let mut buf: [u8; 3] = [0u8; 3];
    // i2c.with_tx(|i| {
    //     i.read(addr, &mut buf);
    // });

    // // Write / Write
    // i2c.with_tx(|i| {
    //     i.write(addr, &[0x0d]);
    //     i.restart();
    //     i.write(addr, &[0x0d]);
    // });

    // // Write / Read
    // let mut buf: [u8; 1] = [0u8; 1];
    // i2c.with_tx(|i| {
    //     i.write(addr, &[0x0d]);
    //     i.restart();
    //     i.read(addr, &mut buf);
    // });

    // // Write / Read
    // let mut buf: [u8; 6] = [0u8; 6];
    // i2c.with_tx(|i| {
    //     i.write(addr, &[0x00]);
    //     i.restart();
    //     i.read(addr, &mut buf);
    // });


    // let whoami = i2c.reg_read(addr, 0xd);
    // println!("whoami: {:02x}", whoami);

    // for _ in 0..5 {
    //     println!("temp: {}", i2c.reg_read(addr, 0x51));
    //     println!("{} {} | {} {} | {} {}",
    //         i2c.reg_read(addr, 0x01),
    //         i2c.reg_read(addr, 0x02),
    //         i2c.reg_read(addr, 0x03),
    //         i2c.reg_read(addr, 0x04),
    //         i2c.reg_read(addr, 0x05),
    //         i2c.reg_read(addr, 0x06),
    //     );
    //     board::delay(1000);
    // }

    let cmd: [u8; 1] = [0xd];
    let mut buf = [0u8];
    i2c.transfer(addr, &cmd, &mut buf);
    assert_eq!(buf[0], 0xc7);

    i2c.sim_disable();
    println!("[pass] I2C OK");

}

/// Jumper PTD2(D12) and PTD3(D13)
fn test_spi() {
    pub use board::hal::spi::*;
    pub use board::hal::port::*;
    pub use board::hal::gpio::*;


    let port = PORTD;
    let port_sck = PTD1; // D13
    let port_sout = PTD2; // D12
    let port_sin = PTD3; // D11
    let port_pcs0 = PTD0; // D10

    let spi = SPI0;

    port.sim_enable();
    port_sck.mode_spi_sck(&spi);
    port_sout.mode_spi_sout(&spi);
    port_sin.mode_spi_sin(&spi);
    port_pcs0.mode_spi_pcs0(&spi);

    spi.sim_enable();
    spi.init(0b1000, 0b00);

    let mut bytes_out = [0u8; 32];
    let mut bytes_in = [0u8; 32];
    for (i, b) in bytes_out.iter_mut().enumerate() {
        *b = i as u8
    }

    spi.transfer(&bytes_out, &mut bytes_in);
    assert_eq!(bytes_out, bytes_in);

    spi.sim_disable();

    println!("[pass] SPI OK");

    fn reg_read(spi: &SpiPeriph, reg: u8) -> u8 {
        let cmd = [reg];
        let mut buf = [0u8];
        spi.write(&cmd);   
        spi.read(&mut buf);
        buf[0]
    }

}


/// RFM9x LoRa Radio on pins D10-D13
fn test_spi_lora() {
    pub use board::hal::spi::*;
    pub use board::hal::port::*;
    pub use board::hal::gpio::*;


    let port = PORTD;
    let port_sck = PTD1; // D13
    let port_sout = PTD2; // D12
    let port_sin = PTD3; // D11
    let port_pcs0 = PTD0; // D10

    let spi = SPI0;

    port.sim_enable();
    port_sck.mode_spi_sck(&spi);
    port_sout.mode_spi_sout(&spi);
    port_sin.mode_spi_sin(&spi);
    port_pcs0.mode_spi_pcs0(&spi);

    spi.sim_enable();
    spi.init(0b1000, 0b00);

    // Read Version @ 0x42, expect 0x12
    let v = reg_read(&spi, 0x42);
    // println!("version: 0x{:02x}", v);
    assert_eq!(v, 0x12);

    spi.sim_disable();

    println!("[pass] SPI OK");

    fn reg_read(spi: &SpiPeriph, reg: u8) -> u8 {
        let cmd = [reg];
        let mut buf = [0u8];
        spi.write(&cmd);   
        spi.read(&mut buf);
        buf[0]
    }

}