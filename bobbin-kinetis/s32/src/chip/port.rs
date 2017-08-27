#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "PORT", peripherals: [Peripheral { derived_from: None, group_name: None, name: "PORTA", address: 1074040832, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Pin Control and Interrupts"), modules: [], features: [], links: [Link { name: "GPIO", peripheral_group: "GPIO", peripheral: "GPIOA", channel: "" }], interrupts: [Interrupt { name: "PORTA", types: ["PORT"], value: 59, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [Pin { name: "PTA0", index: Some(0), description: None, altfns: [AltFn { index: 0, signal: "ADC0_SE0", description: None }, AltFn { index: 0, signal: "CMP0_IN0", description: None }, AltFn { index: 1, signal: "PTA0", description: None }, AltFn { index: 2, signal: "FTM2_CH1", description: None }, AltFn { index: 3, signal: "LPI2C0_SCLS", description: None }, AltFn { index: 4, signal: "FXIO_D2", description: None }, AltFn { index: 5, signal: "FTM2_QD_PHA", description: None }, AltFn { index: 6, signal: "LPUART0_CTS", description: None }, AltFn { index: 7, signal: "TRGMUX_OUT3", description: None }] }, Pin { name: "PTA1", index: Some(1), description: None, altfns: [AltFn { index: 0, signal: "ADC0_SE1", description: None }, AltFn { index: 0, signal: "CMP0_IN1", description: None }, AltFn { index: 1, signal: "PTA1", description: None }, AltFn { index: 2, signal: "FTM1_CH1", description: None }, AltFn { index: 3, signal: "LPI2C0_SDAS", description: None }, AltFn { index: 4, signal: "FXIO_D3", description: None }, AltFn { index: 5, signal: "FTM1_QD_PHA", description: None }, AltFn { index: 6, signal: "LPUART0_RTS", description: None }, AltFn { index: 7, signal: "TRGMUX_OUT0", description: None }] }, Pin { name: "PTA2", index: Some(2), description: None, altfns: [AltFn { index: 0, signal: "ADC1_SE0", description: None }, AltFn { index: 1, signal: "PTA2", description: None }, AltFn { index: 2, signal: "FTM3_CH0", description: None }, AltFn { index: 3, signal: "LPI2C0_SDA", description: None }, AltFn { index: 4, signal: "EWM_OUT_b", description: None }, AltFn { index: 5, signal: "FXIO_D4", description: None }, AltFn { index: 6, signal: "LPUART0_RX", description: None }] }, Pin { name: "PTA3", index: Some(3), description: None, altfns: [AltFn { index: 0, signal: "ADC1_SE1", description: None }, AltFn { index: 1, signal: "PTA3", description: None }, AltFn { index: 2, signal: "FTM3_CH1", description: None }, AltFn { index: 3, signal: "LPI2C0_SCL", description: None }, AltFn { index: 4, signal: "EWM_IN", description: None }, AltFn { index: 5, signal: "FXIO_D5", description: None }, AltFn { index: 6, signal: "LPUART0_TX", description: None }] }, Pin { name: "PTA4", index: Some(4), description: None, altfns: [AltFn { index: 1, signal: "PTA4", description: None }, AltFn { index: 4, signal: "CMP0_OUT", description: None }, AltFn { index: 5, signal: "EWM_OUT_b", description: None }, AltFn { index: 7, signal: "JTAG_TMS", description: None }, AltFn { index: 7, signal: "SWD_DIO", description: None }] }, Pin { name: "PTA5", index: Some(5), description: None, altfns: [AltFn { index: 1, signal: "PTA5", description: None }, AltFn { index: 3, signal: "TCLK1", description: None }, AltFn { index: 7, signal: "RESET_b", description: None }] }, Pin { name: "PTA6", index: Some(6), description: None, altfns: [AltFn { index: 0, signal: "ADC0_SE2", description: None }, AltFn { index: 1, signal: "PTA6", description: None }, AltFn { index: 2, signal: "FTM0_FLT1", description: None }, AltFn { index: 3, signal: "LPSPI1_PCS1", description: None }, AltFn { index: 6, signal: "LPUART1_CTS", description: None }] }, Pin { name: "PTA7", index: Some(7), description: None, altfns: [AltFn { index: 0, signal: "ADC0_SE3", description: None }, AltFn { index: 1, signal: "PTA7", description: None }, AltFn { index: 2, signal: "FTM0_FLT2", description: None }, AltFn { index: 4, signal: "RTC_CLKIN", description: None }, AltFn { index: 6, signal: "LPUART1_RTS", description: None }] }, Pin { name: "PTA8", index: Some(8), description: None, altfns: [AltFn { index: 1, signal: "PTA8", description: None }, AltFn { index: 2, signal: "LPUART2_RX", description: None }, AltFn { index: 3, signal: "LPSPI2_SOUT", description: None }, AltFn { index: 4, signal: "FXIO_D6", description: None }, AltFn { index: 5, signal: "FTM3_FLT3", description: None }] }, Pin { name: "PTA9", index: Some(9), description: None, altfns: [AltFn { index: 1, signal: "PTA9", description: None }, AltFn { index: 2, signal: "LPUART2_TX", description: None }, AltFn { index: 3, signal: "LPSPI2_PCS0", description: None }, AltFn { index: 4, signal: "FXIO_D7", description: None }, AltFn { index: 5, signal: "FTM3_FLT2", description: None }, AltFn { index: 6, signal: "FTM1_FLT3", description: None }] }, Pin { name: "PTA10", index: Some(10), description: None, altfns: [AltFn { index: 1, signal: "PTA10", description: None }, AltFn { index: 2, signal: "FTM1_CH4", description: None }, AltFn { index: 4, signal: "FXIO_D0", description: None }, AltFn { index: 7, signal: "JTAG_TDO", description: None }, AltFn { index: 7, signal: "noetm_TRACE_SWO", description: None }] }, Pin { name: "PTA11", index: Some(11), description: None, altfns: [AltFn { index: 1, signal: "PTA11", description: None }, AltFn { index: 2, signal: "FTM1_CH5", description: None }, AltFn { index: 4, signal: "FXIO_D1", description: None }, AltFn { index: 5, signal: "CMP0_RRT", description: None }] }, Pin { name: "PTA12", index: Some(12), description: None, altfns: [AltFn { index: 1, signal: "PTA12", description: None }, AltFn { index: 2, signal: "FTM1_CH6", description: None }, AltFn { index: 3, signal: "CAN1_RX", description: None }, AltFn { index: 6, signal: "FTM2_QD_PHB", description: None }] }, Pin { name: "PTA13", index: Some(13), description: None, altfns: [AltFn { index: 1, signal: "PTA13", description: None }, AltFn { index: 2, signal: "FTM1_CH7", description: None }, AltFn { index: 3, signal: "CAN1_TX", description: None }, AltFn { index: 6, signal: "FTM2_QD_PHA", description: None }] }, Pin { name: "PTA14", index: Some(14), description: None, altfns: [AltFn { index: 1, signal: "PTA14", description: None }, AltFn { index: 2, signal: "FTM0_FLT0", description: None }, AltFn { index: 3, signal: "FTM3_FLT1", description: None }, AltFn { index: 4, signal: "EWM_IN", description: None }, AltFn { index: 6, signal: "FTM1_FLT0", description: None }] }, Pin { name: "PTA15", index: Some(15), description: None, altfns: [AltFn { index: 0, signal: "ADC1_SE12", description: None }, AltFn { index: 1, signal: "PTA15", description: None }, AltFn { index: 2, signal: "FTM1_CH2", description: None }, AltFn { index: 3, signal: "LPSPI0_PCS3", description: None }, AltFn { index: 4, signal: "LPSPI2_PCS3", description: None }] }, Pin { name: "PTA16", index: Some(16), description: None, altfns: [AltFn { index: 0, signal: "ADC1_SE13", description: None }, AltFn { index: 1, signal: "PTA16", description: None }, AltFn { index: 2, signal: "FTM1_CH3", description: None }, AltFn { index: 3, signal: "LPSPI1_PCS2", description: None }] }, Pin { name: "PTA17", index: Some(17), description: None, altfns: [AltFn { index: 1, signal: "PTA17", description: None }, AltFn { index: 2, signal: "FTM0_CH6", description: None }, AltFn { index: 3, signal: "FTM3_FLT0", description: None }, AltFn { index: 4, signal: "EWM_OUT_b", description: None }] }], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "PORTB", address: 1074044928, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Pin Control and Interrupts"), modules: [], features: [], links: [Link { name: "GPIO", peripheral_group: "GPIO", peripheral: "GPIOB", channel: "" }], interrupts: [Interrupt { name: "PORTB", types: ["PORT"], value: 60, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [Pin { name: "PTB0", index: Some(0), description: None, altfns: [AltFn { index: 0, signal: "ADC0_SE4", description: None }, AltFn { index: 0, signal: "ADC1_SE14", description: None }, AltFn { index: 1, signal: "PTB0", description: None }, AltFn { index: 2, signal: "LPUART0_RX", description: None }, AltFn { index: 3, signal: "LPSPI0_PCS0", description: None }, AltFn { index: 4, signal: "LPTMR0_ALT3", description: None }, AltFn { index: 5, signal: "CAN0_RX", description: None }] }, Pin { name: "PTB1", index: Some(1), description: None, altfns: [AltFn { index: 0, signal: "ADC0_SE5", description: None }, AltFn { index: 0, signal: "ADC1_SE15", description: None }, AltFn { index: 1, signal: "PTB1", description: None }, AltFn { index: 2, signal: "LPUART0_TX", description: None }, AltFn { index: 3, signal: "LPSPI0_SOUT", description: None }, AltFn { index: 4, signal: "TCLK0", description: None }, AltFn { index: 5, signal: "CAN0_TX", description: None }] }, Pin { name: "PTB2", index: Some(2), description: None, altfns: [AltFn { index: 0, signal: "ADC0_SE6", description: None }, AltFn { index: 1, signal: "PTB2", description: None }, AltFn { index: 2, signal: "FTM1_CH0", description: None }, AltFn { index: 3, signal: "LPSPI0_SCK", description: None }, AltFn { index: 4, signal: "FTM1_QD_PHB", description: None }, AltFn { index: 6, signal: "TRGMUX_IN3", description: None }] }, Pin { name: "PTB3", index: Some(3), description: None, altfns: [AltFn { index: 0, signal: "ADC0_SE7", description: None }, AltFn { index: 1, signal: "PTB3", description: None }, AltFn { index: 2, signal: "FTM1_CH1", description: None }, AltFn { index: 3, signal: "LPSPI0_SIN", description: None }, AltFn { index: 4, signal: "FTM1_QD_PHA", description: None }, AltFn { index: 6, signal: "TRGMUX_IN2", description: None }] }, Pin { name: "PTB4", index: Some(4), description: None, altfns: [AltFn { index: 1, signal: "PTB4", description: None }, AltFn { index: 2, signal: "FTM0_CH4", description: None }, AltFn { index: 3, signal: "LPSPI0_SOUT", description: None }, AltFn { index: 6, signal: "TRGMUX_IN1", description: None }] }, Pin { name: "PTB5", index: Some(5), description: None, altfns: [AltFn { index: 1, signal: "PTB5", description: None }, AltFn { index: 2, signal: "FTM0_CH5", description: None }, AltFn { index: 3, signal: "LPSPI0_PCS1", description: None }, AltFn { index: 4, signal: "LPSPI0_PCS0", description: None }, AltFn { index: 5, signal: "CLKOUT", description: None }, AltFn { index: 6, signal: "TRGMUX_IN0", description: None }] }, Pin { name: "PTB6", index: Some(6), description: None, altfns: [AltFn { index: 0, signal: "XTAL", description: None }, AltFn { index: 1, signal: "PTB6", description: None }, AltFn { index: 2, signal: "LPI2C0_SDA", description: None }] }, Pin { name: "PTB7", index: Some(7), description: None, altfns: [AltFn { index: 0, signal: "EXTAL", description: None }, AltFn { index: 1, signal: "PTB7", description: None }, AltFn { index: 2, signal: "LPI2C0_SCL", description: None }] }, Pin { name: "PTB8", index: Some(8), description: None, altfns: [AltFn { index: 1, signal: "PTB8", description: None }, AltFn { index: 2, signal: "FTM3_CH0", description: None }] }, Pin { name: "PTB9", index: Some(9), description: None, altfns: [AltFn { index: 1, signal: "PTB9", description: None }, AltFn { index: 2, signal: "FTM3_CH1", description: None }, AltFn { index: 3, signal: "LPI2C0_SCLS", description: None }] }, Pin { name: "PTB10", index: Some(10), description: None, altfns: [AltFn { index: 1, signal: "PTB10", description: None }, AltFn { index: 2, signal: "FTM3_CH2", description: None }, AltFn { index: 3, signal: "LPI2C0_SDAS", description: None }] }, Pin { name: "PTB11", index: Some(11), description: None, altfns: [AltFn { index: 1, signal: "PTB11", description: None }, AltFn { index: 2, signal: "FTM3_CH3", description: None }, AltFn { index: 3, signal: "LPI2C0_HREQ", description: None }] }, Pin { name: "PTB12", index: Some(12), description: None, altfns: [AltFn { index: 0, signal: "ADC1_SE7", description: None }, AltFn { index: 1, signal: "PTB12", description: None }, AltFn { index: 2, signal: "FTM0_CH0", description: None }, AltFn { index: 3, signal: "FTM3_FLT2", description: None }, AltFn { index: 4, signal: "CAN2_RX", description: None }] }, Pin { name: "PTB13", index: Some(13), description: None, altfns: [AltFn { index: 0, signal: "ADC1_SE8", description: None }, AltFn { index: 0, signal: "ADC0_SE8", description: None }, AltFn { index: 1, signal: "PTB13", description: None }, AltFn { index: 2, signal: "FTM0_CH1", description: None }, AltFn { index: 3, signal: "FTM3_FLT1", description: None }, AltFn { index: 4, signal: "CAN2_TX", description: None }] }, Pin { name: "PTB14", index: Some(14), description: None, altfns: [AltFn { index: 0, signal: "ADC1_SE9", description: None }, AltFn { index: 0, signal: "ADC0_SE9", description: None }, AltFn { index: 1, signal: "PTB14", description: None }, AltFn { index: 2, signal: "FTM0_CH2", description: None }, AltFn { index: 3, signal: "LPSPI1_SCK", description: None }] }, Pin { name: "PTB15", index: Some(15), description: None, altfns: [AltFn { index: 0, signal: "ADC1_SE14", description: None }, AltFn { index: 1, signal: "PTB15", description: None }, AltFn { index: 2, signal: "FTM0_CH3", description: None }, AltFn { index: 3, signal: "LPSPI1_SIN", description: None }] }, Pin { name: "PTB16", index: Some(16), description: None, altfns: [AltFn { index: 0, signal: "ADC1_SE15", description: None }, AltFn { index: 1, signal: "PTB16", description: None }, AltFn { index: 2, signal: "FTM0_CH4", description: None }, AltFn { index: 3, signal: "LPSPI1_SOUT", description: None }] }, Pin { name: "PTB17", index: Some(17), description: None, altfns: [AltFn { index: 1, signal: "PTB17", description: None }, AltFn { index: 2, signal: "FTM0_CH5", description: None }, AltFn { index: 3, signal: "LPSPI1_PCS3", description: None }] }], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "PORTC", address: 1074049024, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Pin Control and Interrupts"), modules: [], features: [], links: [Link { name: "GPIO", peripheral_group: "GPIO", peripheral: "GPIOC", channel: "" }], interrupts: [Interrupt { name: "PORTC", types: ["PORT"], value: 61, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [Pin { name: "PTC0", index: Some(0), description: None, altfns: [AltFn { index: 0, signal: "ADC0_SE8", description: None }, AltFn { index: 1, signal: "PTC0", description: None }, AltFn { index: 2, signal: "FTM0_CH0", description: None }, AltFn { index: 3, signal: "LPSPI2_SIN", description: None }, AltFn { index: 6, signal: "FTM1_CH6", description: None }] }, Pin { name: "PTC1", index: Some(1), description: None, altfns: [AltFn { index: 0, signal: "ADC0_SE9", description: None }, AltFn { index: 1, signal: "PTC1", description: None }, AltFn { index: 2, signal: "FTM0_CH1", description: None }, AltFn { index: 3, signal: "LPSPI2_SOUT", description: None }, AltFn { index: 6, signal: "FTM1_CH7", description: None }] }, Pin { name: "PTC2", index: Some(2), description: None, altfns: [AltFn { index: 0, signal: "ADC0_SE10", description: None }, AltFn { index: 0, signal: "CMP0_IN5", description: None }, AltFn { index: 1, signal: "PTC2", description: None }, AltFn { index: 2, signal: "FTM0_CH2", description: None }, AltFn { index: 3, signal: "CAN0_RX", description: None }, AltFn { index: 4, signal: "LPUART0_RX", description: None }] }, Pin { name: "PTC3", index: Some(3), description: None, altfns: [AltFn { index: 0, signal: "ADC0_SE11", description: None }, AltFn { index: 0, signal: "CMP0_IN4", description: None }, AltFn { index: 1, signal: "PTC3", description: None }, AltFn { index: 2, signal: "FTM0_CH3", description: None }, AltFn { index: 3, signal: "CAN0_TX", description: None }, AltFn { index: 4, signal: "LPUART0_TX", description: None }] }, Pin { name: "PTC4", index: Some(4), description: None, altfns: [AltFn { index: 0, signal: "CMP0_IN2", description: None }, AltFn { index: 1, signal: "PTC4", description: None }, AltFn { index: 2, signal: "FTM1_CH0", description: None }, AltFn { index: 3, signal: "RTC_CLKOUT", description: None }, AltFn { index: 5, signal: "EWM_IN", description: None }, AltFn { index: 6, signal: "FTM1_QD_PHB", description: None }, AltFn { index: 7, signal: "JTAG_TCLK", description: None }, AltFn { index: 7, signal: "SWD_CLK", description: None }] }, Pin { name: "PTC5", index: Some(5), description: None, altfns: [AltFn { index: 1, signal: "PTC5", description: None }, AltFn { index: 2, signal: "FTM2_CH0", description: None }, AltFn { index: 3, signal: "RTC_CLKOUT", description: None }, AltFn { index: 6, signal: "FTM2_QD_PHB", description: None }, AltFn { index: 7, signal: "JTAG_TDI", description: None }] }, Pin { name: "PTC6", index: Some(6), description: None, altfns: [AltFn { index: 0, signal: "ADC1_SE4", description: None }, AltFn { index: 1, signal: "PTC6", description: None }, AltFn { index: 2, signal: "LPUART1_RX", description: None }, AltFn { index: 3, signal: "CAN1_RX", description: None }, AltFn { index: 4, signal: "FTM3_CH2", description: None }, AltFn { index: 6, signal: "FTM1_QD_PHB", description: None }] }, Pin { name: "PTC7", index: Some(7), description: None, altfns: [AltFn { index: 0, signal: "ADC1_SE5", description: None }, AltFn { index: 1, signal: "PTC7", description: None }, AltFn { index: 2, signal: "LPUART1_TX", description: None }, AltFn { index: 3, signal: "CAN1_TX", description: None }, AltFn { index: 4, signal: "FTM3_CH3", description: None }, AltFn { index: 6, signal: "FTM1_QD_PHA", description: None }] }, Pin { name: "PTC8", index: Some(8), description: None, altfns: [AltFn { index: 1, signal: "PTC8", description: None }, AltFn { index: 2, signal: "LPUART1_RX", description: None }, AltFn { index: 3, signal: "FTM1_FLT0", description: None }, AltFn { index: 6, signal: "LPUART0_CTS", description: None }] }, Pin { name: "PTC9", index: Some(9), description: None, altfns: [AltFn { index: 1, signal: "PTC9", description: None }, AltFn { index: 2, signal: "LPUART1_TX", description: None }, AltFn { index: 3, signal: "FTM1_FLT1", description: None }, AltFn { index: 6, signal: "LPUART0_RTS", description: None }] }, Pin { name: "PTC10", index: Some(10), description: None, altfns: [AltFn { index: 1, signal: "PTC10", description: None }, AltFn { index: 2, signal: "FTM3_CH4", description: None }, AltFn { index: 6, signal: "TRGMUX_IN11", description: None }] }, Pin { name: "PTC11", index: Some(11), description: None, altfns: [AltFn { index: 1, signal: "PTC11", description: None }, AltFn { index: 2, signal: "FTM3_CH5", description: None }, AltFn { index: 6, signal: "TRGMUX_IN10", description: None }] }, Pin { name: "PTC12", index: Some(12), description: None, altfns: [AltFn { index: 1, signal: "PTC12", description: None }, AltFn { index: 2, signal: "FTM3_CH6", description: None }, AltFn { index: 3, signal: "FTM2_CH6", description: None }, AltFn { index: 4, signal: "LPUART2_CTS", description: None }] }, Pin { name: "PTC13", index: Some(13), description: None, altfns: [AltFn { index: 1, signal: "PTC13", description: None }, AltFn { index: 2, signal: "FTM3_CH7", description: None }, AltFn { index: 3, signal: "FTM2_CH7", description: None }, AltFn { index: 4, signal: "LPUART2_RTS", description: None }] }, Pin { name: "PTC14", index: Some(14), description: None, altfns: [AltFn { index: 0, signal: "ADC0_SE12", description: None }, AltFn { index: 1, signal: "PTC14", description: None }, AltFn { index: 2, signal: "FTM1_CH2", description: None }, AltFn { index: 3, signal: "LPSPI2_PCS0", description: None }, AltFn { index: 6, signal: "TRGMUX_IN9", description: None }] }, Pin { name: "PTC15", index: Some(15), description: None, altfns: [AltFn { index: 0, signal: "ADC0_SE13", description: None }, AltFn { index: 1, signal: "PTC15", description: None }, AltFn { index: 2, signal: "FTM1_CH3", description: None }, AltFn { index: 3, signal: "LPSPI2_SCK", description: None }, AltFn { index: 6, signal: "TRGMUX_IN8", description: None }] }, Pin { name: "PTC16", index: Some(16), description: None, altfns: [AltFn { index: 0, signal: "ADC0_SE14", description: None }, AltFn { index: 1, signal: "PTC16", description: None }, AltFn { index: 2, signal: "FTM1_FLT2", description: None }, AltFn { index: 3, signal: "CAN2_RX", description: None }] }, Pin { name: "PTC17", index: Some(17), description: None, altfns: [AltFn { index: 0, signal: "ADC0_SE15", description: None }, AltFn { index: 1, signal: "PTC17", description: None }, AltFn { index: 2, signal: "FTM1_FLT3", description: None }, AltFn { index: 3, signal: "CAN2_TX", description: None }] }], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "PORTD", address: 1074053120, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Pin Control and Interrupts"), modules: [], features: [], links: [Link { name: "GPIO", peripheral_group: "GPIO", peripheral: "GPIOD", channel: "" }], interrupts: [Interrupt { name: "PORTD", types: ["PORT"], value: 62, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [Pin { name: "PTD0", index: Some(0), description: None, altfns: [AltFn { index: 1, signal: "PTD0", description: None }, AltFn { index: 2, signal: "FTM0_CH2", description: None }, AltFn { index: 3, signal: "LPSPI1_SCK", description: None }, AltFn { index: 4, signal: "FTM2_CH0", description: None }, AltFn { index: 6, signal: "FXIO_D0", description: None }, AltFn { index: 7, signal: "TRGMUX_OUT1", description: None }] }, Pin { name: "PTD1", index: Some(1), description: None, altfns: [AltFn { index: 1, signal: "PTD1", description: None }, AltFn { index: 2, signal: "FTM0_CH3", description: None }, AltFn { index: 3, signal: "LPSPI1_SIN", description: None }, AltFn { index: 4, signal: "FTM2_CH1", description: None }, AltFn { index: 6, signal: "FXIO_D1", description: None }, AltFn { index: 7, signal: "TRGMUX_OUT2", description: None }] }, Pin { name: "PTD2", index: Some(2), description: None, altfns: [AltFn { index: 0, signal: "ADC1_SE2", description: None }, AltFn { index: 1, signal: "PTD2", description: None }, AltFn { index: 2, signal: "FTM3_CH4", description: None }, AltFn { index: 3, signal: "LPSPI1_SOUT", description: None }, AltFn { index: 4, signal: "FXIO_D4", description: None }, AltFn { index: 5, signal: "FXIO_D6", description: None }, AltFn { index: 6, signal: "TRGMUX_IN5", description: None }] }, Pin { name: "PTD3", index: Some(3), description: None, altfns: [AltFn { index: 0, signal: "ADC1_SE3", description: None }, AltFn { index: 1, signal: "PTD3", description: None }, AltFn { index: 2, signal: "FTM3_CH5", description: None }, AltFn { index: 3, signal: "LPSPI1_PCS0", description: None }, AltFn { index: 4, signal: "FXIO_D5", description: None }, AltFn { index: 5, signal: "FXIO_D7", description: None }, AltFn { index: 6, signal: "TRGMUX_IN4", description: None }, AltFn { index: 7, signal: "NMI_b", description: None }] }, Pin { name: "PTD4", index: Some(4), description: None, altfns: [AltFn { index: 0, signal: "ADC1_SE6", description: None }, AltFn { index: 1, signal: "PTD4", description: None }, AltFn { index: 2, signal: "FTM0_FLT3", description: None }, AltFn { index: 3, signal: "FTM3_FLT3", description: None }] }, Pin { name: "PTD5", index: Some(5), description: None, altfns: [AltFn { index: 1, signal: "PTD5", description: None }, AltFn { index: 2, signal: "FTM2_CH3", description: None }, AltFn { index: 3, signal: "LPTMR0_ALT2", description: None }, AltFn { index: 4, signal: "FTM2_FLT1", description: None }, AltFn { index: 6, signal: "TRGMUX_IN7", description: None }] }, Pin { name: "PTD6", index: Some(6), description: None, altfns: [AltFn { index: 0, signal: "CMP0_IN7", description: None }, AltFn { index: 1, signal: "PTD6", description: None }, AltFn { index: 2, signal: "LPUART2_RX", description: None }, AltFn { index: 4, signal: "FTM2_FLT2", description: None }] }, Pin { name: "PTD7", index: Some(7), description: None, altfns: [AltFn { index: 0, signal: "CMP0_IN6", description: None }, AltFn { index: 1, signal: "PTD7", description: None }, AltFn { index: 2, signal: "LPUART2_TX", description: None }, AltFn { index: 4, signal: "FTM2_FLT3", description: None }] }, Pin { name: "PTD8", index: Some(8), description: None, altfns: [AltFn { index: 1, signal: "PTD8", description: None }, AltFn { index: 4, signal: "FTM2_FLT2", description: None }, AltFn { index: 5, signal: "FXIO_D1", description: None }, AltFn { index: 6, signal: "FTM1_CH4", description: None }] }, Pin { name: "PTD9", index: Some(9), description: None, altfns: [AltFn { index: 1, signal: "PTD9", description: None }, AltFn { index: 3, signal: "FXIO_D0", description: None }, AltFn { index: 4, signal: "FTM2_FLT3", description: None }, AltFn { index: 6, signal: "FTM1_CH5", description: None }] }, Pin { name: "PTD10", index: Some(10), description: None, altfns: [AltFn { index: 1, signal: "PTD10", description: None }, AltFn { index: 2, signal: "FTM2_CH0", description: None }, AltFn { index: 3, signal: "FTM2_QD_PHB", description: None }] }, Pin { name: "PTD11", index: Some(11), description: None, altfns: [AltFn { index: 1, signal: "PTD11", description: None }, AltFn { index: 2, signal: "FTM2_CH1", description: None }, AltFn { index: 3, signal: "FTM2_QD_PHA", description: None }, AltFn { index: 6, signal: "LPUART2_CTS", description: None }] }, Pin { name: "PTD12", index: Some(12), description: None, altfns: [AltFn { index: 1, signal: "PTD12", description: None }, AltFn { index: 2, signal: "FTM2_CH2", description: None }, AltFn { index: 6, signal: "LPUART2_RTS", description: None }] }, Pin { name: "PTD13", index: Some(13), description: None, altfns: [AltFn { index: 1, signal: "PTD13", description: None }, AltFn { index: 2, signal: "FTM2_CH4", description: None }, AltFn { index: 3, signal: "LPUART1_RX", description: None }, AltFn { index: 7, signal: "RTC_CLKOUT", description: None }] }, Pin { name: "PTD14", index: Some(14), description: None, altfns: [AltFn { index: 1, signal: "PTD14", description: None }, AltFn { index: 2, signal: "FTM2_CH5", description: None }, AltFn { index: 3, signal: "LPUART1_TX", description: None }, AltFn { index: 7, signal: "CLKOUT", description: None }] }, Pin { name: "PTD15", index: Some(15), description: None, altfns: [AltFn { index: 1, signal: "PTD15", description: None }, AltFn { index: 2, signal: "FTM0_CH0", description: None }, AltFn { index: 4, signal: "LPSPI0_SCK", description: None }] }, Pin { name: "PTD16", index: Some(16), description: None, altfns: [AltFn { index: 1, signal: "PTD16", description: None }, AltFn { index: 2, signal: "FTM0_CH1", description: None }, AltFn { index: 4, signal: "LPSPI0_SIN", description: None }, AltFn { index: 5, signal: "CMP0_RRT", description: None }] }, Pin { name: "PTD17", index: Some(17), description: None, altfns: [AltFn { index: 1, signal: "PTD17", description: None }, AltFn { index: 2, signal: "FTM0_FLT2", description: None }, AltFn { index: 3, signal: "LPUART2_RX", description: None }] }], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "PORTE", address: 1074057216, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Pin Control and Interrupts"), modules: [], features: [], links: [Link { name: "GPIO", peripheral_group: "GPIO", peripheral: "GPIOE", channel: "" }], interrupts: [Interrupt { name: "PORTE", types: [], value: 63, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [Pin { name: "PTE0", index: Some(0), description: None, altfns: [AltFn { index: 1, signal: "PTE0", description: None }, AltFn { index: 2, signal: "LPSPI0_SCK", description: None }, AltFn { index: 3, signal: "TCLK1", description: None }, AltFn { index: 5, signal: "LPSPI1_SOUT", description: None }, AltFn { index: 6, signal: "FTM1_FLT2", description: None }] }, Pin { name: "PTE1", index: Some(1), description: None, altfns: [AltFn { index: 1, signal: "PTE1", description: None }, AltFn { index: 2, signal: "LPSPI0_SIN", description: None }, AltFn { index: 3, signal: "LPI2C0_HREQ", description: None }, AltFn { index: 5, signal: "LPSPI1_PCS0", description: None }, AltFn { index: 6, signal: "FTM1_FLT1", description: None }] }, Pin { name: "PTE2", index: Some(2), description: None, altfns: [AltFn { index: 0, signal: "ADC1_SE10", description: None }, AltFn { index: 1, signal: "PTE2", description: None }, AltFn { index: 2, signal: "LPSPI0_SOUT", description: None }, AltFn { index: 3, signal: "LPTMR0_ALT3", description: None }, AltFn { index: 4, signal: "FTM3_CH6", description: None }, AltFn { index: 6, signal: "LPUART1_CTS", description: None }] }, Pin { name: "PTE3", index: Some(3), description: None, altfns: [AltFn { index: 1, signal: "PTE3", description: None }, AltFn { index: 2, signal: "FTM0_FLT0", description: None }, AltFn { index: 3, signal: "LPUART2_RTS", description: None }, AltFn { index: 4, signal: "FTM2_FLT0", description: None }, AltFn { index: 6, signal: "TRGMUX_IN6", description: None }, AltFn { index: 7, signal: "CMP0_OUT", description: None }] }, Pin { name: "PTE4", index: Some(4), description: None, altfns: [AltFn { index: 1, signal: "PTE4", description: None }, AltFn { index: 3, signal: "FTM2_QD_PHB", description: None }, AltFn { index: 4, signal: "FTM2_CH2", description: None }, AltFn { index: 5, signal: "CAN0_RX", description: None }, AltFn { index: 6, signal: "FXIO_D6", description: None }, AltFn { index: 7, signal: "EWM_OUT_b", description: None }] }, Pin { name: "PTE5", index: Some(5), description: None, altfns: [AltFn { index: 1, signal: "PTE5", description: None }, AltFn { index: 2, signal: "TCLK2", description: None }, AltFn { index: 3, signal: "FTM2_QD_PHA", description: None }, AltFn { index: 4, signal: "FTM2_CH3", description: None }, AltFn { index: 5, signal: "CAN0_TX", description: None }, AltFn { index: 6, signal: "FXIO_D7", description: None }, AltFn { index: 7, signal: "EWM_IN", description: None }] }, Pin { name: "PTE6", index: Some(6), description: None, altfns: [AltFn { index: 0, signal: "ADC1_SE11", description: None }, AltFn { index: 1, signal: "PTE6", description: None }, AltFn { index: 2, signal: "LPSPI0_PCS2", description: None }, AltFn { index: 4, signal: "FTM3_CH7", description: None }, AltFn { index: 6, signal: "LPUART1_RTS", description: None }] }, Pin { name: "PTE7", index: Some(7), description: None, altfns: [AltFn { index: 1, signal: "PTE7", description: None }, AltFn { index: 2, signal: "FTM0_CH7", description: None }, AltFn { index: 3, signal: "FTM3_FLT0", description: None }] }, Pin { name: "PTE8", index: Some(8), description: None, altfns: [AltFn { index: 0, signal: "CMP0_IN3", description: None }, AltFn { index: 1, signal: "PTE8", description: None }, AltFn { index: 2, signal: "FTM0_CH6", description: None }] }, Pin { name: "PTE9", index: Some(9), description: None, altfns: [AltFn { index: 1, signal: "PTE9", description: None }, AltFn { index: 2, signal: "FTM0_CH7", description: None }, AltFn { index: 3, signal: "LPUART2_CTS", description: None }] }, Pin { name: "PTE10", index: Some(10), description: None, altfns: [AltFn { index: 1, signal: "PTE10", description: None }, AltFn { index: 2, signal: "CLKOUT", description: None }, AltFn { index: 3, signal: "LPSPI2_PCS1", description: None }, AltFn { index: 4, signal: "FTM2_CH4", description: None }, AltFn { index: 6, signal: "FXIO_D4", description: None }, AltFn { index: 7, signal: "TRGMUX_OUT4", description: None }] }, Pin { name: "PTE11", index: Some(11), description: None, altfns: [AltFn { index: 1, signal: "PTE11", description: None }, AltFn { index: 2, signal: "LPSPI2_PCS0", description: None }, AltFn { index: 3, signal: "LPTMR0_ALT1", description: None }, AltFn { index: 4, signal: "FTM2_CH5", description: None }, AltFn { index: 6, signal: "FXIO_D5", description: None }, AltFn { index: 7, signal: "TRGMUX_OUT5", description: None }] }, Pin { name: "PTE12", index: Some(12), description: None, altfns: [AltFn { index: 1, signal: "PTE12", description: None }, AltFn { index: 2, signal: "FTM0_FLT3", description: None }, AltFn { index: 3, signal: "LPUART2_TX", description: None }] }, Pin { name: "PTE13", index: Some(13), description: None, altfns: [AltFn { index: 1, signal: "PTE13", description: None }, AltFn { index: 3, signal: "LPSPI2_PCS2", description: None }, AltFn { index: 4, signal: "FTM2_FLT0", description: None }] }, Pin { name: "PTE14", index: Some(14), description: None, altfns: [AltFn { index: 1, signal: "PTE14", description: None }, AltFn { index: 2, signal: "FTM0_FLT1", description: None }, AltFn { index: 4, signal: "FTM2_FLT1", description: None }] }, Pin { name: "PTE15", index: Some(15), description: None, altfns: [AltFn { index: 1, signal: "PTE15", description: None }, AltFn { index: 2, signal: "LPUART1_CTS", description: None }, AltFn { index: 3, signal: "LPSPI2_SCK", description: None }, AltFn { index: 4, signal: "FTM2_CH6", description: None }, AltFn { index: 6, signal: "FXIO_D2", description: None }, AltFn { index: 7, signal: "TRGMUX_OUT6", description: None }] }, Pin { name: "PTE16", index: Some(16), description: None, altfns: [AltFn { index: 1, signal: "PTE16", description: None }, AltFn { index: 2, signal: "LPUART1_RTS", description: None }, AltFn { index: 3, signal: "LPSPI2_SIN", description: None }, AltFn { index: 4, signal: "FTM2_CH7", description: None }, AltFn { index: 6, signal: "FXIO_D3", description: None }, AltFn { index: 7, signal: "TRGMUX_OUT7", description: None }] }], channels: [], dim: None, dim_increment: None, dim_index: None }], prototype: None, modules: [Module { name: "kinetis_common::chip::port::*", _as: None }], has_pins: false, has_channels: false, description: None }
pub use kinetis_common::chip::port::*;

pub trait LinkGpio<T> {
   fn gpio(&self) -> T;
}

periph!( PORTA, Porta, _PORTA, PortPeriph, 0x40049000);
periph!( PORTB, Portb, _PORTB, PortPeriph, 0x4004a000);
periph!( PORTC, Portc, _PORTC, PortPeriph, 0x4004b000);
periph!( PORTD, Portd, _PORTD, PortPeriph, 0x4004c000);
periph!( PORTE, Porte, _PORTE, PortPeriph, 0x4004d000);

impl LinkGpio<super::gpio::Periph<super::gpio::GpioaId>> for Porta {
   fn gpio(&self) -> super::gpio::Periph<super::gpio::GpioaId> { super::gpio::GPIOA }
}


impl LinkGpio<super::gpio::Periph<super::gpio::GpiobId>> for Portb {
   fn gpio(&self) -> super::gpio::Periph<super::gpio::GpiobId> { super::gpio::GPIOB }
}


impl LinkGpio<super::gpio::Periph<super::gpio::GpiocId>> for Portc {
   fn gpio(&self) -> super::gpio::Periph<super::gpio::GpiocId> { super::gpio::GPIOC }
}


impl LinkGpio<super::gpio::Periph<super::gpio::GpiodId>> for Portd {
   fn gpio(&self) -> super::gpio::Periph<super::gpio::GpiodId> { super::gpio::GPIOD }
}


impl LinkGpio<super::gpio::Periph<super::gpio::GpioeId>> for Porte {
   fn gpio(&self) -> super::gpio::Periph<super::gpio::GpioeId> { super::gpio::GPIOE }
}



pin!(PTA0, Pta0, PORTA, Porta, _PTA0, PortPin, _PORTA, 0);
   alt_fn!(Pta0, super::sig::Adc0Se0, 0);
   alt_fn!(Pta0, super::sig::Cmp0In0, 0);
   alt_fn!(Pta0, super::sig::Pta0, 1);
   alt_fn!(Pta0, super::sig::Ftm2Ch1, 2);
   alt_fn!(Pta0, super::sig::Lpi2c0Scls, 3);
   alt_fn!(Pta0, super::sig::FxioD2, 4);
   alt_fn!(Pta0, super::sig::Ftm2QdPha, 5);
   alt_fn!(Pta0, super::sig::Lpuart0Cts, 6);
   alt_fn!(Pta0, super::sig::TrgmuxOut3, 7);

pin!(PTA1, Pta1, PORTA, Porta, _PTA1, PortPin, _PORTA, 1);
   alt_fn!(Pta1, super::sig::Adc0Se1, 0);
   alt_fn!(Pta1, super::sig::Cmp0In1, 0);
   alt_fn!(Pta1, super::sig::Pta1, 1);
   alt_fn!(Pta1, super::sig::Ftm1Ch1, 2);
   alt_fn!(Pta1, super::sig::Lpi2c0Sdas, 3);
   alt_fn!(Pta1, super::sig::FxioD3, 4);
   alt_fn!(Pta1, super::sig::Ftm1QdPha, 5);
   alt_fn!(Pta1, super::sig::Lpuart0Rts, 6);
   alt_fn!(Pta1, super::sig::TrgmuxOut0, 7);

pin!(PTA2, Pta2, PORTA, Porta, _PTA2, PortPin, _PORTA, 2);
   alt_fn!(Pta2, super::sig::Adc1Se0, 0);
   alt_fn!(Pta2, super::sig::Pta2, 1);
   alt_fn!(Pta2, super::sig::Ftm3Ch0, 2);
   alt_fn!(Pta2, super::sig::Lpi2c0Sda, 3);
   alt_fn!(Pta2, super::sig::EwmOutB, 4);
   alt_fn!(Pta2, super::sig::FxioD4, 5);
   alt_fn!(Pta2, super::sig::Lpuart0Rx, 6);

pin!(PTA3, Pta3, PORTA, Porta, _PTA3, PortPin, _PORTA, 3);
   alt_fn!(Pta3, super::sig::Adc1Se1, 0);
   alt_fn!(Pta3, super::sig::Pta3, 1);
   alt_fn!(Pta3, super::sig::Ftm3Ch1, 2);
   alt_fn!(Pta3, super::sig::Lpi2c0Scl, 3);
   alt_fn!(Pta3, super::sig::EwmIn, 4);
   alt_fn!(Pta3, super::sig::FxioD5, 5);
   alt_fn!(Pta3, super::sig::Lpuart0Tx, 6);

pin!(PTA4, Pta4, PORTA, Porta, _PTA4, PortPin, _PORTA, 4);
   alt_fn!(Pta4, super::sig::Pta4, 1);
   alt_fn!(Pta4, super::sig::Cmp0Out, 4);
   alt_fn!(Pta4, super::sig::EwmOutB, 5);
   alt_fn!(Pta4, super::sig::JtagTms, 7);
   alt_fn!(Pta4, super::sig::SwdDio, 7);

pin!(PTA5, Pta5, PORTA, Porta, _PTA5, PortPin, _PORTA, 5);
   alt_fn!(Pta5, super::sig::Pta5, 1);
   alt_fn!(Pta5, super::sig::Tclk1, 3);
   alt_fn!(Pta5, super::sig::ResetB, 7);

pin!(PTA6, Pta6, PORTA, Porta, _PTA6, PortPin, _PORTA, 6);
   alt_fn!(Pta6, super::sig::Adc0Se2, 0);
   alt_fn!(Pta6, super::sig::Pta6, 1);
   alt_fn!(Pta6, super::sig::Ftm0Flt1, 2);
   alt_fn!(Pta6, super::sig::Lpspi1Pcs1, 3);
   alt_fn!(Pta6, super::sig::Lpuart1Cts, 6);

pin!(PTA7, Pta7, PORTA, Porta, _PTA7, PortPin, _PORTA, 7);
   alt_fn!(Pta7, super::sig::Adc0Se3, 0);
   alt_fn!(Pta7, super::sig::Pta7, 1);
   alt_fn!(Pta7, super::sig::Ftm0Flt2, 2);
   alt_fn!(Pta7, super::sig::RtcClkin, 4);
   alt_fn!(Pta7, super::sig::Lpuart1Rts, 6);

pin!(PTA8, Pta8, PORTA, Porta, _PTA8, PortPin, _PORTA, 8);
   alt_fn!(Pta8, super::sig::Pta8, 1);
   alt_fn!(Pta8, super::sig::Lpuart2Rx, 2);
   alt_fn!(Pta8, super::sig::Lpspi2Sout, 3);
   alt_fn!(Pta8, super::sig::FxioD6, 4);
   alt_fn!(Pta8, super::sig::Ftm3Flt3, 5);

pin!(PTA9, Pta9, PORTA, Porta, _PTA9, PortPin, _PORTA, 9);
   alt_fn!(Pta9, super::sig::Pta9, 1);
   alt_fn!(Pta9, super::sig::Lpuart2Tx, 2);
   alt_fn!(Pta9, super::sig::Lpspi2Pcs0, 3);
   alt_fn!(Pta9, super::sig::FxioD7, 4);
   alt_fn!(Pta9, super::sig::Ftm3Flt2, 5);
   alt_fn!(Pta9, super::sig::Ftm1Flt3, 6);

pin!(PTA10, Pta10, PORTA, Porta, _PTA10, PortPin, _PORTA, 10);
   alt_fn!(Pta10, super::sig::Pta10, 1);
   alt_fn!(Pta10, super::sig::Ftm1Ch4, 2);
   alt_fn!(Pta10, super::sig::FxioD0, 4);
   alt_fn!(Pta10, super::sig::JtagTdo, 7);
   alt_fn!(Pta10, super::sig::NoetmTraceSwo, 7);

pin!(PTA11, Pta11, PORTA, Porta, _PTA11, PortPin, _PORTA, 11);
   alt_fn!(Pta11, super::sig::Pta11, 1);
   alt_fn!(Pta11, super::sig::Ftm1Ch5, 2);
   alt_fn!(Pta11, super::sig::FxioD1, 4);
   alt_fn!(Pta11, super::sig::Cmp0Rrt, 5);

pin!(PTA12, Pta12, PORTA, Porta, _PTA12, PortPin, _PORTA, 12);
   alt_fn!(Pta12, super::sig::Pta12, 1);
   alt_fn!(Pta12, super::sig::Ftm1Ch6, 2);
   alt_fn!(Pta12, super::sig::Can1Rx, 3);
   alt_fn!(Pta12, super::sig::Ftm2QdPhb, 6);

pin!(PTA13, Pta13, PORTA, Porta, _PTA13, PortPin, _PORTA, 13);
   alt_fn!(Pta13, super::sig::Pta13, 1);
   alt_fn!(Pta13, super::sig::Ftm1Ch7, 2);
   alt_fn!(Pta13, super::sig::Can1Tx, 3);
   alt_fn!(Pta13, super::sig::Ftm2QdPha, 6);

pin!(PTA14, Pta14, PORTA, Porta, _PTA14, PortPin, _PORTA, 14);
   alt_fn!(Pta14, super::sig::Pta14, 1);
   alt_fn!(Pta14, super::sig::Ftm0Flt0, 2);
   alt_fn!(Pta14, super::sig::Ftm3Flt1, 3);
   alt_fn!(Pta14, super::sig::EwmIn, 4);
   alt_fn!(Pta14, super::sig::Ftm1Flt0, 6);

pin!(PTA15, Pta15, PORTA, Porta, _PTA15, PortPin, _PORTA, 15);
   alt_fn!(Pta15, super::sig::Adc1Se12, 0);
   alt_fn!(Pta15, super::sig::Pta15, 1);
   alt_fn!(Pta15, super::sig::Ftm1Ch2, 2);
   alt_fn!(Pta15, super::sig::Lpspi0Pcs3, 3);
   alt_fn!(Pta15, super::sig::Lpspi2Pcs3, 4);

pin!(PTA16, Pta16, PORTA, Porta, _PTA16, PortPin, _PORTA, 16);
   alt_fn!(Pta16, super::sig::Adc1Se13, 0);
   alt_fn!(Pta16, super::sig::Pta16, 1);
   alt_fn!(Pta16, super::sig::Ftm1Ch3, 2);
   alt_fn!(Pta16, super::sig::Lpspi1Pcs2, 3);

pin!(PTA17, Pta17, PORTA, Porta, _PTA17, PortPin, _PORTA, 17);
   alt_fn!(Pta17, super::sig::Pta17, 1);
   alt_fn!(Pta17, super::sig::Ftm0Ch6, 2);
   alt_fn!(Pta17, super::sig::Ftm3Flt0, 3);
   alt_fn!(Pta17, super::sig::EwmOutB, 4);

pin!(PTB0, Ptb0, PORTB, Portb, _PTB0, PortPin, _PORTB, 0);
   alt_fn!(Ptb0, super::sig::Adc0Se4, 0);
   alt_fn!(Ptb0, super::sig::Adc1Se14, 0);
   alt_fn!(Ptb0, super::sig::Ptb0, 1);
   alt_fn!(Ptb0, super::sig::Lpuart0Rx, 2);
   alt_fn!(Ptb0, super::sig::Lpspi0Pcs0, 3);
   alt_fn!(Ptb0, super::sig::Lptmr0Alt3, 4);
   alt_fn!(Ptb0, super::sig::Can0Rx, 5);

pin!(PTB1, Ptb1, PORTB, Portb, _PTB1, PortPin, _PORTB, 1);
   alt_fn!(Ptb1, super::sig::Adc0Se5, 0);
   alt_fn!(Ptb1, super::sig::Adc1Se15, 0);
   alt_fn!(Ptb1, super::sig::Ptb1, 1);
   alt_fn!(Ptb1, super::sig::Lpuart0Tx, 2);
   alt_fn!(Ptb1, super::sig::Lpspi0Sout, 3);
   alt_fn!(Ptb1, super::sig::Tclk0, 4);
   alt_fn!(Ptb1, super::sig::Can0Tx, 5);

pin!(PTB2, Ptb2, PORTB, Portb, _PTB2, PortPin, _PORTB, 2);
   alt_fn!(Ptb2, super::sig::Adc0Se6, 0);
   alt_fn!(Ptb2, super::sig::Ptb2, 1);
   alt_fn!(Ptb2, super::sig::Ftm1Ch0, 2);
   alt_fn!(Ptb2, super::sig::Lpspi0Sck, 3);
   alt_fn!(Ptb2, super::sig::Ftm1QdPhb, 4);
   alt_fn!(Ptb2, super::sig::TrgmuxIn3, 6);

pin!(PTB3, Ptb3, PORTB, Portb, _PTB3, PortPin, _PORTB, 3);
   alt_fn!(Ptb3, super::sig::Adc0Se7, 0);
   alt_fn!(Ptb3, super::sig::Ptb3, 1);
   alt_fn!(Ptb3, super::sig::Ftm1Ch1, 2);
   alt_fn!(Ptb3, super::sig::Lpspi0Sin, 3);
   alt_fn!(Ptb3, super::sig::Ftm1QdPha, 4);
   alt_fn!(Ptb3, super::sig::TrgmuxIn2, 6);

pin!(PTB4, Ptb4, PORTB, Portb, _PTB4, PortPin, _PORTB, 4);
   alt_fn!(Ptb4, super::sig::Ptb4, 1);
   alt_fn!(Ptb4, super::sig::Ftm0Ch4, 2);
   alt_fn!(Ptb4, super::sig::Lpspi0Sout, 3);
   alt_fn!(Ptb4, super::sig::TrgmuxIn1, 6);

pin!(PTB5, Ptb5, PORTB, Portb, _PTB5, PortPin, _PORTB, 5);
   alt_fn!(Ptb5, super::sig::Ptb5, 1);
   alt_fn!(Ptb5, super::sig::Ftm0Ch5, 2);
   alt_fn!(Ptb5, super::sig::Lpspi0Pcs1, 3);
   alt_fn!(Ptb5, super::sig::Lpspi0Pcs0, 4);
   alt_fn!(Ptb5, super::sig::Clkout, 5);
   alt_fn!(Ptb5, super::sig::TrgmuxIn0, 6);

pin!(PTB6, Ptb6, PORTB, Portb, _PTB6, PortPin, _PORTB, 6);
   alt_fn!(Ptb6, super::sig::Xtal, 0);
   alt_fn!(Ptb6, super::sig::Ptb6, 1);
   alt_fn!(Ptb6, super::sig::Lpi2c0Sda, 2);

pin!(PTB7, Ptb7, PORTB, Portb, _PTB7, PortPin, _PORTB, 7);
   alt_fn!(Ptb7, super::sig::Extal, 0);
   alt_fn!(Ptb7, super::sig::Ptb7, 1);
   alt_fn!(Ptb7, super::sig::Lpi2c0Scl, 2);

pin!(PTB8, Ptb8, PORTB, Portb, _PTB8, PortPin, _PORTB, 8);
   alt_fn!(Ptb8, super::sig::Ptb8, 1);
   alt_fn!(Ptb8, super::sig::Ftm3Ch0, 2);

pin!(PTB9, Ptb9, PORTB, Portb, _PTB9, PortPin, _PORTB, 9);
   alt_fn!(Ptb9, super::sig::Ptb9, 1);
   alt_fn!(Ptb9, super::sig::Ftm3Ch1, 2);
   alt_fn!(Ptb9, super::sig::Lpi2c0Scls, 3);

pin!(PTB10, Ptb10, PORTB, Portb, _PTB10, PortPin, _PORTB, 10);
   alt_fn!(Ptb10, super::sig::Ptb10, 1);
   alt_fn!(Ptb10, super::sig::Ftm3Ch2, 2);
   alt_fn!(Ptb10, super::sig::Lpi2c0Sdas, 3);

pin!(PTB11, Ptb11, PORTB, Portb, _PTB11, PortPin, _PORTB, 11);
   alt_fn!(Ptb11, super::sig::Ptb11, 1);
   alt_fn!(Ptb11, super::sig::Ftm3Ch3, 2);
   alt_fn!(Ptb11, super::sig::Lpi2c0Hreq, 3);

pin!(PTB12, Ptb12, PORTB, Portb, _PTB12, PortPin, _PORTB, 12);
   alt_fn!(Ptb12, super::sig::Adc1Se7, 0);
   alt_fn!(Ptb12, super::sig::Ptb12, 1);
   alt_fn!(Ptb12, super::sig::Ftm0Ch0, 2);
   alt_fn!(Ptb12, super::sig::Ftm3Flt2, 3);
   alt_fn!(Ptb12, super::sig::Can2Rx, 4);

pin!(PTB13, Ptb13, PORTB, Portb, _PTB13, PortPin, _PORTB, 13);
   alt_fn!(Ptb13, super::sig::Adc1Se8, 0);
   alt_fn!(Ptb13, super::sig::Adc0Se8, 0);
   alt_fn!(Ptb13, super::sig::Ptb13, 1);
   alt_fn!(Ptb13, super::sig::Ftm0Ch1, 2);
   alt_fn!(Ptb13, super::sig::Ftm3Flt1, 3);
   alt_fn!(Ptb13, super::sig::Can2Tx, 4);

pin!(PTB14, Ptb14, PORTB, Portb, _PTB14, PortPin, _PORTB, 14);
   alt_fn!(Ptb14, super::sig::Adc1Se9, 0);
   alt_fn!(Ptb14, super::sig::Adc0Se9, 0);
   alt_fn!(Ptb14, super::sig::Ptb14, 1);
   alt_fn!(Ptb14, super::sig::Ftm0Ch2, 2);
   alt_fn!(Ptb14, super::sig::Lpspi1Sck, 3);

pin!(PTB15, Ptb15, PORTB, Portb, _PTB15, PortPin, _PORTB, 15);
   alt_fn!(Ptb15, super::sig::Adc1Se14, 0);
   alt_fn!(Ptb15, super::sig::Ptb15, 1);
   alt_fn!(Ptb15, super::sig::Ftm0Ch3, 2);
   alt_fn!(Ptb15, super::sig::Lpspi1Sin, 3);

pin!(PTB16, Ptb16, PORTB, Portb, _PTB16, PortPin, _PORTB, 16);
   alt_fn!(Ptb16, super::sig::Adc1Se15, 0);
   alt_fn!(Ptb16, super::sig::Ptb16, 1);
   alt_fn!(Ptb16, super::sig::Ftm0Ch4, 2);
   alt_fn!(Ptb16, super::sig::Lpspi1Sout, 3);

pin!(PTB17, Ptb17, PORTB, Portb, _PTB17, PortPin, _PORTB, 17);
   alt_fn!(Ptb17, super::sig::Ptb17, 1);
   alt_fn!(Ptb17, super::sig::Ftm0Ch5, 2);
   alt_fn!(Ptb17, super::sig::Lpspi1Pcs3, 3);

pin!(PTC0, Ptc0, PORTC, Portc, _PTC0, PortPin, _PORTC, 0);
   alt_fn!(Ptc0, super::sig::Adc0Se8, 0);
   alt_fn!(Ptc0, super::sig::Ptc0, 1);
   alt_fn!(Ptc0, super::sig::Ftm0Ch0, 2);
   alt_fn!(Ptc0, super::sig::Lpspi2Sin, 3);
   alt_fn!(Ptc0, super::sig::Ftm1Ch6, 6);

pin!(PTC1, Ptc1, PORTC, Portc, _PTC1, PortPin, _PORTC, 1);
   alt_fn!(Ptc1, super::sig::Adc0Se9, 0);
   alt_fn!(Ptc1, super::sig::Ptc1, 1);
   alt_fn!(Ptc1, super::sig::Ftm0Ch1, 2);
   alt_fn!(Ptc1, super::sig::Lpspi2Sout, 3);
   alt_fn!(Ptc1, super::sig::Ftm1Ch7, 6);

pin!(PTC2, Ptc2, PORTC, Portc, _PTC2, PortPin, _PORTC, 2);
   alt_fn!(Ptc2, super::sig::Adc0Se10, 0);
   alt_fn!(Ptc2, super::sig::Cmp0In5, 0);
   alt_fn!(Ptc2, super::sig::Ptc2, 1);
   alt_fn!(Ptc2, super::sig::Ftm0Ch2, 2);
   alt_fn!(Ptc2, super::sig::Can0Rx, 3);
   alt_fn!(Ptc2, super::sig::Lpuart0Rx, 4);

pin!(PTC3, Ptc3, PORTC, Portc, _PTC3, PortPin, _PORTC, 3);
   alt_fn!(Ptc3, super::sig::Adc0Se11, 0);
   alt_fn!(Ptc3, super::sig::Cmp0In4, 0);
   alt_fn!(Ptc3, super::sig::Ptc3, 1);
   alt_fn!(Ptc3, super::sig::Ftm0Ch3, 2);
   alt_fn!(Ptc3, super::sig::Can0Tx, 3);
   alt_fn!(Ptc3, super::sig::Lpuart0Tx, 4);

pin!(PTC4, Ptc4, PORTC, Portc, _PTC4, PortPin, _PORTC, 4);
   alt_fn!(Ptc4, super::sig::Cmp0In2, 0);
   alt_fn!(Ptc4, super::sig::Ptc4, 1);
   alt_fn!(Ptc4, super::sig::Ftm1Ch0, 2);
   alt_fn!(Ptc4, super::sig::RtcClkout, 3);
   alt_fn!(Ptc4, super::sig::EwmIn, 5);
   alt_fn!(Ptc4, super::sig::Ftm1QdPhb, 6);
   alt_fn!(Ptc4, super::sig::JtagTclk, 7);
   alt_fn!(Ptc4, super::sig::SwdClk, 7);

pin!(PTC5, Ptc5, PORTC, Portc, _PTC5, PortPin, _PORTC, 5);
   alt_fn!(Ptc5, super::sig::Ptc5, 1);
   alt_fn!(Ptc5, super::sig::Ftm2Ch0, 2);
   alt_fn!(Ptc5, super::sig::RtcClkout, 3);
   alt_fn!(Ptc5, super::sig::Ftm2QdPhb, 6);
   alt_fn!(Ptc5, super::sig::JtagTdi, 7);

pin!(PTC6, Ptc6, PORTC, Portc, _PTC6, PortPin, _PORTC, 6);
   alt_fn!(Ptc6, super::sig::Adc1Se4, 0);
   alt_fn!(Ptc6, super::sig::Ptc6, 1);
   alt_fn!(Ptc6, super::sig::Lpuart1Rx, 2);
   alt_fn!(Ptc6, super::sig::Can1Rx, 3);
   alt_fn!(Ptc6, super::sig::Ftm3Ch2, 4);
   alt_fn!(Ptc6, super::sig::Ftm1QdPhb, 6);

pin!(PTC7, Ptc7, PORTC, Portc, _PTC7, PortPin, _PORTC, 7);
   alt_fn!(Ptc7, super::sig::Adc1Se5, 0);
   alt_fn!(Ptc7, super::sig::Ptc7, 1);
   alt_fn!(Ptc7, super::sig::Lpuart1Tx, 2);
   alt_fn!(Ptc7, super::sig::Can1Tx, 3);
   alt_fn!(Ptc7, super::sig::Ftm3Ch3, 4);
   alt_fn!(Ptc7, super::sig::Ftm1QdPha, 6);

pin!(PTC8, Ptc8, PORTC, Portc, _PTC8, PortPin, _PORTC, 8);
   alt_fn!(Ptc8, super::sig::Ptc8, 1);
   alt_fn!(Ptc8, super::sig::Lpuart1Rx, 2);
   alt_fn!(Ptc8, super::sig::Ftm1Flt0, 3);
   alt_fn!(Ptc8, super::sig::Lpuart0Cts, 6);

pin!(PTC9, Ptc9, PORTC, Portc, _PTC9, PortPin, _PORTC, 9);
   alt_fn!(Ptc9, super::sig::Ptc9, 1);
   alt_fn!(Ptc9, super::sig::Lpuart1Tx, 2);
   alt_fn!(Ptc9, super::sig::Ftm1Flt1, 3);
   alt_fn!(Ptc9, super::sig::Lpuart0Rts, 6);

pin!(PTC10, Ptc10, PORTC, Portc, _PTC10, PortPin, _PORTC, 10);
   alt_fn!(Ptc10, super::sig::Ptc10, 1);
   alt_fn!(Ptc10, super::sig::Ftm3Ch4, 2);
   alt_fn!(Ptc10, super::sig::TrgmuxIn11, 6);

pin!(PTC11, Ptc11, PORTC, Portc, _PTC11, PortPin, _PORTC, 11);
   alt_fn!(Ptc11, super::sig::Ptc11, 1);
   alt_fn!(Ptc11, super::sig::Ftm3Ch5, 2);
   alt_fn!(Ptc11, super::sig::TrgmuxIn10, 6);

pin!(PTC12, Ptc12, PORTC, Portc, _PTC12, PortPin, _PORTC, 12);
   alt_fn!(Ptc12, super::sig::Ptc12, 1);
   alt_fn!(Ptc12, super::sig::Ftm3Ch6, 2);
   alt_fn!(Ptc12, super::sig::Ftm2Ch6, 3);
   alt_fn!(Ptc12, super::sig::Lpuart2Cts, 4);

pin!(PTC13, Ptc13, PORTC, Portc, _PTC13, PortPin, _PORTC, 13);
   alt_fn!(Ptc13, super::sig::Ptc13, 1);
   alt_fn!(Ptc13, super::sig::Ftm3Ch7, 2);
   alt_fn!(Ptc13, super::sig::Ftm2Ch7, 3);
   alt_fn!(Ptc13, super::sig::Lpuart2Rts, 4);

pin!(PTC14, Ptc14, PORTC, Portc, _PTC14, PortPin, _PORTC, 14);
   alt_fn!(Ptc14, super::sig::Adc0Se12, 0);
   alt_fn!(Ptc14, super::sig::Ptc14, 1);
   alt_fn!(Ptc14, super::sig::Ftm1Ch2, 2);
   alt_fn!(Ptc14, super::sig::Lpspi2Pcs0, 3);
   alt_fn!(Ptc14, super::sig::TrgmuxIn9, 6);

pin!(PTC15, Ptc15, PORTC, Portc, _PTC15, PortPin, _PORTC, 15);
   alt_fn!(Ptc15, super::sig::Adc0Se13, 0);
   alt_fn!(Ptc15, super::sig::Ptc15, 1);
   alt_fn!(Ptc15, super::sig::Ftm1Ch3, 2);
   alt_fn!(Ptc15, super::sig::Lpspi2Sck, 3);
   alt_fn!(Ptc15, super::sig::TrgmuxIn8, 6);

pin!(PTC16, Ptc16, PORTC, Portc, _PTC16, PortPin, _PORTC, 16);
   alt_fn!(Ptc16, super::sig::Adc0Se14, 0);
   alt_fn!(Ptc16, super::sig::Ptc16, 1);
   alt_fn!(Ptc16, super::sig::Ftm1Flt2, 2);
   alt_fn!(Ptc16, super::sig::Can2Rx, 3);

pin!(PTC17, Ptc17, PORTC, Portc, _PTC17, PortPin, _PORTC, 17);
   alt_fn!(Ptc17, super::sig::Adc0Se15, 0);
   alt_fn!(Ptc17, super::sig::Ptc17, 1);
   alt_fn!(Ptc17, super::sig::Ftm1Flt3, 2);
   alt_fn!(Ptc17, super::sig::Can2Tx, 3);

pin!(PTD0, Ptd0, PORTD, Portd, _PTD0, PortPin, _PORTD, 0);
   alt_fn!(Ptd0, super::sig::Ptd0, 1);
   alt_fn!(Ptd0, super::sig::Ftm0Ch2, 2);
   alt_fn!(Ptd0, super::sig::Lpspi1Sck, 3);
   alt_fn!(Ptd0, super::sig::Ftm2Ch0, 4);
   alt_fn!(Ptd0, super::sig::FxioD0, 6);
   alt_fn!(Ptd0, super::sig::TrgmuxOut1, 7);

pin!(PTD1, Ptd1, PORTD, Portd, _PTD1, PortPin, _PORTD, 1);
   alt_fn!(Ptd1, super::sig::Ptd1, 1);
   alt_fn!(Ptd1, super::sig::Ftm0Ch3, 2);
   alt_fn!(Ptd1, super::sig::Lpspi1Sin, 3);
   alt_fn!(Ptd1, super::sig::Ftm2Ch1, 4);
   alt_fn!(Ptd1, super::sig::FxioD1, 6);
   alt_fn!(Ptd1, super::sig::TrgmuxOut2, 7);

pin!(PTD2, Ptd2, PORTD, Portd, _PTD2, PortPin, _PORTD, 2);
   alt_fn!(Ptd2, super::sig::Adc1Se2, 0);
   alt_fn!(Ptd2, super::sig::Ptd2, 1);
   alt_fn!(Ptd2, super::sig::Ftm3Ch4, 2);
   alt_fn!(Ptd2, super::sig::Lpspi1Sout, 3);
   alt_fn!(Ptd2, super::sig::FxioD4, 4);
   alt_fn!(Ptd2, super::sig::FxioD6, 5);
   alt_fn!(Ptd2, super::sig::TrgmuxIn5, 6);

pin!(PTD3, Ptd3, PORTD, Portd, _PTD3, PortPin, _PORTD, 3);
   alt_fn!(Ptd3, super::sig::Adc1Se3, 0);
   alt_fn!(Ptd3, super::sig::Ptd3, 1);
   alt_fn!(Ptd3, super::sig::Ftm3Ch5, 2);
   alt_fn!(Ptd3, super::sig::Lpspi1Pcs0, 3);
   alt_fn!(Ptd3, super::sig::FxioD5, 4);
   alt_fn!(Ptd3, super::sig::FxioD7, 5);
   alt_fn!(Ptd3, super::sig::TrgmuxIn4, 6);
   alt_fn!(Ptd3, super::sig::NmiB, 7);

pin!(PTD4, Ptd4, PORTD, Portd, _PTD4, PortPin, _PORTD, 4);
   alt_fn!(Ptd4, super::sig::Adc1Se6, 0);
   alt_fn!(Ptd4, super::sig::Ptd4, 1);
   alt_fn!(Ptd4, super::sig::Ftm0Flt3, 2);
   alt_fn!(Ptd4, super::sig::Ftm3Flt3, 3);

pin!(PTD5, Ptd5, PORTD, Portd, _PTD5, PortPin, _PORTD, 5);
   alt_fn!(Ptd5, super::sig::Ptd5, 1);
   alt_fn!(Ptd5, super::sig::Ftm2Ch3, 2);
   alt_fn!(Ptd5, super::sig::Lptmr0Alt2, 3);
   alt_fn!(Ptd5, super::sig::Ftm2Flt1, 4);
   alt_fn!(Ptd5, super::sig::TrgmuxIn7, 6);

pin!(PTD6, Ptd6, PORTD, Portd, _PTD6, PortPin, _PORTD, 6);
   alt_fn!(Ptd6, super::sig::Cmp0In7, 0);
   alt_fn!(Ptd6, super::sig::Ptd6, 1);
   alt_fn!(Ptd6, super::sig::Lpuart2Rx, 2);
   alt_fn!(Ptd6, super::sig::Ftm2Flt2, 4);

pin!(PTD7, Ptd7, PORTD, Portd, _PTD7, PortPin, _PORTD, 7);
   alt_fn!(Ptd7, super::sig::Cmp0In6, 0);
   alt_fn!(Ptd7, super::sig::Ptd7, 1);
   alt_fn!(Ptd7, super::sig::Lpuart2Tx, 2);
   alt_fn!(Ptd7, super::sig::Ftm2Flt3, 4);

pin!(PTD8, Ptd8, PORTD, Portd, _PTD8, PortPin, _PORTD, 8);
   alt_fn!(Ptd8, super::sig::Ptd8, 1);
   alt_fn!(Ptd8, super::sig::Ftm2Flt2, 4);
   alt_fn!(Ptd8, super::sig::FxioD1, 5);
   alt_fn!(Ptd8, super::sig::Ftm1Ch4, 6);

pin!(PTD9, Ptd9, PORTD, Portd, _PTD9, PortPin, _PORTD, 9);
   alt_fn!(Ptd9, super::sig::Ptd9, 1);
   alt_fn!(Ptd9, super::sig::FxioD0, 3);
   alt_fn!(Ptd9, super::sig::Ftm2Flt3, 4);
   alt_fn!(Ptd9, super::sig::Ftm1Ch5, 6);

pin!(PTD10, Ptd10, PORTD, Portd, _PTD10, PortPin, _PORTD, 10);
   alt_fn!(Ptd10, super::sig::Ptd10, 1);
   alt_fn!(Ptd10, super::sig::Ftm2Ch0, 2);
   alt_fn!(Ptd10, super::sig::Ftm2QdPhb, 3);

pin!(PTD11, Ptd11, PORTD, Portd, _PTD11, PortPin, _PORTD, 11);
   alt_fn!(Ptd11, super::sig::Ptd11, 1);
   alt_fn!(Ptd11, super::sig::Ftm2Ch1, 2);
   alt_fn!(Ptd11, super::sig::Ftm2QdPha, 3);
   alt_fn!(Ptd11, super::sig::Lpuart2Cts, 6);

pin!(PTD12, Ptd12, PORTD, Portd, _PTD12, PortPin, _PORTD, 12);
   alt_fn!(Ptd12, super::sig::Ptd12, 1);
   alt_fn!(Ptd12, super::sig::Ftm2Ch2, 2);
   alt_fn!(Ptd12, super::sig::Lpuart2Rts, 6);

pin!(PTD13, Ptd13, PORTD, Portd, _PTD13, PortPin, _PORTD, 13);
   alt_fn!(Ptd13, super::sig::Ptd13, 1);
   alt_fn!(Ptd13, super::sig::Ftm2Ch4, 2);
   alt_fn!(Ptd13, super::sig::Lpuart1Rx, 3);
   alt_fn!(Ptd13, super::sig::RtcClkout, 7);

pin!(PTD14, Ptd14, PORTD, Portd, _PTD14, PortPin, _PORTD, 14);
   alt_fn!(Ptd14, super::sig::Ptd14, 1);
   alt_fn!(Ptd14, super::sig::Ftm2Ch5, 2);
   alt_fn!(Ptd14, super::sig::Lpuart1Tx, 3);
   alt_fn!(Ptd14, super::sig::Clkout, 7);

pin!(PTD15, Ptd15, PORTD, Portd, _PTD15, PortPin, _PORTD, 15);
   alt_fn!(Ptd15, super::sig::Ptd15, 1);
   alt_fn!(Ptd15, super::sig::Ftm0Ch0, 2);
   alt_fn!(Ptd15, super::sig::Lpspi0Sck, 4);

pin!(PTD16, Ptd16, PORTD, Portd, _PTD16, PortPin, _PORTD, 16);
   alt_fn!(Ptd16, super::sig::Ptd16, 1);
   alt_fn!(Ptd16, super::sig::Ftm0Ch1, 2);
   alt_fn!(Ptd16, super::sig::Lpspi0Sin, 4);
   alt_fn!(Ptd16, super::sig::Cmp0Rrt, 5);

pin!(PTD17, Ptd17, PORTD, Portd, _PTD17, PortPin, _PORTD, 17);
   alt_fn!(Ptd17, super::sig::Ptd17, 1);
   alt_fn!(Ptd17, super::sig::Ftm0Flt2, 2);
   alt_fn!(Ptd17, super::sig::Lpuart2Rx, 3);

pin!(PTE0, Pte0, PORTE, Porte, _PTE0, PortPin, _PORTE, 0);
   alt_fn!(Pte0, super::sig::Pte0, 1);
   alt_fn!(Pte0, super::sig::Lpspi0Sck, 2);
   alt_fn!(Pte0, super::sig::Tclk1, 3);
   alt_fn!(Pte0, super::sig::Lpspi1Sout, 5);
   alt_fn!(Pte0, super::sig::Ftm1Flt2, 6);

pin!(PTE1, Pte1, PORTE, Porte, _PTE1, PortPin, _PORTE, 1);
   alt_fn!(Pte1, super::sig::Pte1, 1);
   alt_fn!(Pte1, super::sig::Lpspi0Sin, 2);
   alt_fn!(Pte1, super::sig::Lpi2c0Hreq, 3);
   alt_fn!(Pte1, super::sig::Lpspi1Pcs0, 5);
   alt_fn!(Pte1, super::sig::Ftm1Flt1, 6);

pin!(PTE2, Pte2, PORTE, Porte, _PTE2, PortPin, _PORTE, 2);
   alt_fn!(Pte2, super::sig::Adc1Se10, 0);
   alt_fn!(Pte2, super::sig::Pte2, 1);
   alt_fn!(Pte2, super::sig::Lpspi0Sout, 2);
   alt_fn!(Pte2, super::sig::Lptmr0Alt3, 3);
   alt_fn!(Pte2, super::sig::Ftm3Ch6, 4);
   alt_fn!(Pte2, super::sig::Lpuart1Cts, 6);

pin!(PTE3, Pte3, PORTE, Porte, _PTE3, PortPin, _PORTE, 3);
   alt_fn!(Pte3, super::sig::Pte3, 1);
   alt_fn!(Pte3, super::sig::Ftm0Flt0, 2);
   alt_fn!(Pte3, super::sig::Lpuart2Rts, 3);
   alt_fn!(Pte3, super::sig::Ftm2Flt0, 4);
   alt_fn!(Pte3, super::sig::TrgmuxIn6, 6);
   alt_fn!(Pte3, super::sig::Cmp0Out, 7);

pin!(PTE4, Pte4, PORTE, Porte, _PTE4, PortPin, _PORTE, 4);
   alt_fn!(Pte4, super::sig::Pte4, 1);
   alt_fn!(Pte4, super::sig::Ftm2QdPhb, 3);
   alt_fn!(Pte4, super::sig::Ftm2Ch2, 4);
   alt_fn!(Pte4, super::sig::Can0Rx, 5);
   alt_fn!(Pte4, super::sig::FxioD6, 6);
   alt_fn!(Pte4, super::sig::EwmOutB, 7);

pin!(PTE5, Pte5, PORTE, Porte, _PTE5, PortPin, _PORTE, 5);
   alt_fn!(Pte5, super::sig::Pte5, 1);
   alt_fn!(Pte5, super::sig::Tclk2, 2);
   alt_fn!(Pte5, super::sig::Ftm2QdPha, 3);
   alt_fn!(Pte5, super::sig::Ftm2Ch3, 4);
   alt_fn!(Pte5, super::sig::Can0Tx, 5);
   alt_fn!(Pte5, super::sig::FxioD7, 6);
   alt_fn!(Pte5, super::sig::EwmIn, 7);

pin!(PTE6, Pte6, PORTE, Porte, _PTE6, PortPin, _PORTE, 6);
   alt_fn!(Pte6, super::sig::Adc1Se11, 0);
   alt_fn!(Pte6, super::sig::Pte6, 1);
   alt_fn!(Pte6, super::sig::Lpspi0Pcs2, 2);
   alt_fn!(Pte6, super::sig::Ftm3Ch7, 4);
   alt_fn!(Pte6, super::sig::Lpuart1Rts, 6);

pin!(PTE7, Pte7, PORTE, Porte, _PTE7, PortPin, _PORTE, 7);
   alt_fn!(Pte7, super::sig::Pte7, 1);
   alt_fn!(Pte7, super::sig::Ftm0Ch7, 2);
   alt_fn!(Pte7, super::sig::Ftm3Flt0, 3);

pin!(PTE8, Pte8, PORTE, Porte, _PTE8, PortPin, _PORTE, 8);
   alt_fn!(Pte8, super::sig::Cmp0In3, 0);
   alt_fn!(Pte8, super::sig::Pte8, 1);
   alt_fn!(Pte8, super::sig::Ftm0Ch6, 2);

pin!(PTE9, Pte9, PORTE, Porte, _PTE9, PortPin, _PORTE, 9);
   alt_fn!(Pte9, super::sig::Pte9, 1);
   alt_fn!(Pte9, super::sig::Ftm0Ch7, 2);
   alt_fn!(Pte9, super::sig::Lpuart2Cts, 3);

pin!(PTE10, Pte10, PORTE, Porte, _PTE10, PortPin, _PORTE, 10);
   alt_fn!(Pte10, super::sig::Pte10, 1);
   alt_fn!(Pte10, super::sig::Clkout, 2);
   alt_fn!(Pte10, super::sig::Lpspi2Pcs1, 3);
   alt_fn!(Pte10, super::sig::Ftm2Ch4, 4);
   alt_fn!(Pte10, super::sig::FxioD4, 6);
   alt_fn!(Pte10, super::sig::TrgmuxOut4, 7);

pin!(PTE11, Pte11, PORTE, Porte, _PTE11, PortPin, _PORTE, 11);
   alt_fn!(Pte11, super::sig::Pte11, 1);
   alt_fn!(Pte11, super::sig::Lpspi2Pcs0, 2);
   alt_fn!(Pte11, super::sig::Lptmr0Alt1, 3);
   alt_fn!(Pte11, super::sig::Ftm2Ch5, 4);
   alt_fn!(Pte11, super::sig::FxioD5, 6);
   alt_fn!(Pte11, super::sig::TrgmuxOut5, 7);

pin!(PTE12, Pte12, PORTE, Porte, _PTE12, PortPin, _PORTE, 12);
   alt_fn!(Pte12, super::sig::Pte12, 1);
   alt_fn!(Pte12, super::sig::Ftm0Flt3, 2);
   alt_fn!(Pte12, super::sig::Lpuart2Tx, 3);

pin!(PTE13, Pte13, PORTE, Porte, _PTE13, PortPin, _PORTE, 13);
   alt_fn!(Pte13, super::sig::Pte13, 1);
   alt_fn!(Pte13, super::sig::Lpspi2Pcs2, 3);
   alt_fn!(Pte13, super::sig::Ftm2Flt0, 4);

pin!(PTE14, Pte14, PORTE, Porte, _PTE14, PortPin, _PORTE, 14);
   alt_fn!(Pte14, super::sig::Pte14, 1);
   alt_fn!(Pte14, super::sig::Ftm0Flt1, 2);
   alt_fn!(Pte14, super::sig::Ftm2Flt1, 4);

pin!(PTE15, Pte15, PORTE, Porte, _PTE15, PortPin, _PORTE, 15);
   alt_fn!(Pte15, super::sig::Pte15, 1);
   alt_fn!(Pte15, super::sig::Lpuart1Cts, 2);
   alt_fn!(Pte15, super::sig::Lpspi2Sck, 3);
   alt_fn!(Pte15, super::sig::Ftm2Ch6, 4);
   alt_fn!(Pte15, super::sig::FxioD2, 6);
   alt_fn!(Pte15, super::sig::TrgmuxOut6, 7);

pin!(PTE16, Pte16, PORTE, Porte, _PTE16, PortPin, _PORTE, 16);
   alt_fn!(Pte16, super::sig::Pte16, 1);
   alt_fn!(Pte16, super::sig::Lpuart1Rts, 2);
   alt_fn!(Pte16, super::sig::Lpspi2Sin, 3);
   alt_fn!(Pte16, super::sig::Ftm2Ch7, 4);
   alt_fn!(Pte16, super::sig::FxioD3, 6);
   alt_fn!(Pte16, super::sig::TrgmuxOut7, 7);

