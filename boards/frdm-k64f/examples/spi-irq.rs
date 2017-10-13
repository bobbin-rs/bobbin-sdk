#![no_std]
#![no_main]

#[macro_use]
extern crate frdm_k64f as board;

use board::hal::spi::*;
use board::hal::port::*;
use board::hal::gpio::*;

use board::common::bits::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();    
    println!("Running SPI Driver");

    let port = PORTD;
    let port_sck = PTD1; // D13
    let port_sout = PTD2; // D12
    let port_sin = PTD3; // D11
    let port_pcs0 = PTD0; // D10

    let spi = SPI0;

    port.sim_enable();
    port_sck.mode_spi_sck(&spi);
    port_sout.mode_spi_sout(&spi);
    port_sin.mode_spi_sin(&spi);
    // port_pcs0.mode_spi_pcs0(&spi);

    port_pcs0.set_mux_gpio();

    let cs = port_pcs0.gpio_pin();
    cs.set_dir_output().set_output(true);

    spi.sim_enable();
    spi.init(0b1000, 0b00);
    
    println!("SPI Initialized");

    for r in 0..16 {
        cs.set_output(false);
        println!("{:02x}: {:02x}", r, reg_read(&spi, r));
        cs.set_output(true);
    }

    cs.set_output(false);
    let mut buf = [0u8; 6];
    spi.transfer(&[0x0a, 0x55, 0x55, 0x55, 0x55, 0x55], &mut buf);
    cs.set_output(true);
    for i in 1..buf.len() {
        print!("{:02x} ", buf[i]);
    }
    println!("");

    cs.set_output(false);
    let mut buf = [0u8; 6];
    spi.transfer(&[0x0b, 0x55, 0x55, 0x55, 0x55, 0x55], &mut buf);
    cs.set_output(true);
    for i in 1..buf.len() {
        print!("{:02x} ", buf[i]);
    }
    println!("");

    cs.set_output(false);
    let mut buf = [0u8; 6];
    spi.transfer(&[0x10, 0x55, 0x55, 0x55, 0x55, 0x55], &mut buf);
    cs.set_output(true);
    for i in 1..buf.len() {
        print!("{:02x} ", buf[i]);
    }
    println!("");



    loop {}

}

fn reg_read(spi: &SpiPeriph, reg: u8) -> u8 {
    let cmd = [reg];
    let mut buf = [0u8];
    spi.write(&cmd);   
    spi.read(&mut buf);
    buf[0]
}
