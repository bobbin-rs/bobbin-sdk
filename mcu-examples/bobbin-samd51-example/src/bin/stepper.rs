#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m::asm;

use samd51::bobbin_mcu::prelude::*;
use samd51::bobbin_hal::prelude::*;
use bobbin_samd51_example::*;

use samd51::sercom::*;
use samd51::ext::sercom::i2c::SercomError;
use samd51::port::PORTA;
use samd51::sercom::SERCOM2;

// PCA9685

// SCL = A12 = SERCOM2/PAD[0] @ C / SERCOM4/PAD[1] @ D
// SDA = A13 = SERCOM2/PAD[1] @ C / SERCOM4/PAD[0] @ D

#[entry]
fn main() -> ! {
    init();

    // Use SERCOM2 for I2C
    // Set PinMux to 0x2 (C)
    PORTA.with_pmux(6, |r| r.set_pmuxe(0x2).set_pmuxo(0x2));
    // Enable PinMux
    PORTA.with_pincfg(12, |r| r.set_pmuxen(1));
    PORTA.with_pincfg(13, |r| r.set_pmuxen(1));

    SERCOM2.gate_enable();

    // set baud rate

    let baud_divider = 20;
    let addr = 0x60;
    SERCOM2.init_i2c(baud_divider);
    SERCOM2.enable_i2c();
    let pca = Pca9685::new(SERCOM2, addr);
    println!("Initializing PCA9685");
    pca.reset().unwrap();
    pca.init().unwrap();

    pca.set_pwm_freq(1600).unwrap();
    pca.set(2, true).unwrap();
    pca.set(7, true).unwrap();

    let mut i = 0i32;
    let delay = 16000;    
    loop {
        LED0.toggle_output();
        if false {
            microstep(&pca, i);
        } else {
            full_step(&pca, i);
        }
        for _ in 0..delay {
            asm::nop();
        }
        i = i.wrapping_add(1);
    }        
}

fn microstep(pca: &Pca9685, i: i32) {
    // i from 0.64

    let (a2, b1, a1, b2) = match (i / 8) % 4 {
        0 => (true, true, false, false),
        1 => (false, true, true, false),
        2 => (false, false, true, true),
        3 => (true, false, false, true),
        _ => unimplemented!()
    };

    let ocrb = mag(i + 8);
    let ocra = mag(i);

    // println!("i: {:>4} orca: {:>4} orcb: {:>4} a2: {:8} b1: {:8} a1: {:8} b2: {:8}", i, ocra, ocrb, a2, b1, a1, b2);

    pca.set_pwm(2, ocra * 16, 4096 - ocra * 16).unwrap();
    pca.set_pwm(7, ocrb * 16, 4096 - ocrb * 16).unwrap();

    pca.set4(3, a2, a1, b1, b2).unwrap();
    // if i % 4 == 0 {
        // pca.set(3, a2).unwrap();
        // pca.set(4, a1).unwrap();
        // pca.set(5, b1).unwrap();
        // pca.set(6, b2).unwrap();
    // }
}

fn mag(i: i32) -> u16 {
    const STEPS: [u16; 9] = [0, 50, 98, 142, 180, 212, 236, 250, 255];

    let phase = (i % 32) as usize;
    match phase {
        0..=7 => STEPS[phase],
        8..=15 => STEPS[16 - phase],
        16..=23 => STEPS[phase - 16],
        24..=31 => STEPS[32 - phase],
        _ => panic!("Phase {} out of bounds", phase)
    }
}
fn full_step(pca: &Pca9685, i: i32) {
    // let i = i / 8;
    let (a2, b1, a1, b2) = match i % 4 {
        0 => (true, false, false, false),
        1 => (false, true, false, false),
        2 => (false, false, true, false),
        3 => (false, false, false, true),
        _ => unimplemented!()
    };
    // println!("a2: {} b1: {} a1: {} b2: {}", a2, b1, a1, b2);
    pca.set4(3, a2, a1, b1, b2).unwrap();
    // pca.set(3, a2).unwrap();
    // pca.set(5, b1).unwrap();
    // pca.set(4, a1).unwrap();
    // pca.set(6, b2).unwrap();
}

pub struct Pca9685 {
    i2c: SercomPeriph,
    addr: u8,
    timeout: u32,
}

impl Pca9685 {
    pub fn new<I: Into<SercomPeriph>>(i: I, addr: u8) -> Self {
        Self::new_with_timeout(i, addr, 8192)
    }

    pub fn new_with_timeout<I: Into<SercomPeriph>>(i: I, addr: u8, timeout: u32) -> Self {
        Pca9685 {
            i2c: i.into(),
            addr: addr,
            timeout,
        }
    }

    pub fn write_reg(&self, reg: u8, val: u8) -> Result<(), SercomError> {
        self.i2c.try_transfer(self.addr.into(), &[reg, val], &mut [], self.timeout)?;
        Ok(())
    }

    pub fn read_reg(&self, reg: u8) -> Result<u8, SercomError> {
        let mut buf = [0u8];
        self.i2c.try_transfer(self.addr.into(), &[reg], &mut buf, self.timeout)?;
        Ok(buf[0])
    }


    // pub fn write_reg(&self, reg: u8, val: u8) {
    //     match self.i2c.try_transfer(self.addr.into(), &[reg, val], &mut [], self.timeout) {
    //         Ok(_) => {},
    //         Err(e) => {
    //             println!("write_reg error: {:?}", e);
    //             loop {}
    //         }
    //     }
    // }

    // pub fn read_reg(&self, reg: u8) -> u8 {
    //     let mut buf = [0u8];
    //     match self.i2c.try_transfer(self.addr.into(), &[reg], &mut buf, self.timeout) {
    //         Ok(_) => buf[0],
    //         Err(e) => {
    //             println!("read_reg error: {:?}", e);
    //             loop {}
    //         }
    //     }        
    // }

    pub fn reset(&self) -> Result<(), SercomError> {
        self.write_reg(0x00, 0x06)
    }

    pub fn init(&self) -> Result<(), SercomError> {
        self.set_all_pwm(0, 0)?;
        self.write_reg(MODE2, OUTDRV)?;
        self.write_reg(MODE1, ALLCALL)?;
        let mode1 = self.read_reg(MODE1)? & !SLEEP;
        let mode1 = mode1 | AI;
        self.write_reg(MODE1, mode1)?;
        Ok(())
    }

    pub fn prescale(&self) -> Result<u8, SercomError> {
        self.read_reg(PRESCALE)
    }

    pub fn set_prescale(&self, value: u8) -> Result<(), SercomError>{
        self.write_reg(PRESCALE, value)
    }

    pub fn pwm_freq(&self) -> Result<u32, SercomError> {
        let prescale = self.read_reg(PRESCALE)?;
        Ok(25_000_000 / ((prescale as u32 + 1) * 4096))
    }

    pub fn set(&self, channel: u8, on: bool) -> Result<(), SercomError>  {
        let (on, off) = if on { (4096, 0) } else { (0, 4096) };
        self.set_pwm(channel, on, off)
        // let offset = 4 * channel;

        // let data = [LED0_ON_L + offset, on as u8, (on >> 8) as u8, off as u8, (off >> 8) as u8];
        // self.i2c.try_transfer(self.addr.into(), &data, &mut [], self.timeout)?;

        // let data = [LED0_ON_L + offset, on as u8, (on >> 8) as u8];
        // self.i2c.try_transfer(self.addr.into(), &data, &mut [], self.timeout)?;
        // let data = [LED0_OFF_L + offset, off as u8, (off >> 8) as u8];
        // self.i2c.try_transfer(self.addr.into(), &data, &mut [], self.timeout)?;

        // self.write_reg(LED0_ON_L + offset, on as u8)?;
        // self.write_reg(LED0_ON_H + offset, (on >> 8) as u8)?;
        // self.write_reg(LED0_OFF_L + offset, off as u8)?;
        // self.write_reg(LED0_OFF_H + offset, (off >> 8) as u8)?;
        // Ok(())
    }

    pub fn set4(&self, channel: u8, on0: bool, on1: bool, on2: bool, on3: bool) -> Result<(), SercomError>  {
        let (on0, off0) = if on0 { (4096, 0) } else { (0, 4096) };
        let (on1, off1) = if on1 { (4096, 0) } else { (0, 4096) };
        let (on2, off2) = if on2 { (4096, 0) } else { (0, 4096) };
        let (on3, off3) = if on3 { (4096, 0) } else { (0, 4096) };
        let offset = 4 * channel;
        let data = [LED0_ON_L + offset, 
            on0 as u8, (on0 >> 8) as u8, off0 as u8, (off0 >> 8) as u8,
            on1 as u8, (on1 >> 8) as u8, off1 as u8, (off1 >> 8) as u8,
            on2 as u8, (on2 >> 8) as u8, off2 as u8, (off2 >> 8) as u8,
            on3 as u8, (on3 >> 8) as u8, off3 as u8, (off3 >> 8) as u8,
        ];
        self.i2c.try_transfer(self.addr.into(), &data, &mut [], self.timeout)?;
        Ok(())
    }

    pub fn set_pwm_freq(&self, freq_hz: u32) -> Result<(), SercomError> {
        let mut prescaleval: f32 = 25000000.0;    // 25MHz
        prescaleval /= 4096.0;       // 12-bit
        prescaleval /= freq_hz as f32;
        prescaleval -= 1.0;
        let prescale = (prescaleval + 0.5) as u8;
        // prescale = int(math.floor(prescaleval + 0.5))
        let oldmode = self.read_reg(MODE1)?;
        let newmode = (oldmode & 0x7F) | 0x10;    // sleep
        self.write_reg(MODE1, newmode)?;  // go to sleep
        self.write_reg(PRESCALE, prescale)?;
        self.write_reg(MODE1, oldmode)?;        
        self.write_reg(MODE1, oldmode | 0x80)?;
        Ok(())
    }

    pub fn set_pwm(&self, channel: u8, on: u16, off: u16) -> Result<(), SercomError> {
        // println!("{}/{}: {} {}", self.addr, channel, on, off);
        let offset = 4 * channel;
        let data = [LED0_ON_L + offset, on as u8, (on >> 8) as u8, off as u8, (off >> 8) as u8];
        self.i2c.try_transfer(self.addr.into(), &data, &mut [], self.timeout)?;

        // self.write_reg(LED0_ON_L + offset, on as u8)?;
        // self.write_reg(LED0_ON_H + offset, (on >> 8) as u8)?;
        // self.write_reg(LED0_OFF_L + offset, off as u8)?;
        // self.write_reg(LED0_OFF_H + offset, (off >> 8) as u8)?;
        Ok(())
    }


    // assumes frequency setting of 62.5, resulting in 1ms @ 256 and 2ms @ 512
    pub fn set_servo(&self, channel: u8, val: u8) -> Result<(), SercomError> {
        self.set_pwm(channel, 0, 256 + (val as u16))
    }

    pub fn pwm(&self, channel: u8) -> Result<(u16, u16), SercomError> {
        let offset = 4 * channel;
        let on_l = self.read_reg(LED0_ON_L + offset)?;
        let on_h = self.read_reg(LED0_ON_H + offset)?;
        let off_l = self.read_reg(LED0_OFF_L + offset)?;
        let off_h = self.read_reg(LED0_OFF_H + offset)?;
        Ok((
            (on_h as u16) << 8 | (on_l as u16),
            (off_h as u16) << 8 | (off_l as u16),
        ))
    }

    pub fn set_all_pwm(&self, on: u16, off: u16) -> Result<(), SercomError> {
        self.write_reg(ALL_LED_ON_L, on as u8)?;
        self.write_reg(ALL_LED_ON_H, (on >> 8) as u8)?;
        self.write_reg(ALL_LED_OFF_L, off as u8)?;
        self.write_reg(ALL_LED_OFF_H, (off >> 8) as u8)?;
        Ok(())
    }
}

pub const MODE1              :u8 =  0x00;
pub const MODE2              :u8 =  0x01;
pub const SUBADR1            :u8 =  0x02;
pub const SUBADR2            :u8 =  0x03;
pub const SUBADR3            :u8 =  0x04;
pub const PRESCALE           :u8 =  0xFE;
pub const LED0_ON_L          :u8 =  0x06;
pub const LED0_ON_H          :u8 =  0x07;
pub const LED0_OFF_L         :u8 =  0x08;
pub const LED0_OFF_H         :u8 =  0x09;
pub const ALL_LED_ON_L       :u8 =  0xFA;
pub const ALL_LED_ON_H       :u8 =  0xFB;
pub const ALL_LED_OFF_L      :u8 =  0xFC;
pub const ALL_LED_OFF_H      :u8 =  0xFD;

pub const RESTART            :u8 = 0x80;
pub const SLEEP              :u8 = 0x10;
pub const AI                 :u8 = 0x20;
pub const ALLCALL            :u8 = 0x01;
pub const INVRT              :u8 = 0x10;
pub const OUTDRV             :u8 = 0x04;