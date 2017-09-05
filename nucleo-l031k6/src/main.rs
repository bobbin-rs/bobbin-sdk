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