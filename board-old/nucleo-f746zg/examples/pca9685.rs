#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f746zg as board;

use board::hal::i2c::*;
use board::hal::gpio::*;
use board::common::bits::*;


// A5 = PB6 = I2C1_SCL
// A4 = PB7 = I2C1_SDA

// Address = 0x35

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("PCA9685 Test");
    
    let addr: U7 = U7::from(0x40);

    let i2c = I2C1;
    let i2c_port = GPIOB;
    let i2c_scl = PB8; // D15
    let i2c_sda = PB9; // D14

    GPIOA.rcc_enable();
    PA6.mode_input().open_drain();
    PA5.mode_input().open_drain();

    i2c.rcc_enable();
    i2c_port.rcc_enable();

    i2c_scl.mode_i2c_scl(&i2c).open_drain();
    i2c_sda.mode_i2c_sda(&i2c).open_drain();

    println!("# Configuring I2C");

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

    let pwm = Pca9685::new(i2c, addr);
    pwm.reset();
    pwm.init();    
    println!("PWM Initialized");
    pwm.set_pwm(0, 0, 4096);
    pwm.set_pwm(1, 0, 0);
    loop {}

}

pub struct Pca9685 {
    i2c: I2cPeriph,
    addr: U7,
}

impl Pca9685 {
    pub fn new<I: Into<I2cPeriph>>(i: I, addr: U7) -> Self {
        Pca9685 {
            i2c: i.into(),
            addr: addr,
        }
    }

    pub fn write_reg(&self, reg: u8, val: u8) {
        self.i2c.write_reg(self.addr, reg, val);
    }

    pub fn read_reg(&self, reg: u8) -> u8 {
        self.i2c.read_reg(self.addr, reg)
    }

    pub fn reset(&self) {
        self.i2c.write_reg(self.addr, 0x00, 0x06);
    }

    pub fn init(&self) {
        self.set_all_pwm(0, 0);
        self.write_reg(MODE2, OUTDRV);
        self.write_reg(MODE1, ALLCALL);
        let mode1 = self.read_reg(MODE1) & !SLEEP;
        self.write_reg(MODE1, mode1);
    }

    pub fn prescale(&self) -> u8 {
        self.read_reg(PRESCALE)
    }

    pub fn set_prescale(&self, value: u8) {
        self.write_reg(PRESCALE, value);
    }

    pub fn pwm_freq(&self) -> u32 {
        let prescale = self.read_reg(PRESCALE);
        25_000_000 / ((prescale as u32 + 1) * 4096)
    }

    pub fn set_pwm_freq(&self, freq_hz: u32) {
        let mut prescaleval: f32 = 25000000.0;    // 25MHz
        prescaleval /= 4096.0;       // 12-bit
        prescaleval /= freq_hz as f32;
        prescaleval -= 1.0;
        let prescale = (prescaleval + 0.5) as u8;
        // prescale = int(math.floor(prescaleval + 0.5))
        let oldmode = self.read_reg(MODE1);
        let newmode = (oldmode & 0x7F) | 0x10;    // sleep
        self.write_reg(MODE1, newmode);  // go to sleep
        self.write_reg(PRESCALE, prescale);
        self.write_reg(MODE1, oldmode);        
        self.write_reg(MODE1, oldmode | 0x80);
    }

    pub fn set_pwm(&self, channel: u8, on: u16, off: u16) {
        let offset = 4 * channel;
        self.write_reg(LED0_ON_L + offset, on as u8);
        self.write_reg(LED0_ON_H + offset, (on >> 8) as u8);
        self.write_reg(LED0_OFF_L + offset, off as u8);
        self.write_reg(LED0_OFF_H + offset, (off >> 8) as u8);        
    }

    pub fn pwm(&self, channel: u8) -> (u16, u16) {
        let offset = 4 * channel;
        let on_l = self.read_reg(LED0_ON_L + offset);
        let on_h = self.read_reg(LED0_ON_H + offset);
        let off_l = self.read_reg(LED0_OFF_L + offset);
        let off_h = self.read_reg(LED0_OFF_H + offset);
        (
            (on_h as u16) << 8 | (on_l as u16),
            (off_h as u16) << 8 | (off_l as u16),
        )
    }

    pub fn set_all_pwm(&self, on: u16, off: u16) {
        self.write_reg(ALL_LED_ON_L, on as u8);
        self.write_reg(ALL_LED_ON_H, (on >> 8) as u8);
        self.write_reg(ALL_LED_OFF_L, off as u8);
        self.write_reg(ALL_LED_OFF_H, (off >> 8) as u8);
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