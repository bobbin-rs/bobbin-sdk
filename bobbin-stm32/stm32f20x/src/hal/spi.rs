pub use ::chip::spi::*;
//use ::chip::gpio::*;
use ::hal::rcc;
//use ::hal::gpio;

pub const AF_SPI: u32 = 5;



pub fn configure(mut spi: Spi) {
    trace!("spi::configure");

    // // Configure Pins

    // let port_pins = match spi {
    //     SPI1 => [
    //         (GPIOA, 5),
    //         (GPIOA, 6),
    //         (GPIOA, 7),
    //     ],
    //     _ => unimplemented!()
    // };

    // for &(port, pin) in port_pins.iter() {
    //     rcc::set_gpio_enabled(port, true);
    //     // GPIO Mode = AF
    //     gpio::set_mode(port, pin, 0b10);
    //     // GPIO AF = AF_SPI (5)
    //     gpio::set_af(port, pin, AF_SPI);
    //     // GPIO Output Type = Push-Pull
    //     gpio::set_otype(port, pin, 0);
    //     // GPIO Speed = 50MHZ
    //     gpio::set_ospeed(port, pin, 0b10);
    //     // GPIO Pullup-Pulldown = NoPull
    //     gpio::set_pupd(port, pin, 0b00);
    // }

    // Enable SPI Clock

    rcc::set_spi_enabled(spi, true);

    // Configure SPI
    
    //let br: u32 = 0b001; // clk / 4
    let br: u32 = 0b000; // clk / 2
    let cpol: u32 = 1;
    let cpha: u32 = 1;    
    let lsbfirst: u32 = 0;

    unsafe {
        spi.set_cr1(Cr1(0)
            .set_mstr(1)
            .set_ssm(1)
            .set_br(br)
            .set_cpol(cpol)
            .set_cpha(cpha)            
            .set_lsbfirst(lsbfirst));

        // set TI mode?

        spi.set_cr2(Cr2(0)
            .set_ssoe(1)
        );
    }
}

pub fn enable(mut spi: Spi) {
    trace!("spi::enable");
    unsafe {
        spi.with_cr1(|r| r.set_spe(1));
    }
}

pub fn write(mut spi: Spi, data: u8) {
    //trace!("spi::write\n   0x{:02x}", data);
    unsafe {
        while spi.sr().txe() == 0 {}
        spi.set_dr(Dr(0).set_dr(data as u32));
    }
}

pub fn read(spi: Spi) -> u8 {
    trace!("spi::read");
    unsafe {
        while spi.sr().rxne() == 0 {}
        let v = spi.dr().dr() as u8;
        trace!("   0x{:02x}", v);
        v
    }
}