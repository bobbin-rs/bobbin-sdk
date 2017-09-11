#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f103rb as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("[start] Running tests for nucleo-f103rb");
    test_crc();
    test_systick();
    test_dma();
    test_adc();    
    test_i2c();
    println!("[done] All tests passed");
    loop {}
}


fn test_crc() {
    use board::hal::crc::*;

    let crc = CRC;

    // println!("# Setting up CRC");

    crc.rcc_enable();
    // println!("# Starting CRC");

    let expect: [u32;4 ] = [0xffffffff, 0xc704dd7b, 0x6dc5a6ee,0x491308c2];


    for i in 0..4 {
        // println!("{:08x}", crc.read());
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

fn test_adc() {
    use board::hal::adc::*;
    use board::common::bits::*;

    let adc = ADC1;
    let adc_temp = ADC1_TEMP;
    let adc_ref = ADC1_REF;

    adc.rcc_enable();
    adc.set_enabled(true).calibrate();
    adc.with_cr2(|r| r.set_tsvrefe(1));
    // adc.with_smpr(|r| r.set_smp(0b111));
    

    let t: U12 = adc_temp.start().analog_read();
    let v: U12 = adc_ref.start().analog_read();

    println!("# t: {} v: {}", t, v);

    // assert!(t > 110 && t < 130);
    // assert!(v > 220 && t < 240);


    adc.rcc_disable();

    println!("[pass] ADC OK")
}

fn test_i2c() {
    use board::hal::gpio::*;
    use board::hal::i2c::*;
    use board::chip::afio::*;
    use board::common::bits::*;

    let addr: U7 = U7::from(0x60);

    let i2c = I2C1;
    let i2c_port = GPIOB;
    let i2c_scl = PB8; // D15
    let i2c_sda = PB9; // D14

    // NOTE: I2C1 is mapped to PB6 / PB7 by default.
    // Remap I2C1 to PB8 / PB8

    AFIO.with_mapr(|r| r.set_i2c1_remap(true));

    i2c.rcc_enable();
    i2c_port.rcc_enable();

    // Attached to MPL3115A2 
    // NOTE: SCL and SCA must have pull-up resistors.
    // NOTE: HSI Clock must be enabled.

    i2c_scl.mode_alt_fn_open_drain();
    i2c_sda.mode_alt_fn_open_drain();

    i2c.set_enabled(false);

    println!("Configure I2c");        
    i2c.with_cr2(|r| r.set_freq(36));
    
    // Set Clock Speed = 100khz (divisor = 36Mhz / 100khz = 360)
    i2c.set_ccr(|_| Ccr(0).set_f_s(0).set_duty(0).set_ccr(360));

    // // Set Rise Time Register = 43
    i2c.set_trise(|_| Trise(0).set_trise(43));

    println!("Resetting Bus");
    for _ in 0..10 {
        i2c.with_cr1(|r| r.set_start(1));
        i2c.with_cr1(|r| r.set_start(0));
    }

    println!("Read Reg");
    assert_eq!(i2c.read_reg(addr, 0x0c), 0xc4);
   
    // println!("Mode:  0x{:08x}", i2c.read_reg(addr, 0x26));
    

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

    
    i2c_scl.mode_analog();
    i2c_sda.mode_analog();

    i2c.rcc_disable();
    println!("[pass] I2C OK");
}
