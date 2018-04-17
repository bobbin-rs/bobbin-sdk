pub use kinetis_common::wdog::*;

::bobbin_mcu::periph!( WDOG, Wdog, WDOG_PERIPH, WdogPeriph, WDOG_OWNED, WDOG_REF_COUNT, 0x40052000, 0x00, 0x06);

