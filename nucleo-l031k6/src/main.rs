#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_l031k6 as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::console::init();
    println!("[start] Running tests for nucleo-l031K6");
    test_crc();
    test_gpio();
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