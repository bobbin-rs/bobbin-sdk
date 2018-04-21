#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate evb_s32k144 as board;

use board::hal::pcc;
use board::hal::lpi2c::*;
use board::hal::lpspi::*;
use board::hal::port::*;
use board::hal::gpio::*;


#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    PORTA.pcc_enable();
    PORTB.pcc_enable();

    let spi = LPSPI0;

    
    let port_sck = PTB2;
    let port_sin = PTB3;
    let port_sout = PTB4;
    let port_cs = PTB5;    
    let port_g0 = PTD14;
    let port_rst = PTD13;

    println!("# SPI Start");

    port_sck.mode_spi_sck(&spi);
    port_sin.mode_spi_sin(&spi);
    port_sout.mode_spi_sout(&spi);
    // port_cs.mode_spi_pcs0(&spi);

    port_cs.set_mux_gpio();
    port_g0.set_mux_gpio();
    port_rst.set_mux_gpio();

    port_cs.gpio_pin().set_output(true).set_dir_output();
    port_g0.gpio_pin().set_dir_input();    
    
    port_rst.gpio_pin().set_output(true).set_dir_output();

    let cs = port_cs.gpio_pin();
    let _g0 = port_g0.gpio_pin();
    let rst = port_rst.gpio_pin();

    println!("Initialize SPI");

    spi.pcc_set_clock_source(pcc::ClockSource::SPLLDIV2).pcc_set_enabled(true);
    spi.set_enabled(false);    
    
    spi.configure(Config::default()
        .set_master(true)
        .set_clock_config(
            8,
            8,
            9,
            4
        )
        // .sckpcs(4)
        // .pcssck(9)
        // .dbt(8)
        // .sckdiv(8)
        // .txwater(3)        
    );
    spi.with_fcr(|r| r.set_txwater(3));

    spi.set_enabled(true);
    let t = spi.target()
        .cpol(false)
        .cpha(false)
        .prescale(3)
        .pcs(0)
        .framesz(7);
    t.configure();

    println!("SPI Running");

    rst.set_output(false);    

    let irq = spi.irq_lpspi();

    let mut tx_buf = [SpiAction::Idle; 64];
    let mut rx_buf = [0u8; 64];
    let pins: [GpioPin; 1] = [cs.into()];
    let s = SpiDriver::new(spi, &pins, &mut tx_buf, &mut rx_buf);
    s.enable_irq(&irq);

    println!("Reading Version...");
    let v = s.reg_read(0, 0x10);
    println!("Version: 0x{:02x}", v);
    

    loop {}
}

