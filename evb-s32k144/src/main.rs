#![no_std]
#![no_main]

#[macro_use]
extern crate evb_s32k144 as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("[start] Running tests for evb-s32k144");
    test_crc();
    test_gpio();
    test_ftm();
    test_lptmr();
    test_systick();
    test_adc();
    test_dma();
    test_irq();
    test_lpuart();
    test_lpspi();
    println!("[done] All tests passed");
    loop {}
}

fn test_crc() {
    use board::common::crc::*;
    use board::hal::crc::*;

    println!("# Setting up CRC");    

    CRC.pcc_enable();

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

/// Jumper PTA11(D0) to PTA17(D1)
fn test_gpio() {
    use board::hal::port::*;
    use board::hal::gpio::*;

    let port = PORTA;
    let port_out = PTA11;
    let port_in = PTA17;
    let gpio_out = port_out.gpio_pin();
    let gpio_in = port_in.gpio_pin();

    port.pcc_enable();
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


fn test_ftm() {
    use board::hal::pcc;
    use board::hal::ftm::*;

    fn check_progress(tim: &FtmPeriph, tim_ch: &FtmCh) {
        let mut c_max = 0;
        while !tim.timeout_flag() {
            let c = tim.counter();
            if c > c_max {
                c_max = c;
            }
        }
        assert!(tim_ch.compare_flag());
        assert!(c_max > 0);
    }

    let tim = FTM0;
    let tim_ch = FTM0_CH1;
    tim
        .pcc_set_clock_source(pcc::ClockSource::SOSCDIV2)
        .pcc_enable();
    // tim.set_enabled(true);

    // Repeat Up Counter
    
    tim_ch.with_csc(|r| r.set_chie(0).set_msb(0).set_msa(1).set_elsb(0).set_elsa(0));
    tim_ch.set_compare(512);

    tim_ch.clr_compare_flag();
    assert!(!tim_ch.compare_flag());
    tim
        .start_up(1024)
        .clr_timeout_flag();
    check_progress(&tim, &tim_ch);
    tim.clr_timeout_flag();
    tim_ch.clr_compare_flag();
    check_progress(&tim, &tim_ch);    
    
    // tim.set_enabled(false);
    tim.pcc_disable();

    println!("[pass] FTM OK");
}

fn test_lptmr() {
    use board::hal::pcc;
    use board::hal::lptmr::*;

    fn check_progress(tim: &LptmrPeriph) {
        let mut c_max = 0;
        while !tim.timeout_flag() {
            let c = tim.counter();
            if c > c_max {
                c_max = c;
            }
        }
        assert!(tim.compare_flag());
        assert!(c_max > 0);
    }

    let tim = LPTMR0;
    tim
        .pcc_set_clock_source(pcc::ClockSource::SOSCDIV2)
        .pcc_set_clock_divider(pcc::ClockDivider::Div1)
        .pcc_set_clock_divider_frac(pcc::ClockDividerFrac::Frac0)
        .pcc_enable();
    tim.with_csr(|r| r.set_ten(0));
    // select PCC clock as input
    tim.with_psr(|r| r.set_pbyp(1).set_pcs(0b11));

    // Repeat Up Counter    
    tim
        .set_compare(2048)
        .clr_timeout_flag()
        .start_up(4096);
    check_progress(&tim);
    tim.clr_compare_flag().clr_timeout_flag();
    check_progress(&tim);
    
    // tim.set_enabled(false);
    tim.pcc_disable();

    println!("[pass] FTM OK");
}

fn test_systick() {
    use board::hal::systick;

    let reload_value = 1000;

    // println!("# Disable Systick");
    systick::set_enabled(false);
    assert!(!systick::enabled());

    // println!("# Set Reload Value");
    systick::set_reload_value(reload_value);
    assert_eq!(systick::reload_value(), reload_value);

    // println!("# Set Current Value");
    systick::set_current_value(0);
    assert_eq!(systick::current_value(), 0);

    // println!("# Clear Count Flag");
    let _ = systick::count_flag();
    assert!(!systick::count_flag());


    let mut value_min = systick::current_value();

    // println!("# Start Test");
    systick::set_enabled(true);
    assert!(systick::current_value() > 0);

    while !systick::count_flag() {
        let v = systick::current_value();
        if v < value_min {
            value_min = v;
        }
    }
    assert!(value_min < reload_value);
    systick::set_enabled(false);

    println!("[pass] SYSTICK OK");
}
fn test_adc() {
    use board::common::bits::*;
    use board::hal::pcc;
    use board::hal::adc::*;

    let adc0 = ADC0;
    let ch0 = ADC0_TEMP;    
    let ch1 = ADC0_BANDGAP;
    let ch2 = ADC0_REFSH;
    let ch3 = ADC0_REFSL;

    adc0
        .pcc_set_clock_source(pcc::ClockSource::SPLLDIV2)
        .pcc_enable();

    let v: U12 = ch0.analog_read();
    println!("# ADC0_TEMP:    {}", v);
    // Arbitrary bounds - find formula
    // assert!(v.value() > 550 && v.value() < 650);

    let v: U12 = ch1.analog_read();
    println!("# ADC0_BANDGAP: {}", v);
    // Arbitrary bounds - find formula
    assert!(v.value() > 800 && v.value() < 900);

    let v: U12 = ch2.analog_read();
    println!("# ADC0_REFSH:   {}", v);
    assert!(v.value() == 4095);

    let v: U12 = ch3.analog_read();
    println!("# ADC0_REFSL:   {}", v);
    assert!(v.value() == 0);

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

/// Jumper PTA11(D0) to PTA17(D1)
fn test_irq() {
    use board::hal::port::*;
    use board::hal::gpio::*;

    let port = PORTA;
    let port_out = PTA11;
    let port_in = PTA17;
    let gpio_out = port_out.gpio_pin();
    let gpio_in = port_in.gpio_pin();

    port.pcc_enable();
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

fn test_lpuart() {
    use board::hal::pcc;
    use board::hal::lpuart::*;
    use board::hal::clock::Clock;
    use board::clock::CLK;

    let uart_baud = 115_200;
    let uart = LPUART0;

    uart.pcc_set_clock_source(pcc::ClockSource::SPLLDIV2);
    let sbr = uart.clock(&CLK).unwrap() / (uart_baud << 4);    
    uart
        .set_config(|c| c.set_baud_divisor(sbr.into()))    
        .with_ctrl(|r| r.set_loops(1))
        .enable();

    let src = b"Hello World\r\n";
    let mut dst = [0u8; 13];

    assert!(src.len() == dst.len());
    for i in 0..src.len() {
        uart.wait_tx().tx(src[i]);
        dst[i] = uart.wait_rx().rx();
    }
    assert_eq!(src, &dst);
    println!("[pass] LPUART OK");
}

// NOTE: Board must be powered by 12V to use UJA1169
// Without power, all registers will read 0xff

// SPLLDIV2 = 40MHz
// Prescale = Divide by 4 => 10MHz
// SCKDIV = 8 => Divide by 10 => 1MHz
fn test_lpspi() {
    use board::hal::lpspi::*;
    use board::uja1169::Mode;
    use board::hal::pcc::*;
    use board::hal::port::*;


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


    t.configure();

    println!("[pass] LPSPI OK");
}