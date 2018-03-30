#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f746zg as board;

use board::mcu::pin::*;
use board::mcu::spi::*;


#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Running SPI");

    let spi = SPI1;

    let spi_sck = PA5;  // D13
    let spi_miso = PA6; // D12
    let spi_mosi = PA7; // D11
    let spi_nss = PD14; // D10

    spi.gate_enable();
    spi_sck.port().gate_enable();
    spi_miso.port().gate_enable();
    spi_mosi.port().gate_enable();
    spi_nss.port().gate_enable();    

    spi_sck.connect_to(spi);
    spi_miso.connect_to(spi);
    spi_mosi.connect_to(spi);

    // NOTE: Pins must be set with output speed HIGH or leading edge
    // of transmission will occasionally be missed.

    spi_sck.speed_high().push_pull();
    spi_miso.speed_high().pull_up();
    spi_mosi.speed_high().push_pull();
    spi_nss.mode_output().set_output(true).set_output(false).set_output(true);

    spi.set_config(|cfg| cfg
        .set_data_size(DataSize::Bits8)
        .set_master(true)
        .set_baud_divider(0b011.into())
    );
    spi.with_cr1(|r| r.set_cpha(0).set_cpol(0));
    spi.with_cr2(|r| r.set_frxth(1));
    spi.set_output_enabled(true).set_enabled(true);

    println!("SPI Configuration Complete");

    let fram = Mb85rs64 { spi: spi.into(), nss: spi_nss.into() };
    let buf = fram.device_id();
    print!("ID: ");
    for i in 0..buf.len() {
        print!("{:02x} ", buf[i]);
    }
    println!("");
   
    {
        let mut out_buf = [0u8; 0x40];
        let mut in_buf = [0u8; 0x40];
        for i in 0..0x40 {
            out_buf[i] = i as u8;
        }    
        fram.write(0x00, &out_buf);        
        fram.read(0x00, &mut in_buf);
        dump(&in_buf);
    }

    {
        let out_buf = [0u8; 0x40];
        let mut in_buf = [0u8; 0x40];
        fram.write(0x00, &out_buf);        
        fram.read(0x00, &mut in_buf);
        dump(&in_buf);
    }       
    loop {}

}

pub fn dump(buf: &[u8]) {
    for i in 0..buf.len() {
        if i % 16 == 0 {
            if i > 0 {
                println!("");
            }
            print!("{:04x}:", i)
        }
        if i % 8 == 0 {
            print!(" ");
        }
        print!(" {:02x}", buf[i]);
    }
    println!("");    
}


pub const WREN: u8 = 0b0000_0110;
pub const WRDI: u8 = 0b0000_0100;
pub const RDSR: u8 = 0b0000_0101;
pub const WRSR: u8 = 0b0000_0001;
pub const READ: u8 = 0b0000_0011;
pub const WRITE: u8 = 0b0000_0010;
pub const RDID: u8 = 0b1001_1111;

pub struct Mb85rs64 {
    spi: SpiPeriph,
    nss: GpioPin,
}

impl Mb85rs64 {
    pub fn with_nss<F: FnOnce()->R, R>(&self, f: F) -> R {
        self.nss.set_output(false);
        let r = f();
        self.nss.set_output(true);
        r
    }

    pub fn transfer(&self, tx_buf: &[u8], rx_buf: &mut[u8]) {
        while self.spi.busy() {}
        for i in 0..tx_buf.len() {
            while !self.spi.can_tx() {}
            self.spi.tx(tx_buf[i]);
            while !self.spi.can_rx() {}
            let _: u8 = self.spi.rx();
        }
        for i in 0..rx_buf.len() {
            while !self.spi.can_tx() {}
            self.spi.tx(0xffu8);
            while !self.spi.can_rx() {}
            rx_buf[i] = self.spi.rx();
        }
        while self.spi.busy() {}
    }


    pub fn device_id(&self) -> [u8; 4] {
        self.with_nss(|| {
            let mut buf = [0u8; 4];
            self.transfer(&[RDID], &mut buf);
            buf
        })
    }

    pub fn wren(&self) {
        self.transfer(&[WREN], &mut []);
    }

    pub fn wrdi(&self) {
        self.transfer(&[WRDI], &mut []);
    }
    
    pub fn rdsr(&self) -> u8 {
        let mut buf = [0u8];
        self.transfer(&[RDSR], &mut buf);
        buf[0]
    }

    pub fn status(&self) -> u8 {
        self.with_nss(|| self.rdsr())
    }

    pub fn write_enable(&self) {
        self.with_nss(|| self.wren())
    }

    pub fn write_disable(&self) {
        self.with_nss(|| self.wrdi())        
    }

    pub fn write(&self, addr: u16, buf: &[u8]) {
        self.with_nss(|| self.wren());
        self.with_nss(|| {
            // println!("{:02x} {:02x}", addr, self.rdsr());
            self.transfer(&[WRITE, (addr >> 8) as u8, addr as u8], &mut[]);
            self.transfer(buf, &mut []);
        })
    }

    pub fn read(&self, addr: u16, buf: &mut [u8]) {
        self.with_nss(|| {
            self.transfer(&[READ, (addr >> 8) as u8, addr as u8], buf);
        })
    }
}