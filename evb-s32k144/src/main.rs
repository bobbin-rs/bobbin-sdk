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
    // println!("# {:08x} {:08x}", v, 0x4A090E98);
    assert_eq!(v, 0x4A090E98);

    let msg = b"Hello World";
    let v: u32 = CRC.init(CRC_32_START).write(&msg[..]).read();
    // for c in b"Hello World" {
    //     CRC.write(*c);
    // }
    // let v: u32 = CRC.read();
    // println!("# {:08x} {:08x}", v, 0x4A17B156);
    assert_eq!(v, 0x4A17B156);

    // for tot in 0..4 {
    //     for totr in 0..4 {
    //         for fxor in 0..2 {
    //             println!("{} {} {}", tot, totr, fxor);
    //             CRC.set_ctrl(|r| r.set_tcrc(1).set_tot(tot).set_totr(totr).set_fxor(fxor));
    //             CRC.set_gpoly(|_| Gpoly(0x04c11db7u32));
    //             let v: u32 = CRC.poly();
    //             println!("{:08x} {:08x}", v, CRC_32);
    //             CRC.init(0xFFFFFFFFu32);
    //             CRC.write(0x12345678u32);
    //             let v = CRC.data().0;
    //             println!("{:08x} {:08x}", v, 0x4A090E98);
    //             if v == 0x4A090E98 {
    //                 println!("---");
    //                 loop {}
    //             }

    //         }
    //     }
    // }

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