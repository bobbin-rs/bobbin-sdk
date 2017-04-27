use chip::lpuart::LPUART0;
use hal::pcc;
use hal::lpuart;
use hal::port;
use pin;

// UART0 using PTA2 / PTA3 AltFn6


pub fn serial0() -> lpuart::LpuartDevice {
    pcc::set_lpuart_enabled(LPUART0, true);    
    pcc::set_lpuart_clock(LPUART0, 0b110); // Use SPLLDIV2_CLK = 40Mhz
    
    unsafe {
        LPUART0.with_global(|r| r.set_rst(1));
        LPUART0.with_global(|r| r.set_rst(0));        
    }

    // let rx = pin::pta2().into_output();
    // let tx = pin::pta3().into_output();
    // loop {
    //     tx.set(true);
    //     rx.set(false);
    //     ::delay(200);
    //     tx.set(false);
    //     rx.set(true);
    //     ::delay(200);
    // }
    let _rx = pin::pta2().into_altfn(6);
    let _tx = pin::pta3().into_altfn(6);

    

    let u = lpuart::device(LPUART0);
    u.set_osr(0b1111).set_sbr(22).set_te(true);    
    u
}

// pub unsafe fn uart0_unchecked(rx: port::PinUnknown, tx: port::PinUnknown) -> uart::UartDevice {
//     let tx = tx.into_altfn_unchecked();
//     let rx = rx.into_altfn_unchecked();
//     uart::device(UART0, tx, rx, 65) 
// }