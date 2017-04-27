use chip::lpuart::{LPUART0, LPUART1};
use hal::pcc;
use hal::lpuart;
use pin;

// serial0 using UART0 / PTA2 / PTA3 AltFn6
// serial1 using UART1 / PTC6 / PTC7 AltFn2

pub fn serial0() -> lpuart::LpuartDevice {
    pcc::set_lpuart_enabled(LPUART0, true, 0b110);   // Use SPLLDIV2_CLK = 40Mhz 
    
    unsafe {
        LPUART0.with_global(|r| r.set_rst(1));
        LPUART0.with_global(|r| r.set_rst(0));        
    }

    let _rx = pin::pta2().into_altfn(6);
    let _tx = pin::pta3().into_altfn(6);

    

    let u = lpuart::device(LPUART0);
    u.set_osr(0b1111).set_sbr(22).set_te(true);    
    u
}

pub fn serial0_unchecked() -> lpuart::LpuartDevice {
    lpuart::device(LPUART0)
}

// pub unsafe fn uart0_unchecked(rx: port::PinUnknown, tx: port::PinUnknown) -> uart::UartDevice {
//     let tx = tx.into_altfn_unchecked();
//     let rx = rx.into_altfn_unchecked();
//     uart::device(UART0, tx, rx, 65) 
// }


pub fn serial1() -> lpuart::LpuartDevice {
    pcc::set_lpuart_enabled(LPUART1, true, 0b110);   // Use SPLLDIV2_CLK = 40Mhz 

    unsafe {
        LPUART1.with_global(|r| r.set_rst(1));
        LPUART1.with_global(|r| r.set_rst(0));        
    }

    let _rx = pin::ptc6().into_altfn(2);
    let _tx = pin::ptc7().into_altfn(2);

    let u = lpuart::device(LPUART1);
    u.set_osr(0b1111).set_sbr(22).set_te(true);    
    u
}

pub fn serial1_unchecked() -> lpuart::LpuartDevice {
    lpuart::device(LPUART1)
}
