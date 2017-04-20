use chip::uart0::UART0;
use hal::sim;
use hal::uart0;
use hal::port;

pub fn uart0(rx: port::PinUnknown, tx: port::PinUnknown) -> uart0::Uart0Device {
    sim::set_uart0_enabled(true);  
    sim::set_uart0_src(0x1);  

    let tx = tx.into_altfn(2);
    let rx = rx.into_altfn(2);
    let u = uart0::device(UART0, tx, rx, 104);    
    u.enable();
    u
}

pub unsafe fn uart0_unchecked(rx: port::PinUnknown, tx: port::PinUnknown) -> uart0::Uart0Device {
    let tx = tx.into_altfn_unchecked();
    let rx = rx.into_altfn_unchecked();
    uart0::device(UART0, tx, rx, 104) 
}