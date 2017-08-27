//! Timer/Counter
#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "TC", peripherals: [Peripheral { derived_from: None, group_name: None, name: "TC3", address: 1107307520, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Basic Timer Counter 3"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "TC3", types: [], value: 18, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [Channel { name: "TC3_CH0", index: Some(0), description: None, signals: [Signal { name: "TC3_WO_0", types: ["WO"], description: None }], interrupts: [] }, Channel { name: "TC3_CH1", index: Some(1), description: None, signals: [Signal { name: "TC3_WO_1", types: ["WO"], description: None }], interrupts: [] }], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "TC4", address: 1107308544, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Basic Timer Counter 4"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "TC4", types: [], value: 19, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [Channel { name: "TC4_CH0", index: Some(0), description: None, signals: [Signal { name: "TC4_WO_0", types: ["WO"], description: None }], interrupts: [] }, Channel { name: "TC4_CH1", index: Some(1), description: None, signals: [Signal { name: "TC4_WO_1", types: ["WO"], description: None }], interrupts: [] }], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "TC5", address: 1107309568, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Basic Timer Counter 5"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "TC5", types: [], value: 20, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [Channel { name: "TC5_CH0", index: Some(0), description: None, signals: [Signal { name: "TC5_WO_0", types: ["WO"], description: None }], interrupts: [] }, Channel { name: "TC5_CH1", index: Some(1), description: None, signals: [Signal { name: "TC5_WO_1", types: ["WO"], description: None }], interrupts: [] }], dim: None, dim_increment: None, dim_index: None }], prototype: Some(Peripheral { derived_from: None, group_name: Some("TC"), name: "", address: 0, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Basic Timer Counter"), modules: [], features: [], links: [], interrupts: [], clusters: [Cluster { name: "COUNT8", offset: 0, size: None, access: None, reset_value: None, reset_mask: None, description: Some("8-bit Counter Mode"), clusters: [], registers: [Register { name: "CC", offset: 24, size: Some(8), access: None, reset_value: None, reset_mask: None, description: Some("COUNT8 Compare/Capture"), fields: [Field { name: "CC", bit_offset: 0, bit_width: 8, access: None, description: Some("Compare/Capture Value"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: Some(2), dim_increment: Some(1), dim_index: None }, Register { name: "COUNT", offset: 16, size: Some(8), access: None, reset_value: None, reset_mask: None, description: Some("COUNT8 Counter Value"), fields: [Field { name: "COUNT", bit_offset: 0, bit_width: 8, access: None, description: Some("Counter Value"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "PER", offset: 20, size: Some(8), access: None, reset_value: Some(255), reset_mask: None, description: Some("COUNT8 Period Value"), fields: [Field { name: "PER", bit_offset: 0, bit_width: 8, access: None, description: Some("Period Value"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "CTRLA", offset: 0, size: Some(16), access: None, reset_value: None, reset_mask: None, description: Some("Control A"), fields: [Field { name: "SWRST", bit_offset: 0, bit_width: 1, access: Some(WriteOnly), description: Some("Software Reset"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ENABLE", bit_offset: 1, bit_width: 1, access: None, description: Some("Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MODE", bit_offset: 2, bit_width: 2, access: None, description: Some("TC Mode"), enumerated_values: [EnumeratedValue { value: "0x0", name: Some("COUNT16"), description: Some("Counter in 16-bit mode") }, EnumeratedValue { value: "0x1", name: Some("COUNT8"), description: Some("Counter in 8-bit mode") }, EnumeratedValue { value: "0x2", name: Some("COUNT32"), description: Some("Counter in 32-bit mode") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "WAVEGEN", bit_offset: 5, bit_width: 2, access: None, description: Some("Waveform Generation Operation"), enumerated_values: [EnumeratedValue { value: "0x0", name: Some("NFRQ"), description: None }, EnumeratedValue { value: "0x1", name: Some("MFRQ"), description: None }, EnumeratedValue { value: "0x2", name: Some("NPWM"), description: None }, EnumeratedValue { value: "0x3", name: Some("MPWM"), description: None }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "PRESCALER", bit_offset: 8, bit_width: 3, access: None, description: Some("Prescaler"), enumerated_values: [EnumeratedValue { value: "0x0", name: Some("DIV1"), description: Some("Prescaler: GCLK_TC") }, EnumeratedValue { value: "0x1", name: Some("DIV2"), description: Some("Prescaler: GCLK_TC/2") }, EnumeratedValue { value: "0x2", name: Some("DIV4"), description: Some("Prescaler: GCLK_TC/4") }, EnumeratedValue { value: "0x3", name: Some("DIV8"), description: Some("Prescaler: GCLK_TC/8") }, EnumeratedValue { value: "0x4", name: Some("DIV16"), description: Some("Prescaler: GCLK_TC/16") }, EnumeratedValue { value: "0x5", name: Some("DIV64"), description: Some("Prescaler: GCLK_TC/64") }, EnumeratedValue { value: "0x6", name: Some("DIV256"), description: Some("Prescaler: GCLK_TC/256") }, EnumeratedValue { value: "0x7", name: Some("DIV1024"), description: Some("Prescaler: GCLK_TC/1024") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RUNSTDBY", bit_offset: 11, bit_width: 1, access: None, description: Some("Run in Standby"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "PRESCSYNC", bit_offset: 12, bit_width: 2, access: None, description: Some("Prescaler and Counter Synchronization"), enumerated_values: [EnumeratedValue { value: "0x0", name: Some("GCLK"), description: Some("Reload or reset the counter on next generic clock") }, EnumeratedValue { value: "0x1", name: Some("PRESC"), description: Some("Reload or reset the counter on next prescaler clock") }, EnumeratedValue { value: "0x2", name: Some("RESYNC"), description: Some("Reload or reset the counter on next generic clock. Reset the prescaler counter") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "CTRLBCLR", offset: 4, size: Some(8), access: None, reset_value: Some(2), reset_mask: None, description: Some("Control B Clear"), fields: [Field { name: "DIR", bit_offset: 0, bit_width: 1, access: None, description: Some("Counter Direction"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ONESHOT", bit_offset: 2, bit_width: 1, access: None, description: Some("One-Shot"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CMD", bit_offset: 6, bit_width: 2, access: None, description: Some("Command"), enumerated_values: [EnumeratedValue { value: "0x0", name: Some("NONE"), description: Some("No action") }, EnumeratedValue { value: "0x1", name: Some("RETRIGGER"), description: Some("Force a start, restart or retrigger") }, EnumeratedValue { value: "0x2", name: Some("STOP"), description: Some("Force a stop") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "CTRLBSET", offset: 5, size: Some(8), access: None, reset_value: None, reset_mask: None, description: Some("Control B Set"), fields: [Field { name: "DIR", bit_offset: 0, bit_width: 1, access: None, description: Some("Counter Direction"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ONESHOT", bit_offset: 2, bit_width: 1, access: None, description: Some("One-Shot"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CMD", bit_offset: 6, bit_width: 2, access: None, description: Some("Command"), enumerated_values: [EnumeratedValue { value: "0x0", name: Some("NONE"), description: Some("No action") }, EnumeratedValue { value: "0x1", name: Some("RETRIGGER"), description: Some("Force a start, restart or retrigger") }, EnumeratedValue { value: "0x2", name: Some("STOP"), description: Some("Force a stop") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "CTRLC", offset: 6, size: Some(8), access: None, reset_value: None, reset_mask: None, description: Some("Control C"), fields: [Field { name: "INVEN0", bit_offset: 0, bit_width: 1, access: None, description: Some("Output Waveform 0 Invert Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "INVEN1", bit_offset: 1, bit_width: 1, access: None, description: Some("Output Waveform 1 Invert Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CPTEN0", bit_offset: 4, bit_width: 1, access: None, description: Some("Capture Channel 0 Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CPTEN1", bit_offset: 5, bit_width: 1, access: None, description: Some("Capture Channel 1 Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "DBGCTRL", offset: 8, size: Some(8), access: None, reset_value: None, reset_mask: None, description: Some("Debug Control"), fields: [Field { name: "DBGRUN", bit_offset: 0, bit_width: 1, access: None, description: Some("Debug Run Mode"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "EVCTRL", offset: 10, size: Some(16), access: None, reset_value: None, reset_mask: None, description: Some("Event Control"), fields: [Field { name: "EVACT", bit_offset: 0, bit_width: 3, access: None, description: Some("Event Action"), enumerated_values: [EnumeratedValue { value: "0x0", name: Some("OFF"), description: Some("Event action disabled") }, EnumeratedValue { value: "0x1", name: Some("RETRIGGER"), description: Some("Start, restart or retrigger TC on event") }, EnumeratedValue { value: "0x2", name: Some("COUNT"), description: Some("Count on event") }, EnumeratedValue { value: "0x3", name: Some("START"), description: Some("Start TC on event") }, EnumeratedValue { value: "0x5", name: Some("PPW"), description: Some("Period captured in CC0, pulse width in CC1") }, EnumeratedValue { value: "0x6", name: Some("PWP"), description: Some("Period captured in CC1, pulse width in CC0") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TCINV", bit_offset: 4, bit_width: 1, access: None, description: Some("TC Inverted Event Input"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TCEI", bit_offset: 5, bit_width: 1, access: None, description: Some("TC Event Input"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "OVFEO", bit_offset: 8, bit_width: 1, access: None, description: Some("Overflow/Underflow Event Output Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MCEO0", bit_offset: 12, bit_width: 1, access: None, description: Some("Match or Capture Channel 0 Event Output Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MCEO1", bit_offset: 13, bit_width: 1, access: None, description: Some("Match or Capture Channel 1 Event Output Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "INTENCLR", offset: 12, size: Some(8), access: None, reset_value: None, reset_mask: None, description: Some("Interrupt Enable Clear"), fields: [Field { name: "OVF", bit_offset: 0, bit_width: 1, access: None, description: Some("Overflow Interrupt Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ERR", bit_offset: 1, bit_width: 1, access: None, description: Some("Error Interrupt Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SYNCRDY", bit_offset: 3, bit_width: 1, access: None, description: Some("Synchronization Ready Interrupt Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MC0", bit_offset: 4, bit_width: 1, access: None, description: Some("Match or Capture Channel 0 Interrupt Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MC1", bit_offset: 5, bit_width: 1, access: None, description: Some("Match or Capture Channel 1 Interrupt Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "INTENSET", offset: 13, size: Some(8), access: None, reset_value: None, reset_mask: None, description: Some("Interrupt Enable Set"), fields: [Field { name: "OVF", bit_offset: 0, bit_width: 1, access: None, description: Some("Overflow Interrupt Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ERR", bit_offset: 1, bit_width: 1, access: None, description: Some("Error Interrupt Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SYNCRDY", bit_offset: 3, bit_width: 1, access: None, description: Some("Synchronization Ready Interrupt Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MC0", bit_offset: 4, bit_width: 1, access: None, description: Some("Match or Capture Channel 0 Interrupt Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MC1", bit_offset: 5, bit_width: 1, access: None, description: Some("Match or Capture Channel 1 Interrupt Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "INTFLAG", offset: 14, size: Some(8), access: None, reset_value: None, reset_mask: None, description: Some("Interrupt Flag Status and Clear"), fields: [Field { name: "OVF", bit_offset: 0, bit_width: 1, access: None, description: Some("Overflow"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ERR", bit_offset: 1, bit_width: 1, access: None, description: Some("Error"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SYNCRDY", bit_offset: 3, bit_width: 1, access: None, description: Some("Synchronization Ready"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MC0", bit_offset: 4, bit_width: 1, access: None, description: Some("Match or Capture Channel 0"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MC1", bit_offset: 5, bit_width: 1, access: None, description: Some("Match or Capture Channel 1"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "READREQ", offset: 2, size: Some(16), access: None, reset_value: None, reset_mask: None, description: Some("Read Request"), fields: [Field { name: "ADDR", bit_offset: 0, bit_width: 5, access: None, description: Some("Address"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RCONT", bit_offset: 14, bit_width: 1, access: None, description: Some("Read Continuously"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RREQ", bit_offset: 15, bit_width: 1, access: None, description: Some("Read Request"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "STATUS", offset: 15, size: Some(8), access: Some(ReadOnly), reset_value: Some(8), reset_mask: None, description: Some("Status"), fields: [Field { name: "STOP", bit_offset: 3, bit_width: 1, access: Some(ReadOnly), description: Some("Stop"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SLAVE", bit_offset: 4, bit_width: 1, access: Some(ReadOnly), description: Some("Slave"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SYNCBUSY", bit_offset: 7, bit_width: 1, access: Some(ReadOnly), description: Some("Synchronization Busy"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Cluster { name: "COUNT16", offset: 0, size: None, access: None, reset_value: None, reset_mask: None, description: Some("16-bit Counter Mode"), clusters: [], registers: [Register { name: "CC", offset: 24, size: Some(16), access: None, reset_value: None, reset_mask: None, description: Some("COUNT16 Compare/Capture"), fields: [Field { name: "CC", bit_offset: 0, bit_width: 16, access: None, description: Some("Compare/Capture Value"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: Some(2), dim_increment: Some(2), dim_index: None }, Register { name: "COUNT", offset: 16, size: Some(16), access: None, reset_value: None, reset_mask: None, description: Some("COUNT16 Counter Value"), fields: [Field { name: "COUNT", bit_offset: 0, bit_width: 16, access: None, description: Some("Count Value"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "CTRLA", offset: 0, size: Some(16), access: None, reset_value: None, reset_mask: None, description: Some("Control A"), fields: [Field { name: "SWRST", bit_offset: 0, bit_width: 1, access: Some(WriteOnly), description: Some("Software Reset"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ENABLE", bit_offset: 1, bit_width: 1, access: None, description: Some("Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MODE", bit_offset: 2, bit_width: 2, access: None, description: Some("TC Mode"), enumerated_values: [EnumeratedValue { value: "0x0", name: Some("COUNT16"), description: Some("Counter in 16-bit mode") }, EnumeratedValue { value: "0x1", name: Some("COUNT8"), description: Some("Counter in 8-bit mode") }, EnumeratedValue { value: "0x2", name: Some("COUNT32"), description: Some("Counter in 32-bit mode") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "WAVEGEN", bit_offset: 5, bit_width: 2, access: None, description: Some("Waveform Generation Operation"), enumerated_values: [EnumeratedValue { value: "0x0", name: Some("NFRQ"), description: None }, EnumeratedValue { value: "0x1", name: Some("MFRQ"), description: None }, EnumeratedValue { value: "0x2", name: Some("NPWM"), description: None }, EnumeratedValue { value: "0x3", name: Some("MPWM"), description: None }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "PRESCALER", bit_offset: 8, bit_width: 3, access: None, description: Some("Prescaler"), enumerated_values: [EnumeratedValue { value: "0x0", name: Some("DIV1"), description: Some("Prescaler: GCLK_TC") }, EnumeratedValue { value: "0x1", name: Some("DIV2"), description: Some("Prescaler: GCLK_TC/2") }, EnumeratedValue { value: "0x2", name: Some("DIV4"), description: Some("Prescaler: GCLK_TC/4") }, EnumeratedValue { value: "0x3", name: Some("DIV8"), description: Some("Prescaler: GCLK_TC/8") }, EnumeratedValue { value: "0x4", name: Some("DIV16"), description: Some("Prescaler: GCLK_TC/16") }, EnumeratedValue { value: "0x5", name: Some("DIV64"), description: Some("Prescaler: GCLK_TC/64") }, EnumeratedValue { value: "0x6", name: Some("DIV256"), description: Some("Prescaler: GCLK_TC/256") }, EnumeratedValue { value: "0x7", name: Some("DIV1024"), description: Some("Prescaler: GCLK_TC/1024") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RUNSTDBY", bit_offset: 11, bit_width: 1, access: None, description: Some("Run in Standby"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "PRESCSYNC", bit_offset: 12, bit_width: 2, access: None, description: Some("Prescaler and Counter Synchronization"), enumerated_values: [EnumeratedValue { value: "0x0", name: Some("GCLK"), description: Some("Reload or reset the counter on next generic clock") }, EnumeratedValue { value: "0x1", name: Some("PRESC"), description: Some("Reload or reset the counter on next prescaler clock") }, EnumeratedValue { value: "0x2", name: Some("RESYNC"), description: Some("Reload or reset the counter on next generic clock. Reset the prescaler counter") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "CTRLBCLR", offset: 4, size: Some(8), access: None, reset_value: Some(2), reset_mask: None, description: Some("Control B Clear"), fields: [Field { name: "DIR", bit_offset: 0, bit_width: 1, access: None, description: Some("Counter Direction"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ONESHOT", bit_offset: 2, bit_width: 1, access: None, description: Some("One-Shot"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CMD", bit_offset: 6, bit_width: 2, access: None, description: Some("Command"), enumerated_values: [EnumeratedValue { value: "0x0", name: Some("NONE"), description: Some("No action") }, EnumeratedValue { value: "0x1", name: Some("RETRIGGER"), description: Some("Force a start, restart or retrigger") }, EnumeratedValue { value: "0x2", name: Some("STOP"), description: Some("Force a stop") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "CTRLBSET", offset: 5, size: Some(8), access: None, reset_value: None, reset_mask: None, description: Some("Control B Set"), fields: [Field { name: "DIR", bit_offset: 0, bit_width: 1, access: None, description: Some("Counter Direction"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ONESHOT", bit_offset: 2, bit_width: 1, access: None, description: Some("One-Shot"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CMD", bit_offset: 6, bit_width: 2, access: None, description: Some("Command"), enumerated_values: [EnumeratedValue { value: "0x0", name: Some("NONE"), description: Some("No action") }, EnumeratedValue { value: "0x1", name: Some("RETRIGGER"), description: Some("Force a start, restart or retrigger") }, EnumeratedValue { value: "0x2", name: Some("STOP"), description: Some("Force a stop") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "CTRLC", offset: 6, size: Some(8), access: None, reset_value: None, reset_mask: None, description: Some("Control C"), fields: [Field { name: "INVEN0", bit_offset: 0, bit_width: 1, access: None, description: Some("Output Waveform 0 Invert Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "INVEN1", bit_offset: 1, bit_width: 1, access: None, description: Some("Output Waveform 1 Invert Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CPTEN0", bit_offset: 4, bit_width: 1, access: None, description: Some("Capture Channel 0 Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CPTEN1", bit_offset: 5, bit_width: 1, access: None, description: Some("Capture Channel 1 Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "DBGCTRL", offset: 8, size: Some(8), access: None, reset_value: None, reset_mask: None, description: Some("Debug Control"), fields: [Field { name: "DBGRUN", bit_offset: 0, bit_width: 1, access: None, description: Some("Debug Run Mode"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "EVCTRL", offset: 10, size: Some(16), access: None, reset_value: None, reset_mask: None, description: Some("Event Control"), fields: [Field { name: "EVACT", bit_offset: 0, bit_width: 3, access: None, description: Some("Event Action"), enumerated_values: [EnumeratedValue { value: "0x0", name: Some("OFF"), description: Some("Event action disabled") }, EnumeratedValue { value: "0x1", name: Some("RETRIGGER"), description: Some("Start, restart or retrigger TC on event") }, EnumeratedValue { value: "0x2", name: Some("COUNT"), description: Some("Count on event") }, EnumeratedValue { value: "0x3", name: Some("START"), description: Some("Start TC on event") }, EnumeratedValue { value: "0x5", name: Some("PPW"), description: Some("Period captured in CC0, pulse width in CC1") }, EnumeratedValue { value: "0x6", name: Some("PWP"), description: Some("Period captured in CC1, pulse width in CC0") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TCINV", bit_offset: 4, bit_width: 1, access: None, description: Some("TC Inverted Event Input"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TCEI", bit_offset: 5, bit_width: 1, access: None, description: Some("TC Event Input"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "OVFEO", bit_offset: 8, bit_width: 1, access: None, description: Some("Overflow/Underflow Event Output Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MCEO0", bit_offset: 12, bit_width: 1, access: None, description: Some("Match or Capture Channel 0 Event Output Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MCEO1", bit_offset: 13, bit_width: 1, access: None, description: Some("Match or Capture Channel 1 Event Output Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "INTENCLR", offset: 12, size: Some(8), access: None, reset_value: None, reset_mask: None, description: Some("Interrupt Enable Clear"), fields: [Field { name: "OVF", bit_offset: 0, bit_width: 1, access: None, description: Some("Overflow Interrupt Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ERR", bit_offset: 1, bit_width: 1, access: None, description: Some("Error Interrupt Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SYNCRDY", bit_offset: 3, bit_width: 1, access: None, description: Some("Synchronization Ready Interrupt Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MC0", bit_offset: 4, bit_width: 1, access: None, description: Some("Match or Capture Channel 0 Interrupt Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MC1", bit_offset: 5, bit_width: 1, access: None, description: Some("Match or Capture Channel 1 Interrupt Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "INTENSET", offset: 13, size: Some(8), access: None, reset_value: None, reset_mask: None, description: Some("Interrupt Enable Set"), fields: [Field { name: "OVF", bit_offset: 0, bit_width: 1, access: None, description: Some("Overflow Interrupt Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ERR", bit_offset: 1, bit_width: 1, access: None, description: Some("Error Interrupt Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SYNCRDY", bit_offset: 3, bit_width: 1, access: None, description: Some("Synchronization Ready Interrupt Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MC0", bit_offset: 4, bit_width: 1, access: None, description: Some("Match or Capture Channel 0 Interrupt Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MC1", bit_offset: 5, bit_width: 1, access: None, description: Some("Match or Capture Channel 1 Interrupt Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "INTFLAG", offset: 14, size: Some(8), access: None, reset_value: None, reset_mask: None, description: Some("Interrupt Flag Status and Clear"), fields: [Field { name: "OVF", bit_offset: 0, bit_width: 1, access: None, description: Some("Overflow"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ERR", bit_offset: 1, bit_width: 1, access: None, description: Some("Error"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SYNCRDY", bit_offset: 3, bit_width: 1, access: None, description: Some("Synchronization Ready"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MC0", bit_offset: 4, bit_width: 1, access: None, description: Some("Match or Capture Channel 0"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MC1", bit_offset: 5, bit_width: 1, access: None, description: Some("Match or Capture Channel 1"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "READREQ", offset: 2, size: Some(16), access: None, reset_value: None, reset_mask: None, description: Some("Read Request"), fields: [Field { name: "ADDR", bit_offset: 0, bit_width: 5, access: None, description: Some("Address"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RCONT", bit_offset: 14, bit_width: 1, access: None, description: Some("Read Continuously"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RREQ", bit_offset: 15, bit_width: 1, access: None, description: Some("Read Request"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "STATUS", offset: 15, size: Some(8), access: Some(ReadOnly), reset_value: Some(8), reset_mask: None, description: Some("Status"), fields: [Field { name: "STOP", bit_offset: 3, bit_width: 1, access: Some(ReadOnly), description: Some("Stop"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SLAVE", bit_offset: 4, bit_width: 1, access: Some(ReadOnly), description: Some("Slave"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SYNCBUSY", bit_offset: 7, bit_width: 1, access: Some(ReadOnly), description: Some("Synchronization Busy"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Cluster { name: "COUNT32", offset: 0, size: None, access: None, reset_value: None, reset_mask: None, description: Some("32-bit Counter Mode"), clusters: [], registers: [Register { name: "CC", offset: 24, size: Some(32), access: None, reset_value: None, reset_mask: None, description: Some("COUNT32 Compare/Capture"), fields: [Field { name: "CC", bit_offset: 0, bit_width: 32, access: None, description: Some("Compare/Capture Value"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: Some(2), dim_increment: Some(4), dim_index: None }, Register { name: "COUNT", offset: 16, size: Some(32), access: None, reset_value: None, reset_mask: None, description: Some("COUNT32 Counter Value"), fields: [Field { name: "COUNT", bit_offset: 0, bit_width: 32, access: None, description: Some("Count Value"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "CTRLA", offset: 0, size: Some(16), access: None, reset_value: None, reset_mask: None, description: Some("Control A"), fields: [Field { name: "SWRST", bit_offset: 0, bit_width: 1, access: Some(WriteOnly), description: Some("Software Reset"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ENABLE", bit_offset: 1, bit_width: 1, access: None, description: Some("Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MODE", bit_offset: 2, bit_width: 2, access: None, description: Some("TC Mode"), enumerated_values: [EnumeratedValue { value: "0x0", name: Some("COUNT16"), description: Some("Counter in 16-bit mode") }, EnumeratedValue { value: "0x1", name: Some("COUNT8"), description: Some("Counter in 8-bit mode") }, EnumeratedValue { value: "0x2", name: Some("COUNT32"), description: Some("Counter in 32-bit mode") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "WAVEGEN", bit_offset: 5, bit_width: 2, access: None, description: Some("Waveform Generation Operation"), enumerated_values: [EnumeratedValue { value: "0x0", name: Some("NFRQ"), description: None }, EnumeratedValue { value: "0x1", name: Some("MFRQ"), description: None }, EnumeratedValue { value: "0x2", name: Some("NPWM"), description: None }, EnumeratedValue { value: "0x3", name: Some("MPWM"), description: None }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "PRESCALER", bit_offset: 8, bit_width: 3, access: None, description: Some("Prescaler"), enumerated_values: [EnumeratedValue { value: "0x0", name: Some("DIV1"), description: Some("Prescaler: GCLK_TC") }, EnumeratedValue { value: "0x1", name: Some("DIV2"), description: Some("Prescaler: GCLK_TC/2") }, EnumeratedValue { value: "0x2", name: Some("DIV4"), description: Some("Prescaler: GCLK_TC/4") }, EnumeratedValue { value: "0x3", name: Some("DIV8"), description: Some("Prescaler: GCLK_TC/8") }, EnumeratedValue { value: "0x4", name: Some("DIV16"), description: Some("Prescaler: GCLK_TC/16") }, EnumeratedValue { value: "0x5", name: Some("DIV64"), description: Some("Prescaler: GCLK_TC/64") }, EnumeratedValue { value: "0x6", name: Some("DIV256"), description: Some("Prescaler: GCLK_TC/256") }, EnumeratedValue { value: "0x7", name: Some("DIV1024"), description: Some("Prescaler: GCLK_TC/1024") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RUNSTDBY", bit_offset: 11, bit_width: 1, access: None, description: Some("Run in Standby"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "PRESCSYNC", bit_offset: 12, bit_width: 2, access: None, description: Some("Prescaler and Counter Synchronization"), enumerated_values: [EnumeratedValue { value: "0x0", name: Some("GCLK"), description: Some("Reload or reset the counter on next generic clock") }, EnumeratedValue { value: "0x1", name: Some("PRESC"), description: Some("Reload or reset the counter on next prescaler clock") }, EnumeratedValue { value: "0x2", name: Some("RESYNC"), description: Some("Reload or reset the counter on next generic clock. Reset the prescaler counter") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "CTRLBCLR", offset: 4, size: Some(8), access: None, reset_value: Some(2), reset_mask: None, description: Some("Control B Clear"), fields: [Field { name: "DIR", bit_offset: 0, bit_width: 1, access: None, description: Some("Counter Direction"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ONESHOT", bit_offset: 2, bit_width: 1, access: None, description: Some("One-Shot"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CMD", bit_offset: 6, bit_width: 2, access: None, description: Some("Command"), enumerated_values: [EnumeratedValue { value: "0x0", name: Some("NONE"), description: Some("No action") }, EnumeratedValue { value: "0x1", name: Some("RETRIGGER"), description: Some("Force a start, restart or retrigger") }, EnumeratedValue { value: "0x2", name: Some("STOP"), description: Some("Force a stop") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "CTRLBSET", offset: 5, size: Some(8), access: None, reset_value: None, reset_mask: None, description: Some("Control B Set"), fields: [Field { name: "DIR", bit_offset: 0, bit_width: 1, access: None, description: Some("Counter Direction"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ONESHOT", bit_offset: 2, bit_width: 1, access: None, description: Some("One-Shot"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CMD", bit_offset: 6, bit_width: 2, access: None, description: Some("Command"), enumerated_values: [EnumeratedValue { value: "0x0", name: Some("NONE"), description: Some("No action") }, EnumeratedValue { value: "0x1", name: Some("RETRIGGER"), description: Some("Force a start, restart or retrigger") }, EnumeratedValue { value: "0x2", name: Some("STOP"), description: Some("Force a stop") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "CTRLC", offset: 6, size: Some(8), access: None, reset_value: None, reset_mask: None, description: Some("Control C"), fields: [Field { name: "INVEN0", bit_offset: 0, bit_width: 1, access: None, description: Some("Output Waveform 0 Invert Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "INVEN1", bit_offset: 1, bit_width: 1, access: None, description: Some("Output Waveform 1 Invert Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CPTEN0", bit_offset: 4, bit_width: 1, access: None, description: Some("Capture Channel 0 Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CPTEN1", bit_offset: 5, bit_width: 1, access: None, description: Some("Capture Channel 1 Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "DBGCTRL", offset: 8, size: Some(8), access: None, reset_value: None, reset_mask: None, description: Some("Debug Control"), fields: [Field { name: "DBGRUN", bit_offset: 0, bit_width: 1, access: None, description: Some("Debug Run Mode"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "EVCTRL", offset: 10, size: Some(16), access: None, reset_value: None, reset_mask: None, description: Some("Event Control"), fields: [Field { name: "EVACT", bit_offset: 0, bit_width: 3, access: None, description: Some("Event Action"), enumerated_values: [EnumeratedValue { value: "0x0", name: Some("OFF"), description: Some("Event action disabled") }, EnumeratedValue { value: "0x1", name: Some("RETRIGGER"), description: Some("Start, restart or retrigger TC on event") }, EnumeratedValue { value: "0x2", name: Some("COUNT"), description: Some("Count on event") }, EnumeratedValue { value: "0x3", name: Some("START"), description: Some("Start TC on event") }, EnumeratedValue { value: "0x5", name: Some("PPW"), description: Some("Period captured in CC0, pulse width in CC1") }, EnumeratedValue { value: "0x6", name: Some("PWP"), description: Some("Period captured in CC1, pulse width in CC0") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TCINV", bit_offset: 4, bit_width: 1, access: None, description: Some("TC Inverted Event Input"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TCEI", bit_offset: 5, bit_width: 1, access: None, description: Some("TC Event Input"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "OVFEO", bit_offset: 8, bit_width: 1, access: None, description: Some("Overflow/Underflow Event Output Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MCEO0", bit_offset: 12, bit_width: 1, access: None, description: Some("Match or Capture Channel 0 Event Output Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MCEO1", bit_offset: 13, bit_width: 1, access: None, description: Some("Match or Capture Channel 1 Event Output Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "INTENCLR", offset: 12, size: Some(8), access: None, reset_value: None, reset_mask: None, description: Some("Interrupt Enable Clear"), fields: [Field { name: "OVF", bit_offset: 0, bit_width: 1, access: None, description: Some("Overflow Interrupt Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ERR", bit_offset: 1, bit_width: 1, access: None, description: Some("Error Interrupt Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SYNCRDY", bit_offset: 3, bit_width: 1, access: None, description: Some("Synchronization Ready Interrupt Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MC0", bit_offset: 4, bit_width: 1, access: None, description: Some("Match or Capture Channel 0 Interrupt Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MC1", bit_offset: 5, bit_width: 1, access: None, description: Some("Match or Capture Channel 1 Interrupt Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "INTENSET", offset: 13, size: Some(8), access: None, reset_value: None, reset_mask: None, description: Some("Interrupt Enable Set"), fields: [Field { name: "OVF", bit_offset: 0, bit_width: 1, access: None, description: Some("Overflow Interrupt Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ERR", bit_offset: 1, bit_width: 1, access: None, description: Some("Error Interrupt Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SYNCRDY", bit_offset: 3, bit_width: 1, access: None, description: Some("Synchronization Ready Interrupt Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MC0", bit_offset: 4, bit_width: 1, access: None, description: Some("Match or Capture Channel 0 Interrupt Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MC1", bit_offset: 5, bit_width: 1, access: None, description: Some("Match or Capture Channel 1 Interrupt Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "INTFLAG", offset: 14, size: Some(8), access: None, reset_value: None, reset_mask: None, description: Some("Interrupt Flag Status and Clear"), fields: [Field { name: "OVF", bit_offset: 0, bit_width: 1, access: None, description: Some("Overflow"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ERR", bit_offset: 1, bit_width: 1, access: None, description: Some("Error"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SYNCRDY", bit_offset: 3, bit_width: 1, access: None, description: Some("Synchronization Ready"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MC0", bit_offset: 4, bit_width: 1, access: None, description: Some("Match or Capture Channel 0"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MC1", bit_offset: 5, bit_width: 1, access: None, description: Some("Match or Capture Channel 1"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "READREQ", offset: 2, size: Some(16), access: None, reset_value: None, reset_mask: None, description: Some("Read Request"), fields: [Field { name: "ADDR", bit_offset: 0, bit_width: 5, access: None, description: Some("Address"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RCONT", bit_offset: 14, bit_width: 1, access: None, description: Some("Read Continuously"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RREQ", bit_offset: 15, bit_width: 1, access: None, description: Some("Read Request"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "STATUS", offset: 15, size: Some(8), access: Some(ReadOnly), reset_value: Some(8), reset_mask: None, description: Some("Status"), fields: [Field { name: "STOP", bit_offset: 3, bit_width: 1, access: Some(ReadOnly), description: Some("Stop"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SLAVE", bit_offset: 4, bit_width: 1, access: Some(ReadOnly), description: Some("Slave"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SYNCBUSY", bit_offset: 7, bit_width: 1, access: Some(ReadOnly), description: Some("Synchronization Busy"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }], registers: [], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }), modules: [], has_pins: false, has_channels: true, description: Some("Timer/Counter") }
periph!( TC3, Tc3, _TC3, TcPeriph, 0x42002c00);
periph!( TC4, Tc4, _TC4, TcPeriph, 0x42003000);
periph!( TC5, Tc5, _TC5, TcPeriph, 0x42003400);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="TC Peripheral"]
pub struct TcPeriph(pub usize); 

impl super::sig::Signal<super::sig::Tc3Wo0> for Tc3Ch0 {}
impl super::sig::SignalWo<super::sig::Tc3Wo0> for Tc3Ch0 {}
impl super::sig::Signal<super::sig::Tc3Wo1> for Tc3Ch1 {}
impl super::sig::SignalWo<super::sig::Tc3Wo1> for Tc3Ch1 {}

impl super::sig::Signal<super::sig::Tc4Wo0> for Tc4Ch0 {}
impl super::sig::SignalWo<super::sig::Tc4Wo0> for Tc4Ch0 {}
impl super::sig::Signal<super::sig::Tc4Wo1> for Tc4Ch1 {}
impl super::sig::SignalWo<super::sig::Tc4Wo1> for Tc4Ch1 {}

impl super::sig::Signal<super::sig::Tc5Wo0> for Tc5Ch0 {}
impl super::sig::SignalWo<super::sig::Tc5Wo0> for Tc5Ch0 {}
impl super::sig::Signal<super::sig::Tc5Wo1> for Tc5Ch1 {}
impl super::sig::SignalWo<super::sig::Tc5Wo1> for Tc5Ch1 {}


impl TcPeriph {
#[doc="Get 8-bit Counter Mode Peripheral"]
   #[inline] pub fn count8(&self) -> count8::Count8 {
      count8::Count8(self.0 + 0x0)
   }
#[doc="Get 16-bit Counter Mode Peripheral"]
   #[inline] pub fn count16(&self) -> count16::Count16 {
      count16::Count16(self.0 + 0x0)
   }
#[doc="Get 32-bit Counter Mode Peripheral"]
   #[inline] pub fn count32(&self) -> count32::Count32 {
      count32::Count32(self.0 + 0x0)
   }
}
#[doc="8-bit Counter Mode Cluster"]
pub mod count8 {
#[allow(unused_imports)] use bobbin_common::*;
   #[derive(Clone, Copy, PartialEq, Eq)]
#[doc="8-bit Counter Mode Peripheral"]
   pub struct Count8(pub usize);
impl Count8 {
#[doc="Get the *const pointer for the CC register."]
   #[inline] pub fn cc_ptr<I: Into<bits::R2>>(&self, index: I) -> *const u8 { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x18 + (index)) as *const u8
   }
#[doc="Get the *mut pointer for the CC register."]
   #[inline] pub fn cc_mut<I: Into<bits::R2>>(&self, index: I) -> *mut u8 { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x18 + (index)) as *mut u8
   }
#[doc="Read the CC register."]
   #[inline] pub fn cc<I: Into<bits::R2>>(&self, index: I) -> Cc { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         Cc(::core::ptr::read_volatile((self.0 + 0x18 + (index)) as *const u8))
      }
   }
#[doc="Write the CC register."]
   #[inline] pub fn set_cc<I: Into<bits::R2>, F: FnOnce(Cc) -> Cc>(&self, index: I, f: F) -> &Self {
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      let value = f(Cc(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x18 + (index)) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the CC register."]
   #[inline] pub fn with_cc<I: Into<bits::R2> + Copy, F: FnOnce(Cc) -> Cc>(&self, index: I, f: F) -> &Self {
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.cc(index);
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x18 + (index)) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the COUNT register."]
   #[inline] pub fn count_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x10) as *const u8
   }
#[doc="Get the *mut pointer for the COUNT register."]
   #[inline] pub fn count_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x10) as *mut u8
   }
#[doc="Read the COUNT register."]
   #[inline] pub fn count(&self) -> Count { 
      unsafe {
         Count(::core::ptr::read_volatile((self.0 + 0x10) as *const u8))
      }
   }
#[doc="Write the COUNT register."]
   #[inline] pub fn set_count<F: FnOnce(Count) -> Count>(&self, f: F) -> &Self {
      let value = f(Count(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x10) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the COUNT register."]
   #[inline] pub fn with_count<F: FnOnce(Count) -> Count>(&self, f: F) -> &Self {
      let tmp = self.count();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x10) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PER register."]
   #[inline] pub fn per_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x14) as *const u8
   }
#[doc="Get the *mut pointer for the PER register."]
   #[inline] pub fn per_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x14) as *mut u8
   }
#[doc="Read the PER register."]
   #[inline] pub fn per(&self) -> Per { 
      unsafe {
         Per(::core::ptr::read_volatile((self.0 + 0x14) as *const u8))
      }
   }
#[doc="Write the PER register."]
   #[inline] pub fn set_per<F: FnOnce(Per) -> Per>(&self, f: F) -> &Self {
      let value = f(Per(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x14) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the PER register."]
   #[inline] pub fn with_per<F: FnOnce(Per) -> Per>(&self, f: F) -> &Self {
      let tmp = self.per();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x14) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CTRLA register."]
   #[inline] pub fn ctrla_ptr(&self) -> *const u16 { 
      ((self.0 as usize) + 0x0) as *const u16
   }
#[doc="Get the *mut pointer for the CTRLA register."]
   #[inline] pub fn ctrla_mut(&self) -> *mut u16 { 
      ((self.0 as usize) + 0x0) as *mut u16
   }
#[doc="Read the CTRLA register."]
   #[inline] pub fn ctrla(&self) -> Ctrla { 
      unsafe {
         Ctrla(::core::ptr::read_volatile((self.0 + 0x0) as *const u16))
      }
   }
#[doc="Write the CTRLA register."]
   #[inline] pub fn set_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
      let value = f(Ctrla(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x0) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the CTRLA register."]
   #[inline] pub fn with_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
      let tmp = self.ctrla();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x0) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CTRLBCLR register."]
   #[inline] pub fn ctrlbclr_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x4) as *const u8
   }
#[doc="Get the *mut pointer for the CTRLBCLR register."]
   #[inline] pub fn ctrlbclr_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x4) as *mut u8
   }
#[doc="Read the CTRLBCLR register."]
   #[inline] pub fn ctrlbclr(&self) -> Ctrlbclr { 
      unsafe {
         Ctrlbclr(::core::ptr::read_volatile((self.0 + 0x4) as *const u8))
      }
   }
#[doc="Write the CTRLBCLR register."]
   #[inline] pub fn set_ctrlbclr<F: FnOnce(Ctrlbclr) -> Ctrlbclr>(&self, f: F) -> &Self {
      let value = f(Ctrlbclr(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x4) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the CTRLBCLR register."]
   #[inline] pub fn with_ctrlbclr<F: FnOnce(Ctrlbclr) -> Ctrlbclr>(&self, f: F) -> &Self {
      let tmp = self.ctrlbclr();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x4) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CTRLBSET register."]
   #[inline] pub fn ctrlbset_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x5) as *const u8
   }
#[doc="Get the *mut pointer for the CTRLBSET register."]
   #[inline] pub fn ctrlbset_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x5) as *mut u8
   }
#[doc="Read the CTRLBSET register."]
   #[inline] pub fn ctrlbset(&self) -> Ctrlbset { 
      unsafe {
         Ctrlbset(::core::ptr::read_volatile((self.0 + 0x5) as *const u8))
      }
   }
#[doc="Write the CTRLBSET register."]
   #[inline] pub fn set_ctrlbset<F: FnOnce(Ctrlbset) -> Ctrlbset>(&self, f: F) -> &Self {
      let value = f(Ctrlbset(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x5) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the CTRLBSET register."]
   #[inline] pub fn with_ctrlbset<F: FnOnce(Ctrlbset) -> Ctrlbset>(&self, f: F) -> &Self {
      let tmp = self.ctrlbset();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x5) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CTRLC register."]
   #[inline] pub fn ctrlc_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x6) as *const u8
   }
#[doc="Get the *mut pointer for the CTRLC register."]
   #[inline] pub fn ctrlc_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x6) as *mut u8
   }
#[doc="Read the CTRLC register."]
   #[inline] pub fn ctrlc(&self) -> Ctrlc { 
      unsafe {
         Ctrlc(::core::ptr::read_volatile((self.0 + 0x6) as *const u8))
      }
   }
#[doc="Write the CTRLC register."]
   #[inline] pub fn set_ctrlc<F: FnOnce(Ctrlc) -> Ctrlc>(&self, f: F) -> &Self {
      let value = f(Ctrlc(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x6) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the CTRLC register."]
   #[inline] pub fn with_ctrlc<F: FnOnce(Ctrlc) -> Ctrlc>(&self, f: F) -> &Self {
      let tmp = self.ctrlc();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x6) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DBGCTRL register."]
   #[inline] pub fn dbgctrl_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x8) as *const u8
   }
#[doc="Get the *mut pointer for the DBGCTRL register."]
   #[inline] pub fn dbgctrl_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x8) as *mut u8
   }
#[doc="Read the DBGCTRL register."]
   #[inline] pub fn dbgctrl(&self) -> Dbgctrl { 
      unsafe {
         Dbgctrl(::core::ptr::read_volatile((self.0 + 0x8) as *const u8))
      }
   }
#[doc="Write the DBGCTRL register."]
   #[inline] pub fn set_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
      let value = f(Dbgctrl(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x8) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the DBGCTRL register."]
   #[inline] pub fn with_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
      let tmp = self.dbgctrl();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x8) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the EVCTRL register."]
   #[inline] pub fn evctrl_ptr(&self) -> *const u16 { 
      ((self.0 as usize) + 0xa) as *const u16
   }
#[doc="Get the *mut pointer for the EVCTRL register."]
   #[inline] pub fn evctrl_mut(&self) -> *mut u16 { 
      ((self.0 as usize) + 0xa) as *mut u16
   }
#[doc="Read the EVCTRL register."]
   #[inline] pub fn evctrl(&self) -> Evctrl { 
      unsafe {
         Evctrl(::core::ptr::read_volatile((self.0 + 0xa) as *const u16))
      }
   }
#[doc="Write the EVCTRL register."]
   #[inline] pub fn set_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
      let value = f(Evctrl(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xa) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the EVCTRL register."]
   #[inline] pub fn with_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
      let tmp = self.evctrl();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xa) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the INTENCLR register."]
   #[inline] pub fn intenclr_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0xc) as *const u8
   }
#[doc="Get the *mut pointer for the INTENCLR register."]
   #[inline] pub fn intenclr_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0xc) as *mut u8
   }
#[doc="Read the INTENCLR register."]
   #[inline] pub fn intenclr(&self) -> Intenclr { 
      unsafe {
         Intenclr(::core::ptr::read_volatile((self.0 + 0xc) as *const u8))
      }
   }
#[doc="Write the INTENCLR register."]
   #[inline] pub fn set_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
      let value = f(Intenclr(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xc) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the INTENCLR register."]
   #[inline] pub fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
      let tmp = self.intenclr();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xc) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the INTENSET register."]
   #[inline] pub fn intenset_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0xd) as *const u8
   }
#[doc="Get the *mut pointer for the INTENSET register."]
   #[inline] pub fn intenset_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0xd) as *mut u8
   }
#[doc="Read the INTENSET register."]
   #[inline] pub fn intenset(&self) -> Intenset { 
      unsafe {
         Intenset(::core::ptr::read_volatile((self.0 + 0xd) as *const u8))
      }
   }
#[doc="Write the INTENSET register."]
   #[inline] pub fn set_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
      let value = f(Intenset(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xd) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the INTENSET register."]
   #[inline] pub fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
      let tmp = self.intenset();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xd) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the INTFLAG register."]
   #[inline] pub fn intflag_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0xe) as *const u8
   }
#[doc="Get the *mut pointer for the INTFLAG register."]
   #[inline] pub fn intflag_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0xe) as *mut u8
   }
#[doc="Read the INTFLAG register."]
   #[inline] pub fn intflag(&self) -> Intflag { 
      unsafe {
         Intflag(::core::ptr::read_volatile((self.0 + 0xe) as *const u8))
      }
   }
#[doc="Write the INTFLAG register."]
   #[inline] pub fn set_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
      let value = f(Intflag(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xe) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the INTFLAG register."]
   #[inline] pub fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
      let tmp = self.intflag();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xe) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the READREQ register."]
   #[inline] pub fn readreq_ptr(&self) -> *const u16 { 
      ((self.0 as usize) + 0x2) as *const u16
   }
#[doc="Get the *mut pointer for the READREQ register."]
   #[inline] pub fn readreq_mut(&self) -> *mut u16 { 
      ((self.0 as usize) + 0x2) as *mut u16
   }
#[doc="Read the READREQ register."]
   #[inline] pub fn readreq(&self) -> Readreq { 
      unsafe {
         Readreq(::core::ptr::read_volatile((self.0 + 0x2) as *const u16))
      }
   }
#[doc="Write the READREQ register."]
   #[inline] pub fn set_readreq<F: FnOnce(Readreq) -> Readreq>(&self, f: F) -> &Self {
      let value = f(Readreq(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x2) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the READREQ register."]
   #[inline] pub fn with_readreq<F: FnOnce(Readreq) -> Readreq>(&self, f: F) -> &Self {
      let tmp = self.readreq();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x2) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the STATUS register."]
   #[inline] pub fn status_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0xf) as *const u8
   }
#[doc="Get the *mut pointer for the STATUS register."]
   #[inline] pub fn status_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0xf) as *mut u8
   }
#[doc="Read the STATUS register."]
   #[inline] pub fn status(&self) -> Status { 
      unsafe {
         Status(::core::ptr::read_volatile((self.0 + 0xf) as *const u8))
      }
   }

}

#[doc="COUNT8 Compare/Capture"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cc(pub u8);
impl Cc {
#[doc="Compare/Capture Value"]
   #[inline] pub fn cc(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Compare/Capture Value"]
   #[inline] pub fn set_cc<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Cc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cc() != 0 { try!(write!(f, " cc=0x{:x}", self.cc()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="COUNT8 Counter Value"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Count(pub u8);
impl Count {
#[doc="Counter Value"]
   #[inline] pub fn count(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Counter Value"]
   #[inline] pub fn set_count<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Count {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Count {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="COUNT8 Period Value"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Per(pub u8);
impl Per {
#[doc="Period Value"]
   #[inline] pub fn per(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Period Value"]
   #[inline] pub fn set_per<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Per {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Per {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.per() != 0 { try!(write!(f, " per=0x{:x}", self.per()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Control A"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ctrla(pub u16);
impl Ctrla {
#[doc="Software Reset"]
   #[inline] pub fn swrst(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Software Reset"]
   #[inline] pub fn set_swrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Enable"]
   #[inline] pub fn enable(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Enable"]
   #[inline] pub fn set_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="TC Mode"]
   #[inline] pub fn mode(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
   }
#[doc="TC Mode"]
   #[inline] pub fn set_mode<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x3 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Waveform Generation Operation"]
   #[inline] pub fn wavegen(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
   }
#[doc="Waveform Generation Operation"]
   #[inline] pub fn set_wavegen<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x3 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Prescaler"]
   #[inline] pub fn prescaler(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
   }
#[doc="Prescaler"]
   #[inline] pub fn set_prescaler<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x7 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Run in Standby"]
   #[inline] pub fn runstdby(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
   }
#[doc="Run in Standby"]
   #[inline] pub fn set_runstdby<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 11);
      self.0 |= value << 11;
      self
   }

#[doc="Prescaler and Counter Synchronization"]
   #[inline] pub fn prescsync(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
   }
#[doc="Prescaler and Counter Synchronization"]
   #[inline] pub fn set_prescsync<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x3 << 12);
      self.0 |= value << 12;
      self
   }

}
impl ::core::fmt::Display for Ctrla {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctrla {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.swrst() != 0 { try!(write!(f, " swrst"))}
      if self.enable() != 0 { try!(write!(f, " enable"))}
      if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
      if self.wavegen() != 0 { try!(write!(f, " wavegen=0x{:x}", self.wavegen()))}
      if self.prescaler() != 0 { try!(write!(f, " prescaler=0x{:x}", self.prescaler()))}
      if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
      if self.prescsync() != 0 { try!(write!(f, " prescsync=0x{:x}", self.prescsync()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Control B Clear"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ctrlbclr(pub u8);
impl Ctrlbclr {
#[doc="Counter Direction"]
   #[inline] pub fn dir(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Counter Direction"]
   #[inline] pub fn set_dir<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="One-Shot"]
   #[inline] pub fn oneshot(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="One-Shot"]
   #[inline] pub fn set_oneshot<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Command"]
   #[inline] pub fn cmd(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
   }
#[doc="Command"]
   #[inline] pub fn set_cmd<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x3 << 6);
      self.0 |= value << 6;
      self
   }

}
impl ::core::fmt::Display for Ctrlbclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctrlbclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dir() != 0 { try!(write!(f, " dir"))}
      if self.oneshot() != 0 { try!(write!(f, " oneshot"))}
      if self.cmd() != 0 { try!(write!(f, " cmd=0x{:x}", self.cmd()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Control B Set"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ctrlbset(pub u8);
impl Ctrlbset {
#[doc="Counter Direction"]
   #[inline] pub fn dir(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Counter Direction"]
   #[inline] pub fn set_dir<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="One-Shot"]
   #[inline] pub fn oneshot(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="One-Shot"]
   #[inline] pub fn set_oneshot<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Command"]
   #[inline] pub fn cmd(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
   }
#[doc="Command"]
   #[inline] pub fn set_cmd<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x3 << 6);
      self.0 |= value << 6;
      self
   }

}
impl ::core::fmt::Display for Ctrlbset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctrlbset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dir() != 0 { try!(write!(f, " dir"))}
      if self.oneshot() != 0 { try!(write!(f, " oneshot"))}
      if self.cmd() != 0 { try!(write!(f, " cmd=0x{:x}", self.cmd()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Control C"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ctrlc(pub u8);
impl Ctrlc {
#[doc="Output Waveform 0 Invert Enable"]
   #[inline] pub fn inven0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Output Waveform 0 Invert Enable"]
   #[inline] pub fn set_inven0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Output Waveform 1 Invert Enable"]
   #[inline] pub fn inven1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Output Waveform 1 Invert Enable"]
   #[inline] pub fn set_inven1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Capture Channel 0 Enable"]
   #[inline] pub fn cpten0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Capture Channel 0 Enable"]
   #[inline] pub fn set_cpten0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Capture Channel 1 Enable"]
   #[inline] pub fn cpten1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Capture Channel 1 Enable"]
   #[inline] pub fn set_cpten1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

}
impl ::core::fmt::Display for Ctrlc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctrlc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.inven0() != 0 { try!(write!(f, " inven0"))}
      if self.inven1() != 0 { try!(write!(f, " inven1"))}
      if self.cpten0() != 0 { try!(write!(f, " cpten0"))}
      if self.cpten1() != 0 { try!(write!(f, " cpten1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Debug Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
#[doc="Debug Run Mode"]
   #[inline] pub fn dbgrun(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Debug Run Mode"]
   #[inline] pub fn set_dbgrun<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Dbgctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dbgctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dbgrun() != 0 { try!(write!(f, " dbgrun"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Event Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Evctrl(pub u16);
impl Evctrl {
#[doc="Event Action"]
   #[inline] pub fn evact(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
   }
#[doc="Event Action"]
   #[inline] pub fn set_evact<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x7 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="TC Inverted Event Input"]
   #[inline] pub fn tcinv(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="TC Inverted Event Input"]
   #[inline] pub fn set_tcinv<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="TC Event Input"]
   #[inline] pub fn tcei(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="TC Event Input"]
   #[inline] pub fn set_tcei<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Overflow/Underflow Event Output Enable"]
   #[inline] pub fn ovfeo(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="Overflow/Underflow Event Output Enable"]
   #[inline] pub fn set_ovfeo<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Match or Capture Channel 0 Event Output Enable"]
   #[inline] pub fn mceo0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
   }
#[doc="Match or Capture Channel 0 Event Output Enable"]
   #[inline] pub fn set_mceo0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 12);
      self.0 |= value << 12;
      self
   }

#[doc="Match or Capture Channel 1 Event Output Enable"]
   #[inline] pub fn mceo1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
   }
#[doc="Match or Capture Channel 1 Event Output Enable"]
   #[inline] pub fn set_mceo1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 13);
      self.0 |= value << 13;
      self
   }

}
impl ::core::fmt::Display for Evctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Evctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.evact() != 0 { try!(write!(f, " evact=0x{:x}", self.evact()))}
      if self.tcinv() != 0 { try!(write!(f, " tcinv"))}
      if self.tcei() != 0 { try!(write!(f, " tcei"))}
      if self.ovfeo() != 0 { try!(write!(f, " ovfeo"))}
      if self.mceo0() != 0 { try!(write!(f, " mceo0"))}
      if self.mceo1() != 0 { try!(write!(f, " mceo1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Interrupt Enable Clear"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
#[doc="Overflow Interrupt Enable"]
   #[inline] pub fn ovf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Overflow Interrupt Enable"]
   #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Error Interrupt Enable"]
   #[inline] pub fn err(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Error Interrupt Enable"]
   #[inline] pub fn set_err<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Synchronization Ready Interrupt Enable"]
   #[inline] pub fn syncrdy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Synchronization Ready Interrupt Enable"]
   #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Match or Capture Channel 0 Interrupt Enable"]
   #[inline] pub fn mc0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Match or Capture Channel 0 Interrupt Enable"]
   #[inline] pub fn set_mc0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Match or Capture Channel 1 Interrupt Enable"]
   #[inline] pub fn mc1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Match or Capture Channel 1 Interrupt Enable"]
   #[inline] pub fn set_mc1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

}
impl ::core::fmt::Display for Intenclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Intenclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ovf() != 0 { try!(write!(f, " ovf"))}
      if self.err() != 0 { try!(write!(f, " err"))}
      if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
      if self.mc0() != 0 { try!(write!(f, " mc0"))}
      if self.mc1() != 0 { try!(write!(f, " mc1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Interrupt Enable Set"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
#[doc="Overflow Interrupt Enable"]
   #[inline] pub fn ovf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Overflow Interrupt Enable"]
   #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Error Interrupt Enable"]
   #[inline] pub fn err(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Error Interrupt Enable"]
   #[inline] pub fn set_err<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Synchronization Ready Interrupt Enable"]
   #[inline] pub fn syncrdy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Synchronization Ready Interrupt Enable"]
   #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Match or Capture Channel 0 Interrupt Enable"]
   #[inline] pub fn mc0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Match or Capture Channel 0 Interrupt Enable"]
   #[inline] pub fn set_mc0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Match or Capture Channel 1 Interrupt Enable"]
   #[inline] pub fn mc1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Match or Capture Channel 1 Interrupt Enable"]
   #[inline] pub fn set_mc1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

}
impl ::core::fmt::Display for Intenset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Intenset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ovf() != 0 { try!(write!(f, " ovf"))}
      if self.err() != 0 { try!(write!(f, " err"))}
      if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
      if self.mc0() != 0 { try!(write!(f, " mc0"))}
      if self.mc1() != 0 { try!(write!(f, " mc1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Interrupt Flag Status and Clear"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
#[doc="Overflow"]
   #[inline] pub fn ovf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Overflow"]
   #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Error"]
   #[inline] pub fn err(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Error"]
   #[inline] pub fn set_err<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Synchronization Ready"]
   #[inline] pub fn syncrdy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Synchronization Ready"]
   #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Match or Capture Channel 0"]
   #[inline] pub fn mc0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Match or Capture Channel 0"]
   #[inline] pub fn set_mc0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Match or Capture Channel 1"]
   #[inline] pub fn mc1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Match or Capture Channel 1"]
   #[inline] pub fn set_mc1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

}
impl ::core::fmt::Display for Intflag {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Intflag {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ovf() != 0 { try!(write!(f, " ovf"))}
      if self.err() != 0 { try!(write!(f, " err"))}
      if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
      if self.mc0() != 0 { try!(write!(f, " mc0"))}
      if self.mc1() != 0 { try!(write!(f, " mc1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Read Request"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Readreq(pub u16);
impl Readreq {
#[doc="Address"]
   #[inline] pub fn addr(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
   }
#[doc="Address"]
   #[inline] pub fn set_addr<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1f << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Read Continuously"]
   #[inline] pub fn rcont(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
   }
#[doc="Read Continuously"]
   #[inline] pub fn set_rcont<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 14);
      self.0 |= value << 14;
      self
   }

#[doc="Read Request"]
   #[inline] pub fn rreq(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
   }
#[doc="Read Request"]
   #[inline] pub fn set_rreq<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 15);
      self.0 |= value << 15;
      self
   }

}
impl ::core::fmt::Display for Readreq {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Readreq {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.addr() != 0 { try!(write!(f, " addr=0x{:x}", self.addr()))}
      if self.rcont() != 0 { try!(write!(f, " rcont"))}
      if self.rreq() != 0 { try!(write!(f, " rreq"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Status"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u8);
impl Status {
#[doc="Stop"]
   #[inline] pub fn stop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Stop"]
   #[inline] pub fn set_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Slave"]
   #[inline] pub fn slave(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Slave"]
   #[inline] pub fn set_slave<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Synchronization Busy"]
   #[inline] pub fn syncbusy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Synchronization Busy"]
   #[inline] pub fn set_syncbusy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Status {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Status {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.stop() != 0 { try!(write!(f, " stop"))}
      if self.slave() != 0 { try!(write!(f, " slave"))}
      if self.syncbusy() != 0 { try!(write!(f, " syncbusy"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
}
// End of count8
#[doc="16-bit Counter Mode Cluster"]
pub mod count16 {
#[allow(unused_imports)] use bobbin_common::*;
   #[derive(Clone, Copy, PartialEq, Eq)]
#[doc="16-bit Counter Mode Peripheral"]
   pub struct Count16(pub usize);
impl Count16 {
#[doc="Get the *const pointer for the CC register."]
   #[inline] pub fn cc_ptr<I: Into<bits::R2>>(&self, index: I) -> *const u16 { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x18 + (index << 1)) as *const u16
   }
#[doc="Get the *mut pointer for the CC register."]
   #[inline] pub fn cc_mut<I: Into<bits::R2>>(&self, index: I) -> *mut u16 { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x18 + (index << 1)) as *mut u16
   }
#[doc="Read the CC register."]
   #[inline] pub fn cc<I: Into<bits::R2>>(&self, index: I) -> Cc { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         Cc(::core::ptr::read_volatile((self.0 + 0x18 + (index << 1)) as *const u16))
      }
   }
#[doc="Write the CC register."]
   #[inline] pub fn set_cc<I: Into<bits::R2>, F: FnOnce(Cc) -> Cc>(&self, index: I, f: F) -> &Self {
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      let value = f(Cc(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x18 + (index << 1)) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the CC register."]
   #[inline] pub fn with_cc<I: Into<bits::R2> + Copy, F: FnOnce(Cc) -> Cc>(&self, index: I, f: F) -> &Self {
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.cc(index);
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x18 + (index << 1)) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the COUNT register."]
   #[inline] pub fn count_ptr(&self) -> *const u16 { 
      ((self.0 as usize) + 0x10) as *const u16
   }
#[doc="Get the *mut pointer for the COUNT register."]
   #[inline] pub fn count_mut(&self) -> *mut u16 { 
      ((self.0 as usize) + 0x10) as *mut u16
   }
#[doc="Read the COUNT register."]
   #[inline] pub fn count(&self) -> Count { 
      unsafe {
         Count(::core::ptr::read_volatile((self.0 + 0x10) as *const u16))
      }
   }
#[doc="Write the COUNT register."]
   #[inline] pub fn set_count<F: FnOnce(Count) -> Count>(&self, f: F) -> &Self {
      let value = f(Count(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x10) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the COUNT register."]
   #[inline] pub fn with_count<F: FnOnce(Count) -> Count>(&self, f: F) -> &Self {
      let tmp = self.count();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x10) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CTRLA register."]
   #[inline] pub fn ctrla_ptr(&self) -> *const u16 { 
      ((self.0 as usize) + 0x0) as *const u16
   }
#[doc="Get the *mut pointer for the CTRLA register."]
   #[inline] pub fn ctrla_mut(&self) -> *mut u16 { 
      ((self.0 as usize) + 0x0) as *mut u16
   }
#[doc="Read the CTRLA register."]
   #[inline] pub fn ctrla(&self) -> Ctrla { 
      unsafe {
         Ctrla(::core::ptr::read_volatile((self.0 + 0x0) as *const u16))
      }
   }
#[doc="Write the CTRLA register."]
   #[inline] pub fn set_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
      let value = f(Ctrla(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x0) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the CTRLA register."]
   #[inline] pub fn with_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
      let tmp = self.ctrla();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x0) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CTRLBCLR register."]
   #[inline] pub fn ctrlbclr_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x4) as *const u8
   }
#[doc="Get the *mut pointer for the CTRLBCLR register."]
   #[inline] pub fn ctrlbclr_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x4) as *mut u8
   }
#[doc="Read the CTRLBCLR register."]
   #[inline] pub fn ctrlbclr(&self) -> Ctrlbclr { 
      unsafe {
         Ctrlbclr(::core::ptr::read_volatile((self.0 + 0x4) as *const u8))
      }
   }
#[doc="Write the CTRLBCLR register."]
   #[inline] pub fn set_ctrlbclr<F: FnOnce(Ctrlbclr) -> Ctrlbclr>(&self, f: F) -> &Self {
      let value = f(Ctrlbclr(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x4) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the CTRLBCLR register."]
   #[inline] pub fn with_ctrlbclr<F: FnOnce(Ctrlbclr) -> Ctrlbclr>(&self, f: F) -> &Self {
      let tmp = self.ctrlbclr();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x4) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CTRLBSET register."]
   #[inline] pub fn ctrlbset_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x5) as *const u8
   }
#[doc="Get the *mut pointer for the CTRLBSET register."]
   #[inline] pub fn ctrlbset_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x5) as *mut u8
   }
#[doc="Read the CTRLBSET register."]
   #[inline] pub fn ctrlbset(&self) -> Ctrlbset { 
      unsafe {
         Ctrlbset(::core::ptr::read_volatile((self.0 + 0x5) as *const u8))
      }
   }
#[doc="Write the CTRLBSET register."]
   #[inline] pub fn set_ctrlbset<F: FnOnce(Ctrlbset) -> Ctrlbset>(&self, f: F) -> &Self {
      let value = f(Ctrlbset(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x5) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the CTRLBSET register."]
   #[inline] pub fn with_ctrlbset<F: FnOnce(Ctrlbset) -> Ctrlbset>(&self, f: F) -> &Self {
      let tmp = self.ctrlbset();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x5) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CTRLC register."]
   #[inline] pub fn ctrlc_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x6) as *const u8
   }
#[doc="Get the *mut pointer for the CTRLC register."]
   #[inline] pub fn ctrlc_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x6) as *mut u8
   }
#[doc="Read the CTRLC register."]
   #[inline] pub fn ctrlc(&self) -> Ctrlc { 
      unsafe {
         Ctrlc(::core::ptr::read_volatile((self.0 + 0x6) as *const u8))
      }
   }
#[doc="Write the CTRLC register."]
   #[inline] pub fn set_ctrlc<F: FnOnce(Ctrlc) -> Ctrlc>(&self, f: F) -> &Self {
      let value = f(Ctrlc(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x6) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the CTRLC register."]
   #[inline] pub fn with_ctrlc<F: FnOnce(Ctrlc) -> Ctrlc>(&self, f: F) -> &Self {
      let tmp = self.ctrlc();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x6) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DBGCTRL register."]
   #[inline] pub fn dbgctrl_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x8) as *const u8
   }
#[doc="Get the *mut pointer for the DBGCTRL register."]
   #[inline] pub fn dbgctrl_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x8) as *mut u8
   }
#[doc="Read the DBGCTRL register."]
   #[inline] pub fn dbgctrl(&self) -> Dbgctrl { 
      unsafe {
         Dbgctrl(::core::ptr::read_volatile((self.0 + 0x8) as *const u8))
      }
   }
#[doc="Write the DBGCTRL register."]
   #[inline] pub fn set_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
      let value = f(Dbgctrl(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x8) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the DBGCTRL register."]
   #[inline] pub fn with_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
      let tmp = self.dbgctrl();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x8) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the EVCTRL register."]
   #[inline] pub fn evctrl_ptr(&self) -> *const u16 { 
      ((self.0 as usize) + 0xa) as *const u16
   }
#[doc="Get the *mut pointer for the EVCTRL register."]
   #[inline] pub fn evctrl_mut(&self) -> *mut u16 { 
      ((self.0 as usize) + 0xa) as *mut u16
   }
#[doc="Read the EVCTRL register."]
   #[inline] pub fn evctrl(&self) -> Evctrl { 
      unsafe {
         Evctrl(::core::ptr::read_volatile((self.0 + 0xa) as *const u16))
      }
   }
#[doc="Write the EVCTRL register."]
   #[inline] pub fn set_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
      let value = f(Evctrl(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xa) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the EVCTRL register."]
   #[inline] pub fn with_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
      let tmp = self.evctrl();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xa) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the INTENCLR register."]
   #[inline] pub fn intenclr_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0xc) as *const u8
   }
#[doc="Get the *mut pointer for the INTENCLR register."]
   #[inline] pub fn intenclr_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0xc) as *mut u8
   }
#[doc="Read the INTENCLR register."]
   #[inline] pub fn intenclr(&self) -> Intenclr { 
      unsafe {
         Intenclr(::core::ptr::read_volatile((self.0 + 0xc) as *const u8))
      }
   }
#[doc="Write the INTENCLR register."]
   #[inline] pub fn set_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
      let value = f(Intenclr(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xc) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the INTENCLR register."]
   #[inline] pub fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
      let tmp = self.intenclr();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xc) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the INTENSET register."]
   #[inline] pub fn intenset_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0xd) as *const u8
   }
#[doc="Get the *mut pointer for the INTENSET register."]
   #[inline] pub fn intenset_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0xd) as *mut u8
   }
#[doc="Read the INTENSET register."]
   #[inline] pub fn intenset(&self) -> Intenset { 
      unsafe {
         Intenset(::core::ptr::read_volatile((self.0 + 0xd) as *const u8))
      }
   }
#[doc="Write the INTENSET register."]
   #[inline] pub fn set_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
      let value = f(Intenset(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xd) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the INTENSET register."]
   #[inline] pub fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
      let tmp = self.intenset();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xd) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the INTFLAG register."]
   #[inline] pub fn intflag_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0xe) as *const u8
   }
#[doc="Get the *mut pointer for the INTFLAG register."]
   #[inline] pub fn intflag_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0xe) as *mut u8
   }
#[doc="Read the INTFLAG register."]
   #[inline] pub fn intflag(&self) -> Intflag { 
      unsafe {
         Intflag(::core::ptr::read_volatile((self.0 + 0xe) as *const u8))
      }
   }
#[doc="Write the INTFLAG register."]
   #[inline] pub fn set_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
      let value = f(Intflag(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xe) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the INTFLAG register."]
   #[inline] pub fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
      let tmp = self.intflag();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xe) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the READREQ register."]
   #[inline] pub fn readreq_ptr(&self) -> *const u16 { 
      ((self.0 as usize) + 0x2) as *const u16
   }
#[doc="Get the *mut pointer for the READREQ register."]
   #[inline] pub fn readreq_mut(&self) -> *mut u16 { 
      ((self.0 as usize) + 0x2) as *mut u16
   }
#[doc="Read the READREQ register."]
   #[inline] pub fn readreq(&self) -> Readreq { 
      unsafe {
         Readreq(::core::ptr::read_volatile((self.0 + 0x2) as *const u16))
      }
   }
#[doc="Write the READREQ register."]
   #[inline] pub fn set_readreq<F: FnOnce(Readreq) -> Readreq>(&self, f: F) -> &Self {
      let value = f(Readreq(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x2) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the READREQ register."]
   #[inline] pub fn with_readreq<F: FnOnce(Readreq) -> Readreq>(&self, f: F) -> &Self {
      let tmp = self.readreq();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x2) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the STATUS register."]
   #[inline] pub fn status_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0xf) as *const u8
   }
#[doc="Get the *mut pointer for the STATUS register."]
   #[inline] pub fn status_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0xf) as *mut u8
   }
#[doc="Read the STATUS register."]
   #[inline] pub fn status(&self) -> Status { 
      unsafe {
         Status(::core::ptr::read_volatile((self.0 + 0xf) as *const u8))
      }
   }

}

#[doc="COUNT16 Compare/Capture"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cc(pub u16);
impl Cc {
#[doc="Compare/Capture Value"]
   #[inline] pub fn cc(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Compare/Capture Value"]
   #[inline] pub fn set_cc<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Cc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cc() != 0 { try!(write!(f, " cc=0x{:x}", self.cc()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="COUNT16 Counter Value"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Count(pub u16);
impl Count {
#[doc="Count Value"]
   #[inline] pub fn count(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Count Value"]
   #[inline] pub fn set_count<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Count {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Count {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Control A"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ctrla(pub u16);
impl Ctrla {
#[doc="Software Reset"]
   #[inline] pub fn swrst(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Software Reset"]
   #[inline] pub fn set_swrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Enable"]
   #[inline] pub fn enable(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Enable"]
   #[inline] pub fn set_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="TC Mode"]
   #[inline] pub fn mode(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
   }
#[doc="TC Mode"]
   #[inline] pub fn set_mode<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x3 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Waveform Generation Operation"]
   #[inline] pub fn wavegen(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
   }
#[doc="Waveform Generation Operation"]
   #[inline] pub fn set_wavegen<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x3 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Prescaler"]
   #[inline] pub fn prescaler(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
   }
#[doc="Prescaler"]
   #[inline] pub fn set_prescaler<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x7 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Run in Standby"]
   #[inline] pub fn runstdby(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
   }
#[doc="Run in Standby"]
   #[inline] pub fn set_runstdby<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 11);
      self.0 |= value << 11;
      self
   }

#[doc="Prescaler and Counter Synchronization"]
   #[inline] pub fn prescsync(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
   }
#[doc="Prescaler and Counter Synchronization"]
   #[inline] pub fn set_prescsync<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x3 << 12);
      self.0 |= value << 12;
      self
   }

}
impl ::core::fmt::Display for Ctrla {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctrla {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.swrst() != 0 { try!(write!(f, " swrst"))}
      if self.enable() != 0 { try!(write!(f, " enable"))}
      if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
      if self.wavegen() != 0 { try!(write!(f, " wavegen=0x{:x}", self.wavegen()))}
      if self.prescaler() != 0 { try!(write!(f, " prescaler=0x{:x}", self.prescaler()))}
      if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
      if self.prescsync() != 0 { try!(write!(f, " prescsync=0x{:x}", self.prescsync()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Control B Clear"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ctrlbclr(pub u8);
impl Ctrlbclr {
#[doc="Counter Direction"]
   #[inline] pub fn dir(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Counter Direction"]
   #[inline] pub fn set_dir<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="One-Shot"]
   #[inline] pub fn oneshot(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="One-Shot"]
   #[inline] pub fn set_oneshot<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Command"]
   #[inline] pub fn cmd(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
   }
#[doc="Command"]
   #[inline] pub fn set_cmd<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x3 << 6);
      self.0 |= value << 6;
      self
   }

}
impl ::core::fmt::Display for Ctrlbclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctrlbclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dir() != 0 { try!(write!(f, " dir"))}
      if self.oneshot() != 0 { try!(write!(f, " oneshot"))}
      if self.cmd() != 0 { try!(write!(f, " cmd=0x{:x}", self.cmd()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Control B Set"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ctrlbset(pub u8);
impl Ctrlbset {
#[doc="Counter Direction"]
   #[inline] pub fn dir(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Counter Direction"]
   #[inline] pub fn set_dir<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="One-Shot"]
   #[inline] pub fn oneshot(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="One-Shot"]
   #[inline] pub fn set_oneshot<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Command"]
   #[inline] pub fn cmd(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
   }
#[doc="Command"]
   #[inline] pub fn set_cmd<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x3 << 6);
      self.0 |= value << 6;
      self
   }

}
impl ::core::fmt::Display for Ctrlbset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctrlbset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dir() != 0 { try!(write!(f, " dir"))}
      if self.oneshot() != 0 { try!(write!(f, " oneshot"))}
      if self.cmd() != 0 { try!(write!(f, " cmd=0x{:x}", self.cmd()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Control C"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ctrlc(pub u8);
impl Ctrlc {
#[doc="Output Waveform 0 Invert Enable"]
   #[inline] pub fn inven0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Output Waveform 0 Invert Enable"]
   #[inline] pub fn set_inven0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Output Waveform 1 Invert Enable"]
   #[inline] pub fn inven1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Output Waveform 1 Invert Enable"]
   #[inline] pub fn set_inven1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Capture Channel 0 Enable"]
   #[inline] pub fn cpten0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Capture Channel 0 Enable"]
   #[inline] pub fn set_cpten0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Capture Channel 1 Enable"]
   #[inline] pub fn cpten1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Capture Channel 1 Enable"]
   #[inline] pub fn set_cpten1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

}
impl ::core::fmt::Display for Ctrlc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctrlc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.inven0() != 0 { try!(write!(f, " inven0"))}
      if self.inven1() != 0 { try!(write!(f, " inven1"))}
      if self.cpten0() != 0 { try!(write!(f, " cpten0"))}
      if self.cpten1() != 0 { try!(write!(f, " cpten1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Debug Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
#[doc="Debug Run Mode"]
   #[inline] pub fn dbgrun(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Debug Run Mode"]
   #[inline] pub fn set_dbgrun<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Dbgctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dbgctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dbgrun() != 0 { try!(write!(f, " dbgrun"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Event Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Evctrl(pub u16);
impl Evctrl {
#[doc="Event Action"]
   #[inline] pub fn evact(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
   }
#[doc="Event Action"]
   #[inline] pub fn set_evact<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x7 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="TC Inverted Event Input"]
   #[inline] pub fn tcinv(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="TC Inverted Event Input"]
   #[inline] pub fn set_tcinv<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="TC Event Input"]
   #[inline] pub fn tcei(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="TC Event Input"]
   #[inline] pub fn set_tcei<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Overflow/Underflow Event Output Enable"]
   #[inline] pub fn ovfeo(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="Overflow/Underflow Event Output Enable"]
   #[inline] pub fn set_ovfeo<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Match or Capture Channel 0 Event Output Enable"]
   #[inline] pub fn mceo0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
   }
#[doc="Match or Capture Channel 0 Event Output Enable"]
   #[inline] pub fn set_mceo0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 12);
      self.0 |= value << 12;
      self
   }

#[doc="Match or Capture Channel 1 Event Output Enable"]
   #[inline] pub fn mceo1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
   }
#[doc="Match or Capture Channel 1 Event Output Enable"]
   #[inline] pub fn set_mceo1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 13);
      self.0 |= value << 13;
      self
   }

}
impl ::core::fmt::Display for Evctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Evctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.evact() != 0 { try!(write!(f, " evact=0x{:x}", self.evact()))}
      if self.tcinv() != 0 { try!(write!(f, " tcinv"))}
      if self.tcei() != 0 { try!(write!(f, " tcei"))}
      if self.ovfeo() != 0 { try!(write!(f, " ovfeo"))}
      if self.mceo0() != 0 { try!(write!(f, " mceo0"))}
      if self.mceo1() != 0 { try!(write!(f, " mceo1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Interrupt Enable Clear"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
#[doc="Overflow Interrupt Enable"]
   #[inline] pub fn ovf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Overflow Interrupt Enable"]
   #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Error Interrupt Enable"]
   #[inline] pub fn err(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Error Interrupt Enable"]
   #[inline] pub fn set_err<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Synchronization Ready Interrupt Enable"]
   #[inline] pub fn syncrdy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Synchronization Ready Interrupt Enable"]
   #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Match or Capture Channel 0 Interrupt Enable"]
   #[inline] pub fn mc0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Match or Capture Channel 0 Interrupt Enable"]
   #[inline] pub fn set_mc0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Match or Capture Channel 1 Interrupt Enable"]
   #[inline] pub fn mc1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Match or Capture Channel 1 Interrupt Enable"]
   #[inline] pub fn set_mc1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

}
impl ::core::fmt::Display for Intenclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Intenclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ovf() != 0 { try!(write!(f, " ovf"))}
      if self.err() != 0 { try!(write!(f, " err"))}
      if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
      if self.mc0() != 0 { try!(write!(f, " mc0"))}
      if self.mc1() != 0 { try!(write!(f, " mc1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Interrupt Enable Set"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
#[doc="Overflow Interrupt Enable"]
   #[inline] pub fn ovf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Overflow Interrupt Enable"]
   #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Error Interrupt Enable"]
   #[inline] pub fn err(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Error Interrupt Enable"]
   #[inline] pub fn set_err<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Synchronization Ready Interrupt Enable"]
   #[inline] pub fn syncrdy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Synchronization Ready Interrupt Enable"]
   #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Match or Capture Channel 0 Interrupt Enable"]
   #[inline] pub fn mc0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Match or Capture Channel 0 Interrupt Enable"]
   #[inline] pub fn set_mc0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Match or Capture Channel 1 Interrupt Enable"]
   #[inline] pub fn mc1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Match or Capture Channel 1 Interrupt Enable"]
   #[inline] pub fn set_mc1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

}
impl ::core::fmt::Display for Intenset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Intenset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ovf() != 0 { try!(write!(f, " ovf"))}
      if self.err() != 0 { try!(write!(f, " err"))}
      if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
      if self.mc0() != 0 { try!(write!(f, " mc0"))}
      if self.mc1() != 0 { try!(write!(f, " mc1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Interrupt Flag Status and Clear"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
#[doc="Overflow"]
   #[inline] pub fn ovf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Overflow"]
   #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Error"]
   #[inline] pub fn err(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Error"]
   #[inline] pub fn set_err<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Synchronization Ready"]
   #[inline] pub fn syncrdy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Synchronization Ready"]
   #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Match or Capture Channel 0"]
   #[inline] pub fn mc0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Match or Capture Channel 0"]
   #[inline] pub fn set_mc0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Match or Capture Channel 1"]
   #[inline] pub fn mc1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Match or Capture Channel 1"]
   #[inline] pub fn set_mc1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

}
impl ::core::fmt::Display for Intflag {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Intflag {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ovf() != 0 { try!(write!(f, " ovf"))}
      if self.err() != 0 { try!(write!(f, " err"))}
      if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
      if self.mc0() != 0 { try!(write!(f, " mc0"))}
      if self.mc1() != 0 { try!(write!(f, " mc1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Read Request"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Readreq(pub u16);
impl Readreq {
#[doc="Address"]
   #[inline] pub fn addr(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
   }
#[doc="Address"]
   #[inline] pub fn set_addr<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1f << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Read Continuously"]
   #[inline] pub fn rcont(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
   }
#[doc="Read Continuously"]
   #[inline] pub fn set_rcont<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 14);
      self.0 |= value << 14;
      self
   }

#[doc="Read Request"]
   #[inline] pub fn rreq(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
   }
#[doc="Read Request"]
   #[inline] pub fn set_rreq<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 15);
      self.0 |= value << 15;
      self
   }

}
impl ::core::fmt::Display for Readreq {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Readreq {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.addr() != 0 { try!(write!(f, " addr=0x{:x}", self.addr()))}
      if self.rcont() != 0 { try!(write!(f, " rcont"))}
      if self.rreq() != 0 { try!(write!(f, " rreq"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Status"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u8);
impl Status {
#[doc="Stop"]
   #[inline] pub fn stop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Stop"]
   #[inline] pub fn set_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Slave"]
   #[inline] pub fn slave(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Slave"]
   #[inline] pub fn set_slave<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Synchronization Busy"]
   #[inline] pub fn syncbusy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Synchronization Busy"]
   #[inline] pub fn set_syncbusy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Status {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Status {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.stop() != 0 { try!(write!(f, " stop"))}
      if self.slave() != 0 { try!(write!(f, " slave"))}
      if self.syncbusy() != 0 { try!(write!(f, " syncbusy"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
}
// End of count16
#[doc="32-bit Counter Mode Cluster"]
pub mod count32 {
#[allow(unused_imports)] use bobbin_common::*;
   #[derive(Clone, Copy, PartialEq, Eq)]
#[doc="32-bit Counter Mode Peripheral"]
   pub struct Count32(pub usize);
impl Count32 {
#[doc="Get the *const pointer for the CC register."]
   #[inline] pub fn cc_ptr<I: Into<bits::R2>>(&self, index: I) -> *const u32 { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x18 + (index << 2)) as *const u32
   }
#[doc="Get the *mut pointer for the CC register."]
   #[inline] pub fn cc_mut<I: Into<bits::R2>>(&self, index: I) -> *mut u32 { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x18 + (index << 2)) as *mut u32
   }
#[doc="Read the CC register."]
   #[inline] pub fn cc<I: Into<bits::R2>>(&self, index: I) -> Cc { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         Cc(::core::ptr::read_volatile((self.0 + 0x18 + (index << 2)) as *const u32))
      }
   }
#[doc="Write the CC register."]
   #[inline] pub fn set_cc<I: Into<bits::R2>, F: FnOnce(Cc) -> Cc>(&self, index: I, f: F) -> &Self {
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      let value = f(Cc(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x18 + (index << 2)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CC register."]
   #[inline] pub fn with_cc<I: Into<bits::R2> + Copy, F: FnOnce(Cc) -> Cc>(&self, index: I, f: F) -> &Self {
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.cc(index);
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x18 + (index << 2)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the COUNT register."]
   #[inline] pub fn count_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x10) as *const u32
   }
#[doc="Get the *mut pointer for the COUNT register."]
   #[inline] pub fn count_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x10) as *mut u32
   }
#[doc="Read the COUNT register."]
   #[inline] pub fn count(&self) -> Count { 
      unsafe {
         Count(::core::ptr::read_volatile((self.0 + 0x10) as *const u32))
      }
   }
#[doc="Write the COUNT register."]
   #[inline] pub fn set_count<F: FnOnce(Count) -> Count>(&self, f: F) -> &Self {
      let value = f(Count(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x10) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the COUNT register."]
   #[inline] pub fn with_count<F: FnOnce(Count) -> Count>(&self, f: F) -> &Self {
      let tmp = self.count();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x10) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CTRLA register."]
   #[inline] pub fn ctrla_ptr(&self) -> *const u16 { 
      ((self.0 as usize) + 0x0) as *const u16
   }
#[doc="Get the *mut pointer for the CTRLA register."]
   #[inline] pub fn ctrla_mut(&self) -> *mut u16 { 
      ((self.0 as usize) + 0x0) as *mut u16
   }
#[doc="Read the CTRLA register."]
   #[inline] pub fn ctrla(&self) -> Ctrla { 
      unsafe {
         Ctrla(::core::ptr::read_volatile((self.0 + 0x0) as *const u16))
      }
   }
#[doc="Write the CTRLA register."]
   #[inline] pub fn set_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
      let value = f(Ctrla(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x0) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the CTRLA register."]
   #[inline] pub fn with_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
      let tmp = self.ctrla();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x0) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CTRLBCLR register."]
   #[inline] pub fn ctrlbclr_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x4) as *const u8
   }
#[doc="Get the *mut pointer for the CTRLBCLR register."]
   #[inline] pub fn ctrlbclr_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x4) as *mut u8
   }
#[doc="Read the CTRLBCLR register."]
   #[inline] pub fn ctrlbclr(&self) -> Ctrlbclr { 
      unsafe {
         Ctrlbclr(::core::ptr::read_volatile((self.0 + 0x4) as *const u8))
      }
   }
#[doc="Write the CTRLBCLR register."]
   #[inline] pub fn set_ctrlbclr<F: FnOnce(Ctrlbclr) -> Ctrlbclr>(&self, f: F) -> &Self {
      let value = f(Ctrlbclr(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x4) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the CTRLBCLR register."]
   #[inline] pub fn with_ctrlbclr<F: FnOnce(Ctrlbclr) -> Ctrlbclr>(&self, f: F) -> &Self {
      let tmp = self.ctrlbclr();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x4) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CTRLBSET register."]
   #[inline] pub fn ctrlbset_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x5) as *const u8
   }
#[doc="Get the *mut pointer for the CTRLBSET register."]
   #[inline] pub fn ctrlbset_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x5) as *mut u8
   }
#[doc="Read the CTRLBSET register."]
   #[inline] pub fn ctrlbset(&self) -> Ctrlbset { 
      unsafe {
         Ctrlbset(::core::ptr::read_volatile((self.0 + 0x5) as *const u8))
      }
   }
#[doc="Write the CTRLBSET register."]
   #[inline] pub fn set_ctrlbset<F: FnOnce(Ctrlbset) -> Ctrlbset>(&self, f: F) -> &Self {
      let value = f(Ctrlbset(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x5) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the CTRLBSET register."]
   #[inline] pub fn with_ctrlbset<F: FnOnce(Ctrlbset) -> Ctrlbset>(&self, f: F) -> &Self {
      let tmp = self.ctrlbset();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x5) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CTRLC register."]
   #[inline] pub fn ctrlc_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x6) as *const u8
   }
#[doc="Get the *mut pointer for the CTRLC register."]
   #[inline] pub fn ctrlc_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x6) as *mut u8
   }
#[doc="Read the CTRLC register."]
   #[inline] pub fn ctrlc(&self) -> Ctrlc { 
      unsafe {
         Ctrlc(::core::ptr::read_volatile((self.0 + 0x6) as *const u8))
      }
   }
#[doc="Write the CTRLC register."]
   #[inline] pub fn set_ctrlc<F: FnOnce(Ctrlc) -> Ctrlc>(&self, f: F) -> &Self {
      let value = f(Ctrlc(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x6) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the CTRLC register."]
   #[inline] pub fn with_ctrlc<F: FnOnce(Ctrlc) -> Ctrlc>(&self, f: F) -> &Self {
      let tmp = self.ctrlc();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x6) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DBGCTRL register."]
   #[inline] pub fn dbgctrl_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x8) as *const u8
   }
#[doc="Get the *mut pointer for the DBGCTRL register."]
   #[inline] pub fn dbgctrl_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x8) as *mut u8
   }
#[doc="Read the DBGCTRL register."]
   #[inline] pub fn dbgctrl(&self) -> Dbgctrl { 
      unsafe {
         Dbgctrl(::core::ptr::read_volatile((self.0 + 0x8) as *const u8))
      }
   }
#[doc="Write the DBGCTRL register."]
   #[inline] pub fn set_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
      let value = f(Dbgctrl(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x8) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the DBGCTRL register."]
   #[inline] pub fn with_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
      let tmp = self.dbgctrl();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x8) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the EVCTRL register."]
   #[inline] pub fn evctrl_ptr(&self) -> *const u16 { 
      ((self.0 as usize) + 0xa) as *const u16
   }
#[doc="Get the *mut pointer for the EVCTRL register."]
   #[inline] pub fn evctrl_mut(&self) -> *mut u16 { 
      ((self.0 as usize) + 0xa) as *mut u16
   }
#[doc="Read the EVCTRL register."]
   #[inline] pub fn evctrl(&self) -> Evctrl { 
      unsafe {
         Evctrl(::core::ptr::read_volatile((self.0 + 0xa) as *const u16))
      }
   }
#[doc="Write the EVCTRL register."]
   #[inline] pub fn set_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
      let value = f(Evctrl(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xa) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the EVCTRL register."]
   #[inline] pub fn with_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
      let tmp = self.evctrl();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xa) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the INTENCLR register."]
   #[inline] pub fn intenclr_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0xc) as *const u8
   }
#[doc="Get the *mut pointer for the INTENCLR register."]
   #[inline] pub fn intenclr_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0xc) as *mut u8
   }
#[doc="Read the INTENCLR register."]
   #[inline] pub fn intenclr(&self) -> Intenclr { 
      unsafe {
         Intenclr(::core::ptr::read_volatile((self.0 + 0xc) as *const u8))
      }
   }
#[doc="Write the INTENCLR register."]
   #[inline] pub fn set_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
      let value = f(Intenclr(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xc) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the INTENCLR register."]
   #[inline] pub fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
      let tmp = self.intenclr();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xc) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the INTENSET register."]
   #[inline] pub fn intenset_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0xd) as *const u8
   }
#[doc="Get the *mut pointer for the INTENSET register."]
   #[inline] pub fn intenset_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0xd) as *mut u8
   }
#[doc="Read the INTENSET register."]
   #[inline] pub fn intenset(&self) -> Intenset { 
      unsafe {
         Intenset(::core::ptr::read_volatile((self.0 + 0xd) as *const u8))
      }
   }
#[doc="Write the INTENSET register."]
   #[inline] pub fn set_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
      let value = f(Intenset(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xd) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the INTENSET register."]
   #[inline] pub fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
      let tmp = self.intenset();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xd) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the INTFLAG register."]
   #[inline] pub fn intflag_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0xe) as *const u8
   }
#[doc="Get the *mut pointer for the INTFLAG register."]
   #[inline] pub fn intflag_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0xe) as *mut u8
   }
#[doc="Read the INTFLAG register."]
   #[inline] pub fn intflag(&self) -> Intflag { 
      unsafe {
         Intflag(::core::ptr::read_volatile((self.0 + 0xe) as *const u8))
      }
   }
#[doc="Write the INTFLAG register."]
   #[inline] pub fn set_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
      let value = f(Intflag(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xe) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the INTFLAG register."]
   #[inline] pub fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
      let tmp = self.intflag();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xe) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the READREQ register."]
   #[inline] pub fn readreq_ptr(&self) -> *const u16 { 
      ((self.0 as usize) + 0x2) as *const u16
   }
#[doc="Get the *mut pointer for the READREQ register."]
   #[inline] pub fn readreq_mut(&self) -> *mut u16 { 
      ((self.0 as usize) + 0x2) as *mut u16
   }
#[doc="Read the READREQ register."]
   #[inline] pub fn readreq(&self) -> Readreq { 
      unsafe {
         Readreq(::core::ptr::read_volatile((self.0 + 0x2) as *const u16))
      }
   }
#[doc="Write the READREQ register."]
   #[inline] pub fn set_readreq<F: FnOnce(Readreq) -> Readreq>(&self, f: F) -> &Self {
      let value = f(Readreq(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x2) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the READREQ register."]
   #[inline] pub fn with_readreq<F: FnOnce(Readreq) -> Readreq>(&self, f: F) -> &Self {
      let tmp = self.readreq();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x2) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the STATUS register."]
   #[inline] pub fn status_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0xf) as *const u8
   }
#[doc="Get the *mut pointer for the STATUS register."]
   #[inline] pub fn status_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0xf) as *mut u8
   }
#[doc="Read the STATUS register."]
   #[inline] pub fn status(&self) -> Status { 
      unsafe {
         Status(::core::ptr::read_volatile((self.0 + 0xf) as *const u8))
      }
   }

}

#[doc="COUNT32 Compare/Capture"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cc(pub u32);
impl Cc {
#[doc="Compare/Capture Value"]
   #[inline] pub fn cc(&self) -> bits::U32 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
   }
#[doc="Compare/Capture Value"]
   #[inline] pub fn set_cc<V: Into<bits::U32>>(mut self, value: V) -> Self {
      let value: bits::U32 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffffffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Cc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="COUNT32 Counter Value"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Count(pub u32);
impl Count {
#[doc="Count Value"]
   #[inline] pub fn count(&self) -> bits::U32 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
   }
#[doc="Count Value"]
   #[inline] pub fn set_count<V: Into<bits::U32>>(mut self, value: V) -> Self {
      let value: bits::U32 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffffffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Count {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Count {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Control A"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ctrla(pub u16);
impl Ctrla {
#[doc="Software Reset"]
   #[inline] pub fn swrst(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Software Reset"]
   #[inline] pub fn set_swrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Enable"]
   #[inline] pub fn enable(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Enable"]
   #[inline] pub fn set_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="TC Mode"]
   #[inline] pub fn mode(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
   }
#[doc="TC Mode"]
   #[inline] pub fn set_mode<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x3 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Waveform Generation Operation"]
   #[inline] pub fn wavegen(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
   }
#[doc="Waveform Generation Operation"]
   #[inline] pub fn set_wavegen<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x3 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Prescaler"]
   #[inline] pub fn prescaler(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
   }
#[doc="Prescaler"]
   #[inline] pub fn set_prescaler<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x7 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Run in Standby"]
   #[inline] pub fn runstdby(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
   }
#[doc="Run in Standby"]
   #[inline] pub fn set_runstdby<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 11);
      self.0 |= value << 11;
      self
   }

#[doc="Prescaler and Counter Synchronization"]
   #[inline] pub fn prescsync(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
   }
#[doc="Prescaler and Counter Synchronization"]
   #[inline] pub fn set_prescsync<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x3 << 12);
      self.0 |= value << 12;
      self
   }

}
impl ::core::fmt::Display for Ctrla {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctrla {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.swrst() != 0 { try!(write!(f, " swrst"))}
      if self.enable() != 0 { try!(write!(f, " enable"))}
      if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
      if self.wavegen() != 0 { try!(write!(f, " wavegen=0x{:x}", self.wavegen()))}
      if self.prescaler() != 0 { try!(write!(f, " prescaler=0x{:x}", self.prescaler()))}
      if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
      if self.prescsync() != 0 { try!(write!(f, " prescsync=0x{:x}", self.prescsync()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Control B Clear"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ctrlbclr(pub u8);
impl Ctrlbclr {
#[doc="Counter Direction"]
   #[inline] pub fn dir(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Counter Direction"]
   #[inline] pub fn set_dir<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="One-Shot"]
   #[inline] pub fn oneshot(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="One-Shot"]
   #[inline] pub fn set_oneshot<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Command"]
   #[inline] pub fn cmd(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
   }
#[doc="Command"]
   #[inline] pub fn set_cmd<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x3 << 6);
      self.0 |= value << 6;
      self
   }

}
impl ::core::fmt::Display for Ctrlbclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctrlbclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dir() != 0 { try!(write!(f, " dir"))}
      if self.oneshot() != 0 { try!(write!(f, " oneshot"))}
      if self.cmd() != 0 { try!(write!(f, " cmd=0x{:x}", self.cmd()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Control B Set"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ctrlbset(pub u8);
impl Ctrlbset {
#[doc="Counter Direction"]
   #[inline] pub fn dir(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Counter Direction"]
   #[inline] pub fn set_dir<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="One-Shot"]
   #[inline] pub fn oneshot(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="One-Shot"]
   #[inline] pub fn set_oneshot<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Command"]
   #[inline] pub fn cmd(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
   }
#[doc="Command"]
   #[inline] pub fn set_cmd<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x3 << 6);
      self.0 |= value << 6;
      self
   }

}
impl ::core::fmt::Display for Ctrlbset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctrlbset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dir() != 0 { try!(write!(f, " dir"))}
      if self.oneshot() != 0 { try!(write!(f, " oneshot"))}
      if self.cmd() != 0 { try!(write!(f, " cmd=0x{:x}", self.cmd()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Control C"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ctrlc(pub u8);
impl Ctrlc {
#[doc="Output Waveform 0 Invert Enable"]
   #[inline] pub fn inven0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Output Waveform 0 Invert Enable"]
   #[inline] pub fn set_inven0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Output Waveform 1 Invert Enable"]
   #[inline] pub fn inven1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Output Waveform 1 Invert Enable"]
   #[inline] pub fn set_inven1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Capture Channel 0 Enable"]
   #[inline] pub fn cpten0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Capture Channel 0 Enable"]
   #[inline] pub fn set_cpten0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Capture Channel 1 Enable"]
   #[inline] pub fn cpten1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Capture Channel 1 Enable"]
   #[inline] pub fn set_cpten1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

}
impl ::core::fmt::Display for Ctrlc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctrlc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.inven0() != 0 { try!(write!(f, " inven0"))}
      if self.inven1() != 0 { try!(write!(f, " inven1"))}
      if self.cpten0() != 0 { try!(write!(f, " cpten0"))}
      if self.cpten1() != 0 { try!(write!(f, " cpten1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Debug Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
#[doc="Debug Run Mode"]
   #[inline] pub fn dbgrun(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Debug Run Mode"]
   #[inline] pub fn set_dbgrun<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Dbgctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dbgctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dbgrun() != 0 { try!(write!(f, " dbgrun"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Event Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Evctrl(pub u16);
impl Evctrl {
#[doc="Event Action"]
   #[inline] pub fn evact(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
   }
#[doc="Event Action"]
   #[inline] pub fn set_evact<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x7 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="TC Inverted Event Input"]
   #[inline] pub fn tcinv(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="TC Inverted Event Input"]
   #[inline] pub fn set_tcinv<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="TC Event Input"]
   #[inline] pub fn tcei(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="TC Event Input"]
   #[inline] pub fn set_tcei<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Overflow/Underflow Event Output Enable"]
   #[inline] pub fn ovfeo(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="Overflow/Underflow Event Output Enable"]
   #[inline] pub fn set_ovfeo<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Match or Capture Channel 0 Event Output Enable"]
   #[inline] pub fn mceo0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
   }
#[doc="Match or Capture Channel 0 Event Output Enable"]
   #[inline] pub fn set_mceo0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 12);
      self.0 |= value << 12;
      self
   }

#[doc="Match or Capture Channel 1 Event Output Enable"]
   #[inline] pub fn mceo1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
   }
#[doc="Match or Capture Channel 1 Event Output Enable"]
   #[inline] pub fn set_mceo1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 13);
      self.0 |= value << 13;
      self
   }

}
impl ::core::fmt::Display for Evctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Evctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.evact() != 0 { try!(write!(f, " evact=0x{:x}", self.evact()))}
      if self.tcinv() != 0 { try!(write!(f, " tcinv"))}
      if self.tcei() != 0 { try!(write!(f, " tcei"))}
      if self.ovfeo() != 0 { try!(write!(f, " ovfeo"))}
      if self.mceo0() != 0 { try!(write!(f, " mceo0"))}
      if self.mceo1() != 0 { try!(write!(f, " mceo1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Interrupt Enable Clear"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
#[doc="Overflow Interrupt Enable"]
   #[inline] pub fn ovf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Overflow Interrupt Enable"]
   #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Error Interrupt Enable"]
   #[inline] pub fn err(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Error Interrupt Enable"]
   #[inline] pub fn set_err<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Synchronization Ready Interrupt Enable"]
   #[inline] pub fn syncrdy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Synchronization Ready Interrupt Enable"]
   #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Match or Capture Channel 0 Interrupt Enable"]
   #[inline] pub fn mc0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Match or Capture Channel 0 Interrupt Enable"]
   #[inline] pub fn set_mc0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Match or Capture Channel 1 Interrupt Enable"]
   #[inline] pub fn mc1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Match or Capture Channel 1 Interrupt Enable"]
   #[inline] pub fn set_mc1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

}
impl ::core::fmt::Display for Intenclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Intenclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ovf() != 0 { try!(write!(f, " ovf"))}
      if self.err() != 0 { try!(write!(f, " err"))}
      if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
      if self.mc0() != 0 { try!(write!(f, " mc0"))}
      if self.mc1() != 0 { try!(write!(f, " mc1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Interrupt Enable Set"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
#[doc="Overflow Interrupt Enable"]
   #[inline] pub fn ovf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Overflow Interrupt Enable"]
   #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Error Interrupt Enable"]
   #[inline] pub fn err(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Error Interrupt Enable"]
   #[inline] pub fn set_err<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Synchronization Ready Interrupt Enable"]
   #[inline] pub fn syncrdy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Synchronization Ready Interrupt Enable"]
   #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Match or Capture Channel 0 Interrupt Enable"]
   #[inline] pub fn mc0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Match or Capture Channel 0 Interrupt Enable"]
   #[inline] pub fn set_mc0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Match or Capture Channel 1 Interrupt Enable"]
   #[inline] pub fn mc1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Match or Capture Channel 1 Interrupt Enable"]
   #[inline] pub fn set_mc1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

}
impl ::core::fmt::Display for Intenset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Intenset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ovf() != 0 { try!(write!(f, " ovf"))}
      if self.err() != 0 { try!(write!(f, " err"))}
      if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
      if self.mc0() != 0 { try!(write!(f, " mc0"))}
      if self.mc1() != 0 { try!(write!(f, " mc1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Interrupt Flag Status and Clear"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
#[doc="Overflow"]
   #[inline] pub fn ovf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Overflow"]
   #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Error"]
   #[inline] pub fn err(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Error"]
   #[inline] pub fn set_err<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Synchronization Ready"]
   #[inline] pub fn syncrdy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Synchronization Ready"]
   #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Match or Capture Channel 0"]
   #[inline] pub fn mc0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Match or Capture Channel 0"]
   #[inline] pub fn set_mc0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Match or Capture Channel 1"]
   #[inline] pub fn mc1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Match or Capture Channel 1"]
   #[inline] pub fn set_mc1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

}
impl ::core::fmt::Display for Intflag {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Intflag {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ovf() != 0 { try!(write!(f, " ovf"))}
      if self.err() != 0 { try!(write!(f, " err"))}
      if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
      if self.mc0() != 0 { try!(write!(f, " mc0"))}
      if self.mc1() != 0 { try!(write!(f, " mc1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Read Request"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Readreq(pub u16);
impl Readreq {
#[doc="Address"]
   #[inline] pub fn addr(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
   }
#[doc="Address"]
   #[inline] pub fn set_addr<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1f << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Read Continuously"]
   #[inline] pub fn rcont(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
   }
#[doc="Read Continuously"]
   #[inline] pub fn set_rcont<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 14);
      self.0 |= value << 14;
      self
   }

#[doc="Read Request"]
   #[inline] pub fn rreq(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
   }
#[doc="Read Request"]
   #[inline] pub fn set_rreq<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 15);
      self.0 |= value << 15;
      self
   }

}
impl ::core::fmt::Display for Readreq {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Readreq {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.addr() != 0 { try!(write!(f, " addr=0x{:x}", self.addr()))}
      if self.rcont() != 0 { try!(write!(f, " rcont"))}
      if self.rreq() != 0 { try!(write!(f, " rreq"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Status"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u8);
impl Status {
#[doc="Stop"]
   #[inline] pub fn stop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Stop"]
   #[inline] pub fn set_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Slave"]
   #[inline] pub fn slave(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Slave"]
   #[inline] pub fn set_slave<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Synchronization Busy"]
   #[inline] pub fn syncbusy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Synchronization Busy"]
   #[inline] pub fn set_syncbusy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Status {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Status {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.stop() != 0 { try!(write!(f, " stop"))}
      if self.slave() != 0 { try!(write!(f, " slave"))}
      if self.syncbusy() != 0 { try!(write!(f, " syncbusy"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
}
// End of count32
pub struct TcCh { pub periph: TcPeriph, pub index: usize }
channel!(TC3_CH0, Tc3Ch0, TC3, Tc3, _TC3_CH0, TcCh, _TC3, 0);
channel!(TC3_CH1, Tc3Ch1, TC3, Tc3, _TC3_CH1, TcCh, _TC3, 1);
channel!(TC4_CH0, Tc4Ch0, TC4, Tc4, _TC4_CH0, TcCh, _TC4, 0);
channel!(TC4_CH1, Tc4Ch1, TC4, Tc4, _TC4_CH1, TcCh, _TC4, 1);
channel!(TC5_CH0, Tc5Ch0, TC5, Tc5, _TC5_CH0, TcCh, _TC5, 0);
channel!(TC5_CH1, Tc5Ch1, TC5, Tc5, _TC5_CH1, TcCh, _TC5, 1);
