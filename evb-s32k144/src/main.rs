#![no_std]
#![no_main]

#[macro_use]
extern crate evb_s32k144 as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("[start] Running tests for evb-s32k144");
    test_gpio();
    println!("[done] All tests passed");
    loop {}
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