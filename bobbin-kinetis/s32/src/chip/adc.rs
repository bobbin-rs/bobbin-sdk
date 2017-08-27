#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "ADC", peripherals: [Peripheral { derived_from: None, group_name: None, name: "ADC0", address: 1073983488, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Analog-to-Digital Converter"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "ADC0", types: ["ADC"], value: 39, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [Channel { name: "ADC0_CH0", index: Some(0), description: None, signals: [Signal { name: "ADC0_SE0", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC0_CH1", index: Some(1), description: None, signals: [Signal { name: "ADC0_SE1", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC0_CH2", index: Some(2), description: None, signals: [Signal { name: "ADC0_SE2", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC0_CH3", index: Some(3), description: None, signals: [Signal { name: "ADC0_SE3", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC0_CH4", index: Some(4), description: None, signals: [Signal { name: "ADC0_SE4", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC0_CH5", index: Some(5), description: None, signals: [Signal { name: "ADC0_SE5", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC0_CH6", index: Some(6), description: None, signals: [Signal { name: "ADC0_SE6", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC0_CH7", index: Some(7), description: None, signals: [Signal { name: "ADC0_SE7", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC0_CH8", index: Some(8), description: None, signals: [Signal { name: "ADC0_SE8", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC0_CH9", index: Some(9), description: None, signals: [Signal { name: "ADC0_SE9", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC0_CH10", index: Some(10), description: None, signals: [Signal { name: "ADC0_SE10", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC0_CH11", index: Some(11), description: None, signals: [Signal { name: "ADC0_SE11", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC0_CH12", index: Some(12), description: None, signals: [Signal { name: "ADC0_SE12", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC0_CH13", index: Some(13), description: None, signals: [Signal { name: "ADC0_SE13", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC0_CH14", index: Some(14), description: None, signals: [Signal { name: "ADC0_SE14", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC0_CH15", index: Some(15), description: None, signals: [Signal { name: "ADC0_SE15", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC0_IN0", index: Some(21), description: None, signals: [], interrupts: [] }, Channel { name: "ADC0_BANDGAP", index: Some(27), description: None, signals: [], interrupts: [] }, Channel { name: "ADC0_REFSH", index: Some(29), description: None, signals: [], interrupts: [] }, Channel { name: "ADC0_REFSL", index: Some(30), description: None, signals: [], interrupts: [] }], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "ADC1", address: 1073901568, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Analog-to-Digital Converter"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "ADC1", types: ["ADC"], value: 40, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [Channel { name: "ADC1_CH0", index: Some(0), description: None, signals: [Signal { name: "ADC1_SE0", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC1_CH1", index: Some(1), description: None, signals: [Signal { name: "ADC1_SE1", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC1_CH2", index: Some(2), description: None, signals: [Signal { name: "ADC1_SE2", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC1_CH3", index: Some(3), description: None, signals: [Signal { name: "ADC1_SE3", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC1_CH4", index: Some(4), description: None, signals: [Signal { name: "ADC1_SE4", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC1_CH5", index: Some(5), description: None, signals: [Signal { name: "ADC1_SE5", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC1_CH6", index: Some(6), description: None, signals: [Signal { name: "ADC1_SE6", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC1_CH7", index: Some(7), description: None, signals: [Signal { name: "ADC1_SE7", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC1_CH8", index: Some(8), description: None, signals: [Signal { name: "ADC1_SE8", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC1_CH9", index: Some(9), description: None, signals: [Signal { name: "ADC1_SE9", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC1_CH10", index: Some(10), description: None, signals: [Signal { name: "ADC1_SE10", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC1_CH11", index: Some(11), description: None, signals: [Signal { name: "ADC1_SE11", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC1_CH12", index: Some(12), description: None, signals: [Signal { name: "ADC1_SE12", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC1_CH13", index: Some(13), description: None, signals: [Signal { name: "ADC1_SE13", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC1_CH14", index: Some(14), description: None, signals: [Signal { name: "ADC1_SE14", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC1_CH15", index: Some(15), description: None, signals: [Signal { name: "ADC1_SE15", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC1_IN0", index: Some(21), description: None, signals: [], interrupts: [] }, Channel { name: "ADC1_BANDGAP", index: Some(27), description: None, signals: [], interrupts: [] }, Channel { name: "ADC1_REFSH", index: Some(29), description: None, signals: [], interrupts: [] }, Channel { name: "ADC1_REFSL", index: Some(30), description: None, signals: [], interrupts: [] }], dim: None, dim_increment: None, dim_index: None }], prototype: Some(Peripheral { derived_from: None, group_name: Some("ADC"), name: "", address: 0, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [], clusters: [], registers: [Register { name: "SC1", offset: 0, size: Some(32), access: Some(ReadWrite), reset_value: Some(31), reset_mask: Some(4294967295), description: Some("ADC Status and Control Register 1"), fields: [Field { name: "ADCH", bit_offset: 0, bit_width: 5, access: Some(ReadWrite), description: Some("Input channel select"), enumerated_values: [EnumeratedValue { value: "#00000", name: Some("00000"), description: Some("Exernal channel 0 is selected as input.") }, EnumeratedValue { value: "#00001", name: Some("00001"), description: Some("Exernal channel 1 is selected as input.") }, EnumeratedValue { value: "#00010", name: Some("00010"), description: Some("Exernal channel 2 is selected as input.") }, EnumeratedValue { value: "#00011", name: Some("00011"), description: Some("Exernal channel 3 is selected as input.") }, EnumeratedValue { value: "#00100", name: Some("00100"), description: Some("Exernal channel 4 is selected as input.") }, EnumeratedValue { value: "#00101", name: Some("00101"), description: Some("Exernal channel 5 is selected as input.") }, EnumeratedValue { value: "#00110", name: Some("00110"), description: Some("Exernal channel 6 is selected as input.") }, EnumeratedValue { value: "#00111", name: Some("00111"), description: Some("Exernal channel 7 is selected as input.") }, EnumeratedValue { value: "#01000", name: Some("01000"), description: Some("Exernal channel 8 is selected as input.") }, EnumeratedValue { value: "#01001", name: Some("01001"), description: Some("Exernal channel 9 is selected as input.") }, EnumeratedValue { value: "#01010", name: Some("01010"), description: Some("Exernal channel 10 is selected as input.") }, EnumeratedValue { value: "#01011", name: Some("01011"), description: Some("Exernal channel 11 is selected as input.") }, EnumeratedValue { value: "#01100", name: Some("01100"), description: Some("Exernal channel 12 is selected as input.") }, EnumeratedValue { value: "#01101", name: Some("01101"), description: Some("Exernal channel 13 is selected as input.") }, EnumeratedValue { value: "#01110", name: Some("01110"), description: Some("Exernal channel 14 is selected as input.") }, EnumeratedValue { value: "#01111", name: Some("01111"), description: Some("Exernal channel 15 is selected as input.") }, EnumeratedValue { value: "#10010", name: Some("10010"), description: Some("Exernal channel 18 is selected as input.") }, EnumeratedValue { value: "#10011", name: Some("10011"), description: Some("Exernal channel 19 is selected as input.") }, EnumeratedValue { value: "#10101", name: Some("10101"), description: Some("Internal channel 0 is selected as input.") }, EnumeratedValue { value: "#10110", name: Some("10110"), description: Some("Internal channel 1 is selected as input.") }, EnumeratedValue { value: "#10111", name: Some("10111"), description: Some("Internal channel 2 is selected as input.") }, EnumeratedValue { value: "#11010", name: Some("11010"), description: Some("Temp Sensor") }, EnumeratedValue { value: "#11011", name: Some("11011"), description: Some("Band Gap") }, EnumeratedValue { value: "#11100", name: Some("11100"), description: Some("Internal channel 3 is selected as input.") }, EnumeratedValue { value: "#11101", name: Some("11101"), description: Some("VREFSH is selected as input. Voltage reference selected is determined by SC2[REFSEL].") }, EnumeratedValue { value: "#11110", name: Some("11110"), description: Some("VREFSL is selected as input. Voltage reference selected is determined by SC2[REFSEL].") }, EnumeratedValue { value: "#11111", name: Some("11111"), description: Some("Module is disabled") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "AIEN", bit_offset: 6, bit_width: 1, access: Some(ReadWrite), description: Some("Interrupt Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Conversion complete interrupt is disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Conversion complete interrupt is enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "COCO", bit_offset: 7, bit_width: 1, access: Some(ReadOnly), description: Some("Conversion Complete Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Conversion is not completed.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Conversion is completed.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: Some(16), dim_increment: Some(4), dim_index: Some("A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P") }, Register { name: "CFG1", offset: 64, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("ADC Configuration Register 1"), fields: [Field { name: "ADICLK", bit_offset: 0, bit_width: 2, access: Some(ReadWrite), description: Some("Input Clock Select"), enumerated_values: [EnumeratedValue { value: "#00", name: Some("00"), description: Some("Alternate clock 1 (ADC_ALTCLK1)") }, EnumeratedValue { value: "#01", name: Some("01"), description: Some("Alternate clock 2 (ADC_ALTCLK2)") }, EnumeratedValue { value: "#10", name: Some("10"), description: Some("Alternate clock 3 (ADC_ALTCLK3)") }, EnumeratedValue { value: "#11", name: Some("11"), description: Some("Alternate clock 4 (ADC_ALTCLK4)") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MODE", bit_offset: 2, bit_width: 2, access: Some(ReadWrite), description: Some("Conversion mode selection"), enumerated_values: [EnumeratedValue { value: "#00", name: Some("00"), description: Some("8-bit conversion.") }, EnumeratedValue { value: "#01", name: Some("01"), description: Some("12-bit conversion.") }, EnumeratedValue { value: "#10", name: Some("10"), description: Some("10-bit conversion.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ADIV", bit_offset: 5, bit_width: 2, access: Some(ReadWrite), description: Some("Clock Divide Select"), enumerated_values: [EnumeratedValue { value: "#00", name: Some("00"), description: Some("The divide ratio is 1 and the clock rate is input clock.") }, EnumeratedValue { value: "#01", name: Some("01"), description: Some("The divide ratio is 2 and the clock rate is (input clock)/2.") }, EnumeratedValue { value: "#10", name: Some("10"), description: Some("The divide ratio is 4 and the clock rate is (input clock)/4.") }, EnumeratedValue { value: "#11", name: Some("11"), description: Some("The divide ratio is 8 and the clock rate is (input clock)/8.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CLRLTRG", bit_offset: 8, bit_width: 1, access: Some(WriteOnly), description: Some("Clear Latch Trigger in Trigger Handler Block"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "CFG2", offset: 68, size: Some(32), access: Some(ReadWrite), reset_value: Some(12), reset_mask: Some(4294967295), description: Some("ADC Configuration Register 2"), fields: [Field { name: "SMPLTS", bit_offset: 0, bit_width: 8, access: Some(ReadWrite), description: Some("Sample Time Select"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "R", offset: 72, size: Some(32), access: Some(ReadOnly), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("ADC Data Result Registers"), fields: [Field { name: "D", bit_offset: 0, bit_width: 12, access: Some(ReadOnly), description: Some("Data result"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "D12", bit_offset: 0, bit_width: 12, access: Some(ReadOnly), description: Some("Data result (12 bit)"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "D10", bit_offset: 0, bit_width: 10, access: Some(ReadOnly), description: Some("Data result (10 bit)"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "D8", bit_offset: 0, bit_width: 8, access: Some(ReadOnly), description: Some("Data result (8 bit)"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: Some(16), dim_increment: Some(4), dim_index: Some("A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P") }, Register { name: "CV", offset: 136, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Compare Value Registers"), fields: [Field { name: "CV", bit_offset: 0, bit_width: 16, access: Some(ReadWrite), description: Some("Compare Value."), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: Some(2), dim_increment: Some(4), dim_index: Some("1,2") }, Register { name: "SC2", offset: 144, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Status and Control Register 2"), fields: [Field { name: "REFSEL", bit_offset: 0, bit_width: 2, access: Some(ReadWrite), description: Some("Voltage Reference Selection"), enumerated_values: [EnumeratedValue { value: "#00", name: Some("00"), description: Some("Default voltage reference pin pair, that is, external pins VREFH and VREFL") }, EnumeratedValue { value: "#01", name: Some("01"), description: Some("Alternate reference voltage, that is, VALTH. This voltage may be additional external pin or internal source depending on the MCU configuration. See the chip configuration information for details specific to this MCU.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "DMAEN", bit_offset: 2, bit_width: 1, access: Some(ReadWrite), description: Some("DMA Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("DMA is disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("DMA is enabled and will assert the ADC DMA request during an ADC conversion complete event , which is indicated when any SC1n[COCO] flag is asserted.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ACREN", bit_offset: 3, bit_width: 1, access: Some(ReadWrite), description: Some("Compare Function Range Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ACFGT", bit_offset: 4, bit_width: 1, access: Some(ReadWrite), description: Some("Compare Function Greater Than Enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ACFE", bit_offset: 5, bit_width: 1, access: Some(ReadWrite), description: Some("Compare Function Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Compare function disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Compare function enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ADTRG", bit_offset: 6, bit_width: 1, access: Some(ReadWrite), description: Some("Conversion Trigger Select"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Software trigger selected.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Hardware trigger selected.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ADACT", bit_offset: 7, bit_width: 1, access: Some(ReadOnly), description: Some("Conversion Active"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Conversion not in progress.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Conversion in progress.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TRGPRNUM", bit_offset: 13, bit_width: 2, access: Some(ReadOnly), description: Some("Trigger Process Number"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TRGSTLAT", bit_offset: 16, bit_width: 4, access: Some(ReadOnly), description: Some("Trigger Status"), enumerated_values: [EnumeratedValue { value: "#0000", name: Some("0"), description: Some("No trigger request has been latched") }, EnumeratedValue { value: "#0001", name: Some("1"), description: Some("A trigger request has been latched") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TRGSTERR", bit_offset: 24, bit_width: 4, access: Some(ReadOnly), description: Some("Error in Multiplexed Trigger Request"), enumerated_values: [EnumeratedValue { value: "#0000", name: Some("0"), description: Some("No error has occurred") }, EnumeratedValue { value: "#0001", name: Some("1"), description: Some("An error has occurred") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "SC3", offset: 148, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Status and Control Register 3"), fields: [Field { name: "AVGS", bit_offset: 0, bit_width: 2, access: Some(ReadWrite), description: Some("Hardware Average Select"), enumerated_values: [EnumeratedValue { value: "#00", name: Some("00"), description: Some("4 samples averaged.") }, EnumeratedValue { value: "#01", name: Some("01"), description: Some("8 samples averaged.") }, EnumeratedValue { value: "#10", name: Some("10"), description: Some("16 samples averaged.") }, EnumeratedValue { value: "#11", name: Some("11"), description: Some("32 samples averaged.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "AVGE", bit_offset: 2, bit_width: 1, access: Some(ReadWrite), description: Some("Hardware Average Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Hardware average function disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Hardware average function enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ADCO", bit_offset: 3, bit_width: 1, access: Some(ReadWrite), description: Some("Continuous Conversion Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("One conversion will be performed (or one set of conversions, if AVGE is set) after a conversion is initiated.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Continuous conversions will be performed (or continuous sets of conversions, if AVGE is set) after a conversion is initiated.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CAL", bit_offset: 7, bit_width: 1, access: Some(ReadWrite), description: Some("Calibration"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "BASE_OFS", offset: 152, size: Some(32), access: Some(ReadWrite), reset_value: Some(64), reset_mask: Some(4294967295), description: Some("BASE Offset Register"), fields: [Field { name: "BA_OFS", bit_offset: 0, bit_width: 8, access: Some(ReadWrite), description: Some("Base Offset Error Correction Value"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "OFS", offset: 156, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("ADC Offset Correction Register"), fields: [Field { name: "OFS", bit_offset: 0, bit_width: 16, access: Some(ReadWrite), description: Some("Offset Error Correction Value"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "USR_OFS", offset: 160, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("USER Offset Correction Register"), fields: [Field { name: "USR_OFS", bit_offset: 0, bit_width: 8, access: Some(ReadWrite), description: Some("USER Offset Error Correction Value"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "XOFS", offset: 164, size: Some(32), access: Some(ReadWrite), reset_value: Some(48), reset_mask: Some(4294967295), description: Some("ADC X Offset Correction Register"), fields: [Field { name: "XOFS", bit_offset: 0, bit_width: 6, access: Some(ReadWrite), description: Some("X offset error correction value"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "YOFS", offset: 168, size: Some(32), access: Some(ReadWrite), reset_value: Some(55), reset_mask: Some(4294967295), description: Some("ADC Y Offset Correction Register"), fields: [Field { name: "YOFS", bit_offset: 0, bit_width: 8, access: Some(ReadWrite), description: Some("Y offset error correction value"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "G", offset: 172, size: Some(32), access: Some(ReadWrite), reset_value: Some(752), reset_mask: Some(4294967295), description: Some("ADC Gain Register"), fields: [Field { name: "G", bit_offset: 0, bit_width: 11, access: Some(ReadWrite), description: Some("Gain error adjustment factor for the overall conversion"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "UG", offset: 176, size: Some(32), access: Some(ReadWrite), reset_value: Some(4), reset_mask: Some(4294967295), description: Some("ADC User Gain Register"), fields: [Field { name: "UG", bit_offset: 0, bit_width: 10, access: Some(ReadWrite), description: Some("User gain error correction value"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "CLPS", offset: 180, size: Some(32), access: Some(ReadWrite), reset_value: Some(46), reset_mask: Some(4294967295), description: Some("ADC General Calibration Value Register S"), fields: [Field { name: "CLPS", bit_offset: 0, bit_width: 7, access: Some(ReadWrite), description: Some("Calibration Value"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "CLP3", offset: 184, size: Some(32), access: Some(ReadWrite), reset_value: Some(384), reset_mask: Some(4294967295), description: Some("ADC Plus-Side General Calibration Value Register 3"), fields: [Field { name: "CLP3", bit_offset: 0, bit_width: 10, access: Some(ReadWrite), description: Some("Calibration Value"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "CLP2", offset: 188, size: Some(32), access: Some(ReadWrite), reset_value: Some(184), reset_mask: Some(4294967295), description: Some("ADC Plus-Side General Calibration Value Register 2"), fields: [Field { name: "CLP2", bit_offset: 0, bit_width: 10, access: Some(ReadWrite), description: Some("Calibration Value"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "CLP1", offset: 192, size: Some(32), access: Some(ReadWrite), reset_value: Some(92), reset_mask: Some(4294967295), description: Some("ADC Plus-Side General Calibration Value Register 1"), fields: [Field { name: "CLP1", bit_offset: 0, bit_width: 9, access: Some(ReadWrite), description: Some("Calibration Value"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "CLP0", offset: 196, size: Some(32), access: Some(ReadWrite), reset_value: Some(46), reset_mask: Some(4294967295), description: Some("ADC Plus-Side General Calibration Value Register 0"), fields: [Field { name: "CLP0", bit_offset: 0, bit_width: 8, access: Some(ReadWrite), description: Some("Calibration Value"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "CLPX", offset: 200, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("ADC Plus-Side General Calibration Value Register X"), fields: [Field { name: "CLPX", bit_offset: 0, bit_width: 7, access: Some(ReadWrite), description: Some("Calibration Value"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "CLP9", offset: 204, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("ADC Plus-Side General Calibration Value Register 9"), fields: [Field { name: "CLP9", bit_offset: 0, bit_width: 7, access: Some(ReadWrite), description: Some("Calibration Value"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "CLPS_OFS", offset: 208, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("ADC General Calibration Offset Value Register S"), fields: [Field { name: "CLPS_OFS", bit_offset: 0, bit_width: 4, access: Some(ReadWrite), description: Some("CLPS Offset"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "CLP3_OFS", offset: 212, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("ADC Plus-Side General Calibration Offset Value Register 3"), fields: [Field { name: "CLP3_OFS", bit_offset: 0, bit_width: 4, access: Some(ReadWrite), description: Some("CLP3 Offset"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "CLP2_OFS", offset: 216, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("ADC Plus-Side General Calibration Offset Value Register 2"), fields: [Field { name: "CLP2_OFS", bit_offset: 0, bit_width: 4, access: Some(ReadWrite), description: Some("CLP2 Offset"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "CLP1_OFS", offset: 220, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("ADC Plus-Side General Calibration Offset Value Register 1"), fields: [Field { name: "CLP1_OFS", bit_offset: 0, bit_width: 4, access: Some(ReadWrite), description: Some("CLP1 Offset"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "CLP0_OFS", offset: 224, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("ADC Plus-Side General Calibration Offset Value Register 0"), fields: [Field { name: "CLP0_OFS", bit_offset: 0, bit_width: 4, access: Some(ReadWrite), description: Some("CLP0 Offset"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "CLPX_OFS", offset: 228, size: Some(32), access: Some(ReadWrite), reset_value: Some(1088), reset_mask: Some(4294967295), description: Some("ADC Plus-Side General Calibration Offset Value Register X"), fields: [Field { name: "CLPX_OFS", bit_offset: 0, bit_width: 12, access: Some(ReadWrite), description: Some("CLPX Offset"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "CLP9_OFS", offset: 232, size: Some(32), access: Some(ReadWrite), reset_value: Some(576), reset_mask: Some(4294967295), description: Some("ADC Plus-Side General Calibration Offset Value Register 9"), fields: [Field { name: "CLP9_OFS", bit_offset: 0, bit_width: 12, access: Some(ReadWrite), description: Some("CLP9 Offset"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }), modules: [], has_pins: false, has_channels: true, description: None }
periph!( ADC0, Adc0, _ADC0, AdcPeriph, 0x4003b000);
periph!( ADC1, Adc1, _ADC1, AdcPeriph, 0x40027000);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="ADC Peripheral"]
pub struct AdcPeriph(pub usize); 

impl super::sig::Signal<super::sig::Adc0Se0> for Adc0Ch0 {}
impl super::sig::SignalAdc<super::sig::Adc0Se0> for Adc0Ch0 {}
impl super::sig::Signal<super::sig::Adc0Se1> for Adc0Ch1 {}
impl super::sig::SignalAdc<super::sig::Adc0Se1> for Adc0Ch1 {}
impl super::sig::Signal<super::sig::Adc0Se2> for Adc0Ch2 {}
impl super::sig::SignalAdc<super::sig::Adc0Se2> for Adc0Ch2 {}
impl super::sig::Signal<super::sig::Adc0Se3> for Adc0Ch3 {}
impl super::sig::SignalAdc<super::sig::Adc0Se3> for Adc0Ch3 {}
impl super::sig::Signal<super::sig::Adc0Se4> for Adc0Ch4 {}
impl super::sig::SignalAdc<super::sig::Adc0Se4> for Adc0Ch4 {}
impl super::sig::Signal<super::sig::Adc0Se5> for Adc0Ch5 {}
impl super::sig::SignalAdc<super::sig::Adc0Se5> for Adc0Ch5 {}
impl super::sig::Signal<super::sig::Adc0Se6> for Adc0Ch6 {}
impl super::sig::SignalAdc<super::sig::Adc0Se6> for Adc0Ch6 {}
impl super::sig::Signal<super::sig::Adc0Se7> for Adc0Ch7 {}
impl super::sig::SignalAdc<super::sig::Adc0Se7> for Adc0Ch7 {}
impl super::sig::Signal<super::sig::Adc0Se8> for Adc0Ch8 {}
impl super::sig::SignalAdc<super::sig::Adc0Se8> for Adc0Ch8 {}
impl super::sig::Signal<super::sig::Adc0Se9> for Adc0Ch9 {}
impl super::sig::SignalAdc<super::sig::Adc0Se9> for Adc0Ch9 {}
impl super::sig::Signal<super::sig::Adc0Se10> for Adc0Ch10 {}
impl super::sig::SignalAdc<super::sig::Adc0Se10> for Adc0Ch10 {}
impl super::sig::Signal<super::sig::Adc0Se11> for Adc0Ch11 {}
impl super::sig::SignalAdc<super::sig::Adc0Se11> for Adc0Ch11 {}
impl super::sig::Signal<super::sig::Adc0Se12> for Adc0Ch12 {}
impl super::sig::SignalAdc<super::sig::Adc0Se12> for Adc0Ch12 {}
impl super::sig::Signal<super::sig::Adc0Se13> for Adc0Ch13 {}
impl super::sig::SignalAdc<super::sig::Adc0Se13> for Adc0Ch13 {}
impl super::sig::Signal<super::sig::Adc0Se14> for Adc0Ch14 {}
impl super::sig::SignalAdc<super::sig::Adc0Se14> for Adc0Ch14 {}
impl super::sig::Signal<super::sig::Adc0Se15> for Adc0Ch15 {}
impl super::sig::SignalAdc<super::sig::Adc0Se15> for Adc0Ch15 {}

impl super::sig::Signal<super::sig::Adc1Se0> for Adc1Ch0 {}
impl super::sig::SignalAdc<super::sig::Adc1Se0> for Adc1Ch0 {}
impl super::sig::Signal<super::sig::Adc1Se1> for Adc1Ch1 {}
impl super::sig::SignalAdc<super::sig::Adc1Se1> for Adc1Ch1 {}
impl super::sig::Signal<super::sig::Adc1Se2> for Adc1Ch2 {}
impl super::sig::SignalAdc<super::sig::Adc1Se2> for Adc1Ch2 {}
impl super::sig::Signal<super::sig::Adc1Se3> for Adc1Ch3 {}
impl super::sig::SignalAdc<super::sig::Adc1Se3> for Adc1Ch3 {}
impl super::sig::Signal<super::sig::Adc1Se4> for Adc1Ch4 {}
impl super::sig::SignalAdc<super::sig::Adc1Se4> for Adc1Ch4 {}
impl super::sig::Signal<super::sig::Adc1Se5> for Adc1Ch5 {}
impl super::sig::SignalAdc<super::sig::Adc1Se5> for Adc1Ch5 {}
impl super::sig::Signal<super::sig::Adc1Se6> for Adc1Ch6 {}
impl super::sig::SignalAdc<super::sig::Adc1Se6> for Adc1Ch6 {}
impl super::sig::Signal<super::sig::Adc1Se7> for Adc1Ch7 {}
impl super::sig::SignalAdc<super::sig::Adc1Se7> for Adc1Ch7 {}
impl super::sig::Signal<super::sig::Adc1Se8> for Adc1Ch8 {}
impl super::sig::SignalAdc<super::sig::Adc1Se8> for Adc1Ch8 {}
impl super::sig::Signal<super::sig::Adc1Se9> for Adc1Ch9 {}
impl super::sig::SignalAdc<super::sig::Adc1Se9> for Adc1Ch9 {}
impl super::sig::Signal<super::sig::Adc1Se10> for Adc1Ch10 {}
impl super::sig::SignalAdc<super::sig::Adc1Se10> for Adc1Ch10 {}
impl super::sig::Signal<super::sig::Adc1Se11> for Adc1Ch11 {}
impl super::sig::SignalAdc<super::sig::Adc1Se11> for Adc1Ch11 {}
impl super::sig::Signal<super::sig::Adc1Se12> for Adc1Ch12 {}
impl super::sig::SignalAdc<super::sig::Adc1Se12> for Adc1Ch12 {}
impl super::sig::Signal<super::sig::Adc1Se13> for Adc1Ch13 {}
impl super::sig::SignalAdc<super::sig::Adc1Se13> for Adc1Ch13 {}
impl super::sig::Signal<super::sig::Adc1Se14> for Adc1Ch14 {}
impl super::sig::SignalAdc<super::sig::Adc1Se14> for Adc1Ch14 {}
impl super::sig::Signal<super::sig::Adc1Se15> for Adc1Ch15 {}
impl super::sig::SignalAdc<super::sig::Adc1Se15> for Adc1Ch15 {}


impl AdcPeriph {
#[doc="Get the *const pointer for the SC1 register."]
   #[inline] pub fn sc1_ptr<I: Into<bits::R16>>(&self, index: I) -> *const u32 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x0 + (index << 2)) as *const u32
   }
#[doc="Get the *mut pointer for the SC1 register."]
   #[inline] pub fn sc1_mut<I: Into<bits::R16>>(&self, index: I) -> *mut u32 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x0 + (index << 2)) as *mut u32
   }
#[doc="Read the SC1 register."]
   #[inline] pub fn sc1<I: Into<bits::R16>>(&self, index: I) -> Sc1 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         Sc1(::core::ptr::read_volatile((self.0 + 0x0 + (index << 2)) as *const u32))
      }
   }
#[doc="Write the SC1 register."]
   #[inline] pub fn set_sc1<I: Into<bits::R16>, F: FnOnce(Sc1) -> Sc1>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let value = f(Sc1(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x0 + (index << 2)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SC1 register."]
   #[inline] pub fn with_sc1<I: Into<bits::R16> + Copy, F: FnOnce(Sc1) -> Sc1>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.sc1(index);
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x0 + (index << 2)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CFG1 register."]
   #[inline] pub fn cfg1_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x40) as *const u32
   }
#[doc="Get the *mut pointer for the CFG1 register."]
   #[inline] pub fn cfg1_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x40) as *mut u32
   }
#[doc="Read the CFG1 register."]
   #[inline] pub fn cfg1(&self) -> Cfg1 { 
      unsafe {
         Cfg1(::core::ptr::read_volatile((self.0 + 0x40) as *const u32))
      }
   }
#[doc="Write the CFG1 register."]
   #[inline] pub fn set_cfg1<F: FnOnce(Cfg1) -> Cfg1>(&self, f: F) -> &Self {
      let value = f(Cfg1(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x40) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CFG1 register."]
   #[inline] pub fn with_cfg1<F: FnOnce(Cfg1) -> Cfg1>(&self, f: F) -> &Self {
      let tmp = self.cfg1();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x40) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CFG2 register."]
   #[inline] pub fn cfg2_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x44) as *const u32
   }
#[doc="Get the *mut pointer for the CFG2 register."]
   #[inline] pub fn cfg2_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x44) as *mut u32
   }
#[doc="Read the CFG2 register."]
   #[inline] pub fn cfg2(&self) -> Cfg2 { 
      unsafe {
         Cfg2(::core::ptr::read_volatile((self.0 + 0x44) as *const u32))
      }
   }
#[doc="Write the CFG2 register."]
   #[inline] pub fn set_cfg2<F: FnOnce(Cfg2) -> Cfg2>(&self, f: F) -> &Self {
      let value = f(Cfg2(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x44) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CFG2 register."]
   #[inline] pub fn with_cfg2<F: FnOnce(Cfg2) -> Cfg2>(&self, f: F) -> &Self {
      let tmp = self.cfg2();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x44) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the R register."]
   #[inline] pub fn r_ptr<I: Into<bits::R16>>(&self, index: I) -> *const u32 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x48 + (index << 2)) as *const u32
   }
#[doc="Get the *mut pointer for the R register."]
   #[inline] pub fn r_mut<I: Into<bits::R16>>(&self, index: I) -> *mut u32 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x48 + (index << 2)) as *mut u32
   }
#[doc="Read the R register."]
   #[inline] pub fn r<I: Into<bits::R16>>(&self, index: I) -> R { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         R(::core::ptr::read_volatile((self.0 + 0x48 + (index << 2)) as *const u32))
      }
   }

#[doc="Get the *const pointer for the CV register."]
   #[inline] pub fn cv_ptr<I: Into<bits::R2>>(&self, index: I) -> *const u32 { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x88 + (index << 2)) as *const u32
   }
#[doc="Get the *mut pointer for the CV register."]
   #[inline] pub fn cv_mut<I: Into<bits::R2>>(&self, index: I) -> *mut u32 { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x88 + (index << 2)) as *mut u32
   }
#[doc="Read the CV register."]
   #[inline] pub fn cv<I: Into<bits::R2>>(&self, index: I) -> Cv { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         Cv(::core::ptr::read_volatile((self.0 + 0x88 + (index << 2)) as *const u32))
      }
   }
#[doc="Write the CV register."]
   #[inline] pub fn set_cv<I: Into<bits::R2>, F: FnOnce(Cv) -> Cv>(&self, index: I, f: F) -> &Self {
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      let value = f(Cv(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x88 + (index << 2)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CV register."]
   #[inline] pub fn with_cv<I: Into<bits::R2> + Copy, F: FnOnce(Cv) -> Cv>(&self, index: I, f: F) -> &Self {
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.cv(index);
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x88 + (index << 2)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SC2 register."]
   #[inline] pub fn sc2_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x90) as *const u32
   }
#[doc="Get the *mut pointer for the SC2 register."]
   #[inline] pub fn sc2_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x90) as *mut u32
   }
#[doc="Read the SC2 register."]
   #[inline] pub fn sc2(&self) -> Sc2 { 
      unsafe {
         Sc2(::core::ptr::read_volatile((self.0 + 0x90) as *const u32))
      }
   }
#[doc="Write the SC2 register."]
   #[inline] pub fn set_sc2<F: FnOnce(Sc2) -> Sc2>(&self, f: F) -> &Self {
      let value = f(Sc2(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x90) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SC2 register."]
   #[inline] pub fn with_sc2<F: FnOnce(Sc2) -> Sc2>(&self, f: F) -> &Self {
      let tmp = self.sc2();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x90) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SC3 register."]
   #[inline] pub fn sc3_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x94) as *const u32
   }
#[doc="Get the *mut pointer for the SC3 register."]
   #[inline] pub fn sc3_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x94) as *mut u32
   }
#[doc="Read the SC3 register."]
   #[inline] pub fn sc3(&self) -> Sc3 { 
      unsafe {
         Sc3(::core::ptr::read_volatile((self.0 + 0x94) as *const u32))
      }
   }
#[doc="Write the SC3 register."]
   #[inline] pub fn set_sc3<F: FnOnce(Sc3) -> Sc3>(&self, f: F) -> &Self {
      let value = f(Sc3(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x94) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SC3 register."]
   #[inline] pub fn with_sc3<F: FnOnce(Sc3) -> Sc3>(&self, f: F) -> &Self {
      let tmp = self.sc3();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x94) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the BASE_OFS register."]
   #[inline] pub fn base_ofs_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x98) as *const u32
   }
#[doc="Get the *mut pointer for the BASE_OFS register."]
   #[inline] pub fn base_ofs_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x98) as *mut u32
   }
#[doc="Read the BASE_OFS register."]
   #[inline] pub fn base_ofs(&self) -> BaseOfs { 
      unsafe {
         BaseOfs(::core::ptr::read_volatile((self.0 + 0x98) as *const u32))
      }
   }
#[doc="Write the BASE_OFS register."]
   #[inline] pub fn set_base_ofs<F: FnOnce(BaseOfs) -> BaseOfs>(&self, f: F) -> &Self {
      let value = f(BaseOfs(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x98) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the BASE_OFS register."]
   #[inline] pub fn with_base_ofs<F: FnOnce(BaseOfs) -> BaseOfs>(&self, f: F) -> &Self {
      let tmp = self.base_ofs();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x98) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the OFS register."]
   #[inline] pub fn ofs_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x9c) as *const u32
   }
#[doc="Get the *mut pointer for the OFS register."]
   #[inline] pub fn ofs_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x9c) as *mut u32
   }
#[doc="Read the OFS register."]
   #[inline] pub fn ofs(&self) -> Ofs { 
      unsafe {
         Ofs(::core::ptr::read_volatile((self.0 + 0x9c) as *const u32))
      }
   }
#[doc="Write the OFS register."]
   #[inline] pub fn set_ofs<F: FnOnce(Ofs) -> Ofs>(&self, f: F) -> &Self {
      let value = f(Ofs(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x9c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the OFS register."]
   #[inline] pub fn with_ofs<F: FnOnce(Ofs) -> Ofs>(&self, f: F) -> &Self {
      let tmp = self.ofs();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x9c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the USR_OFS register."]
   #[inline] pub fn usr_ofs_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xa0) as *const u32
   }
#[doc="Get the *mut pointer for the USR_OFS register."]
   #[inline] pub fn usr_ofs_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xa0) as *mut u32
   }
#[doc="Read the USR_OFS register."]
   #[inline] pub fn usr_ofs(&self) -> UsrOfs { 
      unsafe {
         UsrOfs(::core::ptr::read_volatile((self.0 + 0xa0) as *const u32))
      }
   }
#[doc="Write the USR_OFS register."]
   #[inline] pub fn set_usr_ofs<F: FnOnce(UsrOfs) -> UsrOfs>(&self, f: F) -> &Self {
      let value = f(UsrOfs(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xa0) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the USR_OFS register."]
   #[inline] pub fn with_usr_ofs<F: FnOnce(UsrOfs) -> UsrOfs>(&self, f: F) -> &Self {
      let tmp = self.usr_ofs();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xa0) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the XOFS register."]
   #[inline] pub fn xofs_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xa4) as *const u32
   }
#[doc="Get the *mut pointer for the XOFS register."]
   #[inline] pub fn xofs_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xa4) as *mut u32
   }
#[doc="Read the XOFS register."]
   #[inline] pub fn xofs(&self) -> Xofs { 
      unsafe {
         Xofs(::core::ptr::read_volatile((self.0 + 0xa4) as *const u32))
      }
   }
#[doc="Write the XOFS register."]
   #[inline] pub fn set_xofs<F: FnOnce(Xofs) -> Xofs>(&self, f: F) -> &Self {
      let value = f(Xofs(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xa4) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the XOFS register."]
   #[inline] pub fn with_xofs<F: FnOnce(Xofs) -> Xofs>(&self, f: F) -> &Self {
      let tmp = self.xofs();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xa4) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the YOFS register."]
   #[inline] pub fn yofs_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xa8) as *const u32
   }
#[doc="Get the *mut pointer for the YOFS register."]
   #[inline] pub fn yofs_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xa8) as *mut u32
   }
#[doc="Read the YOFS register."]
   #[inline] pub fn yofs(&self) -> Yofs { 
      unsafe {
         Yofs(::core::ptr::read_volatile((self.0 + 0xa8) as *const u32))
      }
   }
#[doc="Write the YOFS register."]
   #[inline] pub fn set_yofs<F: FnOnce(Yofs) -> Yofs>(&self, f: F) -> &Self {
      let value = f(Yofs(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xa8) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the YOFS register."]
   #[inline] pub fn with_yofs<F: FnOnce(Yofs) -> Yofs>(&self, f: F) -> &Self {
      let tmp = self.yofs();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xa8) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the G register."]
   #[inline] pub fn g_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xac) as *const u32
   }
#[doc="Get the *mut pointer for the G register."]
   #[inline] pub fn g_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xac) as *mut u32
   }
#[doc="Read the G register."]
   #[inline] pub fn g(&self) -> G { 
      unsafe {
         G(::core::ptr::read_volatile((self.0 + 0xac) as *const u32))
      }
   }
#[doc="Write the G register."]
   #[inline] pub fn set_g<F: FnOnce(G) -> G>(&self, f: F) -> &Self {
      let value = f(G(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xac) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the G register."]
   #[inline] pub fn with_g<F: FnOnce(G) -> G>(&self, f: F) -> &Self {
      let tmp = self.g();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xac) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the UG register."]
   #[inline] pub fn ug_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xb0) as *const u32
   }
#[doc="Get the *mut pointer for the UG register."]
   #[inline] pub fn ug_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xb0) as *mut u32
   }
#[doc="Read the UG register."]
   #[inline] pub fn ug(&self) -> Ug { 
      unsafe {
         Ug(::core::ptr::read_volatile((self.0 + 0xb0) as *const u32))
      }
   }
#[doc="Write the UG register."]
   #[inline] pub fn set_ug<F: FnOnce(Ug) -> Ug>(&self, f: F) -> &Self {
      let value = f(Ug(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xb0) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the UG register."]
   #[inline] pub fn with_ug<F: FnOnce(Ug) -> Ug>(&self, f: F) -> &Self {
      let tmp = self.ug();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xb0) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CLPS register."]
   #[inline] pub fn clps_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xb4) as *const u32
   }
#[doc="Get the *mut pointer for the CLPS register."]
   #[inline] pub fn clps_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xb4) as *mut u32
   }
#[doc="Read the CLPS register."]
   #[inline] pub fn clps(&self) -> Clps { 
      unsafe {
         Clps(::core::ptr::read_volatile((self.0 + 0xb4) as *const u32))
      }
   }
#[doc="Write the CLPS register."]
   #[inline] pub fn set_clps<F: FnOnce(Clps) -> Clps>(&self, f: F) -> &Self {
      let value = f(Clps(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xb4) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CLPS register."]
   #[inline] pub fn with_clps<F: FnOnce(Clps) -> Clps>(&self, f: F) -> &Self {
      let tmp = self.clps();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xb4) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CLP3 register."]
   #[inline] pub fn clp3_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xb8) as *const u32
   }
#[doc="Get the *mut pointer for the CLP3 register."]
   #[inline] pub fn clp3_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xb8) as *mut u32
   }
#[doc="Read the CLP3 register."]
   #[inline] pub fn clp3(&self) -> Clp3 { 
      unsafe {
         Clp3(::core::ptr::read_volatile((self.0 + 0xb8) as *const u32))
      }
   }
#[doc="Write the CLP3 register."]
   #[inline] pub fn set_clp3<F: FnOnce(Clp3) -> Clp3>(&self, f: F) -> &Self {
      let value = f(Clp3(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xb8) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CLP3 register."]
   #[inline] pub fn with_clp3<F: FnOnce(Clp3) -> Clp3>(&self, f: F) -> &Self {
      let tmp = self.clp3();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xb8) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CLP2 register."]
   #[inline] pub fn clp2_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xbc) as *const u32
   }
#[doc="Get the *mut pointer for the CLP2 register."]
   #[inline] pub fn clp2_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xbc) as *mut u32
   }
#[doc="Read the CLP2 register."]
   #[inline] pub fn clp2(&self) -> Clp2 { 
      unsafe {
         Clp2(::core::ptr::read_volatile((self.0 + 0xbc) as *const u32))
      }
   }
#[doc="Write the CLP2 register."]
   #[inline] pub fn set_clp2<F: FnOnce(Clp2) -> Clp2>(&self, f: F) -> &Self {
      let value = f(Clp2(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xbc) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CLP2 register."]
   #[inline] pub fn with_clp2<F: FnOnce(Clp2) -> Clp2>(&self, f: F) -> &Self {
      let tmp = self.clp2();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xbc) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CLP1 register."]
   #[inline] pub fn clp1_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xc0) as *const u32
   }
#[doc="Get the *mut pointer for the CLP1 register."]
   #[inline] pub fn clp1_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xc0) as *mut u32
   }
#[doc="Read the CLP1 register."]
   #[inline] pub fn clp1(&self) -> Clp1 { 
      unsafe {
         Clp1(::core::ptr::read_volatile((self.0 + 0xc0) as *const u32))
      }
   }
#[doc="Write the CLP1 register."]
   #[inline] pub fn set_clp1<F: FnOnce(Clp1) -> Clp1>(&self, f: F) -> &Self {
      let value = f(Clp1(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xc0) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CLP1 register."]
   #[inline] pub fn with_clp1<F: FnOnce(Clp1) -> Clp1>(&self, f: F) -> &Self {
      let tmp = self.clp1();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xc0) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CLP0 register."]
   #[inline] pub fn clp0_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xc4) as *const u32
   }
#[doc="Get the *mut pointer for the CLP0 register."]
   #[inline] pub fn clp0_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xc4) as *mut u32
   }
#[doc="Read the CLP0 register."]
   #[inline] pub fn clp0(&self) -> Clp0 { 
      unsafe {
         Clp0(::core::ptr::read_volatile((self.0 + 0xc4) as *const u32))
      }
   }
#[doc="Write the CLP0 register."]
   #[inline] pub fn set_clp0<F: FnOnce(Clp0) -> Clp0>(&self, f: F) -> &Self {
      let value = f(Clp0(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xc4) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CLP0 register."]
   #[inline] pub fn with_clp0<F: FnOnce(Clp0) -> Clp0>(&self, f: F) -> &Self {
      let tmp = self.clp0();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xc4) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CLPX register."]
   #[inline] pub fn clpx_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xc8) as *const u32
   }
#[doc="Get the *mut pointer for the CLPX register."]
   #[inline] pub fn clpx_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xc8) as *mut u32
   }
#[doc="Read the CLPX register."]
   #[inline] pub fn clpx(&self) -> Clpx { 
      unsafe {
         Clpx(::core::ptr::read_volatile((self.0 + 0xc8) as *const u32))
      }
   }
#[doc="Write the CLPX register."]
   #[inline] pub fn set_clpx<F: FnOnce(Clpx) -> Clpx>(&self, f: F) -> &Self {
      let value = f(Clpx(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xc8) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CLPX register."]
   #[inline] pub fn with_clpx<F: FnOnce(Clpx) -> Clpx>(&self, f: F) -> &Self {
      let tmp = self.clpx();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xc8) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CLP9 register."]
   #[inline] pub fn clp9_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xcc) as *const u32
   }
#[doc="Get the *mut pointer for the CLP9 register."]
   #[inline] pub fn clp9_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xcc) as *mut u32
   }
#[doc="Read the CLP9 register."]
   #[inline] pub fn clp9(&self) -> Clp9 { 
      unsafe {
         Clp9(::core::ptr::read_volatile((self.0 + 0xcc) as *const u32))
      }
   }
#[doc="Write the CLP9 register."]
   #[inline] pub fn set_clp9<F: FnOnce(Clp9) -> Clp9>(&self, f: F) -> &Self {
      let value = f(Clp9(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xcc) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CLP9 register."]
   #[inline] pub fn with_clp9<F: FnOnce(Clp9) -> Clp9>(&self, f: F) -> &Self {
      let tmp = self.clp9();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xcc) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CLPS_OFS register."]
   #[inline] pub fn clps_ofs_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xd0) as *const u32
   }
#[doc="Get the *mut pointer for the CLPS_OFS register."]
   #[inline] pub fn clps_ofs_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xd0) as *mut u32
   }
#[doc="Read the CLPS_OFS register."]
   #[inline] pub fn clps_ofs(&self) -> ClpsOfs { 
      unsafe {
         ClpsOfs(::core::ptr::read_volatile((self.0 + 0xd0) as *const u32))
      }
   }
#[doc="Write the CLPS_OFS register."]
   #[inline] pub fn set_clps_ofs<F: FnOnce(ClpsOfs) -> ClpsOfs>(&self, f: F) -> &Self {
      let value = f(ClpsOfs(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xd0) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CLPS_OFS register."]
   #[inline] pub fn with_clps_ofs<F: FnOnce(ClpsOfs) -> ClpsOfs>(&self, f: F) -> &Self {
      let tmp = self.clps_ofs();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xd0) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CLP3_OFS register."]
   #[inline] pub fn clp3_ofs_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xd4) as *const u32
   }
#[doc="Get the *mut pointer for the CLP3_OFS register."]
   #[inline] pub fn clp3_ofs_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xd4) as *mut u32
   }
#[doc="Read the CLP3_OFS register."]
   #[inline] pub fn clp3_ofs(&self) -> Clp3Ofs { 
      unsafe {
         Clp3Ofs(::core::ptr::read_volatile((self.0 + 0xd4) as *const u32))
      }
   }
#[doc="Write the CLP3_OFS register."]
   #[inline] pub fn set_clp3_ofs<F: FnOnce(Clp3Ofs) -> Clp3Ofs>(&self, f: F) -> &Self {
      let value = f(Clp3Ofs(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xd4) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CLP3_OFS register."]
   #[inline] pub fn with_clp3_ofs<F: FnOnce(Clp3Ofs) -> Clp3Ofs>(&self, f: F) -> &Self {
      let tmp = self.clp3_ofs();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xd4) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CLP2_OFS register."]
   #[inline] pub fn clp2_ofs_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xd8) as *const u32
   }
#[doc="Get the *mut pointer for the CLP2_OFS register."]
   #[inline] pub fn clp2_ofs_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xd8) as *mut u32
   }
#[doc="Read the CLP2_OFS register."]
   #[inline] pub fn clp2_ofs(&self) -> Clp2Ofs { 
      unsafe {
         Clp2Ofs(::core::ptr::read_volatile((self.0 + 0xd8) as *const u32))
      }
   }
#[doc="Write the CLP2_OFS register."]
   #[inline] pub fn set_clp2_ofs<F: FnOnce(Clp2Ofs) -> Clp2Ofs>(&self, f: F) -> &Self {
      let value = f(Clp2Ofs(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xd8) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CLP2_OFS register."]
   #[inline] pub fn with_clp2_ofs<F: FnOnce(Clp2Ofs) -> Clp2Ofs>(&self, f: F) -> &Self {
      let tmp = self.clp2_ofs();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xd8) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CLP1_OFS register."]
   #[inline] pub fn clp1_ofs_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xdc) as *const u32
   }
#[doc="Get the *mut pointer for the CLP1_OFS register."]
   #[inline] pub fn clp1_ofs_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xdc) as *mut u32
   }
#[doc="Read the CLP1_OFS register."]
   #[inline] pub fn clp1_ofs(&self) -> Clp1Ofs { 
      unsafe {
         Clp1Ofs(::core::ptr::read_volatile((self.0 + 0xdc) as *const u32))
      }
   }
#[doc="Write the CLP1_OFS register."]
   #[inline] pub fn set_clp1_ofs<F: FnOnce(Clp1Ofs) -> Clp1Ofs>(&self, f: F) -> &Self {
      let value = f(Clp1Ofs(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xdc) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CLP1_OFS register."]
   #[inline] pub fn with_clp1_ofs<F: FnOnce(Clp1Ofs) -> Clp1Ofs>(&self, f: F) -> &Self {
      let tmp = self.clp1_ofs();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xdc) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CLP0_OFS register."]
   #[inline] pub fn clp0_ofs_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xe0) as *const u32
   }
#[doc="Get the *mut pointer for the CLP0_OFS register."]
   #[inline] pub fn clp0_ofs_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xe0) as *mut u32
   }
#[doc="Read the CLP0_OFS register."]
   #[inline] pub fn clp0_ofs(&self) -> Clp0Ofs { 
      unsafe {
         Clp0Ofs(::core::ptr::read_volatile((self.0 + 0xe0) as *const u32))
      }
   }
#[doc="Write the CLP0_OFS register."]
   #[inline] pub fn set_clp0_ofs<F: FnOnce(Clp0Ofs) -> Clp0Ofs>(&self, f: F) -> &Self {
      let value = f(Clp0Ofs(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xe0) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CLP0_OFS register."]
   #[inline] pub fn with_clp0_ofs<F: FnOnce(Clp0Ofs) -> Clp0Ofs>(&self, f: F) -> &Self {
      let tmp = self.clp0_ofs();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xe0) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CLPX_OFS register."]
   #[inline] pub fn clpx_ofs_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xe4) as *const u32
   }
#[doc="Get the *mut pointer for the CLPX_OFS register."]
   #[inline] pub fn clpx_ofs_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xe4) as *mut u32
   }
#[doc="Read the CLPX_OFS register."]
   #[inline] pub fn clpx_ofs(&self) -> ClpxOfs { 
      unsafe {
         ClpxOfs(::core::ptr::read_volatile((self.0 + 0xe4) as *const u32))
      }
   }
#[doc="Write the CLPX_OFS register."]
   #[inline] pub fn set_clpx_ofs<F: FnOnce(ClpxOfs) -> ClpxOfs>(&self, f: F) -> &Self {
      let value = f(ClpxOfs(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xe4) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CLPX_OFS register."]
   #[inline] pub fn with_clpx_ofs<F: FnOnce(ClpxOfs) -> ClpxOfs>(&self, f: F) -> &Self {
      let tmp = self.clpx_ofs();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xe4) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CLP9_OFS register."]
   #[inline] pub fn clp9_ofs_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xe8) as *const u32
   }
#[doc="Get the *mut pointer for the CLP9_OFS register."]
   #[inline] pub fn clp9_ofs_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xe8) as *mut u32
   }
#[doc="Read the CLP9_OFS register."]
   #[inline] pub fn clp9_ofs(&self) -> Clp9Ofs { 
      unsafe {
         Clp9Ofs(::core::ptr::read_volatile((self.0 + 0xe8) as *const u32))
      }
   }
#[doc="Write the CLP9_OFS register."]
   #[inline] pub fn set_clp9_ofs<F: FnOnce(Clp9Ofs) -> Clp9Ofs>(&self, f: F) -> &Self {
      let value = f(Clp9Ofs(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xe8) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CLP9_OFS register."]
   #[inline] pub fn with_clp9_ofs<F: FnOnce(Clp9Ofs) -> Clp9Ofs>(&self, f: F) -> &Self {
      let tmp = self.clp9_ofs();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xe8) as *mut u32, value.0);
      }
      self
   }

}

#[doc="ADC Status and Control Register 1"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sc1(pub u32);
impl Sc1 {
#[doc="Input channel select"]
   #[inline] pub fn adch(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
   }
#[doc="Input channel select"]
   #[inline] pub fn set_adch<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Interrupt Enable"]
   #[inline] pub fn aien(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Interrupt Enable"]
   #[inline] pub fn set_aien<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Conversion Complete Flag"]
   #[inline] pub fn coco(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Conversion Complete Flag"]
   #[inline] pub fn set_coco<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Sc1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sc1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.adch() != 0 { try!(write!(f, " adch=0x{:x}", self.adch()))}
      if self.aien() != 0 { try!(write!(f, " aien"))}
      if self.coco() != 0 { try!(write!(f, " coco"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC Configuration Register 1"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cfg1(pub u32);
impl Cfg1 {
#[doc="Input Clock Select"]
   #[inline] pub fn adiclk(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
   }
#[doc="Input Clock Select"]
   #[inline] pub fn set_adiclk<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Conversion mode selection"]
   #[inline] pub fn mode(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
   }
#[doc="Conversion mode selection"]
   #[inline] pub fn set_mode<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Clock Divide Select"]
   #[inline] pub fn adiv(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
   }
#[doc="Clock Divide Select"]
   #[inline] pub fn set_adiv<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Clear Latch Trigger in Trigger Handler Block"]
   #[inline] pub fn clrltrg(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="Clear Latch Trigger in Trigger Handler Block"]
   #[inline] pub fn set_clrltrg<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

}
impl ::core::fmt::Display for Cfg1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cfg1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.adiclk() != 0 { try!(write!(f, " adiclk=0x{:x}", self.adiclk()))}
      if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
      if self.adiv() != 0 { try!(write!(f, " adiv=0x{:x}", self.adiv()))}
      if self.clrltrg() != 0 { try!(write!(f, " clrltrg"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC Configuration Register 2"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cfg2(pub u32);
impl Cfg2 {
#[doc="Sample Time Select"]
   #[inline] pub fn smplts(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Sample Time Select"]
   #[inline] pub fn set_smplts<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Cfg2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cfg2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.smplts() != 0 { try!(write!(f, " smplts=0x{:x}", self.smplts()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC Data Result Registers"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct R(pub u32);
impl R {
#[doc="Data result"]
   #[inline] pub fn d(&self) -> bits::U12 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
   }
#[doc="Data result"]
   #[inline] pub fn set_d<V: Into<bits::U12>>(mut self, value: V) -> Self {
      let value: bits::U12 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xfff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Data result (12 bit)"]
   #[inline] pub fn d12(&self) -> bits::U12 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
   }
#[doc="Data result (12 bit)"]
   #[inline] pub fn set_d12<V: Into<bits::U12>>(mut self, value: V) -> Self {
      let value: bits::U12 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xfff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Data result (10 bit)"]
   #[inline] pub fn d10(&self) -> bits::U10 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
   }
#[doc="Data result (10 bit)"]
   #[inline] pub fn set_d10<V: Into<bits::U10>>(mut self, value: V) -> Self {
      let value: bits::U10 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3ff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Data result (8 bit)"]
   #[inline] pub fn d8(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Data result (8 bit)"]
   #[inline] pub fn set_d8<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for R {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for R {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d() != 0 { try!(write!(f, " d=0x{:x}", self.d()))}
      if self.d12() != 0 { try!(write!(f, " d12=0x{:x}", self.d12()))}
      if self.d10() != 0 { try!(write!(f, " d10=0x{:x}", self.d10()))}
      if self.d8() != 0 { try!(write!(f, " d8=0x{:x}", self.d8()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Compare Value Registers"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cv(pub u32);
impl Cv {
#[doc="Compare Value."]
   #[inline] pub fn cv(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Compare Value."]
   #[inline] pub fn set_cv<V: Into<bits::U16>>(mut self, value: V) -> Self {
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
      if self.cv() != 0 { try!(write!(f, " cv=0x{:x}", self.cv()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Status and Control Register 2"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sc2(pub u32);
impl Sc2 {
#[doc="Voltage Reference Selection"]
   #[inline] pub fn refsel(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
   }
#[doc="Voltage Reference Selection"]
   #[inline] pub fn set_refsel<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="DMA Enable"]
   #[inline] pub fn dmaen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="DMA Enable"]
   #[inline] pub fn set_dmaen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Compare Function Range Enable"]
   #[inline] pub fn acren(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Compare Function Range Enable"]
   #[inline] pub fn set_acren<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Compare Function Greater Than Enable"]
   #[inline] pub fn acfgt(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Compare Function Greater Than Enable"]
   #[inline] pub fn set_acfgt<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Compare Function Enable"]
   #[inline] pub fn acfe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Compare Function Enable"]
   #[inline] pub fn set_acfe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Conversion Trigger Select"]
   #[inline] pub fn adtrg(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Conversion Trigger Select"]
   #[inline] pub fn set_adtrg<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Conversion Active"]
   #[inline] pub fn adact(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Conversion Active"]
   #[inline] pub fn set_adact<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="Trigger Process Number"]
   #[inline] pub fn trgprnum(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x3) as u8) } // [14:13]
   }
#[doc="Trigger Process Number"]
   #[inline] pub fn set_trgprnum<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 13);
      self.0 |= value << 13;
      self
   }

#[doc="Trigger Status"]
   #[inline] pub fn trgstlat(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
   }
#[doc="Trigger Status"]
   #[inline] pub fn set_trgstlat<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Error in Multiplexed Trigger Request"]
   #[inline] pub fn trgsterr(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
   }
#[doc="Error in Multiplexed Trigger Request"]
   #[inline] pub fn set_trgsterr<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 24);
      self.0 |= value << 24;
      self
   }

}
impl ::core::fmt::Display for Sc2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sc2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.refsel() != 0 { try!(write!(f, " refsel=0x{:x}", self.refsel()))}
      if self.dmaen() != 0 { try!(write!(f, " dmaen"))}
      if self.acren() != 0 { try!(write!(f, " acren"))}
      if self.acfgt() != 0 { try!(write!(f, " acfgt"))}
      if self.acfe() != 0 { try!(write!(f, " acfe"))}
      if self.adtrg() != 0 { try!(write!(f, " adtrg"))}
      if self.adact() != 0 { try!(write!(f, " adact"))}
      if self.trgprnum() != 0 { try!(write!(f, " trgprnum=0x{:x}", self.trgprnum()))}
      if self.trgstlat() != 0 { try!(write!(f, " trgstlat=0x{:x}", self.trgstlat()))}
      if self.trgsterr() != 0 { try!(write!(f, " trgsterr=0x{:x}", self.trgsterr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Status and Control Register 3"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sc3(pub u32);
impl Sc3 {
#[doc="Hardware Average Select"]
   #[inline] pub fn avgs(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
   }
#[doc="Hardware Average Select"]
   #[inline] pub fn set_avgs<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Hardware Average Enable"]
   #[inline] pub fn avge(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Hardware Average Enable"]
   #[inline] pub fn set_avge<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Continuous Conversion Enable"]
   #[inline] pub fn adco(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Continuous Conversion Enable"]
   #[inline] pub fn set_adco<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Calibration"]
   #[inline] pub fn cal(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Calibration"]
   #[inline] pub fn set_cal<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Sc3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sc3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.avgs() != 0 { try!(write!(f, " avgs=0x{:x}", self.avgs()))}
      if self.avge() != 0 { try!(write!(f, " avge"))}
      if self.adco() != 0 { try!(write!(f, " adco"))}
      if self.cal() != 0 { try!(write!(f, " cal"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="BASE Offset Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BaseOfs(pub u32);
impl BaseOfs {
#[doc="Base Offset Error Correction Value"]
   #[inline] pub fn ba_ofs(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Base Offset Error Correction Value"]
   #[inline] pub fn set_ba_ofs<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for BaseOfs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for BaseOfs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ba_ofs() != 0 { try!(write!(f, " ba_ofs=0x{:x}", self.ba_ofs()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC Offset Correction Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ofs(pub u32);
impl Ofs {
#[doc="Offset Error Correction Value"]
   #[inline] pub fn ofs(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Offset Error Correction Value"]
   #[inline] pub fn set_ofs<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Ofs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ofs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ofs() != 0 { try!(write!(f, " ofs=0x{:x}", self.ofs()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="USER Offset Correction Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct UsrOfs(pub u32);
impl UsrOfs {
#[doc="USER Offset Error Correction Value"]
   #[inline] pub fn usr_ofs(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="USER Offset Error Correction Value"]
   #[inline] pub fn set_usr_ofs<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for UsrOfs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for UsrOfs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.usr_ofs() != 0 { try!(write!(f, " usr_ofs=0x{:x}", self.usr_ofs()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC X Offset Correction Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Xofs(pub u32);
impl Xofs {
#[doc="X offset error correction value"]
   #[inline] pub fn xofs(&self) -> bits::U6 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
   }
#[doc="X offset error correction value"]
   #[inline] pub fn set_xofs<V: Into<bits::U6>>(mut self, value: V) -> Self {
      let value: bits::U6 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3f << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Xofs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Xofs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.xofs() != 0 { try!(write!(f, " xofs=0x{:x}", self.xofs()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC Y Offset Correction Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Yofs(pub u32);
impl Yofs {
#[doc="Y offset error correction value"]
   #[inline] pub fn yofs(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Y offset error correction value"]
   #[inline] pub fn set_yofs<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Yofs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Yofs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.yofs() != 0 { try!(write!(f, " yofs=0x{:x}", self.yofs()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC Gain Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct G(pub u32);
impl G {
#[doc="Gain error adjustment factor for the overall conversion"]
   #[inline] pub fn g(&self) -> bits::U11 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
   }
#[doc="Gain error adjustment factor for the overall conversion"]
   #[inline] pub fn set_g<V: Into<bits::U11>>(mut self, value: V) -> Self {
      let value: bits::U11 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7ff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for G {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for G {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.g() != 0 { try!(write!(f, " g=0x{:x}", self.g()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC User Gain Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ug(pub u32);
impl Ug {
#[doc="User gain error correction value"]
   #[inline] pub fn ug(&self) -> bits::U10 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
   }
#[doc="User gain error correction value"]
   #[inline] pub fn set_ug<V: Into<bits::U10>>(mut self, value: V) -> Self {
      let value: bits::U10 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3ff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Ug {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ug {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ug() != 0 { try!(write!(f, " ug=0x{:x}", self.ug()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC General Calibration Value Register S"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clps(pub u32);
impl Clps {
#[doc="Calibration Value"]
   #[inline] pub fn clps(&self) -> bits::U7 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
   }
#[doc="Calibration Value"]
   #[inline] pub fn set_clps<V: Into<bits::U7>>(mut self, value: V) -> Self {
      let value: bits::U7 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7f << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Clps {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Clps {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.clps() != 0 { try!(write!(f, " clps=0x{:x}", self.clps()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC Plus-Side General Calibration Value Register 3"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clp3(pub u32);
impl Clp3 {
#[doc="Calibration Value"]
   #[inline] pub fn clp3(&self) -> bits::U10 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
   }
#[doc="Calibration Value"]
   #[inline] pub fn set_clp3<V: Into<bits::U10>>(mut self, value: V) -> Self {
      let value: bits::U10 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3ff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Clp3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Clp3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.clp3() != 0 { try!(write!(f, " clp3=0x{:x}", self.clp3()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC Plus-Side General Calibration Value Register 2"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clp2(pub u32);
impl Clp2 {
#[doc="Calibration Value"]
   #[inline] pub fn clp2(&self) -> bits::U10 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
   }
#[doc="Calibration Value"]
   #[inline] pub fn set_clp2<V: Into<bits::U10>>(mut self, value: V) -> Self {
      let value: bits::U10 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3ff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Clp2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Clp2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.clp2() != 0 { try!(write!(f, " clp2=0x{:x}", self.clp2()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC Plus-Side General Calibration Value Register 1"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clp1(pub u32);
impl Clp1 {
#[doc="Calibration Value"]
   #[inline] pub fn clp1(&self) -> bits::U9 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
   }
#[doc="Calibration Value"]
   #[inline] pub fn set_clp1<V: Into<bits::U9>>(mut self, value: V) -> Self {
      let value: bits::U9 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1ff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Clp1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Clp1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.clp1() != 0 { try!(write!(f, " clp1=0x{:x}", self.clp1()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC Plus-Side General Calibration Value Register 0"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clp0(pub u32);
impl Clp0 {
#[doc="Calibration Value"]
   #[inline] pub fn clp0(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Calibration Value"]
   #[inline] pub fn set_clp0<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Clp0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Clp0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.clp0() != 0 { try!(write!(f, " clp0=0x{:x}", self.clp0()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC Plus-Side General Calibration Value Register X"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clpx(pub u32);
impl Clpx {
#[doc="Calibration Value"]
   #[inline] pub fn clpx(&self) -> bits::U7 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
   }
#[doc="Calibration Value"]
   #[inline] pub fn set_clpx<V: Into<bits::U7>>(mut self, value: V) -> Self {
      let value: bits::U7 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7f << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Clpx {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Clpx {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.clpx() != 0 { try!(write!(f, " clpx=0x{:x}", self.clpx()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC Plus-Side General Calibration Value Register 9"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clp9(pub u32);
impl Clp9 {
#[doc="Calibration Value"]
   #[inline] pub fn clp9(&self) -> bits::U7 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
   }
#[doc="Calibration Value"]
   #[inline] pub fn set_clp9<V: Into<bits::U7>>(mut self, value: V) -> Self {
      let value: bits::U7 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7f << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Clp9 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Clp9 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.clp9() != 0 { try!(write!(f, " clp9=0x{:x}", self.clp9()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC General Calibration Offset Value Register S"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ClpsOfs(pub u32);
impl ClpsOfs {
#[doc="CLPS Offset"]
   #[inline] pub fn clps_ofs(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
   }
#[doc="CLPS Offset"]
   #[inline] pub fn set_clps_ofs<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for ClpsOfs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ClpsOfs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.clps_ofs() != 0 { try!(write!(f, " clps_ofs=0x{:x}", self.clps_ofs()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC Plus-Side General Calibration Offset Value Register 3"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clp3Ofs(pub u32);
impl Clp3Ofs {
#[doc="CLP3 Offset"]
   #[inline] pub fn clp3_ofs(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
   }
#[doc="CLP3 Offset"]
   #[inline] pub fn set_clp3_ofs<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Clp3Ofs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Clp3Ofs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.clp3_ofs() != 0 { try!(write!(f, " clp3_ofs=0x{:x}", self.clp3_ofs()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC Plus-Side General Calibration Offset Value Register 2"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clp2Ofs(pub u32);
impl Clp2Ofs {
#[doc="CLP2 Offset"]
   #[inline] pub fn clp2_ofs(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
   }
#[doc="CLP2 Offset"]
   #[inline] pub fn set_clp2_ofs<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Clp2Ofs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Clp2Ofs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.clp2_ofs() != 0 { try!(write!(f, " clp2_ofs=0x{:x}", self.clp2_ofs()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC Plus-Side General Calibration Offset Value Register 1"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clp1Ofs(pub u32);
impl Clp1Ofs {
#[doc="CLP1 Offset"]
   #[inline] pub fn clp1_ofs(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
   }
#[doc="CLP1 Offset"]
   #[inline] pub fn set_clp1_ofs<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Clp1Ofs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Clp1Ofs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.clp1_ofs() != 0 { try!(write!(f, " clp1_ofs=0x{:x}", self.clp1_ofs()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC Plus-Side General Calibration Offset Value Register 0"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clp0Ofs(pub u32);
impl Clp0Ofs {
#[doc="CLP0 Offset"]
   #[inline] pub fn clp0_ofs(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
   }
#[doc="CLP0 Offset"]
   #[inline] pub fn set_clp0_ofs<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Clp0Ofs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Clp0Ofs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.clp0_ofs() != 0 { try!(write!(f, " clp0_ofs=0x{:x}", self.clp0_ofs()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC Plus-Side General Calibration Offset Value Register X"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ClpxOfs(pub u32);
impl ClpxOfs {
#[doc="CLPX Offset"]
   #[inline] pub fn clpx_ofs(&self) -> bits::U12 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
   }
#[doc="CLPX Offset"]
   #[inline] pub fn set_clpx_ofs<V: Into<bits::U12>>(mut self, value: V) -> Self {
      let value: bits::U12 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xfff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for ClpxOfs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ClpxOfs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.clpx_ofs() != 0 { try!(write!(f, " clpx_ofs=0x{:x}", self.clpx_ofs()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC Plus-Side General Calibration Offset Value Register 9"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clp9Ofs(pub u32);
impl Clp9Ofs {
#[doc="CLP9 Offset"]
   #[inline] pub fn clp9_ofs(&self) -> bits::U12 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
   }
#[doc="CLP9 Offset"]
   #[inline] pub fn set_clp9_ofs<V: Into<bits::U12>>(mut self, value: V) -> Self {
      let value: bits::U12 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xfff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Clp9Ofs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Clp9Ofs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.clp9_ofs() != 0 { try!(write!(f, " clp9_ofs=0x{:x}", self.clp9_ofs()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
pub struct AdcCh { pub periph: AdcPeriph, pub index: usize }
channel!(ADC0_CH0, Adc0Ch0, ADC0, Adc0, _ADC0_CH0, AdcCh, _ADC0, 0);
channel!(ADC0_CH1, Adc0Ch1, ADC0, Adc0, _ADC0_CH1, AdcCh, _ADC0, 1);
channel!(ADC0_CH2, Adc0Ch2, ADC0, Adc0, _ADC0_CH2, AdcCh, _ADC0, 2);
channel!(ADC0_CH3, Adc0Ch3, ADC0, Adc0, _ADC0_CH3, AdcCh, _ADC0, 3);
channel!(ADC0_CH4, Adc0Ch4, ADC0, Adc0, _ADC0_CH4, AdcCh, _ADC0, 4);
channel!(ADC0_CH5, Adc0Ch5, ADC0, Adc0, _ADC0_CH5, AdcCh, _ADC0, 5);
channel!(ADC0_CH6, Adc0Ch6, ADC0, Adc0, _ADC0_CH6, AdcCh, _ADC0, 6);
channel!(ADC0_CH7, Adc0Ch7, ADC0, Adc0, _ADC0_CH7, AdcCh, _ADC0, 7);
channel!(ADC0_CH8, Adc0Ch8, ADC0, Adc0, _ADC0_CH8, AdcCh, _ADC0, 8);
channel!(ADC0_CH9, Adc0Ch9, ADC0, Adc0, _ADC0_CH9, AdcCh, _ADC0, 9);
channel!(ADC0_CH10, Adc0Ch10, ADC0, Adc0, _ADC0_CH10, AdcCh, _ADC0, 10);
channel!(ADC0_CH11, Adc0Ch11, ADC0, Adc0, _ADC0_CH11, AdcCh, _ADC0, 11);
channel!(ADC0_CH12, Adc0Ch12, ADC0, Adc0, _ADC0_CH12, AdcCh, _ADC0, 12);
channel!(ADC0_CH13, Adc0Ch13, ADC0, Adc0, _ADC0_CH13, AdcCh, _ADC0, 13);
channel!(ADC0_CH14, Adc0Ch14, ADC0, Adc0, _ADC0_CH14, AdcCh, _ADC0, 14);
channel!(ADC0_CH15, Adc0Ch15, ADC0, Adc0, _ADC0_CH15, AdcCh, _ADC0, 15);
channel!(ADC0_IN0, Adc0In0, ADC0, Adc0, _ADC0_IN0, AdcCh, _ADC0, 21);
channel!(ADC0_BANDGAP, Adc0Bandgap, ADC0, Adc0, _ADC0_BANDGAP, AdcCh, _ADC0, 27);
channel!(ADC0_REFSH, Adc0Refsh, ADC0, Adc0, _ADC0_REFSH, AdcCh, _ADC0, 29);
channel!(ADC0_REFSL, Adc0Refsl, ADC0, Adc0, _ADC0_REFSL, AdcCh, _ADC0, 30);
channel!(ADC1_CH0, Adc1Ch0, ADC1, Adc1, _ADC1_CH0, AdcCh, _ADC1, 0);
channel!(ADC1_CH1, Adc1Ch1, ADC1, Adc1, _ADC1_CH1, AdcCh, _ADC1, 1);
channel!(ADC1_CH2, Adc1Ch2, ADC1, Adc1, _ADC1_CH2, AdcCh, _ADC1, 2);
channel!(ADC1_CH3, Adc1Ch3, ADC1, Adc1, _ADC1_CH3, AdcCh, _ADC1, 3);
channel!(ADC1_CH4, Adc1Ch4, ADC1, Adc1, _ADC1_CH4, AdcCh, _ADC1, 4);
channel!(ADC1_CH5, Adc1Ch5, ADC1, Adc1, _ADC1_CH5, AdcCh, _ADC1, 5);
channel!(ADC1_CH6, Adc1Ch6, ADC1, Adc1, _ADC1_CH6, AdcCh, _ADC1, 6);
channel!(ADC1_CH7, Adc1Ch7, ADC1, Adc1, _ADC1_CH7, AdcCh, _ADC1, 7);
channel!(ADC1_CH8, Adc1Ch8, ADC1, Adc1, _ADC1_CH8, AdcCh, _ADC1, 8);
channel!(ADC1_CH9, Adc1Ch9, ADC1, Adc1, _ADC1_CH9, AdcCh, _ADC1, 9);
channel!(ADC1_CH10, Adc1Ch10, ADC1, Adc1, _ADC1_CH10, AdcCh, _ADC1, 10);
channel!(ADC1_CH11, Adc1Ch11, ADC1, Adc1, _ADC1_CH11, AdcCh, _ADC1, 11);
channel!(ADC1_CH12, Adc1Ch12, ADC1, Adc1, _ADC1_CH12, AdcCh, _ADC1, 12);
channel!(ADC1_CH13, Adc1Ch13, ADC1, Adc1, _ADC1_CH13, AdcCh, _ADC1, 13);
channel!(ADC1_CH14, Adc1Ch14, ADC1, Adc1, _ADC1_CH14, AdcCh, _ADC1, 14);
channel!(ADC1_CH15, Adc1Ch15, ADC1, Adc1, _ADC1_CH15, AdcCh, _ADC1, 15);
channel!(ADC1_IN0, Adc1In0, ADC1, Adc1, _ADC1_IN0, AdcCh, _ADC1, 21);
channel!(ADC1_BANDGAP, Adc1Bandgap, ADC1, Adc1, _ADC1_BANDGAP, AdcCh, _ADC1, 27);
channel!(ADC1_REFSH, Adc1Refsh, ADC1, Adc1, _ADC1_REFSH, AdcCh, _ADC1, 29);
channel!(ADC1_REFSL, Adc1Refsl, ADC1, Adc1, _ADC1_REFSL, AdcCh, _ADC1, 30);
