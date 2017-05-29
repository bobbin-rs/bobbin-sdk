pub use ::chip::spi::*;



pub fn configure(mut spi: Spi) {
    let br: u16 = 0b000; // clk / 2
    let cpol: u16 = 0;
    let cpha: u16 = 0;    
    let lsbfirst: u16 = 0;

    unsafe {
        spi.set_cr1(Cr1(0)
            .set_mstr(1)
            .set_ssm(1)
            .set_br(br)
            .set_cpol(cpol)
            .set_cpha(cpha)            
            .set_lsbfirst(lsbfirst));

        spi.set_cr2(Cr2(0)
            .set_ssoe(1)
        );
    }
}

pub fn enable(mut spi: Spi) {
    unsafe {
        spi.with_cr1(|r| r.set_spe(1));
    }
}

pub fn wait_busy(spi: Spi) {
    unsafe {
        while spi.sr().bsy() != 0 {}
    }
}

pub fn write(mut spi: Spi, data: u8) {
    unsafe {        
        while spi.sr().txe() == 0 {}
        spi.set_dr(Dr(0).set_dr(data as u16));
        while spi.sr().bsy() != 0 {}
    }
}

pub fn read(spi: Spi) -> u8 {
    unsafe {
        while spi.sr().rxne() == 0 {}
        let v = spi.dr().dr() as u8;
        v
    }
}

pub fn transfer(mut spi: Spi, data: u8) -> u8 {
    unsafe {
        while spi.sr().txe() == 0 {}
        spi.set_dr(Dr(0).set_dr(data as u16));
        while spi.sr().rxne() == 0 {}
        let v = spi.dr().dr() as u8;
        v
    }
}