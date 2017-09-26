#![no_std]
#![no_main]

#[macro_use]
extern crate frdm_kl26z as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("[start] Running tests for frdm-kl26z");
    test_gpio();
    test_irq();
    test_systick();
    test_tpm();
    test_pit();
    test_lptmr();
    test_adc();
    test_i2c();
    // test_spi();
    println!("[done] All tests passed");
    loop {}
}

/// Jumper PTA4(D0) to PTA5(D1)
fn test_gpio() {
    use board::hal::port::*;
    use board::hal::gpio::*;

    let port = PORTA;
    let port_out = PTA4;
    let port_in = PTA5;
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

/// Jumper PTA4(D0) to PTA5(D1)
fn test_irq() {
    use board::hal::port::*;
    use board::hal::gpio::*;

    let port = PORTA;
    let port_out = PTA4;
    let port_in = PTA5;
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

fn test_systick() {
    use board::hal::systick::*;

    println!("# Testing SYSTICK");
    test_systick(&SYSTICK, ClockSource::Internal);
    println!("[pass] SYSTICK OK");
}

fn test_tpm() {
    
}


fn test_pit() {
    use board::hal::pit::*;

    fn check_progress(tim: &PitCh) {
        let mut c_min = 4096;
        while !tim.test_timeout() {
            let c = tim.counter();
            if c < c_min {
                c_min = c;
            }
        }
        assert!(c_min < 4096);
    }


    let tim = PIT;
    let tim_ch = PIT_CH0;
    tim.sim_enable();

    // Repeat Up Counter    
    tim_ch
        .clr_timeout()
        .start_down(4096);
    check_progress(&tim_ch);
    tim_ch.clr_timeout();
    check_progress(&tim_ch);

    assert!(tim_ch.running());
    tim_ch.stop();

    tim.sim_disable();

    println!("[pass] PIT OK");
}

fn test_lptmr() {
    use board::hal::lptmr::*;

    fn check_progress(tim: &LptmrPeriph) {
        let mut c_max = 0;
        while !tim.test_timeout() {
            let c = tim.counter();
            if c > c_max {
                c_max = c;
            }
        }
        assert!(tim.test_compare());
        assert!(c_max > 0);
    }

    let tim = LPTMR0;
    tim.sim_enable();
    tim.with_csr(|r| r.set_ten(0));
    tim.with_psr(|r| r.set_pbyp(1).set_pcs(0b11));

    // Repeat Up Counter    
    tim
        .set_compare(2048)
        .clr_timeout()
        .start_up(4096);
    check_progress(&tim);
    tim.clr_compare().clr_timeout();
    check_progress(&tim);
    
    // tim.set_enabled(false);
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
    assert!(v.value() > 4090);

    let v: U12 = ch3.analog_read();
    println!("# ADC0_REFSL:   {}", v);
    assert!(v.value() < 5);

    adc.sim_disable();

    println!("[pass] ADC OK");
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


    i2c.init(mult, icr);

    let cmd: [u8; 1] = [0xd];
    let mut buf = [0u8];
    i2c.transfer(I2CADDR, &cmd, &mut buf);
    assert_eq!(buf[0], 0xc7);

    i2c.sim_disable();
    println!("[pass] I2C OK");

}


// /// Jumper PTD2(D12) and PTD3(D13)
// fn test_spi() {
//     pub use board::hal::spi::*;
//     pub use board::hal::port::*;
//     pub use board::hal::gpio::*;


//     let port = PORTD;
//     let port_sck = PTD1; // D13
//     let port_sout = PTD2; // D12
//     let port_sin = PTD3; // D11
//     let port_pcs0 = PTD0; // D10

//     let spi = SPI0;

//     port.sim_enable();
//     port_sck.mode_spi_sck(&spi);
//     port_sout.mode_spi_miso(&spi);
//     port_sin.mode_spi_mosi(&spi);
//     port_pcs0.mode_spi_pcs0(&spi);

//     println!("Enabling SPI");

//     spi.sim_enable();
//     println!("Enabling SPI");
//     spi.init(0b1000, 0b00);
//     println!("Enabling SPI");

//     let mut bytes_out = [0u8; 32];
//     let mut bytes_in = [0u8; 32];
//     for (i, b) in bytes_out.iter_mut().enumerate() {
//         *b = i as u8
//     }

//     spi.transfer(&bytes_out, &mut bytes_in);
//     assert_eq!(bytes_out, bytes_in);

//     spi.sim_disable();

//     println!("[pass] SPI OK");
    
// }
