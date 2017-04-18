use chip::sercom::SERCOM5;
use driver::usart;
use driver::port::PMux;
use pin;
use hal::pm;
use chip::gclk;

pub fn usart2(_baud: u32) -> usart::UsartDevice {
    pm::set_sercom_enabled(SERCOM5, true);

    // Set GCLK_GEN0 as source for SERCOM

    unsafe {
        gclk::GCLK.set_clkctrl(gclk::Clkctrl(0)
            .set_id(0x14 + 5)
            .set_gen(0x0)
            .set_clken(1)
        );
        // Wait for synchronization
        while gclk::GCLK.status().syncbusy() != 0 {}
    }

    let _rx = pin::pb22().into_pmux(PMux::PMuxD);
    let _rx = pin::pb23().into_pmux(PMux::PMuxD);
    let u = usart::device(SERCOM5);
    u.configure(63018, 1, 3);    
    u
}

pub unsafe fn usart2_unchecked() -> usart::UsartDevice {
    usart::device(SERCOM5)
}