#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "LPUART", peripherals: [], prototype: Some(Peripheral { derived_from: None, group_name: Some("LPUART"), name: "", address: 0, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [], clusters: [], registers: [Register { name: "VERID", offset: 0, size: Some(32), access: Some(ReadOnly), reset_value: Some(67174403), reset_mask: Some(4294967295), description: Some("Version ID Register"), fields: [Field { name: "FEATURE", bit_offset: 0, bit_width: 16, access: Some(ReadOnly), description: Some("Feature Identification Number"), enumerated_values: [EnumeratedValue { value: "#1", name: Some("0000000000000001"), description: Some("Standard feature set.") }, EnumeratedValue { value: "#11", name: Some("0000000000000011"), description: Some("Standard feature set with MODEM/IrDA support.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MINOR", bit_offset: 16, bit_width: 8, access: Some(ReadOnly), description: Some("Minor Version Number"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MAJOR", bit_offset: 24, bit_width: 8, access: Some(ReadOnly), description: Some("Major Version Number"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "PARAM", offset: 4, size: Some(32), access: Some(ReadOnly), reset_value: Some(514), reset_mask: Some(4294967295), description: Some("Parameter Register"), fields: [Field { name: "TXFIFO", bit_offset: 0, bit_width: 8, access: Some(ReadOnly), description: Some("Transmit FIFO Size"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RXFIFO", bit_offset: 8, bit_width: 8, access: Some(ReadOnly), description: Some("Receive FIFO Size"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "GLOBAL", offset: 8, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("LPUART Global Register"), fields: [Field { name: "RST", bit_offset: 1, bit_width: 1, access: Some(ReadWrite), description: Some("Software Reset"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Module is not reset.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Module is reset.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "PINCFG", offset: 12, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("LPUART Pin Configuration Register"), fields: [Field { name: "TRGSEL", bit_offset: 0, bit_width: 2, access: Some(ReadWrite), description: Some("Trigger Select"), enumerated_values: [EnumeratedValue { value: "#00", name: Some("00"), description: Some("Input trigger is disabled.") }, EnumeratedValue { value: "#01", name: Some("01"), description: Some("Input trigger is used instead of RXD pin input.") }, EnumeratedValue { value: "#10", name: Some("10"), description: Some("Input trigger is used instead of CTS_B pin input.") }, EnumeratedValue { value: "#11", name: Some("11"), description: Some("Input trigger is used to modulate the TXD pin output. The TXD pin output (after TXINV configuration) is ANDed with the input trigger.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "BAUD", offset: 16, size: Some(32), access: Some(ReadWrite), reset_value: Some(251658244), reset_mask: Some(4294967295), description: Some("LPUART Baud Rate Register"), fields: [Field { name: "SBR", bit_offset: 0, bit_width: 13, access: Some(ReadWrite), description: Some("Baud Rate Modulo Divisor."), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SBNS", bit_offset: 13, bit_width: 1, access: Some(ReadWrite), description: Some("Stop Bit Number Select"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("One stop bit.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Two stop bits.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RXEDGIE", bit_offset: 14, bit_width: 1, access: Some(ReadWrite), description: Some("RX Input Active Edge Interrupt Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Hardware interrupts from LPUART_STAT[RXEDGIF] disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Hardware interrupt requested when LPUART_STAT[RXEDGIF] flag is 1.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "LBKDIE", bit_offset: 15, bit_width: 1, access: Some(ReadWrite), description: Some("LIN Break Detect Interrupt Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Hardware interrupts from LPUART_STAT[LBKDIF] disabled (use polling).") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Hardware interrupt requested when LPUART_STAT[LBKDIF] flag is 1.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RESYNCDIS", bit_offset: 16, bit_width: 1, access: Some(ReadWrite), description: Some("Resynchronization Disable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Resynchronization during received data word is supported") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Resynchronization during received data word is disabled") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "BOTHEDGE", bit_offset: 17, bit_width: 1, access: Some(ReadWrite), description: Some("Both Edge Sampling"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Receiver samples input data using the rising edge of the baud rate clock.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Receiver samples input data using the rising and falling edge of the baud rate clock.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MATCFG", bit_offset: 18, bit_width: 2, access: Some(ReadWrite), description: Some("Match Configuration"), enumerated_values: [EnumeratedValue { value: "#00", name: Some("00"), description: Some("Address Match Wakeup") }, EnumeratedValue { value: "#01", name: Some("01"), description: Some("Idle Match Wakeup") }, EnumeratedValue { value: "#10", name: Some("10"), description: Some("Match On and Match Off") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RIDMAE", bit_offset: 20, bit_width: 1, access: Some(ReadWrite), description: Some("Receiver Idle DMA Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("DMA request disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("DMA request enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RDMAE", bit_offset: 21, bit_width: 1, access: Some(ReadWrite), description: Some("Receiver Full DMA Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("DMA request disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("DMA request enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TDMAE", bit_offset: 23, bit_width: 1, access: Some(ReadWrite), description: Some("Transmitter DMA Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("DMA request disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("DMA request enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "OSR", bit_offset: 24, bit_width: 5, access: Some(ReadWrite), description: Some("Oversampling Ratio"), enumerated_values: [EnumeratedValue { value: "#00000", name: Some("00000"), description: Some("Writing 0 to this field will result in an oversampling ratio of 16") }, EnumeratedValue { value: "#00011", name: Some("00011"), description: Some("Oversampling ratio of 4, requires BOTHEDGE to be set.") }, EnumeratedValue { value: "#00100", name: Some("00100"), description: Some("Oversampling ratio of 5, requires BOTHEDGE to be set.") }, EnumeratedValue { value: "#00101", name: Some("00101"), description: Some("Oversampling ratio of 6, requires BOTHEDGE to be set.") }, EnumeratedValue { value: "#00110", name: Some("00110"), description: Some("Oversampling ratio of 7, requires BOTHEDGE to be set.") }, EnumeratedValue { value: "#00111", name: Some("00111"), description: Some("Oversampling ratio of 8.") }, EnumeratedValue { value: "#01000", name: Some("01000"), description: Some("Oversampling ratio of 9.") }, EnumeratedValue { value: "#01001", name: Some("01001"), description: Some("Oversampling ratio of 10.") }, EnumeratedValue { value: "#01010", name: Some("01010"), description: Some("Oversampling ratio of 11.") }, EnumeratedValue { value: "#01011", name: Some("01011"), description: Some("Oversampling ratio of 12.") }, EnumeratedValue { value: "#01100", name: Some("01100"), description: Some("Oversampling ratio of 13.") }, EnumeratedValue { value: "#01101", name: Some("01101"), description: Some("Oversampling ratio of 14.") }, EnumeratedValue { value: "#01110", name: Some("01110"), description: Some("Oversampling ratio of 15.") }, EnumeratedValue { value: "#01111", name: Some("01111"), description: Some("Oversampling ratio of 16.") }, EnumeratedValue { value: "#10000", name: Some("10000"), description: Some("Oversampling ratio of 17.") }, EnumeratedValue { value: "#10001", name: Some("10001"), description: Some("Oversampling ratio of 18.") }, EnumeratedValue { value: "#10010", name: Some("10010"), description: Some("Oversampling ratio of 19.") }, EnumeratedValue { value: "#10011", name: Some("10011"), description: Some("Oversampling ratio of 20.") }, EnumeratedValue { value: "#10100", name: Some("10100"), description: Some("Oversampling ratio of 21.") }, EnumeratedValue { value: "#10101", name: Some("10101"), description: Some("Oversampling ratio of 22.") }, EnumeratedValue { value: "#10110", name: Some("10110"), description: Some("Oversampling ratio of 23.") }, EnumeratedValue { value: "#10111", name: Some("10111"), description: Some("Oversampling ratio of 24.") }, EnumeratedValue { value: "#11000", name: Some("11000"), description: Some("Oversampling ratio of 25.") }, EnumeratedValue { value: "#11001", name: Some("11001"), description: Some("Oversampling ratio of 26.") }, EnumeratedValue { value: "#11010", name: Some("11010"), description: Some("Oversampling ratio of 27.") }, EnumeratedValue { value: "#11011", name: Some("11011"), description: Some("Oversampling ratio of 28.") }, EnumeratedValue { value: "#11100", name: Some("11100"), description: Some("Oversampling ratio of 29.") }, EnumeratedValue { value: "#11101", name: Some("11101"), description: Some("Oversampling ratio of 30.") }, EnumeratedValue { value: "#11110", name: Some("11110"), description: Some("Oversampling ratio of 31.") }, EnumeratedValue { value: "#11111", name: Some("11111"), description: Some("Oversampling ratio of 32.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "M10", bit_offset: 29, bit_width: 1, access: Some(ReadWrite), description: Some("10-bit Mode select"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Receiver and transmitter use 7-bit to 9-bit data characters.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Receiver and transmitter use 10-bit data characters.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MAEN2", bit_offset: 30, bit_width: 1, access: Some(ReadWrite), description: Some("Match Address Mode Enable 2"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Normal operation.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Enables automatic address matching or data matching mode for MATCH[MA2].") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MAEN1", bit_offset: 31, bit_width: 1, access: Some(ReadWrite), description: Some("Match Address Mode Enable 1"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Normal operation.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Enables automatic address matching or data matching mode for MATCH[MA1].") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "STAT", offset: 20, size: Some(32), access: Some(ReadWrite), reset_value: Some(12582912), reset_mask: Some(4294967295), description: Some("LPUART Status Register"), fields: [Field { name: "MA2F", bit_offset: 14, bit_width: 1, access: Some(ReadWrite), description: Some("Match 2 Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Received data is not equal to MA2") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Received data is equal to MA2") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MA1F", bit_offset: 15, bit_width: 1, access: Some(ReadWrite), description: Some("Match 1 Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Received data is not equal to MA1") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Received data is equal to MA1") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "PF", bit_offset: 16, bit_width: 1, access: Some(ReadWrite), description: Some("Parity Error Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No parity error.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Parity error.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "FE", bit_offset: 17, bit_width: 1, access: Some(ReadWrite), description: Some("Framing Error Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No framing error detected. This does not guarantee the framing is correct.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Framing error.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "NF", bit_offset: 18, bit_width: 1, access: Some(ReadWrite), description: Some("Noise Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No noise detected.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Noise detected in the received character in LPUART_DATA.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "OR", bit_offset: 19, bit_width: 1, access: Some(ReadWrite), description: Some("Receiver Overrun Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No overrun.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Receive overrun (new LPUART data lost).") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "IDLE", bit_offset: 20, bit_width: 1, access: Some(ReadWrite), description: Some("Idle Line Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No idle line detected.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Idle line was detected.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RDRF", bit_offset: 21, bit_width: 1, access: Some(ReadOnly), description: Some("Receive Data Register Full Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Receive data buffer empty.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Receive data buffer full.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TC", bit_offset: 22, bit_width: 1, access: Some(ReadOnly), description: Some("Transmission Complete Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Transmitter active (sending data, a preamble, or a break).") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Transmitter idle (transmission activity complete).") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TDRE", bit_offset: 23, bit_width: 1, access: Some(ReadOnly), description: Some("Transmit Data Register Empty Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Transmit data buffer full.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Transmit data buffer empty.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RAF", bit_offset: 24, bit_width: 1, access: Some(ReadOnly), description: Some("Receiver Active Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("LPUART receiver idle waiting for a start bit.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("LPUART receiver active (RXD input not idle).") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "LBKDE", bit_offset: 25, bit_width: 1, access: Some(ReadWrite), description: Some("LIN Break Detection Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("LIN break detect is disabled, normal break character can be detected.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("LIN break detect is enabled. LIN break character is detected at length of 11 bit times (if M = 0) or 12 (if M = 1) or 13 (M10 = 1).") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "BRK13", bit_offset: 26, bit_width: 1, access: Some(ReadWrite), description: Some("Break Character Generation Length"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Break character is transmitted with length of 9 to 13 bit times.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Break character is transmitted with length of 12 to 15 bit times.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RWUID", bit_offset: 27, bit_width: 1, access: Some(ReadWrite), description: Some("Receive Wake Up Idle Detect"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("During receive standby state (RWU = 1), the IDLE bit does not get set upon detection of an idle character. During address match wakeup, the IDLE bit does not set when an address does not match.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("During receive standby state (RWU = 1), the IDLE bit gets set upon detection of an idle character. During address match wakeup, the IDLE bit does set when an address does not match.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RXINV", bit_offset: 28, bit_width: 1, access: Some(ReadWrite), description: Some("Receive Data Inversion"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Receive data not inverted.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Receive data inverted.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MSBF", bit_offset: 29, bit_width: 1, access: Some(ReadWrite), description: Some("MSB First"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("LSB (bit0) is the first bit that is transmitted following the start bit. Further, the first bit received after the start bit is identified as bit0.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("MSB (bit9, bit8, bit7 or bit6) is the first bit that is transmitted following the start bit depending on the setting of CTRL[M], CTRL[PE] and BAUD[M10]. Further, the first bit received after the start bit is identified as bit9, bit8, bit7 or bit6 depending on the setting of CTRL[M] and CTRL[PE].") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RXEDGIF", bit_offset: 30, bit_width: 1, access: Some(ReadWrite), description: Some("RXD Pin Active Edge Interrupt Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No active edge on the receive pin has occurred.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("An active edge on the receive pin has occurred.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "LBKDIF", bit_offset: 31, bit_width: 1, access: Some(ReadWrite), description: Some("LIN Break Detect Interrupt Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No LIN break character has been detected.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("LIN break character has been detected.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "CTRL", offset: 24, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("LPUART Control Register"), fields: [Field { name: "PT", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("Parity Type"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Even parity.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Odd parity.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "PE", bit_offset: 1, bit_width: 1, access: Some(ReadWrite), description: Some("Parity Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No hardware parity generation or checking.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Parity enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ILT", bit_offset: 2, bit_width: 1, access: Some(ReadWrite), description: Some("Idle Line Type Select"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Idle character bit count starts after start bit.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Idle character bit count starts after stop bit.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "WAKE", bit_offset: 3, bit_width: 1, access: Some(ReadWrite), description: Some("Receiver Wakeup Method Select"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Configures RWU for idle-line wakeup.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Configures RWU with address-mark wakeup.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "M", bit_offset: 4, bit_width: 1, access: Some(ReadWrite), description: Some("9-Bit or 8-Bit Mode Select"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Receiver and transmitter use 8-bit data characters.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Receiver and transmitter use 9-bit data characters.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RSRC", bit_offset: 5, bit_width: 1, access: Some(ReadWrite), description: Some("Receiver Source Select"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Provided LOOPS is set, RSRC is cleared, selects internal loop back mode and the LPUART does not use the RXD pin.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Single-wire LPUART mode where the TXD pin is connected to the transmitter output and receiver input.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "DOZEEN", bit_offset: 6, bit_width: 1, access: Some(ReadWrite), description: Some("Doze Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("LPUART is enabled in Doze mode.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("LPUART is disabled in Doze mode.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "LOOPS", bit_offset: 7, bit_width: 1, access: Some(ReadWrite), description: Some("Loop Mode Select"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Normal operation - RXD and TXD use separate pins.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Loop mode or single-wire mode where transmitter outputs are internally connected to receiver input (see RSRC bit).") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "IDLECFG", bit_offset: 8, bit_width: 3, access: Some(ReadWrite), description: Some("Idle Configuration"), enumerated_values: [EnumeratedValue { value: "#000", name: Some("000"), description: Some("1 idle character") }, EnumeratedValue { value: "#001", name: Some("001"), description: Some("2 idle characters") }, EnumeratedValue { value: "#010", name: Some("010"), description: Some("4 idle characters") }, EnumeratedValue { value: "#011", name: Some("011"), description: Some("8 idle characters") }, EnumeratedValue { value: "#100", name: Some("100"), description: Some("16 idle characters") }, EnumeratedValue { value: "#101", name: Some("101"), description: Some("32 idle characters") }, EnumeratedValue { value: "#110", name: Some("110"), description: Some("64 idle characters") }, EnumeratedValue { value: "#111", name: Some("111"), description: Some("128 idle characters") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "M7", bit_offset: 11, bit_width: 1, access: Some(ReadWrite), description: Some("7-Bit Mode Select"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Receiver and transmitter use 8-bit to 10-bit data characters.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Receiver and transmitter use 7-bit data characters.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MA2IE", bit_offset: 14, bit_width: 1, access: Some(ReadWrite), description: Some("Match 2 Interrupt Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("MA2F interrupt disabled") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("MA2F interrupt enabled") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MA1IE", bit_offset: 15, bit_width: 1, access: Some(ReadWrite), description: Some("Match 1 Interrupt Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("MA1F interrupt disabled") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("MA1F interrupt enabled") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SBK", bit_offset: 16, bit_width: 1, access: Some(ReadWrite), description: Some("Send Break"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Normal transmitter operation.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Queue break character(s) to be sent.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RWU", bit_offset: 17, bit_width: 1, access: Some(ReadWrite), description: Some("Receiver Wakeup Control"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Normal receiver operation.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("LPUART receiver in standby waiting for wakeup condition.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RE", bit_offset: 18, bit_width: 1, access: Some(ReadWrite), description: Some("Receiver Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Receiver disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Receiver enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TE", bit_offset: 19, bit_width: 1, access: Some(ReadWrite), description: Some("Transmitter Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Transmitter disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Transmitter enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ILIE", bit_offset: 20, bit_width: 1, access: Some(ReadWrite), description: Some("Idle Line Interrupt Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Hardware interrupts from IDLE disabled; use polling.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Hardware interrupt requested when IDLE flag is 1.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RIE", bit_offset: 21, bit_width: 1, access: Some(ReadWrite), description: Some("Receiver Interrupt Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Hardware interrupts from RDRF disabled; use polling.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Hardware interrupt requested when RDRF flag is 1.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TCIE", bit_offset: 22, bit_width: 1, access: Some(ReadWrite), description: Some("Transmission Complete Interrupt Enable for"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Hardware interrupts from TC disabled; use polling.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Hardware interrupt requested when TC flag is 1.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TIE", bit_offset: 23, bit_width: 1, access: Some(ReadWrite), description: Some("Transmit Interrupt Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Hardware interrupts from TDRE disabled; use polling.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Hardware interrupt requested when TDRE flag is 1.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "PEIE", bit_offset: 24, bit_width: 1, access: Some(ReadWrite), description: Some("Parity Error Interrupt Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("PF interrupts disabled; use polling).") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Hardware interrupt requested when PF is set.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "FEIE", bit_offset: 25, bit_width: 1, access: Some(ReadWrite), description: Some("Framing Error Interrupt Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("FE interrupts disabled; use polling.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Hardware interrupt requested when FE is set.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "NEIE", bit_offset: 26, bit_width: 1, access: Some(ReadWrite), description: Some("Noise Error Interrupt Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("NF interrupts disabled; use polling.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Hardware interrupt requested when NF is set.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ORIE", bit_offset: 27, bit_width: 1, access: Some(ReadWrite), description: Some("Overrun Interrupt Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("OR interrupts disabled; use polling.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Hardware interrupt requested when OR is set.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TXINV", bit_offset: 28, bit_width: 1, access: Some(ReadWrite), description: Some("Transmit Data Inversion"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Transmit data not inverted.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Transmit data inverted.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TXDIR", bit_offset: 29, bit_width: 1, access: Some(ReadWrite), description: Some("TXD Pin Direction in Single-Wire Mode"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("TXD pin is an input in single-wire mode.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("TXD pin is an output in single-wire mode.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "R9T8", bit_offset: 30, bit_width: 1, access: Some(ReadWrite), description: Some("Receive Bit 9 / Transmit Bit 8"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "R8T9", bit_offset: 31, bit_width: 1, access: Some(ReadWrite), description: Some("Receive Bit 8 / Transmit Bit 9"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "DATA", offset: 28, size: Some(32), access: Some(ReadWrite), reset_value: Some(4096), reset_mask: Some(4294967295), description: Some("LPUART Data Register"), fields: [Field { name: "RT", bit_offset: 0, bit_width: 10, access: Some(ReadWrite), description: Some("RT"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "IDLINE", bit_offset: 11, bit_width: 1, access: Some(ReadOnly), description: Some("Idle Line"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Receiver was not idle before receiving this character.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Receiver was idle before receiving this character.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RXEMPT", bit_offset: 12, bit_width: 1, access: Some(ReadOnly), description: Some("Receive Buffer Empty"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Receive buffer contains valid data.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Receive buffer is empty, data returned on read is not valid.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "FRETSC", bit_offset: 13, bit_width: 1, access: Some(ReadWrite), description: Some("Frame Error / Transmit Special Character"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The dataword was received without a frame error on read, or transmit a normal character on write.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The dataword was received with a frame error, or transmit an idle or break character on transmit.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "PARITYE", bit_offset: 14, bit_width: 1, access: Some(ReadOnly), description: Some("PARITYE"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The dataword was received without a parity error.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The dataword was received with a parity error.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "NOISY", bit_offset: 15, bit_width: 1, access: Some(ReadOnly), description: Some("NOISY"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The dataword was received without noise.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The data was received with noise.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "MATCH", offset: 32, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("LPUART Match Address Register"), fields: [Field { name: "MA1", bit_offset: 0, bit_width: 10, access: Some(ReadWrite), description: Some("Match Address 1"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MA2", bit_offset: 16, bit_width: 10, access: Some(ReadWrite), description: Some("Match Address 2"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "MODIR", offset: 36, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("LPUART Modem IrDA Register"), fields: [Field { name: "TXCTSE", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("Transmitter clear-to-send enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("CTS has no effect on the transmitter.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Enables clear-to-send operation. The transmitter checks the state of CTS each time it is ready to send a character. If CTS is asserted, the character is sent. If CTS is deasserted, the signal TXD remains in the mark state and transmission is delayed until CTS is asserted. Changes in CTS as a character is being sent do not affect its transmission.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TXRTSE", bit_offset: 1, bit_width: 1, access: Some(ReadWrite), description: Some("Transmitter request-to-send enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The transmitter has no effect on RTS.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("When a character is placed into an empty transmitter data buffer , RTS asserts one bit time before the start bit is transmitted. RTS deasserts one bit time after all characters in the transmitter data buffer and shift register are completely sent, including the last stop bit.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TXRTSPOL", bit_offset: 2, bit_width: 1, access: Some(ReadWrite), description: Some("Transmitter request-to-send polarity"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Transmitter RTS is active low.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Transmitter RTS is active high.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RXRTSE", bit_offset: 3, bit_width: 1, access: Some(ReadWrite), description: Some("Receiver request-to-send enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The receiver has no effect on RTS.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TXCTSC", bit_offset: 4, bit_width: 1, access: Some(ReadWrite), description: Some("Transmit CTS Configuration"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("CTS input is sampled at the start of each character.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("CTS input is sampled when the transmitter is idle.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TXCTSSRC", bit_offset: 5, bit_width: 1, access: Some(ReadWrite), description: Some("Transmit CTS Source"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("CTS input is the CTS_B pin.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("CTS input is the inverted Receiver Match result.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RTSWATER", bit_offset: 8, bit_width: 2, access: Some(ReadWrite), description: Some("Receive RTS Configuration"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TNP", bit_offset: 16, bit_width: 2, access: Some(ReadWrite), description: Some("Transmitter narrow pulse"), enumerated_values: [EnumeratedValue { value: "#00", name: Some("00"), description: Some("1/OSR.") }, EnumeratedValue { value: "#01", name: Some("01"), description: Some("2/OSR.") }, EnumeratedValue { value: "#10", name: Some("10"), description: Some("3/OSR.") }, EnumeratedValue { value: "#11", name: Some("11"), description: Some("4/OSR.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "IREN", bit_offset: 18, bit_width: 1, access: Some(ReadWrite), description: Some("Infrared enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("IR disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("IR enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "FIFO", offset: 40, size: Some(32), access: Some(ReadWrite), reset_value: Some(12582929), reset_mask: Some(4294967295), description: Some("LPUART FIFO Register"), fields: [Field { name: "RXFIFOSIZE", bit_offset: 0, bit_width: 3, access: Some(ReadOnly), description: Some("Receive FIFO. Buffer Depth"), enumerated_values: [EnumeratedValue { value: "#000", name: Some("000"), description: Some("Receive FIFO/Buffer depth = 1 dataword.") }, EnumeratedValue { value: "#001", name: Some("001"), description: Some("Receive FIFO/Buffer depth = 4 datawords.") }, EnumeratedValue { value: "#010", name: Some("010"), description: Some("Receive FIFO/Buffer depth = 8 datawords.") }, EnumeratedValue { value: "#011", name: Some("011"), description: Some("Receive FIFO/Buffer depth = 16 datawords.") }, EnumeratedValue { value: "#100", name: Some("100"), description: Some("Receive FIFO/Buffer depth = 32 datawords.") }, EnumeratedValue { value: "#101", name: Some("101"), description: Some("Receive FIFO/Buffer depth = 64 datawords.") }, EnumeratedValue { value: "#110", name: Some("110"), description: Some("Receive FIFO/Buffer depth = 128 datawords.") }, EnumeratedValue { value: "#111", name: Some("111"), description: Some("Receive FIFO/Buffer depth = 256 datawords.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RXFE", bit_offset: 3, bit_width: 1, access: Some(ReadWrite), description: Some("Receive FIFO Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Receive FIFO is not enabled. Buffer is depth 1. (Legacy support)") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Receive FIFO is enabled. Buffer is depth indicted by RXFIFOSIZE.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TXFIFOSIZE", bit_offset: 4, bit_width: 3, access: Some(ReadOnly), description: Some("Transmit FIFO. Buffer Depth"), enumerated_values: [EnumeratedValue { value: "#000", name: Some("000"), description: Some("Transmit FIFO/Buffer depth = 1 dataword.") }, EnumeratedValue { value: "#001", name: Some("001"), description: Some("Transmit FIFO/Buffer depth = 4 datawords.") }, EnumeratedValue { value: "#010", name: Some("010"), description: Some("Transmit FIFO/Buffer depth = 8 datawords.") }, EnumeratedValue { value: "#011", name: Some("011"), description: Some("Transmit FIFO/Buffer depth = 16 datawords.") }, EnumeratedValue { value: "#100", name: Some("100"), description: Some("Transmit FIFO/Buffer depth = 32 datawords.") }, EnumeratedValue { value: "#101", name: Some("101"), description: Some("Transmit FIFO/Buffer depth = 64 datawords.") }, EnumeratedValue { value: "#110", name: Some("110"), description: Some("Transmit FIFO/Buffer depth = 128 datawords.") }, EnumeratedValue { value: "#111", name: Some("111"), description: Some("Transmit FIFO/Buffer depth = 256 datawords") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TXFE", bit_offset: 7, bit_width: 1, access: Some(ReadWrite), description: Some("Transmit FIFO Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Transmit FIFO is not enabled. Buffer is depth 1. (Legacy support).") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Transmit FIFO is enabled. Buffer is depth indicated by TXFIFOSIZE.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RXUFE", bit_offset: 8, bit_width: 1, access: Some(ReadWrite), description: Some("Receive FIFO Underflow Interrupt Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("RXUF flag does not generate an interrupt to the host.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("RXUF flag generates an interrupt to the host.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TXOFE", bit_offset: 9, bit_width: 1, access: Some(ReadWrite), description: Some("Transmit FIFO Overflow Interrupt Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("TXOF flag does not generate an interrupt to the host.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("TXOF flag generates an interrupt to the host.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RXIDEN", bit_offset: 10, bit_width: 3, access: Some(ReadWrite), description: Some("Receiver Idle Empty Enable"), enumerated_values: [EnumeratedValue { value: "#000", name: Some("000"), description: Some("Disable RDRF assertion due to partially filled FIFO when receiver is idle.") }, EnumeratedValue { value: "#001", name: Some("001"), description: Some("Enable RDRF assertion due to partially filled FIFO when receiver is idle for 1 character.") }, EnumeratedValue { value: "#010", name: Some("010"), description: Some("Enable RDRF assertion due to partially filled FIFO when receiver is idle for 2 characters.") }, EnumeratedValue { value: "#011", name: Some("011"), description: Some("Enable RDRF assertion due to partially filled FIFO when receiver is idle for 4 characters.") }, EnumeratedValue { value: "#100", name: Some("100"), description: Some("Enable RDRF assertion due to partially filled FIFO when receiver is idle for 8 characters.") }, EnumeratedValue { value: "#101", name: Some("101"), description: Some("Enable RDRF assertion due to partially filled FIFO when receiver is idle for 16 characters.") }, EnumeratedValue { value: "#110", name: Some("110"), description: Some("Enable RDRF assertion due to partially filled FIFO when receiver is idle for 32 characters.") }, EnumeratedValue { value: "#111", name: Some("111"), description: Some("Enable RDRF assertion due to partially filled FIFO when receiver is idle for 64 characters.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RXFLUSH", bit_offset: 14, bit_width: 1, access: Some(WriteOnly), description: Some("Receive FIFO/Buffer Flush"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No flush operation occurs.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("All data in the receive FIFO/buffer is cleared out.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TXFLUSH", bit_offset: 15, bit_width: 1, access: Some(WriteOnly), description: Some("Transmit FIFO/Buffer Flush"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No flush operation occurs.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("All data in the transmit FIFO/Buffer is cleared out.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RXUF", bit_offset: 16, bit_width: 1, access: Some(ReadWrite), description: Some("Receiver Buffer Underflow Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No receive buffer underflow has occurred since the last time the flag was cleared.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("At least one receive buffer underflow has occurred since the last time the flag was cleared.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TXOF", bit_offset: 17, bit_width: 1, access: Some(ReadWrite), description: Some("Transmitter Buffer Overflow Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No transmit buffer overflow has occurred since the last time the flag was cleared.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("At least one transmit buffer overflow has occurred since the last time the flag was cleared.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RXEMPT", bit_offset: 22, bit_width: 1, access: Some(ReadOnly), description: Some("Receive Buffer/FIFO Empty"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Receive buffer is not empty.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Receive buffer is empty.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TXEMPT", bit_offset: 23, bit_width: 1, access: Some(ReadOnly), description: Some("Transmit Buffer/FIFO Empty"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Transmit buffer is not empty.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Transmit buffer is empty.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "WATER", offset: 44, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("LPUART Watermark Register"), fields: [Field { name: "TXWATER", bit_offset: 0, bit_width: 2, access: Some(ReadWrite), description: Some("Transmit Watermark"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TXCOUNT", bit_offset: 8, bit_width: 3, access: Some(ReadOnly), description: Some("Transmit Counter"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RXWATER", bit_offset: 16, bit_width: 2, access: Some(ReadWrite), description: Some("Receive Watermark"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RXCOUNT", bit_offset: 24, bit_width: 3, access: Some(ReadOnly), description: Some("Receive Counter"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }), modules: [], has_pins: false, has_channels: false, description: None }

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="LPUART Peripheral"]
pub struct LpuartPeriph(pub usize); 


impl LpuartPeriph {
#[doc="Get the *const pointer for the VERID register."]
   #[inline] pub fn verid_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x0) as *const u32
   }
#[doc="Get the *mut pointer for the VERID register."]
   #[inline] pub fn verid_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x0) as *mut u32
   }
#[doc="Read the VERID register."]
   #[inline] pub fn verid(&self) -> Verid { 
      unsafe {
         Verid(::core::ptr::read_volatile((self.0 + 0x0) as *const u32))
      }
   }

#[doc="Get the *const pointer for the PARAM register."]
   #[inline] pub fn param_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x4) as *const u32
   }
#[doc="Get the *mut pointer for the PARAM register."]
   #[inline] pub fn param_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x4) as *mut u32
   }
#[doc="Read the PARAM register."]
   #[inline] pub fn param(&self) -> Param { 
      unsafe {
         Param(::core::ptr::read_volatile((self.0 + 0x4) as *const u32))
      }
   }

#[doc="Get the *const pointer for the GLOBAL register."]
   #[inline] pub fn global_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x8) as *const u32
   }
#[doc="Get the *mut pointer for the GLOBAL register."]
   #[inline] pub fn global_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x8) as *mut u32
   }
#[doc="Read the GLOBAL register."]
   #[inline] pub fn global(&self) -> Global { 
      unsafe {
         Global(::core::ptr::read_volatile((self.0 + 0x8) as *const u32))
      }
   }
#[doc="Write the GLOBAL register."]
   #[inline] pub fn set_global<F: FnOnce(Global) -> Global>(&self, f: F) -> &Self {
      let value = f(Global(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x8) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the GLOBAL register."]
   #[inline] pub fn with_global<F: FnOnce(Global) -> Global>(&self, f: F) -> &Self {
      let tmp = self.global();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x8) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PINCFG register."]
   #[inline] pub fn pincfg_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xc) as *const u32
   }
#[doc="Get the *mut pointer for the PINCFG register."]
   #[inline] pub fn pincfg_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xc) as *mut u32
   }
#[doc="Read the PINCFG register."]
   #[inline] pub fn pincfg(&self) -> Pincfg { 
      unsafe {
         Pincfg(::core::ptr::read_volatile((self.0 + 0xc) as *const u32))
      }
   }
#[doc="Write the PINCFG register."]
   #[inline] pub fn set_pincfg<F: FnOnce(Pincfg) -> Pincfg>(&self, f: F) -> &Self {
      let value = f(Pincfg(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xc) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PINCFG register."]
   #[inline] pub fn with_pincfg<F: FnOnce(Pincfg) -> Pincfg>(&self, f: F) -> &Self {
      let tmp = self.pincfg();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xc) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the BAUD register."]
   #[inline] pub fn baud_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x10) as *const u32
   }
#[doc="Get the *mut pointer for the BAUD register."]
   #[inline] pub fn baud_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x10) as *mut u32
   }
#[doc="Read the BAUD register."]
   #[inline] pub fn baud(&self) -> Baud { 
      unsafe {
         Baud(::core::ptr::read_volatile((self.0 + 0x10) as *const u32))
      }
   }
#[doc="Write the BAUD register."]
   #[inline] pub fn set_baud<F: FnOnce(Baud) -> Baud>(&self, f: F) -> &Self {
      let value = f(Baud(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x10) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the BAUD register."]
   #[inline] pub fn with_baud<F: FnOnce(Baud) -> Baud>(&self, f: F) -> &Self {
      let tmp = self.baud();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x10) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the STAT register."]
   #[inline] pub fn stat_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x14) as *const u32
   }
#[doc="Get the *mut pointer for the STAT register."]
   #[inline] pub fn stat_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x14) as *mut u32
   }
#[doc="Read the STAT register."]
   #[inline] pub fn stat(&self) -> Stat { 
      unsafe {
         Stat(::core::ptr::read_volatile((self.0 + 0x14) as *const u32))
      }
   }
#[doc="Write the STAT register."]
   #[inline] pub fn set_stat<F: FnOnce(Stat) -> Stat>(&self, f: F) -> &Self {
      let value = f(Stat(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x14) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the STAT register."]
   #[inline] pub fn with_stat<F: FnOnce(Stat) -> Stat>(&self, f: F) -> &Self {
      let tmp = self.stat();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x14) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CTRL register."]
   #[inline] pub fn ctrl_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x18) as *const u32
   }
#[doc="Get the *mut pointer for the CTRL register."]
   #[inline] pub fn ctrl_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x18) as *mut u32
   }
#[doc="Read the CTRL register."]
   #[inline] pub fn ctrl(&self) -> Ctrl { 
      unsafe {
         Ctrl(::core::ptr::read_volatile((self.0 + 0x18) as *const u32))
      }
   }
#[doc="Write the CTRL register."]
   #[inline] pub fn set_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
      let value = f(Ctrl(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x18) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CTRL register."]
   #[inline] pub fn with_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
      let tmp = self.ctrl();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x18) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DATA register."]
   #[inline] pub fn data_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x1c) as *const u32
   }
#[doc="Get the *mut pointer for the DATA register."]
   #[inline] pub fn data_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x1c) as *mut u32
   }
#[doc="Read the DATA register."]
   #[inline] pub fn data(&self) -> Data { 
      unsafe {
         Data(::core::ptr::read_volatile((self.0 + 0x1c) as *const u32))
      }
   }
#[doc="Write the DATA register."]
   #[inline] pub fn set_data<F: FnOnce(Data) -> Data>(&self, f: F) -> &Self {
      let value = f(Data(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DATA register."]
   #[inline] pub fn with_data<F: FnOnce(Data) -> Data>(&self, f: F) -> &Self {
      let tmp = self.data();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the MATCH register."]
   #[inline] pub fn match_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x20) as *const u32
   }
#[doc="Get the *mut pointer for the MATCH register."]
   #[inline] pub fn match_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x20) as *mut u32
   }
#[doc="Read the MATCH register."]
   #[inline] pub fn _match(&self) -> Match { 
      unsafe {
         Match(::core::ptr::read_volatile((self.0 + 0x20) as *const u32))
      }
   }
#[doc="Write the MATCH register."]
   #[inline] pub fn set_match<F: FnOnce(Match) -> Match>(&self, f: F) -> &Self {
      let value = f(Match(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x20) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the MATCH register."]
   #[inline] pub fn with_match<F: FnOnce(Match) -> Match>(&self, f: F) -> &Self {
      let tmp = self._match();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x20) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the MODIR register."]
   #[inline] pub fn modir_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x24) as *const u32
   }
#[doc="Get the *mut pointer for the MODIR register."]
   #[inline] pub fn modir_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x24) as *mut u32
   }
#[doc="Read the MODIR register."]
   #[inline] pub fn modir(&self) -> Modir { 
      unsafe {
         Modir(::core::ptr::read_volatile((self.0 + 0x24) as *const u32))
      }
   }
#[doc="Write the MODIR register."]
   #[inline] pub fn set_modir<F: FnOnce(Modir) -> Modir>(&self, f: F) -> &Self {
      let value = f(Modir(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x24) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the MODIR register."]
   #[inline] pub fn with_modir<F: FnOnce(Modir) -> Modir>(&self, f: F) -> &Self {
      let tmp = self.modir();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x24) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the FIFO register."]
   #[inline] pub fn fifo_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x28) as *const u32
   }
#[doc="Get the *mut pointer for the FIFO register."]
   #[inline] pub fn fifo_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x28) as *mut u32
   }
#[doc="Read the FIFO register."]
   #[inline] pub fn fifo(&self) -> Fifo { 
      unsafe {
         Fifo(::core::ptr::read_volatile((self.0 + 0x28) as *const u32))
      }
   }
#[doc="Write the FIFO register."]
   #[inline] pub fn set_fifo<F: FnOnce(Fifo) -> Fifo>(&self, f: F) -> &Self {
      let value = f(Fifo(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x28) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the FIFO register."]
   #[inline] pub fn with_fifo<F: FnOnce(Fifo) -> Fifo>(&self, f: F) -> &Self {
      let tmp = self.fifo();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x28) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the WATER register."]
   #[inline] pub fn water_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x2c) as *const u32
   }
#[doc="Get the *mut pointer for the WATER register."]
   #[inline] pub fn water_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x2c) as *mut u32
   }
#[doc="Read the WATER register."]
   #[inline] pub fn water(&self) -> Water { 
      unsafe {
         Water(::core::ptr::read_volatile((self.0 + 0x2c) as *const u32))
      }
   }
#[doc="Write the WATER register."]
   #[inline] pub fn set_water<F: FnOnce(Water) -> Water>(&self, f: F) -> &Self {
      let value = f(Water(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x2c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the WATER register."]
   #[inline] pub fn with_water<F: FnOnce(Water) -> Water>(&self, f: F) -> &Self {
      let tmp = self.water();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x2c) as *mut u32, value.0);
      }
      self
   }

}

#[doc="Version ID Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Verid(pub u32);
impl Verid {
#[doc="Feature Identification Number"]
   #[inline] pub fn feature(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Feature Identification Number"]
   #[inline] pub fn set_feature<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Minor Version Number"]
   #[inline] pub fn minor(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
   }
#[doc="Minor Version Number"]
   #[inline] pub fn set_minor<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Major Version Number"]
   #[inline] pub fn major(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
   }
#[doc="Major Version Number"]
   #[inline] pub fn set_major<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 24);
      self.0 |= value << 24;
      self
   }

}
impl ::core::fmt::Display for Verid {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Verid {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.feature() != 0 { try!(write!(f, " feature=0x{:x}", self.feature()))}
      if self.minor() != 0 { try!(write!(f, " minor=0x{:x}", self.minor()))}
      if self.major() != 0 { try!(write!(f, " major=0x{:x}", self.major()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Parameter Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Param(pub u32);
impl Param {
#[doc="Transmit FIFO Size"]
   #[inline] pub fn txfifo(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Transmit FIFO Size"]
   #[inline] pub fn set_txfifo<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Receive FIFO Size"]
   #[inline] pub fn rxfifo(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
   }
#[doc="Receive FIFO Size"]
   #[inline] pub fn set_rxfifo<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 8);
      self.0 |= value << 8;
      self
   }

}
impl ::core::fmt::Display for Param {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Param {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.txfifo() != 0 { try!(write!(f, " txfifo=0x{:x}", self.txfifo()))}
      if self.rxfifo() != 0 { try!(write!(f, " rxfifo=0x{:x}", self.rxfifo()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="LPUART Global Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Global(pub u32);
impl Global {
#[doc="Software Reset"]
   #[inline] pub fn rst(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Software Reset"]
   #[inline] pub fn set_rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

}
impl ::core::fmt::Display for Global {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Global {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rst() != 0 { try!(write!(f, " rst"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="LPUART Pin Configuration Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pincfg(pub u32);
impl Pincfg {
#[doc="Trigger Select"]
   #[inline] pub fn trgsel(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
   }
#[doc="Trigger Select"]
   #[inline] pub fn set_trgsel<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Pincfg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pincfg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.trgsel() != 0 { try!(write!(f, " trgsel=0x{:x}", self.trgsel()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="LPUART Baud Rate Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Baud(pub u32);
impl Baud {
#[doc="Baud Rate Modulo Divisor."]
   #[inline] pub fn sbr(&self) -> bits::U13 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1fff) as u16) } // [12:0]
   }
#[doc="Baud Rate Modulo Divisor."]
   #[inline] pub fn set_sbr<V: Into<bits::U13>>(mut self, value: V) -> Self {
      let value: bits::U13 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1fff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Stop Bit Number Select"]
   #[inline] pub fn sbns(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
   }
#[doc="Stop Bit Number Select"]
   #[inline] pub fn set_sbns<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 13);
      self.0 |= value << 13;
      self
   }

#[doc="RX Input Active Edge Interrupt Enable"]
   #[inline] pub fn rxedgie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
   }
#[doc="RX Input Active Edge Interrupt Enable"]
   #[inline] pub fn set_rxedgie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 14);
      self.0 |= value << 14;
      self
   }

#[doc="LIN Break Detect Interrupt Enable"]
   #[inline] pub fn lbkdie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
   }
#[doc="LIN Break Detect Interrupt Enable"]
   #[inline] pub fn set_lbkdie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 15);
      self.0 |= value << 15;
      self
   }

#[doc="Resynchronization Disable"]
   #[inline] pub fn resyncdis(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
   }
#[doc="Resynchronization Disable"]
   #[inline] pub fn set_resyncdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Both Edge Sampling"]
   #[inline] pub fn bothedge(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
   }
#[doc="Both Edge Sampling"]
   #[inline] pub fn set_bothedge<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 17);
      self.0 |= value << 17;
      self
   }

#[doc="Match Configuration"]
   #[inline] pub fn matcfg(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x3) as u8) } // [19:18]
   }
#[doc="Match Configuration"]
   #[inline] pub fn set_matcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 18);
      self.0 |= value << 18;
      self
   }

#[doc="Receiver Idle DMA Enable"]
   #[inline] pub fn ridmae(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
   }
#[doc="Receiver Idle DMA Enable"]
   #[inline] pub fn set_ridmae<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 20);
      self.0 |= value << 20;
      self
   }

#[doc="Receiver Full DMA Enable"]
   #[inline] pub fn rdmae(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
   }
#[doc="Receiver Full DMA Enable"]
   #[inline] pub fn set_rdmae<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 21);
      self.0 |= value << 21;
      self
   }

#[doc="Transmitter DMA Enable"]
   #[inline] pub fn tdmae(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
   }
#[doc="Transmitter DMA Enable"]
   #[inline] pub fn set_tdmae<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 23);
      self.0 |= value << 23;
      self
   }

#[doc="Oversampling Ratio"]
   #[inline] pub fn osr(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1f) as u8) } // [28:24]
   }
#[doc="Oversampling Ratio"]
   #[inline] pub fn set_osr<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 24);
      self.0 |= value << 24;
      self
   }

#[doc="10-bit Mode select"]
   #[inline] pub fn m10(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
   }
#[doc="10-bit Mode select"]
   #[inline] pub fn set_m10<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 29);
      self.0 |= value << 29;
      self
   }

#[doc="Match Address Mode Enable 2"]
   #[inline] pub fn maen2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Match Address Mode Enable 2"]
   #[inline] pub fn set_maen2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Match Address Mode Enable 1"]
   #[inline] pub fn maen1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Match Address Mode Enable 1"]
   #[inline] pub fn set_maen1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 31);
      self.0 |= value << 31;
      self
   }

}
impl ::core::fmt::Display for Baud {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Baud {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sbr() != 0 { try!(write!(f, " sbr=0x{:x}", self.sbr()))}
      if self.sbns() != 0 { try!(write!(f, " sbns"))}
      if self.rxedgie() != 0 { try!(write!(f, " rxedgie"))}
      if self.lbkdie() != 0 { try!(write!(f, " lbkdie"))}
      if self.resyncdis() != 0 { try!(write!(f, " resyncdis"))}
      if self.bothedge() != 0 { try!(write!(f, " bothedge"))}
      if self.matcfg() != 0 { try!(write!(f, " matcfg=0x{:x}", self.matcfg()))}
      if self.ridmae() != 0 { try!(write!(f, " ridmae"))}
      if self.rdmae() != 0 { try!(write!(f, " rdmae"))}
      if self.tdmae() != 0 { try!(write!(f, " tdmae"))}
      if self.osr() != 0 { try!(write!(f, " osr=0x{:x}", self.osr()))}
      if self.m10() != 0 { try!(write!(f, " m10"))}
      if self.maen2() != 0 { try!(write!(f, " maen2"))}
      if self.maen1() != 0 { try!(write!(f, " maen1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="LPUART Status Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Stat(pub u32);
impl Stat {
#[doc="Match 2 Flag"]
   #[inline] pub fn ma2f(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
   }
#[doc="Match 2 Flag"]
   #[inline] pub fn set_ma2f<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 14);
      self.0 |= value << 14;
      self
   }

#[doc="Match 1 Flag"]
   #[inline] pub fn ma1f(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
   }
#[doc="Match 1 Flag"]
   #[inline] pub fn set_ma1f<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 15);
      self.0 |= value << 15;
      self
   }

#[doc="Parity Error Flag"]
   #[inline] pub fn pf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
   }
#[doc="Parity Error Flag"]
   #[inline] pub fn set_pf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Framing Error Flag"]
   #[inline] pub fn fe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
   }
#[doc="Framing Error Flag"]
   #[inline] pub fn set_fe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 17);
      self.0 |= value << 17;
      self
   }

#[doc="Noise Flag"]
   #[inline] pub fn nf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
   }
#[doc="Noise Flag"]
   #[inline] pub fn set_nf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 18);
      self.0 |= value << 18;
      self
   }

#[doc="Receiver Overrun Flag"]
   #[inline] pub fn or(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
   }
#[doc="Receiver Overrun Flag"]
   #[inline] pub fn set_or<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 19);
      self.0 |= value << 19;
      self
   }

#[doc="Idle Line Flag"]
   #[inline] pub fn idle(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
   }
#[doc="Idle Line Flag"]
   #[inline] pub fn set_idle<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 20);
      self.0 |= value << 20;
      self
   }

#[doc="Receive Data Register Full Flag"]
   #[inline] pub fn rdrf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
   }
#[doc="Receive Data Register Full Flag"]
   #[inline] pub fn set_rdrf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 21);
      self.0 |= value << 21;
      self
   }

#[doc="Transmission Complete Flag"]
   #[inline] pub fn tc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
   }
#[doc="Transmission Complete Flag"]
   #[inline] pub fn set_tc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 22);
      self.0 |= value << 22;
      self
   }

#[doc="Transmit Data Register Empty Flag"]
   #[inline] pub fn tdre(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
   }
#[doc="Transmit Data Register Empty Flag"]
   #[inline] pub fn set_tdre<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 23);
      self.0 |= value << 23;
      self
   }

#[doc="Receiver Active Flag"]
   #[inline] pub fn raf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
   }
#[doc="Receiver Active Flag"]
   #[inline] pub fn set_raf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 24);
      self.0 |= value << 24;
      self
   }

#[doc="LIN Break Detection Enable"]
   #[inline] pub fn lbkde(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
   }
#[doc="LIN Break Detection Enable"]
   #[inline] pub fn set_lbkde<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 25);
      self.0 |= value << 25;
      self
   }

#[doc="Break Character Generation Length"]
   #[inline] pub fn brk13(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
   }
#[doc="Break Character Generation Length"]
   #[inline] pub fn set_brk13<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 26);
      self.0 |= value << 26;
      self
   }

#[doc="Receive Wake Up Idle Detect"]
   #[inline] pub fn rwuid(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
   }
#[doc="Receive Wake Up Idle Detect"]
   #[inline] pub fn set_rwuid<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 27);
      self.0 |= value << 27;
      self
   }

#[doc="Receive Data Inversion"]
   #[inline] pub fn rxinv(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
   }
#[doc="Receive Data Inversion"]
   #[inline] pub fn set_rxinv<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 28);
      self.0 |= value << 28;
      self
   }

#[doc="MSB First"]
   #[inline] pub fn msbf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
   }
#[doc="MSB First"]
   #[inline] pub fn set_msbf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 29);
      self.0 |= value << 29;
      self
   }

#[doc="RXD Pin Active Edge Interrupt Flag"]
   #[inline] pub fn rxedgif(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="RXD Pin Active Edge Interrupt Flag"]
   #[inline] pub fn set_rxedgif<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="LIN Break Detect Interrupt Flag"]
   #[inline] pub fn lbkdif(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="LIN Break Detect Interrupt Flag"]
   #[inline] pub fn set_lbkdif<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 31);
      self.0 |= value << 31;
      self
   }

}
impl ::core::fmt::Display for Stat {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Stat {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ma2f() != 0 { try!(write!(f, " ma2f"))}
      if self.ma1f() != 0 { try!(write!(f, " ma1f"))}
      if self.pf() != 0 { try!(write!(f, " pf"))}
      if self.fe() != 0 { try!(write!(f, " fe"))}
      if self.nf() != 0 { try!(write!(f, " nf"))}
      if self.or() != 0 { try!(write!(f, " or"))}
      if self.idle() != 0 { try!(write!(f, " idle"))}
      if self.rdrf() != 0 { try!(write!(f, " rdrf"))}
      if self.tc() != 0 { try!(write!(f, " tc"))}
      if self.tdre() != 0 { try!(write!(f, " tdre"))}
      if self.raf() != 0 { try!(write!(f, " raf"))}
      if self.lbkde() != 0 { try!(write!(f, " lbkde"))}
      if self.brk13() != 0 { try!(write!(f, " brk13"))}
      if self.rwuid() != 0 { try!(write!(f, " rwuid"))}
      if self.rxinv() != 0 { try!(write!(f, " rxinv"))}
      if self.msbf() != 0 { try!(write!(f, " msbf"))}
      if self.rxedgif() != 0 { try!(write!(f, " rxedgif"))}
      if self.lbkdif() != 0 { try!(write!(f, " lbkdif"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="LPUART Control Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ctrl(pub u32);
impl Ctrl {
#[doc="Parity Type"]
   #[inline] pub fn pt(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Parity Type"]
   #[inline] pub fn set_pt<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Parity Enable"]
   #[inline] pub fn pe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Parity Enable"]
   #[inline] pub fn set_pe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Idle Line Type Select"]
   #[inline] pub fn ilt(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Idle Line Type Select"]
   #[inline] pub fn set_ilt<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Receiver Wakeup Method Select"]
   #[inline] pub fn wake(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Receiver Wakeup Method Select"]
   #[inline] pub fn set_wake<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="9-Bit or 8-Bit Mode Select"]
   #[inline] pub fn m(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="9-Bit or 8-Bit Mode Select"]
   #[inline] pub fn set_m<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Receiver Source Select"]
   #[inline] pub fn rsrc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Receiver Source Select"]
   #[inline] pub fn set_rsrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Doze Enable"]
   #[inline] pub fn dozeen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Doze Enable"]
   #[inline] pub fn set_dozeen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Loop Mode Select"]
   #[inline] pub fn loops(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Loop Mode Select"]
   #[inline] pub fn set_loops<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="Idle Configuration"]
   #[inline] pub fn idlecfg(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
   }
#[doc="Idle Configuration"]
   #[inline] pub fn set_idlecfg<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="7-Bit Mode Select"]
   #[inline] pub fn m7(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
   }
#[doc="7-Bit Mode Select"]
   #[inline] pub fn set_m7<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 11);
      self.0 |= value << 11;
      self
   }

#[doc="Match 2 Interrupt Enable"]
   #[inline] pub fn ma2ie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
   }
#[doc="Match 2 Interrupt Enable"]
   #[inline] pub fn set_ma2ie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 14);
      self.0 |= value << 14;
      self
   }

#[doc="Match 1 Interrupt Enable"]
   #[inline] pub fn ma1ie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
   }
#[doc="Match 1 Interrupt Enable"]
   #[inline] pub fn set_ma1ie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 15);
      self.0 |= value << 15;
      self
   }

#[doc="Send Break"]
   #[inline] pub fn sbk(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
   }
#[doc="Send Break"]
   #[inline] pub fn set_sbk<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Receiver Wakeup Control"]
   #[inline] pub fn rwu(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
   }
#[doc="Receiver Wakeup Control"]
   #[inline] pub fn set_rwu<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 17);
      self.0 |= value << 17;
      self
   }

#[doc="Receiver Enable"]
   #[inline] pub fn re(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
   }
#[doc="Receiver Enable"]
   #[inline] pub fn set_re<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 18);
      self.0 |= value << 18;
      self
   }

#[doc="Transmitter Enable"]
   #[inline] pub fn te(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
   }
#[doc="Transmitter Enable"]
   #[inline] pub fn set_te<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 19);
      self.0 |= value << 19;
      self
   }

#[doc="Idle Line Interrupt Enable"]
   #[inline] pub fn ilie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
   }
#[doc="Idle Line Interrupt Enable"]
   #[inline] pub fn set_ilie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 20);
      self.0 |= value << 20;
      self
   }

#[doc="Receiver Interrupt Enable"]
   #[inline] pub fn rie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
   }
#[doc="Receiver Interrupt Enable"]
   #[inline] pub fn set_rie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 21);
      self.0 |= value << 21;
      self
   }

#[doc="Transmission Complete Interrupt Enable for"]
   #[inline] pub fn tcie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
   }
#[doc="Transmission Complete Interrupt Enable for"]
   #[inline] pub fn set_tcie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 22);
      self.0 |= value << 22;
      self
   }

#[doc="Transmit Interrupt Enable"]
   #[inline] pub fn tie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
   }
#[doc="Transmit Interrupt Enable"]
   #[inline] pub fn set_tie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 23);
      self.0 |= value << 23;
      self
   }

#[doc="Parity Error Interrupt Enable"]
   #[inline] pub fn peie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
   }
#[doc="Parity Error Interrupt Enable"]
   #[inline] pub fn set_peie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 24);
      self.0 |= value << 24;
      self
   }

#[doc="Framing Error Interrupt Enable"]
   #[inline] pub fn feie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
   }
#[doc="Framing Error Interrupt Enable"]
   #[inline] pub fn set_feie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 25);
      self.0 |= value << 25;
      self
   }

#[doc="Noise Error Interrupt Enable"]
   #[inline] pub fn neie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
   }
#[doc="Noise Error Interrupt Enable"]
   #[inline] pub fn set_neie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 26);
      self.0 |= value << 26;
      self
   }

#[doc="Overrun Interrupt Enable"]
   #[inline] pub fn orie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
   }
#[doc="Overrun Interrupt Enable"]
   #[inline] pub fn set_orie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 27);
      self.0 |= value << 27;
      self
   }

#[doc="Transmit Data Inversion"]
   #[inline] pub fn txinv(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
   }
#[doc="Transmit Data Inversion"]
   #[inline] pub fn set_txinv<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 28);
      self.0 |= value << 28;
      self
   }

#[doc="TXD Pin Direction in Single-Wire Mode"]
   #[inline] pub fn txdir(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
   }
#[doc="TXD Pin Direction in Single-Wire Mode"]
   #[inline] pub fn set_txdir<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 29);
      self.0 |= value << 29;
      self
   }

#[doc="Receive Bit 9 / Transmit Bit 8"]
   #[inline] pub fn r9t8(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Receive Bit 9 / Transmit Bit 8"]
   #[inline] pub fn set_r9t8<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Receive Bit 8 / Transmit Bit 9"]
   #[inline] pub fn r8t9(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Receive Bit 8 / Transmit Bit 9"]
   #[inline] pub fn set_r8t9<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 31);
      self.0 |= value << 31;
      self
   }

}
impl ::core::fmt::Display for Ctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pt() != 0 { try!(write!(f, " pt"))}
      if self.pe() != 0 { try!(write!(f, " pe"))}
      if self.ilt() != 0 { try!(write!(f, " ilt"))}
      if self.wake() != 0 { try!(write!(f, " wake"))}
      if self.m() != 0 { try!(write!(f, " m"))}
      if self.rsrc() != 0 { try!(write!(f, " rsrc"))}
      if self.dozeen() != 0 { try!(write!(f, " dozeen"))}
      if self.loops() != 0 { try!(write!(f, " loops"))}
      if self.idlecfg() != 0 { try!(write!(f, " idlecfg=0x{:x}", self.idlecfg()))}
      if self.m7() != 0 { try!(write!(f, " m7"))}
      if self.ma2ie() != 0 { try!(write!(f, " ma2ie"))}
      if self.ma1ie() != 0 { try!(write!(f, " ma1ie"))}
      if self.sbk() != 0 { try!(write!(f, " sbk"))}
      if self.rwu() != 0 { try!(write!(f, " rwu"))}
      if self.re() != 0 { try!(write!(f, " re"))}
      if self.te() != 0 { try!(write!(f, " te"))}
      if self.ilie() != 0 { try!(write!(f, " ilie"))}
      if self.rie() != 0 { try!(write!(f, " rie"))}
      if self.tcie() != 0 { try!(write!(f, " tcie"))}
      if self.tie() != 0 { try!(write!(f, " tie"))}
      if self.peie() != 0 { try!(write!(f, " peie"))}
      if self.feie() != 0 { try!(write!(f, " feie"))}
      if self.neie() != 0 { try!(write!(f, " neie"))}
      if self.orie() != 0 { try!(write!(f, " orie"))}
      if self.txinv() != 0 { try!(write!(f, " txinv"))}
      if self.txdir() != 0 { try!(write!(f, " txdir"))}
      if self.r9t8() != 0 { try!(write!(f, " r9t8"))}
      if self.r8t9() != 0 { try!(write!(f, " r8t9"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="LPUART Data Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Data(pub u32);
impl Data {
#[doc="RT"]
   #[inline] pub fn rt(&self) -> bits::U10 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
   }
#[doc="RT"]
   #[inline] pub fn set_rt<V: Into<bits::U10>>(mut self, value: V) -> Self {
      let value: bits::U10 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3ff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Idle Line"]
   #[inline] pub fn idline(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
   }
#[doc="Idle Line"]
   #[inline] pub fn set_idline<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 11);
      self.0 |= value << 11;
      self
   }

#[doc="Receive Buffer Empty"]
   #[inline] pub fn rxempt(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
   }
#[doc="Receive Buffer Empty"]
   #[inline] pub fn set_rxempt<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 12);
      self.0 |= value << 12;
      self
   }

#[doc="Frame Error / Transmit Special Character"]
   #[inline] pub fn fretsc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
   }
#[doc="Frame Error / Transmit Special Character"]
   #[inline] pub fn set_fretsc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 13);
      self.0 |= value << 13;
      self
   }

#[doc="PARITYE"]
   #[inline] pub fn paritye(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
   }
#[doc="PARITYE"]
   #[inline] pub fn set_paritye<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 14);
      self.0 |= value << 14;
      self
   }

#[doc="NOISY"]
   #[inline] pub fn noisy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
   }
#[doc="NOISY"]
   #[inline] pub fn set_noisy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 15);
      self.0 |= value << 15;
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
      if self.rt() != 0 { try!(write!(f, " rt=0x{:x}", self.rt()))}
      if self.idline() != 0 { try!(write!(f, " idline"))}
      if self.rxempt() != 0 { try!(write!(f, " rxempt"))}
      if self.fretsc() != 0 { try!(write!(f, " fretsc"))}
      if self.paritye() != 0 { try!(write!(f, " paritye"))}
      if self.noisy() != 0 { try!(write!(f, " noisy"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="LPUART Match Address Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Match(pub u32);
impl Match {
#[doc="Match Address 1"]
   #[inline] pub fn ma1(&self) -> bits::U10 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
   }
#[doc="Match Address 1"]
   #[inline] pub fn set_ma1<V: Into<bits::U10>>(mut self, value: V) -> Self {
      let value: bits::U10 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3ff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Match Address 2"]
   #[inline] pub fn ma2(&self) -> bits::U10 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3ff) as u16) } // [25:16]
   }
#[doc="Match Address 2"]
   #[inline] pub fn set_ma2<V: Into<bits::U10>>(mut self, value: V) -> Self {
      let value: bits::U10 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3ff << 16);
      self.0 |= value << 16;
      self
   }

}
impl ::core::fmt::Display for Match {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Match {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ma1() != 0 { try!(write!(f, " ma1=0x{:x}", self.ma1()))}
      if self.ma2() != 0 { try!(write!(f, " ma2=0x{:x}", self.ma2()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="LPUART Modem IrDA Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Modir(pub u32);
impl Modir {
#[doc="Transmitter clear-to-send enable"]
   #[inline] pub fn txctse(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Transmitter clear-to-send enable"]
   #[inline] pub fn set_txctse<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Transmitter request-to-send enable"]
   #[inline] pub fn txrtse(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Transmitter request-to-send enable"]
   #[inline] pub fn set_txrtse<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Transmitter request-to-send polarity"]
   #[inline] pub fn txrtspol(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Transmitter request-to-send polarity"]
   #[inline] pub fn set_txrtspol<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Receiver request-to-send enable"]
   #[inline] pub fn rxrtse(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Receiver request-to-send enable"]
   #[inline] pub fn set_rxrtse<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Transmit CTS Configuration"]
   #[inline] pub fn txctsc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Transmit CTS Configuration"]
   #[inline] pub fn set_txctsc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Transmit CTS Source"]
   #[inline] pub fn txctssrc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Transmit CTS Source"]
   #[inline] pub fn set_txctssrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Receive RTS Configuration"]
   #[inline] pub fn rtswater(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
   }
#[doc="Receive RTS Configuration"]
   #[inline] pub fn set_rtswater<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Transmitter narrow pulse"]
   #[inline] pub fn tnp(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
   }
#[doc="Transmitter narrow pulse"]
   #[inline] pub fn set_tnp<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Infrared enable"]
   #[inline] pub fn iren(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
   }
#[doc="Infrared enable"]
   #[inline] pub fn set_iren<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 18);
      self.0 |= value << 18;
      self
   }

}
impl ::core::fmt::Display for Modir {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Modir {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.txctse() != 0 { try!(write!(f, " txctse"))}
      if self.txrtse() != 0 { try!(write!(f, " txrtse"))}
      if self.txrtspol() != 0 { try!(write!(f, " txrtspol"))}
      if self.rxrtse() != 0 { try!(write!(f, " rxrtse"))}
      if self.txctsc() != 0 { try!(write!(f, " txctsc"))}
      if self.txctssrc() != 0 { try!(write!(f, " txctssrc"))}
      if self.rtswater() != 0 { try!(write!(f, " rtswater=0x{:x}", self.rtswater()))}
      if self.tnp() != 0 { try!(write!(f, " tnp=0x{:x}", self.tnp()))}
      if self.iren() != 0 { try!(write!(f, " iren"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="LPUART FIFO Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Fifo(pub u32);
impl Fifo {
#[doc="Receive FIFO. Buffer Depth"]
   #[inline] pub fn rxfifosize(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
   }
#[doc="Receive FIFO. Buffer Depth"]
   #[inline] pub fn set_rxfifosize<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Receive FIFO Enable"]
   #[inline] pub fn rxfe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Receive FIFO Enable"]
   #[inline] pub fn set_rxfe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Transmit FIFO. Buffer Depth"]
   #[inline] pub fn txfifosize(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
   }
#[doc="Transmit FIFO. Buffer Depth"]
   #[inline] pub fn set_txfifosize<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Transmit FIFO Enable"]
   #[inline] pub fn txfe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Transmit FIFO Enable"]
   #[inline] pub fn set_txfe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="Receive FIFO Underflow Interrupt Enable"]
   #[inline] pub fn rxufe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="Receive FIFO Underflow Interrupt Enable"]
   #[inline] pub fn set_rxufe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Transmit FIFO Overflow Interrupt Enable"]
   #[inline] pub fn txofe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="Transmit FIFO Overflow Interrupt Enable"]
   #[inline] pub fn set_txofe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

#[doc="Receiver Idle Empty Enable"]
   #[inline] pub fn rxiden(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x7) as u8) } // [12:10]
   }
#[doc="Receiver Idle Empty Enable"]
   #[inline] pub fn set_rxiden<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 10);
      self.0 |= value << 10;
      self
   }

#[doc="Receive FIFO/Buffer Flush"]
   #[inline] pub fn rxflush(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
   }
#[doc="Receive FIFO/Buffer Flush"]
   #[inline] pub fn set_rxflush<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 14);
      self.0 |= value << 14;
      self
   }

#[doc="Transmit FIFO/Buffer Flush"]
   #[inline] pub fn txflush(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
   }
#[doc="Transmit FIFO/Buffer Flush"]
   #[inline] pub fn set_txflush<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 15);
      self.0 |= value << 15;
      self
   }

#[doc="Receiver Buffer Underflow Flag"]
   #[inline] pub fn rxuf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
   }
#[doc="Receiver Buffer Underflow Flag"]
   #[inline] pub fn set_rxuf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Transmitter Buffer Overflow Flag"]
   #[inline] pub fn txof(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
   }
#[doc="Transmitter Buffer Overflow Flag"]
   #[inline] pub fn set_txof<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 17);
      self.0 |= value << 17;
      self
   }

#[doc="Receive Buffer/FIFO Empty"]
   #[inline] pub fn rxempt(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
   }
#[doc="Receive Buffer/FIFO Empty"]
   #[inline] pub fn set_rxempt<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 22);
      self.0 |= value << 22;
      self
   }

#[doc="Transmit Buffer/FIFO Empty"]
   #[inline] pub fn txempt(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
   }
#[doc="Transmit Buffer/FIFO Empty"]
   #[inline] pub fn set_txempt<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 23);
      self.0 |= value << 23;
      self
   }

}
impl ::core::fmt::Display for Fifo {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Fifo {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rxfifosize() != 0 { try!(write!(f, " rxfifosize=0x{:x}", self.rxfifosize()))}
      if self.rxfe() != 0 { try!(write!(f, " rxfe"))}
      if self.txfifosize() != 0 { try!(write!(f, " txfifosize=0x{:x}", self.txfifosize()))}
      if self.txfe() != 0 { try!(write!(f, " txfe"))}
      if self.rxufe() != 0 { try!(write!(f, " rxufe"))}
      if self.txofe() != 0 { try!(write!(f, " txofe"))}
      if self.rxiden() != 0 { try!(write!(f, " rxiden=0x{:x}", self.rxiden()))}
      if self.rxflush() != 0 { try!(write!(f, " rxflush"))}
      if self.txflush() != 0 { try!(write!(f, " txflush"))}
      if self.rxuf() != 0 { try!(write!(f, " rxuf"))}
      if self.txof() != 0 { try!(write!(f, " txof"))}
      if self.rxempt() != 0 { try!(write!(f, " rxempt"))}
      if self.txempt() != 0 { try!(write!(f, " txempt"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="LPUART Watermark Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Water(pub u32);
impl Water {
#[doc="Transmit Watermark"]
   #[inline] pub fn txwater(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
   }
#[doc="Transmit Watermark"]
   #[inline] pub fn set_txwater<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Transmit Counter"]
   #[inline] pub fn txcount(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
   }
#[doc="Transmit Counter"]
   #[inline] pub fn set_txcount<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Receive Watermark"]
   #[inline] pub fn rxwater(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
   }
#[doc="Receive Watermark"]
   #[inline] pub fn set_rxwater<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Receive Counter"]
   #[inline] pub fn rxcount(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
   }
#[doc="Receive Counter"]
   #[inline] pub fn set_rxcount<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 24);
      self.0 |= value << 24;
      self
   }

}
impl ::core::fmt::Display for Water {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Water {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.txwater() != 0 { try!(write!(f, " txwater=0x{:x}", self.txwater()))}
      if self.txcount() != 0 { try!(write!(f, " txcount=0x{:x}", self.txcount()))}
      if self.rxwater() != 0 { try!(write!(f, " rxwater=0x{:x}", self.rxwater()))}
      if self.rxcount() != 0 { try!(write!(f, " rxcount=0x{:x}", self.rxcount()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
