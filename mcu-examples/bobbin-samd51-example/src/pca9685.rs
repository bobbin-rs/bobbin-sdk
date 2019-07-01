#![no_std]
#![no_main]

extern crate panic_abort;
use cortex_m_rt::entry;
use cortex_m::asm;

use samd51::bobbin_mcu::prelude::*;
use samd51::bobbin_hal::prelude::*;
use samd51::bobbin_sys::prelude::*;
use samd51::bobbin_sys::{print, println};
use samd51::ext::clock;

use samd51::sercom::*;
use samd51::ext::sercom::i2c::SercomError;
use samd51::port::{PORTA, PORTB};
use samd51::sercom::{SERCOM2, SERCOM5};
use samd51::pin::PA23;
use samd51::gclk::{self, GCLK};

// PCA9685

// SCL = A12 = SERCOM2/PAD[0] @ C / SERCOM4/PAD[1] @ D
// SDA = A13 = SERCOM2/PAD[1] @ C / SERCOM4/PAD[0] @ D

#[entry]
fn main() -> ! {
    clock::run_120mhz();

    // Enable LED Output
    PA23.set_output(true);

    // Set PinMux to 0x2 (C)
    PORTB.with_pmux(8, |r| r.set_pmuxe(0x2).set_pmuxo(0x2));
    // Enable PinMux
    PORTB.with_pincfg(16, |r| r.set_pmuxen(1));
    PORTB.with_pincfg(17, |r| r.set_pmuxen(1));

    // Use clockgen 0 for SERCOM5

    GCLK.write_pchctrl(35, gclk::Pchctrl(0)
        .set_gen(0x0)
        .set_chen(1)
    );

    SERCOM5.gate_enable();

    let cfg = samd51::ext::sercom::Config::default()
        .set_mode_usart_int()
        .set_baud_clock(115_200, 120_000_000)
        .set_rxpo(0x1)
        .set_txpo(0x0);
    SERCOM5.configure(cfg);
    SERCOM5.set_enabled(true);
    Console::set(Console::new(SERCOM5.as_periph(), ConsoleMode::Cooked));

    // Use SERCOM2 for I2C

    // Set PinMux to 0x2 (C)
    PORTA.with_pmux(6, |r| r.set_pmuxe(0x2).set_pmuxo(0x2));
    // Enable PinMux
    PORTA.with_pincfg(12, |r| r.set_pmuxen(1));
    PORTA.with_pincfg(13, |r| r.set_pmuxen(1));

    SERCOM2.gate_enable();

    // set baud rate

    let baud_divider = 100;
    let addr = 0x60;
    SERCOM2.init_i2c(baud_divider);
    SERCOM2.enable_i2c();
    let pwm = Pca9685::new(SERCOM2, addr);
    println!("reset: {:?}", pwm.reset());
    println!("init: {:?}", pwm.init());

    let mut i = 0u32;
    loop {
        for _ in 0..10_000_000 {
            asm::nop();
        }
        PA23.toggle_output();
        println!("0x00: {:08x}", pwm.read_reg(0x00).unwrap());
        println!("0x01: {:08x}", pwm.read_reg(0x01).unwrap());
        i = i.wrapping_add(1);
    }
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
        self.write_reg(LED0_ON_L + offset, on as u8)?;
        self.write_reg(LED0_ON_H + offset, (on >> 8) as u8)?;
        self.write_reg(LED0_OFF_L + offset, off as u8)?;
        self.write_reg(LED0_OFF_H + offset, (off >> 8) as u8)?;
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
pub const ALLCALL            :u8 = 0x01;
pub const INVRT              :u8 = 0x10;
pub const OUTDRV             :u8 = 0x04;