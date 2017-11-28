#![no_std]
#![no_main]
#![allow(dead_code)]

#[macro_use]
extern crate nero_f7 as board;
extern crate icm20602;

use board::hal::gpio::{DigitalOutput};

use board::hal::gpio::*;
use board::hal::spi::*;

// ICM-20602 on SPI1
// SCK = PA5
// MISO = PA6
// MOSI = PA7
// CS = PC4
// IRQ = PB2

use board::hal::clock::*;
use board::clock::CLK;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Testing IMU");
    board::delay(100);

    println!("CLK: {:?}", CLK);

    let spi = SPI1;
    let spi_sck = PA5;
    let spi_miso = PA6;
    let spi_mosi = PA7;
    let spi_cs = PC4;
    let spi_irq = PB2;

    {
        println!("Setting up Clocks and Pins");

        // Enable Clocks

        spi.rcc_enable();
        spi_sck.port().rcc_enable();
        spi_miso.port().rcc_enable();
        spi_mosi.port().rcc_enable();
        spi_cs.port().rcc_enable();
        spi_irq.port().rcc_enable();

        // Setup Pins
        spi_cs.mode_output();
        spi_cs.set_output(true);

        spi_miso.mode_spi_miso(&spi).speed_high().pull_up();
        spi_mosi.mode_spi_mosi(&spi).speed_high().push_pull();
        spi_sck.mode_spi_sck(&spi).speed_high();
        spi_irq.mode_input();

    }

    println!("SPI: {:?}", spi.clock(&CLK));

    {
        // Setup SPI

        spi.set_config(|cfg| cfg
            .set_data_size(DataSize::Bits8)
            .set_master(true)
            .set_baud_divider(0b100.into())
        );
        spi.with_cr2(|r| r.set_frxth(1));
        spi.set_output_enabled(true).set_enabled(true);      


        // spi.with_cr1(|r| r.set_ssm(1).set_cpol(0).set_cpha(0)); 
        // spi.with_cr1(|r| r.set_ssm(1)); 
        // spi.with_cr2(|r| r.set_frxth(1));

        // spi.set_cr1(|r| r
        //     .set_br(0b111)
        //     .set_mstr(1)
        //     .set_ssm(1)
        //     .set_ssi(1)
        // );

        // spi.set_cr2(|r| r
        //     .set_frxth(1)
        //     .set_ds(0b0111)
        // );

        // spi.with_cr1(|r| r.set_spe(1));

        // spi.set_enabled(true);        

    }
    println!("CR1: {:?}", spi.cr1());
    println!("CR2: {:?}", spi.cr2());
    // board::delay(100);
    {
        // Disable I2C

        // println!("Disable I2C Mode");
        // reg_write(&spi, &spi_cs, icm20602::REG_PWR_MGMT_1, 1 << 7);
        // board::delay(100);
        // reg_write(&spi, &spi_cs, icm20602::REG_I2C_IF, 1 << 6);
        // println!("0x70: {:02x} should be 40", reg_read(&spi, &spi_cs, 0x80 | 0x70));

        // Check for ICM-20602
        // println!("Resetting");
        // reg_write(&spi, &spi_cs, 0x6b, 0x80);
        // println!("Waiting for reset to clear");
        // while reg_read(&spi, &spi_cs, 0x80 | 0x6b) & 0x80 != 0 {}
        // board::delay(100);
        // reg_write(&spi, &spi_cs, 0x70, 0x40);
        // println!("I2C_IF: {:02x}", reg_read(&spi, &spi_cs, 0x80 |0x70));

        println!("Probing for ICM20602");
        assert_eq!(reg_read(&spi, &spi_cs, 0x75), 0x12);


        println!("0x75: {:02x}", reg_read(&spi, &spi_cs, 0x75));

        println!("0x6a: {:02x}", reg_read(&spi, &spi_cs, 0x6a));
        println!("0x6b: {:02x}", reg_read(&spi, &spi_cs, 0x6b));
        println!("0x6c: {:02x}", reg_read(&spi, &spi_cs, 0x6c));

        reg_write(&spi, &spi_cs, 0x6b, 0x01);
        println!("0x6b: {:02x}", reg_read(&spi, &spi_cs, 0x6b));
        // println!("0x1C: {:02x} should be 00", reg_read(&spi, &spi_cs, 0x80 | 0x1c));
        // println!("0x1D: {:02x} should be 00", reg_read(&spi, &spi_cs, 0x80 | 0x1d));
        // println!("0x1E: {:02x} should be 00", reg_read(&spi, &spi_cs, 0x80 | 0x1e));
    }
    println!("Initialization Complete");

    loop {
        // println!("Reading Temperature");
        let th = reg_read(&spi, &spi_cs, icm20602::REG_TEMP_OUT_H);
        let tl = reg_read(&spi, &spi_cs, icm20602::REG_TEMP_OUT_L);

        let axh = reg_read(&spi, &spi_cs, icm20602::REG_ACCEL_XOUT_H);
        let axl = reg_read(&spi, &spi_cs, icm20602::REG_ACCEL_XOUT_L);
        let ax = ((axh as u16) << 8 | (axl as u16)) as i16;

        let ayh = reg_read(&spi, &spi_cs, icm20602::REG_ACCEL_YOUT_H);
        let ayl = reg_read(&spi, &spi_cs, icm20602::REG_ACCEL_YOUT_L);
        let ay = ((ayh as u16) << 8 | (ayl as u16)) as i16;

        let azh = reg_read(&spi, &spi_cs, icm20602::REG_ACCEL_ZOUT_H);
        let azl = reg_read(&spi, &spi_cs, icm20602::REG_ACCEL_ZOUT_L);
        let az = ((azh as u16) << 8 | (azl as u16)) as i16;

        let gxh = reg_read(&spi, &spi_cs, icm20602::REG_GYRO_XOUT_H);
        let gxl = reg_read(&spi, &spi_cs, icm20602::REG_GYRO_XOUT_L);
        let gx = ((gxh as u16) << 8 | (gxl as u16)) as i16;

        let gyh = reg_read(&spi, &spi_cs, icm20602::REG_GYRO_YOUT_H);
        let gyl = reg_read(&spi, &spi_cs, icm20602::REG_GYRO_YOUT_L);
        let gy = ((gyh as u16) << 8 | (gyl as u16)) as i16;

        let gzh = reg_read(&spi, &spi_cs, icm20602::REG_GYRO_ZOUT_H);
        let gzl = reg_read(&spi, &spi_cs, icm20602::REG_GYRO_ZOUT_L);
        let gz = ((gzh as u16) << 8 | (gzl as u16)) as i16;

        let t = (((th as u16) << 8) | (tl as u16)) as i16;
        let tc = ((t as f32) / 326.8) + 25.0;
        println!("{:2.2} | {} {} {} | {} {} {}", tc, ax, ay, az, gx, gy, gz);

        board::delay(1000);
    }


    let led0 = board::led::LED0;
    loop {
        led0.toggle_output();
        board::delay(1000);
    }
}

fn transfer(spi: &SpiPeriph, nss: &GpioPin, src: &[u8], dst: &mut[u8]) {
    let mut i = 0;
    let mut j = 0;
    nss.set_output(false);
    loop {
        if i < src.len() && spi.can_tx() {
            // println!("TX: {:02x}", src[i]);
            spi.tx(src[i]);
            i += 1;
        }
        if j < dst.len() && spi.can_rx() {
            dst[j] = spi.rx();
            // println!("RX: {:02x}", dst[j]);
            j += 1;
        }
        if j == dst.len() {
            break;
        }        
    }
    nss.set_output(true);
}

fn reg_write(spi: &SpiPeriph, nss: &GpioPin, reg: u8, value: u8) {
    let cmd = [reg, value];
    let mut buf = [0u8, 0u8];
    transfer(spi, nss, &cmd, &mut buf)
}

fn reg_read(spi: &SpiPeriph, nss: &GpioPin, reg: u8) -> u8 {
    let cmd = [ 0x80 | reg, 0x00];
    let mut buf = [0u8; 2];    
    transfer(spi, nss, &cmd, &mut buf);
    buf[1]
}
    