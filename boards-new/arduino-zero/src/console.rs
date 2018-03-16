pub use mcu::bobbin_common::console::*;
use common::periph::IntoPeriph;
use common::configure::Configure;
use common::enabled::Enabled;
use mcu::pin::*;
use mcu::sercom::*;
use mcu::gclk;

pub const SERCOM: Sercom5 = SERCOM5;
pub const SERCOM_TX: Pb22 = PB22;
pub const SERCOM_RX: Pb23 = PB23;

pub fn init() {
    SERCOM.gate_enable();
    SERCOM_RX.port().gate_enable();
    SERCOM_TX.port().gate_enable();
    // Set GCLK_GEN0 as source for SERCOM

    gclk::GCLK.set_clkctrl(|r| r
        .set_id(0x14 + 5)
        .set_gen(0x0)
        .set_clken(1)
    );
    // Wait for synchronization
    while gclk::GCLK.status().syncbusy() != 0 {}

    // Set Pin Configuration
    SERCOM_TX.mode_pad_2(&SERCOM);
    SERCOM_RX.mode_pad_3(&SERCOM);

    SERCOM
        .set_config(|c| c
            .set_mode_usart_int()
            .set_baud(63018)
            .set_txpo(1)
            .set_rxpo(3)
        )
        .set_enabled(true);

    set_console(Console::new(SERCOM.into_periph()));
}
