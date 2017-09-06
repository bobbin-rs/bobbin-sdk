#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_l031k6 as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("[start] Running tests for nucleo-l031K6");
    test_crc();
    test_gpio();
    test_lptim();
    test_systick();
    test_adc();
    test_dma();
    test_exti();
    test_lpuart();
    println!("[done] All tests passed");
    loop {}
}

/// Pin PA10 / D0 must be connected to Pin PA9 / D1
fn test_gpio() {
    use board::hal::gpio::*;
    let port = GPIOA;
    let port_out = PA10; // D0
    let port_in = PA9; // D1

    // println!("# Setting up GPIO");

    port.rcc_enable();
    port_out.mode_output();
    port_in.mode_input();

    // println!("# Starting GPIO");

    port_out.set_output(false);
    assert_eq!(port_out.output(), false);
    assert_eq!(port_in.input(), false);

    port_out.set_output(true);
    assert_eq!(port_out.output(), true);
    assert_eq!(port_in.input(), true);

    port_out.toggle_output();
    assert_eq!(port_out.output(), false);
    assert_eq!(port_in.input(), false);

    port_out.mode_analog();
    port_in.mode_analog();

    println!("[pass] GPIO OK");
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

fn test_lptim() {
    use board::hal::lptim::*;

    fn check_progress(tim: Lptim) {
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

    let tim = LPTIM;
    tim.rcc_enable();
    tim.set_enabled(true);

    // Repeat Up Counter

    tim
        .set_compare(512)
        .clr_compare_flag()
        .clr_timeout_flag()
        .start_up(1024);
    check_progress(tim);
    tim
        .clr_compare_flag()
        .clr_timeout_flag();
    check_progress(tim);    
    
    // Single Up Counter
    tim
        .set_compare(512)
        .clr_compare_flag()
        .clr_timeout_flag()
        .start_up_once(1024);
    check_progress(tim);   

    tim.set_enabled(false);
    tim.rcc_disable();

    println!("[pass] LPTIM OK");
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
    use board::hal::adc::*;

    let adc = ADC1;
    let adc_temp = ADC1_TEMP;
    let adc_ref = ADC1_REFINT;

    adc.rcc_enable();
    adc.init();
    adc.with_ccr(|r| r.set_tsen(1).set_vrefen(1));
    adc.with_smpr(|r| r.set_smp(0b111));

    let t: u8 = <AdcCh as AnalogRead<u8>>::start(&adc_temp).analog_read();
    let v: u8 = <AdcCh as AnalogRead<u8>>::start(&adc_ref).analog_read();

    println!("# t: {} v: {}", t, v);

    assert!(t > 110 && t < 130);
    assert!(v > 220 && t < 240);


    adc.rcc_disable();

    println!("[pass] ADC OK")
}

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

/// Pin PA10 / D0 must be connected to Pin PA9 / D1
fn test_exti() {
    use board::hal::gpio::*;
    use board::hal::syscfg::*;
    use board::hal::exti::*;    

    let port = GPIOA;
    let port_out = PA10; // D0
    let port_in = PA9; // D1
    let line = EXTI_LINE9;

    port.rcc_enable();
    port_out.mode_output().set_output(false);
    port_in.mode_input();

    SYSCFG.rcc_enable();
    SYSCFG.set_exti(line.index, Source::GpioA);
    
    line.set_interrupt_mask(true);
    line.set_rising_trigger(true);
    line.set_falling_trigger(true);

    // Test for rising edge trigger
    line.clr_pending(); 
    assert_eq!(line.pending(), false);
    port_out.set_output(true);
    assert_eq!(line.pending(), true);    

    // Test for falling edge trigger
    line.clr_pending(); 
    port_out.set_output(false);
    assert_eq!(line.pending(), true);    
    line.clr_pending(); 

    SYSCFG.rcc_disable();

    println!("[pass] EXTI OK");
}

/// PA3(A2) must be connected to PA2(A7)
fn test_lpuart() {
    use board::console;
    use board::clock::{CLK, Clock};
    use board::hal::gpio::*;
    use board::hal::lpuart::*;

    let uart = LPUART1;
    let port = GPIOA;
    let rx = PA3; // A2
    let tx = PA2; // A7

    let f_ck = uart.clock(&CLK).unwrap();
    println!("Clock: {:?}", f_ck);



    board::delay(10);
    console::disable();

    
    uart.rcc_enable();
    port.rcc_enable();

    rx.mode_rx(&uart);
    tx.mode_tx(&uart);

    // uart.with_config(|c| c.set_baud(115200, f_ck));
    // uart.with_config(|c| c.set_brr(13u32.into()));
    uart.set_brr(|r| r.set_brr(71_111));
    // uart.with_cr3(|r| r.set_hdsel(1));
    uart.set_enabled(true);

    let src = [b'a', b'b', b'c', b'd'];
    let mut dst = [0u8; 4];

    if uart.can_rx() {
        let _ = uart.rx();
    }

    for i in 0..4 {
        uart.putc(src[i]);
        while !uart.isr().test_tc() {}
        if uart.can_rx() {
            dst[i] = uart.rx();
        }
    }
    // while uart.isr().test_busy() {}

    uart.rcc_disable();
    console::init();
    println!("src: {:?}", src);
    println!("dst: {:?}", dst);
    println!("[pass] LPUART OK");
}