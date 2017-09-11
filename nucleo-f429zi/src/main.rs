#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f429zi as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("[start] Running tests for nucleo-f429zi");
    test_i2c();
    test_spi_lora();
    println!("[done] All tests passed");
    loop {}
}


fn test_i2c() {
    use board::hal::gpio::*;
    use board::hal::i2c::*;
    use board::common::bits::*;

    let addr: U7 = U7::from(0x60);

    let i2c = I2C1;
    let i2c_port = GPIOB;
    let i2c_scl = PB8; // D15
    let i2c_sda = PB9; // D14

    i2c.rcc_enable();
    i2c_port.rcc_enable();

    // Attached to MPL3115A2 
    // NOTE: SCL and SCA must have pull-up resistors.
    // NOTE: HSI Clock must be enabled.

    i2c_scl.mode_i2c_scl(&i2c).open_drain();
    i2c_sda.mode_i2c_sda(&i2c).open_drain();

    // i2c.set_config(|c| c.set_timing(0x8.into(), 0x3.into(), 0x0.into(), 0xd.into(), 0x12.into()));
    i2c.set_enabled(false);
    // i2c.set_timingr(|_| Timingr(0x00300619));
        // Set Peripheral Clock Frequency = 42Mhz
    i2c.with_cr2(|r| r.set_freq(42));
    
    // Set Clock Speed = 100khz (divisor = 42Mhz / 100khz = 420)
    i2c.set_ccr(|_| Ccr(0).set_f_s(0).set_duty(0).set_ccr(420));

    // Set Rise Time Register = 43
    i2c.set_trise(|_| Trise(0).set_trise(43));

    assert_eq!(i2c.read_reg(addr, 0x0c), 0xc4);
   
    // println!("Mode:  0x{:08x}", i2c.read_reg(addr, 0x26));
    

    i2c.write_reg(addr, 0x26, 0xb8); // OSR = 128
    i2c.write_reg(addr, 0x13, 0x06); // Enable Data Flags
    i2c.write_reg(addr, 0x26, 0xb9); // Set Active
    // println!("Mode:  0x{:08x}", i2c.read_reg(addr, 0x26));

    loop {
        while i2c.read_reg(addr, 0x00) != 0x04 {}    
        let mut buf = [0u8; 5];
        i2c.transfer(addr, &[0x01], &mut buf);
        println!("# {:?}", buf);
        // assert!(buf[0] == 0 && buf[1] != 0 && buf[2] != 0 && buf[3] != 0 && buf[4] != 0);
        break
    }

    

    i2c_port.rcc_disable();
    i2c.rcc_disable();
    println!("[pass] I2C OK");
}


fn test_spi_lora() {
    use board::hal::gpio::*;
    use board::hal::spi::*;

    let spi = SPI1;
    let port = GPIOA;
    let spi_miso = PA6; // A5
    let spi_mosi = PA7; // A6
    let spi_sck = PA5;
    let spi_nss = PD14; // D10

    spi.rcc_enable();
    port.rcc_enable();
    GPIOD.rcc_enable();

    // NOTE: Pins must be set with output speed HIGH or leading edge
    // of transmission will occasionally be missed.

    spi_miso.mode_spi_miso(&spi).speed_high().pull_up();
    spi_mosi.mode_spi_mosi(&spi).speed_high().push_pull();
    spi_sck.mode_spi_sck(&spi).speed_high().push_pull();
    spi_nss.mode_output();

    spi.set_config(|cfg| cfg
        .set_frame_size(FrameSize::Bits8)
        .set_master(true)
        .set_baud_divider(0b100.into())
    );

    spi.set_output_enabled(true).set_enabled(true);


    let test_data = [(0x42, 0x12), (0x01, 0x09), (0x02, 0x1a), (0x03, 0x0b), (0x04, 0x00), (0x05, 0x52), (0x06, 0x6c)];

    for &(tx, rx) in test_data.iter() {
        // println!("0x{:02x}: 0x{:02x}", tx, rx);
        assert_eq!(reg_read(&spi, &spi_nss, tx), rx);
    }


    println!("[pass] SPI OK");
    spi.rcc_disable();

    fn transfer(spi: &SpiPeriph, nss: &GpioPin, src: &[u8], dst: &mut[u8]) {
        let mut i = 0;
        let mut j = 0;
        nss.set_output(false);
        loop {
            if i < src.len() && spi.can_tx() {
                spi.tx(src[i]);
                i += 1;
            }
            if j < dst.len() && spi.can_rx() {
                dst[j] = spi.rx();
                j += 1;
            }
            if j == dst.len() {
                break;
            }        
        }
        nss.set_output(true);
    }

    fn reg_read(spi: &SpiPeriph, nss: &GpioPin, reg: u8) -> u8 {
        let cmd = [reg, 0xff];
        let mut buf = [0u8, 0u8];
        transfer(spi, nss, &cmd, &mut buf);
        buf[1]
    }
    
}
