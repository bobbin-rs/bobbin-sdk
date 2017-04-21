use chip::sercom::SERCOM0;
use hal::usart;
use hal::port::PMux;
use pin;
use hal::pm;
use chip::gclk;

pub fn usart0(_baud: u32) -> usart::UsartDevice {
    pm::set_sercom_enabled(SERCOM0, true);

    // Set GCLK_GEN0 as source for SERCOM

    unsafe {
        gclk::GCLK.set_clkctrl(gclk::Clkctrl(0)
            .set_id(0x14 + 0)
            .set_gen(0x0)
            .set_clken(1)
        );
        // Wait for synchronization
        while gclk::GCLK.status().syncbusy() != 0 {}
    }

    let _rx = pin::pa10().into_pmux(PMux::PMuxC);
    let _rx = pin::pa11().into_pmux(PMux::PMuxC);
    let u = usart::device(SERCOM0);
    u.configure(63018, 1, 3);    
    u
}

pub unsafe fn usart0_unchecked() -> usart::UsartDevice {
    usart::device(SERCOM0)
}