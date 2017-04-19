use chip::uart::UART0;
use hal::sim;
use hal::uart;
use hal::port;

pub fn uart0(rx: port::PinUnknown, tx: port::PinUnknown) -> uart::UartDevice {
    sim::set_uart_enabled(UART0, true);    

    let tx = tx.into_altfn(3);
    let rx = rx.into_altfn(3);
    let u = uart::device(UART0, tx, rx, 65);    
    u.enable();
    u
}

pub unsafe fn uart0_unchecked(rx: port::PinUnknown, tx: port::PinUnknown) -> uart::UartDevice {
    let tx = tx.into_altfn_unchecked();
    let rx = rx.into_altfn_unchecked();
    uart::device(UART0, tx, rx, 65) 
}