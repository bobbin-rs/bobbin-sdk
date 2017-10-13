#![no_std]
#![no_main]

#[macro_use]
extern crate frdm_k64f as board;

use board::hal::i2c::*;
use board::hal::port::*;


#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();    
    println!("Running I2c Driver");

    // FXOS8700CQ Accelerometer + Magnetometer

    // I2C = I2C0
    // SCL = PTE24
    // SDA = PTE25
    // I2C_ADDR: 0x1D

    pub const I2C_ADDR: u8 = 0x1d;

    PORTE.sim_enable();
    I2C0.sim_enable();

    let i2c = I2C0;
    let _i2c_scl = PTE24.mode_i2c_scl(&i2c).set_pull_none().set_ode(true);
    let _i2c_sda = PTE25.mode_i2c_sda(&i2c).set_pull_none().set_ode(true);
    
    let mult = 0;
    let icr = 0x1c;
    let addr = I2C_ADDR;

    i2c.init(mult, icr);    
    println!("Configuration Complete");

    // Check WHO_AM_I @ 0x0d = 0xC7
    assert_eq!(i2c.reg_read(addr, 0x0d), 0xc7);

    // Standby Mode
    i2c.reg_write(addr, 0x2A, 0x00);

    println!("CTRL_REG1:      0x{:02x} = 0x00", i2c.reg_read(addr, 0x2a));

    // write 0001 1111 = 0x1F to magnetometer control register 1
    i2c.reg_write(addr, 0x5B, 0x1f);
    println!("MAG_CTRL_REG1:  0x{:02x} = 0x1f", i2c.reg_read(addr, 0x5b));

    // write 0010 0000 = 0x20 to magnetometer control register 2
    i2c.reg_write(addr, 0x5C, 0x20);
    println!("MAG_CTRL_REG2:  0x{:02x} = 0x20", i2c.reg_read(addr, 0x5c));

    // write 0000 0001 = 0x01 to XYZ_DATA_CFG register
    i2c.reg_write(addr, 0x0E, 0x01);
    println!("XYZ_DATA_CFG:   0x{:02x} = 0x01", i2c.reg_read(addr, 0x0e));

    // write 0000 1101 = 0x0D to accelerometer control register 1
    i2c.reg_write(addr, 0x2A, 0x0D);
    println!("CTRL_REG1:      0x{:02x} = 0x0d", i2c.reg_read(addr, 0x2a));

    let mut cfg = [0u8; 5];
    i2c.transfer(addr, &[0x2a], &mut cfg);
    for b in cfg.iter() {
        print!("{:02x}", b);
    }
    println!("");

    println!("I2C Configured");    
    let mut buf = [0u8; 17];
    loop {
        i2c.transfer(addr, &[0x00], &mut buf);
        for b in buf.iter() {
            print!("{:02x} ", b);
        }
        let data = MagAccelData::from(buf);
        println!("| {:?}", data);

        board::delay(500);
    }
}

#[derive(Debug)]
pub struct MagAccelData {
    ax: i16,
    ay: i16,
    az: i16,
    mx: i16,
    my: i16,
    mz: i16,
}

impl From<[u8; 17]> for MagAccelData {
    fn from(other: [u8; 17]) -> Self {
        MagAccelData {
            ax: (((other[1] as u16) << 8 | (other[2] as u16)) as i16) >> 2,
            ay: (((other[3] as u16) << 8 | (other[4] as u16)) as i16) >> 2,
            az: (((other[5] as u16) << 8 | (other[6] as u16)) as i16) >> 2,
            mx: ((other[7] as u16) << 8 | (other[8] as u16)) as i16,
            my: ((other[9] as u16) << 8 | (other[10] as u16)) as i16,
            mz: ((other[11] as u16) << 8 | (other[12] as u16)) as i16,
        }
    }
}