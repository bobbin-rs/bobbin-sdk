#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "FTM", peripherals: [], prototype: Some(Peripheral { derived_from: None, group_name: Some("FTM"), name: "", address: 0, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [], clusters: [], registers: [Register { name: "SC", offset: 0, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Status And Control"), fields: [Field { name: "PS", bit_offset: 0, bit_width: 3, access: Some(ReadWrite), description: Some("Prescale Factor Selection"), enumerated_values: [EnumeratedValue { value: "#000", name: Some("000"), description: Some("Divide by 1") }, EnumeratedValue { value: "#001", name: Some("001"), description: Some("Divide by 2") }, EnumeratedValue { value: "#010", name: Some("010"), description: Some("Divide by 4") }, EnumeratedValue { value: "#011", name: Some("011"), description: Some("Divide by 8") }, EnumeratedValue { value: "#100", name: Some("100"), description: Some("Divide by 16") }, EnumeratedValue { value: "#101", name: Some("101"), description: Some("Divide by 32") }, EnumeratedValue { value: "#110", name: Some("110"), description: Some("Divide by 64") }, EnumeratedValue { value: "#111", name: Some("111"), description: Some("Divide by 128") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CLKS", bit_offset: 3, bit_width: 2, access: Some(ReadWrite), description: Some("Clock Source Selection"), enumerated_values: [EnumeratedValue { value: "#00", name: Some("00"), description: Some("No clock selected. This in effect disables the FTM counter.") }, EnumeratedValue { value: "#01", name: Some("01"), description: Some("System clock") }, EnumeratedValue { value: "#10", name: Some("10"), description: Some("Fixed frequency clock") }, EnumeratedValue { value: "#11", name: Some("11"), description: Some("External clock") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CPWMS", bit_offset: 5, bit_width: 1, access: Some(ReadWrite), description: Some("Center-Aligned PWM Select"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("FTM counter operates in Up Counting mode.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("FTM counter operates in Up-Down Counting mode.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TOIE", bit_offset: 6, bit_width: 1, access: Some(ReadWrite), description: Some("Timer Overflow Interrupt Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Disable TOF interrupts. Use software polling.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Enable TOF interrupts. An interrupt is generated when TOF equals one.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TOF", bit_offset: 7, bit_width: 1, access: Some(ReadOnly), description: Some("Timer Overflow Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("FTM counter has not overflowed.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("FTM counter has overflowed.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "PWMEN", bit_offset: 16, bit_width: 1, access: Some(ReadWrite), description: Some("PWM Enable"), enumerated_values: [], links: [], dim: Some(8), dim_increment: Some(1), dim_index: None }, Field { name: "FLTPS", bit_offset: 24, bit_width: 4, access: Some(ReadWrite), description: Some("Filter Prescaler"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "CNT", offset: 4, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Counter"), fields: [Field { name: "COUNT", bit_offset: 0, bit_width: 16, access: Some(ReadWrite), description: Some("Counter Value"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "MOD", offset: 8, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Modulo"), fields: [Field { name: "MOD", bit_offset: 0, bit_width: 16, access: Some(ReadWrite), description: Some("Modulo Value"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "CSC", offset: 12, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Channel (n) Status And Control"), fields: [Field { name: "DMA", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("DMA Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Disable DMA transfers.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Enable DMA transfers.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ELSA", bit_offset: 2, bit_width: 1, access: Some(ReadWrite), description: Some("Edge or Level Select"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ELSB", bit_offset: 3, bit_width: 1, access: Some(ReadWrite), description: Some("Edge or Level Select"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MSA", bit_offset: 4, bit_width: 1, access: Some(ReadWrite), description: Some("Channel Mode Select"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MSB", bit_offset: 5, bit_width: 1, access: Some(ReadWrite), description: Some("Channel Mode Select"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CHIE", bit_offset: 6, bit_width: 1, access: Some(ReadWrite), description: Some("Channel Interrupt Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Disable channel interrupts. Use software polling.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Enable channel interrupts.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CHF", bit_offset: 7, bit_width: 1, access: Some(ReadOnly), description: Some("Channel Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No channel event has occurred.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("A channel event has occurred.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: Some(8), dim_increment: Some(8), dim_index: Some("0,1,2,3,4,5,6,7") }, Register { name: "CV", offset: 16, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Channel (n) Value"), fields: [Field { name: "VAL", bit_offset: 0, bit_width: 16, access: Some(ReadWrite), description: Some("Channel Value"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: Some(8), dim_increment: Some(8), dim_index: Some("0,1,2,3,4,5,6,7") }, Register { name: "CNTIN", offset: 76, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Counter Initial Value"), fields: [Field { name: "INIT", bit_offset: 0, bit_width: 16, access: Some(ReadWrite), description: Some("Initial Value Of The FTM Counter"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "STATUS", offset: 80, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Capture And Compare Status"), fields: [Field { name: "CHF", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("Channel n Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No channel event has occurred.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("A channel event has occurred.") }], links: [], dim: Some(8), dim_increment: Some(1), dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "MODE", offset: 84, size: Some(32), access: Some(ReadWrite), reset_value: Some(4), reset_mask: Some(4294967295), description: Some("Features Mode Selection"), fields: [Field { name: "FTMEN", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("FTM Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Only the TPM-compatible registers (first set of registers) can be used without any restriction. Do not use the FTM-specific registers.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("All registers including the FTM-specific registers (second set of registers) are available for use with no restrictions.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "INIT", bit_offset: 1, bit_width: 1, access: Some(ReadWrite), description: Some("Initialize The Channels Output"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "WPDIS", bit_offset: 2, bit_width: 1, access: Some(ReadWrite), description: Some("Write Protection Disable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Write protection is enabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Write protection is disabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "PWMSYNC", bit_offset: 3, bit_width: 1, access: Some(ReadWrite), description: Some("PWM Synchronization Mode"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No restrictions. Software and hardware triggers can be used by MOD, CnV, OUTMASK, and FTM counter synchronization.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Software trigger can only be used by MOD and CnV synchronization, and hardware triggers can only be used by OUTMASK and FTM counter synchronization.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CAPTEST", bit_offset: 4, bit_width: 1, access: Some(ReadWrite), description: Some("Capture Test Mode Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Capture test mode is disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Capture test mode is enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "FAULTM", bit_offset: 5, bit_width: 2, access: Some(ReadWrite), description: Some("Fault Control Mode"), enumerated_values: [EnumeratedValue { value: "#00", name: Some("00"), description: Some("Fault control is disabled for all channels.") }, EnumeratedValue { value: "#01", name: Some("01"), description: Some("Fault control is enabled for even channels only (channels 0, 2, 4, and 6), and the selected mode is the manual fault clearing.") }, EnumeratedValue { value: "#10", name: Some("10"), description: Some("Fault control is enabled for all channels, and the selected mode is the manual fault clearing.") }, EnumeratedValue { value: "#11", name: Some("11"), description: Some("Fault control is enabled for all channels, and the selected mode is the automatic fault clearing.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "FAULTIE", bit_offset: 7, bit_width: 1, access: Some(ReadWrite), description: Some("Fault Interrupt Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Fault control interrupt is disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Fault control interrupt is enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "SYNC", offset: 88, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Synchronization"), fields: [Field { name: "CNTMIN", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("Minimum Loading Point Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The minimum loading point is disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The minimum loading point is enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CNTMAX", bit_offset: 1, bit_width: 1, access: Some(ReadWrite), description: Some("Maximum Loading Point Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The maximum loading point is disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The maximum loading point is enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "REINIT", bit_offset: 2, bit_width: 1, access: Some(ReadWrite), description: Some("FTM Counter Reinitialization By Synchronization (FTM counter synchronization)"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("FTM counter continues to count normally.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("FTM counter is updated with its initial value when the selected trigger is detected.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SYNCHOM", bit_offset: 3, bit_width: 1, access: Some(ReadWrite), description: Some("Output Mask Synchronization"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("OUTMASK register is updated with the value of its buffer in all rising edges of the system clock.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("OUTMASK register is updated with the value of its buffer only by the PWM synchronization.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TRIG0", bit_offset: 4, bit_width: 1, access: Some(ReadWrite), description: Some("PWM Synchronization Hardware Trigger 0"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Trigger is disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Trigger is enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TRIG1", bit_offset: 5, bit_width: 1, access: Some(ReadWrite), description: Some("PWM Synchronization Hardware Trigger 1"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Trigger is disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Trigger is enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TRIG2", bit_offset: 6, bit_width: 1, access: Some(ReadWrite), description: Some("PWM Synchronization Hardware Trigger 2"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Trigger is disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Trigger is enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SWSYNC", bit_offset: 7, bit_width: 1, access: Some(ReadWrite), description: Some("PWM Synchronization Software Trigger"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Software trigger is not selected.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Software trigger is selected.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "OUTINIT", offset: 92, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Initial State For Channels Output"), fields: [Field { name: "CHOI", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("Channel n Output Initialization Value"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The initialization value is 0.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The initialization value is 1.") }], links: [], dim: Some(8), dim_increment: Some(1), dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "OUTMASK", offset: 96, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Output Mask"), fields: [Field { name: "CHOM", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("Channel 0 Output Mask"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Channel output is not masked. It continues to operate normally.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Channel output is masked. It is forced to its inactive state.") }], links: [], dim: Some(8), dim_increment: Some(1), dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "COMBINE", offset: 100, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Function For Linked Channels"), fields: [Field { name: "COMBINE", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("Combine Channels For n"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Channels (n) and (n+1) are independent.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Channels (n) and (n+1) are combined.") }], links: [], dim: Some(4), dim_increment: Some(8), dim_index: None }, Field { name: "COMP", bit_offset: 1, bit_width: 1, access: Some(ReadWrite), description: Some("Complement Of Channel (n) For n"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The channel (n+1) output is the same as the channel (n) output.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The channel (n+1) output is the complement of the channel (n) output.") }], links: [], dim: Some(4), dim_increment: Some(8), dim_index: None }, Field { name: "DECAPEN", bit_offset: 2, bit_width: 1, access: Some(ReadWrite), description: Some("Dual Edge Capture Mode Enable For n"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The Dual Edge Capture mode in this pair of channels is disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The Dual Edge Capture mode in this pair of channels is enabled.") }], links: [], dim: Some(4), dim_increment: Some(8), dim_index: None }, Field { name: "DECAP", bit_offset: 3, bit_width: 1, access: Some(ReadWrite), description: Some("Dual Edge Capture Mode Captures For n"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The dual edge captures are inactive.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The dual edge captures are active.") }], links: [], dim: Some(4), dim_increment: Some(8), dim_index: None }, Field { name: "DTEN", bit_offset: 4, bit_width: 1, access: Some(ReadWrite), description: Some("Deadtime Enable For n"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The deadtime insertion in this pair of channels is disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The deadtime insertion in this pair of channels is enabled.") }], links: [], dim: Some(4), dim_increment: Some(8), dim_index: None }, Field { name: "SYNCEN", bit_offset: 5, bit_width: 1, access: Some(ReadWrite), description: Some("Synchronization Enable For n"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The PWM synchronization in this pair of channels is disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The PWM synchronization in this pair of channels is enabled.") }], links: [], dim: Some(4), dim_increment: Some(8), dim_index: None }, Field { name: "FAULTEN", bit_offset: 6, bit_width: 1, access: Some(ReadWrite), description: Some("Fault Control Enable For n"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The fault control in this pair of channels is disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The fault control in this pair of channels is enabled.") }], links: [], dim: Some(4), dim_increment: Some(8), dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "DEADTIME", offset: 104, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Deadtime Insertion Control"), fields: [Field { name: "DTVAL", bit_offset: 0, bit_width: 6, access: Some(ReadWrite), description: Some("Deadtime Value"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "DTPS", bit_offset: 6, bit_width: 2, access: Some(ReadWrite), description: Some("Deadtime Prescaler Value"), enumerated_values: [EnumeratedValue { value: "#0x", name: Some("0x"), description: Some("Divide the system clock by 1.") }, EnumeratedValue { value: "#10", name: Some("10"), description: Some("Divide the system clock by 4.") }, EnumeratedValue { value: "#11", name: Some("11"), description: Some("Divide the system clock by 16.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "EXTTRIG", offset: 108, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("FTM External Trigger"), fields: [Field { name: "CH2TRIG", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("Channel 2 Trigger Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The generation of the channel trigger is disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The generation of the channel trigger is enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CH3TRIG", bit_offset: 1, bit_width: 1, access: Some(ReadWrite), description: Some("Channel 3 Trigger Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The generation of the channel trigger is disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The generation of the channel trigger is enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CH4TRIG", bit_offset: 2, bit_width: 1, access: Some(ReadWrite), description: Some("Channel 4 Trigger Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The generation of the channel trigger is disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The generation of the channel trigger is enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CH5TRIG", bit_offset: 3, bit_width: 1, access: Some(ReadWrite), description: Some("Channel 5 Trigger Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The generation of the channel trigger is disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The generation of the channel trigger is enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CH0TRIG", bit_offset: 4, bit_width: 1, access: Some(ReadWrite), description: Some("Channel 0 Trigger Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The generation of the channel trigger is disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The generation of the channel trigger is enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CH1TRIG", bit_offset: 5, bit_width: 1, access: Some(ReadWrite), description: Some("Channel 1 Trigger Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The generation of the channel trigger is disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The generation of the channel trigger is enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "INITTRIGEN", bit_offset: 6, bit_width: 1, access: Some(ReadWrite), description: Some("Initialization Trigger Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The generation of initialization trigger is disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The generation of initialization trigger is enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TRIGF", bit_offset: 7, bit_width: 1, access: Some(ReadOnly), description: Some("Channel Trigger Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No channel trigger was generated.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("A channel trigger was generated.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "POL", offset: 112, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Channels Polarity"), fields: [Field { name: "POL", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("Channel n Polarity"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The channel polarity is active high.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The channel polarity is active low.") }], links: [], dim: Some(8), dim_increment: Some(1), dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "FMS", offset: 116, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Fault Mode Status"), fields: [Field { name: "FAULTF0", bit_offset: 0, bit_width: 1, access: Some(ReadOnly), description: Some("Fault Detection Flag 0"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No fault condition was detected at the fault input.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("A fault condition was detected at the fault input.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "FAULTF1", bit_offset: 1, bit_width: 1, access: Some(ReadOnly), description: Some("Fault Detection Flag 1"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No fault condition was detected at the fault input.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("A fault condition was detected at the fault input.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "FAULTF2", bit_offset: 2, bit_width: 1, access: Some(ReadOnly), description: Some("Fault Detection Flag 2"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No fault condition was detected at the fault input.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("A fault condition was detected at the fault input.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "FAULTF3", bit_offset: 3, bit_width: 1, access: Some(ReadOnly), description: Some("Fault Detection Flag 3"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No fault condition was detected at the fault input.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("A fault condition was detected at the fault input.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "FAULTIN", bit_offset: 5, bit_width: 1, access: Some(ReadOnly), description: Some("Fault Inputs"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The logic OR of the enabled fault inputs is 0.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The logic OR of the enabled fault inputs is 1.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "WPEN", bit_offset: 6, bit_width: 1, access: Some(ReadWrite), description: Some("Write Protection Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Write protection is disabled. Write protected bits can be written.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Write protection is enabled. Write protected bits cannot be written.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "FAULTF", bit_offset: 7, bit_width: 1, access: Some(ReadOnly), description: Some("Fault Detection Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No fault condition was detected.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("A fault condition was detected.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "FILTER", offset: 120, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Input Capture Filter Control"), fields: [Field { name: "CHFVAL", bit_offset: 0, bit_width: 4, access: Some(ReadWrite), description: Some("Channel n Input Filter"), enumerated_values: [], links: [], dim: Some(4), dim_increment: Some(4), dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "FLTCTRL", offset: 124, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Fault Control"), fields: [Field { name: "FAULTEN", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("Fault Input n Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Fault input is disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Fault input is enabled.") }], links: [], dim: Some(4), dim_increment: Some(1), dim_index: None }, Field { name: "FFLTREN", bit_offset: 4, bit_width: 1, access: Some(ReadWrite), description: Some("Fault Input 0 Filter Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Fault input filter is disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Fault input filter is enabled.") }], links: [], dim: Some(4), dim_increment: Some(1), dim_index: None }, Field { name: "FFVAL", bit_offset: 8, bit_width: 4, access: Some(ReadWrite), description: Some("Fault Input Filter"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "QDCTRL", offset: 128, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Quadrature Decoder Control And Status"), fields: [Field { name: "QUADEN", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("Quadrature Decoder Mode Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Quadrature Decoder mode is disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Quadrature Decoder mode is enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TOFDIR", bit_offset: 1, bit_width: 1, access: Some(ReadOnly), description: Some("Timer Overflow Direction In Quadrature Decoder Mode"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("TOF bit was set on the bottom of counting. There was an FTM counter decrement and FTM counter changes from its minimum value (CNTIN register) to its maximum value (MOD register).") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("TOF bit was set on the top of counting. There was an FTM counter increment and FTM counter changes from its maximum value (MOD register) to its minimum value (CNTIN register).") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "QUADIR", bit_offset: 2, bit_width: 1, access: Some(ReadOnly), description: Some("FTM Counter Direction In Quadrature Decoder Mode"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Counting direction is decreasing (FTM counter decrement).") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Counting direction is increasing (FTM counter increment).") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "QUADMODE", bit_offset: 3, bit_width: 1, access: Some(ReadWrite), description: Some("Quadrature Decoder Mode"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Phase A and phase B encoding mode.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Count and direction encoding mode.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "PHBPOL", bit_offset: 4, bit_width: 1, access: Some(ReadWrite), description: Some("Phase B Input Polarity"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Normal polarity. Phase B input signal is not inverted before identifying the rising and falling edges of this signal.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Inverted polarity. Phase B input signal is inverted before identifying the rising and falling edges of this signal.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "PHAPOL", bit_offset: 5, bit_width: 1, access: Some(ReadWrite), description: Some("Phase A Input Polarity"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Normal polarity. Phase A input signal is not inverted before identifying the rising and falling edges of this signal.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Inverted polarity. Phase A input signal is inverted before identifying the rising and falling edges of this signal.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "PHBFLTREN", bit_offset: 6, bit_width: 1, access: Some(ReadWrite), description: Some("Phase B Input Filter Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Phase B input filter is disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Phase B input filter is enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "PHAFLTREN", bit_offset: 7, bit_width: 1, access: Some(ReadWrite), description: Some("Phase A Input Filter Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Phase A input filter is disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Phase A input filter is enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "CONF", offset: 132, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Configuration"), fields: [Field { name: "NUMTOF", bit_offset: 0, bit_width: 5, access: Some(ReadWrite), description: Some("TOF Frequency"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "BDMMODE", bit_offset: 6, bit_width: 2, access: Some(ReadWrite), description: Some("BDM Mode"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "GTBEEN", bit_offset: 9, bit_width: 1, access: Some(ReadWrite), description: Some("Global Time Base Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Use of an external global time base is disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Use of an external global time base is enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "GTBEOUT", bit_offset: 10, bit_width: 1, access: Some(ReadWrite), description: Some("Global Time Base Output"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("A global time base signal generation is disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("A global time base signal generation is enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "FLTPOL", offset: 136, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("FTM Fault Input Polarity"), fields: [Field { name: "FLTPOL", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("Fault Input 0 Polarity"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The fault input polarity is active high. A 1 at the fault input indicates a fault.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The fault input polarity is active low. A 0 at the fault input indicates a fault.") }], links: [], dim: Some(4), dim_increment: Some(1), dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "SYNCONF", offset: 140, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Synchronization Configuration"), fields: [Field { name: "HWTRIGMODE", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("Hardware Trigger Mode"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("FTM clears the TRIGj bit when the hardware trigger j is detected, where j = 0, 1,2.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("FTM does not clear the TRIGj bit when the hardware trigger j is detected, where j = 0, 1,2.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CNTINC", bit_offset: 2, bit_width: 1, access: Some(ReadWrite), description: Some("CNTIN Register Synchronization"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("CNTIN register is updated with its buffer value at all rising edges of system clock.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("CNTIN register is updated with its buffer value by the PWM synchronization.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "INVC", bit_offset: 4, bit_width: 1, access: Some(ReadWrite), description: Some("INVCTRL Register Synchronization"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("INVCTRL register is updated with its buffer value at all rising edges of system clock.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("INVCTRL register is updated with its buffer value by the PWM synchronization.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SWOC", bit_offset: 5, bit_width: 1, access: Some(ReadWrite), description: Some("SWOCTRL Register Synchronization"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("SWOCTRL register is updated with its buffer value at all rising edges of system clock.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("SWOCTRL register is updated with its buffer value by the PWM synchronization.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SYNCMODE", bit_offset: 7, bit_width: 1, access: Some(ReadWrite), description: Some("Synchronization Mode"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Legacy PWM synchronization is selected.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Enhanced PWM synchronization is selected.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SWRSTCNT", bit_offset: 8, bit_width: 1, access: Some(ReadWrite), description: Some("FTM counter synchronization is activated by the software trigger."), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The software trigger does not activate the FTM counter synchronization.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The software trigger activates the FTM counter synchronization.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SWWRBUF", bit_offset: 9, bit_width: 1, access: Some(ReadWrite), description: Some("MOD, CNTIN, and CV registers synchronization is activated by the software trigger."), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The software trigger does not activate MOD, CNTIN, and CV registers synchronization.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The software trigger activates MOD, CNTIN, and CV registers synchronization.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SWOM", bit_offset: 10, bit_width: 1, access: Some(ReadWrite), description: Some("Output mask synchronization is activated by the software trigger."), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The software trigger does not activate the OUTMASK register synchronization.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The software trigger activates the OUTMASK register synchronization.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SWINVC", bit_offset: 11, bit_width: 1, access: Some(ReadWrite), description: Some("Inverting control synchronization is activated by the software trigger."), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The software trigger does not activate the INVCTRL register synchronization.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The software trigger activates the INVCTRL register synchronization.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SWSOC", bit_offset: 12, bit_width: 1, access: Some(ReadWrite), description: Some("Software output control synchronization is activated by the software trigger."), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The software trigger does not activate the SWOCTRL register synchronization.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The software trigger activates the SWOCTRL register synchronization.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "HWRSTCNT", bit_offset: 16, bit_width: 1, access: Some(ReadWrite), description: Some("FTM counter synchronization is activated by a hardware trigger."), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("A hardware trigger does not activate the FTM counter synchronization.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("A hardware trigger activates the FTM counter synchronization.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "HWWRBUF", bit_offset: 17, bit_width: 1, access: Some(ReadWrite), description: Some("MOD, CNTIN, and CV registers synchronization is activated by a hardware trigger."), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("A hardware trigger does not activate MOD, CNTIN, and CV registers synchronization.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("A hardware trigger activates MOD, CNTIN, and CV registers synchronization.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "HWOM", bit_offset: 18, bit_width: 1, access: Some(ReadWrite), description: Some("Output mask synchronization is activated by a hardware trigger."), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("A hardware trigger does not activate the OUTMASK register synchronization.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("A hardware trigger activates the OUTMASK register synchronization.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "HWINVC", bit_offset: 19, bit_width: 1, access: Some(ReadWrite), description: Some("Inverting control synchronization is activated by a hardware trigger."), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("A hardware trigger does not activate the INVCTRL register synchronization.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("A hardware trigger activates the INVCTRL register synchronization.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "HWSOC", bit_offset: 20, bit_width: 1, access: Some(ReadWrite), description: Some("Software output control synchronization is activated by a hardware trigger."), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("A hardware trigger does not activate the SWOCTRL register synchronization.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("A hardware trigger activates the SWOCTRL register synchronization.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "INVCTRL", offset: 144, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("FTM Inverting Control"), fields: [Field { name: "INVEN", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("Pair Channels n Inverting Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Inverting is disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Inverting is enabled.") }], links: [], dim: Some(4), dim_increment: Some(1), dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "SWOCTRL", offset: 148, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("FTM Software Output Control"), fields: [Field { name: "CHOC", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("Channel 0 Software Output Control Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The channel output is not affected by software output control.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The channel output is affected by software output control.") }], links: [], dim: Some(8), dim_increment: Some(1), dim_index: None }, Field { name: "CHOCV", bit_offset: 8, bit_width: 1, access: Some(ReadWrite), description: Some("Channel n Software Output Control Value"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The software output control forces 0 to the channel output.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The software output control forces 1 to the channel output.") }], links: [], dim: Some(8), dim_increment: Some(1), dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "PWMLOAD", offset: 152, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("FTM PWM Load"), fields: [Field { name: "CHSEL", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("Channel n Select"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Do not include the channel in the matching process.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Include the channel in the matching process.") }], links: [], dim: Some(8), dim_increment: Some(1), dim_index: None }, Field { name: "LDOK", bit_offset: 9, bit_width: 1, access: Some(ReadWrite), description: Some("Load Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Loading updated values is disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Loading updated values is enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }), modules: [], has_pins: false, has_channels: true, description: None }

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="FTM Peripheral"]
pub struct FtmPeriph(pub usize); 


impl FtmPeriph {
#[doc="Get the *const pointer for the SC register."]
   #[inline] pub fn sc_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x0) as *const u32
   }
#[doc="Get the *mut pointer for the SC register."]
   #[inline] pub fn sc_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x0) as *mut u32
   }
#[doc="Read the SC register."]
   #[inline] pub fn sc(&self) -> Sc { 
      unsafe {
         Sc(::core::ptr::read_volatile((self.0 + 0x0) as *const u32))
      }
   }
#[doc="Write the SC register."]
   #[inline] pub fn set_sc<F: FnOnce(Sc) -> Sc>(&self, f: F) -> &Self {
      let value = f(Sc(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x0) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SC register."]
   #[inline] pub fn with_sc<F: FnOnce(Sc) -> Sc>(&self, f: F) -> &Self {
      let tmp = self.sc();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x0) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CNT register."]
   #[inline] pub fn cnt_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x4) as *const u32
   }
#[doc="Get the *mut pointer for the CNT register."]
   #[inline] pub fn cnt_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x4) as *mut u32
   }
#[doc="Read the CNT register."]
   #[inline] pub fn cnt(&self) -> Cnt { 
      unsafe {
         Cnt(::core::ptr::read_volatile((self.0 + 0x4) as *const u32))
      }
   }
#[doc="Write the CNT register."]
   #[inline] pub fn set_cnt<F: FnOnce(Cnt) -> Cnt>(&self, f: F) -> &Self {
      let value = f(Cnt(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x4) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CNT register."]
   #[inline] pub fn with_cnt<F: FnOnce(Cnt) -> Cnt>(&self, f: F) -> &Self {
      let tmp = self.cnt();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x4) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the MOD register."]
   #[inline] pub fn mod_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x8) as *const u32
   }
#[doc="Get the *mut pointer for the MOD register."]
   #[inline] pub fn mod_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x8) as *mut u32
   }
#[doc="Read the MOD register."]
   #[inline] pub fn _mod(&self) -> Mod { 
      unsafe {
         Mod(::core::ptr::read_volatile((self.0 + 0x8) as *const u32))
      }
   }
#[doc="Write the MOD register."]
   #[inline] pub fn set_mod<F: FnOnce(Mod) -> Mod>(&self, f: F) -> &Self {
      let value = f(Mod(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x8) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the MOD register."]
   #[inline] pub fn with_mod<F: FnOnce(Mod) -> Mod>(&self, f: F) -> &Self {
      let tmp = self._mod();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x8) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CSC register."]
   #[inline] pub fn csc_ptr<I: Into<bits::R8>>(&self, index: I) -> *const u32 { 
      let index: bits::R8 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0xc + (index << 3)) as *const u32
   }
#[doc="Get the *mut pointer for the CSC register."]
   #[inline] pub fn csc_mut<I: Into<bits::R8>>(&self, index: I) -> *mut u32 { 
      let index: bits::R8 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0xc + (index << 3)) as *mut u32
   }
#[doc="Read the CSC register."]
   #[inline] pub fn csc<I: Into<bits::R8>>(&self, index: I) -> Csc { 
      let index: bits::R8 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         Csc(::core::ptr::read_volatile((self.0 + 0xc + (index << 3)) as *const u32))
      }
   }
#[doc="Write the CSC register."]
   #[inline] pub fn set_csc<I: Into<bits::R8>, F: FnOnce(Csc) -> Csc>(&self, index: I, f: F) -> &Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value() as usize;
      let value = f(Csc(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xc + (index << 3)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CSC register."]
   #[inline] pub fn with_csc<I: Into<bits::R8> + Copy, F: FnOnce(Csc) -> Csc>(&self, index: I, f: F) -> &Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.csc(index);
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xc + (index << 3)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CV register."]
   #[inline] pub fn cv_ptr<I: Into<bits::R8>>(&self, index: I) -> *const u32 { 
      let index: bits::R8 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x10 + (index << 3)) as *const u32
   }
#[doc="Get the *mut pointer for the CV register."]
   #[inline] pub fn cv_mut<I: Into<bits::R8>>(&self, index: I) -> *mut u32 { 
      let index: bits::R8 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x10 + (index << 3)) as *mut u32
   }
#[doc="Read the CV register."]
   #[inline] pub fn cv<I: Into<bits::R8>>(&self, index: I) -> Cv { 
      let index: bits::R8 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         Cv(::core::ptr::read_volatile((self.0 + 0x10 + (index << 3)) as *const u32))
      }
   }
#[doc="Write the CV register."]
   #[inline] pub fn set_cv<I: Into<bits::R8>, F: FnOnce(Cv) -> Cv>(&self, index: I, f: F) -> &Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value() as usize;
      let value = f(Cv(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x10 + (index << 3)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CV register."]
   #[inline] pub fn with_cv<I: Into<bits::R8> + Copy, F: FnOnce(Cv) -> Cv>(&self, index: I, f: F) -> &Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.cv(index);
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x10 + (index << 3)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CNTIN register."]
   #[inline] pub fn cntin_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x4c) as *const u32
   }
#[doc="Get the *mut pointer for the CNTIN register."]
   #[inline] pub fn cntin_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x4c) as *mut u32
   }
#[doc="Read the CNTIN register."]
   #[inline] pub fn cntin(&self) -> Cntin { 
      unsafe {
         Cntin(::core::ptr::read_volatile((self.0 + 0x4c) as *const u32))
      }
   }
#[doc="Write the CNTIN register."]
   #[inline] pub fn set_cntin<F: FnOnce(Cntin) -> Cntin>(&self, f: F) -> &Self {
      let value = f(Cntin(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x4c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CNTIN register."]
   #[inline] pub fn with_cntin<F: FnOnce(Cntin) -> Cntin>(&self, f: F) -> &Self {
      let tmp = self.cntin();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x4c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the STATUS register."]
   #[inline] pub fn status_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x50) as *const u32
   }
#[doc="Get the *mut pointer for the STATUS register."]
   #[inline] pub fn status_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x50) as *mut u32
   }
#[doc="Read the STATUS register."]
   #[inline] pub fn status(&self) -> Status { 
      unsafe {
         Status(::core::ptr::read_volatile((self.0 + 0x50) as *const u32))
      }
   }
#[doc="Write the STATUS register."]
   #[inline] pub fn set_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
      let value = f(Status(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x50) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the STATUS register."]
   #[inline] pub fn with_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
      let tmp = self.status();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x50) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the MODE register."]
   #[inline] pub fn mode_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x54) as *const u32
   }
#[doc="Get the *mut pointer for the MODE register."]
   #[inline] pub fn mode_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x54) as *mut u32
   }
#[doc="Read the MODE register."]
   #[inline] pub fn mode(&self) -> Mode { 
      unsafe {
         Mode(::core::ptr::read_volatile((self.0 + 0x54) as *const u32))
      }
   }
#[doc="Write the MODE register."]
   #[inline] pub fn set_mode<F: FnOnce(Mode) -> Mode>(&self, f: F) -> &Self {
      let value = f(Mode(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x54) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the MODE register."]
   #[inline] pub fn with_mode<F: FnOnce(Mode) -> Mode>(&self, f: F) -> &Self {
      let tmp = self.mode();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x54) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SYNC register."]
   #[inline] pub fn sync_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x58) as *const u32
   }
#[doc="Get the *mut pointer for the SYNC register."]
   #[inline] pub fn sync_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x58) as *mut u32
   }
#[doc="Read the SYNC register."]
   #[inline] pub fn sync(&self) -> Sync { 
      unsafe {
         Sync(::core::ptr::read_volatile((self.0 + 0x58) as *const u32))
      }
   }
#[doc="Write the SYNC register."]
   #[inline] pub fn set_sync<F: FnOnce(Sync) -> Sync>(&self, f: F) -> &Self {
      let value = f(Sync(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x58) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SYNC register."]
   #[inline] pub fn with_sync<F: FnOnce(Sync) -> Sync>(&self, f: F) -> &Self {
      let tmp = self.sync();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x58) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the OUTINIT register."]
   #[inline] pub fn outinit_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x5c) as *const u32
   }
#[doc="Get the *mut pointer for the OUTINIT register."]
   #[inline] pub fn outinit_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x5c) as *mut u32
   }
#[doc="Read the OUTINIT register."]
   #[inline] pub fn outinit(&self) -> Outinit { 
      unsafe {
         Outinit(::core::ptr::read_volatile((self.0 + 0x5c) as *const u32))
      }
   }
#[doc="Write the OUTINIT register."]
   #[inline] pub fn set_outinit<F: FnOnce(Outinit) -> Outinit>(&self, f: F) -> &Self {
      let value = f(Outinit(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x5c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the OUTINIT register."]
   #[inline] pub fn with_outinit<F: FnOnce(Outinit) -> Outinit>(&self, f: F) -> &Self {
      let tmp = self.outinit();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x5c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the OUTMASK register."]
   #[inline] pub fn outmask_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x60) as *const u32
   }
#[doc="Get the *mut pointer for the OUTMASK register."]
   #[inline] pub fn outmask_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x60) as *mut u32
   }
#[doc="Read the OUTMASK register."]
   #[inline] pub fn outmask(&self) -> Outmask { 
      unsafe {
         Outmask(::core::ptr::read_volatile((self.0 + 0x60) as *const u32))
      }
   }
#[doc="Write the OUTMASK register."]
   #[inline] pub fn set_outmask<F: FnOnce(Outmask) -> Outmask>(&self, f: F) -> &Self {
      let value = f(Outmask(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x60) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the OUTMASK register."]
   #[inline] pub fn with_outmask<F: FnOnce(Outmask) -> Outmask>(&self, f: F) -> &Self {
      let tmp = self.outmask();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x60) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the COMBINE register."]
   #[inline] pub fn combine_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x64) as *const u32
   }
#[doc="Get the *mut pointer for the COMBINE register."]
   #[inline] pub fn combine_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x64) as *mut u32
   }
#[doc="Read the COMBINE register."]
   #[inline] pub fn combine(&self) -> Combine { 
      unsafe {
         Combine(::core::ptr::read_volatile((self.0 + 0x64) as *const u32))
      }
   }
#[doc="Write the COMBINE register."]
   #[inline] pub fn set_combine<F: FnOnce(Combine) -> Combine>(&self, f: F) -> &Self {
      let value = f(Combine(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x64) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the COMBINE register."]
   #[inline] pub fn with_combine<F: FnOnce(Combine) -> Combine>(&self, f: F) -> &Self {
      let tmp = self.combine();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x64) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DEADTIME register."]
   #[inline] pub fn deadtime_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x68) as *const u32
   }
#[doc="Get the *mut pointer for the DEADTIME register."]
   #[inline] pub fn deadtime_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x68) as *mut u32
   }
#[doc="Read the DEADTIME register."]
   #[inline] pub fn deadtime(&self) -> Deadtime { 
      unsafe {
         Deadtime(::core::ptr::read_volatile((self.0 + 0x68) as *const u32))
      }
   }
#[doc="Write the DEADTIME register."]
   #[inline] pub fn set_deadtime<F: FnOnce(Deadtime) -> Deadtime>(&self, f: F) -> &Self {
      let value = f(Deadtime(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x68) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DEADTIME register."]
   #[inline] pub fn with_deadtime<F: FnOnce(Deadtime) -> Deadtime>(&self, f: F) -> &Self {
      let tmp = self.deadtime();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x68) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the EXTTRIG register."]
   #[inline] pub fn exttrig_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x6c) as *const u32
   }
#[doc="Get the *mut pointer for the EXTTRIG register."]
   #[inline] pub fn exttrig_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x6c) as *mut u32
   }
#[doc="Read the EXTTRIG register."]
   #[inline] pub fn exttrig(&self) -> Exttrig { 
      unsafe {
         Exttrig(::core::ptr::read_volatile((self.0 + 0x6c) as *const u32))
      }
   }
#[doc="Write the EXTTRIG register."]
   #[inline] pub fn set_exttrig<F: FnOnce(Exttrig) -> Exttrig>(&self, f: F) -> &Self {
      let value = f(Exttrig(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x6c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the EXTTRIG register."]
   #[inline] pub fn with_exttrig<F: FnOnce(Exttrig) -> Exttrig>(&self, f: F) -> &Self {
      let tmp = self.exttrig();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x6c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the POL register."]
   #[inline] pub fn pol_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x70) as *const u32
   }
#[doc="Get the *mut pointer for the POL register."]
   #[inline] pub fn pol_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x70) as *mut u32
   }
#[doc="Read the POL register."]
   #[inline] pub fn pol(&self) -> Pol { 
      unsafe {
         Pol(::core::ptr::read_volatile((self.0 + 0x70) as *const u32))
      }
   }
#[doc="Write the POL register."]
   #[inline] pub fn set_pol<F: FnOnce(Pol) -> Pol>(&self, f: F) -> &Self {
      let value = f(Pol(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x70) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the POL register."]
   #[inline] pub fn with_pol<F: FnOnce(Pol) -> Pol>(&self, f: F) -> &Self {
      let tmp = self.pol();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x70) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the FMS register."]
   #[inline] pub fn fms_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x74) as *const u32
   }
#[doc="Get the *mut pointer for the FMS register."]
   #[inline] pub fn fms_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x74) as *mut u32
   }
#[doc="Read the FMS register."]
   #[inline] pub fn fms(&self) -> Fms { 
      unsafe {
         Fms(::core::ptr::read_volatile((self.0 + 0x74) as *const u32))
      }
   }
#[doc="Write the FMS register."]
   #[inline] pub fn set_fms<F: FnOnce(Fms) -> Fms>(&self, f: F) -> &Self {
      let value = f(Fms(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x74) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the FMS register."]
   #[inline] pub fn with_fms<F: FnOnce(Fms) -> Fms>(&self, f: F) -> &Self {
      let tmp = self.fms();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x74) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the FILTER register."]
   #[inline] pub fn filter_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x78) as *const u32
   }
#[doc="Get the *mut pointer for the FILTER register."]
   #[inline] pub fn filter_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x78) as *mut u32
   }
#[doc="Read the FILTER register."]
   #[inline] pub fn filter(&self) -> Filter { 
      unsafe {
         Filter(::core::ptr::read_volatile((self.0 + 0x78) as *const u32))
      }
   }
#[doc="Write the FILTER register."]
   #[inline] pub fn set_filter<F: FnOnce(Filter) -> Filter>(&self, f: F) -> &Self {
      let value = f(Filter(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x78) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the FILTER register."]
   #[inline] pub fn with_filter<F: FnOnce(Filter) -> Filter>(&self, f: F) -> &Self {
      let tmp = self.filter();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x78) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the FLTCTRL register."]
   #[inline] pub fn fltctrl_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x7c) as *const u32
   }
#[doc="Get the *mut pointer for the FLTCTRL register."]
   #[inline] pub fn fltctrl_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x7c) as *mut u32
   }
#[doc="Read the FLTCTRL register."]
   #[inline] pub fn fltctrl(&self) -> Fltctrl { 
      unsafe {
         Fltctrl(::core::ptr::read_volatile((self.0 + 0x7c) as *const u32))
      }
   }
#[doc="Write the FLTCTRL register."]
   #[inline] pub fn set_fltctrl<F: FnOnce(Fltctrl) -> Fltctrl>(&self, f: F) -> &Self {
      let value = f(Fltctrl(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x7c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the FLTCTRL register."]
   #[inline] pub fn with_fltctrl<F: FnOnce(Fltctrl) -> Fltctrl>(&self, f: F) -> &Self {
      let tmp = self.fltctrl();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x7c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the QDCTRL register."]
   #[inline] pub fn qdctrl_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x80) as *const u32
   }
#[doc="Get the *mut pointer for the QDCTRL register."]
   #[inline] pub fn qdctrl_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x80) as *mut u32
   }
#[doc="Read the QDCTRL register."]
   #[inline] pub fn qdctrl(&self) -> Qdctrl { 
      unsafe {
         Qdctrl(::core::ptr::read_volatile((self.0 + 0x80) as *const u32))
      }
   }
#[doc="Write the QDCTRL register."]
   #[inline] pub fn set_qdctrl<F: FnOnce(Qdctrl) -> Qdctrl>(&self, f: F) -> &Self {
      let value = f(Qdctrl(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x80) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the QDCTRL register."]
   #[inline] pub fn with_qdctrl<F: FnOnce(Qdctrl) -> Qdctrl>(&self, f: F) -> &Self {
      let tmp = self.qdctrl();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x80) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CONF register."]
   #[inline] pub fn conf_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x84) as *const u32
   }
#[doc="Get the *mut pointer for the CONF register."]
   #[inline] pub fn conf_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x84) as *mut u32
   }
#[doc="Read the CONF register."]
   #[inline] pub fn conf(&self) -> Conf { 
      unsafe {
         Conf(::core::ptr::read_volatile((self.0 + 0x84) as *const u32))
      }
   }
#[doc="Write the CONF register."]
   #[inline] pub fn set_conf<F: FnOnce(Conf) -> Conf>(&self, f: F) -> &Self {
      let value = f(Conf(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x84) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CONF register."]
   #[inline] pub fn with_conf<F: FnOnce(Conf) -> Conf>(&self, f: F) -> &Self {
      let tmp = self.conf();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x84) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the FLTPOL register."]
   #[inline] pub fn fltpol_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x88) as *const u32
   }
#[doc="Get the *mut pointer for the FLTPOL register."]
   #[inline] pub fn fltpol_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x88) as *mut u32
   }
#[doc="Read the FLTPOL register."]
   #[inline] pub fn fltpol(&self) -> Fltpol { 
      unsafe {
         Fltpol(::core::ptr::read_volatile((self.0 + 0x88) as *const u32))
      }
   }
#[doc="Write the FLTPOL register."]
   #[inline] pub fn set_fltpol<F: FnOnce(Fltpol) -> Fltpol>(&self, f: F) -> &Self {
      let value = f(Fltpol(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x88) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the FLTPOL register."]
   #[inline] pub fn with_fltpol<F: FnOnce(Fltpol) -> Fltpol>(&self, f: F) -> &Self {
      let tmp = self.fltpol();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x88) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SYNCONF register."]
   #[inline] pub fn synconf_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x8c) as *const u32
   }
#[doc="Get the *mut pointer for the SYNCONF register."]
   #[inline] pub fn synconf_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x8c) as *mut u32
   }
#[doc="Read the SYNCONF register."]
   #[inline] pub fn synconf(&self) -> Synconf { 
      unsafe {
         Synconf(::core::ptr::read_volatile((self.0 + 0x8c) as *const u32))
      }
   }
#[doc="Write the SYNCONF register."]
   #[inline] pub fn set_synconf<F: FnOnce(Synconf) -> Synconf>(&self, f: F) -> &Self {
      let value = f(Synconf(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x8c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SYNCONF register."]
   #[inline] pub fn with_synconf<F: FnOnce(Synconf) -> Synconf>(&self, f: F) -> &Self {
      let tmp = self.synconf();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x8c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the INVCTRL register."]
   #[inline] pub fn invctrl_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x90) as *const u32
   }
#[doc="Get the *mut pointer for the INVCTRL register."]
   #[inline] pub fn invctrl_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x90) as *mut u32
   }
#[doc="Read the INVCTRL register."]
   #[inline] pub fn invctrl(&self) -> Invctrl { 
      unsafe {
         Invctrl(::core::ptr::read_volatile((self.0 + 0x90) as *const u32))
      }
   }
#[doc="Write the INVCTRL register."]
   #[inline] pub fn set_invctrl<F: FnOnce(Invctrl) -> Invctrl>(&self, f: F) -> &Self {
      let value = f(Invctrl(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x90) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the INVCTRL register."]
   #[inline] pub fn with_invctrl<F: FnOnce(Invctrl) -> Invctrl>(&self, f: F) -> &Self {
      let tmp = self.invctrl();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x90) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SWOCTRL register."]
   #[inline] pub fn swoctrl_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x94) as *const u32
   }
#[doc="Get the *mut pointer for the SWOCTRL register."]
   #[inline] pub fn swoctrl_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x94) as *mut u32
   }
#[doc="Read the SWOCTRL register."]
   #[inline] pub fn swoctrl(&self) -> Swoctrl { 
      unsafe {
         Swoctrl(::core::ptr::read_volatile((self.0 + 0x94) as *const u32))
      }
   }
#[doc="Write the SWOCTRL register."]
   #[inline] pub fn set_swoctrl<F: FnOnce(Swoctrl) -> Swoctrl>(&self, f: F) -> &Self {
      let value = f(Swoctrl(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x94) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SWOCTRL register."]
   #[inline] pub fn with_swoctrl<F: FnOnce(Swoctrl) -> Swoctrl>(&self, f: F) -> &Self {
      let tmp = self.swoctrl();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x94) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PWMLOAD register."]
   #[inline] pub fn pwmload_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x98) as *const u32
   }
#[doc="Get the *mut pointer for the PWMLOAD register."]
   #[inline] pub fn pwmload_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x98) as *mut u32
   }
#[doc="Read the PWMLOAD register."]
   #[inline] pub fn pwmload(&self) -> Pwmload { 
      unsafe {
         Pwmload(::core::ptr::read_volatile((self.0 + 0x98) as *const u32))
      }
   }
#[doc="Write the PWMLOAD register."]
   #[inline] pub fn set_pwmload<F: FnOnce(Pwmload) -> Pwmload>(&self, f: F) -> &Self {
      let value = f(Pwmload(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x98) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PWMLOAD register."]
   #[inline] pub fn with_pwmload<F: FnOnce(Pwmload) -> Pwmload>(&self, f: F) -> &Self {
      let tmp = self.pwmload();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x98) as *mut u32, value.0);
      }
      self
   }

}

#[doc="Status And Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sc(pub u32);
impl Sc {
#[doc="Prescale Factor Selection"]
   #[inline] pub fn ps(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
   }
#[doc="Prescale Factor Selection"]
   #[inline] pub fn set_ps<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Clock Source Selection"]
   #[inline] pub fn clks(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
   }
#[doc="Clock Source Selection"]
   #[inline] pub fn set_clks<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Center-Aligned PWM Select"]
   #[inline] pub fn cpwms(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Center-Aligned PWM Select"]
   #[inline] pub fn set_cpwms<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Timer Overflow Interrupt Enable"]
   #[inline] pub fn toie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Timer Overflow Interrupt Enable"]
   #[inline] pub fn set_toie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Timer Overflow Flag"]
   #[inline] pub fn tof(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Timer Overflow Flag"]
   #[inline] pub fn set_tof<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="PWM Enable"]
   #[inline] pub fn pwmen<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 16 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [16]
   }
#[doc="PWM Enable"]
   #[inline] pub fn set_pwmen<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 16 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

#[doc="Filter Prescaler"]
   #[inline] pub fn fltps(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
   }
#[doc="Filter Prescaler"]
   #[inline] pub fn set_fltps<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 24);
      self.0 |= value << 24;
      self
   }

}
impl ::core::fmt::Display for Sc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ps() != 0 { try!(write!(f, " ps=0x{:x}", self.ps()))}
      if self.clks() != 0 { try!(write!(f, " clks=0x{:x}", self.clks()))}
      if self.cpwms() != 0 { try!(write!(f, " cpwms"))}
      if self.toie() != 0 { try!(write!(f, " toie"))}
      if self.tof() != 0 { try!(write!(f, " tof"))}
      if self.pwmen(0) != 0 { try!(write!(f, " pwmen[0]"))}
      if self.pwmen(1) != 0 { try!(write!(f, " pwmen[1]"))}
      if self.pwmen(2) != 0 { try!(write!(f, " pwmen[2]"))}
      if self.pwmen(3) != 0 { try!(write!(f, " pwmen[3]"))}
      if self.pwmen(4) != 0 { try!(write!(f, " pwmen[4]"))}
      if self.pwmen(5) != 0 { try!(write!(f, " pwmen[5]"))}
      if self.pwmen(6) != 0 { try!(write!(f, " pwmen[6]"))}
      if self.pwmen(7) != 0 { try!(write!(f, " pwmen[7]"))}
      if self.fltps() != 0 { try!(write!(f, " fltps=0x{:x}", self.fltps()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Counter"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cnt(pub u32);
impl Cnt {
#[doc="Counter Value"]
   #[inline] pub fn count(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Counter Value"]
   #[inline] pub fn set_count<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Cnt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cnt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Modulo"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Mod(pub u32);
impl Mod {
#[doc="Modulo Value"]
   #[inline] pub fn _mod(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Modulo Value"]
   #[inline] pub fn set_mod<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Mod {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Mod {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self._mod() != 0 { try!(write!(f, " mod=0x{:x}", self._mod()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Channel (n) Status And Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Csc(pub u32);
impl Csc {
#[doc="DMA Enable"]
   #[inline] pub fn dma(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="DMA Enable"]
   #[inline] pub fn set_dma<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Edge or Level Select"]
   #[inline] pub fn elsa(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Edge or Level Select"]
   #[inline] pub fn set_elsa<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Edge or Level Select"]
   #[inline] pub fn elsb(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Edge or Level Select"]
   #[inline] pub fn set_elsb<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Channel Mode Select"]
   #[inline] pub fn msa(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Channel Mode Select"]
   #[inline] pub fn set_msa<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Channel Mode Select"]
   #[inline] pub fn msb(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Channel Mode Select"]
   #[inline] pub fn set_msb<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Channel Interrupt Enable"]
   #[inline] pub fn chie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Channel Interrupt Enable"]
   #[inline] pub fn set_chie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Channel Flag"]
   #[inline] pub fn chf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Channel Flag"]
   #[inline] pub fn set_chf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Csc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Csc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dma() != 0 { try!(write!(f, " dma"))}
      if self.elsa() != 0 { try!(write!(f, " elsa"))}
      if self.elsb() != 0 { try!(write!(f, " elsb"))}
      if self.msa() != 0 { try!(write!(f, " msa"))}
      if self.msb() != 0 { try!(write!(f, " msb"))}
      if self.chie() != 0 { try!(write!(f, " chie"))}
      if self.chf() != 0 { try!(write!(f, " chf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Channel (n) Value"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cv(pub u32);
impl Cv {
#[doc="Channel Value"]
   #[inline] pub fn val(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Channel Value"]
   #[inline] pub fn set_val<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Cv {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cv {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.val() != 0 { try!(write!(f, " val=0x{:x}", self.val()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Counter Initial Value"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cntin(pub u32);
impl Cntin {
#[doc="Initial Value Of The FTM Counter"]
   #[inline] pub fn init(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Initial Value Of The FTM Counter"]
   #[inline] pub fn set_init<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Cntin {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cntin {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.init() != 0 { try!(write!(f, " init=0x{:x}", self.init()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Capture And Compare Status"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u32);
impl Status {
#[doc="Channel n Flag"]
   #[inline] pub fn chf<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="Channel n Flag"]
   #[inline] pub fn set_chf<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
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
impl ::core::fmt::Display for Status {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Status {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.chf(0) != 0 { try!(write!(f, " chf[0]"))}
      if self.chf(1) != 0 { try!(write!(f, " chf[1]"))}
      if self.chf(2) != 0 { try!(write!(f, " chf[2]"))}
      if self.chf(3) != 0 { try!(write!(f, " chf[3]"))}
      if self.chf(4) != 0 { try!(write!(f, " chf[4]"))}
      if self.chf(5) != 0 { try!(write!(f, " chf[5]"))}
      if self.chf(6) != 0 { try!(write!(f, " chf[6]"))}
      if self.chf(7) != 0 { try!(write!(f, " chf[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Features Mode Selection"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Mode(pub u32);
impl Mode {
#[doc="FTM Enable"]
   #[inline] pub fn ftmen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="FTM Enable"]
   #[inline] pub fn set_ftmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Initialize The Channels Output"]
   #[inline] pub fn init(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Initialize The Channels Output"]
   #[inline] pub fn set_init<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Write Protection Disable"]
   #[inline] pub fn wpdis(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Write Protection Disable"]
   #[inline] pub fn set_wpdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="PWM Synchronization Mode"]
   #[inline] pub fn pwmsync(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="PWM Synchronization Mode"]
   #[inline] pub fn set_pwmsync<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Capture Test Mode Enable"]
   #[inline] pub fn captest(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Capture Test Mode Enable"]
   #[inline] pub fn set_captest<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Fault Control Mode"]
   #[inline] pub fn faultm(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
   }
#[doc="Fault Control Mode"]
   #[inline] pub fn set_faultm<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Fault Interrupt Enable"]
   #[inline] pub fn faultie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Fault Interrupt Enable"]
   #[inline] pub fn set_faultie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Mode {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Mode {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ftmen() != 0 { try!(write!(f, " ftmen"))}
      if self.init() != 0 { try!(write!(f, " init"))}
      if self.wpdis() != 0 { try!(write!(f, " wpdis"))}
      if self.pwmsync() != 0 { try!(write!(f, " pwmsync"))}
      if self.captest() != 0 { try!(write!(f, " captest"))}
      if self.faultm() != 0 { try!(write!(f, " faultm=0x{:x}", self.faultm()))}
      if self.faultie() != 0 { try!(write!(f, " faultie"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Synchronization"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sync(pub u32);
impl Sync {
#[doc="Minimum Loading Point Enable"]
   #[inline] pub fn cntmin(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Minimum Loading Point Enable"]
   #[inline] pub fn set_cntmin<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Maximum Loading Point Enable"]
   #[inline] pub fn cntmax(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Maximum Loading Point Enable"]
   #[inline] pub fn set_cntmax<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="FTM Counter Reinitialization By Synchronization (FTM counter synchronization)"]
   #[inline] pub fn reinit(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="FTM Counter Reinitialization By Synchronization (FTM counter synchronization)"]
   #[inline] pub fn set_reinit<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Output Mask Synchronization"]
   #[inline] pub fn synchom(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Output Mask Synchronization"]
   #[inline] pub fn set_synchom<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="PWM Synchronization Hardware Trigger 0"]
   #[inline] pub fn trig0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="PWM Synchronization Hardware Trigger 0"]
   #[inline] pub fn set_trig0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="PWM Synchronization Hardware Trigger 1"]
   #[inline] pub fn trig1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="PWM Synchronization Hardware Trigger 1"]
   #[inline] pub fn set_trig1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="PWM Synchronization Hardware Trigger 2"]
   #[inline] pub fn trig2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="PWM Synchronization Hardware Trigger 2"]
   #[inline] pub fn set_trig2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="PWM Synchronization Software Trigger"]
   #[inline] pub fn swsync(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="PWM Synchronization Software Trigger"]
   #[inline] pub fn set_swsync<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Sync {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sync {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cntmin() != 0 { try!(write!(f, " cntmin"))}
      if self.cntmax() != 0 { try!(write!(f, " cntmax"))}
      if self.reinit() != 0 { try!(write!(f, " reinit"))}
      if self.synchom() != 0 { try!(write!(f, " synchom"))}
      if self.trig0() != 0 { try!(write!(f, " trig0"))}
      if self.trig1() != 0 { try!(write!(f, " trig1"))}
      if self.trig2() != 0 { try!(write!(f, " trig2"))}
      if self.swsync() != 0 { try!(write!(f, " swsync"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Initial State For Channels Output"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Outinit(pub u32);
impl Outinit {
#[doc="Channel n Output Initialization Value"]
   #[inline] pub fn choi<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="Channel n Output Initialization Value"]
   #[inline] pub fn set_choi<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
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
impl ::core::fmt::Display for Outinit {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Outinit {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.choi(0) != 0 { try!(write!(f, " choi[0]"))}
      if self.choi(1) != 0 { try!(write!(f, " choi[1]"))}
      if self.choi(2) != 0 { try!(write!(f, " choi[2]"))}
      if self.choi(3) != 0 { try!(write!(f, " choi[3]"))}
      if self.choi(4) != 0 { try!(write!(f, " choi[4]"))}
      if self.choi(5) != 0 { try!(write!(f, " choi[5]"))}
      if self.choi(6) != 0 { try!(write!(f, " choi[6]"))}
      if self.choi(7) != 0 { try!(write!(f, " choi[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Output Mask"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Outmask(pub u32);
impl Outmask {
#[doc="Channel 0 Output Mask"]
   #[inline] pub fn chom<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="Channel 0 Output Mask"]
   #[inline] pub fn set_chom<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
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
impl ::core::fmt::Display for Outmask {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Outmask {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.chom(0) != 0 { try!(write!(f, " chom[0]"))}
      if self.chom(1) != 0 { try!(write!(f, " chom[1]"))}
      if self.chom(2) != 0 { try!(write!(f, " chom[2]"))}
      if self.chom(3) != 0 { try!(write!(f, " chom[3]"))}
      if self.chom(4) != 0 { try!(write!(f, " chom[4]"))}
      if self.chom(5) != 0 { try!(write!(f, " chom[5]"))}
      if self.chom(6) != 0 { try!(write!(f, " chom[6]"))}
      if self.chom(7) != 0 { try!(write!(f, " chom[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Function For Linked Channels"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Combine(pub u32);
impl Combine {
#[doc="Combine Channels For n"]
   #[inline] pub fn combine<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + (index << 3);
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="Combine Channels For n"]
   #[inline] pub fn set_combine<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + (index << 3);
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

#[doc="Complement Of Channel (n) For n"]
   #[inline] pub fn comp<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let shift: usize = 1 + (index << 3);
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [1]
   }
#[doc="Complement Of Channel (n) For n"]
   #[inline] pub fn set_comp<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 1 + (index << 3);
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

#[doc="Dual Edge Capture Mode Enable For n"]
   #[inline] pub fn decapen<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let shift: usize = 2 + (index << 3);
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [2]
   }
#[doc="Dual Edge Capture Mode Enable For n"]
   #[inline] pub fn set_decapen<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 2 + (index << 3);
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

#[doc="Dual Edge Capture Mode Captures For n"]
   #[inline] pub fn decap<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let shift: usize = 3 + (index << 3);
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [3]
   }
#[doc="Dual Edge Capture Mode Captures For n"]
   #[inline] pub fn set_decap<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 3 + (index << 3);
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

#[doc="Deadtime Enable For n"]
   #[inline] pub fn dten<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let shift: usize = 4 + (index << 3);
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [4]
   }
#[doc="Deadtime Enable For n"]
   #[inline] pub fn set_dten<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 4 + (index << 3);
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

#[doc="Synchronization Enable For n"]
   #[inline] pub fn syncen<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let shift: usize = 5 + (index << 3);
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [5]
   }
#[doc="Synchronization Enable For n"]
   #[inline] pub fn set_syncen<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 5 + (index << 3);
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

#[doc="Fault Control Enable For n"]
   #[inline] pub fn faulten<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let shift: usize = 6 + (index << 3);
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [6]
   }
#[doc="Fault Control Enable For n"]
   #[inline] pub fn set_faulten<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 6 + (index << 3);
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Combine {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Combine {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.combine(0) != 0 { try!(write!(f, " combine[0]"))}
      if self.combine(1) != 0 { try!(write!(f, " combine[1]"))}
      if self.combine(2) != 0 { try!(write!(f, " combine[2]"))}
      if self.combine(3) != 0 { try!(write!(f, " combine[3]"))}
      if self.comp(0) != 0 { try!(write!(f, " comp[0]"))}
      if self.comp(1) != 0 { try!(write!(f, " comp[1]"))}
      if self.comp(2) != 0 { try!(write!(f, " comp[2]"))}
      if self.comp(3) != 0 { try!(write!(f, " comp[3]"))}
      if self.decapen(0) != 0 { try!(write!(f, " decapen[0]"))}
      if self.decapen(1) != 0 { try!(write!(f, " decapen[1]"))}
      if self.decapen(2) != 0 { try!(write!(f, " decapen[2]"))}
      if self.decapen(3) != 0 { try!(write!(f, " decapen[3]"))}
      if self.decap(0) != 0 { try!(write!(f, " decap[0]"))}
      if self.decap(1) != 0 { try!(write!(f, " decap[1]"))}
      if self.decap(2) != 0 { try!(write!(f, " decap[2]"))}
      if self.decap(3) != 0 { try!(write!(f, " decap[3]"))}
      if self.dten(0) != 0 { try!(write!(f, " dten[0]"))}
      if self.dten(1) != 0 { try!(write!(f, " dten[1]"))}
      if self.dten(2) != 0 { try!(write!(f, " dten[2]"))}
      if self.dten(3) != 0 { try!(write!(f, " dten[3]"))}
      if self.syncen(0) != 0 { try!(write!(f, " syncen[0]"))}
      if self.syncen(1) != 0 { try!(write!(f, " syncen[1]"))}
      if self.syncen(2) != 0 { try!(write!(f, " syncen[2]"))}
      if self.syncen(3) != 0 { try!(write!(f, " syncen[3]"))}
      if self.faulten(0) != 0 { try!(write!(f, " faulten[0]"))}
      if self.faulten(1) != 0 { try!(write!(f, " faulten[1]"))}
      if self.faulten(2) != 0 { try!(write!(f, " faulten[2]"))}
      if self.faulten(3) != 0 { try!(write!(f, " faulten[3]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Deadtime Insertion Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Deadtime(pub u32);
impl Deadtime {
#[doc="Deadtime Value"]
   #[inline] pub fn dtval(&self) -> bits::U6 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
   }
#[doc="Deadtime Value"]
   #[inline] pub fn set_dtval<V: Into<bits::U6>>(mut self, value: V) -> Self {
      let value: bits::U6 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3f << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Deadtime Prescaler Value"]
   #[inline] pub fn dtps(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
   }
#[doc="Deadtime Prescaler Value"]
   #[inline] pub fn set_dtps<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 6);
      self.0 |= value << 6;
      self
   }

}
impl ::core::fmt::Display for Deadtime {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Deadtime {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dtval() != 0 { try!(write!(f, " dtval=0x{:x}", self.dtval()))}
      if self.dtps() != 0 { try!(write!(f, " dtps=0x{:x}", self.dtps()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="FTM External Trigger"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Exttrig(pub u32);
impl Exttrig {
#[doc="Channel 2 Trigger Enable"]
   #[inline] pub fn ch2trig(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Channel 2 Trigger Enable"]
   #[inline] pub fn set_ch2trig<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Channel 3 Trigger Enable"]
   #[inline] pub fn ch3trig(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Channel 3 Trigger Enable"]
   #[inline] pub fn set_ch3trig<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Channel 4 Trigger Enable"]
   #[inline] pub fn ch4trig(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Channel 4 Trigger Enable"]
   #[inline] pub fn set_ch4trig<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Channel 5 Trigger Enable"]
   #[inline] pub fn ch5trig(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Channel 5 Trigger Enable"]
   #[inline] pub fn set_ch5trig<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Channel 0 Trigger Enable"]
   #[inline] pub fn ch0trig(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Channel 0 Trigger Enable"]
   #[inline] pub fn set_ch0trig<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Channel 1 Trigger Enable"]
   #[inline] pub fn ch1trig(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Channel 1 Trigger Enable"]
   #[inline] pub fn set_ch1trig<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Initialization Trigger Enable"]
   #[inline] pub fn inittrigen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Initialization Trigger Enable"]
   #[inline] pub fn set_inittrigen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Channel Trigger Flag"]
   #[inline] pub fn trigf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Channel Trigger Flag"]
   #[inline] pub fn set_trigf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Exttrig {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Exttrig {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ch2trig() != 0 { try!(write!(f, " ch2trig"))}
      if self.ch3trig() != 0 { try!(write!(f, " ch3trig"))}
      if self.ch4trig() != 0 { try!(write!(f, " ch4trig"))}
      if self.ch5trig() != 0 { try!(write!(f, " ch5trig"))}
      if self.ch0trig() != 0 { try!(write!(f, " ch0trig"))}
      if self.ch1trig() != 0 { try!(write!(f, " ch1trig"))}
      if self.inittrigen() != 0 { try!(write!(f, " inittrigen"))}
      if self.trigf() != 0 { try!(write!(f, " trigf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Channels Polarity"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pol(pub u32);
impl Pol {
#[doc="Channel n Polarity"]
   #[inline] pub fn pol<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="Channel n Polarity"]
   #[inline] pub fn set_pol<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
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
impl ::core::fmt::Display for Pol {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pol {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pol(0) != 0 { try!(write!(f, " pol[0]"))}
      if self.pol(1) != 0 { try!(write!(f, " pol[1]"))}
      if self.pol(2) != 0 { try!(write!(f, " pol[2]"))}
      if self.pol(3) != 0 { try!(write!(f, " pol[3]"))}
      if self.pol(4) != 0 { try!(write!(f, " pol[4]"))}
      if self.pol(5) != 0 { try!(write!(f, " pol[5]"))}
      if self.pol(6) != 0 { try!(write!(f, " pol[6]"))}
      if self.pol(7) != 0 { try!(write!(f, " pol[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Fault Mode Status"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Fms(pub u32);
impl Fms {
#[doc="Fault Detection Flag 0"]
   #[inline] pub fn faultf0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Fault Detection Flag 0"]
   #[inline] pub fn set_faultf0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Fault Detection Flag 1"]
   #[inline] pub fn faultf1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Fault Detection Flag 1"]
   #[inline] pub fn set_faultf1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Fault Detection Flag 2"]
   #[inline] pub fn faultf2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Fault Detection Flag 2"]
   #[inline] pub fn set_faultf2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Fault Detection Flag 3"]
   #[inline] pub fn faultf3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Fault Detection Flag 3"]
   #[inline] pub fn set_faultf3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Fault Inputs"]
   #[inline] pub fn faultin(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Fault Inputs"]
   #[inline] pub fn set_faultin<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Write Protection Enable"]
   #[inline] pub fn wpen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Write Protection Enable"]
   #[inline] pub fn set_wpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Fault Detection Flag"]
   #[inline] pub fn faultf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Fault Detection Flag"]
   #[inline] pub fn set_faultf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Fms {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Fms {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.faultf0() != 0 { try!(write!(f, " faultf0"))}
      if self.faultf1() != 0 { try!(write!(f, " faultf1"))}
      if self.faultf2() != 0 { try!(write!(f, " faultf2"))}
      if self.faultf3() != 0 { try!(write!(f, " faultf3"))}
      if self.faultin() != 0 { try!(write!(f, " faultin"))}
      if self.wpen() != 0 { try!(write!(f, " wpen"))}
      if self.faultf() != 0 { try!(write!(f, " faultf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Input Capture Filter Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Filter(pub u32);
impl Filter {
#[doc="Channel n Input Filter"]
   #[inline] pub fn chfval<I: Into<bits::R4>>(&self, index: I) -> bits::U4 {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + (index << 2);
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0xf) as u8) } // [3:0]
   }
#[doc="Channel n Input Filter"]
   #[inline] pub fn set_chfval<I: Into<bits::R4>, V: Into<bits::U4>>(mut self, index: I, value: V) -> Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + (index << 2);
      self.0 &= !(0xf << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Filter {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Filter {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.chfval(0) != 0 { try!(write!(f, " chfval[0]=0x{:x}", self.chfval(0)))}
      if self.chfval(1) != 0 { try!(write!(f, " chfval[1]=0x{:x}", self.chfval(1)))}
      if self.chfval(2) != 0 { try!(write!(f, " chfval[2]=0x{:x}", self.chfval(2)))}
      if self.chfval(3) != 0 { try!(write!(f, " chfval[3]=0x{:x}", self.chfval(3)))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Fault Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Fltctrl(pub u32);
impl Fltctrl {
#[doc="Fault Input n Enable"]
   #[inline] pub fn faulten<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="Fault Input n Enable"]
   #[inline] pub fn set_faulten<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

#[doc="Fault Input 0 Filter Enable"]
   #[inline] pub fn ffltren<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let shift: usize = 4 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [4]
   }
#[doc="Fault Input 0 Filter Enable"]
   #[inline] pub fn set_ffltren<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 4 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

#[doc="Fault Input Filter"]
   #[inline] pub fn ffval(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
   }
#[doc="Fault Input Filter"]
   #[inline] pub fn set_ffval<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 8);
      self.0 |= value << 8;
      self
   }

}
impl ::core::fmt::Display for Fltctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Fltctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.faulten(0) != 0 { try!(write!(f, " faulten[0]"))}
      if self.faulten(1) != 0 { try!(write!(f, " faulten[1]"))}
      if self.faulten(2) != 0 { try!(write!(f, " faulten[2]"))}
      if self.faulten(3) != 0 { try!(write!(f, " faulten[3]"))}
      if self.ffltren(0) != 0 { try!(write!(f, " ffltren[0]"))}
      if self.ffltren(1) != 0 { try!(write!(f, " ffltren[1]"))}
      if self.ffltren(2) != 0 { try!(write!(f, " ffltren[2]"))}
      if self.ffltren(3) != 0 { try!(write!(f, " ffltren[3]"))}
      if self.ffval() != 0 { try!(write!(f, " ffval=0x{:x}", self.ffval()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Quadrature Decoder Control And Status"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Qdctrl(pub u32);
impl Qdctrl {
#[doc="Quadrature Decoder Mode Enable"]
   #[inline] pub fn quaden(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Quadrature Decoder Mode Enable"]
   #[inline] pub fn set_quaden<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Timer Overflow Direction In Quadrature Decoder Mode"]
   #[inline] pub fn tofdir(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Timer Overflow Direction In Quadrature Decoder Mode"]
   #[inline] pub fn set_tofdir<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="FTM Counter Direction In Quadrature Decoder Mode"]
   #[inline] pub fn quadir(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="FTM Counter Direction In Quadrature Decoder Mode"]
   #[inline] pub fn set_quadir<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Quadrature Decoder Mode"]
   #[inline] pub fn quadmode(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Quadrature Decoder Mode"]
   #[inline] pub fn set_quadmode<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Phase B Input Polarity"]
   #[inline] pub fn phbpol(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Phase B Input Polarity"]
   #[inline] pub fn set_phbpol<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Phase A Input Polarity"]
   #[inline] pub fn phapol(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Phase A Input Polarity"]
   #[inline] pub fn set_phapol<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Phase B Input Filter Enable"]
   #[inline] pub fn phbfltren(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Phase B Input Filter Enable"]
   #[inline] pub fn set_phbfltren<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Phase A Input Filter Enable"]
   #[inline] pub fn phafltren(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Phase A Input Filter Enable"]
   #[inline] pub fn set_phafltren<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Qdctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Qdctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.quaden() != 0 { try!(write!(f, " quaden"))}
      if self.tofdir() != 0 { try!(write!(f, " tofdir"))}
      if self.quadir() != 0 { try!(write!(f, " quadir"))}
      if self.quadmode() != 0 { try!(write!(f, " quadmode"))}
      if self.phbpol() != 0 { try!(write!(f, " phbpol"))}
      if self.phapol() != 0 { try!(write!(f, " phapol"))}
      if self.phbfltren() != 0 { try!(write!(f, " phbfltren"))}
      if self.phafltren() != 0 { try!(write!(f, " phafltren"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Configuration"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Conf(pub u32);
impl Conf {
#[doc="TOF Frequency"]
   #[inline] pub fn numtof(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
   }
#[doc="TOF Frequency"]
   #[inline] pub fn set_numtof<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 0);
      self.0 |= value << 0;
      self
   }

#[doc="BDM Mode"]
   #[inline] pub fn bdmmode(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
   }
#[doc="BDM Mode"]
   #[inline] pub fn set_bdmmode<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Global Time Base Enable"]
   #[inline] pub fn gtbeen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="Global Time Base Enable"]
   #[inline] pub fn set_gtbeen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

#[doc="Global Time Base Output"]
   #[inline] pub fn gtbeout(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
   }
#[doc="Global Time Base Output"]
   #[inline] pub fn set_gtbeout<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 10);
      self.0 |= value << 10;
      self
   }

}
impl ::core::fmt::Display for Conf {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Conf {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.numtof() != 0 { try!(write!(f, " numtof=0x{:x}", self.numtof()))}
      if self.bdmmode() != 0 { try!(write!(f, " bdmmode=0x{:x}", self.bdmmode()))}
      if self.gtbeen() != 0 { try!(write!(f, " gtbeen"))}
      if self.gtbeout() != 0 { try!(write!(f, " gtbeout"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="FTM Fault Input Polarity"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Fltpol(pub u32);
impl Fltpol {
#[doc="Fault Input 0 Polarity"]
   #[inline] pub fn fltpol<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="Fault Input 0 Polarity"]
   #[inline] pub fn set_fltpol<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Fltpol {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Fltpol {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.fltpol(0) != 0 { try!(write!(f, " fltpol[0]"))}
      if self.fltpol(1) != 0 { try!(write!(f, " fltpol[1]"))}
      if self.fltpol(2) != 0 { try!(write!(f, " fltpol[2]"))}
      if self.fltpol(3) != 0 { try!(write!(f, " fltpol[3]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Synchronization Configuration"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Synconf(pub u32);
impl Synconf {
#[doc="Hardware Trigger Mode"]
   #[inline] pub fn hwtrigmode(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Hardware Trigger Mode"]
   #[inline] pub fn set_hwtrigmode<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="CNTIN Register Synchronization"]
   #[inline] pub fn cntinc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="CNTIN Register Synchronization"]
   #[inline] pub fn set_cntinc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="INVCTRL Register Synchronization"]
   #[inline] pub fn invc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="INVCTRL Register Synchronization"]
   #[inline] pub fn set_invc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="SWOCTRL Register Synchronization"]
   #[inline] pub fn swoc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="SWOCTRL Register Synchronization"]
   #[inline] pub fn set_swoc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Synchronization Mode"]
   #[inline] pub fn syncmode(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Synchronization Mode"]
   #[inline] pub fn set_syncmode<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="FTM counter synchronization is activated by the software trigger."]
   #[inline] pub fn swrstcnt(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="FTM counter synchronization is activated by the software trigger."]
   #[inline] pub fn set_swrstcnt<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="MOD, CNTIN, and CV registers synchronization is activated by the software trigger."]
   #[inline] pub fn swwrbuf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="MOD, CNTIN, and CV registers synchronization is activated by the software trigger."]
   #[inline] pub fn set_swwrbuf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

#[doc="Output mask synchronization is activated by the software trigger."]
   #[inline] pub fn swom(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
   }
#[doc="Output mask synchronization is activated by the software trigger."]
   #[inline] pub fn set_swom<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 10);
      self.0 |= value << 10;
      self
   }

#[doc="Inverting control synchronization is activated by the software trigger."]
   #[inline] pub fn swinvc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
   }
#[doc="Inverting control synchronization is activated by the software trigger."]
   #[inline] pub fn set_swinvc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 11);
      self.0 |= value << 11;
      self
   }

#[doc="Software output control synchronization is activated by the software trigger."]
   #[inline] pub fn swsoc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
   }
#[doc="Software output control synchronization is activated by the software trigger."]
   #[inline] pub fn set_swsoc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 12);
      self.0 |= value << 12;
      self
   }

#[doc="FTM counter synchronization is activated by a hardware trigger."]
   #[inline] pub fn hwrstcnt(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
   }
#[doc="FTM counter synchronization is activated by a hardware trigger."]
   #[inline] pub fn set_hwrstcnt<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 16);
      self.0 |= value << 16;
      self
   }

#[doc="MOD, CNTIN, and CV registers synchronization is activated by a hardware trigger."]
   #[inline] pub fn hwwrbuf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
   }
#[doc="MOD, CNTIN, and CV registers synchronization is activated by a hardware trigger."]
   #[inline] pub fn set_hwwrbuf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 17);
      self.0 |= value << 17;
      self
   }

#[doc="Output mask synchronization is activated by a hardware trigger."]
   #[inline] pub fn hwom(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
   }
#[doc="Output mask synchronization is activated by a hardware trigger."]
   #[inline] pub fn set_hwom<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 18);
      self.0 |= value << 18;
      self
   }

#[doc="Inverting control synchronization is activated by a hardware trigger."]
   #[inline] pub fn hwinvc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
   }
#[doc="Inverting control synchronization is activated by a hardware trigger."]
   #[inline] pub fn set_hwinvc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 19);
      self.0 |= value << 19;
      self
   }

#[doc="Software output control synchronization is activated by a hardware trigger."]
   #[inline] pub fn hwsoc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
   }
#[doc="Software output control synchronization is activated by a hardware trigger."]
   #[inline] pub fn set_hwsoc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 20);
      self.0 |= value << 20;
      self
   }

}
impl ::core::fmt::Display for Synconf {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Synconf {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.hwtrigmode() != 0 { try!(write!(f, " hwtrigmode"))}
      if self.cntinc() != 0 { try!(write!(f, " cntinc"))}
      if self.invc() != 0 { try!(write!(f, " invc"))}
      if self.swoc() != 0 { try!(write!(f, " swoc"))}
      if self.syncmode() != 0 { try!(write!(f, " syncmode"))}
      if self.swrstcnt() != 0 { try!(write!(f, " swrstcnt"))}
      if self.swwrbuf() != 0 { try!(write!(f, " swwrbuf"))}
      if self.swom() != 0 { try!(write!(f, " swom"))}
      if self.swinvc() != 0 { try!(write!(f, " swinvc"))}
      if self.swsoc() != 0 { try!(write!(f, " swsoc"))}
      if self.hwrstcnt() != 0 { try!(write!(f, " hwrstcnt"))}
      if self.hwwrbuf() != 0 { try!(write!(f, " hwwrbuf"))}
      if self.hwom() != 0 { try!(write!(f, " hwom"))}
      if self.hwinvc() != 0 { try!(write!(f, " hwinvc"))}
      if self.hwsoc() != 0 { try!(write!(f, " hwsoc"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="FTM Inverting Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Invctrl(pub u32);
impl Invctrl {
#[doc="Pair Channels n Inverting Enable"]
   #[inline] pub fn inven<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="Pair Channels n Inverting Enable"]
   #[inline] pub fn set_inven<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Invctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Invctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.inven(0) != 0 { try!(write!(f, " inven[0]"))}
      if self.inven(1) != 0 { try!(write!(f, " inven[1]"))}
      if self.inven(2) != 0 { try!(write!(f, " inven[2]"))}
      if self.inven(3) != 0 { try!(write!(f, " inven[3]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="FTM Software Output Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Swoctrl(pub u32);
impl Swoctrl {
#[doc="Channel 0 Software Output Control Enable"]
   #[inline] pub fn choc<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="Channel 0 Software Output Control Enable"]
   #[inline] pub fn set_choc<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

#[doc="Channel n Software Output Control Value"]
   #[inline] pub fn chocv<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 8 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [8]
   }
#[doc="Channel n Software Output Control Value"]
   #[inline] pub fn set_chocv<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 8 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Swoctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Swoctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.choc(0) != 0 { try!(write!(f, " choc[0]"))}
      if self.choc(1) != 0 { try!(write!(f, " choc[1]"))}
      if self.choc(2) != 0 { try!(write!(f, " choc[2]"))}
      if self.choc(3) != 0 { try!(write!(f, " choc[3]"))}
      if self.choc(4) != 0 { try!(write!(f, " choc[4]"))}
      if self.choc(5) != 0 { try!(write!(f, " choc[5]"))}
      if self.choc(6) != 0 { try!(write!(f, " choc[6]"))}
      if self.choc(7) != 0 { try!(write!(f, " choc[7]"))}
      if self.chocv(0) != 0 { try!(write!(f, " chocv[0]"))}
      if self.chocv(1) != 0 { try!(write!(f, " chocv[1]"))}
      if self.chocv(2) != 0 { try!(write!(f, " chocv[2]"))}
      if self.chocv(3) != 0 { try!(write!(f, " chocv[3]"))}
      if self.chocv(4) != 0 { try!(write!(f, " chocv[4]"))}
      if self.chocv(5) != 0 { try!(write!(f, " chocv[5]"))}
      if self.chocv(6) != 0 { try!(write!(f, " chocv[6]"))}
      if self.chocv(7) != 0 { try!(write!(f, " chocv[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="FTM PWM Load"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pwmload(pub u32);
impl Pwmload {
#[doc="Channel n Select"]
   #[inline] pub fn chsel<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="Channel n Select"]
   #[inline] pub fn set_chsel<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

#[doc="Load Enable"]
   #[inline] pub fn ldok(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="Load Enable"]
   #[inline] pub fn set_ldok<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

}
impl ::core::fmt::Display for Pwmload {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pwmload {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.chsel(0) != 0 { try!(write!(f, " chsel[0]"))}
      if self.chsel(1) != 0 { try!(write!(f, " chsel[1]"))}
      if self.chsel(2) != 0 { try!(write!(f, " chsel[2]"))}
      if self.chsel(3) != 0 { try!(write!(f, " chsel[3]"))}
      if self.chsel(4) != 0 { try!(write!(f, " chsel[4]"))}
      if self.chsel(5) != 0 { try!(write!(f, " chsel[5]"))}
      if self.chsel(6) != 0 { try!(write!(f, " chsel[6]"))}
      if self.chsel(7) != 0 { try!(write!(f, " chsel[7]"))}
      if self.ldok() != 0 { try!(write!(f, " ldok"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
pub struct FtmCh { pub periph: FtmPeriph, pub index: usize }
