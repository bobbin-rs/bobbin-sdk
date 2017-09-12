#![no_std]
#![no_main]
#![allow(dead_code)]

#[macro_use]
extern crate ek_tm4c1294xl as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("[start] Running tests for ek-tm4c1294xl");
    test_systick();
    // test_crc();
    test_dma();
    // test_gpio();
    test_i2c();
    test_spi_lora();
    println!("[done] All tests passed");
    loop {}
}

fn test_systick() {
    use board::hal::systick::{self, ClockSource};

    let reload_value = 10000;

    // println!("# Disable Systick");
    systick::set_enabled(false);
    assert!(!systick::enabled());

    systick::set_clock_source(ClockSource::Internal);

    // println!("# Set Reload Value");
    systick::set_reload_value(reload_value);
    assert_eq!(systick::reload_value(), reload_value);

    // println!("# Set Current Value");
    systick::set_current_value(0);
    assert_eq!(systick::current_value(), 0);

    // println!("# Clear Count Flag");
    let _ = systick::count_flag();
    assert!(!systick::count_flag());


    let mut value_min = systick::current_value();

    systick::set_enabled(true);
    assert!(systick::enabled());    
    assert!(systick::current_value() > 0);
    while !systick::count_flag() {
        let v = systick::current_value();
        if v < value_min {
            value_min = v;
        }
    }
    assert!(value_min < reload_value);
    systick::set_enabled(false);

    println!("[pass] SYSTICK OK");
}

// fn test_crc() {
//     use board::hal::ccm::*;

//     let crc = CCM0;

//     crc.sysctl_set_enabled(true);

//     // NOTE: TODO after full CRC test suite is complete

//     crc.sysctl_set_enabled(false);

//     println!("[pass] CRC OK");
// }

fn test_dma() {
    use board::hal::udma::*;
    use core::mem;

    let mut src = [0u8; 32];
    let mut dst = [0u8; 32];

    for (i, s) in src.iter_mut().enumerate() {
        *s = i as u8;
    }    

    let mut buf = [0u8; 4096];
    let ptr = unsafe {
        let mut ptr = buf.as_mut_ptr();
        while ptr as u32 % 128 != 0 {        
            ptr = ptr.offset(1);
        }
        ptr      
    };
    let desc: &mut Chdesc = unsafe { mem::transmute(ptr) };
    let desc_addr = desc as *const _ as u32;

    let d = UDMA;
    let ch = UDMA_CH0;

    d.sysctl_set_enabled(true);
    d.set_cfg(|r| r.set_masten(1));
    d.set_ctlbase(|r| r.set_addr(desc_addr >> 10));

    d.set_prioset(|r| r.set_set(ch.index(), 1));
    d.set_altclr(|r| r.set_clr(ch.index(), 1));
    d.set_useburstclr(|r| r.set_clr(ch.index(), 1));
    d.set_reqmaskclr(|r| r.set_clr(ch.index(), 1));
    unsafe {
        desc.set_srcendp(|_| Srcendp(src.as_ptr().offset(src.len() as isize) as u32));
        desc.set_dstendp(|_| Dstendp(dst.as_mut_ptr().offset(src.len() as isize)as u32));
    }
    desc.set_chctl(|r| r
        .set_dstinc(0x0)
        .set_dstsize(0x0)
        .set_srcinc(0x0)
        .set_srcsize(0x0)
        .set_xfersize(src.len() as u32)
        .set_xfermode(0x2)
    );

    d.set_enaset(|r| r.set_set(ch.index(), 1));
    d.set_swreq(|r| r.set_swreq(ch.index(), 1));

    while d.enaset().set(ch.index()) != 0 {}

    assert_eq!(&src[..], &dst[..]);
    println!("[pass] DMA OK");
}

fn test_gpio() {
    use board::hal::gpio::*;

    let port = GPIOF;
    let port_out = PF1;
    let port_in = PF2;

    println!("Testing GPIO");

    port.sysctl_set_enabled(true);
    port_out.mode_output();
    port_in.mode_input().pull_up();
    port_in.set_ibe(true);

    println!("Testing Configured");
    port_out.set_output(true);
    assert_eq!(port_in.input(), true);    
    port_in.clr_ris();
    assert!(!port_in.test_ris());
    
    port_out.set_output(false);
    assert_eq!(port_in.input(), false);
    assert!(port_in.test_ris());
    port_in.clr_ris();
    assert!(!port_in.test_ris());

    port_out.toggle_output();
    assert_eq!(port_in.input(), true);
    assert!(port_in.test_ris());
    port_in.clr_ris();
    assert!(!port_in.test_ris());

    println!("[pass] GPIO OK");
}

fn test_i2c() {
    use board::hal::gpio::*;
    use board::hal::i2c::*;

    let addr: u8 = 0x60;

    let port = GPIOB;
    let port_scl = PB2;
    let port_sda = PB3;

    let i2c = I2C0;

    port.sysctl_set_enabled(true);
    port_scl.mode_i2c_scl(&i2c);
    port_sda.mode_i2c_sda(&i2c);

    i2c.sysctl_set_enabled(true);

    i2c.init_master();

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
    
    i2c.sysctl_set_enabled(false);
    port_scl.mode_disabled();
    port_sda.mode_disabled();
    println!("[pass] I2C OK");
}

/// RFM9x LoRa Radio on pins D10-D13
fn test_spi_lora() {
    use board::hal::gpio::*;
    use board::hal::ssi::*;

    let spi = SSI2;
    let port = GPIOD;
    let spi_miso = PD1; // D13
    let spi_mosi = PD0; // D12
    let spi_sck = PD3; // D11
    let spi_nss = PC7; // D10

    spi.sysctl_enable();
    port.sysctl_enable();
    GPIOC.sysctl_enable();

    // NOTE: Pins must be set with output speed HIGH or leading edge
    // of transmission will occasionally be missed.

    spi_miso.mode_ssi_dat0(&spi);
    spi_mosi.mode_ssi_dat1(&spi);
    spi_sck.mode_ssi_clk(&spi);
    spi_nss.mode_output().pull_up().set_output(true);

    spi.init();

    let test_data = [(0x42, 0x12), (0x01, 0x09), (0x02, 0x1a), (0x03, 0x0b), (0x04, 0x00), (0x05, 0x52), (0x06, 0x6c)];

    for &(tx, rx) in test_data.iter() {
        // println!("0x{:02x}: 0x{:02x}", tx, rx);
        assert_eq!(reg_read(&spi, &spi_nss, tx), rx);
    }

    println!("[pass] SPI OK");
    spi.sysctl_disable();
    spi_sck.mode_disabled();
    spi_mosi.mode_disabled();
    spi_miso.mode_disabled();
    spi_nss.mode_disabled();

    fn reg_read(spi: &SsiPeriph, nss: &GpioPin, reg: u8) -> u8 {
        let cmd = [reg, 0xff];
        let mut buf = [0u8, 0u8];
        nss.set_output(false);
        spi.transfer(&cmd, &mut buf);
        nss.set_output(true);
        buf[1]
    }

}