use chip::uart::UART0;
use hal::sysctl;
use hal::uart;
use hal::gpio;

pub fn uart0(rx: gpio::PinUnknown, tx: gpio::PinUnknown) -> uart::UartDevice {
    sysctl::set_uart_enabled(UART0, true);

    let tx = tx.into_altfn(1);
    let rx = rx.into_altfn(1);
    let u = uart::device(UART0, tx, rx);
    u.enable(115_200);
    u
}

pub unsafe fn uart0_unchecked(rx: gpio::PinUnknown, tx: gpio::PinUnknown) -> uart::UartDevice {
    let tx = tx.into_altfn_unchecked();
    let rx = rx.into_altfn_unchecked();
    uart::device(UART0, tx, rx) 
}