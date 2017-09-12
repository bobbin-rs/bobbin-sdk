#![no_std]
#![no_main]
#![allow(dead_code)]

#[macro_use]
extern crate arduino_zero as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("[start] Running tests for arduino-zero");
    // test_systick();
    // test_dma();
    // test_adc();
    test_i2c();
    println!("[done] All tests passed");
    loop {}
}


fn test_systick() {
    use board::hal::systick;

    let reload_value = 1000;

    // println!("# Disable Systick");
    systick::set_enabled(false);
    assert!(!systick::enabled());

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

    // println!("# Start Test");
    systick::set_enabled(true);
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

fn test_dma() {
    // use board::hal::dmac;
    use board::hal::dmac::*;
    use core::mem;
    

    let mut src = [0u8; 32];
    let mut dst = [0u8; 32];

    for (i, s) in src.iter_mut().enumerate() {
        *s = i as u8;
    }

    let dma = DMAC;
    let ch = DMAC_CH0;    

    dma.pm_set_enabled(true);
    dma.set_ctrl(|r| r.set_swrst(1));

    let mut buf = [0u8; 1024];
    let ptr = unsafe {
        let mut ptr = buf.as_mut_ptr();
        while ptr as u32 % 16 != 0 {        
            ptr = ptr.offset(1);
        }
        ptr      
    };

    let desc: &mut Transfer = unsafe { mem::transmute(ptr) };
    let wb: &mut Transfer = unsafe { mem::transmute(ptr.offset(16))};
    let desc_addr = desc as *mut Transfer as u32;
    let wb_addr = wb as *mut Transfer as u32;
    
    {
        // SRCADDR / DSTADDR must be the ending values after a transfer
        unsafe {
            desc.set_srcaddr(|_| Srcaddr(src.as_ptr().offset(src.len() as isize) as u32));
            desc.set_dstaddr(|_| Dstaddr(dst.as_mut_ptr().offset(dst.len() as isize) as u32));
        }
        desc.set_btcnt(|_| Btcnt(src.len() as u16));
        desc.with_btctrl(|r| r.set_dstinc(1).set_srcinc(1).set_valid(1));
    }    


    // Set Descriptor Base
    dma.set_baseaddr(|_| Baseaddr(desc_addr));
    // Set Writeback Base
    dma.set_wrbaddr(|_| Wrbaddr(wb_addr));

    // Set Priority Level 0 Enabled
    dma.with_ctrl(|r| r.set_lvlen(0, 1));

    // Enable DMAC
    dma.with_ctrl(|r| r.set_dmaenable(1));

    // Set Channel ID
    dma.set_chid(|r| r.set_id(ch.index() as u8));
    dma.set_chctrla(|r| r.set_swrst(1));
    dma.set_chid(|r| r.set_id(ch.index() as u8));
    dma.set_chctrlb(|r| r.set_trigact(0x3));

    // Set Channel Enabled
    dma.set_chid(|r| r.set_id(ch.index() as u8));
    dma.set_chctrla(|r| r.set_enable(1));

    dma.set_swtrigctrl(|r| r.set_swtrig(ch.index(), 1));

    loop {
        let f = dma.chintflag();
        if f.terr() != 0 {
            println!("[fail] Transfer Error");
            break;
        }
        if f.tcmpl() != 0 {
            break;
        }
        if f.susp() != 0 {
            println!("[fail] Transfer Suspended");
            break;
        }
    }    

    assert_eq!(&src[..], &dst[..]);
    println!("[pass] DMA OK");    
}

fn test_adc() {    
    pub use board::hal::adc::*;
    pub use board::hal::port::*;
    use board::common::bits::*;
    
    init();
    let v_temp: U12 = ADC_TEMP.analog_read();
    let v_bandgap: U12 = ADC_BANDGAP.analog_read();
    let v_scaled_core: U12 = ADC_SCALED_CORE.analog_read();
    let v_scaled_io: U12 = ADC_SCALED_IO.analog_read();
    println!("# {} {} {} {}", v_temp, v_bandgap, v_scaled_core, v_scaled_io);

    assert!(v_temp.value() > 1000 && v_temp.value() < 1200);
    assert!(v_bandgap.value() > 1450 && v_bandgap.value() < 1600);
    assert!(v_scaled_core.value() > 350 && v_scaled_core.value() < 450);
    assert!(v_scaled_io.value() > 1000 && v_scaled_io.value() < 1100);

    println!("[pass] ADC OK");
}

fn test_i2c() {
    pub use board::hal::i2c::*;
    pub use board::hal::port::*;

    // PA22, PA23 - SERCOM3

    let addr: u8 = 0x60;
    let i2c = SERCOM3;
   
    let p0 = PA22;
    let p1 = PA23;

    i2c.pm_set_enabled(true);

    p0.set_mode_output();
    for _ in 0..10 {
        p0.toggle_output();
        board::delay(10);
    }

    p0.mode_pad_0(&i2c);
    p1.mode_pad_1(&i2c);
    

    println!("Starting I2c");
    println!("Initializing");
    i2c.init_i2c(240);
    println!("Enabling");
    i2c.enable_i2c();
    println!("CTRLA:  {:?}", i2c.i2cm().ctrla());
    println!("CTRLB:  {:?}", i2c.i2cm().ctrlb());
    println!("BAUD:   {:?}", i2c.i2cm().baud());
    println!("STATUS: {:?}", i2c.i2cm().status());
    println!("idle? {} owner? {}", i2c.bus_idle(), i2c.bus_owner());

    let cmd = [0x0c_u8];
    let mut buf = [0u8];
    // i2c.write(addr, &cmd);
    // i2c.read(addr, &mut buf);
    i2c.transfer(addr, &cmd, &mut buf);
    println!("0x{:02x}", buf[0]);

    println!("Read Register");
    assert_eq!(i2c.read_reg(addr, 0x0c), 0xc4);
    println!("Done Reading Register");
    
    println!("Mode:  0x{:02x}", i2c.read_reg(addr, 0x26));   
    
    i2c.write_reg(addr, 0x26, 0xb8); // OSR = 128
    i2c.write_reg(addr, 0x13, 0x06); // Enable Data Flags
    i2c.write_reg(addr, 0x26, 0xb9); // Set Active
    println!("Mode:  0x{:02x}", i2c.read_reg(addr, 0x26));

    while i2c.read_reg(addr, 0x00) != 0x04 {}
    println!("Ready to read");

    loop {
        // while i2c.read_reg(addr, 0x00) != 0x04 {}    
        let mut buf = [0u8; 5];
        i2c.transfer(addr, &[0x01], &mut buf);
        println!("# {:?}", buf);
        // assert!(buf[0] == 0 && buf[1] != 0 && buf[2] != 0 && buf[3] != 0 && buf[4] != 0);
        break
    }
        

    

    println!("[pass] I2C OK");    
}