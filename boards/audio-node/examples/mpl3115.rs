#![no_std]
#![no_main]

#[macro_use]
extern crate audio_node as board;

use board::hal::i2c::*;
use board::hal::gpio::*;
use board::common::bits::*;


// A5 = PB6 = I2C1_SCL
// A4 = PB7 = I2C1_SDA

// Address = 0x35

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Running I2C");

    let addr: U7 = U7::from(0x60);

    let i2c = I2C1;
    let i2c_port = GPIOB;
    let i2c_scl = PB6; // D4
    let i2c_sda = PB7; // D5

    i2c.rcc_enable();
    i2c_port.rcc_enable();

    // Attached to MPL3115A2 
    // NOTE: SCL and SCA must have pull-up resistors.

    i2c_scl.mode_i2c_scl(&i2c).open_drain();
    i2c_sda.mode_i2c_sda(&i2c).open_drain();

    // println!("# Configuring I2C");

    // i2c.set_config(|c| c.set_timing(0x8.into(), 0x3.into(), 0x0.into(), 0xd.into(), 0x12.into()));
    i2c.set_enabled(false);
    // i2c.set_timingr(|_| Timingr(0x00300619));
    i2c.set_timingr(|r| r
        .set_presc(0x0)
        .set_scldel(0x3)
        .set_sdadel(0x0)
        .set_sclh(0xF)
        .set_scll(0x12)
    );

    let p: I2cPeriph = i2c.into();

    let mpl = Mpl3115::new(p);
    assert!(mpl.probe());

    // assert_eq!(i2c.read_reg(addr, 0x0c), 0xc4);
    
    // println!("Mode:  0x{:08x}", i2c.read_reg(addr, 0x26));
    

    // i2c.write_reg(addr, 0x26, 0xb8); // OSR = 128
    i2c.write_reg(addr, 0x13, 0x06); // Enable Data Flags
    mpl.set_mode(Mode::Barometer);
    mpl.set_active(true);
    // i2c.write_reg(addr, 0x26, 0x09); // Set Active
    // println!("Mode:  0x{:08x}", i2c.read_reg(addr, 0x26));


    // loop {
    //     while i2c.read_reg(addr, 0x00) != 0x04 {}    
    //     println!("reading..");
    //     // if mpl.pressure_ready() {
    //         let p: f32  = mpl.pressure().into();
    //         println!("p: {:2.2}", p);
    //         board::delay(100);
    //     // }
    //     // board::delay(100);
    // }



    loop {
        while i2c.read_reg(addr, 0x00) != 0x04 {}
        // if mpl.pressure_ready() {
            // let a: f32 = mpl.altitude().into();
            // println!("A: {:2.2}", a);
        // }

        let t = mpl.temperature();
        let t_c = t.as_f32();
        let t_f = t.as_f32() * 9.0 / 5.0 + 32.0; 
        
        print!("{:2.2} C / {:2.2} F | ", t_c, t_f);

        match mpl.mode() {
            Mode::Barometer => {
                let p = mpl.pressure();
                let hg_mb = p.as_f32() * 0.01;
                let hg_in = p.as_f32() * 0.0002953;
                println!("{:4.2} millibar / {:4.4} inches hg", hg_mb,  hg_in);
            },
            Mode::Altimeter => {
                let a = mpl.altitude();
                let a_m = a.as_f32();
                let a_f = a_m * 3.28084;
                println!("{:2.2} Meters / {:2.2} Feet", a_m, a_f);
            },
            Mode::Raw => {
                println!("RAW");
            }
        }


        // let mut buf = [0u8; 5];
        // i2c.transfer(addr, &[0x01], &mut buf);
        // println!("# {:?}", buf);
        board::delay(500);
    }
}

use board::common::i2c::I2cTransfer;

pub struct DrStatus(u8);

impl DrStatus {
    pub fn ptow(&self) -> bool {
        (self.0 & 1 << 7) != 0
    }

    pub fn pow(&self) -> bool {
        (self.0 & 1 << 6) != 0
    }

    pub fn tow(&self) -> bool {
        (self.0 & 1 << 5) != 0
    }

    pub fn ptdr(&self) -> bool {
        (self.0 & 1 << 3) != 0
    }

    pub fn pdr(&self) -> bool {
        (self.0 & 1 << 2) != 0
    }

    pub fn tdr(&self) -> bool {
        (self.0 & 1 << 1) != 0
    }
}

#[allow(non_camel_case_types)]
pub struct Q18_2(u8, u8, u8);

impl Q18_2 {
    fn int(&self) -> i32 {
        ((self.0 as u32) << 10 | (self.1 as u32) << 2 | (self.0 as u32) >> 6) as i32
    }

    fn frac_num(&self) -> u8 {
        (self.2 >> 4) & 0x02
    }    

    fn frac_den(&self) -> u8 {
        0x2
    }

    fn as_f32(&self) -> f32 {
        self.int() as f32 + (self.frac_num() as f32) / (self.frac_den() as f32)
    }    
}

impl From<Q18_2> for f32 {
    fn from(other: Q18_2) -> f32 {
        other.as_f32()
    }
}

#[allow(non_camel_case_types)]
pub struct Q16_4(u8, u8, u8);

impl Q16_4 {
    fn int(&self) -> i16 {
        ((self.0 as u16) << 8 | (self.1 as u16)) as i16
    }

    fn frac_num(&self) -> u8 {
        self.2 >> 4
    }    

    fn frac_den(&self) -> u8 {
        0x4
    }

    fn as_f32(&self) -> f32 {
        self.int() as f32 + (self.frac_num() as f32) / (self.frac_den() as f32)
    }
}

impl From<Q16_4> for f32 {
    fn from(other: Q16_4) -> f32 {
        other.as_f32()
    }
}

#[allow(non_camel_case_types)]
pub struct Q8_4(u8, u8);

impl Q8_4 {
    fn int(&self) -> i8 {
        self.0 as i8
    }

    fn frac_num(&self) -> u8 {
       self.0 >> 4
    }    

    fn frac_den(&self) -> u8 {
        0x4
    }

    fn as_f32(&self) -> f32 {
        self.int() as f32 + (self.frac_num() as f32) / (self.frac_den() as f32)
    }        
}

impl From<Q8_4> for f32 {
    fn from(other: Q8_4) -> f32 {
        other.as_f32()
    }
}

pub trait Pressure<T> {
    fn pressure(&self) -> T;
    fn pressure_ready(&self) -> bool;
}

pub trait Altitude<T> {
    fn altitude(&self) -> T;
    fn altitude_ready(&self) -> bool;
}

pub trait Temperature<T> {
    fn temperature(&self) -> T;
    fn temperature_ready(&self) -> bool;
}

pub enum Mode {
    Barometer = 0b00,
    Raw = 0b01,
    Altimeter = 0b10,
}

pub struct Mpl3115<I: I2cTransfer<u8>> {
    i2c: I,
}

impl<I: I2cTransfer<u8>> Mpl3115<I> {
    const I2C_ADDR: u8 = 0x60;

    const REG_WHO_AM_I: u8 = 0x0c;
    const VAL_WHO_AM_I: u8 = 0xc4;

    const REG_OUT_P_MSB: u8 = 0x01;
    const REG_OUT_P_CSB: u8 = 0x02;
    const REG_OUT_P_LSB: u8 = 0x03;

    const REG_OUT_T_MSB: u8 = 0x04;
    const REG_OUT_T_LSB: u8 = 0x05;

    const REG_DR_STATUS: u8 = 0x06;

    const REG_CTRL_REG1: u8 = 0x26;

    pub fn new(i2c: I) -> Self {
        Mpl3115 { i2c: i2c }
    }

    pub fn read(&self, reg: u8) -> u8 {
        self.i2c.read_reg(Self::I2C_ADDR, reg)
    }

    pub fn write(&self, reg: u8, val: u8) -> &Self {
        self.i2c.write_reg(Self::I2C_ADDR, reg, val);
        self
    }

    pub fn probe(&self) -> bool {
        self.who_am_i() == Self::VAL_WHO_AM_I
    }

    pub fn who_am_i(&self) -> u8 {
        self.read(Self::REG_WHO_AM_I)
    }

    pub fn dr_status(&self) -> DrStatus {
        DrStatus(self.read(Self::REG_DR_STATUS))
    }

    pub fn mode(&self) -> Mode {
        match self.read(Self::REG_CTRL_REG1) >> 6 {
            0b00 => Mode::Barometer,
            0b10 => Mode::Altimeter,
            0b01 => Mode::Raw,
            0b11 => Mode::Raw,
            _ => unimplemented!()
        }
    }

    pub fn set_mode(&self, mode: Mode) {
        let mut v = self.read(Self::REG_CTRL_REG1);
        v & 0b00111111;
        v |= (mode as u8) << 6;
        self.write(Self::REG_CTRL_REG1, v);
    }

    pub fn active(&self) -> bool {
        self.read(Self::REG_CTRL_REG1) & 0x01 != 0
    }

    pub fn set_active(&self, value: bool) {
        let value = if value { 1 } else {0 };
        let mut v = self.read(Self::REG_CTRL_REG1);
        v & !1;
        v |= value;
        self.write(Self::REG_CTRL_REG1, v);
        
    }
}

impl<I: I2cTransfer<u8>> Altitude<Q16_4> for Mpl3115<I> {
    fn altitude(&self) -> Q16_4 {
        let p_msb = self.read(Self::REG_OUT_P_MSB);
        let p_csb = self.read(Self::REG_OUT_P_CSB);
        let p_lsb = self.read(Self::REG_OUT_P_LSB);
        Q16_4(p_msb, p_csb, p_lsb)
    }

    fn altitude_ready(&self) -> bool {
        self.dr_status().pdr()
    }
}


impl<I: I2cTransfer<u8>> Pressure<Q18_2> for Mpl3115<I> {
    fn pressure(&self) -> Q18_2 {
        let p_msb = self.read(Self::REG_OUT_P_MSB);
        let p_csb = self.read(Self::REG_OUT_P_CSB);
        let p_lsb = self.read(Self::REG_OUT_P_LSB);
        Q18_2(p_msb, p_csb, p_lsb)
    }

    fn pressure_ready(&self) -> bool {
        self.dr_status().pdr()
    }
}


impl<I: I2cTransfer<u8>> Temperature<Q8_4> for Mpl3115<I> {
    fn temperature(&self) -> Q8_4 {
        let t_msb = self.read(Self::REG_OUT_T_MSB);
        let t_lsb = self.read(Self::REG_OUT_T_LSB);
        Q8_4(t_msb, t_lsb)
    }

    fn temperature_ready(&self) -> bool {
        self.dr_status().tdr()
    }
}
