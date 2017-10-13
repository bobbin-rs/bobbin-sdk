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
    

    let mut tx_buf = [SpiAction::Idle; 64];
    let mut rx_buf = [0u8; 64];
    let pins: [GpioPin; 1] = [cs.into()];
    let irq = spi.irq_spi();
    let spi = SpiDriver::new(spi, &pins, &mut tx_buf, &mut rx_buf);
    spi.enable_irq(&irq);
    println!("SPI Initialized");
    for r in 0..16 {
        println!("{:02x}: {:02x}", r, spi.reg_read(0, r));
    }

    let mut buf = [0u8; 6];
    spi.transfer(0, &[0x0a, 0x55, 0x55, 0x55, 0x55, 0x55], &mut buf);
    for i in 1..buf.len() {
        print!("{:02x} ", buf[i]);
    }
    println!("");

    let mut buf = [0u8; 6];
    spi.transfer(0, &[0x0b, 0x55, 0x55, 0x55, 0x55, 0x55], &mut buf);
    for i in 1..buf.len() {
        print!("{:02x} ", buf[i]);
    }
    println!("");

    let mut buf = [0u8; 6];
    spi.transfer(0, &[0x10, 0x55, 0x55, 0x55, 0x55, 0x55], &mut buf);
    for i in 1..buf.len() {
        print!("{:02x} ", buf[i]);
    }
    println!("");



    loop {}

}

