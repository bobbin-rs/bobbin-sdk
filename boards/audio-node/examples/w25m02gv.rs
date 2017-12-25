

#![no_std]
#![no_main]

#[macro_use]
extern crate audio_node as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Flash Driver Test");    
    test_spi_flash();    
    loop {}
}

fn test_spi_flash() {
    use board::hal::gpio::*;
    use board::hal::spi::*;

    board::delay(100);

    let spi = SPI2;

    let spi_mosi = PB15;
    let spi_miso = PB14;
    let spi_sck = PB13;
    let spi_nss = PB12;

    spi.rcc_enable();
    GPIOB.rcc_enable();

    // NOTE: Pins must be set with output speed HIGH or leading edge
    // of transmission will occasionally be missed.

    spi_miso.mode_spi_miso(&spi).speed_high().push_pull().pull_down();
    spi_mosi.mode_spi_mosi(&spi).speed_high().push_pull().pull_down();
    spi_sck.mode_spi_sck(&spi).speed_high().push_pull().pull_down();
    // spi_nss.mode_spi_nss(&spi).speed_high().push_pull();
    spi_nss.mode_output().set_output(true);
    board::delay(10);
    spi_nss.set_output(false);
    board::delay(10);
    spi_nss.set_output(true);

    spi.set_config(|cfg| cfg
        .set_data_size(DataSize::Bits8)
        .set_master(true)
        .set_baud_divider(0b100.into())
    );

    spi.with_cr2(|r| r.set_frxth(1));
    spi.set_output_enabled(true).set_enabled(true);

    let pins: [GpioPin; 1] = [spi_nss.into()];
    let mut tx_buf = [SpiAction::Idle; 16];
    let mut rx_buf = [0u8; 16];
    let s = SpiDriver::new(spi, &pins, &mut tx_buf, &mut rx_buf);
    s.enable_irq(&spi.irq_spi());

    loop {
        let id = s.read_id();
        if id[0] != 0 {
            println!("ID: {:02x} {:02x} {:02x}", id[0], id[1], id[2]);
            break;
        } else {
            board::delay(100);
        }
    }

    s.reset();
    board::delay(100);

    // Status Register A0 => 0x7c | 0111 1100 | bp3 bp2 bp1 bp0 tb
    // Status Register B0 => 0x18 | 0001 1000 | ecc-e buf
    // Status Register C0 => 0x00 | 0000 0000 |

    println!("Initial Status");
    for r in [0xa0, 0xb0, 0xc0].iter() {
        println!("{:02x}: {:02x}", r, s.read_status_register(*r));
    }

    println!("Disable Write Protect");
    s.write_status_register(0xa0, 0);

    board::delay(100);
    for r in [0xa0, 0xb0, 0xc0].iter() {
        println!("{:02x}: {:02x}", r, s.read_status_register(*r));
    }

    if false {
        println!("Dump Current Data");
        let mut buf = [0u8; 32];
        s.read_page(0x0000, &mut buf);
        dump(&buf);
    }


    if true {
        println!("Block Erase");
        s.write_enable();        
        println!("  C0 = {:02x}", s.read_status_register(0xc0));
        println!("  Write Enabled? {}", s.write_enabled());
        s.block_erase(0x0000);
        
        println!("Writing Data");
        let out_buf = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
        dump(&out_buf);
        s.write_enable();        
        s.load_program_data(0x0000, &out_buf);
        println!("  C0 = {:02x}", s.read_status_register(0xc0));
        println!("  Write Enabled? {}", s.write_enabled());

        println!("Program Execute");
        s.program_execute(0x0000);
        println!("  C0 = {:02x}", s.read_status_register(0xc0));
        println!("  Write Enabled? {}", s.write_enabled());

        while s.busy() {}

        for r in [0xa0, 0xb0, 0xc0].iter() {
            println!("{:02x}: {:02x}", r, s.read_status_register(*r));
        }


        println!("Page Data Read");
        s.page_data_read(0x0000);

        for r in [0xa0, 0xb0, 0xc0].iter() {
            println!("{:02x}: {:02x}", r, s.read_status_register(*r));
        }

        println!("Read Page 0");
        let mut buf = [0u8; 64];
        s.read_page(0x0000, &mut buf);

        for r in [0xa0, 0xb0, 0xc0].iter() {
            println!("{:02x}: {:02x}", r, s.read_status_register(*r));
        }
        
        dump(&buf);

        println!("Read Page 1");
        let mut buf = [0u8; 64];
        s.read_page(0x0001, &mut buf);

        for r in [0xa0, 0xb0, 0xc0].iter() {
            println!("{:02x}: {:02x}", r, s.read_status_register(*r));
        }
        
        dump(&buf);

    }

    if false {
        println!("B0 => 0x58");

        s.write_status_register(0xb0, 0x58);
        
        println!("Reading Unique ID Page");
        {
            s.page_data_read(0x0000);
            let mut buf = [0u8; 32 * 16];
            s.read_page(0x0000, &mut buf);
            dump(&buf);
        }

        println!("Reading Parameter Page");
        {
            s.page_data_read(0x0000);
            while s.busy() {}
            let mut buf = [0u8; 256];
            s.read_page(0x0000, &mut buf);
            dump(&buf);
        }
    }
    
    loop {}

}

fn dump(buf: &[u8]) {
    for (i, b) in buf.iter().enumerate() {
        if i > 0 && i % 16 == 0 {
            println!("");
        }
        print!(" {:02x}", b);
    }
    println!("");
}

pub trait SpiFlash {
    fn reset(&self);
    fn read_id(&self) -> [u8; 3];
    fn read_status_register(&self, reg: u8) -> u8;
    fn block_erase(&self, page: u16);
    fn page_data_read(&self, page: u16);
    fn read_page(&self, addr: u16, buf: &mut [u8]);
    fn load_program_data(&self, addr: u16, buf: &[u8]);
    fn program_execute(&self, page: u16);

    fn write_enable(&self);
    fn write_disable(&self);
        
    fn write_status_register(&self, reg: u8, value: u8);
    fn with_status_register<F: FnOnce(u8) -> u8>(&self, reg: u8, f: F);
    fn write_enabled(&self) -> bool {
        self.read_status_register(0xc0) & 2 != 0
    }
    fn busy(&self) -> bool {
        self.read_status_register(0xc0) & 1 != 0
    }
}

impl<'a> SpiFlash for board::hal::spi::SpiDriver<'a> {
    fn read_id(&self) -> [u8; 3] {
        let mut buf = [0u8; 3];
        self.transfer_blocking(0, &[0x9f, 0xff], &mut buf);
        buf
    }

    fn read_status_register(&self, reg: u8) -> u8 {
        let mut buf = [0u8; 1];
        self.transfer_blocking(0, &[0x0f, reg], &mut buf);
        return buf[0]        
    }

    fn write_status_register(&self, reg: u8, value: u8) {
        let mut buf = [];
        self.transfer_blocking(0, &[0x1f, reg, value], &mut buf);
    }

    fn with_status_register<F: FnOnce(u8) -> u8>(&self, reg: u8, f: F) {
        let v = self.read_status_register(reg);
        self.write_status_register(reg, f(v));
    }

    fn block_erase(&self, page: u16) {
        while self.busy() {}        
        let cmd = [0xd8, 0x00, (page >> 8) as u8, page as u8];
        let mut tmp = [];
        self.transfer_blocking(0, &cmd, &mut tmp);
    }

    fn program_execute(&self, page: u16) {
        while self.busy() {}        
        let cmd = [0x10, 0x00, (page >> 8) as u8, page as u8];
        let mut tmp = [];
        self.transfer_blocking(0, &cmd, &mut tmp);
    }


    fn page_data_read(&self, page: u16) {
        while self.busy() {}        
        let cmd = [0x13, 0x00, (page >> 8) as u8, page as u8];
        let mut tmp = [];
        self.transfer_blocking(0, &cmd, &mut tmp);
    }

    fn read_page(&self, addr: u16, buf: &mut [u8]) {
        while self.busy() {}        
        self.transfer_blocking(0, &[0x03, (addr >> 8) as u8, addr as u8, 0xff], buf)
    }

    fn load_program_data(&self, addr: u16, buf: &[u8]) {
        while self.busy() {}        
        self.transfer_start(0);
        self.send_blocking(&[0x03, (addr >> 8) as u8, addr as u8, 0xff]);
        self.send_blocking(buf);
        self.transfer_end(0);
    }

    fn write_enable(&self) {
        let mut tmp = [];
        self.transfer_blocking(0, &[0x06], &mut tmp);
    }

    fn write_disable(&self) {
        let mut tmp = [];
        self.transfer_blocking(0, &[0x04], &mut tmp);
    }

    fn reset(&self) {
        let mut tmp = [];
        self.transfer_blocking(0, &[0xff], &mut tmp);
    }
}