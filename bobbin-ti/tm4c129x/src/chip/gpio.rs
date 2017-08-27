#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "GPIO", peripherals: [Peripheral { derived_from: None, group_name: None, name: "GPIOA", address: 1073758208, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Register map for GPIOA peripheral"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "GPIOA", types: [], value: 0, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [Pin { name: "PA0", index: Some(0), description: None, altfns: [AltFn { index: 1, signal: "U0Rx", description: None }, AltFn { index: 2, signal: "I2C9SCL", description: None }, AltFn { index: 3, signal: "T0CCP0", description: None }, AltFn { index: 7, signal: "CAN0Rx", description: None }] }, Pin { name: "PA1", index: Some(1), description: None, altfns: [AltFn { index: 1, signal: "U0Tx", description: None }, AltFn { index: 2, signal: "I2C9SDA", description: None }, AltFn { index: 3, signal: "T0CCP1", description: None }, AltFn { index: 7, signal: "CAN0Tx", description: None }] }, Pin { name: "PA2", index: Some(2), description: None, altfns: [AltFn { index: 1, signal: "U4Rx", description: None }, AltFn { index: 2, signal: "I2C8SCL", description: None }, AltFn { index: 3, signal: "T1CCP0", description: None }, AltFn { index: 15, signal: "SSI0Clk", description: None }] }, Pin { name: "PA3", index: Some(3), description: None, altfns: [AltFn { index: 1, signal: "U4Tx", description: None }, AltFn { index: 2, signal: "I2C8SDA", description: None }, AltFn { index: 3, signal: "T1CCP1", description: None }, AltFn { index: 15, signal: "SSI0Fss", description: None }] }, Pin { name: "PA4", index: Some(4), description: None, altfns: [AltFn { index: 1, signal: "U3Rx", description: None }, AltFn { index: 2, signal: "I2C7SCL", description: None }, AltFn { index: 3, signal: "T2CCP0", description: None }, AltFn { index: 15, signal: "SSI0XDAT0", description: None }] }, Pin { name: "PA5", index: Some(5), description: None, altfns: [AltFn { index: 1, signal: "U3Tx", description: None }, AltFn { index: 2, signal: "I2C7SDA", description: None }, AltFn { index: 3, signal: "T2CCP1", description: None }, AltFn { index: 15, signal: "SSI0XDAT1", description: None }] }, Pin { name: "PA6", index: Some(6), description: None, altfns: [AltFn { index: 1, signal: "U2Rx", description: None }, AltFn { index: 2, signal: "I2C6SCL", description: None }, AltFn { index: 3, signal: "T3CCP0", description: None }, AltFn { index: 5, signal: "USB0EPEN", description: None }, AltFn { index: 13, signal: "SSI0XDAT2", description: None }, AltFn { index: 15, signal: "EPI0S8", description: None }] }, Pin { name: "PA7", index: Some(7), description: None, altfns: [AltFn { index: 1, signal: "U2Tx", description: None }, AltFn { index: 2, signal: "I2C6SDA", description: None }, AltFn { index: 3, signal: "T3CCP1", description: None }, AltFn { index: 5, signal: "USB0PFLT", description: None }, AltFn { index: 11, signal: "USB0EPEN", description: None }, AltFn { index: 13, signal: "SSI0XDAT3", description: None }, AltFn { index: 15, signal: "EPI0S9", description: None }] }], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "GPIOB", address: 1073762304, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "GPIOB", types: [], value: 1, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [Pin { name: "PB0", index: Some(0), description: None, altfns: [AltFn { index: 0, signal: "USB0ID", description: None }, AltFn { index: 1, signal: "U1Rx", description: None }, AltFn { index: 2, signal: "I2C5SCL", description: None }, AltFn { index: 3, signal: "T4CCP0", description: None }, AltFn { index: 7, signal: "CAN1Rx", description: None }] }, Pin { name: "PB1", index: Some(1), description: None, altfns: [AltFn { index: 0, signal: "USB0VBUS", description: None }, AltFn { index: 1, signal: "U1Tx", description: None }, AltFn { index: 2, signal: "I2C5SDA", description: None }, AltFn { index: 3, signal: "T4CCP1", description: None }, AltFn { index: 7, signal: "CAN1Tx", description: None }] }, Pin { name: "PB2", index: Some(2), description: None, altfns: [AltFn { index: 2, signal: "I2C0SCL", description: None }, AltFn { index: 3, signal: "T5CCP0", description: None }, AltFn { index: 14, signal: "USB0STP", description: None }, AltFn { index: 15, signal: "EPI0S27", description: None }] }, Pin { name: "PB3", index: Some(3), description: None, altfns: [AltFn { index: 2, signal: "I2C0SDA", description: None }, AltFn { index: 3, signal: "T5CCP1", description: None }, AltFn { index: 14, signal: "USB0CLK", description: None }, AltFn { index: 15, signal: "EPI0S28", description: None }] }, Pin { name: "PB4", index: Some(4), description: None, altfns: [AltFn { index: 0, signal: "AIN10", description: None }, AltFn { index: 1, signal: "U0CTS", description: None }, AltFn { index: 2, signal: "I2C5SCL", description: None }, AltFn { index: 15, signal: "SSI1Fss", description: None }] }, Pin { name: "PB5", index: Some(5), description: None, altfns: [AltFn { index: 0, signal: "AIN11", description: None }, AltFn { index: 1, signal: "U0RTS", description: None }, AltFn { index: 2, signal: "I2C5SDA", description: None }, AltFn { index: 15, signal: "SSI1Clk", description: None }] }], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "GPIOC", address: 1073766400, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "GPIOC", types: [], value: 2, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [Pin { name: "PC0", index: Some(0), description: None, altfns: [AltFn { index: 1, signal: "TCK", description: None }, AltFn { index: 1, signal: "SWCLK", description: None }] }, Pin { name: "PC1", index: Some(1), description: None, altfns: [AltFn { index: 1, signal: "TMS", description: None }, AltFn { index: 1, signal: "SWDIO", description: None }] }, Pin { name: "PC2", index: Some(2), description: None, altfns: [AltFn { index: 1, signal: "TDI", description: None }] }, Pin { name: "PC3", index: Some(3), description: None, altfns: [AltFn { index: 1, signal: "TDO", description: None }, AltFn { index: 1, signal: "SWO", description: None }] }, Pin { name: "PC4", index: Some(4), description: None, altfns: [AltFn { index: 0, signal: "C1_NEG", description: None }, AltFn { index: 1, signal: "U7Rx", description: None }, AltFn { index: 15, signal: "EPI0S7", description: None }] }, Pin { name: "PC5", index: Some(5), description: None, altfns: [AltFn { index: 0, signal: "C1_POS", description: None }, AltFn { index: 1, signal: "U7Tx", description: None }, AltFn { index: 7, signal: "RTCCLK", description: None }, AltFn { index: 15, signal: "EPI0S6", description: None }] }, Pin { name: "PC6", index: Some(6), description: None, altfns: [AltFn { index: 0, signal: "C0_NEG", description: None }, AltFn { index: 1, signal: "U5Rx", description: None }, AltFn { index: 15, signal: "EPI0S5", description: None }] }, Pin { name: "PC7", index: Some(7), description: None, altfns: [AltFn { index: 0, signal: "C0_POS", description: None }, AltFn { index: 1, signal: "U5Tx", description: None }, AltFn { index: 15, signal: "EPI0S4", description: None }] }], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "GPIOD", address: 1073770496, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "GPIOD", types: [], value: 3, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [Pin { name: "PD0", index: Some(0), description: None, altfns: [AltFn { index: 0, signal: "AIN15", description: None }, AltFn { index: 2, signal: "I2C7SCL", description: None }, AltFn { index: 3, signal: "T0CCP0", description: None }, AltFn { index: 5, signal: "C0o", description: None }, AltFn { index: 15, signal: "SSI2XDAT1", description: None }] }, Pin { name: "PD1", index: Some(1), description: None, altfns: [AltFn { index: 0, signal: "AIN14", description: None }, AltFn { index: 2, signal: "I2C7SDA", description: None }, AltFn { index: 3, signal: "T0CCP1", description: None }, AltFn { index: 5, signal: "C1o", description: None }, AltFn { index: 15, signal: "SSI2XDAT0", description: None }] }, Pin { name: "PD2", index: Some(2), description: None, altfns: [AltFn { index: 0, signal: "AIN13", description: None }, AltFn { index: 2, signal: "I2C8SCL", description: None }, AltFn { index: 3, signal: "T1CCP0", description: None }, AltFn { index: 5, signal: "C2o", description: None }, AltFn { index: 15, signal: "SSI2Fss", description: None }] }, Pin { name: "PD3", index: Some(3), description: None, altfns: [AltFn { index: 0, signal: "AIN12", description: None }, AltFn { index: 2, signal: "I2C8SDA", description: None }, AltFn { index: 3, signal: "T1CCP1", description: None }, AltFn { index: 15, signal: "SSI2Clk", description: None }] }, Pin { name: "PD4", index: Some(4), description: None, altfns: [AltFn { index: 0, signal: "AIN7", description: None }, AltFn { index: 1, signal: "U2Rx", description: None }, AltFn { index: 3, signal: "T3CCP0", description: None }, AltFn { index: 15, signal: "SSI1XDAT2", description: None }] }, Pin { name: "PD5", index: Some(5), description: None, altfns: [AltFn { index: 0, signal: "AIN6", description: None }, AltFn { index: 1, signal: "U2Tx", description: None }, AltFn { index: 3, signal: "T3CCP1", description: None }, AltFn { index: 15, signal: "SSI1XDAT3", description: None }] }, Pin { name: "PD6", index: Some(6), description: None, altfns: [AltFn { index: 0, signal: "AIN5", description: None }, AltFn { index: 1, signal: "U2RTS", description: None }, AltFn { index: 3, signal: "T4CCP0", description: None }, AltFn { index: 5, signal: "USB0EPEN", description: None }, AltFn { index: 15, signal: "SSI2XDAT3", description: None }] }, Pin { name: "PD7", index: Some(7), description: None, altfns: [AltFn { index: 0, signal: "AIN4", description: None }, AltFn { index: 1, signal: "U2CTS", description: None }, AltFn { index: 3, signal: "T4CCP1", description: None }, AltFn { index: 5, signal: "USB0PFLT", description: None }, AltFn { index: 8, signal: "NMI", description: None }, AltFn { index: 15, signal: "SSI2XDAT2", description: None }] }], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "GPIOE", address: 1073889280, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "GPIOE", types: [], value: 4, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [Pin { name: "PE0", index: Some(0), description: None, altfns: [AltFn { index: 0, signal: "AIN3", description: None }, AltFn { index: 1, signal: "U1RTS", description: None }] }, Pin { name: "PE1", index: Some(1), description: None, altfns: [AltFn { index: 0, signal: "AIN2", description: None }, AltFn { index: 1, signal: "U1DSR", description: None }] }, Pin { name: "PE2", index: Some(2), description: None, altfns: [AltFn { index: 0, signal: "AIN1", description: None }, AltFn { index: 1, signal: "U1DCD", description: None }] }, Pin { name: "PE3", index: Some(3), description: None, altfns: [AltFn { index: 0, signal: "AIN0", description: None }, AltFn { index: 1, signal: "U1DTR", description: None }] }, Pin { name: "PE4", index: Some(4), description: None, altfns: [AltFn { index: 0, signal: "AIN9", description: None }, AltFn { index: 1, signal: "U1RI", description: None }, AltFn { index: 15, signal: "SSI1XDAT0", description: None }] }, Pin { name: "PE5", index: Some(5), description: None, altfns: [AltFn { index: 0, signal: "AIN8", description: None }, AltFn { index: 15, signal: "SSI1XDAT1", description: None }] }], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "GPIOF", address: 1073893376, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "GPIOF", types: [], value: 30, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [Pin { name: "PF0", index: Some(0), description: None, altfns: [AltFn { index: 5, signal: "EN0LED0", description: None }, AltFn { index: 6, signal: "M0PWM0", description: None }, AltFn { index: 14, signal: "SSI3XDAT1", description: None }, AltFn { index: 15, signal: "TRD2", description: None }] }, Pin { name: "PF1", index: Some(1), description: None, altfns: [AltFn { index: 5, signal: "EN0LED2", description: None }, AltFn { index: 6, signal: "M0PWM1", description: None }, AltFn { index: 14, signal: "SSI3XDAT0", description: None }, AltFn { index: 15, signal: "TRD1", description: None }] }, Pin { name: "PF2", index: Some(2), description: None, altfns: [AltFn { index: 6, signal: "M0PWM2", description: None }, AltFn { index: 14, signal: "SSI3Fss", description: None }, AltFn { index: 15, signal: "TRD0", description: None }] }, Pin { name: "PF3", index: Some(3), description: None, altfns: [AltFn { index: 6, signal: "M0PWM3", description: None }, AltFn { index: 14, signal: "SSI3Clk", description: None }, AltFn { index: 15, signal: "TRCLK", description: None }] }, Pin { name: "PF4", index: Some(4), description: None, altfns: [AltFn { index: 5, signal: "EN0LED1", description: None }, AltFn { index: 6, signal: "M0FAULT0", description: None }, AltFn { index: 14, signal: "SSI3XDAT2", description: None }, AltFn { index: 15, signal: "TRD3", description: None }] }], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "GPIOG", address: 1073897472, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "GPIOG", types: [], value: 31, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [Pin { name: "PG0", index: Some(0), description: None, altfns: [AltFn { index: 2, signal: "I2C1SCL", description: None }, AltFn { index: 5, signal: "EN0PPS", description: None }, AltFn { index: 6, signal: "M0PWM4", description: None }, AltFn { index: 15, signal: "EPI0S11", description: None }] }, Pin { name: "PG1", index: Some(1), description: None, altfns: [AltFn { index: 2, signal: "I2C1SDA", description: None }, AltFn { index: 6, signal: "M0PWM5", description: None }, AltFn { index: 15, signal: "EPI0S10", description: None }] }], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "GPIOH", address: 1073901568, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "GPIOH", types: [], value: 32, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [Pin { name: "PH0", index: Some(0), description: None, altfns: [AltFn { index: 1, signal: "U0RTS", description: None }, AltFn { index: 15, signal: "EPI0S0", description: None }] }, Pin { name: "PH1", index: Some(1), description: None, altfns: [AltFn { index: 1, signal: "U0CTS", description: None }, AltFn { index: 15, signal: "EPI0S1", description: None }] }, Pin { name: "PH2", index: Some(2), description: None, altfns: [AltFn { index: 1, signal: "U0DCD", description: None }, AltFn { index: 15, signal: "EPI0S2", description: None }] }, Pin { name: "PH3", index: Some(3), description: None, altfns: [AltFn { index: 1, signal: "U0DSR", description: None }, AltFn { index: 15, signal: "EPI0S3", description: None }] }], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "GPIOJ", address: 1073991680, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "GPIOJ", types: [], value: 54, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [Pin { name: "PJ0", index: Some(0), description: None, altfns: [AltFn { index: 1, signal: "U3Rx", description: None }, AltFn { index: 5, signal: "EN0PPS", description: None }] }, Pin { name: "PJ1", index: Some(1), description: None, altfns: [AltFn { index: 1, signal: "U3Tx", description: None }] }], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "GPIOA_AHB", address: 1074102272, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "GPIOA_AHB", types: [], value: 0, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "GPIOB_AHB", address: 1074106368, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "GPIOB_AHB", types: [], value: 1, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "GPIOC_AHB", address: 1074110464, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "GPIOC_AHB", types: [], value: 2, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "GPIOD_AHB", address: 1074114560, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "GPIOD_AHB", types: [], value: 3, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "GPIOE_AHB", address: 1074118656, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "GPIOE_AHB", types: [], value: 4, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "GPIOF_AHB", address: 1074122752, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "GPIOF_AHB", types: [], value: 30, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "GPIOG_AHB", address: 1074126848, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "GPIOG_AHB", types: [], value: 31, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "GPIOH_AHB", address: 1074130944, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "GPIOH_AHB", types: [], value: 32, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "GPIOJ_AHB", address: 1074135040, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "GPIOJ_AHB", types: [], value: 54, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "GPIOK", address: 1074139136, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "GPIOK", types: [], value: 55, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [Pin { name: "PK0", index: Some(0), description: None, altfns: [AltFn { index: 0, signal: "AIN16", description: None }, AltFn { index: 1, signal: "U4Rx", description: None }, AltFn { index: 15, signal: "EPI0S0", description: None }] }, Pin { name: "PK1", index: Some(1), description: None, altfns: [AltFn { index: 0, signal: "AIN17", description: None }, AltFn { index: 1, signal: "U4Tx", description: None }, AltFn { index: 15, signal: "EPI0S1", description: None }] }, Pin { name: "PK2", index: Some(2), description: None, altfns: [AltFn { index: 0, signal: "AIN18", description: None }, AltFn { index: 1, signal: "U4RTS", description: None }, AltFn { index: 15, signal: "EPI0S2", description: None }] }, Pin { name: "PK3", index: Some(3), description: None, altfns: [AltFn { index: 0, signal: "AIN19", description: None }, AltFn { index: 1, signal: "U4CTS", description: None }, AltFn { index: 15, signal: "EPI0S3", description: None }] }, Pin { name: "PK4", index: Some(4), description: None, altfns: [AltFn { index: 2, signal: "I2C3SCL", description: None }, AltFn { index: 5, signal: "EN0LED0", description: None }, AltFn { index: 6, signal: "M0PWM6", description: None }, AltFn { index: 15, signal: "EPI0S32", description: None }] }, Pin { name: "PK5", index: Some(5), description: None, altfns: [AltFn { index: 2, signal: "I2C3SDA", description: None }, AltFn { index: 5, signal: "EN0LED2", description: None }, AltFn { index: 6, signal: "M0PWM7", description: None }, AltFn { index: 15, signal: "EPI0S31", description: None }] }, Pin { name: "PK6", index: Some(6), description: None, altfns: [AltFn { index: 2, signal: "I2C4SCL", description: None }, AltFn { index: 5, signal: "EN0LED1", description: None }, AltFn { index: 6, signal: "M0FAULT1", description: None }, AltFn { index: 15, signal: "EPI0S25", description: None }] }, Pin { name: "PK7", index: Some(7), description: None, altfns: [AltFn { index: 1, signal: "U0RI", description: None }, AltFn { index: 2, signal: "I2C4SDA", description: None }, AltFn { index: 5, signal: "RTCCLK", description: None }, AltFn { index: 6, signal: "M0FAULT2", description: None }, AltFn { index: 15, signal: "EPI0S24", description: None }] }], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "GPIOL", address: 1074143232, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "GPIOL", types: [], value: 53, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [Pin { name: "PL0", index: Some(0), description: None, altfns: [AltFn { index: 2, signal: "I2C2SDA", description: None }, AltFn { index: 6, signal: "M0FAULT3", description: None }, AltFn { index: 14, signal: "USB0D0", description: None }, AltFn { index: 15, signal: "EPI0S16", description: None }] }, Pin { name: "PL1", index: Some(1), description: None, altfns: [AltFn { index: 2, signal: "I2C2SCL", description: None }, AltFn { index: 6, signal: "PhA0", description: None }, AltFn { index: 14, signal: "USB0D1", description: None }, AltFn { index: 15, signal: "EPI0S17", description: None }] }, Pin { name: "PL2", index: Some(2), description: None, altfns: [AltFn { index: 5, signal: "C0o", description: None }, AltFn { index: 6, signal: "PhB0", description: None }, AltFn { index: 14, signal: "USB0D2", description: None }, AltFn { index: 15, signal: "EPI0S18", description: None }] }, Pin { name: "PL3", index: Some(3), description: None, altfns: [AltFn { index: 5, signal: "C1o", description: None }, AltFn { index: 6, signal: "IDX0", description: None }, AltFn { index: 14, signal: "USB0D3", description: None }, AltFn { index: 15, signal: "EPI0S19", description: None }] }, Pin { name: "PL4", index: Some(4), description: None, altfns: [AltFn { index: 3, signal: "T0CCP0", description: None }, AltFn { index: 14, signal: "USB0D4", description: None }, AltFn { index: 15, signal: "EPI0S26", description: None }] }, Pin { name: "PL5", index: Some(5), description: None, altfns: [AltFn { index: 3, signal: "T0CCP1", description: None }, AltFn { index: 14, signal: "USB0D5", description: None }, AltFn { index: 15, signal: "EPI0S33", description: None }] }, Pin { name: "PL6", index: Some(6), description: None, altfns: [AltFn { index: 0, signal: "USB0DP", description: None }, AltFn { index: 3, signal: "T1CCP0", description: None }] }, Pin { name: "PL7", index: Some(7), description: None, altfns: [AltFn { index: 0, signal: "USB0DM", description: None }, AltFn { index: 3, signal: "T1CCP1", description: None }] }], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "GPIOM", address: 1074147328, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "GPIOM", types: [], value: 72, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [Pin { name: "PM0", index: Some(0), description: None, altfns: [AltFn { index: 3, signal: "T2CCP0", description: None }, AltFn { index: 15, signal: "EPI0S15", description: None }] }, Pin { name: "PM1", index: Some(1), description: None, altfns: [AltFn { index: 3, signal: "T2CCP1", description: None }, AltFn { index: 15, signal: "EPI0S14", description: None }] }, Pin { name: "PM2", index: Some(2), description: None, altfns: [AltFn { index: 3, signal: "T3CCP0", description: None }, AltFn { index: 15, signal: "EPI0S13", description: None }] }, Pin { name: "PM3", index: Some(3), description: None, altfns: [AltFn { index: 3, signal: "T3CCP1", description: None }, AltFn { index: 15, signal: "EPI0S12", description: None }] }, Pin { name: "PM4", index: Some(4), description: None, altfns: [AltFn { index: 0, signal: "TMPR3", description: None }, AltFn { index: 1, signal: "U0CTS", description: None }, AltFn { index: 3, signal: "T4CCP0", description: None }] }, Pin { name: "PM5", index: Some(5), description: None, altfns: [AltFn { index: 0, signal: "TMPR2", description: None }, AltFn { index: 1, signal: "U0DCD", description: None }, AltFn { index: 3, signal: "T4CCP1", description: None }] }, Pin { name: "PM6", index: Some(6), description: None, altfns: [AltFn { index: 0, signal: "TMPR1", description: None }, AltFn { index: 1, signal: "U0DSR", description: None }, AltFn { index: 3, signal: "T5CCP0", description: None }] }, Pin { name: "PM7", index: Some(7), description: None, altfns: [AltFn { index: 0, signal: "TMPR0", description: None }, AltFn { index: 1, signal: "U0RI", description: None }, AltFn { index: 3, signal: "T5CCP1", description: None }] }], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "GPION", address: 1074151424, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "GPION", types: [], value: 73, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [Pin { name: "PN0", index: Some(0), description: None, altfns: [AltFn { index: 1, signal: "U1RTS", description: None }] }, Pin { name: "PN1", index: Some(1), description: None, altfns: [AltFn { index: 1, signal: "U1CTS", description: None }] }, Pin { name: "PN2", index: Some(2), description: None, altfns: [AltFn { index: 1, signal: "U1DCD", description: None }, AltFn { index: 2, signal: "U2RTS", description: None }, AltFn { index: 15, signal: "EPI0S29", description: None }] }, Pin { name: "PN3", index: Some(3), description: None, altfns: [AltFn { index: 1, signal: "U1DSR", description: None }, AltFn { index: 2, signal: "U2CTS", description: None }, AltFn { index: 15, signal: "EPI0S30", description: None }] }, Pin { name: "PN4", index: Some(4), description: None, altfns: [AltFn { index: 1, signal: "U1DTR", description: None }, AltFn { index: 2, signal: "U3RTS", description: None }, AltFn { index: 3, signal: "I2C2SDA", description: None }, AltFn { index: 15, signal: "EPI0S34", description: None }] }, Pin { name: "PN5", index: Some(5), description: None, altfns: [AltFn { index: 1, signal: "U1RI", description: None }, AltFn { index: 2, signal: "U3CTS", description: None }, AltFn { index: 3, signal: "I2C2SCL", description: None }, AltFn { index: 15, signal: "EPI0S35", description: None }] }], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "GPIOP", address: 1074155520, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "GPIOP0", types: [], value: 76, description: None }, Interrupt { name: "GPIOP1", types: [], value: 77, description: None }, Interrupt { name: "GPIOP2", types: [], value: 78, description: None }, Interrupt { name: "GPIOP3", types: [], value: 79, description: None }, Interrupt { name: "GPIOP4", types: [], value: 80, description: None }, Interrupt { name: "GPIOP5", types: [], value: 81, description: None }, Interrupt { name: "GPIOP6", types: [], value: 82, description: None }, Interrupt { name: "GPIOP7", types: [], value: 83, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [Pin { name: "PP0", index: Some(0), description: None, altfns: [AltFn { index: 0, signal: "C2_POS", description: None }, AltFn { index: 1, signal: "U6Rx", description: None }, AltFn { index: 15, signal: "SSI3XDAT", description: None }] }, Pin { name: "PP1", index: Some(1), description: None, altfns: [AltFn { index: 0, signal: "C2_NEG", description: None }, AltFn { index: 1, signal: "U6Tx", description: None }, AltFn { index: 15, signal: "SSI3XDAT", description: None }] }, Pin { name: "PP2", index: Some(2), description: None, altfns: [AltFn { index: 1, signal: "U0DTR", description: None }, AltFn { index: 14, signal: "USB0NXT", description: None }, AltFn { index: 15, signal: "EPI0S29", description: None }] }, Pin { name: "PP3", index: Some(3), description: None, altfns: [AltFn { index: 1, signal: "U1CTS", description: None }, AltFn { index: 2, signal: "U0DCD", description: None }, AltFn { index: 7, signal: "RTCCLK", description: None }, AltFn { index: 14, signal: "USB0DIR", description: None }, AltFn { index: 15, signal: "EPI0S30", description: None }] }, Pin { name: "PP4", index: Some(4), description: None, altfns: [AltFn { index: 1, signal: "U3RTS", description: None }, AltFn { index: 2, signal: "U0DSR", description: None }, AltFn { index: 14, signal: "USB0D7", description: None }] }, Pin { name: "PP5", index: Some(5), description: None, altfns: [AltFn { index: 1, signal: "U3CTS", description: None }, AltFn { index: 2, signal: "I2C2SCL", description: None }, AltFn { index: 14, signal: "USB0D6", description: None }] }], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "GPIOQ", address: 1074159616, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "GPIOQ0", types: [], value: 84, description: None }, Interrupt { name: "GPIOQ1", types: [], value: 85, description: None }, Interrupt { name: "GPIOQ2", types: [], value: 86, description: None }, Interrupt { name: "GPIOQ3", types: [], value: 87, description: None }, Interrupt { name: "GPIOQ4", types: [], value: 88, description: None }, Interrupt { name: "GPIOQ5", types: [], value: 89, description: None }, Interrupt { name: "GPIOQ6", types: [], value: 90, description: None }, Interrupt { name: "GPIOQ7", types: [], value: 91, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [Pin { name: "PQ0", index: Some(0), description: None, altfns: [AltFn { index: 14, signal: "SSI3Clk", description: None }, AltFn { index: 15, signal: "EPI0S20", description: None }] }, Pin { name: "PQ1", index: Some(1), description: None, altfns: [AltFn { index: 14, signal: "SSI3Fss", description: None }, AltFn { index: 15, signal: "EPI0S21", description: None }] }, Pin { name: "PQ2", index: Some(2), description: None, altfns: [AltFn { index: 14, signal: "SSI3XDAT0", description: None }, AltFn { index: 15, signal: "EPI0S22", description: None }] }, Pin { name: "PQ3", index: Some(3), description: None, altfns: [AltFn { index: 14, signal: "SSI3XDAT1", description: None }, AltFn { index: 15, signal: "EPI0S23", description: None }] }, Pin { name: "PQ4", index: Some(4), description: None, altfns: [AltFn { index: 1, signal: "U1Rx", description: None }, AltFn { index: 7, signal: "DIVSCLK", description: None }] }], channels: [], dim: None, dim_increment: None, dim_index: None }], prototype: Some(Peripheral { derived_from: None, group_name: Some("GPIO"), name: "", address: 0, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [], clusters: [], registers: [Register { name: "DATA", offset: 1020, size: None, access: None, reset_value: None, reset_mask: None, description: Some("GPIO Data"), fields: [Field { name: "DATA", bit_offset: 0, bit_width: 1, access: None, description: Some("GPIO Data"), enumerated_values: [], links: [], dim: Some(8), dim_increment: Some(1), dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "DIR", offset: 1024, size: None, access: None, reset_value: None, reset_mask: None, description: Some("GPIO Direction"), fields: [Field { name: "DIR", bit_offset: 0, bit_width: 1, access: None, description: Some("GPIO Direction"), enumerated_values: [], links: [], dim: Some(8), dim_increment: Some(1), dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "IS", offset: 1028, size: None, access: None, reset_value: None, reset_mask: None, description: Some("GPIO Interrupt Sense"), fields: [Field { name: "IS", bit_offset: 0, bit_width: 1, access: None, description: Some("GPIO Interrupt Sense"), enumerated_values: [], links: [], dim: Some(8), dim_increment: Some(1), dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "IBE", offset: 1032, size: None, access: None, reset_value: None, reset_mask: None, description: Some("GPIO Interrupt Both Edges"), fields: [Field { name: "IBE", bit_offset: 0, bit_width: 1, access: None, description: Some("GPIO Interrupt Both Edges"), enumerated_values: [], links: [], dim: Some(8), dim_increment: Some(1), dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "IEV", offset: 1036, size: None, access: None, reset_value: None, reset_mask: None, description: Some("GPIO Interrupt Event"), fields: [Field { name: "IEV", bit_offset: 0, bit_width: 1, access: None, description: Some("GPIO Interrupt Event"), enumerated_values: [], links: [], dim: Some(8), dim_increment: Some(1), dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "IM", offset: 1040, size: None, access: None, reset_value: None, reset_mask: None, description: Some("GPIO Interrupt Mask"), fields: [Field { name: "IM", bit_offset: 0, bit_width: 1, access: None, description: Some("GPIO Interrupt Mask"), enumerated_values: [], links: [], dim: Some(8), dim_increment: Some(1), dim_index: None }, Field { name: "DMAIME", bit_offset: 8, bit_width: 1, access: None, description: Some("GPIO uDMA Done Interrupt Mask Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "RIS", offset: 1044, size: None, access: None, reset_value: None, reset_mask: None, description: Some("GPIO Raw Interrupt Status"), fields: [Field { name: "RIS", bit_offset: 0, bit_width: 1, access: None, description: Some("GPIO Raw Interrupt Status"), enumerated_values: [], links: [], dim: Some(8), dim_increment: Some(1), dim_index: None }, Field { name: "DMARIS", bit_offset: 8, bit_width: 1, access: None, description: Some("GPIO uDMA Done Interrupt Raw Status"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "MIS", offset: 1048, size: None, access: None, reset_value: None, reset_mask: None, description: Some("GPIO Masked Interrupt Status"), fields: [Field { name: "MIS", bit_offset: 0, bit_width: 1, access: None, description: Some("GPIO Masked Interrupt Status"), enumerated_values: [], links: [], dim: Some(8), dim_increment: Some(1), dim_index: None }, Field { name: "DMAMIS", bit_offset: 8, bit_width: 1, access: None, description: Some("GPIO uDMA Done Interrupt Masked Status"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "ICR", offset: 1052, size: None, access: Some(WriteOnly), reset_value: None, reset_mask: None, description: Some("GPIO Interrupt Clear"), fields: [Field { name: "ICR", bit_offset: 0, bit_width: 1, access: None, description: Some("GPIO Interrupt Clear"), enumerated_values: [], links: [], dim: Some(8), dim_increment: Some(1), dim_index: None }, Field { name: "DMAMIC", bit_offset: 8, bit_width: 1, access: None, description: Some("GPIO uDMA Done Interrupt Clear"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "AFSEL", offset: 1056, size: None, access: None, reset_value: None, reset_mask: None, description: Some("GPIO Alternate Function Select"), fields: [Field { name: "AFSEL", bit_offset: 0, bit_width: 1, access: None, description: Some("GPIO Alternate Function Select"), enumerated_values: [], links: [], dim: Some(8), dim_increment: Some(1), dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "DR2R", offset: 1280, size: None, access: None, reset_value: None, reset_mask: None, description: Some("GPIO 2-mA Drive Select"), fields: [Field { name: "DRV2", bit_offset: 0, bit_width: 1, access: None, description: Some("GPIO 2-mA Drive Enable"), enumerated_values: [], links: [], dim: Some(8), dim_increment: Some(1), dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "DR4R", offset: 1284, size: None, access: None, reset_value: None, reset_mask: None, description: Some("GPIO 4-mA Drive Select"), fields: [Field { name: "DRV4", bit_offset: 0, bit_width: 1, access: None, description: Some("GPIO 4-mA Drive Enable"), enumerated_values: [], links: [], dim: Some(8), dim_increment: Some(1), dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "DR8R", offset: 1288, size: None, access: None, reset_value: None, reset_mask: None, description: Some("GPIO 8-mA Drive Select"), fields: [Field { name: "DRV8", bit_offset: 0, bit_width: 1, access: None, description: Some("GPIO 8-mA Drive Enable"), enumerated_values: [], links: [], dim: Some(8), dim_increment: Some(1), dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "ODR", offset: 1292, size: None, access: None, reset_value: None, reset_mask: None, description: Some("GPIO Open Drain Select"), fields: [Field { name: "ODE", bit_offset: 0, bit_width: 1, access: None, description: Some("GPIO Open Drain Enable"), enumerated_values: [], links: [], dim: Some(8), dim_increment: Some(1), dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "PUR", offset: 1296, size: None, access: None, reset_value: None, reset_mask: None, description: Some("GPIO Pull-Up Select"), fields: [Field { name: "PUE", bit_offset: 0, bit_width: 1, access: None, description: Some("GPIO Pull-Up Enable"), enumerated_values: [], links: [], dim: Some(8), dim_increment: Some(1), dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "PDR", offset: 1300, size: None, access: None, reset_value: None, reset_mask: None, description: Some("GPIO Pull-Down Select"), fields: [Field { name: "PDE", bit_offset: 0, bit_width: 1, access: None, description: Some("GPIO Pull-Down Enable"), enumerated_values: [], links: [], dim: Some(8), dim_increment: Some(1), dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "SLR", offset: 1304, size: None, access: None, reset_value: None, reset_mask: None, description: Some("GPIO Slew Rate Control Select"), fields: [Field { name: "SLR", bit_offset: 0, bit_width: 1, access: None, description: Some("GPIO Slew Rate Limit Enable"), enumerated_values: [], links: [], dim: Some(8), dim_increment: Some(1), dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "DEN", offset: 1308, size: None, access: None, reset_value: None, reset_mask: None, description: Some("GPIO Digital Enable"), fields: [Field { name: "DEN", bit_offset: 0, bit_width: 1, access: None, description: Some("GPIO Digital Enable"), enumerated_values: [], links: [], dim: Some(8), dim_increment: Some(1), dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "LOCK", offset: 1312, size: None, access: None, reset_value: None, reset_mask: None, description: Some("GPIO Lock"), fields: [Field { name: "GPIO_LOCK", bit_offset: 0, bit_width: 32, access: None, description: Some("GPIO Lock"), enumerated_values: [EnumeratedValue { value: "0x0", name: Some("GPIO_LOCK_UNLOCKED"), description: Some("The GPIOCR register is unlocked and may be modified") }, EnumeratedValue { value: "0x1", name: Some("GPIO_LOCK_LOCKED"), description: Some("The GPIOCR register is locked and may not be modified") }, EnumeratedValue { value: "0x4c4f434b", name: Some("GPIO_LOCK_KEY"), description: Some("Unlocks the GPIO_CR register") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "CR", offset: 1316, size: None, access: Some(ReadOnly), reset_value: None, reset_mask: None, description: Some("GPIO Commit"), fields: [Field { name: "CR", bit_offset: 0, bit_width: 1, access: None, description: Some("GPIO Commit"), enumerated_values: [], links: [], dim: Some(8), dim_increment: Some(1), dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "AMSEL", offset: 1320, size: None, access: None, reset_value: None, reset_mask: None, description: Some("GPIO Analog Mode Select"), fields: [Field { name: "GPIOAMSEL", bit_offset: 0, bit_width: 1, access: None, description: Some("GPIO Analog Mode Select"), enumerated_values: [], links: [], dim: Some(8), dim_increment: Some(1), dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "PCTL", offset: 1324, size: None, access: None, reset_value: None, reset_mask: None, description: Some("GPIO Port Control"), fields: [Field { name: "PMC", bit_offset: 0, bit_width: 4, access: None, description: Some("GPIO Port Mux Control"), enumerated_values: [], links: [], dim: Some(8), dim_increment: Some(4), dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "ADCCTL", offset: 1328, size: None, access: None, reset_value: None, reset_mask: None, description: Some("GPIO ADC Control"), fields: [Field { name: "ADCEN", bit_offset: 0, bit_width: 1, access: None, description: Some("ADC Trigger Enable"), enumerated_values: [], links: [], dim: Some(8), dim_increment: Some(1), dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "DMACTL", offset: 1332, size: None, access: None, reset_value: None, reset_mask: None, description: Some("GPIO DMA Control"), fields: [Field { name: "DMAEN", bit_offset: 0, bit_width: 1, access: None, description: Some("uDMA Trigger Enable"), enumerated_values: [], links: [], dim: Some(8), dim_increment: Some(1), dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "GPIOSI", offset: 1337, size: None, access: None, reset_value: None, reset_mask: None, description: Some("GPIO Select Interrupt"), fields: [Field { name: "SUM", bit_offset: 0, bit_width: 1, access: None, description: Some("Summary Interrupt"), enumerated_values: [], links: [], dim: Some(8), dim_increment: Some(1), dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "GPIODR12R", offset: 1340, size: None, access: None, reset_value: None, reset_mask: None, description: Some("GPIO 12-mA Drive Select"), fields: [Field { name: "DRV12", bit_offset: 0, bit_width: 1, access: None, description: Some("12-mA Drive Enable"), enumerated_values: [], links: [], dim: Some(8), dim_increment: Some(1), dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "GPIOWAKEPEN", offset: 1344, size: None, access: None, reset_value: None, reset_mask: None, description: Some("GPIO Wake Pin Enable"), fields: [Field { name: "WAKEP7", bit_offset: 7, bit_width: 1, access: None, description: Some("K[7] Wake Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "WAKEP6", bit_offset: 6, bit_width: 1, access: None, description: Some("K[6] Wake Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "WAKEP5", bit_offset: 5, bit_width: 1, access: None, description: Some("K[5] Wake Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "WAKEP4", bit_offset: 4, bit_width: 1, access: None, description: Some("K[4] Wake Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "GPIOWAKELVL", offset: 1348, size: None, access: None, reset_value: None, reset_mask: None, description: Some("GPIO Wake Level"), fields: [Field { name: "WAKELVL7", bit_offset: 7, bit_width: 1, access: None, description: Some("K[7] Wake Level"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "WAKELVL6", bit_offset: 6, bit_width: 1, access: None, description: Some("K[6] Wake Level"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "WAKELVL5", bit_offset: 5, bit_width: 1, access: None, description: Some("K[5] Wake Level"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "WAKELVL4", bit_offset: 4, bit_width: 1, access: None, description: Some("K[4] Wake Level"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "GPIOWAKESTAT", offset: 1352, size: None, access: None, reset_value: None, reset_mask: None, description: Some("GPIO Wake Level"), fields: [Field { name: "WAKESTAT7", bit_offset: 7, bit_width: 1, access: None, description: Some("K[7] Wake Status"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "WAKESTAT6", bit_offset: 6, bit_width: 1, access: None, description: Some("K[6] Wake Status"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "WAKESTAT5", bit_offset: 5, bit_width: 1, access: None, description: Some("K[5] Wake Status"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "WAKESTAT4", bit_offset: 4, bit_width: 1, access: None, description: Some("K[4] Wake Status"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }), modules: [], has_pins: true, has_channels: false, description: None }
periph!( GPIOA, Gpioa, _GPIOA, GpioPeriph, 0x40004000);
periph!( GPIOB, Gpiob, _GPIOB, GpioPeriph, 0x40005000);
periph!( GPIOC, Gpioc, _GPIOC, GpioPeriph, 0x40006000);
periph!( GPIOD, Gpiod, _GPIOD, GpioPeriph, 0x40007000);
periph!( GPIOE, Gpioe, _GPIOE, GpioPeriph, 0x40024000);
periph!( GPIOF, Gpiof, _GPIOF, GpioPeriph, 0x40025000);
periph!( GPIOG, Gpiog, _GPIOG, GpioPeriph, 0x40026000);
periph!( GPIOH, Gpioh, _GPIOH, GpioPeriph, 0x40027000);
periph!( GPIOJ, Gpioj, _GPIOJ, GpioPeriph, 0x4003d000);
periph!( GPIOA_AHB, GpioaAhb, _GPIOA_AHB, GpioPeriph, 0x40058000);
periph!( GPIOB_AHB, GpiobAhb, _GPIOB_AHB, GpioPeriph, 0x40059000);
periph!( GPIOC_AHB, GpiocAhb, _GPIOC_AHB, GpioPeriph, 0x4005a000);
periph!( GPIOD_AHB, GpiodAhb, _GPIOD_AHB, GpioPeriph, 0x4005b000);
periph!( GPIOE_AHB, GpioeAhb, _GPIOE_AHB, GpioPeriph, 0x4005c000);
periph!( GPIOF_AHB, GpiofAhb, _GPIOF_AHB, GpioPeriph, 0x4005d000);
periph!( GPIOG_AHB, GpiogAhb, _GPIOG_AHB, GpioPeriph, 0x4005e000);
periph!( GPIOH_AHB, GpiohAhb, _GPIOH_AHB, GpioPeriph, 0x4005f000);
periph!( GPIOJ_AHB, GpiojAhb, _GPIOJ_AHB, GpioPeriph, 0x40060000);
periph!( GPIOK, Gpiok, _GPIOK, GpioPeriph, 0x40061000);
periph!( GPIOL, Gpiol, _GPIOL, GpioPeriph, 0x40062000);
periph!( GPIOM, Gpiom, _GPIOM, GpioPeriph, 0x40063000);
periph!( GPION, Gpion, _GPION, GpioPeriph, 0x40064000);
periph!( GPIOP, Gpiop, _GPIOP, GpioPeriph, 0x40065000);
periph!( GPIOQ, Gpioq, _GPIOQ, GpioPeriph, 0x40066000);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="GPIO Peripheral"]
pub struct GpioPeriph(pub usize); 


























impl GpioPeriph {
#[doc="Get the *const pointer for the DATA register."]
   #[inline] pub fn data_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x3fc) as *const u32
   }
#[doc="Get the *mut pointer for the DATA register."]
   #[inline] pub fn data_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x3fc) as *mut u32
   }
#[doc="Read the DATA register."]
   #[inline] pub fn data(&self) -> Data { 
      unsafe {
         Data(::core::ptr::read_volatile((self.0 + 0x3fc) as *const u32))
      }
   }
#[doc="Write the DATA register."]
   #[inline] pub fn set_data<F: FnOnce(Data) -> Data>(&self, f: F) -> &Self {
      let value = f(Data(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x3fc) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DATA register."]
   #[inline] pub fn with_data<F: FnOnce(Data) -> Data>(&self, f: F) -> &Self {
      let tmp = self.data();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x3fc) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DIR register."]
   #[inline] pub fn dir_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x400) as *const u32
   }
#[doc="Get the *mut pointer for the DIR register."]
   #[inline] pub fn dir_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x400) as *mut u32
   }
#[doc="Read the DIR register."]
   #[inline] pub fn dir(&self) -> Dir { 
      unsafe {
         Dir(::core::ptr::read_volatile((self.0 + 0x400) as *const u32))
      }
   }
#[doc="Write the DIR register."]
   #[inline] pub fn set_dir<F: FnOnce(Dir) -> Dir>(&self, f: F) -> &Self {
      let value = f(Dir(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x400) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DIR register."]
   #[inline] pub fn with_dir<F: FnOnce(Dir) -> Dir>(&self, f: F) -> &Self {
      let tmp = self.dir();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x400) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the IS register."]
   #[inline] pub fn is_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x404) as *const u32
   }
#[doc="Get the *mut pointer for the IS register."]
   #[inline] pub fn is_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x404) as *mut u32
   }
#[doc="Read the IS register."]
   #[inline] pub fn is(&self) -> Is { 
      unsafe {
         Is(::core::ptr::read_volatile((self.0 + 0x404) as *const u32))
      }
   }
#[doc="Write the IS register."]
   #[inline] pub fn set_is<F: FnOnce(Is) -> Is>(&self, f: F) -> &Self {
      let value = f(Is(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x404) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the IS register."]
   #[inline] pub fn with_is<F: FnOnce(Is) -> Is>(&self, f: F) -> &Self {
      let tmp = self.is();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x404) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the IBE register."]
   #[inline] pub fn ibe_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x408) as *const u32
   }
#[doc="Get the *mut pointer for the IBE register."]
   #[inline] pub fn ibe_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x408) as *mut u32
   }
#[doc="Read the IBE register."]
   #[inline] pub fn ibe(&self) -> Ibe { 
      unsafe {
         Ibe(::core::ptr::read_volatile((self.0 + 0x408) as *const u32))
      }
   }
#[doc="Write the IBE register."]
   #[inline] pub fn set_ibe<F: FnOnce(Ibe) -> Ibe>(&self, f: F) -> &Self {
      let value = f(Ibe(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x408) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the IBE register."]
   #[inline] pub fn with_ibe<F: FnOnce(Ibe) -> Ibe>(&self, f: F) -> &Self {
      let tmp = self.ibe();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x408) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the IEV register."]
   #[inline] pub fn iev_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x40c) as *const u32
   }
#[doc="Get the *mut pointer for the IEV register."]
   #[inline] pub fn iev_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x40c) as *mut u32
   }
#[doc="Read the IEV register."]
   #[inline] pub fn iev(&self) -> Iev { 
      unsafe {
         Iev(::core::ptr::read_volatile((self.0 + 0x40c) as *const u32))
      }
   }
#[doc="Write the IEV register."]
   #[inline] pub fn set_iev<F: FnOnce(Iev) -> Iev>(&self, f: F) -> &Self {
      let value = f(Iev(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x40c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the IEV register."]
   #[inline] pub fn with_iev<F: FnOnce(Iev) -> Iev>(&self, f: F) -> &Self {
      let tmp = self.iev();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x40c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the IM register."]
   #[inline] pub fn im_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x410) as *const u32
   }
#[doc="Get the *mut pointer for the IM register."]
   #[inline] pub fn im_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x410) as *mut u32
   }
#[doc="Read the IM register."]
   #[inline] pub fn im(&self) -> Im { 
      unsafe {
         Im(::core::ptr::read_volatile((self.0 + 0x410) as *const u32))
      }
   }
#[doc="Write the IM register."]
   #[inline] pub fn set_im<F: FnOnce(Im) -> Im>(&self, f: F) -> &Self {
      let value = f(Im(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x410) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the IM register."]
   #[inline] pub fn with_im<F: FnOnce(Im) -> Im>(&self, f: F) -> &Self {
      let tmp = self.im();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x410) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the RIS register."]
   #[inline] pub fn ris_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x414) as *const u32
   }
#[doc="Get the *mut pointer for the RIS register."]
   #[inline] pub fn ris_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x414) as *mut u32
   }
#[doc="Read the RIS register."]
   #[inline] pub fn ris(&self) -> Ris { 
      unsafe {
         Ris(::core::ptr::read_volatile((self.0 + 0x414) as *const u32))
      }
   }
#[doc="Write the RIS register."]
   #[inline] pub fn set_ris<F: FnOnce(Ris) -> Ris>(&self, f: F) -> &Self {
      let value = f(Ris(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x414) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the RIS register."]
   #[inline] pub fn with_ris<F: FnOnce(Ris) -> Ris>(&self, f: F) -> &Self {
      let tmp = self.ris();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x414) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the MIS register."]
   #[inline] pub fn mis_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x418) as *const u32
   }
#[doc="Get the *mut pointer for the MIS register."]
   #[inline] pub fn mis_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x418) as *mut u32
   }
#[doc="Read the MIS register."]
   #[inline] pub fn mis(&self) -> Mis { 
      unsafe {
         Mis(::core::ptr::read_volatile((self.0 + 0x418) as *const u32))
      }
   }
#[doc="Write the MIS register."]
   #[inline] pub fn set_mis<F: FnOnce(Mis) -> Mis>(&self, f: F) -> &Self {
      let value = f(Mis(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x418) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the MIS register."]
   #[inline] pub fn with_mis<F: FnOnce(Mis) -> Mis>(&self, f: F) -> &Self {
      let tmp = self.mis();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x418) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the ICR register."]
   #[inline] pub fn icr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x41c) as *const u32
   }
#[doc="Get the *mut pointer for the ICR register."]
   #[inline] pub fn icr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x41c) as *mut u32
   }
#[doc="Write the ICR register."]
   #[inline] pub fn set_icr<F: FnOnce(Icr) -> Icr>(&self, f: F) -> &Self {
      let value = f(Icr(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x41c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the AFSEL register."]
   #[inline] pub fn afsel_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x420) as *const u32
   }
#[doc="Get the *mut pointer for the AFSEL register."]
   #[inline] pub fn afsel_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x420) as *mut u32
   }
#[doc="Read the AFSEL register."]
   #[inline] pub fn afsel(&self) -> Afsel { 
      unsafe {
         Afsel(::core::ptr::read_volatile((self.0 + 0x420) as *const u32))
      }
   }
#[doc="Write the AFSEL register."]
   #[inline] pub fn set_afsel<F: FnOnce(Afsel) -> Afsel>(&self, f: F) -> &Self {
      let value = f(Afsel(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x420) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the AFSEL register."]
   #[inline] pub fn with_afsel<F: FnOnce(Afsel) -> Afsel>(&self, f: F) -> &Self {
      let tmp = self.afsel();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x420) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DR2R register."]
   #[inline] pub fn dr2r_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x500) as *const u32
   }
#[doc="Get the *mut pointer for the DR2R register."]
   #[inline] pub fn dr2r_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x500) as *mut u32
   }
#[doc="Read the DR2R register."]
   #[inline] pub fn dr2r(&self) -> Dr2r { 
      unsafe {
         Dr2r(::core::ptr::read_volatile((self.0 + 0x500) as *const u32))
      }
   }
#[doc="Write the DR2R register."]
   #[inline] pub fn set_dr2r<F: FnOnce(Dr2r) -> Dr2r>(&self, f: F) -> &Self {
      let value = f(Dr2r(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x500) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DR2R register."]
   #[inline] pub fn with_dr2r<F: FnOnce(Dr2r) -> Dr2r>(&self, f: F) -> &Self {
      let tmp = self.dr2r();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x500) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DR4R register."]
   #[inline] pub fn dr4r_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x504) as *const u32
   }
#[doc="Get the *mut pointer for the DR4R register."]
   #[inline] pub fn dr4r_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x504) as *mut u32
   }
#[doc="Read the DR4R register."]
   #[inline] pub fn dr4r(&self) -> Dr4r { 
      unsafe {
         Dr4r(::core::ptr::read_volatile((self.0 + 0x504) as *const u32))
      }
   }
#[doc="Write the DR4R register."]
   #[inline] pub fn set_dr4r<F: FnOnce(Dr4r) -> Dr4r>(&self, f: F) -> &Self {
      let value = f(Dr4r(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x504) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DR4R register."]
   #[inline] pub fn with_dr4r<F: FnOnce(Dr4r) -> Dr4r>(&self, f: F) -> &Self {
      let tmp = self.dr4r();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x504) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DR8R register."]
   #[inline] pub fn dr8r_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x508) as *const u32
   }
#[doc="Get the *mut pointer for the DR8R register."]
   #[inline] pub fn dr8r_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x508) as *mut u32
   }
#[doc="Read the DR8R register."]
   #[inline] pub fn dr8r(&self) -> Dr8r { 
      unsafe {
         Dr8r(::core::ptr::read_volatile((self.0 + 0x508) as *const u32))
      }
   }
#[doc="Write the DR8R register."]
   #[inline] pub fn set_dr8r<F: FnOnce(Dr8r) -> Dr8r>(&self, f: F) -> &Self {
      let value = f(Dr8r(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x508) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DR8R register."]
   #[inline] pub fn with_dr8r<F: FnOnce(Dr8r) -> Dr8r>(&self, f: F) -> &Self {
      let tmp = self.dr8r();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x508) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the ODR register."]
   #[inline] pub fn odr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x50c) as *const u32
   }
#[doc="Get the *mut pointer for the ODR register."]
   #[inline] pub fn odr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x50c) as *mut u32
   }
#[doc="Read the ODR register."]
   #[inline] pub fn odr(&self) -> Odr { 
      unsafe {
         Odr(::core::ptr::read_volatile((self.0 + 0x50c) as *const u32))
      }
   }
#[doc="Write the ODR register."]
   #[inline] pub fn set_odr<F: FnOnce(Odr) -> Odr>(&self, f: F) -> &Self {
      let value = f(Odr(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x50c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the ODR register."]
   #[inline] pub fn with_odr<F: FnOnce(Odr) -> Odr>(&self, f: F) -> &Self {
      let tmp = self.odr();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x50c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PUR register."]
   #[inline] pub fn pur_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x510) as *const u32
   }
#[doc="Get the *mut pointer for the PUR register."]
   #[inline] pub fn pur_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x510) as *mut u32
   }
#[doc="Read the PUR register."]
   #[inline] pub fn pur(&self) -> Pur { 
      unsafe {
         Pur(::core::ptr::read_volatile((self.0 + 0x510) as *const u32))
      }
   }
#[doc="Write the PUR register."]
   #[inline] pub fn set_pur<F: FnOnce(Pur) -> Pur>(&self, f: F) -> &Self {
      let value = f(Pur(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x510) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PUR register."]
   #[inline] pub fn with_pur<F: FnOnce(Pur) -> Pur>(&self, f: F) -> &Self {
      let tmp = self.pur();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x510) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PDR register."]
   #[inline] pub fn pdr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x514) as *const u32
   }
#[doc="Get the *mut pointer for the PDR register."]
   #[inline] pub fn pdr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x514) as *mut u32
   }
#[doc="Read the PDR register."]
   #[inline] pub fn pdr(&self) -> Pdr { 
      unsafe {
         Pdr(::core::ptr::read_volatile((self.0 + 0x514) as *const u32))
      }
   }
#[doc="Write the PDR register."]
   #[inline] pub fn set_pdr<F: FnOnce(Pdr) -> Pdr>(&self, f: F) -> &Self {
      let value = f(Pdr(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x514) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PDR register."]
   #[inline] pub fn with_pdr<F: FnOnce(Pdr) -> Pdr>(&self, f: F) -> &Self {
      let tmp = self.pdr();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x514) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SLR register."]
   #[inline] pub fn slr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x518) as *const u32
   }
#[doc="Get the *mut pointer for the SLR register."]
   #[inline] pub fn slr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x518) as *mut u32
   }
#[doc="Read the SLR register."]
   #[inline] pub fn slr(&self) -> Slr { 
      unsafe {
         Slr(::core::ptr::read_volatile((self.0 + 0x518) as *const u32))
      }
   }
#[doc="Write the SLR register."]
   #[inline] pub fn set_slr<F: FnOnce(Slr) -> Slr>(&self, f: F) -> &Self {
      let value = f(Slr(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x518) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SLR register."]
   #[inline] pub fn with_slr<F: FnOnce(Slr) -> Slr>(&self, f: F) -> &Self {
      let tmp = self.slr();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x518) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DEN register."]
   #[inline] pub fn den_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x51c) as *const u32
   }
#[doc="Get the *mut pointer for the DEN register."]
   #[inline] pub fn den_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x51c) as *mut u32
   }
#[doc="Read the DEN register."]
   #[inline] pub fn den(&self) -> Den { 
      unsafe {
         Den(::core::ptr::read_volatile((self.0 + 0x51c) as *const u32))
      }
   }
#[doc="Write the DEN register."]
   #[inline] pub fn set_den<F: FnOnce(Den) -> Den>(&self, f: F) -> &Self {
      let value = f(Den(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x51c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DEN register."]
   #[inline] pub fn with_den<F: FnOnce(Den) -> Den>(&self, f: F) -> &Self {
      let tmp = self.den();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x51c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the LOCK register."]
   #[inline] pub fn lock_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x520) as *const u32
   }
#[doc="Get the *mut pointer for the LOCK register."]
   #[inline] pub fn lock_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x520) as *mut u32
   }
#[doc="Read the LOCK register."]
   #[inline] pub fn lock(&self) -> Lock { 
      unsafe {
         Lock(::core::ptr::read_volatile((self.0 + 0x520) as *const u32))
      }
   }
#[doc="Write the LOCK register."]
   #[inline] pub fn set_lock<F: FnOnce(Lock) -> Lock>(&self, f: F) -> &Self {
      let value = f(Lock(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x520) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the LOCK register."]
   #[inline] pub fn with_lock<F: FnOnce(Lock) -> Lock>(&self, f: F) -> &Self {
      let tmp = self.lock();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x520) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CR register."]
   #[inline] pub fn cr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x524) as *const u32
   }
#[doc="Get the *mut pointer for the CR register."]
   #[inline] pub fn cr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x524) as *mut u32
   }
#[doc="Read the CR register."]
   #[inline] pub fn cr(&self) -> Cr { 
      unsafe {
         Cr(::core::ptr::read_volatile((self.0 + 0x524) as *const u32))
      }
   }

#[doc="Get the *const pointer for the AMSEL register."]
   #[inline] pub fn amsel_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x528) as *const u32
   }
#[doc="Get the *mut pointer for the AMSEL register."]
   #[inline] pub fn amsel_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x528) as *mut u32
   }
#[doc="Read the AMSEL register."]
   #[inline] pub fn amsel(&self) -> Amsel { 
      unsafe {
         Amsel(::core::ptr::read_volatile((self.0 + 0x528) as *const u32))
      }
   }
#[doc="Write the AMSEL register."]
   #[inline] pub fn set_amsel<F: FnOnce(Amsel) -> Amsel>(&self, f: F) -> &Self {
      let value = f(Amsel(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x528) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the AMSEL register."]
   #[inline] pub fn with_amsel<F: FnOnce(Amsel) -> Amsel>(&self, f: F) -> &Self {
      let tmp = self.amsel();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x528) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PCTL register."]
   #[inline] pub fn pctl_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x52c) as *const u32
   }
#[doc="Get the *mut pointer for the PCTL register."]
   #[inline] pub fn pctl_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x52c) as *mut u32
   }
#[doc="Read the PCTL register."]
   #[inline] pub fn pctl(&self) -> Pctl { 
      unsafe {
         Pctl(::core::ptr::read_volatile((self.0 + 0x52c) as *const u32))
      }
   }
#[doc="Write the PCTL register."]
   #[inline] pub fn set_pctl<F: FnOnce(Pctl) -> Pctl>(&self, f: F) -> &Self {
      let value = f(Pctl(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x52c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCTL register."]
   #[inline] pub fn with_pctl<F: FnOnce(Pctl) -> Pctl>(&self, f: F) -> &Self {
      let tmp = self.pctl();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x52c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the ADCCTL register."]
   #[inline] pub fn adcctl_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x530) as *const u32
   }
#[doc="Get the *mut pointer for the ADCCTL register."]
   #[inline] pub fn adcctl_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x530) as *mut u32
   }
#[doc="Read the ADCCTL register."]
   #[inline] pub fn adcctl(&self) -> Adcctl { 
      unsafe {
         Adcctl(::core::ptr::read_volatile((self.0 + 0x530) as *const u32))
      }
   }
#[doc="Write the ADCCTL register."]
   #[inline] pub fn set_adcctl<F: FnOnce(Adcctl) -> Adcctl>(&self, f: F) -> &Self {
      let value = f(Adcctl(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x530) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the ADCCTL register."]
   #[inline] pub fn with_adcctl<F: FnOnce(Adcctl) -> Adcctl>(&self, f: F) -> &Self {
      let tmp = self.adcctl();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x530) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DMACTL register."]
   #[inline] pub fn dmactl_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x534) as *const u32
   }
#[doc="Get the *mut pointer for the DMACTL register."]
   #[inline] pub fn dmactl_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x534) as *mut u32
   }
#[doc="Read the DMACTL register."]
   #[inline] pub fn dmactl(&self) -> Dmactl { 
      unsafe {
         Dmactl(::core::ptr::read_volatile((self.0 + 0x534) as *const u32))
      }
   }
#[doc="Write the DMACTL register."]
   #[inline] pub fn set_dmactl<F: FnOnce(Dmactl) -> Dmactl>(&self, f: F) -> &Self {
      let value = f(Dmactl(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x534) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DMACTL register."]
   #[inline] pub fn with_dmactl<F: FnOnce(Dmactl) -> Dmactl>(&self, f: F) -> &Self {
      let tmp = self.dmactl();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x534) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the GPIOSI register."]
   #[inline] pub fn gpiosi_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x539) as *const u32
   }
#[doc="Get the *mut pointer for the GPIOSI register."]
   #[inline] pub fn gpiosi_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x539) as *mut u32
   }
#[doc="Read the GPIOSI register."]
   #[inline] pub fn gpiosi(&self) -> Gpiosi { 
      unsafe {
         Gpiosi(::core::ptr::read_volatile((self.0 + 0x539) as *const u32))
      }
   }
#[doc="Write the GPIOSI register."]
   #[inline] pub fn set_gpiosi<F: FnOnce(Gpiosi) -> Gpiosi>(&self, f: F) -> &Self {
      let value = f(Gpiosi(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x539) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the GPIOSI register."]
   #[inline] pub fn with_gpiosi<F: FnOnce(Gpiosi) -> Gpiosi>(&self, f: F) -> &Self {
      let tmp = self.gpiosi();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x539) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the GPIODR12R register."]
   #[inline] pub fn gpiodr12r_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x53c) as *const u32
   }
#[doc="Get the *mut pointer for the GPIODR12R register."]
   #[inline] pub fn gpiodr12r_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x53c) as *mut u32
   }
#[doc="Read the GPIODR12R register."]
   #[inline] pub fn gpiodr12r(&self) -> Gpiodr12r { 
      unsafe {
         Gpiodr12r(::core::ptr::read_volatile((self.0 + 0x53c) as *const u32))
      }
   }
#[doc="Write the GPIODR12R register."]
   #[inline] pub fn set_gpiodr12r<F: FnOnce(Gpiodr12r) -> Gpiodr12r>(&self, f: F) -> &Self {
      let value = f(Gpiodr12r(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x53c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the GPIODR12R register."]
   #[inline] pub fn with_gpiodr12r<F: FnOnce(Gpiodr12r) -> Gpiodr12r>(&self, f: F) -> &Self {
      let tmp = self.gpiodr12r();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x53c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the GPIOWAKEPEN register."]
   #[inline] pub fn gpiowakepen_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x540) as *const u32
   }
#[doc="Get the *mut pointer for the GPIOWAKEPEN register."]
   #[inline] pub fn gpiowakepen_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x540) as *mut u32
   }
#[doc="Read the GPIOWAKEPEN register."]
   #[inline] pub fn gpiowakepen(&self) -> Gpiowakepen { 
      unsafe {
         Gpiowakepen(::core::ptr::read_volatile((self.0 + 0x540) as *const u32))
      }
   }
#[doc="Write the GPIOWAKEPEN register."]
   #[inline] pub fn set_gpiowakepen<F: FnOnce(Gpiowakepen) -> Gpiowakepen>(&self, f: F) -> &Self {
      let value = f(Gpiowakepen(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x540) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the GPIOWAKEPEN register."]
   #[inline] pub fn with_gpiowakepen<F: FnOnce(Gpiowakepen) -> Gpiowakepen>(&self, f: F) -> &Self {
      let tmp = self.gpiowakepen();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x540) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the GPIOWAKELVL register."]
   #[inline] pub fn gpiowakelvl_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x544) as *const u32
   }
#[doc="Get the *mut pointer for the GPIOWAKELVL register."]
   #[inline] pub fn gpiowakelvl_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x544) as *mut u32
   }
#[doc="Read the GPIOWAKELVL register."]
   #[inline] pub fn gpiowakelvl(&self) -> Gpiowakelvl { 
      unsafe {
         Gpiowakelvl(::core::ptr::read_volatile((self.0 + 0x544) as *const u32))
      }
   }
#[doc="Write the GPIOWAKELVL register."]
   #[inline] pub fn set_gpiowakelvl<F: FnOnce(Gpiowakelvl) -> Gpiowakelvl>(&self, f: F) -> &Self {
      let value = f(Gpiowakelvl(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x544) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the GPIOWAKELVL register."]
   #[inline] pub fn with_gpiowakelvl<F: FnOnce(Gpiowakelvl) -> Gpiowakelvl>(&self, f: F) -> &Self {
      let tmp = self.gpiowakelvl();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x544) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the GPIOWAKESTAT register."]
   #[inline] pub fn gpiowakestat_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x548) as *const u32
   }
#[doc="Get the *mut pointer for the GPIOWAKESTAT register."]
   #[inline] pub fn gpiowakestat_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x548) as *mut u32
   }
#[doc="Read the GPIOWAKESTAT register."]
   #[inline] pub fn gpiowakestat(&self) -> Gpiowakestat { 
      unsafe {
         Gpiowakestat(::core::ptr::read_volatile((self.0 + 0x548) as *const u32))
      }
   }
#[doc="Write the GPIOWAKESTAT register."]
   #[inline] pub fn set_gpiowakestat<F: FnOnce(Gpiowakestat) -> Gpiowakestat>(&self, f: F) -> &Self {
      let value = f(Gpiowakestat(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x548) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the GPIOWAKESTAT register."]
   #[inline] pub fn with_gpiowakestat<F: FnOnce(Gpiowakestat) -> Gpiowakestat>(&self, f: F) -> &Self {
      let tmp = self.gpiowakestat();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x548) as *mut u32, value.0);
      }
      self
   }

}

#[doc="GPIO Data"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Data(pub u32);
impl Data {
#[doc="GPIO Data"]
   #[inline] pub fn data<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="GPIO Data"]
   #[inline] pub fn set_data<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Data {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Data {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.data(0) != 0 { try!(write!(f, " data[0]"))}
      if self.data(1) != 0 { try!(write!(f, " data[1]"))}
      if self.data(2) != 0 { try!(write!(f, " data[2]"))}
      if self.data(3) != 0 { try!(write!(f, " data[3]"))}
      if self.data(4) != 0 { try!(write!(f, " data[4]"))}
      if self.data(5) != 0 { try!(write!(f, " data[5]"))}
      if self.data(6) != 0 { try!(write!(f, " data[6]"))}
      if self.data(7) != 0 { try!(write!(f, " data[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="GPIO Direction"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dir(pub u32);
impl Dir {
#[doc="GPIO Direction"]
   #[inline] pub fn dir<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="GPIO Direction"]
   #[inline] pub fn set_dir<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Dir {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dir {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dir(0) != 0 { try!(write!(f, " dir[0]"))}
      if self.dir(1) != 0 { try!(write!(f, " dir[1]"))}
      if self.dir(2) != 0 { try!(write!(f, " dir[2]"))}
      if self.dir(3) != 0 { try!(write!(f, " dir[3]"))}
      if self.dir(4) != 0 { try!(write!(f, " dir[4]"))}
      if self.dir(5) != 0 { try!(write!(f, " dir[5]"))}
      if self.dir(6) != 0 { try!(write!(f, " dir[6]"))}
      if self.dir(7) != 0 { try!(write!(f, " dir[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="GPIO Interrupt Sense"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Is(pub u32);
impl Is {
#[doc="GPIO Interrupt Sense"]
   #[inline] pub fn is<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="GPIO Interrupt Sense"]
   #[inline] pub fn set_is<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Is {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Is {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.is(0) != 0 { try!(write!(f, " is[0]"))}
      if self.is(1) != 0 { try!(write!(f, " is[1]"))}
      if self.is(2) != 0 { try!(write!(f, " is[2]"))}
      if self.is(3) != 0 { try!(write!(f, " is[3]"))}
      if self.is(4) != 0 { try!(write!(f, " is[4]"))}
      if self.is(5) != 0 { try!(write!(f, " is[5]"))}
      if self.is(6) != 0 { try!(write!(f, " is[6]"))}
      if self.is(7) != 0 { try!(write!(f, " is[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="GPIO Interrupt Both Edges"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ibe(pub u32);
impl Ibe {
#[doc="GPIO Interrupt Both Edges"]
   #[inline] pub fn ibe<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="GPIO Interrupt Both Edges"]
   #[inline] pub fn set_ibe<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Ibe {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ibe {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ibe(0) != 0 { try!(write!(f, " ibe[0]"))}
      if self.ibe(1) != 0 { try!(write!(f, " ibe[1]"))}
      if self.ibe(2) != 0 { try!(write!(f, " ibe[2]"))}
      if self.ibe(3) != 0 { try!(write!(f, " ibe[3]"))}
      if self.ibe(4) != 0 { try!(write!(f, " ibe[4]"))}
      if self.ibe(5) != 0 { try!(write!(f, " ibe[5]"))}
      if self.ibe(6) != 0 { try!(write!(f, " ibe[6]"))}
      if self.ibe(7) != 0 { try!(write!(f, " ibe[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="GPIO Interrupt Event"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Iev(pub u32);
impl Iev {
#[doc="GPIO Interrupt Event"]
   #[inline] pub fn iev<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="GPIO Interrupt Event"]
   #[inline] pub fn set_iev<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Iev {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Iev {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.iev(0) != 0 { try!(write!(f, " iev[0]"))}
      if self.iev(1) != 0 { try!(write!(f, " iev[1]"))}
      if self.iev(2) != 0 { try!(write!(f, " iev[2]"))}
      if self.iev(3) != 0 { try!(write!(f, " iev[3]"))}
      if self.iev(4) != 0 { try!(write!(f, " iev[4]"))}
      if self.iev(5) != 0 { try!(write!(f, " iev[5]"))}
      if self.iev(6) != 0 { try!(write!(f, " iev[6]"))}
      if self.iev(7) != 0 { try!(write!(f, " iev[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="GPIO Interrupt Mask"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Im(pub u32);
impl Im {
#[doc="GPIO Interrupt Mask"]
   #[inline] pub fn im<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="GPIO Interrupt Mask"]
   #[inline] pub fn set_im<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

#[doc="GPIO uDMA Done Interrupt Mask Enable"]
   #[inline] pub fn dmaime(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="GPIO uDMA Done Interrupt Mask Enable"]
   #[inline] pub fn set_dmaime<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

}
impl ::core::fmt::Display for Im {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Im {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.im(0) != 0 { try!(write!(f, " im[0]"))}
      if self.im(1) != 0 { try!(write!(f, " im[1]"))}
      if self.im(2) != 0 { try!(write!(f, " im[2]"))}
      if self.im(3) != 0 { try!(write!(f, " im[3]"))}
      if self.im(4) != 0 { try!(write!(f, " im[4]"))}
      if self.im(5) != 0 { try!(write!(f, " im[5]"))}
      if self.im(6) != 0 { try!(write!(f, " im[6]"))}
      if self.im(7) != 0 { try!(write!(f, " im[7]"))}
      if self.dmaime() != 0 { try!(write!(f, " dmaime"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="GPIO Raw Interrupt Status"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ris(pub u32);
impl Ris {
#[doc="GPIO Raw Interrupt Status"]
   #[inline] pub fn ris<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="GPIO Raw Interrupt Status"]
   #[inline] pub fn set_ris<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

#[doc="GPIO uDMA Done Interrupt Raw Status"]
   #[inline] pub fn dmaris(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="GPIO uDMA Done Interrupt Raw Status"]
   #[inline] pub fn set_dmaris<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

}
impl ::core::fmt::Display for Ris {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ris {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ris(0) != 0 { try!(write!(f, " ris[0]"))}
      if self.ris(1) != 0 { try!(write!(f, " ris[1]"))}
      if self.ris(2) != 0 { try!(write!(f, " ris[2]"))}
      if self.ris(3) != 0 { try!(write!(f, " ris[3]"))}
      if self.ris(4) != 0 { try!(write!(f, " ris[4]"))}
      if self.ris(5) != 0 { try!(write!(f, " ris[5]"))}
      if self.ris(6) != 0 { try!(write!(f, " ris[6]"))}
      if self.ris(7) != 0 { try!(write!(f, " ris[7]"))}
      if self.dmaris() != 0 { try!(write!(f, " dmaris"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="GPIO Masked Interrupt Status"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Mis(pub u32);
impl Mis {
#[doc="GPIO Masked Interrupt Status"]
   #[inline] pub fn mis<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="GPIO Masked Interrupt Status"]
   #[inline] pub fn set_mis<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

#[doc="GPIO uDMA Done Interrupt Masked Status"]
   #[inline] pub fn dmamis(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="GPIO uDMA Done Interrupt Masked Status"]
   #[inline] pub fn set_dmamis<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

}
impl ::core::fmt::Display for Mis {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Mis {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.mis(0) != 0 { try!(write!(f, " mis[0]"))}
      if self.mis(1) != 0 { try!(write!(f, " mis[1]"))}
      if self.mis(2) != 0 { try!(write!(f, " mis[2]"))}
      if self.mis(3) != 0 { try!(write!(f, " mis[3]"))}
      if self.mis(4) != 0 { try!(write!(f, " mis[4]"))}
      if self.mis(5) != 0 { try!(write!(f, " mis[5]"))}
      if self.mis(6) != 0 { try!(write!(f, " mis[6]"))}
      if self.mis(7) != 0 { try!(write!(f, " mis[7]"))}
      if self.dmamis() != 0 { try!(write!(f, " dmamis"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="GPIO Interrupt Clear"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Icr(pub u32);
impl Icr {
#[doc="GPIO Interrupt Clear"]
   #[inline] pub fn icr<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="GPIO Interrupt Clear"]
   #[inline] pub fn set_icr<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

#[doc="GPIO uDMA Done Interrupt Clear"]
   #[inline] pub fn dmamic(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="GPIO uDMA Done Interrupt Clear"]
   #[inline] pub fn set_dmamic<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

}
impl ::core::fmt::Display for Icr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Icr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.icr(0) != 0 { try!(write!(f, " icr[0]"))}
      if self.icr(1) != 0 { try!(write!(f, " icr[1]"))}
      if self.icr(2) != 0 { try!(write!(f, " icr[2]"))}
      if self.icr(3) != 0 { try!(write!(f, " icr[3]"))}
      if self.icr(4) != 0 { try!(write!(f, " icr[4]"))}
      if self.icr(5) != 0 { try!(write!(f, " icr[5]"))}
      if self.icr(6) != 0 { try!(write!(f, " icr[6]"))}
      if self.icr(7) != 0 { try!(write!(f, " icr[7]"))}
      if self.dmamic() != 0 { try!(write!(f, " dmamic"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="GPIO Alternate Function Select"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Afsel(pub u32);
impl Afsel {
#[doc="GPIO Alternate Function Select"]
   #[inline] pub fn afsel<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="GPIO Alternate Function Select"]
   #[inline] pub fn set_afsel<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Afsel {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Afsel {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.afsel(0) != 0 { try!(write!(f, " afsel[0]"))}
      if self.afsel(1) != 0 { try!(write!(f, " afsel[1]"))}
      if self.afsel(2) != 0 { try!(write!(f, " afsel[2]"))}
      if self.afsel(3) != 0 { try!(write!(f, " afsel[3]"))}
      if self.afsel(4) != 0 { try!(write!(f, " afsel[4]"))}
      if self.afsel(5) != 0 { try!(write!(f, " afsel[5]"))}
      if self.afsel(6) != 0 { try!(write!(f, " afsel[6]"))}
      if self.afsel(7) != 0 { try!(write!(f, " afsel[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="GPIO 2-mA Drive Select"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dr2r(pub u32);
impl Dr2r {
#[doc="GPIO 2-mA Drive Enable"]
   #[inline] pub fn drv2<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="GPIO 2-mA Drive Enable"]
   #[inline] pub fn set_drv2<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Dr2r {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dr2r {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.drv2(0) != 0 { try!(write!(f, " drv2[0]"))}
      if self.drv2(1) != 0 { try!(write!(f, " drv2[1]"))}
      if self.drv2(2) != 0 { try!(write!(f, " drv2[2]"))}
      if self.drv2(3) != 0 { try!(write!(f, " drv2[3]"))}
      if self.drv2(4) != 0 { try!(write!(f, " drv2[4]"))}
      if self.drv2(5) != 0 { try!(write!(f, " drv2[5]"))}
      if self.drv2(6) != 0 { try!(write!(f, " drv2[6]"))}
      if self.drv2(7) != 0 { try!(write!(f, " drv2[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="GPIO 4-mA Drive Select"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dr4r(pub u32);
impl Dr4r {
#[doc="GPIO 4-mA Drive Enable"]
   #[inline] pub fn drv4<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="GPIO 4-mA Drive Enable"]
   #[inline] pub fn set_drv4<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Dr4r {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dr4r {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.drv4(0) != 0 { try!(write!(f, " drv4[0]"))}
      if self.drv4(1) != 0 { try!(write!(f, " drv4[1]"))}
      if self.drv4(2) != 0 { try!(write!(f, " drv4[2]"))}
      if self.drv4(3) != 0 { try!(write!(f, " drv4[3]"))}
      if self.drv4(4) != 0 { try!(write!(f, " drv4[4]"))}
      if self.drv4(5) != 0 { try!(write!(f, " drv4[5]"))}
      if self.drv4(6) != 0 { try!(write!(f, " drv4[6]"))}
      if self.drv4(7) != 0 { try!(write!(f, " drv4[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="GPIO 8-mA Drive Select"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dr8r(pub u32);
impl Dr8r {
#[doc="GPIO 8-mA Drive Enable"]
   #[inline] pub fn drv8<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="GPIO 8-mA Drive Enable"]
   #[inline] pub fn set_drv8<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Dr8r {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dr8r {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.drv8(0) != 0 { try!(write!(f, " drv8[0]"))}
      if self.drv8(1) != 0 { try!(write!(f, " drv8[1]"))}
      if self.drv8(2) != 0 { try!(write!(f, " drv8[2]"))}
      if self.drv8(3) != 0 { try!(write!(f, " drv8[3]"))}
      if self.drv8(4) != 0 { try!(write!(f, " drv8[4]"))}
      if self.drv8(5) != 0 { try!(write!(f, " drv8[5]"))}
      if self.drv8(6) != 0 { try!(write!(f, " drv8[6]"))}
      if self.drv8(7) != 0 { try!(write!(f, " drv8[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="GPIO Open Drain Select"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Odr(pub u32);
impl Odr {
#[doc="GPIO Open Drain Enable"]
   #[inline] pub fn ode<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="GPIO Open Drain Enable"]
   #[inline] pub fn set_ode<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Odr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Odr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ode(0) != 0 { try!(write!(f, " ode[0]"))}
      if self.ode(1) != 0 { try!(write!(f, " ode[1]"))}
      if self.ode(2) != 0 { try!(write!(f, " ode[2]"))}
      if self.ode(3) != 0 { try!(write!(f, " ode[3]"))}
      if self.ode(4) != 0 { try!(write!(f, " ode[4]"))}
      if self.ode(5) != 0 { try!(write!(f, " ode[5]"))}
      if self.ode(6) != 0 { try!(write!(f, " ode[6]"))}
      if self.ode(7) != 0 { try!(write!(f, " ode[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="GPIO Pull-Up Select"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pur(pub u32);
impl Pur {
#[doc="GPIO Pull-Up Enable"]
   #[inline] pub fn pue<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="GPIO Pull-Up Enable"]
   #[inline] pub fn set_pue<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Pur {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pur {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pue(0) != 0 { try!(write!(f, " pue[0]"))}
      if self.pue(1) != 0 { try!(write!(f, " pue[1]"))}
      if self.pue(2) != 0 { try!(write!(f, " pue[2]"))}
      if self.pue(3) != 0 { try!(write!(f, " pue[3]"))}
      if self.pue(4) != 0 { try!(write!(f, " pue[4]"))}
      if self.pue(5) != 0 { try!(write!(f, " pue[5]"))}
      if self.pue(6) != 0 { try!(write!(f, " pue[6]"))}
      if self.pue(7) != 0 { try!(write!(f, " pue[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="GPIO Pull-Down Select"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pdr(pub u32);
impl Pdr {
#[doc="GPIO Pull-Down Enable"]
   #[inline] pub fn pde<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="GPIO Pull-Down Enable"]
   #[inline] pub fn set_pde<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Pdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pde(0) != 0 { try!(write!(f, " pde[0]"))}
      if self.pde(1) != 0 { try!(write!(f, " pde[1]"))}
      if self.pde(2) != 0 { try!(write!(f, " pde[2]"))}
      if self.pde(3) != 0 { try!(write!(f, " pde[3]"))}
      if self.pde(4) != 0 { try!(write!(f, " pde[4]"))}
      if self.pde(5) != 0 { try!(write!(f, " pde[5]"))}
      if self.pde(6) != 0 { try!(write!(f, " pde[6]"))}
      if self.pde(7) != 0 { try!(write!(f, " pde[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="GPIO Slew Rate Control Select"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Slr(pub u32);
impl Slr {
#[doc="GPIO Slew Rate Limit Enable"]
   #[inline] pub fn slr<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="GPIO Slew Rate Limit Enable"]
   #[inline] pub fn set_slr<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Slr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Slr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.slr(0) != 0 { try!(write!(f, " slr[0]"))}
      if self.slr(1) != 0 { try!(write!(f, " slr[1]"))}
      if self.slr(2) != 0 { try!(write!(f, " slr[2]"))}
      if self.slr(3) != 0 { try!(write!(f, " slr[3]"))}
      if self.slr(4) != 0 { try!(write!(f, " slr[4]"))}
      if self.slr(5) != 0 { try!(write!(f, " slr[5]"))}
      if self.slr(6) != 0 { try!(write!(f, " slr[6]"))}
      if self.slr(7) != 0 { try!(write!(f, " slr[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="GPIO Digital Enable"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Den(pub u32);
impl Den {
#[doc="GPIO Digital Enable"]
   #[inline] pub fn den<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="GPIO Digital Enable"]
   #[inline] pub fn set_den<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Den {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Den {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.den(0) != 0 { try!(write!(f, " den[0]"))}
      if self.den(1) != 0 { try!(write!(f, " den[1]"))}
      if self.den(2) != 0 { try!(write!(f, " den[2]"))}
      if self.den(3) != 0 { try!(write!(f, " den[3]"))}
      if self.den(4) != 0 { try!(write!(f, " den[4]"))}
      if self.den(5) != 0 { try!(write!(f, " den[5]"))}
      if self.den(6) != 0 { try!(write!(f, " den[6]"))}
      if self.den(7) != 0 { try!(write!(f, " den[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="GPIO Lock"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Lock(pub u32);
impl Lock {
#[doc="GPIO Lock"]
   #[inline] pub fn gpio_lock(&self) -> bits::U32 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
   }
#[doc="GPIO Lock"]
   #[inline] pub fn set_gpio_lock<V: Into<bits::U32>>(mut self, value: V) -> Self {
      let value: bits::U32 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffffffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Lock {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Lock {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="GPIO Commit"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
#[doc="GPIO Commit"]
   #[inline] pub fn cr<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="GPIO Commit"]
   #[inline] pub fn set_cr<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Cr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cr(0) != 0 { try!(write!(f, " cr[0]"))}
      if self.cr(1) != 0 { try!(write!(f, " cr[1]"))}
      if self.cr(2) != 0 { try!(write!(f, " cr[2]"))}
      if self.cr(3) != 0 { try!(write!(f, " cr[3]"))}
      if self.cr(4) != 0 { try!(write!(f, " cr[4]"))}
      if self.cr(5) != 0 { try!(write!(f, " cr[5]"))}
      if self.cr(6) != 0 { try!(write!(f, " cr[6]"))}
      if self.cr(7) != 0 { try!(write!(f, " cr[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="GPIO Analog Mode Select"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Amsel(pub u32);
impl Amsel {
#[doc="GPIO Analog Mode Select"]
   #[inline] pub fn gpioamsel<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="GPIO Analog Mode Select"]
   #[inline] pub fn set_gpioamsel<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Amsel {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Amsel {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.gpioamsel(0) != 0 { try!(write!(f, " gpioamsel[0]"))}
      if self.gpioamsel(1) != 0 { try!(write!(f, " gpioamsel[1]"))}
      if self.gpioamsel(2) != 0 { try!(write!(f, " gpioamsel[2]"))}
      if self.gpioamsel(3) != 0 { try!(write!(f, " gpioamsel[3]"))}
      if self.gpioamsel(4) != 0 { try!(write!(f, " gpioamsel[4]"))}
      if self.gpioamsel(5) != 0 { try!(write!(f, " gpioamsel[5]"))}
      if self.gpioamsel(6) != 0 { try!(write!(f, " gpioamsel[6]"))}
      if self.gpioamsel(7) != 0 { try!(write!(f, " gpioamsel[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="GPIO Port Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pctl(pub u32);
impl Pctl {
#[doc="GPIO Port Mux Control"]
   #[inline] pub fn pmc<I: Into<bits::R8>>(&self, index: I) -> bits::U4 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + (index << 2);
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0xf) as u8) } // [3:0]
   }
#[doc="GPIO Port Mux Control"]
   #[inline] pub fn set_pmc<I: Into<bits::R8>, V: Into<bits::U4>>(mut self, index: I, value: V) -> Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + (index << 2);
      self.0 &= !(0xf << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Pctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pmc(0) != 0 { try!(write!(f, " pmc[0]=0x{:x}", self.pmc(0)))}
      if self.pmc(1) != 0 { try!(write!(f, " pmc[1]=0x{:x}", self.pmc(1)))}
      if self.pmc(2) != 0 { try!(write!(f, " pmc[2]=0x{:x}", self.pmc(2)))}
      if self.pmc(3) != 0 { try!(write!(f, " pmc[3]=0x{:x}", self.pmc(3)))}
      if self.pmc(4) != 0 { try!(write!(f, " pmc[4]=0x{:x}", self.pmc(4)))}
      if self.pmc(5) != 0 { try!(write!(f, " pmc[5]=0x{:x}", self.pmc(5)))}
      if self.pmc(6) != 0 { try!(write!(f, " pmc[6]=0x{:x}", self.pmc(6)))}
      if self.pmc(7) != 0 { try!(write!(f, " pmc[7]=0x{:x}", self.pmc(7)))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="GPIO ADC Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Adcctl(pub u32);
impl Adcctl {
#[doc="ADC Trigger Enable"]
   #[inline] pub fn adcen<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="ADC Trigger Enable"]
   #[inline] pub fn set_adcen<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Adcctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Adcctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.adcen(0) != 0 { try!(write!(f, " adcen[0]"))}
      if self.adcen(1) != 0 { try!(write!(f, " adcen[1]"))}
      if self.adcen(2) != 0 { try!(write!(f, " adcen[2]"))}
      if self.adcen(3) != 0 { try!(write!(f, " adcen[3]"))}
      if self.adcen(4) != 0 { try!(write!(f, " adcen[4]"))}
      if self.adcen(5) != 0 { try!(write!(f, " adcen[5]"))}
      if self.adcen(6) != 0 { try!(write!(f, " adcen[6]"))}
      if self.adcen(7) != 0 { try!(write!(f, " adcen[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="GPIO DMA Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dmactl(pub u32);
impl Dmactl {
#[doc="uDMA Trigger Enable"]
   #[inline] pub fn dmaen<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="uDMA Trigger Enable"]
   #[inline] pub fn set_dmaen<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Dmactl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dmactl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dmaen(0) != 0 { try!(write!(f, " dmaen[0]"))}
      if self.dmaen(1) != 0 { try!(write!(f, " dmaen[1]"))}
      if self.dmaen(2) != 0 { try!(write!(f, " dmaen[2]"))}
      if self.dmaen(3) != 0 { try!(write!(f, " dmaen[3]"))}
      if self.dmaen(4) != 0 { try!(write!(f, " dmaen[4]"))}
      if self.dmaen(5) != 0 { try!(write!(f, " dmaen[5]"))}
      if self.dmaen(6) != 0 { try!(write!(f, " dmaen[6]"))}
      if self.dmaen(7) != 0 { try!(write!(f, " dmaen[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="GPIO Select Interrupt"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Gpiosi(pub u32);
impl Gpiosi {
#[doc="Summary Interrupt"]
   #[inline] pub fn sum<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="Summary Interrupt"]
   #[inline] pub fn set_sum<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Gpiosi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Gpiosi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sum(0) != 0 { try!(write!(f, " sum[0]"))}
      if self.sum(1) != 0 { try!(write!(f, " sum[1]"))}
      if self.sum(2) != 0 { try!(write!(f, " sum[2]"))}
      if self.sum(3) != 0 { try!(write!(f, " sum[3]"))}
      if self.sum(4) != 0 { try!(write!(f, " sum[4]"))}
      if self.sum(5) != 0 { try!(write!(f, " sum[5]"))}
      if self.sum(6) != 0 { try!(write!(f, " sum[6]"))}
      if self.sum(7) != 0 { try!(write!(f, " sum[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="GPIO 12-mA Drive Select"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Gpiodr12r(pub u32);
impl Gpiodr12r {
#[doc="12-mA Drive Enable"]
   #[inline] pub fn drv12<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="12-mA Drive Enable"]
   #[inline] pub fn set_drv12<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Gpiodr12r {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Gpiodr12r {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.drv12(0) != 0 { try!(write!(f, " drv12[0]"))}
      if self.drv12(1) != 0 { try!(write!(f, " drv12[1]"))}
      if self.drv12(2) != 0 { try!(write!(f, " drv12[2]"))}
      if self.drv12(3) != 0 { try!(write!(f, " drv12[3]"))}
      if self.drv12(4) != 0 { try!(write!(f, " drv12[4]"))}
      if self.drv12(5) != 0 { try!(write!(f, " drv12[5]"))}
      if self.drv12(6) != 0 { try!(write!(f, " drv12[6]"))}
      if self.drv12(7) != 0 { try!(write!(f, " drv12[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="GPIO Wake Pin Enable"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Gpiowakepen(pub u32);
impl Gpiowakepen {
#[doc="K[7] Wake Enable"]
   #[inline] pub fn wakep7(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="K[7] Wake Enable"]
   #[inline] pub fn set_wakep7<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="K[6] Wake Enable"]
   #[inline] pub fn wakep6(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="K[6] Wake Enable"]
   #[inline] pub fn set_wakep6<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="K[5] Wake Enable"]
   #[inline] pub fn wakep5(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="K[5] Wake Enable"]
   #[inline] pub fn set_wakep5<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="K[4] Wake Enable"]
   #[inline] pub fn wakep4(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="K[4] Wake Enable"]
   #[inline] pub fn set_wakep4<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

}
impl ::core::fmt::Display for Gpiowakepen {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Gpiowakepen {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.wakep7() != 0 { try!(write!(f, " wakep7"))}
      if self.wakep6() != 0 { try!(write!(f, " wakep6"))}
      if self.wakep5() != 0 { try!(write!(f, " wakep5"))}
      if self.wakep4() != 0 { try!(write!(f, " wakep4"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="GPIO Wake Level"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Gpiowakelvl(pub u32);
impl Gpiowakelvl {
#[doc="K[7] Wake Level"]
   #[inline] pub fn wakelvl7(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="K[7] Wake Level"]
   #[inline] pub fn set_wakelvl7<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="K[6] Wake Level"]
   #[inline] pub fn wakelvl6(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="K[6] Wake Level"]
   #[inline] pub fn set_wakelvl6<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="K[5] Wake Level"]
   #[inline] pub fn wakelvl5(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="K[5] Wake Level"]
   #[inline] pub fn set_wakelvl5<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="K[4] Wake Level"]
   #[inline] pub fn wakelvl4(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="K[4] Wake Level"]
   #[inline] pub fn set_wakelvl4<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

}
impl ::core::fmt::Display for Gpiowakelvl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Gpiowakelvl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.wakelvl7() != 0 { try!(write!(f, " wakelvl7"))}
      if self.wakelvl6() != 0 { try!(write!(f, " wakelvl6"))}
      if self.wakelvl5() != 0 { try!(write!(f, " wakelvl5"))}
      if self.wakelvl4() != 0 { try!(write!(f, " wakelvl4"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="GPIO Wake Level"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Gpiowakestat(pub u32);
impl Gpiowakestat {
#[doc="K[7] Wake Status"]
   #[inline] pub fn wakestat7(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="K[7] Wake Status"]
   #[inline] pub fn set_wakestat7<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="K[6] Wake Status"]
   #[inline] pub fn wakestat6(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="K[6] Wake Status"]
   #[inline] pub fn set_wakestat6<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="K[5] Wake Status"]
   #[inline] pub fn wakestat5(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="K[5] Wake Status"]
   #[inline] pub fn set_wakestat5<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="K[4] Wake Status"]
   #[inline] pub fn wakestat4(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="K[4] Wake Status"]
   #[inline] pub fn set_wakestat4<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

}
impl ::core::fmt::Display for Gpiowakestat {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Gpiowakestat {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.wakestat7() != 0 { try!(write!(f, " wakestat7"))}
      if self.wakestat6() != 0 { try!(write!(f, " wakestat6"))}
      if self.wakestat5() != 0 { try!(write!(f, " wakestat5"))}
      if self.wakestat4() != 0 { try!(write!(f, " wakestat4"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
pub struct GpioPin { pub port: GpioPeriph, pub index: usize }
pin!(PA0, Pa0, GPIOA, Gpioa, _PA0, GpioPin, _GPIOA, 0);
   alt_fn!(Pa0, super::sig::U0rx, 1);
   alt_fn!(Pa0, super::sig::I2c9scl, 2);
   alt_fn!(Pa0, super::sig::T0ccp0, 3);
   alt_fn!(Pa0, super::sig::Can0rx, 7);

pin!(PA1, Pa1, GPIOA, Gpioa, _PA1, GpioPin, _GPIOA, 1);
   alt_fn!(Pa1, super::sig::U0tx, 1);
   alt_fn!(Pa1, super::sig::I2c9sda, 2);
   alt_fn!(Pa1, super::sig::T0ccp1, 3);
   alt_fn!(Pa1, super::sig::Can0tx, 7);

pin!(PA2, Pa2, GPIOA, Gpioa, _PA2, GpioPin, _GPIOA, 2);
   alt_fn!(Pa2, super::sig::U4rx, 1);
   alt_fn!(Pa2, super::sig::I2c8scl, 2);
   alt_fn!(Pa2, super::sig::T1ccp0, 3);
   alt_fn!(Pa2, super::sig::Ssi0clk, 15);

pin!(PA3, Pa3, GPIOA, Gpioa, _PA3, GpioPin, _GPIOA, 3);
   alt_fn!(Pa3, super::sig::U4tx, 1);
   alt_fn!(Pa3, super::sig::I2c8sda, 2);
   alt_fn!(Pa3, super::sig::T1ccp1, 3);
   alt_fn!(Pa3, super::sig::Ssi0fss, 15);

pin!(PA4, Pa4, GPIOA, Gpioa, _PA4, GpioPin, _GPIOA, 4);
   alt_fn!(Pa4, super::sig::U3rx, 1);
   alt_fn!(Pa4, super::sig::I2c7scl, 2);
   alt_fn!(Pa4, super::sig::T2ccp0, 3);
   alt_fn!(Pa4, super::sig::Ssi0xdat0, 15);

pin!(PA5, Pa5, GPIOA, Gpioa, _PA5, GpioPin, _GPIOA, 5);
   alt_fn!(Pa5, super::sig::U3tx, 1);
   alt_fn!(Pa5, super::sig::I2c7sda, 2);
   alt_fn!(Pa5, super::sig::T2ccp1, 3);
   alt_fn!(Pa5, super::sig::Ssi0xdat1, 15);

pin!(PA6, Pa6, GPIOA, Gpioa, _PA6, GpioPin, _GPIOA, 6);
   alt_fn!(Pa6, super::sig::U2rx, 1);
   alt_fn!(Pa6, super::sig::I2c6scl, 2);
   alt_fn!(Pa6, super::sig::T3ccp0, 3);
   alt_fn!(Pa6, super::sig::Usb0epen, 5);
   alt_fn!(Pa6, super::sig::Ssi0xdat2, 13);
   alt_fn!(Pa6, super::sig::Epi0s8, 15);

pin!(PA7, Pa7, GPIOA, Gpioa, _PA7, GpioPin, _GPIOA, 7);
   alt_fn!(Pa7, super::sig::U2tx, 1);
   alt_fn!(Pa7, super::sig::I2c6sda, 2);
   alt_fn!(Pa7, super::sig::T3ccp1, 3);
   alt_fn!(Pa7, super::sig::Usb0pflt, 5);
   alt_fn!(Pa7, super::sig::Usb0epen, 11);
   alt_fn!(Pa7, super::sig::Ssi0xdat3, 13);
   alt_fn!(Pa7, super::sig::Epi0s9, 15);

pin!(PB0, Pb0, GPIOB, Gpiob, _PB0, GpioPin, _GPIOB, 0);
   alt_fn!(Pb0, super::sig::Usb0id, 0);
   alt_fn!(Pb0, super::sig::U1rx, 1);
   alt_fn!(Pb0, super::sig::I2c5scl, 2);
   alt_fn!(Pb0, super::sig::T4ccp0, 3);
   alt_fn!(Pb0, super::sig::Can1rx, 7);

pin!(PB1, Pb1, GPIOB, Gpiob, _PB1, GpioPin, _GPIOB, 1);
   alt_fn!(Pb1, super::sig::Usb0vbus, 0);
   alt_fn!(Pb1, super::sig::U1tx, 1);
   alt_fn!(Pb1, super::sig::I2c5sda, 2);
   alt_fn!(Pb1, super::sig::T4ccp1, 3);
   alt_fn!(Pb1, super::sig::Can1tx, 7);

pin!(PB2, Pb2, GPIOB, Gpiob, _PB2, GpioPin, _GPIOB, 2);
   alt_fn!(Pb2, super::sig::I2c0scl, 2);
   alt_fn!(Pb2, super::sig::T5ccp0, 3);
   alt_fn!(Pb2, super::sig::Usb0stp, 14);
   alt_fn!(Pb2, super::sig::Epi0s27, 15);

pin!(PB3, Pb3, GPIOB, Gpiob, _PB3, GpioPin, _GPIOB, 3);
   alt_fn!(Pb3, super::sig::I2c0sda, 2);
   alt_fn!(Pb3, super::sig::T5ccp1, 3);
   alt_fn!(Pb3, super::sig::Usb0clk, 14);
   alt_fn!(Pb3, super::sig::Epi0s28, 15);

pin!(PB4, Pb4, GPIOB, Gpiob, _PB4, GpioPin, _GPIOB, 4);
   alt_fn!(Pb4, super::sig::Ain10, 0);
   alt_fn!(Pb4, super::sig::U0cts, 1);
   alt_fn!(Pb4, super::sig::I2c5scl, 2);
   alt_fn!(Pb4, super::sig::Ssi1fss, 15);

pin!(PB5, Pb5, GPIOB, Gpiob, _PB5, GpioPin, _GPIOB, 5);
   alt_fn!(Pb5, super::sig::Ain11, 0);
   alt_fn!(Pb5, super::sig::U0rts, 1);
   alt_fn!(Pb5, super::sig::I2c5sda, 2);
   alt_fn!(Pb5, super::sig::Ssi1clk, 15);

pin!(PC0, Pc0, GPIOC, Gpioc, _PC0, GpioPin, _GPIOC, 0);
   alt_fn!(Pc0, super::sig::Tck, 1);
   alt_fn!(Pc0, super::sig::Swclk, 1);

pin!(PC1, Pc1, GPIOC, Gpioc, _PC1, GpioPin, _GPIOC, 1);
   alt_fn!(Pc1, super::sig::Tms, 1);
   alt_fn!(Pc1, super::sig::Swdio, 1);

pin!(PC2, Pc2, GPIOC, Gpioc, _PC2, GpioPin, _GPIOC, 2);
   alt_fn!(Pc2, super::sig::Tdi, 1);

pin!(PC3, Pc3, GPIOC, Gpioc, _PC3, GpioPin, _GPIOC, 3);
   alt_fn!(Pc3, super::sig::Tdo, 1);
   alt_fn!(Pc3, super::sig::Swo, 1);

pin!(PC4, Pc4, GPIOC, Gpioc, _PC4, GpioPin, _GPIOC, 4);
   alt_fn!(Pc4, super::sig::C1Neg, 0);
   alt_fn!(Pc4, super::sig::U7rx, 1);
   alt_fn!(Pc4, super::sig::Epi0s7, 15);

pin!(PC5, Pc5, GPIOC, Gpioc, _PC5, GpioPin, _GPIOC, 5);
   alt_fn!(Pc5, super::sig::C1Pos, 0);
   alt_fn!(Pc5, super::sig::U7tx, 1);
   alt_fn!(Pc5, super::sig::Rtcclk, 7);
   alt_fn!(Pc5, super::sig::Epi0s6, 15);

pin!(PC6, Pc6, GPIOC, Gpioc, _PC6, GpioPin, _GPIOC, 6);
   alt_fn!(Pc6, super::sig::C0Neg, 0);
   alt_fn!(Pc6, super::sig::U5rx, 1);
   alt_fn!(Pc6, super::sig::Epi0s5, 15);

pin!(PC7, Pc7, GPIOC, Gpioc, _PC7, GpioPin, _GPIOC, 7);
   alt_fn!(Pc7, super::sig::C0Pos, 0);
   alt_fn!(Pc7, super::sig::U5tx, 1);
   alt_fn!(Pc7, super::sig::Epi0s4, 15);

pin!(PD0, Pd0, GPIOD, Gpiod, _PD0, GpioPin, _GPIOD, 0);
   alt_fn!(Pd0, super::sig::Ain15, 0);
   alt_fn!(Pd0, super::sig::I2c7scl, 2);
   alt_fn!(Pd0, super::sig::T0ccp0, 3);
   alt_fn!(Pd0, super::sig::C0o, 5);
   alt_fn!(Pd0, super::sig::Ssi2xdat1, 15);

pin!(PD1, Pd1, GPIOD, Gpiod, _PD1, GpioPin, _GPIOD, 1);
   alt_fn!(Pd1, super::sig::Ain14, 0);
   alt_fn!(Pd1, super::sig::I2c7sda, 2);
   alt_fn!(Pd1, super::sig::T0ccp1, 3);
   alt_fn!(Pd1, super::sig::C1o, 5);
   alt_fn!(Pd1, super::sig::Ssi2xdat0, 15);

pin!(PD2, Pd2, GPIOD, Gpiod, _PD2, GpioPin, _GPIOD, 2);
   alt_fn!(Pd2, super::sig::Ain13, 0);
   alt_fn!(Pd2, super::sig::I2c8scl, 2);
   alt_fn!(Pd2, super::sig::T1ccp0, 3);
   alt_fn!(Pd2, super::sig::C2o, 5);
   alt_fn!(Pd2, super::sig::Ssi2fss, 15);

pin!(PD3, Pd3, GPIOD, Gpiod, _PD3, GpioPin, _GPIOD, 3);
   alt_fn!(Pd3, super::sig::Ain12, 0);
   alt_fn!(Pd3, super::sig::I2c8sda, 2);
   alt_fn!(Pd3, super::sig::T1ccp1, 3);
   alt_fn!(Pd3, super::sig::Ssi2clk, 15);

pin!(PD4, Pd4, GPIOD, Gpiod, _PD4, GpioPin, _GPIOD, 4);
   alt_fn!(Pd4, super::sig::Ain7, 0);
   alt_fn!(Pd4, super::sig::U2rx, 1);
   alt_fn!(Pd4, super::sig::T3ccp0, 3);
   alt_fn!(Pd4, super::sig::Ssi1xdat2, 15);

pin!(PD5, Pd5, GPIOD, Gpiod, _PD5, GpioPin, _GPIOD, 5);
   alt_fn!(Pd5, super::sig::Ain6, 0);
   alt_fn!(Pd5, super::sig::U2tx, 1);
   alt_fn!(Pd5, super::sig::T3ccp1, 3);
   alt_fn!(Pd5, super::sig::Ssi1xdat3, 15);

pin!(PD6, Pd6, GPIOD, Gpiod, _PD6, GpioPin, _GPIOD, 6);
   alt_fn!(Pd6, super::sig::Ain5, 0);
   alt_fn!(Pd6, super::sig::U2rts, 1);
   alt_fn!(Pd6, super::sig::T4ccp0, 3);
   alt_fn!(Pd6, super::sig::Usb0epen, 5);
   alt_fn!(Pd6, super::sig::Ssi2xdat3, 15);

pin!(PD7, Pd7, GPIOD, Gpiod, _PD7, GpioPin, _GPIOD, 7);
   alt_fn!(Pd7, super::sig::Ain4, 0);
   alt_fn!(Pd7, super::sig::U2cts, 1);
   alt_fn!(Pd7, super::sig::T4ccp1, 3);
   alt_fn!(Pd7, super::sig::Usb0pflt, 5);
   alt_fn!(Pd7, super::sig::Nmi, 8);
   alt_fn!(Pd7, super::sig::Ssi2xdat2, 15);

pin!(PE0, Pe0, GPIOE, Gpioe, _PE0, GpioPin, _GPIOE, 0);
   alt_fn!(Pe0, super::sig::Ain3, 0);
   alt_fn!(Pe0, super::sig::U1rts, 1);

pin!(PE1, Pe1, GPIOE, Gpioe, _PE1, GpioPin, _GPIOE, 1);
   alt_fn!(Pe1, super::sig::Ain2, 0);
   alt_fn!(Pe1, super::sig::U1dsr, 1);

pin!(PE2, Pe2, GPIOE, Gpioe, _PE2, GpioPin, _GPIOE, 2);
   alt_fn!(Pe2, super::sig::Ain1, 0);
   alt_fn!(Pe2, super::sig::U1dcd, 1);

pin!(PE3, Pe3, GPIOE, Gpioe, _PE3, GpioPin, _GPIOE, 3);
   alt_fn!(Pe3, super::sig::Ain0, 0);
   alt_fn!(Pe3, super::sig::U1dtr, 1);

pin!(PE4, Pe4, GPIOE, Gpioe, _PE4, GpioPin, _GPIOE, 4);
   alt_fn!(Pe4, super::sig::Ain9, 0);
   alt_fn!(Pe4, super::sig::U1ri, 1);
   alt_fn!(Pe4, super::sig::Ssi1xdat0, 15);

pin!(PE5, Pe5, GPIOE, Gpioe, _PE5, GpioPin, _GPIOE, 5);
   alt_fn!(Pe5, super::sig::Ain8, 0);
   alt_fn!(Pe5, super::sig::Ssi1xdat1, 15);

pin!(PF0, Pf0, GPIOF, Gpiof, _PF0, GpioPin, _GPIOF, 0);
   alt_fn!(Pf0, super::sig::En0led0, 5);
   alt_fn!(Pf0, super::sig::M0pwm0, 6);
   alt_fn!(Pf0, super::sig::Ssi3xdat1, 14);
   alt_fn!(Pf0, super::sig::Trd2, 15);

pin!(PF1, Pf1, GPIOF, Gpiof, _PF1, GpioPin, _GPIOF, 1);
   alt_fn!(Pf1, super::sig::En0led2, 5);
   alt_fn!(Pf1, super::sig::M0pwm1, 6);
   alt_fn!(Pf1, super::sig::Ssi3xdat0, 14);
   alt_fn!(Pf1, super::sig::Trd1, 15);

pin!(PF2, Pf2, GPIOF, Gpiof, _PF2, GpioPin, _GPIOF, 2);
   alt_fn!(Pf2, super::sig::M0pwm2, 6);
   alt_fn!(Pf2, super::sig::Ssi3fss, 14);
   alt_fn!(Pf2, super::sig::Trd0, 15);

pin!(PF3, Pf3, GPIOF, Gpiof, _PF3, GpioPin, _GPIOF, 3);
   alt_fn!(Pf3, super::sig::M0pwm3, 6);
   alt_fn!(Pf3, super::sig::Ssi3clk, 14);
   alt_fn!(Pf3, super::sig::Trclk, 15);

pin!(PF4, Pf4, GPIOF, Gpiof, _PF4, GpioPin, _GPIOF, 4);
   alt_fn!(Pf4, super::sig::En0led1, 5);
   alt_fn!(Pf4, super::sig::M0fault0, 6);
   alt_fn!(Pf4, super::sig::Ssi3xdat2, 14);
   alt_fn!(Pf4, super::sig::Trd3, 15);

pin!(PG0, Pg0, GPIOG, Gpiog, _PG0, GpioPin, _GPIOG, 0);
   alt_fn!(Pg0, super::sig::I2c1scl, 2);
   alt_fn!(Pg0, super::sig::En0pps, 5);
   alt_fn!(Pg0, super::sig::M0pwm4, 6);
   alt_fn!(Pg0, super::sig::Epi0s11, 15);

pin!(PG1, Pg1, GPIOG, Gpiog, _PG1, GpioPin, _GPIOG, 1);
   alt_fn!(Pg1, super::sig::I2c1sda, 2);
   alt_fn!(Pg1, super::sig::M0pwm5, 6);
   alt_fn!(Pg1, super::sig::Epi0s10, 15);

pin!(PH0, Ph0, GPIOH, Gpioh, _PH0, GpioPin, _GPIOH, 0);
   alt_fn!(Ph0, super::sig::U0rts, 1);
   alt_fn!(Ph0, super::sig::Epi0s0, 15);

pin!(PH1, Ph1, GPIOH, Gpioh, _PH1, GpioPin, _GPIOH, 1);
   alt_fn!(Ph1, super::sig::U0cts, 1);
   alt_fn!(Ph1, super::sig::Epi0s1, 15);

pin!(PH2, Ph2, GPIOH, Gpioh, _PH2, GpioPin, _GPIOH, 2);
   alt_fn!(Ph2, super::sig::U0dcd, 1);
   alt_fn!(Ph2, super::sig::Epi0s2, 15);

pin!(PH3, Ph3, GPIOH, Gpioh, _PH3, GpioPin, _GPIOH, 3);
   alt_fn!(Ph3, super::sig::U0dsr, 1);
   alt_fn!(Ph3, super::sig::Epi0s3, 15);

pin!(PJ0, Pj0, GPIOJ, Gpioj, _PJ0, GpioPin, _GPIOJ, 0);
   alt_fn!(Pj0, super::sig::U3rx, 1);
   alt_fn!(Pj0, super::sig::En0pps, 5);

pin!(PJ1, Pj1, GPIOJ, Gpioj, _PJ1, GpioPin, _GPIOJ, 1);
   alt_fn!(Pj1, super::sig::U3tx, 1);

pin!(PK0, Pk0, GPIOK, Gpiok, _PK0, GpioPin, _GPIOK, 0);
   alt_fn!(Pk0, super::sig::Ain16, 0);
   alt_fn!(Pk0, super::sig::U4rx, 1);
   alt_fn!(Pk0, super::sig::Epi0s0, 15);

pin!(PK1, Pk1, GPIOK, Gpiok, _PK1, GpioPin, _GPIOK, 1);
   alt_fn!(Pk1, super::sig::Ain17, 0);
   alt_fn!(Pk1, super::sig::U4tx, 1);
   alt_fn!(Pk1, super::sig::Epi0s1, 15);

pin!(PK2, Pk2, GPIOK, Gpiok, _PK2, GpioPin, _GPIOK, 2);
   alt_fn!(Pk2, super::sig::Ain18, 0);
   alt_fn!(Pk2, super::sig::U4rts, 1);
   alt_fn!(Pk2, super::sig::Epi0s2, 15);

pin!(PK3, Pk3, GPIOK, Gpiok, _PK3, GpioPin, _GPIOK, 3);
   alt_fn!(Pk3, super::sig::Ain19, 0);
   alt_fn!(Pk3, super::sig::U4cts, 1);
   alt_fn!(Pk3, super::sig::Epi0s3, 15);

pin!(PK4, Pk4, GPIOK, Gpiok, _PK4, GpioPin, _GPIOK, 4);
   alt_fn!(Pk4, super::sig::I2c3scl, 2);
   alt_fn!(Pk4, super::sig::En0led0, 5);
   alt_fn!(Pk4, super::sig::M0pwm6, 6);
   alt_fn!(Pk4, super::sig::Epi0s32, 15);

pin!(PK5, Pk5, GPIOK, Gpiok, _PK5, GpioPin, _GPIOK, 5);
   alt_fn!(Pk5, super::sig::I2c3sda, 2);
   alt_fn!(Pk5, super::sig::En0led2, 5);
   alt_fn!(Pk5, super::sig::M0pwm7, 6);
   alt_fn!(Pk5, super::sig::Epi0s31, 15);

pin!(PK6, Pk6, GPIOK, Gpiok, _PK6, GpioPin, _GPIOK, 6);
   alt_fn!(Pk6, super::sig::I2c4scl, 2);
   alt_fn!(Pk6, super::sig::En0led1, 5);
   alt_fn!(Pk6, super::sig::M0fault1, 6);
   alt_fn!(Pk6, super::sig::Epi0s25, 15);

pin!(PK7, Pk7, GPIOK, Gpiok, _PK7, GpioPin, _GPIOK, 7);
   alt_fn!(Pk7, super::sig::U0ri, 1);
   alt_fn!(Pk7, super::sig::I2c4sda, 2);
   alt_fn!(Pk7, super::sig::Rtcclk, 5);
   alt_fn!(Pk7, super::sig::M0fault2, 6);
   alt_fn!(Pk7, super::sig::Epi0s24, 15);

pin!(PL0, Pl0, GPIOL, Gpiol, _PL0, GpioPin, _GPIOL, 0);
   alt_fn!(Pl0, super::sig::I2c2sda, 2);
   alt_fn!(Pl0, super::sig::M0fault3, 6);
   alt_fn!(Pl0, super::sig::Usb0d0, 14);
   alt_fn!(Pl0, super::sig::Epi0s16, 15);

pin!(PL1, Pl1, GPIOL, Gpiol, _PL1, GpioPin, _GPIOL, 1);
   alt_fn!(Pl1, super::sig::I2c2scl, 2);
   alt_fn!(Pl1, super::sig::Pha0, 6);
   alt_fn!(Pl1, super::sig::Usb0d1, 14);
   alt_fn!(Pl1, super::sig::Epi0s17, 15);

pin!(PL2, Pl2, GPIOL, Gpiol, _PL2, GpioPin, _GPIOL, 2);
   alt_fn!(Pl2, super::sig::C0o, 5);
   alt_fn!(Pl2, super::sig::Phb0, 6);
   alt_fn!(Pl2, super::sig::Usb0d2, 14);
   alt_fn!(Pl2, super::sig::Epi0s18, 15);

pin!(PL3, Pl3, GPIOL, Gpiol, _PL3, GpioPin, _GPIOL, 3);
   alt_fn!(Pl3, super::sig::C1o, 5);
   alt_fn!(Pl3, super::sig::Idx0, 6);
   alt_fn!(Pl3, super::sig::Usb0d3, 14);
   alt_fn!(Pl3, super::sig::Epi0s19, 15);

pin!(PL4, Pl4, GPIOL, Gpiol, _PL4, GpioPin, _GPIOL, 4);
   alt_fn!(Pl4, super::sig::T0ccp0, 3);
   alt_fn!(Pl4, super::sig::Usb0d4, 14);
   alt_fn!(Pl4, super::sig::Epi0s26, 15);

pin!(PL5, Pl5, GPIOL, Gpiol, _PL5, GpioPin, _GPIOL, 5);
   alt_fn!(Pl5, super::sig::T0ccp1, 3);
   alt_fn!(Pl5, super::sig::Usb0d5, 14);
   alt_fn!(Pl5, super::sig::Epi0s33, 15);

pin!(PL6, Pl6, GPIOL, Gpiol, _PL6, GpioPin, _GPIOL, 6);
   alt_fn!(Pl6, super::sig::Usb0dp, 0);
   alt_fn!(Pl6, super::sig::T1ccp0, 3);

pin!(PL7, Pl7, GPIOL, Gpiol, _PL7, GpioPin, _GPIOL, 7);
   alt_fn!(Pl7, super::sig::Usb0dm, 0);
   alt_fn!(Pl7, super::sig::T1ccp1, 3);

pin!(PM0, Pm0, GPIOM, Gpiom, _PM0, GpioPin, _GPIOM, 0);
   alt_fn!(Pm0, super::sig::T2ccp0, 3);
   alt_fn!(Pm0, super::sig::Epi0s15, 15);

pin!(PM1, Pm1, GPIOM, Gpiom, _PM1, GpioPin, _GPIOM, 1);
   alt_fn!(Pm1, super::sig::T2ccp1, 3);
   alt_fn!(Pm1, super::sig::Epi0s14, 15);

pin!(PM2, Pm2, GPIOM, Gpiom, _PM2, GpioPin, _GPIOM, 2);
   alt_fn!(Pm2, super::sig::T3ccp0, 3);
   alt_fn!(Pm2, super::sig::Epi0s13, 15);

pin!(PM3, Pm3, GPIOM, Gpiom, _PM3, GpioPin, _GPIOM, 3);
   alt_fn!(Pm3, super::sig::T3ccp1, 3);
   alt_fn!(Pm3, super::sig::Epi0s12, 15);

pin!(PM4, Pm4, GPIOM, Gpiom, _PM4, GpioPin, _GPIOM, 4);
   alt_fn!(Pm4, super::sig::Tmpr3, 0);
   alt_fn!(Pm4, super::sig::U0cts, 1);
   alt_fn!(Pm4, super::sig::T4ccp0, 3);

pin!(PM5, Pm5, GPIOM, Gpiom, _PM5, GpioPin, _GPIOM, 5);
   alt_fn!(Pm5, super::sig::Tmpr2, 0);
   alt_fn!(Pm5, super::sig::U0dcd, 1);
   alt_fn!(Pm5, super::sig::T4ccp1, 3);

pin!(PM6, Pm6, GPIOM, Gpiom, _PM6, GpioPin, _GPIOM, 6);
   alt_fn!(Pm6, super::sig::Tmpr1, 0);
   alt_fn!(Pm6, super::sig::U0dsr, 1);
   alt_fn!(Pm6, super::sig::T5ccp0, 3);

pin!(PM7, Pm7, GPIOM, Gpiom, _PM7, GpioPin, _GPIOM, 7);
   alt_fn!(Pm7, super::sig::Tmpr0, 0);
   alt_fn!(Pm7, super::sig::U0ri, 1);
   alt_fn!(Pm7, super::sig::T5ccp1, 3);

pin!(PN0, Pn0, GPION, Gpion, _PN0, GpioPin, _GPION, 0);
   alt_fn!(Pn0, super::sig::U1rts, 1);

pin!(PN1, Pn1, GPION, Gpion, _PN1, GpioPin, _GPION, 1);
   alt_fn!(Pn1, super::sig::U1cts, 1);

pin!(PN2, Pn2, GPION, Gpion, _PN2, GpioPin, _GPION, 2);
   alt_fn!(Pn2, super::sig::U1dcd, 1);
   alt_fn!(Pn2, super::sig::U2rts, 2);
   alt_fn!(Pn2, super::sig::Epi0s29, 15);

pin!(PN3, Pn3, GPION, Gpion, _PN3, GpioPin, _GPION, 3);
   alt_fn!(Pn3, super::sig::U1dsr, 1);
   alt_fn!(Pn3, super::sig::U2cts, 2);
   alt_fn!(Pn3, super::sig::Epi0s30, 15);

pin!(PN4, Pn4, GPION, Gpion, _PN4, GpioPin, _GPION, 4);
   alt_fn!(Pn4, super::sig::U1dtr, 1);
   alt_fn!(Pn4, super::sig::U3rts, 2);
   alt_fn!(Pn4, super::sig::I2c2sda, 3);
   alt_fn!(Pn4, super::sig::Epi0s34, 15);

pin!(PN5, Pn5, GPION, Gpion, _PN5, GpioPin, _GPION, 5);
   alt_fn!(Pn5, super::sig::U1ri, 1);
   alt_fn!(Pn5, super::sig::U3cts, 2);
   alt_fn!(Pn5, super::sig::I2c2scl, 3);
   alt_fn!(Pn5, super::sig::Epi0s35, 15);

pin!(PP0, Pp0, GPIOP, Gpiop, _PP0, GpioPin, _GPIOP, 0);
   alt_fn!(Pp0, super::sig::C2Pos, 0);
   alt_fn!(Pp0, super::sig::U6rx, 1);
   alt_fn!(Pp0, super::sig::Ssi3xdat, 15);

pin!(PP1, Pp1, GPIOP, Gpiop, _PP1, GpioPin, _GPIOP, 1);
   alt_fn!(Pp1, super::sig::C2Neg, 0);
   alt_fn!(Pp1, super::sig::U6tx, 1);
   alt_fn!(Pp1, super::sig::Ssi3xdat, 15);

pin!(PP2, Pp2, GPIOP, Gpiop, _PP2, GpioPin, _GPIOP, 2);
   alt_fn!(Pp2, super::sig::U0dtr, 1);
   alt_fn!(Pp2, super::sig::Usb0nxt, 14);
   alt_fn!(Pp2, super::sig::Epi0s29, 15);

pin!(PP3, Pp3, GPIOP, Gpiop, _PP3, GpioPin, _GPIOP, 3);
   alt_fn!(Pp3, super::sig::U1cts, 1);
   alt_fn!(Pp3, super::sig::U0dcd, 2);
   alt_fn!(Pp3, super::sig::Rtcclk, 7);
   alt_fn!(Pp3, super::sig::Usb0dir, 14);
   alt_fn!(Pp3, super::sig::Epi0s30, 15);

pin!(PP4, Pp4, GPIOP, Gpiop, _PP4, GpioPin, _GPIOP, 4);
   alt_fn!(Pp4, super::sig::U3rts, 1);
   alt_fn!(Pp4, super::sig::U0dsr, 2);
   alt_fn!(Pp4, super::sig::Usb0d7, 14);

pin!(PP5, Pp5, GPIOP, Gpiop, _PP5, GpioPin, _GPIOP, 5);
   alt_fn!(Pp5, super::sig::U3cts, 1);
   alt_fn!(Pp5, super::sig::I2c2scl, 2);
   alt_fn!(Pp5, super::sig::Usb0d6, 14);

pin!(PQ0, Pq0, GPIOQ, Gpioq, _PQ0, GpioPin, _GPIOQ, 0);
   alt_fn!(Pq0, super::sig::Ssi3clk, 14);
   alt_fn!(Pq0, super::sig::Epi0s20, 15);

pin!(PQ1, Pq1, GPIOQ, Gpioq, _PQ1, GpioPin, _GPIOQ, 1);
   alt_fn!(Pq1, super::sig::Ssi3fss, 14);
   alt_fn!(Pq1, super::sig::Epi0s21, 15);

pin!(PQ2, Pq2, GPIOQ, Gpioq, _PQ2, GpioPin, _GPIOQ, 2);
   alt_fn!(Pq2, super::sig::Ssi3xdat0, 14);
   alt_fn!(Pq2, super::sig::Epi0s22, 15);

pin!(PQ3, Pq3, GPIOQ, Gpioq, _PQ3, GpioPin, _GPIOQ, 3);
   alt_fn!(Pq3, super::sig::Ssi3xdat1, 14);
   alt_fn!(Pq3, super::sig::Epi0s23, 15);

pin!(PQ4, Pq4, GPIOQ, Gpioq, _PQ4, GpioPin, _GPIOQ, 4);
   alt_fn!(Pq4, super::sig::U1rx, 1);
   alt_fn!(Pq4, super::sig::Divsclk, 7);

