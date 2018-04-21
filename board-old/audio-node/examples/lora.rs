
#![no_std]
#![no_main]

#[macro_use]
extern crate audio_node as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("LORA Driver Test");    
    test_spi_lora();    
    loop {}
}

/// RFM69 Radio on pins D10-D13
fn test_spi_lora() {
    use board::hal::gpio::*;
    use board::hal::spi::*;

    board::delay(100);

    let spi = SPI1;

    let spi_mosi = PA7;
    let spi_miso = PA6;
    let spi_sck = PA5;
    let spi_nss = PA4;

    spi.rcc_enable();
    GPIOA.rcc_enable();

    // NOTE: Pins must be set with output speed HIGH or leading edge
    // of transmission will occasionally be missed.

    spi_miso.mode_spi_miso(&spi).speed_high().pull_up();
    spi_mosi.mode_spi_mosi(&spi).speed_high().push_pull();
    spi_sck.mode_spi_sck(&spi).speed_high().push_pull();
    // spi_nss.mode_spi_nss(&spi).speed_high().push_pull();
    spi_nss.mode_output().set_output(true);

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

    let test_data = [(0x10, 0x24), (0x01, 0x00), (0x02, 0x00), (0x03, 0x1a), (0x04, 0x0b), (0x05, 0x00), (0x06, 0x52)];


    for &(tx, rx) in test_data.iter() {
        let a = s.reg_read(0, tx);
        println!("0x{:02x}: 0x{:02x} = 0x{:02x}", tx, rx, a);
        assert_eq!(rx, a);
    }

    println!("--- transfer ---");

    // let tx_buf: [u8; 7] = [0x01, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55];    
    let tx_buf = [0x01];
    let mut rx_buf = [0u8; 6];    

    s.transfer(0, &tx_buf, &mut rx_buf);
    for i in 0..rx_buf.len() {        
        println!("0x{:02x}", rx_buf[i]);
    }
        
    assert_eq!(rx_buf[0], 0x00);
    assert_eq!(rx_buf[1], 0x00);
    assert_eq!(rx_buf[2], 0x1a);
    assert_eq!(rx_buf[3], 0x0b);
    assert_eq!(rx_buf[4], 0x00);
    assert_eq!(rx_buf[5], 0x52);

    println!("--- commands --- ");

    s.enqueue(SpiAction::Start(0));
    s.enqueue(SpiAction::Write(0x01));    
    s.enqueue(SpiAction::Repeat(5));
    s.enqueue(SpiAction::Transfer(0x55));
    s.enqueue(SpiAction::Stop(0));

    let mut rx_buf = [0u8; 6];    

    s.read(&mut rx_buf);
    for i in 0..rx_buf.len() {        
        println!("0x{:02x}", rx_buf[i]);
    }
        
    assert_eq!(rx_buf[0], 0x00);
    assert_eq!(rx_buf[1], 0x00);
    assert_eq!(rx_buf[2], 0x1a);
    assert_eq!(rx_buf[3], 0x0b);
    assert_eq!(rx_buf[4], 0x00);
    assert_eq!(rx_buf[5], 0x52);

    println!("[pass] SPI OK");
    spi.rcc_disable();
    spi_sck.mode_analog();
    spi_mosi.mode_analog();
    spi_miso.mode_analog();
    spi_nss.mode_analog();

}
