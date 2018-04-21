#![no_std]
#![no_main]

#[macro_use]
extern crate openmv_m7 as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("[start] Running tests for nucleo-f429zi");
    // test_crc();
    // test_systick();
    // test_dma();
    // test_adc();
    test_i2c();
    // test_spi_lora();
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
    use board::hal::systick::*;

    println!("# Testing SYSTICK");
    test_systick(&SYSTICK, ClockSource::Internal);
    println!("[pass] SYSTICK OK");
}


fn test_dma() {
    use board::hal::dma::*;    
    
    let dma = DMA2;
    let dma_ch = DMA2_STREAM0;

    dma.rcc_enable();

    let src = [0xffu8; 1024];
    let dst = [0u8; 1024];

    dma_ch    
        .set_pa(&src as *const u8 as u32)
        .set_m0a(&dst as *const u8 as u32)
        .set_psize(Size::Bit8)
        .set_pinc(true)
        .set_msize(Size::Bit8)
        .set_minc(true)
        .set_dir(Dir::MtoM)
        .set_ndt(1024)
        .set_tcie(true)
        .clr_teif()
        .clr_tcif();

    println!("Starting DMA Transfer");
    dma_ch.clr_tcif().set_enabled(true);
    while !dma_ch.tcif() {}
    for i in 0..1024 {
        assert_eq!(src[i], dst[i]);
    }

    dma.rcc_disable();
    println!("[pass] DMA OK");
}

fn test_adc() {
    use board::chip::c_adc::*;
    use board::hal::adc::*;

    let adc = ADC1;
    let adc_temp = ADC1_TEMP;
    let adc_ref = ADC1_REF;

    adc.rcc_enable();
    adc.set_enabled(true).calibrate();
    C_ADC.with_ccr(|r| r.set_tsvrefe(1));
    // adc.with_smpr(|r| r.set_smp(0b111));
    

    let t: u8 = <AdcCh as AnalogRead<u8>>::start(&adc_temp).analog_read();
    let v: u8 = <AdcCh as AnalogRead<u8>>::start(&adc_ref).analog_read();

    println!("# t: {} v: {}", t, v);

    // assert!(t > 110 && t < 130);
    // assert!(v > 220 && t < 240);


    adc.rcc_disable();

    println!("[pass] ADC OK")
}

#[allow(dead_code)]
fn test_i2c() {
    use board::clock::CLK;
    use board::hal::clock::Clock;
    use board::hal::gpio::*;
    use board::hal::tim_adv::*;
    use board::hal::i2c::*;
    use board::common::bits::*;

    let addr: U7 = U7::from(0x21);
    // let addr: U7 = U7::from(0x15);

    let dcmi_pwdn = PB5;
    let dcmi_rst = PA10;
    let dcmi_hsync = PA4;
    let dcmi_vsync = PB7;
    let dcmi_pxclk = PA6;

    dcmi_pwdn.port().rcc_enable();
    dcmi_rst.port().rcc_enable();
    dcmi_hsync.port().rcc_enable();
    dcmi_vsync.port().rcc_enable();
    dcmi_pxclk.port().rcc_enable();

    dcmi_pwdn.mode_output();
    dcmi_rst.mode_output();
    dcmi_rst.set_output(true);

    dcmi_hsync.mode_input();
    dcmi_vsync.mode_input();
    dcmi_pxclk.mode_input();
    
    println!("DCMI_PWDN_HIGH");
    dcmi_pwdn.set_output(true);
    board::delay(10);
    dcmi_pwdn.set_output(false);
    println!("DCMI_PWDN_LOW");
    board::delay(10);

    // let tim = TIM4;
    // let tim_ch = TIM4_CH1;
    // let tim_pin = PD12;

    let tim = TIM1;
    let tim_ch = TIM1_CH1;
    let tim_pin = PA8;


    tim.rcc_enable();
    tim_pin.port().rcc_enable();
    tim_pin.mode_tim(&tim_ch);

    let frequency = 9_000_000;
    let tclk = tim.clock(&CLK).unwrap();
    let period = (tclk / frequency) - 1;
    let compare = ((period + 1) / 2) - 1;
    // let compare = 3;
    println!("tclk: {} freq: {} period: {} comp: {}", tclk, frequency, period, compare);

    // TIM1 Input Clock = 216MHz
    // TIM1 Prescaler = 24
    // TIM1 Output = 9MHz

    /* TCLK (PCLK * 2) */
    // int tclk  = DCMI_TIM_PCLK_FREQ() * 2;

    // /* Timer base configuration */
    // TIMHandle.Instance          = DCMI_TIM;
    // TIMHandle.Init.Period       = period;
    // TIMHandle.Init.Prescaler    = 0;
    // TIMHandle.Init.ClockDivision = 0;
    // TIMHandle.Init.CounterMode   = TIM_COUNTERMODE_UP;

    // /* Timer channel configuration */
    // TIM_OC_InitTypeDef TIMOCHandle;
    // TIMOCHandle.Pulse       = period/2;
    // TIMOCHandle.OCMode      = TIM_OCMODE_PWM1;
    // TIMOCHandle.OCPolarity  = TIM_OCPOLARITY_HIGH;
    // TIMOCHandle.OCFastMode  = TIM_OCFAST_DISABLE;
    // TIMOCHandle.OCIdleState = TIM_OCIDLESTATE_RESET;

    tim.set_enabled(false);
    tim.with_cr1(|r| r.set_cms(0b00).set_dir(0));
    tim.with_bdtr(|r| r.set_moe(1));
    tim.set_auto_reload(period);    
    tim_ch.set_output_compare_mode(OcMode::Pwm1);
    tim_ch.set_preload_enable(true);
    tim_ch.set_capture_compare(compare);
    tim_ch.set_capture_compare_enabled(true);
    tim.set_update_event();
    tim.set_enabled(true);
    
    println!("Timer Enabled");

    // loop {
    //     let (hsync, vsync, pxclk) = (dcmi_hsync.input(), dcmi_vsync.input(), dcmi_pxclk.input());
    //     println!("{} {} {}", hsync, vsync, pxclk);
    //     board::delay(100);
    // }



    let i2c = I2C1;
    let i2c_port = GPIOB;
    let i2c_scl = PB8; // D15
    let i2c_sda = PB9; // D14

    i2c.rcc_enable();
    i2c_port.rcc_enable();

    i2c_scl.mode_i2c_scl(&i2c).open_drain();
    i2c_sda.mode_i2c_sda(&i2c).open_drain();

    // i2c.set_config(|c| c.set_timing(0x8.into(), 0x3.into(), 0x0.into(), 0xd.into(), 0x12.into()));
    i2c.set_enabled(false);

    i2c.set_timingr(|_| Timingr(0x20404768));
    // println!("I2C_TIMINGR: {:?}", i2c.timingr());

    // Scan I2C
    println!("Scanning I2C - RST_HIGH");

    dcmi_pwdn.set_output(true);
    println!("DCMI_RST_HIGH");
    board::delay(10);

    // let mut found = false;
    // for i in 0x00..0x7F {   
    //     if is_device_ready(i2c.into(), U7::from(i)) {
    //         found = true;
    //         println!("  0x{:02x}", i);
    //     }
    // }
    // if found {
    //     println!("Found I2C Device");
    // } else {
    //     println!("No I2C Devices Found");
    // }

    // Scan I2C
    println!("Scanning I2C - RST_LOW");

    dcmi_pwdn.set_output(false);
    // println!("DCMI_RST_LOW");
    // board::delay(10);

    // let mut found = false;
    // for i in 0x00..0x7F {   
    //     if is_device_ready(i2c.into(), U7::from(i)) {
    //         found = true;
    //         println!("  0x{:02x}", i);
    //     }
    // }
    // if found {
    //     println!("Found I2C Device");
    // } else {
    //     println!("No I2C Devices Found");
    // }


    fn is_device_ready(i2c: I2cPeriph, addr: U7) -> bool {        
        // Stop and Start Peripheral
        i2c
            .with_cr1(|r| r.set_pe(0))
            .with_cr1(|r| r.set_pe(1));

        // Panic if bus is busy
        if i2c.isr().test_busy() {
            panic!("I2C Bus is Busy");            
        }

        i2c.with_cr2(|r| r.set_sadd(addr.value() << 1).set_autoend(1).set_start(1));
        let mut timeout = 100_000;
        loop {
            if timeout == 0 {
                return false;
            }
            let isr = i2c.isr();            
            if isr.test_stopf() {
                return true
            } else if isr.test_nackf() {
                return false
            }
            timeout -= 1;
        }
        i2c.with_cr1(|r| r.set_pe(0));
    }

    fn master_tx(i2c: I2cPeriph, addr: U7, buf: &[u8]) {
        i2c.set_enabled(true);
        i2c.wait_not_busy();
        i2c.with_cr2(|r| r
            .set_sadd(addr.value() << 1)
            .set_rd_wrn(0)
            .set_nbytes(buf.len() as u8)
            .set_autoend(1)
            .set_start(1));
        for c in buf.iter() {
            i2c.wait_txis();            
            i2c.set_txdr(|r| r.set_txdata(*c));
        }
        i2c.wait_stopf();
        i2c.set_enabled(false);
    }

    fn master_rx(i2c: I2cPeriph, addr: U7, buf: &mut [u8]) {
        i2c.set_enabled(true);
        i2c.wait_not_busy();
        i2c.with_cr2(|r| r
            .set_sadd(addr.value() << 1)
            .set_rd_wrn(1)
            .set_nbytes(buf.len() as u8)
            .set_autoend(1) // Move to after start?
            .set_start(1));
        for i in 0..buf.len() {
            i2c.wait_rxne();
            buf[i] = i2c.rxdr().rxdata().value();
        }
        i2c.wait_stopf();
        i2c.set_enabled(false);
    }

    fn master_xfer(i2c: I2cPeriph, addr: U7, out_buf: &[u8], in_buf: &mut[u8]) {        
        master_tx(i2c, addr, out_buf);
        master_rx(i2c, addr, in_buf);
    }

    fn read_reg(i2c: I2cPeriph, addr: U7, reg: u8) -> u8 {
        let mut cmd = [reg];
        let mut buf = [0u8; 1];
        master_xfer(i2c.into(), addr, &cmd, &mut buf);
        return buf[0]
    }

    fn write_reg(i2c: I2cPeriph, addr: U7, reg: u8, data: u8) {
        let mut cmd = [reg, data];
        let mut buf = [0u8; 1];
        master_tx(i2c.into(), addr, &cmd);
    }

    // i2c.set_timingr(|r| r
    //     .set_presc(0x0)
    //     .set_scldel(0x3)
    //     .set_sdadel(0x0)
    //     .set_sclh(0xF)
    //     .set_scll(0x12)
    // );

    // println!("DCMI_RST_HIGH");
    // dcmi_pwdn.set_output(1);
    // board::delay(10);

    println!("Reading I2C");

    for r in [0x0au8, 0x0b, 0x1c, 0x1d].iter() {
        println!("0x{:02x}: 0x{:02x}", *r, read_reg(i2c.into(), addr, *r));
    }
    println!("");
    // for i in 0..0xAC {
    //     println!("0x{:02x}: 0x{:02x}", i, read_reg(i2c.into(), addr, i));
    // }


    i2c_port.rcc_disable();
    i2c.rcc_disable();
    println!("[pass] I2C OK");
}

// #[allow(dead_code)]
// fn test_spi_lora() {
//     use board::hal::gpio::*;
//     use board::hal::spi::*;

//     let spi = SPI1;
//     let port = GPIOA;
//     let spi_miso = PA6; // A5
//     let spi_mosi = PA7; // A6
//     let spi_sck = PA5;
//     let spi_nss = PD14; // D10

//     spi.rcc_enable();
//     port.rcc_enable();
//     GPIOD.rcc_enable();

//     // NOTE: Pins must be set with output speed HIGH or leading edge
//     // of transmission will occasionally be missed.

//     spi_miso.mode_spi_miso(&spi).speed_high().pull_up();
//     spi_mosi.mode_spi_mosi(&spi).speed_high().push_pull();
//     spi_sck.mode_spi_sck(&spi).speed_high().push_pull();
//     spi_nss.mode_output();

//     spi.set_config(|cfg| cfg
//         .set_frame_size(FrameSize::Bits8)
//         .set_master(true)
//         .set_baud_divider(0b100.into())
//     );

//     spi.set_output_enabled(true).set_enabled(true);


//     let test_data = [(0x42, 0x12), (0x01, 0x09), (0x02, 0x1a), (0x03, 0x0b), (0x04, 0x00), (0x05, 0x52), (0x06, 0x6c)];

//     for &(tx, rx) in test_data.iter() {
//         // println!("0x{:02x}: 0x{:02x}", tx, rx);
//         assert_eq!(reg_read(&spi, &spi_nss, tx), rx);
//     }


//     println!("[pass] SPI OK");
//     spi.rcc_disable();
//     spi_sck.mode_analog();
//     spi_mosi.mode_analog();
//     spi_miso.mode_analog();
//     spi_nss.mode_analog();

//     fn transfer(spi: &SpiPeriph, nss: &GpioPin, src: &[u8], dst: &mut[u8]) {
//         let mut i = 0;
//         let mut j = 0;
//         nss.set_output(false);
//         loop {
//             if i < src.len() && spi.can_tx() {
//                 spi.tx(src[i]);
//                 i += 1;
//             }
//             if j < dst.len() && spi.can_rx() {
//                 dst[j] = spi.rx();
//                 j += 1;
//             }
//             if j == dst.len() {
//                 break;
//             }        
//         }
//         nss.set_output(true);
//     }

//     fn reg_read(spi: &SpiPeriph, nss: &GpioPin, reg: u8) -> u8 {
//         let cmd = [reg, 0xff];
//         let mut buf = [0u8, 0u8];
//         transfer(spi, nss, &cmd, &mut buf);
//         buf[1]
//     }
    
// }
