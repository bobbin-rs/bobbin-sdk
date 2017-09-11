#![no_std]
#![no_main]
#![allow(dead_code)]

#[macro_use]
extern crate nucleo_f303re as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("[start] Running tests for nucleo-f303re");
    test_crc();
    test_systick();
    // test_adc();
    test_dma();
    // test_exti();
    // test_lpuart();
    // test_usart();
    // test_spi();
    test_i2c();
    println!("[done] All tests passed");
    loop {}
}

fn test_crc() {
    use board::hal::crc::*;

    let crc = CRC;

    // println!("# Setting up CRC");

    crc.rcc_enable();
    crc.configure(Config::default()).initialize(0x1234);

    // println!("# Starting CRC");

    let expect = [0x00001234, 0x77951e50, 0x24934150, 0x87e34974];

    for i in 0..4 {
        assert_eq!(crc.read(), expect[i]);
        crc.write(i as u32);
    }

    crc.rcc_disable();

    println!("[pass] CRC OK");    
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

// fn test_adc() {
//     use board::hal::adc::*;

//     let adc = ADC1;
//     let adc_temp = ADC1_TEMP;
//     let adc_ref = ADC1_REFINT;

//     adc.rcc_enable();
//     adc.init();
//     adc.with_ccr(|r| r.set_tsen(1).set_vrefen(1));
//     adc.with_smpr(|r| r.set_smp(0b111));

//     let t: u8 = <AdcCh as AnalogRead<u8>>::start(&adc_temp).analog_read();
//     let v: u8 = <AdcCh as AnalogRead<u8>>::start(&adc_ref).analog_read();

//     println!("# t: {} v: {}", t, v);

//     // assert!(t > 110 && t < 130);
//     // assert!(v > 220 && t < 240);


//     adc.rcc_disable();

//     println!("[pass] ADC OK")
// }

fn test_dma() {
    use board::hal::dma::*;    
    let src = [0xffu8; 1024];
    let dst = [0u8; 1024];
    
    let dma = DMA1;
    let dma_ch = DMA1_CH1;

    // ch.irq_dma().set_enabled(true);

    dma.rcc_enable();
    
    dma_ch    
        .set_pa(&src as *const u8 as u32)
        .set_ma(&dst as *const u8 as u32)
        .set_psize(Size::Bit8)
        .set_pinc(true)
        .set_msize(Size::Bit8)
        .set_minc(true)
        .set_mem2mem(true)
        .set_ndt(1024)
        .set_tcie(true)
        .clr_teif()
        .clr_tcif();

    dma_ch.clr_tcif().set_enabled(true);

    while !dma_ch.tcif() {}

    for i in 0..1024 {
        assert_eq!(src[i], dst[i]);
    }    

    dma.rcc_disable();
    println!("[pass] DMA OK");
}

// /// Pin PA10 / D0 must be connected to Pin PA9 / D1
// fn test_exti() {
//     use board::hal::gpio::*;
//     use board::hal::syscfg::*;
//     use board::hal::exti::*;    

//     let port = GPIOA;
//     let port_out = PA10; // D0
//     let port_in = PA9; // D1
//     let line = EXTI_LINE9;

//     port.rcc_enable();
//     port_out.mode_output().set_output(false);
//     port_in.mode_input();

//     SYSCFG.rcc_enable();
//     SYSCFG.set_exti(line.index, Source::GpioA);
    
//     line.set_interrupt_mask(true);
//     line.set_rising_trigger(true);
//     line.set_falling_trigger(true);

//     // Test for rising edge trigger
//     line.clr_pending(); 
//     assert_eq!(line.pending(), false);
//     port_out.set_output(true);
//     assert_eq!(line.pending(), true);    

//     // Test for falling edge trigger
//     line.clr_pending(); 
//     port_out.set_output(false);
//     assert_eq!(line.pending(), true);    
//     line.clr_pending(); 

//     SYSCFG.rcc_disable();

//     println!("[pass] EXTI OK");
// }

// fn test_lpuart() {
//     use board::console;
//     use board::clock::{CLK, Clock};
//     use board::hal::gpio::*;
//     use board::hal::lpuart::*;

//     let uart = LPUART1;
//     let port = GPIOA;
//     let tx = PA2;

//     let f_ck = uart.clock(&CLK).unwrap();

//     board::delay(1);
//     console::disable();
   
//     uart.rcc_enable();
//     port.rcc_enable();

//     tx.mode_tx(&uart);

//     uart.with_config(|c| c.set_baud(115200, f_ck));
//     // uart.set_brr(|r| r.set_brr(71_111));
//     uart.with_cr3(|r| r.set_hdsel(1));
//     uart.set_enabled(true);

//     let src = b"# ABCD\r\n";;
//     let mut dst = [0u8; 8];

//     if uart.can_rx() {
//         let _ = uart.rx();
//     }

//     for i in 0..src.len() {
//         uart.putc(src[i]);
//         while !uart.isr().test_tc() {}
//         if uart.can_rx() {
//             dst[i] = uart.rx();
//         }
//     }
//     // while uart.isr().test_busy() {}
//     uart.rcc_disable();
//     console::init();
//     // println!("# src: {:?}", src);
//     // println!("# dst: {:?}", dst);
//     assert_eq!(src, &dst);
//     println!("[pass] LPUART OK");
// }


// fn test_usart() {
//     use board::console;
//     use board::clock::{CLK, Clock};
//     use board::hal::gpio::*;
//     use board::hal::usart::*;

//     let uart = USART2;
//     let port = GPIOA;
//     let tx = PA2;

//     let f_ck = uart.clock(&CLK).unwrap();

//     board::delay(1);
//     console::disable();
   
//     uart.rcc_enable();
//     port.rcc_enable();

//     tx.mode_tx(&uart);

//     uart.with_config(|c| c.set_baud(115200, f_ck));
//     // uart.set_brr(|r| r.set_brr(71_111));
//     uart.with_cr3(|r| r.set_hdsel(1));
//     uart.set_enabled(true);

//     let src = b"# ABCD\r\n";;
//     let mut dst = [0u8; 8];

//     if uart.can_rx() {
//         let _ = uart.rx();
//     }

//     for i in 0..src.len() {
//         uart.putc(src[i]);
//         while !uart.isr().test_tc() {}
//         if uart.can_rx() {
//             dst[i] = uart.rx();
//         }
//     }
//     // while uart.isr().test_busy() {}
//     uart.rcc_disable();
//     console::init();
//     // println!("# src: {:?}", src);
//     // println!("# dst: {:?}", dst);
//     assert_eq!(src, &dst);
//     println!("[pass] USART OK");
// }

// /// PA6(A5) and PA7(A6) must be jumpered together for loopback.
// fn test_spi() {
//     use board::hal::gpio::*;
//     use board::hal::spi::*;

//     let spi = SPI1;
//     let port = GPIOA;
//     let spi_miso = PA6; // A5
//     let spi_mosi = PA7; // A6
//     let spi_sck = PA5;

//     spi.rcc_enable();
//     port.rcc_enable();

//     // NOTE: Pins must be set with output speed HIGH or leading edge
//     // of transmission will occasionally be missed.

//     spi_miso.mode_spi_miso(&spi).speed_high().pull_up();
//     spi_mosi.mode_spi_mosi(&spi).speed_high().push_pull();
//     spi_sck.mode_spi_sck(&spi).speed_high().push_pull();

//     spi.set_config(|cfg| cfg
//         .set_frame_size(FrameSize::Bits8)
//         .set_master(true)
//         .set_baud_divider(0b0.into())
//     );

//     spi.set_output_enabled(true).set_enabled(true);

//     let src: [u8; 8] = [0xde, 0xad, 0xbe, 0xef, 0x12, 0x34, 0x56, 0x78];
//     let mut dst = [0u8; 8];

//     let mut i = 0;
//     let mut j = 0;
//     loop {
//         if i < src.len() && spi.can_tx() {
//             spi.tx(src[i]);
//             i += 1;
//         }
//         if j < dst.len() && spi.can_rx() {
//             dst[j] = spi.rx();
//             j += 1;
//         }
//         if j == dst.len() {
//             break;
//         }        
//     }
//     // println!("# src: {:?}", src);
//     // println!("# dst: {:?}", dst);
//     assert_eq!(src, dst);
    
//     spi.set_enabled(false);
//     spi.rcc_disable();

//     println!("[pass] SPI OK");
// }

fn test_i2c() {
    use board::hal::gpio::*;
    use board::hal::i2c::*;
    use board::common::bits::*;

    let addr: U7 = U7::from(0x60);

    let i2c = I2C1;
    let i2c_port = GPIOB;
    let i2c_scl = PB8; // D15
    let i2c_sda = PB9; // D14

    i2c.rcc_enable();
    i2c_port.rcc_enable();

    // Attached to MPL3115A2 
    // NOTE: SCL and SCA must have pull-up resistors.
    // NOTE: HSI Clock must be enabled.

    i2c_scl.mode_i2c_scl(&i2c).open_drain();
    i2c_sda.mode_i2c_sda(&i2c).open_drain();

    println!("# Configuring I2C");

    // i2c.set_config(|c| c.set_timing(0x8.into(), 0x3.into(), 0x0.into(), 0xd.into(), 0x12.into()));
    i2c.set_enabled(false);
    println!("enabled");
    // i2c.set_timingr(|_| Timingr(0x00300619));
    i2c.set_timingr(|r| r
        .set_presc(0x0)
        .set_scldel(0x3)
        .set_sdadel(0x0)
        .set_sclh(0xF)
        .set_scll(0x12)
    );
    println!("CR1 {:?}", i2c.cr1());
    println!("CR2 {:?}", i2c.cr2());
    println!("TIMINGR {:?}", i2c.timingr());
    println!("read_reg");
    assert_eq!(i2c.read_reg(addr, 0x0c), 0xc4);
    
    println!("Mode:  0x{:08x}", i2c.read_reg(addr, 0x26));
    

    i2c.write_reg(addr, 0x26, 0xb8); // OSR = 128
    i2c.write_reg(addr, 0x13, 0x06); // Enable Data Flags
    i2c.write_reg(addr, 0x26, 0xb9); // Set Active
    // println!("Mode:  0x{:08x}", i2c.read_reg(addr, 0x26));

    loop {
        while i2c.read_reg(addr, 0x00) != 0x04 {}    
        let mut buf = [0u8; 5];
        i2c.transfer(addr, &[0x01], &mut buf);
        println!("# {:?}", buf);
        // assert!(buf[0] == 0 && buf[1] != 0 && buf[2] != 0 && buf[3] != 0 && buf[4] != 0);
        break
    }

    

    i2c_port.rcc_disable();
    i2c.rcc_disable();
    println!("[pass] I2C OK");
}
