use ::chip::i2c::*;
use ::hal::rcc;

// I2C_A - PB8 / PB9
// AF4 = I2C 1..3
// I2C1 is on APB1
// APB1 Frequency (default) = 168MHz / 4 = 42MHz

pub const AF_I2C: u32 = 4;

pub fn configure(mut i2c: I2c) {

    // // Configure Pins

    // let (port, p1, p2) = match i2c {
    //     I2C1 => (GPIOB, 8, 9),
    //     _ => unimplemented!()
    // };
    // rcc::set_gpio_enabled(port, true);

    // // GPIO Mode = AF
    // gpio::set_mode(port, p1, 0b10);
    // gpio::set_mode(port, p2, 0b10);
    // // GPIO AF = AF_I2C
    // gpio::set_af(port, p1, AF_I2C);
    // gpio::set_af(port, p2, AF_I2C);    
    // // GPIO Output Type = OD
    // gpio::set_otype(port, p1, 1);
    // gpio::set_otype(port, p2, 1);    
    // // GPIO PullUp / Pulldown = PullUp
    // gpio::set_pupd(port, p1, 0b01);
    // gpio::set_pupd(port, p2, 0b01);
    // // GPIO Speed = High
    // gpio::set_ospeed(port, p1, 0b10);    
    // gpio::set_ospeed(port, p2, 0b10);    

    rcc::set_i2c_enabled(i2c, true);

    // Configure I2C

    unsafe {
        // Set Peripheral Clock Frequency = 42Mhz
        i2c.with_cr2(|r| r.set_freq(42));
        
        // Set Clock Speed = 100khz (divisor = 42Mhz / 100khz = 420)
        i2c.set_ccr(Ccr(0).set_f_s(0).set_duty(0).set_ccr(420));

        // Set Rise Time Register = 43
        i2c.set_trise(Trise(0).set_trise(43));

    }

    // Set ACK when reading = Disabled
    // Set Address Length = 7bit

}

pub fn enable(mut i2c: I2c) {
    unsafe {
        i2c.with_cr1(|r| r.set_pe(1));
    }
}

pub fn write(mut i2c: I2c, addr: u8, data: &[u8]) {
    trace!("write: 0x{:x}, {:?}", addr, data);
    unsafe {
        // Wait until BUSY is clear
        while i2c.sr2().busy() != 0 {}
        
        // Set START
        i2c.with_cr1(|r| r.set_start(1));

        // EV5: SB=1, cleared by reading SR1 followed by writing to DR

        // Wait until START is set
        while i2c.sr1().sb() == 0 {}

        // Send Slave Address
        i2c.set_dr(Dr(0).set_dr((addr as u32) << 1));

        // EV6: ADDR=1, cleared by reading SR1 followed by reading SR2

        // Wait until ADDR is set
        while i2c.sr1().addr() == 0 {}

        // Clear ADDR
        i2c.with_sr1(|r| r.set_addr(0));

        // Read SR2
        let _ = i2c.sr2();

        let mut i = 0;

        while i < data.len() {
            let mut b = data[i];
            trace!("sending {}", b);
            // Wait until TXE is set
            while i2c.sr1().txe() == 0 {}

            // Write data to DR
            i2c.set_dr(Dr(0).set_dr(b as u32));
            i += 1;
            
            if i < data.len() {
                b = data[i];
                // Write data to DR
                i2c.set_dr(Dr(0).set_dr(b as u32));
                i += 1;
            }

            // Wait until BTF is set
            while i2c.sr1().btf() == 0 {}
        }
        // Set STOP
        i2c.with_cr1(|r| r.set_stop(1));        
    }
    trace!("received {:?}", data);
}

pub fn write_read(mut i2c: I2c, addr: u8, cmd: &[u8], data: &mut [u8]) {
    trace!("write_read: 0x{:x}, {:?}, {:?}", addr, cmd, data);
    unsafe {
        // Wait until BUSY is clear
        while i2c.sr2().busy() != 0 {}

        // Set START
        i2c.with_cr1(|r| r.set_start(1));

        // EV5: SB=1, cleared by reading SR1 followed by writing to DR

        // Wait until START is set
        while i2c.sr1().sb() == 0 {}

        // Send Slave Address
        i2c.set_dr(Dr(0).set_dr((addr as u32) << 1));

        // EV6: ADDR=1, cleared by reading SR1 followed by reading SR2

        // Wait until ADDR is set
        while i2c.sr1().addr() == 0 {}

        // Clear ADDR
        i2c.with_sr1(|r| r.set_addr(0));

        // Read SR2
        let _ = i2c.sr2();

        let mut i = 0;

        while i < cmd.len() {
            let mut b = cmd[i];
            trace!("sending {}", b);
            // Wait until TXE is set
            while i2c.sr1().txe() == 0 {}

            // Write data to DR
            i2c.set_dr(Dr(0).set_dr(b as u32));
            i += 1;

            if i < cmd.len() {
                b = cmd[i];
                // Write data to DR
                i2c.set_dr(Dr(0).set_dr(b as u32));
                i += 1;
            }

            // Wait until BTF is set
            while i2c.sr1().btf() == 0 {}
        }    

        // Set START
        i2c.with_cr1(|r| r.set_start(1));

        // EV5: SB=1, cleared by reading SR1 followed by writing to DR

        // Wait until START is set
        while i2c.sr1().sb() == 0 {}

        // Send Slave Address
        i2c.set_dr(Dr(0).set_dr(((addr as u32) << 1) | 1));

        // EV6: ADDR=1, cleared by reading SR1 followed by reading SR2

        // Wait until ADDR is set
        while i2c.sr1().addr() == 0 {}

        // Clear ADDR
        i2c.with_sr1(|r| r.set_addr(0));

        // Read SR2
        let _ = i2c.sr2();

        let mut i = 0;        
        while i < data.len() {
            // if last packet, clear ACK else set ACK
            if i == data.len() - 1 {
                i2c.with_cr1(|r| r.set_ack(0));
            } else {
                i2c.with_cr1(|r| r.set_ack(1));
            }
            
            // Wait until RXnE
            while i2c.sr1().rxne() == 0 {}
            data[i] = i2c.dr().dr() as u8;
            i += 1;
        }

        i2c.with_cr1(|r| r.set_stop(1));

    }
}



pub fn read(_i2c: I2c, _addr: u8, _data: &mut [u8]) {
    unimplemented!()
}
