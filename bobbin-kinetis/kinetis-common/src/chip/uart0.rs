#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "UART0", peripherals: [], prototype: Some(Peripheral { derived_from: None, group_name: Some("UART0"), name: "", address: 0, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [], clusters: [], registers: [Register { name: "BDH", offset: 0, size: Some(8), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(255), description: Some("UART Baud Rate Register High"), fields: [Field { name: "SBR", bit_offset: 0, bit_width: 5, access: Some(ReadWrite), description: Some("Baud Rate Modulo Divisor."), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SBNS", bit_offset: 5, bit_width: 1, access: Some(ReadWrite), description: Some("Stop Bit Number Select"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("One stop bit.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Two stop bit.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RXEDGIE", bit_offset: 6, bit_width: 1, access: Some(ReadWrite), description: Some("RX Input Active Edge Interrupt Enable (for RXEDGIF)"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Hardware interrupts from UART_S2[RXEDGIF] disabled (use polling).") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Hardware interrupt requested when UART_S2[RXEDGIF] flag is 1.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "LBKDIE", bit_offset: 7, bit_width: 1, access: Some(ReadWrite), description: Some("LIN Break Detect Interrupt Enable (for LBKDIF)"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Hardware interrupts from UART_S2[LBKDIF] disabled (use polling).") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Hardware interrupt requested when UART_S2[LBKDIF] flag is 1.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "BDL", offset: 1, size: Some(8), access: Some(ReadWrite), reset_value: Some(4), reset_mask: Some(255), description: Some("UART Baud Rate Register Low"), fields: [Field { name: "SBR", bit_offset: 0, bit_width: 8, access: Some(ReadWrite), description: Some("Baud Rate Modulo Divisor"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "C1", offset: 2, size: Some(8), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(255), description: Some("UART Control Register 1"), fields: [Field { name: "PT", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("Parity Type"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Even parity.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Odd parity.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "PE", bit_offset: 1, bit_width: 1, access: Some(ReadWrite), description: Some("Parity Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No hardware parity generation or checking.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Parity enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ILT", bit_offset: 2, bit_width: 1, access: Some(ReadWrite), description: Some("Idle Line Type Select"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Idle character bit count starts after start bit.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Idle character bit count starts after stop bit.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "WAKE", bit_offset: 3, bit_width: 1, access: Some(ReadWrite), description: Some("Receiver Wakeup Method Select"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Idle-line wakeup.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Address-mark wakeup.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "M", bit_offset: 4, bit_width: 1, access: Some(ReadWrite), description: Some("9-Bit or 8-Bit Mode Select"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Receiver and transmitter use 8-bit data characters.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Receiver and transmitter use 9-bit data characters.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RSRC", bit_offset: 5, bit_width: 1, access: Some(ReadWrite), description: Some("Receiver Source Select"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Provided LOOPS is set, RSRC is cleared, selects internal loop back mode and the UART does not use the UART_RX pins.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Single-wire UART mode where the UART_TX pin is connected to the transmitter output and receiver input.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "DOZEEN", bit_offset: 6, bit_width: 1, access: Some(ReadWrite), description: Some("Doze Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("UART is enabled in Wait mode.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("UART is disabled in Wait mode.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "LOOPS", bit_offset: 7, bit_width: 1, access: Some(ReadWrite), description: Some("Loop Mode Select"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Normal operation - UART_RX and UART_TX use separate pins.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Loop mode or single-wire mode where transmitter outputs are internally connected to receiver input. (See RSRC bit.) UART_RX pin is not used by UART.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "C2", offset: 3, size: Some(8), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(255), description: Some("UART Control Register 2"), fields: [Field { name: "SBK", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("Send Break"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Normal transmitter operation.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Queue break character(s) to be sent.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RWU", bit_offset: 1, bit_width: 1, access: Some(ReadWrite), description: Some("Receiver Wakeup Control"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Normal UART receiver operation.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("UART receiver in standby waiting for wakeup condition.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RE", bit_offset: 2, bit_width: 1, access: Some(ReadWrite), description: Some("Receiver Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Receiver disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Receiver enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TE", bit_offset: 3, bit_width: 1, access: Some(ReadWrite), description: Some("Transmitter Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Transmitter disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Transmitter enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ILIE", bit_offset: 4, bit_width: 1, access: Some(ReadWrite), description: Some("Idle Line Interrupt Enable for IDLE"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Hardware interrupts from IDLE disabled; use polling.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Hardware interrupt requested when IDLE flag is 1.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RIE", bit_offset: 5, bit_width: 1, access: Some(ReadWrite), description: Some("Receiver Interrupt Enable for RDRF"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Hardware interrupts from RDRF disabled; use polling.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Hardware interrupt requested when RDRF flag is 1.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TCIE", bit_offset: 6, bit_width: 1, access: Some(ReadWrite), description: Some("Transmission Complete Interrupt Enable for TC"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Hardware interrupts from TC disabled; use polling.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Hardware interrupt requested when TC flag is 1.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TIE", bit_offset: 7, bit_width: 1, access: Some(ReadWrite), description: Some("Transmit Interrupt Enable for TDRE"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Hardware interrupts from TDRE disabled; use polling.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Hardware interrupt requested when TDRE flag is 1.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "S1", offset: 4, size: Some(8), access: Some(ReadWrite), reset_value: Some(192), reset_mask: Some(255), description: Some("UART Status Register 1"), fields: [Field { name: "PF", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("Parity Error Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No parity error.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Parity error.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "FE", bit_offset: 1, bit_width: 1, access: Some(ReadWrite), description: Some("Framing Error Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No framing error detected. This does not guarantee the framing is correct.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Framing error.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "NF", bit_offset: 2, bit_width: 1, access: Some(ReadWrite), description: Some("Noise Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No noise detected.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Noise detected in the received character in UART_D.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "OR", bit_offset: 3, bit_width: 1, access: Some(ReadWrite), description: Some("Receiver Overrun Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No overrun.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Receive overrun (new UART data lost).") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "IDLE", bit_offset: 4, bit_width: 1, access: Some(ReadWrite), description: Some("Idle Line Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No idle line detected.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Idle line was detected.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RDRF", bit_offset: 5, bit_width: 1, access: Some(ReadOnly), description: Some("Receive Data Register Full Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Receive data buffer empty.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Receive data buffer full.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TC", bit_offset: 6, bit_width: 1, access: Some(ReadOnly), description: Some("Transmission Complete Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Transmitter active (sending data, a preamble, or a break).") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Transmitter idle (transmission activity complete).") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TDRE", bit_offset: 7, bit_width: 1, access: Some(ReadOnly), description: Some("Transmit Data Register Empty Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Transmit data buffer full.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Transmit data buffer empty.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "S2", offset: 5, size: Some(8), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(255), description: Some("UART Status Register 2"), fields: [Field { name: "RAF", bit_offset: 0, bit_width: 1, access: Some(ReadOnly), description: Some("Receiver Active Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("UART receiver idle waiting for a start bit.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("UART receiver active ( UART_RXD input not idle).") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "LBKDE", bit_offset: 1, bit_width: 1, access: Some(ReadWrite), description: Some("LIN Break Detection Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Break character is detected at length 10 bit times (if M = 0, SBNS = 0) or 11 (if M = 1, SBNS = 0 or M = 0, SBNS = 1) or 12 (if M = 1, SBNS = 1 or M10 = 1, SNBS = 0) or 13 (if M10 = 1, SNBS = 1).") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Break character is detected at length of 11 bit times (if M = 0, SBNS = 0) or 12 (if M = 1, SBNS = 0 or M = 0, SBNS = 1) or 14 (if M = 1, SBNS = 1 or M10 = 1, SNBS = 0) or 15 (if M10 = 1, SNBS = 1).") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "BRK13", bit_offset: 2, bit_width: 1, access: Some(ReadWrite), description: Some("Break Character Generation Length"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Break character is transmitted with length of 10 bit times (if M = 0, SBNS = 0) or 11 (if M = 1, SBNS = 0 or M = 0, SBNS = 1) or 12 (if M = 1, SBNS = 1 or M10 = 1, SNBS = 0) or 13 (if M10 = 1, SNBS = 1).") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Break character is transmitted with length of 13 bit times (if M = 0, SBNS = 0) or 14 (if M = 1, SBNS = 0 or M = 0, SBNS = 1) or 15 (if M = 1, SBNS = 1 or M10 = 1, SNBS = 0) or 16 (if M10 = 1, SNBS = 1).") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RWUID", bit_offset: 3, bit_width: 1, access: Some(ReadWrite), description: Some("Receive Wake Up Idle Detect"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("During receive standby state (RWU = 1), the IDLE bit does not get set upon detection of an idle character.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("During receive standby state (RWU = 1), the IDLE bit gets set upon detection of an idle character.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RXINV", bit_offset: 4, bit_width: 1, access: Some(ReadWrite), description: Some("Receive Data Inversion"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Receive data not inverted.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Receive data inverted.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MSBF", bit_offset: 5, bit_width: 1, access: Some(ReadWrite), description: Some("MSB First"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("LSB (bit0) is the first bit that is transmitted following the start bit. Further, the first bit received after the start bit is identified as bit0.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("MSB (bit9, bit8, bit7 or bit6) is the first bit that is transmitted following the start bit depending on the setting of C1[M], C1[PE] and C4[M10]. Further, the first bit received after the start bit is identified as bit9, bit8, bit7 or bit6 depending on the setting of C1[M] and C1[PE].") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RXEDGIF", bit_offset: 6, bit_width: 1, access: Some(ReadWrite), description: Some("UART_RX Pin Active Edge Interrupt Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No active edge on the receive pin has occurred.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("An active edge on the receive pin has occurred.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "LBKDIF", bit_offset: 7, bit_width: 1, access: Some(ReadWrite), description: Some("LIN Break Detect Interrupt Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No LIN break character has been detected.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("LIN break character has been detected.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "C3", offset: 6, size: Some(8), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(255), description: Some("UART Control Register 3"), fields: [Field { name: "PEIE", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("Parity Error Interrupt Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("PF interrupts disabled; use polling).") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Hardware interrupt requested when PF is set.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "FEIE", bit_offset: 1, bit_width: 1, access: Some(ReadWrite), description: Some("Framing Error Interrupt Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("FE interrupts disabled; use polling.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Hardware interrupt requested when FE is set.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "NEIE", bit_offset: 2, bit_width: 1, access: Some(ReadWrite), description: Some("Noise Error Interrupt Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("NF interrupts disabled; use polling.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Hardware interrupt requested when NF is set.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ORIE", bit_offset: 3, bit_width: 1, access: Some(ReadWrite), description: Some("Overrun Interrupt Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("OR interrupts disabled; use polling.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Hardware interrupt requested when OR is set.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TXINV", bit_offset: 4, bit_width: 1, access: Some(ReadWrite), description: Some("Transmit Data Inversion"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Transmit data not inverted.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Transmit data inverted.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TXDIR", bit_offset: 5, bit_width: 1, access: Some(ReadWrite), description: Some("UART_TX Pin Direction in Single-Wire Mode"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("UART_TXD pin is an input in single-wire mode.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("UART_TXD pin is an output in single-wire mode.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "R9T8", bit_offset: 6, bit_width: 1, access: Some(ReadWrite), description: Some("Receive Bit 9 / Transmit Bit 8"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "R8T9", bit_offset: 7, bit_width: 1, access: Some(ReadWrite), description: Some("Receive Bit 8 / Transmit Bit 9"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "D", offset: 7, size: Some(8), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(255), description: Some("UART Data Register"), fields: [Field { name: "RT", bit_offset: 0, bit_width: 8, access: Some(ReadWrite), description: Some("Read receive data buffer or write transmit data buffer."), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "MA1", offset: 8, size: Some(8), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(255), description: Some("UART Match Address Registers 1"), fields: [Field { name: "MA", bit_offset: 0, bit_width: 8, access: Some(ReadWrite), description: Some("Match Address"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "MA2", offset: 9, size: Some(8), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(255), description: Some("UART Match Address Registers 2"), fields: [Field { name: "MA", bit_offset: 0, bit_width: 8, access: Some(ReadWrite), description: Some("Match Address"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "C4", offset: 10, size: Some(8), access: Some(ReadWrite), reset_value: Some(15), reset_mask: Some(255), description: Some("UART Control Register 4"), fields: [Field { name: "OSR", bit_offset: 0, bit_width: 5, access: Some(ReadWrite), description: Some("Over Sampling Ratio"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "M10", bit_offset: 5, bit_width: 1, access: Some(ReadWrite), description: Some("10-bit Mode select"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Receiver and transmitter use 8-bit or 9-bit data characters.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Receiver and transmitter use 10-bit data characters.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MAEN2", bit_offset: 6, bit_width: 1, access: Some(ReadWrite), description: Some("Match Address Mode Enable 2"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("All data received is transferred to the data buffer if MAEN1 is cleared.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("All data received with the most significant bit cleared, is discarded. All data received with the most significant bit set, is compared with contents of MA2 register. If no match occurs, the data is discarded. If match occurs, data is transferred to the data buffer.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MAEN1", bit_offset: 7, bit_width: 1, access: Some(ReadWrite), description: Some("Match Address Mode Enable 1"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("All data received is transferred to the data buffer if MAEN2 is cleared.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("All data received with the most significant bit cleared, is discarded. All data received with the most significant bit set, is compared with contents of MA1 register. If no match occurs, the data is discarded. If match occurs, data is transferred to the data buffer.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "C5", offset: 11, size: Some(8), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(255), description: Some("UART Control Register 5"), fields: [Field { name: "RESYNCDIS", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("Resynchronization Disable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Resynchronization during received data word is supported") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Resynchronization during received data word is disabled") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "BOTHEDGE", bit_offset: 1, bit_width: 1, access: Some(ReadWrite), description: Some("Both Edge Sampling"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Receiver samples input data using the rising edge of the baud rate clock.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Receiver samples input data using the rising and falling edge of the baud rate clock.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RDMAE", bit_offset: 5, bit_width: 1, access: Some(ReadWrite), description: Some("Receiver Full DMA Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("DMA request disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("DMA request enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TDMAE", bit_offset: 7, bit_width: 1, access: Some(ReadWrite), description: Some("Transmitter DMA Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("DMA request disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("DMA request enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }), modules: [], has_pins: false, has_channels: false, description: None }

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="UART0 Peripheral"]
pub struct Uart0Periph(pub usize); 


impl Uart0Periph {
#[doc="Get the *const pointer for the BDH register."]
   #[inline] pub fn bdh_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x0) as *const u8
   }
#[doc="Get the *mut pointer for the BDH register."]
   #[inline] pub fn bdh_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x0) as *mut u8
   }
#[doc="Read the BDH register."]
   #[inline] pub fn bdh(&self) -> Bdh { 
      unsafe {
         Bdh(::core::ptr::read_volatile((self.0 + 0x0) as *const u8))
      }
   }
#[doc="Write the BDH register."]
   #[inline] pub fn set_bdh<F: FnOnce(Bdh) -> Bdh>(&self, f: F) -> &Self {
      let value = f(Bdh(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x0) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the BDH register."]
   #[inline] pub fn with_bdh<F: FnOnce(Bdh) -> Bdh>(&self, f: F) -> &Self {
      let tmp = self.bdh();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x0) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the BDL register."]
   #[inline] pub fn bdl_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x1) as *const u8
   }
#[doc="Get the *mut pointer for the BDL register."]
   #[inline] pub fn bdl_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x1) as *mut u8
   }
#[doc="Read the BDL register."]
   #[inline] pub fn bdl(&self) -> Bdl { 
      unsafe {
         Bdl(::core::ptr::read_volatile((self.0 + 0x1) as *const u8))
      }
   }
#[doc="Write the BDL register."]
   #[inline] pub fn set_bdl<F: FnOnce(Bdl) -> Bdl>(&self, f: F) -> &Self {
      let value = f(Bdl(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the BDL register."]
   #[inline] pub fn with_bdl<F: FnOnce(Bdl) -> Bdl>(&self, f: F) -> &Self {
      let tmp = self.bdl();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the C1 register."]
   #[inline] pub fn c1_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x2) as *const u8
   }
#[doc="Get the *mut pointer for the C1 register."]
   #[inline] pub fn c1_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x2) as *mut u8
   }
#[doc="Read the C1 register."]
   #[inline] pub fn c1(&self) -> C1 { 
      unsafe {
         C1(::core::ptr::read_volatile((self.0 + 0x2) as *const u8))
      }
   }
#[doc="Write the C1 register."]
   #[inline] pub fn set_c1<F: FnOnce(C1) -> C1>(&self, f: F) -> &Self {
      let value = f(C1(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x2) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the C1 register."]
   #[inline] pub fn with_c1<F: FnOnce(C1) -> C1>(&self, f: F) -> &Self {
      let tmp = self.c1();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x2) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the C2 register."]
   #[inline] pub fn c2_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x3) as *const u8
   }
#[doc="Get the *mut pointer for the C2 register."]
   #[inline] pub fn c2_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x3) as *mut u8
   }
#[doc="Read the C2 register."]
   #[inline] pub fn c2(&self) -> C2 { 
      unsafe {
         C2(::core::ptr::read_volatile((self.0 + 0x3) as *const u8))
      }
   }
#[doc="Write the C2 register."]
   #[inline] pub fn set_c2<F: FnOnce(C2) -> C2>(&self, f: F) -> &Self {
      let value = f(C2(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x3) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the C2 register."]
   #[inline] pub fn with_c2<F: FnOnce(C2) -> C2>(&self, f: F) -> &Self {
      let tmp = self.c2();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x3) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the S1 register."]
   #[inline] pub fn s1_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x4) as *const u8
   }
#[doc="Get the *mut pointer for the S1 register."]
   #[inline] pub fn s1_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x4) as *mut u8
   }
#[doc="Read the S1 register."]
   #[inline] pub fn s1(&self) -> S1 { 
      unsafe {
         S1(::core::ptr::read_volatile((self.0 + 0x4) as *const u8))
      }
   }
#[doc="Write the S1 register."]
   #[inline] pub fn set_s1<F: FnOnce(S1) -> S1>(&self, f: F) -> &Self {
      let value = f(S1(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x4) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the S1 register."]
   #[inline] pub fn with_s1<F: FnOnce(S1) -> S1>(&self, f: F) -> &Self {
      let tmp = self.s1();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x4) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the S2 register."]
   #[inline] pub fn s2_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x5) as *const u8
   }
#[doc="Get the *mut pointer for the S2 register."]
   #[inline] pub fn s2_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x5) as *mut u8
   }
#[doc="Read the S2 register."]
   #[inline] pub fn s2(&self) -> S2 { 
      unsafe {
         S2(::core::ptr::read_volatile((self.0 + 0x5) as *const u8))
      }
   }
#[doc="Write the S2 register."]
   #[inline] pub fn set_s2<F: FnOnce(S2) -> S2>(&self, f: F) -> &Self {
      let value = f(S2(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x5) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the S2 register."]
   #[inline] pub fn with_s2<F: FnOnce(S2) -> S2>(&self, f: F) -> &Self {
      let tmp = self.s2();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x5) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the C3 register."]
   #[inline] pub fn c3_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x6) as *const u8
   }
#[doc="Get the *mut pointer for the C3 register."]
   #[inline] pub fn c3_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x6) as *mut u8
   }
#[doc="Read the C3 register."]
   #[inline] pub fn c3(&self) -> C3 { 
      unsafe {
         C3(::core::ptr::read_volatile((self.0 + 0x6) as *const u8))
      }
   }
#[doc="Write the C3 register."]
   #[inline] pub fn set_c3<F: FnOnce(C3) -> C3>(&self, f: F) -> &Self {
      let value = f(C3(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x6) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the C3 register."]
   #[inline] pub fn with_c3<F: FnOnce(C3) -> C3>(&self, f: F) -> &Self {
      let tmp = self.c3();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x6) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the D register."]
   #[inline] pub fn d_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x7) as *const u8
   }
#[doc="Get the *mut pointer for the D register."]
   #[inline] pub fn d_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x7) as *mut u8
   }
#[doc="Read the D register."]
   #[inline] pub fn d(&self) -> D { 
      unsafe {
         D(::core::ptr::read_volatile((self.0 + 0x7) as *const u8))
      }
   }
#[doc="Write the D register."]
   #[inline] pub fn set_d<F: FnOnce(D) -> D>(&self, f: F) -> &Self {
      let value = f(D(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x7) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the D register."]
   #[inline] pub fn with_d<F: FnOnce(D) -> D>(&self, f: F) -> &Self {
      let tmp = self.d();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x7) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the MA1 register."]
   #[inline] pub fn ma1_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x8) as *const u8
   }
#[doc="Get the *mut pointer for the MA1 register."]
   #[inline] pub fn ma1_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x8) as *mut u8
   }
#[doc="Read the MA1 register."]
   #[inline] pub fn ma1(&self) -> Ma1 { 
      unsafe {
         Ma1(::core::ptr::read_volatile((self.0 + 0x8) as *const u8))
      }
   }
#[doc="Write the MA1 register."]
   #[inline] pub fn set_ma1<F: FnOnce(Ma1) -> Ma1>(&self, f: F) -> &Self {
      let value = f(Ma1(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x8) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the MA1 register."]
   #[inline] pub fn with_ma1<F: FnOnce(Ma1) -> Ma1>(&self, f: F) -> &Self {
      let tmp = self.ma1();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x8) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the MA2 register."]
   #[inline] pub fn ma2_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x9) as *const u8
   }
#[doc="Get the *mut pointer for the MA2 register."]
   #[inline] pub fn ma2_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x9) as *mut u8
   }
#[doc="Read the MA2 register."]
   #[inline] pub fn ma2(&self) -> Ma2 { 
      unsafe {
         Ma2(::core::ptr::read_volatile((self.0 + 0x9) as *const u8))
      }
   }
#[doc="Write the MA2 register."]
   #[inline] pub fn set_ma2<F: FnOnce(Ma2) -> Ma2>(&self, f: F) -> &Self {
      let value = f(Ma2(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x9) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the MA2 register."]
   #[inline] pub fn with_ma2<F: FnOnce(Ma2) -> Ma2>(&self, f: F) -> &Self {
      let tmp = self.ma2();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x9) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the C4 register."]
   #[inline] pub fn c4_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0xa) as *const u8
   }
#[doc="Get the *mut pointer for the C4 register."]
   #[inline] pub fn c4_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0xa) as *mut u8
   }
#[doc="Read the C4 register."]
   #[inline] pub fn c4(&self) -> C4 { 
      unsafe {
         C4(::core::ptr::read_volatile((self.0 + 0xa) as *const u8))
      }
   }
#[doc="Write the C4 register."]
   #[inline] pub fn set_c4<F: FnOnce(C4) -> C4>(&self, f: F) -> &Self {
      let value = f(C4(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xa) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the C4 register."]
   #[inline] pub fn with_c4<F: FnOnce(C4) -> C4>(&self, f: F) -> &Self {
      let tmp = self.c4();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xa) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the C5 register."]
   #[inline] pub fn c5_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0xb) as *const u8
   }
#[doc="Get the *mut pointer for the C5 register."]
   #[inline] pub fn c5_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0xb) as *mut u8
   }
#[doc="Read the C5 register."]
   #[inline] pub fn c5(&self) -> C5 { 
      unsafe {
         C5(::core::ptr::read_volatile((self.0 + 0xb) as *const u8))
      }
   }
#[doc="Write the C5 register."]
   #[inline] pub fn set_c5<F: FnOnce(C5) -> C5>(&self, f: F) -> &Self {
      let value = f(C5(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xb) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the C5 register."]
   #[inline] pub fn with_c5<F: FnOnce(C5) -> C5>(&self, f: F) -> &Self {
      let tmp = self.c5();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xb) as *mut u8, value.0);
      }
      self
   }

}

#[doc="UART Baud Rate Register High"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Bdh(pub u8);
impl Bdh {
#[doc="Baud Rate Modulo Divisor."]
   #[inline] pub fn sbr(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
   }
#[doc="Baud Rate Modulo Divisor."]
   #[inline] pub fn set_sbr<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1f << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Stop Bit Number Select"]
   #[inline] pub fn sbns(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Stop Bit Number Select"]
   #[inline] pub fn set_sbns<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="RX Input Active Edge Interrupt Enable (for RXEDGIF)"]
   #[inline] pub fn rxedgie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="RX Input Active Edge Interrupt Enable (for RXEDGIF)"]
   #[inline] pub fn set_rxedgie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="LIN Break Detect Interrupt Enable (for LBKDIF)"]
   #[inline] pub fn lbkdie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="LIN Break Detect Interrupt Enable (for LBKDIF)"]
   #[inline] pub fn set_lbkdie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Bdh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Bdh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sbr() != 0 { try!(write!(f, " sbr=0x{:x}", self.sbr()))}
      if self.sbns() != 0 { try!(write!(f, " sbns"))}
      if self.rxedgie() != 0 { try!(write!(f, " rxedgie"))}
      if self.lbkdie() != 0 { try!(write!(f, " lbkdie"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART Baud Rate Register Low"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Bdl(pub u8);
impl Bdl {
#[doc="Baud Rate Modulo Divisor"]
   #[inline] pub fn sbr(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Baud Rate Modulo Divisor"]
   #[inline] pub fn set_sbr<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Bdl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Bdl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sbr() != 0 { try!(write!(f, " sbr=0x{:x}", self.sbr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART Control Register 1"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct C1(pub u8);
impl C1 {
#[doc="Parity Type"]
   #[inline] pub fn pt(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Parity Type"]
   #[inline] pub fn set_pt<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
      let value: u8 = value.into();
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
      let value: u8 = value.into();
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
      let value: u8 = value.into();
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
      let value: u8 = value.into();
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
      let value: u8 = value.into();
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
      let value: u8 = value.into();
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
      let value: u8 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for C1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for C1 {
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
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART Control Register 2"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct C2(pub u8);
impl C2 {
#[doc="Send Break"]
   #[inline] pub fn sbk(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Send Break"]
   #[inline] pub fn set_sbk<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Receiver Wakeup Control"]
   #[inline] pub fn rwu(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Receiver Wakeup Control"]
   #[inline] pub fn set_rwu<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Receiver Enable"]
   #[inline] pub fn re(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Receiver Enable"]
   #[inline] pub fn set_re<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Transmitter Enable"]
   #[inline] pub fn te(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Transmitter Enable"]
   #[inline] pub fn set_te<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Idle Line Interrupt Enable for IDLE"]
   #[inline] pub fn ilie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Idle Line Interrupt Enable for IDLE"]
   #[inline] pub fn set_ilie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Receiver Interrupt Enable for RDRF"]
   #[inline] pub fn rie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Receiver Interrupt Enable for RDRF"]
   #[inline] pub fn set_rie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Transmission Complete Interrupt Enable for TC"]
   #[inline] pub fn tcie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Transmission Complete Interrupt Enable for TC"]
   #[inline] pub fn set_tcie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Transmit Interrupt Enable for TDRE"]
   #[inline] pub fn tie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Transmit Interrupt Enable for TDRE"]
   #[inline] pub fn set_tie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for C2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for C2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sbk() != 0 { try!(write!(f, " sbk"))}
      if self.rwu() != 0 { try!(write!(f, " rwu"))}
      if self.re() != 0 { try!(write!(f, " re"))}
      if self.te() != 0 { try!(write!(f, " te"))}
      if self.ilie() != 0 { try!(write!(f, " ilie"))}
      if self.rie() != 0 { try!(write!(f, " rie"))}
      if self.tcie() != 0 { try!(write!(f, " tcie"))}
      if self.tie() != 0 { try!(write!(f, " tie"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART Status Register 1"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct S1(pub u8);
impl S1 {
#[doc="Parity Error Flag"]
   #[inline] pub fn pf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Parity Error Flag"]
   #[inline] pub fn set_pf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Framing Error Flag"]
   #[inline] pub fn fe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Framing Error Flag"]
   #[inline] pub fn set_fe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Noise Flag"]
   #[inline] pub fn nf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Noise Flag"]
   #[inline] pub fn set_nf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Receiver Overrun Flag"]
   #[inline] pub fn or(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Receiver Overrun Flag"]
   #[inline] pub fn set_or<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Idle Line Flag"]
   #[inline] pub fn idle(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Idle Line Flag"]
   #[inline] pub fn set_idle<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Receive Data Register Full Flag"]
   #[inline] pub fn rdrf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Receive Data Register Full Flag"]
   #[inline] pub fn set_rdrf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Transmission Complete Flag"]
   #[inline] pub fn tc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Transmission Complete Flag"]
   #[inline] pub fn set_tc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Transmit Data Register Empty Flag"]
   #[inline] pub fn tdre(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Transmit Data Register Empty Flag"]
   #[inline] pub fn set_tdre<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for S1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for S1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pf() != 0 { try!(write!(f, " pf"))}
      if self.fe() != 0 { try!(write!(f, " fe"))}
      if self.nf() != 0 { try!(write!(f, " nf"))}
      if self.or() != 0 { try!(write!(f, " or"))}
      if self.idle() != 0 { try!(write!(f, " idle"))}
      if self.rdrf() != 0 { try!(write!(f, " rdrf"))}
      if self.tc() != 0 { try!(write!(f, " tc"))}
      if self.tdre() != 0 { try!(write!(f, " tdre"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART Status Register 2"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct S2(pub u8);
impl S2 {
#[doc="Receiver Active Flag"]
   #[inline] pub fn raf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Receiver Active Flag"]
   #[inline] pub fn set_raf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="LIN Break Detection Enable"]
   #[inline] pub fn lbkde(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="LIN Break Detection Enable"]
   #[inline] pub fn set_lbkde<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Break Character Generation Length"]
   #[inline] pub fn brk13(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Break Character Generation Length"]
   #[inline] pub fn set_brk13<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Receive Wake Up Idle Detect"]
   #[inline] pub fn rwuid(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Receive Wake Up Idle Detect"]
   #[inline] pub fn set_rwuid<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Receive Data Inversion"]
   #[inline] pub fn rxinv(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Receive Data Inversion"]
   #[inline] pub fn set_rxinv<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="MSB First"]
   #[inline] pub fn msbf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="MSB First"]
   #[inline] pub fn set_msbf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="UART_RX Pin Active Edge Interrupt Flag"]
   #[inline] pub fn rxedgif(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="UART_RX Pin Active Edge Interrupt Flag"]
   #[inline] pub fn set_rxedgif<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="LIN Break Detect Interrupt Flag"]
   #[inline] pub fn lbkdif(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="LIN Break Detect Interrupt Flag"]
   #[inline] pub fn set_lbkdif<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for S2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for S2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
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
#[doc="UART Control Register 3"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct C3(pub u8);
impl C3 {
#[doc="Parity Error Interrupt Enable"]
   #[inline] pub fn peie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Parity Error Interrupt Enable"]
   #[inline] pub fn set_peie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Framing Error Interrupt Enable"]
   #[inline] pub fn feie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Framing Error Interrupt Enable"]
   #[inline] pub fn set_feie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Noise Error Interrupt Enable"]
   #[inline] pub fn neie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Noise Error Interrupt Enable"]
   #[inline] pub fn set_neie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Overrun Interrupt Enable"]
   #[inline] pub fn orie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Overrun Interrupt Enable"]
   #[inline] pub fn set_orie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Transmit Data Inversion"]
   #[inline] pub fn txinv(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Transmit Data Inversion"]
   #[inline] pub fn set_txinv<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="UART_TX Pin Direction in Single-Wire Mode"]
   #[inline] pub fn txdir(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="UART_TX Pin Direction in Single-Wire Mode"]
   #[inline] pub fn set_txdir<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Receive Bit 9 / Transmit Bit 8"]
   #[inline] pub fn r9t8(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Receive Bit 9 / Transmit Bit 8"]
   #[inline] pub fn set_r9t8<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Receive Bit 8 / Transmit Bit 9"]
   #[inline] pub fn r8t9(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Receive Bit 8 / Transmit Bit 9"]
   #[inline] pub fn set_r8t9<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for C3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for C3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
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
#[doc="UART Data Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct D(pub u8);
impl D {
#[doc="Read receive data buffer or write transmit data buffer."]
   #[inline] pub fn rt(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Read receive data buffer or write transmit data buffer."]
   #[inline] pub fn set_rt<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for D {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for D {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rt() != 0 { try!(write!(f, " rt=0x{:x}", self.rt()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART Match Address Registers 1"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ma1(pub u8);
impl Ma1 {
#[doc="Match Address"]
   #[inline] pub fn ma(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Match Address"]
   #[inline] pub fn set_ma<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Ma1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ma1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ma() != 0 { try!(write!(f, " ma=0x{:x}", self.ma()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART Match Address Registers 2"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ma2(pub u8);
impl Ma2 {
#[doc="Match Address"]
   #[inline] pub fn ma(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Match Address"]
   #[inline] pub fn set_ma<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Ma2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ma2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ma() != 0 { try!(write!(f, " ma=0x{:x}", self.ma()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART Control Register 4"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct C4(pub u8);
impl C4 {
#[doc="Over Sampling Ratio"]
   #[inline] pub fn osr(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
   }
#[doc="Over Sampling Ratio"]
   #[inline] pub fn set_osr<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1f << 0);
      self.0 |= value << 0;
      self
   }

#[doc="10-bit Mode select"]
   #[inline] pub fn m10(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="10-bit Mode select"]
   #[inline] pub fn set_m10<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Match Address Mode Enable 2"]
   #[inline] pub fn maen2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Match Address Mode Enable 2"]
   #[inline] pub fn set_maen2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Match Address Mode Enable 1"]
   #[inline] pub fn maen1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Match Address Mode Enable 1"]
   #[inline] pub fn set_maen1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for C4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for C4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.osr() != 0 { try!(write!(f, " osr=0x{:x}", self.osr()))}
      if self.m10() != 0 { try!(write!(f, " m10"))}
      if self.maen2() != 0 { try!(write!(f, " maen2"))}
      if self.maen1() != 0 { try!(write!(f, " maen1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART Control Register 5"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct C5(pub u8);
impl C5 {
#[doc="Resynchronization Disable"]
   #[inline] pub fn resyncdis(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Resynchronization Disable"]
   #[inline] pub fn set_resyncdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Both Edge Sampling"]
   #[inline] pub fn bothedge(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Both Edge Sampling"]
   #[inline] pub fn set_bothedge<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Receiver Full DMA Enable"]
   #[inline] pub fn rdmae(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Receiver Full DMA Enable"]
   #[inline] pub fn set_rdmae<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Transmitter DMA Enable"]
   #[inline] pub fn tdmae(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Transmitter DMA Enable"]
   #[inline] pub fn set_tdmae<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for C5 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for C5 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.resyncdis() != 0 { try!(write!(f, " resyncdis"))}
      if self.bothedge() != 0 { try!(write!(f, " bothedge"))}
      if self.rdmae() != 0 { try!(write!(f, " rdmae"))}
      if self.tdmae() != 0 { try!(write!(f, " tdmae"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
