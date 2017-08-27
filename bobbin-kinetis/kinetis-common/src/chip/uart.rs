#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "UART", peripherals: [], prototype: Some(Peripheral { derived_from: None, group_name: Some("UART"), name: "", address: 0, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [], clusters: [], registers: [Register { name: "BDH", offset: 0, size: Some(8), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(255), description: Some("UART Baud Rate Registers: High"), fields: [Field { name: "SBR", bit_offset: 0, bit_width: 5, access: Some(ReadWrite), description: Some("UART Baud Rate Bits"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SBNS", bit_offset: 5, bit_width: 1, access: Some(ReadWrite), description: Some("Stop Bit Number Select"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Data frame consists of a single stop bit.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Data frame consists of two stop bits.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RXEDGIE", bit_offset: 6, bit_width: 1, access: Some(ReadWrite), description: Some("RxD Input Active Edge Interrupt Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Hardware interrupts from RXEDGIF disabled using polling.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("RXEDGIF interrupt request enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "LBKDIE", bit_offset: 7, bit_width: 1, access: Some(ReadWrite), description: Some("LIN Break Detect Interrupt or DMA Request Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("LBKDIF interrupt and DMA transfer requests disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("LBKDIF interrupt or DMA transfer requests enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "BDL", offset: 1, size: Some(8), access: Some(ReadWrite), reset_value: Some(4), reset_mask: Some(255), description: Some("UART Baud Rate Registers: Low"), fields: [Field { name: "SBR", bit_offset: 0, bit_width: 8, access: Some(ReadWrite), description: Some("UART Baud Rate Bits"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "C1", offset: 2, size: Some(8), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(255), description: Some("UART Control Register 1"), fields: [Field { name: "PT", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("Parity Type"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Even parity.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Odd parity.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "PE", bit_offset: 1, bit_width: 1, access: Some(ReadWrite), description: Some("Parity Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Parity function disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Parity function enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ILT", bit_offset: 2, bit_width: 1, access: Some(ReadWrite), description: Some("Idle Line Type Select"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Idle character bit count starts after start bit.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Idle character bit count starts after stop bit.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "WAKE", bit_offset: 3, bit_width: 1, access: Some(ReadWrite), description: Some("Receiver Wakeup Method Select"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Idle line wakeup.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Address mark wakeup.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "M", bit_offset: 4, bit_width: 1, access: Some(ReadWrite), description: Some("9-bit or 8-bit Mode Select"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Normal-start + 8 data bits (MSB/LSB first as determined by MSBF) + stop.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Use-start + 9 data bits (MSB/LSB first as determined by MSBF) + stop.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RSRC", bit_offset: 5, bit_width: 1, access: Some(ReadWrite), description: Some("Receiver Source Select"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Selects internal loop back mode. The receiver input is internally connected to transmitter output.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Single wire UART mode where the receiver input is connected to the transmit pin input signal.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "UARTSWAI", bit_offset: 6, bit_width: 1, access: Some(ReadWrite), description: Some("UART Stops in Wait Mode"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("UART clock continues to run in Wait mode.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("UART clock freezes while CPU is in Wait mode.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "LOOPS", bit_offset: 7, bit_width: 1, access: Some(ReadWrite), description: Some("Loop Mode Select"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Normal operation.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Loop mode where transmitter output is internally connected to receiver input. The receiver input is determined by RSRC.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "C2", offset: 3, size: Some(8), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(255), description: Some("UART Control Register 2"), fields: [Field { name: "SBK", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("Send Break"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Normal transmitter operation.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Queue break characters to be sent.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RWU", bit_offset: 1, bit_width: 1, access: Some(ReadWrite), description: Some("Receiver Wakeup Control"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Normal operation.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("RWU enables the wakeup function and inhibits further receiver interrupt requests. Normally, hardware wakes the receiver by automatically clearing RWU.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RE", bit_offset: 2, bit_width: 1, access: Some(ReadWrite), description: Some("Receiver Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Receiver off.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Receiver on.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TE", bit_offset: 3, bit_width: 1, access: Some(ReadWrite), description: Some("Transmitter Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Transmitter off.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Transmitter on.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ILIE", bit_offset: 4, bit_width: 1, access: Some(ReadWrite), description: Some("Idle Line Interrupt DMA Transfer Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("IDLE interrupt requests disabled. and DMA transfer") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("IDLE interrupt requests enabled. or DMA transfer") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RIE", bit_offset: 5, bit_width: 1, access: Some(ReadWrite), description: Some("Receiver Full Interrupt or DMA Transfer Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("RDRF interrupt and DMA transfer requests disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("RDRF interrupt or DMA transfer requests enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TCIE", bit_offset: 6, bit_width: 1, access: Some(ReadWrite), description: Some("Transmission Complete Interrupt or DMA Transfer Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("TC interrupt and DMA transfer requests disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("TC interrupt or DMA transfer requests enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TIE", bit_offset: 7, bit_width: 1, access: Some(ReadWrite), description: Some("Transmitter Interrupt or DMA Transfer Enable."), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("TDRE interrupt and DMA transfer requests disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("TDRE interrupt or DMA transfer requests enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "S1", offset: 4, size: Some(8), access: Some(ReadOnly), reset_value: Some(192), reset_mask: Some(255), description: Some("UART Status Register 1"), fields: [Field { name: "PF", bit_offset: 0, bit_width: 1, access: Some(ReadOnly), description: Some("Parity Error Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No parity error detected since the last time this flag was cleared. If the receive buffer has a depth greater than 1, then there may be data in the receive buffer what was received with a parity error.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("At least one dataword was received with a parity error since the last time this flag was cleared.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "FE", bit_offset: 1, bit_width: 1, access: Some(ReadOnly), description: Some("Framing Error Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No framing error detected.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Framing error.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "NF", bit_offset: 2, bit_width: 1, access: Some(ReadOnly), description: Some("Noise Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No noise detected since the last time this flag was cleared. If the receive buffer has a depth greater than 1 then there may be data in the receiver buffer that was received with noise.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("At least one dataword was received with noise detected since the last time the flag was cleared.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "OR", bit_offset: 3, bit_width: 1, access: Some(ReadOnly), description: Some("Receiver Overrun Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No overrun has occurred since the last time the flag was cleared.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Overrun has occurred or the overrun flag has not been cleared since the last overrun occured.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "IDLE", bit_offset: 4, bit_width: 1, access: Some(ReadOnly), description: Some("Idle Line Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Receiver input is either active now or has never become active since the IDLE flag was last cleared.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Receiver input has become idle or the flag has not been cleared since it last asserted.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RDRF", bit_offset: 5, bit_width: 1, access: Some(ReadOnly), description: Some("Receive Data Register Full Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The number of datawords in the receive buffer is less than the number indicated by RXWATER.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The number of datawords in the receive buffer is equal to or greater than the number indicated by RXWATER at some point in time since this flag was last cleared.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TC", bit_offset: 6, bit_width: 1, access: Some(ReadOnly), description: Some("Transmit Complete Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Transmitter active (sending data, a preamble, or a break).") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Transmitter idle (transmission activity complete).") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TDRE", bit_offset: 7, bit_width: 1, access: Some(ReadOnly), description: Some("Transmit Data Register Empty Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The amount of data in the transmit buffer is greater than the value indicated by TWFIFO[TXWATER].") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The amount of data in the transmit buffer is less than or equal to the value indicated by TWFIFO[TXWATER] at some point in time since the flag has been cleared.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "S2", offset: 5, size: Some(8), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(255), description: Some("UART Status Register 2"), fields: [Field { name: "RAF", bit_offset: 0, bit_width: 1, access: Some(ReadOnly), description: Some("Receiver Active Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("UART receiver idle/inactive waiting for a start bit.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("UART receiver active, RxD input not idle.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "LBKDE", bit_offset: 1, bit_width: 1, access: Some(ReadWrite), description: Some("LIN Break Detection Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Break character detection is disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Break character is detected at length of 11 bit times if C1[M] = 0 or 12 bits time if C1[M] = 1.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "BRK13", bit_offset: 2, bit_width: 1, access: Some(ReadWrite), description: Some("Break Transmit Character Length"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Break character is 10, 11, or 12 bits long.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Break character is 13 or 14 bits long.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RWUID", bit_offset: 3, bit_width: 1, access: Some(ReadWrite), description: Some("Receive Wakeup Idle Detect"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("S1[IDLE] is not set upon detection of an idle character.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("S1[IDLE] is set upon detection of an idle character.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RXINV", bit_offset: 4, bit_width: 1, access: Some(ReadWrite), description: Some("Receive Data Inversion"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Receive data is not inverted.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Receive data is inverted.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MSBF", bit_offset: 5, bit_width: 1, access: Some(ReadWrite), description: Some("Most Significant Bit First"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("LSB (bit0) is the first bit that is transmitted following the start bit. Further, the first bit received after the start bit is identified as bit0.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("MSB (bit8, bit7 or bit6) is the first bit that is transmitted following the start bit, depending on the setting of C1[M] and C1[PE]. Further, the first bit received after the start bit is identified as bit8, bit7, or bit6, depending on the setting of C1[M] and C1[PE].") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RXEDGIF", bit_offset: 6, bit_width: 1, access: Some(ReadWrite), description: Some("RxD Pin Active Edge Interrupt Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No active edge on the receive pin has occurred.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("An active edge on the receive pin has occurred.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "LBKDIF", bit_offset: 7, bit_width: 1, access: Some(ReadWrite), description: Some("LIN Break Detect Interrupt Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No LIN break character detected.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("LIN break character detected.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "C3", offset: 6, size: Some(8), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(255), description: Some("UART Control Register 3"), fields: [Field { name: "PEIE", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("Parity Error Interrupt Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("PF interrupt requests are disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("PF interrupt requests are enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "FEIE", bit_offset: 1, bit_width: 1, access: Some(ReadWrite), description: Some("Framing Error Interrupt Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("FE interrupt requests are disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("FE interrupt requests are enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "NEIE", bit_offset: 2, bit_width: 1, access: Some(ReadWrite), description: Some("Noise Error Interrupt Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("NF interrupt requests are disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("NF interrupt requests are enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ORIE", bit_offset: 3, bit_width: 1, access: Some(ReadWrite), description: Some("Overrun Error Interrupt Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("OR interrupts are disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("OR interrupt requests are enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TXINV", bit_offset: 4, bit_width: 1, access: Some(ReadWrite), description: Some("Transmit Data Inversion."), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Transmit data is not inverted.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Transmit data is inverted.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TXDIR", bit_offset: 5, bit_width: 1, access: Some(ReadWrite), description: Some("Transmitter Pin Data Direction in Single-Wire mode"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("TXD pin is an input in single wire mode.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("TXD pin is an output in single wire mode.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "T8", bit_offset: 6, bit_width: 1, access: Some(ReadWrite), description: Some("Transmit Bit 8"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "R8", bit_offset: 7, bit_width: 1, access: Some(ReadOnly), description: Some("Received Bit 8"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "D", offset: 7, size: Some(8), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(255), description: Some("UART Data Register"), fields: [Field { name: "RT", bit_offset: 0, bit_width: 8, access: Some(ReadWrite), description: Some("Reads return the contents of the read-only receive data register and writes go to the write-only transmit data register"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "MA1", offset: 8, size: Some(8), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(255), description: Some("UART Match Address Registers 1"), fields: [Field { name: "MA", bit_offset: 0, bit_width: 8, access: Some(ReadWrite), description: Some("Match Address"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "MA2", offset: 9, size: Some(8), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(255), description: Some("UART Match Address Registers 2"), fields: [Field { name: "MA", bit_offset: 0, bit_width: 8, access: Some(ReadWrite), description: Some("Match Address"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "C4", offset: 10, size: Some(8), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(255), description: Some("UART Control Register 4"), fields: [Field { name: "BRFA", bit_offset: 0, bit_width: 5, access: Some(ReadWrite), description: Some("Baud Rate Fine Adjust"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "M10", bit_offset: 5, bit_width: 1, access: Some(ReadWrite), description: Some("10-bit Mode select"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The parity bit is the ninth bit in the serial transmission.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The parity bit is the tenth bit in the serial transmission.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MAEN2", bit_offset: 6, bit_width: 1, access: Some(ReadWrite), description: Some("Match Address Mode Enable 2"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("All data received is transferred to the data buffer if MAEN1 is cleared.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("All data received with the most significant bit cleared, is discarded. All data received with the most significant bit set, is compared with contents of MA2 register. If no match occurs, the data is discarded. If a match occurs, data is transferred to the data buffer. This field must be cleared when C7816[ISO7816E] is set/enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MAEN1", bit_offset: 7, bit_width: 1, access: Some(ReadWrite), description: Some("Match Address Mode Enable 1"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("All data received is transferred to the data buffer if MAEN2 is cleared.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("All data received with the most significant bit cleared, is discarded. All data received with the most significant bit set, is compared with contents of MA1 register. If no match occurs, the data is discarded. If match occurs, data is transferred to the data buffer. This field must be cleared when C7816[ISO7816E] is set/enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "C5", offset: 11, size: Some(8), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(255), description: Some("UART Control Register 5"), fields: [Field { name: "LBKDDMAS", bit_offset: 3, bit_width: 1, access: Some(ReadWrite), description: Some("LIN Break Detect DMA Select Bit"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("If BDH[LBKDIE] and S2[LBKDIF] are set, the LBKDIF interrupt signal is asserted to request an interrupt service.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("If BDH[LBKDIE] and S2[LBKDIF] are set, the LBKDIF DMA request signal is asserted to request a DMA transfer.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ILDMAS", bit_offset: 4, bit_width: 1, access: Some(ReadWrite), description: Some("Idle Line DMA Select"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("If C2[ILIE] and S1[IDLE] are set, the IDLE interrupt request signal is asserted to request an interrupt service.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("If C2[ILIE] and S1[IDLE] are set, the IDLE DMA request signal is asserted to request a DMA transfer.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RDMAS", bit_offset: 5, bit_width: 1, access: Some(ReadWrite), description: Some("Receiver Full DMA Select"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("If C2[RIE] and S1[RDRF] are set, the RDFR interrupt request signal is asserted to request an interrupt service.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("If C2[RIE] and S1[RDRF] are set, the RDRF DMA request signal is asserted to request a DMA transfer.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TCDMAS", bit_offset: 6, bit_width: 1, access: Some(ReadWrite), description: Some("Transmission Complete DMA Select"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("If C2[TCIE] is set and the S1[TC] flag is set, the TC interrupt request signal is asserted to request an interrupt service.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("If C2[TCIE] is set and the S1[TC] flag is set, the TC DMA request signal is asserted to request a DMA transfer.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TDMAS", bit_offset: 7, bit_width: 1, access: Some(ReadWrite), description: Some("Transmitter DMA Select"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("If C2[TIE] is set and the S1[TDRE] flag is set, the TDRE interrupt request signal is asserted to request interrupt service.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("If C2[TIE] is set and the S1[TDRE] flag is set, the TDRE DMA request signal is asserted to request a DMA transfer.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "ED", offset: 12, size: Some(8), access: Some(ReadOnly), reset_value: Some(0), reset_mask: Some(255), description: Some("UART Extended Data Register"), fields: [Field { name: "PARITYE", bit_offset: 6, bit_width: 1, access: Some(ReadOnly), description: Some("The current received dataword contained in D and C3[R8] was received with a parity error."), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The dataword was received without a parity error.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The dataword was received with a parity error.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "NOISY", bit_offset: 7, bit_width: 1, access: Some(ReadOnly), description: Some("The current received dataword contained in D and C3[R8] was received with noise."), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The dataword was received without noise.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The data was received with noise.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "MODEM", offset: 13, size: Some(8), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(255), description: Some("UART Modem Register"), fields: [Field { name: "TXCTSE", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("Transmitter clear-to-send enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("CTS has no effect on the transmitter.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Enables clear-to-send operation. The transmitter checks the state of CTS each time it is ready to send a character. If CTS is asserted, the character is sent. If CTS is deasserted, the signal TXD remains in the mark state and transmission is delayed until CTS is asserted. Changes in CTS as a character is being sent do not affect its transmission.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TXRTSE", bit_offset: 1, bit_width: 1, access: Some(ReadWrite), description: Some("Transmitter request-to-send enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The transmitter has no effect on RTS.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("When a character is placed into an empty transmitter data buffer , RTS asserts one bit time before the start bit is transmitted. RTS deasserts one bit time after all characters in the transmitter data buffer and shift register are completely sent, including the last stop bit. (FIFO) (FIFO)") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TXRTSPOL", bit_offset: 2, bit_width: 1, access: Some(ReadWrite), description: Some("Transmitter request-to-send polarity"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Transmitter RTS is active low.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Transmitter RTS is active high.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RXRTSE", bit_offset: 3, bit_width: 1, access: Some(ReadWrite), description: Some("Receiver request-to-send enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The receiver has no effect on RTS.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("RTS is deasserted if the number of characters in the receiver data register (FIFO) is equal to or greater than RWFIFO[RXWATER]. RTS is asserted when the number of characters in the receiver data register (FIFO) is less than RWFIFO[RXWATER].") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "IR", offset: 14, size: Some(8), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(255), description: Some("UART Infrared Register"), fields: [Field { name: "TNP", bit_offset: 0, bit_width: 2, access: Some(ReadWrite), description: Some("Transmitter narrow pulse"), enumerated_values: [EnumeratedValue { value: "#00", name: Some("00"), description: Some("3/16.") }, EnumeratedValue { value: "#01", name: Some("01"), description: Some("1/16.") }, EnumeratedValue { value: "#10", name: Some("10"), description: Some("1/32.") }, EnumeratedValue { value: "#11", name: Some("11"), description: Some("1/4.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "IREN", bit_offset: 2, bit_width: 1, access: Some(ReadWrite), description: Some("Infrared enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("IR disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("IR enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "PFIFO", offset: 16, size: Some(8), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(255), description: Some("UART FIFO Parameters"), fields: [Field { name: "RXFIFOSIZE", bit_offset: 0, bit_width: 3, access: Some(ReadOnly), description: Some("Receive FIFO. Buffer Depth"), enumerated_values: [EnumeratedValue { value: "#000", name: Some("000"), description: Some("Receive FIFO/Buffer depth = 1 dataword.") }, EnumeratedValue { value: "#001", name: Some("001"), description: Some("Receive FIFO/Buffer depth = 4 datawords.") }, EnumeratedValue { value: "#010", name: Some("010"), description: Some("Receive FIFO/Buffer depth = 8 datawords.") }, EnumeratedValue { value: "#011", name: Some("011"), description: Some("Receive FIFO/Buffer depth = 16 datawords.") }, EnumeratedValue { value: "#100", name: Some("100"), description: Some("Receive FIFO/Buffer depth = 32 datawords.") }, EnumeratedValue { value: "#101", name: Some("101"), description: Some("Receive FIFO/Buffer depth = 64 datawords.") }, EnumeratedValue { value: "#110", name: Some("110"), description: Some("Receive FIFO/Buffer depth = 128 datawords.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RXFE", bit_offset: 3, bit_width: 1, access: Some(ReadWrite), description: Some("Receive FIFO Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Receive FIFO is not enabled. Buffer is depth 1. (Legacy support)") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Receive FIFO is enabled. Buffer is depth indicted by RXFIFOSIZE.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TXFIFOSIZE", bit_offset: 4, bit_width: 3, access: Some(ReadOnly), description: Some("Transmit FIFO. Buffer Depth"), enumerated_values: [EnumeratedValue { value: "#000", name: Some("000"), description: Some("Transmit FIFO/Buffer depth = 1 dataword.") }, EnumeratedValue { value: "#001", name: Some("001"), description: Some("Transmit FIFO/Buffer depth = 4 datawords.") }, EnumeratedValue { value: "#010", name: Some("010"), description: Some("Transmit FIFO/Buffer depth = 8 datawords.") }, EnumeratedValue { value: "#011", name: Some("011"), description: Some("Transmit FIFO/Buffer depth = 16 datawords.") }, EnumeratedValue { value: "#100", name: Some("100"), description: Some("Transmit FIFO/Buffer depth = 32 datawords.") }, EnumeratedValue { value: "#101", name: Some("101"), description: Some("Transmit FIFO/Buffer depth = 64 datawords.") }, EnumeratedValue { value: "#110", name: Some("110"), description: Some("Transmit FIFO/Buffer depth = 128 datawords.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TXFE", bit_offset: 7, bit_width: 1, access: Some(ReadWrite), description: Some("Transmit FIFO Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Transmit FIFO is not enabled. Buffer is depth 1. (Legacy support).") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Transmit FIFO is enabled. Buffer is depth indicated by TXFIFOSIZE.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "CFIFO", offset: 17, size: Some(8), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(255), description: Some("UART FIFO Control Register"), fields: [Field { name: "RXUFE", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("Receive FIFO Underflow Interrupt Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("RXUF flag does not generate an interrupt to the host.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("RXUF flag generates an interrupt to the host.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TXOFE", bit_offset: 1, bit_width: 1, access: Some(ReadWrite), description: Some("Transmit FIFO Overflow Interrupt Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("TXOF flag does not generate an interrupt to the host.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("TXOF flag generates an interrupt to the host.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RXOFE", bit_offset: 2, bit_width: 1, access: Some(ReadWrite), description: Some("Receive FIFO Overflow Interrupt Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("RXOF flag does not generate an interrupt to the host.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("RXOF flag generates an interrupt to the host.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RXFLUSH", bit_offset: 6, bit_width: 1, access: Some(WriteOnly), description: Some("Receive FIFO/Buffer Flush"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No flush operation occurs.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("All data in the receive FIFO/buffer is cleared out.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TXFLUSH", bit_offset: 7, bit_width: 1, access: Some(WriteOnly), description: Some("Transmit FIFO/Buffer Flush"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No flush operation occurs.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("All data in the transmit FIFO/Buffer is cleared out.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "SFIFO", offset: 18, size: Some(8), access: Some(ReadWrite), reset_value: Some(192), reset_mask: Some(255), description: Some("UART FIFO Status Register"), fields: [Field { name: "RXUF", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("Receiver Buffer Underflow Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No receive buffer underflow has occurred since the last time the flag was cleared.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("At least one receive buffer underflow has occurred since the last time the flag was cleared.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TXOF", bit_offset: 1, bit_width: 1, access: Some(ReadWrite), description: Some("Transmitter Buffer Overflow Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No transmit buffer overflow has occurred since the last time the flag was cleared.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("At least one transmit buffer overflow has occurred since the last time the flag was cleared.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RXOF", bit_offset: 2, bit_width: 1, access: Some(ReadWrite), description: Some("Receiver Buffer Overflow Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No receive buffer overflow has occurred since the last time the flag was cleared.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("At least one receive buffer overflow has occurred since the last time the flag was cleared.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RXEMPT", bit_offset: 6, bit_width: 1, access: Some(ReadOnly), description: Some("Receive Buffer/FIFO Empty"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Receive buffer is not empty.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Receive buffer is empty.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TXEMPT", bit_offset: 7, bit_width: 1, access: Some(ReadOnly), description: Some("Transmit Buffer/FIFO Empty"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Transmit buffer is not empty.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Transmit buffer is empty.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "TWFIFO", offset: 19, size: Some(8), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(255), description: Some("UART FIFO Transmit Watermark"), fields: [Field { name: "TXWATER", bit_offset: 0, bit_width: 8, access: Some(ReadWrite), description: Some("Transmit Watermark"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "TCFIFO", offset: 20, size: Some(8), access: Some(ReadOnly), reset_value: Some(0), reset_mask: Some(255), description: Some("UART FIFO Transmit Count"), fields: [Field { name: "TXCOUNT", bit_offset: 0, bit_width: 8, access: Some(ReadOnly), description: Some("Transmit Counter"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "RWFIFO", offset: 21, size: Some(8), access: Some(ReadWrite), reset_value: Some(1), reset_mask: Some(255), description: Some("UART FIFO Receive Watermark"), fields: [Field { name: "RXWATER", bit_offset: 0, bit_width: 8, access: Some(ReadWrite), description: Some("Receive Watermark"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "RCFIFO", offset: 22, size: Some(8), access: Some(ReadOnly), reset_value: Some(0), reset_mask: Some(255), description: Some("UART FIFO Receive Count"), fields: [Field { name: "RXCOUNT", bit_offset: 0, bit_width: 8, access: Some(ReadOnly), description: Some("Receive Counter"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "C7816", offset: 24, size: Some(8), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(255), description: Some("UART 7816 Control Register"), fields: [Field { name: "ISO_7816E", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("ISO-7816 Functionality Enabled"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("ISO-7816 functionality is turned off/not enabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("ISO-7816 functionality is turned on/enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TTYPE", bit_offset: 1, bit_width: 1, access: Some(ReadWrite), description: Some("Transfer Type"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("T = 0 per the ISO-7816 specification.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("T = 1 per the ISO-7816 specification.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "INIT", bit_offset: 2, bit_width: 1, access: Some(ReadWrite), description: Some("Detect Initial Character"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Normal operating mode. Receiver does not seek to identify initial character.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Receiver searches for initial character.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ANACK", bit_offset: 3, bit_width: 1, access: Some(ReadWrite), description: Some("Generate NACK on Error"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No NACK is automatically generated.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("A NACK is automatically generated if a parity error is detected or if an invalid initial character is detected.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ONACK", bit_offset: 4, bit_width: 1, access: Some(ReadWrite), description: Some("Generate NACK on Overflow"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The received data does not generate a NACK when the receipt of the data results in an overflow event.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("If the receiver buffer overflows, a NACK is automatically sent on a received character.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "IE7816", offset: 25, size: Some(8), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(255), description: Some("UART 7816 Interrupt Enable Register"), fields: [Field { name: "RXTE", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("Receive Threshold Exceeded Interrupt Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The assertion of IS7816[RXT] does not result in the generation of an interrupt.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The assertion of IS7816[RXT] results in the generation of an interrupt.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TXTE", bit_offset: 1, bit_width: 1, access: Some(ReadWrite), description: Some("Transmit Threshold Exceeded Interrupt Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The assertion of IS7816[TXT] does not result in the generation of an interrupt.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The assertion of IS7816[TXT] results in the generation of an interrupt.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "GTVE", bit_offset: 2, bit_width: 1, access: Some(ReadWrite), description: Some("Guard Timer Violated Interrupt Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The assertion of IS7816[GTV] does not result in the generation of an interrupt.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The assertion of IS7816[GTV] results in the generation of an interrupt.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "INITDE", bit_offset: 4, bit_width: 1, access: Some(ReadWrite), description: Some("Initial Character Detected Interrupt Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The assertion of IS7816[INITD] does not result in the generation of an interrupt.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The assertion of IS7816[INITD] results in the generation of an interrupt.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "BWTE", bit_offset: 5, bit_width: 1, access: Some(ReadWrite), description: Some("Block Wait Timer Interrupt Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The assertion of IS7816[BWT] does not result in the generation of an interrupt.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The assertion of IS7816[BWT] results in the generation of an interrupt.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CWTE", bit_offset: 6, bit_width: 1, access: Some(ReadWrite), description: Some("Character Wait Timer Interrupt Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The assertion of IS7816[CWT] does not result in the generation of an interrupt.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The assertion of IS7816[CWT] results in the generation of an interrupt.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "WTE", bit_offset: 7, bit_width: 1, access: Some(ReadWrite), description: Some("Wait Timer Interrupt Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The assertion of IS7816[WT] does not result in the generation of an interrupt.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The assertion of IS7816[WT] results in the generation of an interrupt.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "IS7816", offset: 26, size: Some(8), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(255), description: Some("UART 7816 Interrupt Status Register"), fields: [Field { name: "RXT", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("Receive Threshold Exceeded Interrupt"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The number of consecutive NACKS generated as a result of parity errors and buffer overruns is less than or equal to the value in ET7816[RXTHRESHOLD].") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The number of consecutive NACKS generated as a result of parity errors and buffer overruns is greater than the value in ET7816[RXTHRESHOLD].") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TXT", bit_offset: 1, bit_width: 1, access: Some(ReadWrite), description: Some("Transmit Threshold Exceeded Interrupt"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The number of retries and corresponding NACKS does not exceed the value in ET7816[TXTHRESHOLD].") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The number of retries and corresponding NACKS exceeds the value in ET7816[TXTHRESHOLD].") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "GTV", bit_offset: 2, bit_width: 1, access: Some(ReadWrite), description: Some("Guard Timer Violated Interrupt"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("A guard time (GT, CGT, or BGT) has not been violated.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("A guard time (GT, CGT, or BGT) has been violated.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "INITD", bit_offset: 4, bit_width: 1, access: Some(ReadWrite), description: Some("Initial Character Detected Interrupt"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("A valid initial character has not been received.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("A valid initial character has been received.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "BWT", bit_offset: 5, bit_width: 1, access: Some(ReadWrite), description: Some("Block Wait Timer Interrupt"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Block wait time (BWT) has not been violated.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Block wait time (BWT) has been violated.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CWT", bit_offset: 6, bit_width: 1, access: Some(ReadWrite), description: Some("Character Wait Timer Interrupt"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Character wait time (CWT) has not been violated.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Character wait time (CWT) has been violated.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "WT", bit_offset: 7, bit_width: 1, access: Some(ReadWrite), description: Some("Wait Timer Interrupt"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Wait time (WT) has not been violated.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Wait time (WT) has been violated.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "WP7816T0", offset: 27, size: Some(8), access: Some(ReadWrite), reset_value: Some(10), reset_mask: Some(255), description: Some("UART 7816 Wait Parameter Register"), fields: [Field { name: "WI", bit_offset: 0, bit_width: 8, access: Some(ReadWrite), description: Some("Wait Time Integer (C7816[TTYPE] = 0)"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "WP7816T1", offset: 27, size: Some(8), access: Some(ReadWrite), reset_value: Some(10), reset_mask: Some(255), description: Some("UART 7816 Wait Parameter Register"), fields: [Field { name: "BWI", bit_offset: 0, bit_width: 4, access: Some(ReadWrite), description: Some("Block Wait Time Integer(C7816[TTYPE] = 1)"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CWI", bit_offset: 4, bit_width: 4, access: Some(ReadWrite), description: Some("Character Wait Time Integer (C7816[TTYPE] = 1)"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "WN7816", offset: 28, size: Some(8), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(255), description: Some("UART 7816 Wait N Register"), fields: [Field { name: "GTN", bit_offset: 0, bit_width: 8, access: Some(ReadWrite), description: Some("Guard Band N"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "WF7816", offset: 29, size: Some(8), access: Some(ReadWrite), reset_value: Some(1), reset_mask: Some(255), description: Some("UART 7816 Wait FD Register"), fields: [Field { name: "GTFD", bit_offset: 0, bit_width: 8, access: Some(ReadWrite), description: Some("FD Multiplier"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "ET7816", offset: 30, size: Some(8), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(255), description: Some("UART 7816 Error Threshold Register"), fields: [Field { name: "RXTHRESHOLD", bit_offset: 0, bit_width: 4, access: Some(ReadWrite), description: Some("Receive NACK Threshold"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TXTHRESHOLD", bit_offset: 4, bit_width: 4, access: Some(ReadWrite), description: Some("Transmit NACK Threshold"), enumerated_values: [EnumeratedValue { value: "#0000", name: Some("0"), description: Some("TXT asserts on the first NACK that is received.") }, EnumeratedValue { value: "#0001", name: Some("1"), description: Some("TXT asserts on the second NACK that is received.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "TL7816", offset: 31, size: Some(8), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(255), description: Some("UART 7816 Transmit Length Register"), fields: [Field { name: "TLEN", bit_offset: 0, bit_width: 8, access: Some(ReadWrite), description: Some("Transmit Length"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }), modules: [], has_pins: false, has_channels: false, description: None }

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="UART Peripheral"]
pub struct UartPeriph(pub usize); 


impl UartPeriph {
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

#[doc="Get the *const pointer for the ED register."]
   #[inline] pub fn ed_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0xc) as *const u8
   }
#[doc="Get the *mut pointer for the ED register."]
   #[inline] pub fn ed_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0xc) as *mut u8
   }
#[doc="Read the ED register."]
   #[inline] pub fn ed(&self) -> Ed { 
      unsafe {
         Ed(::core::ptr::read_volatile((self.0 + 0xc) as *const u8))
      }
   }

#[doc="Get the *const pointer for the MODEM register."]
   #[inline] pub fn modem_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0xd) as *const u8
   }
#[doc="Get the *mut pointer for the MODEM register."]
   #[inline] pub fn modem_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0xd) as *mut u8
   }
#[doc="Read the MODEM register."]
   #[inline] pub fn modem(&self) -> Modem { 
      unsafe {
         Modem(::core::ptr::read_volatile((self.0 + 0xd) as *const u8))
      }
   }
#[doc="Write the MODEM register."]
   #[inline] pub fn set_modem<F: FnOnce(Modem) -> Modem>(&self, f: F) -> &Self {
      let value = f(Modem(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xd) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the MODEM register."]
   #[inline] pub fn with_modem<F: FnOnce(Modem) -> Modem>(&self, f: F) -> &Self {
      let tmp = self.modem();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xd) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the IR register."]
   #[inline] pub fn ir_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0xe) as *const u8
   }
#[doc="Get the *mut pointer for the IR register."]
   #[inline] pub fn ir_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0xe) as *mut u8
   }
#[doc="Read the IR register."]
   #[inline] pub fn ir(&self) -> Ir { 
      unsafe {
         Ir(::core::ptr::read_volatile((self.0 + 0xe) as *const u8))
      }
   }
#[doc="Write the IR register."]
   #[inline] pub fn set_ir<F: FnOnce(Ir) -> Ir>(&self, f: F) -> &Self {
      let value = f(Ir(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xe) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the IR register."]
   #[inline] pub fn with_ir<F: FnOnce(Ir) -> Ir>(&self, f: F) -> &Self {
      let tmp = self.ir();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xe) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PFIFO register."]
   #[inline] pub fn pfifo_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x10) as *const u8
   }
#[doc="Get the *mut pointer for the PFIFO register."]
   #[inline] pub fn pfifo_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x10) as *mut u8
   }
#[doc="Read the PFIFO register."]
   #[inline] pub fn pfifo(&self) -> Pfifo { 
      unsafe {
         Pfifo(::core::ptr::read_volatile((self.0 + 0x10) as *const u8))
      }
   }
#[doc="Write the PFIFO register."]
   #[inline] pub fn set_pfifo<F: FnOnce(Pfifo) -> Pfifo>(&self, f: F) -> &Self {
      let value = f(Pfifo(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x10) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the PFIFO register."]
   #[inline] pub fn with_pfifo<F: FnOnce(Pfifo) -> Pfifo>(&self, f: F) -> &Self {
      let tmp = self.pfifo();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x10) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CFIFO register."]
   #[inline] pub fn cfifo_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x11) as *const u8
   }
#[doc="Get the *mut pointer for the CFIFO register."]
   #[inline] pub fn cfifo_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x11) as *mut u8
   }
#[doc="Read the CFIFO register."]
   #[inline] pub fn cfifo(&self) -> Cfifo { 
      unsafe {
         Cfifo(::core::ptr::read_volatile((self.0 + 0x11) as *const u8))
      }
   }
#[doc="Write the CFIFO register."]
   #[inline] pub fn set_cfifo<F: FnOnce(Cfifo) -> Cfifo>(&self, f: F) -> &Self {
      let value = f(Cfifo(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x11) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the CFIFO register."]
   #[inline] pub fn with_cfifo<F: FnOnce(Cfifo) -> Cfifo>(&self, f: F) -> &Self {
      let tmp = self.cfifo();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x11) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SFIFO register."]
   #[inline] pub fn sfifo_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x12) as *const u8
   }
#[doc="Get the *mut pointer for the SFIFO register."]
   #[inline] pub fn sfifo_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x12) as *mut u8
   }
#[doc="Read the SFIFO register."]
   #[inline] pub fn sfifo(&self) -> Sfifo { 
      unsafe {
         Sfifo(::core::ptr::read_volatile((self.0 + 0x12) as *const u8))
      }
   }
#[doc="Write the SFIFO register."]
   #[inline] pub fn set_sfifo<F: FnOnce(Sfifo) -> Sfifo>(&self, f: F) -> &Self {
      let value = f(Sfifo(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x12) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the SFIFO register."]
   #[inline] pub fn with_sfifo<F: FnOnce(Sfifo) -> Sfifo>(&self, f: F) -> &Self {
      let tmp = self.sfifo();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x12) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the TWFIFO register."]
   #[inline] pub fn twfifo_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x13) as *const u8
   }
#[doc="Get the *mut pointer for the TWFIFO register."]
   #[inline] pub fn twfifo_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x13) as *mut u8
   }
#[doc="Read the TWFIFO register."]
   #[inline] pub fn twfifo(&self) -> Twfifo { 
      unsafe {
         Twfifo(::core::ptr::read_volatile((self.0 + 0x13) as *const u8))
      }
   }
#[doc="Write the TWFIFO register."]
   #[inline] pub fn set_twfifo<F: FnOnce(Twfifo) -> Twfifo>(&self, f: F) -> &Self {
      let value = f(Twfifo(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x13) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the TWFIFO register."]
   #[inline] pub fn with_twfifo<F: FnOnce(Twfifo) -> Twfifo>(&self, f: F) -> &Self {
      let tmp = self.twfifo();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x13) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the TCFIFO register."]
   #[inline] pub fn tcfifo_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x14) as *const u8
   }
#[doc="Get the *mut pointer for the TCFIFO register."]
   #[inline] pub fn tcfifo_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x14) as *mut u8
   }
#[doc="Read the TCFIFO register."]
   #[inline] pub fn tcfifo(&self) -> Tcfifo { 
      unsafe {
         Tcfifo(::core::ptr::read_volatile((self.0 + 0x14) as *const u8))
      }
   }

#[doc="Get the *const pointer for the RWFIFO register."]
   #[inline] pub fn rwfifo_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x15) as *const u8
   }
#[doc="Get the *mut pointer for the RWFIFO register."]
   #[inline] pub fn rwfifo_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x15) as *mut u8
   }
#[doc="Read the RWFIFO register."]
   #[inline] pub fn rwfifo(&self) -> Rwfifo { 
      unsafe {
         Rwfifo(::core::ptr::read_volatile((self.0 + 0x15) as *const u8))
      }
   }
#[doc="Write the RWFIFO register."]
   #[inline] pub fn set_rwfifo<F: FnOnce(Rwfifo) -> Rwfifo>(&self, f: F) -> &Self {
      let value = f(Rwfifo(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x15) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the RWFIFO register."]
   #[inline] pub fn with_rwfifo<F: FnOnce(Rwfifo) -> Rwfifo>(&self, f: F) -> &Self {
      let tmp = self.rwfifo();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x15) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the RCFIFO register."]
   #[inline] pub fn rcfifo_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x16) as *const u8
   }
#[doc="Get the *mut pointer for the RCFIFO register."]
   #[inline] pub fn rcfifo_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x16) as *mut u8
   }
#[doc="Read the RCFIFO register."]
   #[inline] pub fn rcfifo(&self) -> Rcfifo { 
      unsafe {
         Rcfifo(::core::ptr::read_volatile((self.0 + 0x16) as *const u8))
      }
   }

#[doc="Get the *const pointer for the C7816 register."]
   #[inline] pub fn c7816_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x18) as *const u8
   }
#[doc="Get the *mut pointer for the C7816 register."]
   #[inline] pub fn c7816_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x18) as *mut u8
   }
#[doc="Read the C7816 register."]
   #[inline] pub fn c7816(&self) -> C7816 { 
      unsafe {
         C7816(::core::ptr::read_volatile((self.0 + 0x18) as *const u8))
      }
   }
#[doc="Write the C7816 register."]
   #[inline] pub fn set_c7816<F: FnOnce(C7816) -> C7816>(&self, f: F) -> &Self {
      let value = f(C7816(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x18) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the C7816 register."]
   #[inline] pub fn with_c7816<F: FnOnce(C7816) -> C7816>(&self, f: F) -> &Self {
      let tmp = self.c7816();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x18) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the IE7816 register."]
   #[inline] pub fn ie7816_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x19) as *const u8
   }
#[doc="Get the *mut pointer for the IE7816 register."]
   #[inline] pub fn ie7816_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x19) as *mut u8
   }
#[doc="Read the IE7816 register."]
   #[inline] pub fn ie7816(&self) -> Ie7816 { 
      unsafe {
         Ie7816(::core::ptr::read_volatile((self.0 + 0x19) as *const u8))
      }
   }
#[doc="Write the IE7816 register."]
   #[inline] pub fn set_ie7816<F: FnOnce(Ie7816) -> Ie7816>(&self, f: F) -> &Self {
      let value = f(Ie7816(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x19) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the IE7816 register."]
   #[inline] pub fn with_ie7816<F: FnOnce(Ie7816) -> Ie7816>(&self, f: F) -> &Self {
      let tmp = self.ie7816();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x19) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the IS7816 register."]
   #[inline] pub fn is7816_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x1a) as *const u8
   }
#[doc="Get the *mut pointer for the IS7816 register."]
   #[inline] pub fn is7816_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x1a) as *mut u8
   }
#[doc="Read the IS7816 register."]
   #[inline] pub fn is7816(&self) -> Is7816 { 
      unsafe {
         Is7816(::core::ptr::read_volatile((self.0 + 0x1a) as *const u8))
      }
   }
#[doc="Write the IS7816 register."]
   #[inline] pub fn set_is7816<F: FnOnce(Is7816) -> Is7816>(&self, f: F) -> &Self {
      let value = f(Is7816(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1a) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the IS7816 register."]
   #[inline] pub fn with_is7816<F: FnOnce(Is7816) -> Is7816>(&self, f: F) -> &Self {
      let tmp = self.is7816();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1a) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the WP7816T0 register."]
   #[inline] pub fn wp7816t0_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x1b) as *const u8
   }
#[doc="Get the *mut pointer for the WP7816T0 register."]
   #[inline] pub fn wp7816t0_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x1b) as *mut u8
   }
#[doc="Read the WP7816T0 register."]
   #[inline] pub fn wp7816t0(&self) -> Wp7816t0 { 
      unsafe {
         Wp7816t0(::core::ptr::read_volatile((self.0 + 0x1b) as *const u8))
      }
   }
#[doc="Write the WP7816T0 register."]
   #[inline] pub fn set_wp7816t0<F: FnOnce(Wp7816t0) -> Wp7816t0>(&self, f: F) -> &Self {
      let value = f(Wp7816t0(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1b) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the WP7816T0 register."]
   #[inline] pub fn with_wp7816t0<F: FnOnce(Wp7816t0) -> Wp7816t0>(&self, f: F) -> &Self {
      let tmp = self.wp7816t0();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1b) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the WP7816T1 register."]
   #[inline] pub fn wp7816t1_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x1b) as *const u8
   }
#[doc="Get the *mut pointer for the WP7816T1 register."]
   #[inline] pub fn wp7816t1_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x1b) as *mut u8
   }
#[doc="Read the WP7816T1 register."]
   #[inline] pub fn wp7816t1(&self) -> Wp7816t1 { 
      unsafe {
         Wp7816t1(::core::ptr::read_volatile((self.0 + 0x1b) as *const u8))
      }
   }
#[doc="Write the WP7816T1 register."]
   #[inline] pub fn set_wp7816t1<F: FnOnce(Wp7816t1) -> Wp7816t1>(&self, f: F) -> &Self {
      let value = f(Wp7816t1(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1b) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the WP7816T1 register."]
   #[inline] pub fn with_wp7816t1<F: FnOnce(Wp7816t1) -> Wp7816t1>(&self, f: F) -> &Self {
      let tmp = self.wp7816t1();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1b) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the WN7816 register."]
   #[inline] pub fn wn7816_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x1c) as *const u8
   }
#[doc="Get the *mut pointer for the WN7816 register."]
   #[inline] pub fn wn7816_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x1c) as *mut u8
   }
#[doc="Read the WN7816 register."]
   #[inline] pub fn wn7816(&self) -> Wn7816 { 
      unsafe {
         Wn7816(::core::ptr::read_volatile((self.0 + 0x1c) as *const u8))
      }
   }
#[doc="Write the WN7816 register."]
   #[inline] pub fn set_wn7816<F: FnOnce(Wn7816) -> Wn7816>(&self, f: F) -> &Self {
      let value = f(Wn7816(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1c) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the WN7816 register."]
   #[inline] pub fn with_wn7816<F: FnOnce(Wn7816) -> Wn7816>(&self, f: F) -> &Self {
      let tmp = self.wn7816();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1c) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the WF7816 register."]
   #[inline] pub fn wf7816_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x1d) as *const u8
   }
#[doc="Get the *mut pointer for the WF7816 register."]
   #[inline] pub fn wf7816_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x1d) as *mut u8
   }
#[doc="Read the WF7816 register."]
   #[inline] pub fn wf7816(&self) -> Wf7816 { 
      unsafe {
         Wf7816(::core::ptr::read_volatile((self.0 + 0x1d) as *const u8))
      }
   }
#[doc="Write the WF7816 register."]
   #[inline] pub fn set_wf7816<F: FnOnce(Wf7816) -> Wf7816>(&self, f: F) -> &Self {
      let value = f(Wf7816(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1d) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the WF7816 register."]
   #[inline] pub fn with_wf7816<F: FnOnce(Wf7816) -> Wf7816>(&self, f: F) -> &Self {
      let tmp = self.wf7816();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1d) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the ET7816 register."]
   #[inline] pub fn et7816_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x1e) as *const u8
   }
#[doc="Get the *mut pointer for the ET7816 register."]
   #[inline] pub fn et7816_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x1e) as *mut u8
   }
#[doc="Read the ET7816 register."]
   #[inline] pub fn et7816(&self) -> Et7816 { 
      unsafe {
         Et7816(::core::ptr::read_volatile((self.0 + 0x1e) as *const u8))
      }
   }
#[doc="Write the ET7816 register."]
   #[inline] pub fn set_et7816<F: FnOnce(Et7816) -> Et7816>(&self, f: F) -> &Self {
      let value = f(Et7816(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1e) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the ET7816 register."]
   #[inline] pub fn with_et7816<F: FnOnce(Et7816) -> Et7816>(&self, f: F) -> &Self {
      let tmp = self.et7816();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1e) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the TL7816 register."]
   #[inline] pub fn tl7816_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x1f) as *const u8
   }
#[doc="Get the *mut pointer for the TL7816 register."]
   #[inline] pub fn tl7816_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x1f) as *mut u8
   }
#[doc="Read the TL7816 register."]
   #[inline] pub fn tl7816(&self) -> Tl7816 { 
      unsafe {
         Tl7816(::core::ptr::read_volatile((self.0 + 0x1f) as *const u8))
      }
   }
#[doc="Write the TL7816 register."]
   #[inline] pub fn set_tl7816<F: FnOnce(Tl7816) -> Tl7816>(&self, f: F) -> &Self {
      let value = f(Tl7816(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1f) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the TL7816 register."]
   #[inline] pub fn with_tl7816<F: FnOnce(Tl7816) -> Tl7816>(&self, f: F) -> &Self {
      let tmp = self.tl7816();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1f) as *mut u8, value.0);
      }
      self
   }

}

#[doc="UART Baud Rate Registers: High"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Bdh(pub u8);
impl Bdh {
#[doc="UART Baud Rate Bits"]
   #[inline] pub fn sbr(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
   }
#[doc="UART Baud Rate Bits"]
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

#[doc="RxD Input Active Edge Interrupt Enable"]
   #[inline] pub fn rxedgie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="RxD Input Active Edge Interrupt Enable"]
   #[inline] pub fn set_rxedgie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="LIN Break Detect Interrupt or DMA Request Enable"]
   #[inline] pub fn lbkdie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="LIN Break Detect Interrupt or DMA Request Enable"]
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
#[doc="UART Baud Rate Registers: Low"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Bdl(pub u8);
impl Bdl {
#[doc="UART Baud Rate Bits"]
   #[inline] pub fn sbr(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="UART Baud Rate Bits"]
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

#[doc="9-bit or 8-bit Mode Select"]
   #[inline] pub fn m(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="9-bit or 8-bit Mode Select"]
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

#[doc="UART Stops in Wait Mode"]
   #[inline] pub fn uartswai(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="UART Stops in Wait Mode"]
   #[inline] pub fn set_uartswai<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
      if self.uartswai() != 0 { try!(write!(f, " uartswai"))}
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

#[doc="Idle Line Interrupt DMA Transfer Enable"]
   #[inline] pub fn ilie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Idle Line Interrupt DMA Transfer Enable"]
   #[inline] pub fn set_ilie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Receiver Full Interrupt or DMA Transfer Enable"]
   #[inline] pub fn rie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Receiver Full Interrupt or DMA Transfer Enable"]
   #[inline] pub fn set_rie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Transmission Complete Interrupt or DMA Transfer Enable"]
   #[inline] pub fn tcie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Transmission Complete Interrupt or DMA Transfer Enable"]
   #[inline] pub fn set_tcie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Transmitter Interrupt or DMA Transfer Enable."]
   #[inline] pub fn tie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Transmitter Interrupt or DMA Transfer Enable."]
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

#[doc="Transmit Complete Flag"]
   #[inline] pub fn tc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Transmit Complete Flag"]
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

#[doc="Break Transmit Character Length"]
   #[inline] pub fn brk13(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Break Transmit Character Length"]
   #[inline] pub fn set_brk13<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Receive Wakeup Idle Detect"]
   #[inline] pub fn rwuid(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Receive Wakeup Idle Detect"]
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

#[doc="Most Significant Bit First"]
   #[inline] pub fn msbf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Most Significant Bit First"]
   #[inline] pub fn set_msbf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="RxD Pin Active Edge Interrupt Flag"]
   #[inline] pub fn rxedgif(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="RxD Pin Active Edge Interrupt Flag"]
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

#[doc="Overrun Error Interrupt Enable"]
   #[inline] pub fn orie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Overrun Error Interrupt Enable"]
   #[inline] pub fn set_orie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Transmit Data Inversion."]
   #[inline] pub fn txinv(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Transmit Data Inversion."]
   #[inline] pub fn set_txinv<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Transmitter Pin Data Direction in Single-Wire mode"]
   #[inline] pub fn txdir(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Transmitter Pin Data Direction in Single-Wire mode"]
   #[inline] pub fn set_txdir<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Transmit Bit 8"]
   #[inline] pub fn t8(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Transmit Bit 8"]
   #[inline] pub fn set_t8<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Received Bit 8"]
   #[inline] pub fn r8(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Received Bit 8"]
   #[inline] pub fn set_r8<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
      if self.t8() != 0 { try!(write!(f, " t8"))}
      if self.r8() != 0 { try!(write!(f, " r8"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART Data Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct D(pub u8);
impl D {
#[doc="Reads return the contents of the read-only receive data register and writes go to the write-only transmit data register"]
   #[inline] pub fn rt(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Reads return the contents of the read-only receive data register and writes go to the write-only transmit data register"]
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
#[doc="Baud Rate Fine Adjust"]
   #[inline] pub fn brfa(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
   }
#[doc="Baud Rate Fine Adjust"]
   #[inline] pub fn set_brfa<V: Into<bits::U5>>(mut self, value: V) -> Self {
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
      if self.brfa() != 0 { try!(write!(f, " brfa=0x{:x}", self.brfa()))}
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
#[doc="LIN Break Detect DMA Select Bit"]
   #[inline] pub fn lbkddmas(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="LIN Break Detect DMA Select Bit"]
   #[inline] pub fn set_lbkddmas<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Idle Line DMA Select"]
   #[inline] pub fn ildmas(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Idle Line DMA Select"]
   #[inline] pub fn set_ildmas<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Receiver Full DMA Select"]
   #[inline] pub fn rdmas(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Receiver Full DMA Select"]
   #[inline] pub fn set_rdmas<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Transmission Complete DMA Select"]
   #[inline] pub fn tcdmas(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Transmission Complete DMA Select"]
   #[inline] pub fn set_tcdmas<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Transmitter DMA Select"]
   #[inline] pub fn tdmas(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Transmitter DMA Select"]
   #[inline] pub fn set_tdmas<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
      if self.lbkddmas() != 0 { try!(write!(f, " lbkddmas"))}
      if self.ildmas() != 0 { try!(write!(f, " ildmas"))}
      if self.rdmas() != 0 { try!(write!(f, " rdmas"))}
      if self.tcdmas() != 0 { try!(write!(f, " tcdmas"))}
      if self.tdmas() != 0 { try!(write!(f, " tdmas"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART Extended Data Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ed(pub u8);
impl Ed {
#[doc="The current received dataword contained in D and C3[R8] was received with a parity error."]
   #[inline] pub fn paritye(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="The current received dataword contained in D and C3[R8] was received with a parity error."]
   #[inline] pub fn set_paritye<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="The current received dataword contained in D and C3[R8] was received with noise."]
   #[inline] pub fn noisy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="The current received dataword contained in D and C3[R8] was received with noise."]
   #[inline] pub fn set_noisy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Ed {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ed {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.paritye() != 0 { try!(write!(f, " paritye"))}
      if self.noisy() != 0 { try!(write!(f, " noisy"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART Modem Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Modem(pub u8);
impl Modem {
#[doc="Transmitter clear-to-send enable"]
   #[inline] pub fn txctse(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Transmitter clear-to-send enable"]
   #[inline] pub fn set_txctse<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
      let value: u8 = value.into();
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
      let value: u8 = value.into();
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
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

}
impl ::core::fmt::Display for Modem {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Modem {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.txctse() != 0 { try!(write!(f, " txctse"))}
      if self.txrtse() != 0 { try!(write!(f, " txrtse"))}
      if self.txrtspol() != 0 { try!(write!(f, " txrtspol"))}
      if self.rxrtse() != 0 { try!(write!(f, " rxrtse"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART Infrared Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ir(pub u8);
impl Ir {
#[doc="Transmitter narrow pulse"]
   #[inline] pub fn tnp(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
   }
#[doc="Transmitter narrow pulse"]
   #[inline] pub fn set_tnp<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x3 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Infrared enable"]
   #[inline] pub fn iren(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Infrared enable"]
   #[inline] pub fn set_iren<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

}
impl ::core::fmt::Display for Ir {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ir {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tnp() != 0 { try!(write!(f, " tnp=0x{:x}", self.tnp()))}
      if self.iren() != 0 { try!(write!(f, " iren"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART FIFO Parameters"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pfifo(pub u8);
impl Pfifo {
#[doc="Receive FIFO. Buffer Depth"]
   #[inline] pub fn rxfifosize(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
   }
#[doc="Receive FIFO. Buffer Depth"]
   #[inline] pub fn set_rxfifosize<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u8 = value.into();
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
      let value: u8 = value.into();
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
      let value: u8 = value.into();
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
      let value: u8 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Pfifo {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pfifo {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rxfifosize() != 0 { try!(write!(f, " rxfifosize=0x{:x}", self.rxfifosize()))}
      if self.rxfe() != 0 { try!(write!(f, " rxfe"))}
      if self.txfifosize() != 0 { try!(write!(f, " txfifosize=0x{:x}", self.txfifosize()))}
      if self.txfe() != 0 { try!(write!(f, " txfe"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART FIFO Control Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cfifo(pub u8);
impl Cfifo {
#[doc="Receive FIFO Underflow Interrupt Enable"]
   #[inline] pub fn rxufe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Receive FIFO Underflow Interrupt Enable"]
   #[inline] pub fn set_rxufe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Transmit FIFO Overflow Interrupt Enable"]
   #[inline] pub fn txofe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Transmit FIFO Overflow Interrupt Enable"]
   #[inline] pub fn set_txofe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Receive FIFO Overflow Interrupt Enable"]
   #[inline] pub fn rxofe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Receive FIFO Overflow Interrupt Enable"]
   #[inline] pub fn set_rxofe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Receive FIFO/Buffer Flush"]
   #[inline] pub fn rxflush(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Receive FIFO/Buffer Flush"]
   #[inline] pub fn set_rxflush<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Transmit FIFO/Buffer Flush"]
   #[inline] pub fn txflush(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Transmit FIFO/Buffer Flush"]
   #[inline] pub fn set_txflush<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Cfifo {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cfifo {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rxufe() != 0 { try!(write!(f, " rxufe"))}
      if self.txofe() != 0 { try!(write!(f, " txofe"))}
      if self.rxofe() != 0 { try!(write!(f, " rxofe"))}
      if self.rxflush() != 0 { try!(write!(f, " rxflush"))}
      if self.txflush() != 0 { try!(write!(f, " txflush"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART FIFO Status Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sfifo(pub u8);
impl Sfifo {
#[doc="Receiver Buffer Underflow Flag"]
   #[inline] pub fn rxuf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Receiver Buffer Underflow Flag"]
   #[inline] pub fn set_rxuf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Transmitter Buffer Overflow Flag"]
   #[inline] pub fn txof(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Transmitter Buffer Overflow Flag"]
   #[inline] pub fn set_txof<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Receiver Buffer Overflow Flag"]
   #[inline] pub fn rxof(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Receiver Buffer Overflow Flag"]
   #[inline] pub fn set_rxof<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Receive Buffer/FIFO Empty"]
   #[inline] pub fn rxempt(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Receive Buffer/FIFO Empty"]
   #[inline] pub fn set_rxempt<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Transmit Buffer/FIFO Empty"]
   #[inline] pub fn txempt(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Transmit Buffer/FIFO Empty"]
   #[inline] pub fn set_txempt<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Sfifo {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sfifo {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rxuf() != 0 { try!(write!(f, " rxuf"))}
      if self.txof() != 0 { try!(write!(f, " txof"))}
      if self.rxof() != 0 { try!(write!(f, " rxof"))}
      if self.rxempt() != 0 { try!(write!(f, " rxempt"))}
      if self.txempt() != 0 { try!(write!(f, " txempt"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART FIFO Transmit Watermark"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Twfifo(pub u8);
impl Twfifo {
#[doc="Transmit Watermark"]
   #[inline] pub fn txwater(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Transmit Watermark"]
   #[inline] pub fn set_txwater<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Twfifo {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Twfifo {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.txwater() != 0 { try!(write!(f, " txwater=0x{:x}", self.txwater()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART FIFO Transmit Count"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tcfifo(pub u8);
impl Tcfifo {
#[doc="Transmit Counter"]
   #[inline] pub fn txcount(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Transmit Counter"]
   #[inline] pub fn set_txcount<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Tcfifo {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tcfifo {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.txcount() != 0 { try!(write!(f, " txcount=0x{:x}", self.txcount()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART FIFO Receive Watermark"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rwfifo(pub u8);
impl Rwfifo {
#[doc="Receive Watermark"]
   #[inline] pub fn rxwater(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Receive Watermark"]
   #[inline] pub fn set_rxwater<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Rwfifo {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rwfifo {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rxwater() != 0 { try!(write!(f, " rxwater=0x{:x}", self.rxwater()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART FIFO Receive Count"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rcfifo(pub u8);
impl Rcfifo {
#[doc="Receive Counter"]
   #[inline] pub fn rxcount(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Receive Counter"]
   #[inline] pub fn set_rxcount<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Rcfifo {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rcfifo {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rxcount() != 0 { try!(write!(f, " rxcount=0x{:x}", self.rxcount()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART 7816 Control Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct C7816(pub u8);
impl C7816 {
#[doc="ISO-7816 Functionality Enabled"]
   #[inline] pub fn iso_7816e(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="ISO-7816 Functionality Enabled"]
   #[inline] pub fn set_iso_7816e<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Transfer Type"]
   #[inline] pub fn ttype(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Transfer Type"]
   #[inline] pub fn set_ttype<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Detect Initial Character"]
   #[inline] pub fn init(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Detect Initial Character"]
   #[inline] pub fn set_init<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Generate NACK on Error"]
   #[inline] pub fn anack(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Generate NACK on Error"]
   #[inline] pub fn set_anack<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Generate NACK on Overflow"]
   #[inline] pub fn onack(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Generate NACK on Overflow"]
   #[inline] pub fn set_onack<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

}
impl ::core::fmt::Display for C7816 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for C7816 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.iso_7816e() != 0 { try!(write!(f, " iso_7816e"))}
      if self.ttype() != 0 { try!(write!(f, " ttype"))}
      if self.init() != 0 { try!(write!(f, " init"))}
      if self.anack() != 0 { try!(write!(f, " anack"))}
      if self.onack() != 0 { try!(write!(f, " onack"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART 7816 Interrupt Enable Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ie7816(pub u8);
impl Ie7816 {
#[doc="Receive Threshold Exceeded Interrupt Enable"]
   #[inline] pub fn rxte(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Receive Threshold Exceeded Interrupt Enable"]
   #[inline] pub fn set_rxte<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Transmit Threshold Exceeded Interrupt Enable"]
   #[inline] pub fn txte(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Transmit Threshold Exceeded Interrupt Enable"]
   #[inline] pub fn set_txte<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Guard Timer Violated Interrupt Enable"]
   #[inline] pub fn gtve(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Guard Timer Violated Interrupt Enable"]
   #[inline] pub fn set_gtve<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Initial Character Detected Interrupt Enable"]
   #[inline] pub fn initde(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Initial Character Detected Interrupt Enable"]
   #[inline] pub fn set_initde<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Block Wait Timer Interrupt Enable"]
   #[inline] pub fn bwte(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Block Wait Timer Interrupt Enable"]
   #[inline] pub fn set_bwte<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Character Wait Timer Interrupt Enable"]
   #[inline] pub fn cwte(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Character Wait Timer Interrupt Enable"]
   #[inline] pub fn set_cwte<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Wait Timer Interrupt Enable"]
   #[inline] pub fn wte(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Wait Timer Interrupt Enable"]
   #[inline] pub fn set_wte<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Ie7816 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ie7816 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rxte() != 0 { try!(write!(f, " rxte"))}
      if self.txte() != 0 { try!(write!(f, " txte"))}
      if self.gtve() != 0 { try!(write!(f, " gtve"))}
      if self.initde() != 0 { try!(write!(f, " initde"))}
      if self.bwte() != 0 { try!(write!(f, " bwte"))}
      if self.cwte() != 0 { try!(write!(f, " cwte"))}
      if self.wte() != 0 { try!(write!(f, " wte"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART 7816 Interrupt Status Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Is7816(pub u8);
impl Is7816 {
#[doc="Receive Threshold Exceeded Interrupt"]
   #[inline] pub fn rxt(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Receive Threshold Exceeded Interrupt"]
   #[inline] pub fn set_rxt<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Transmit Threshold Exceeded Interrupt"]
   #[inline] pub fn txt(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Transmit Threshold Exceeded Interrupt"]
   #[inline] pub fn set_txt<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Guard Timer Violated Interrupt"]
   #[inline] pub fn gtv(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Guard Timer Violated Interrupt"]
   #[inline] pub fn set_gtv<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Initial Character Detected Interrupt"]
   #[inline] pub fn initd(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Initial Character Detected Interrupt"]
   #[inline] pub fn set_initd<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Block Wait Timer Interrupt"]
   #[inline] pub fn bwt(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Block Wait Timer Interrupt"]
   #[inline] pub fn set_bwt<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Character Wait Timer Interrupt"]
   #[inline] pub fn cwt(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Character Wait Timer Interrupt"]
   #[inline] pub fn set_cwt<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Wait Timer Interrupt"]
   #[inline] pub fn wt(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Wait Timer Interrupt"]
   #[inline] pub fn set_wt<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Is7816 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Is7816 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rxt() != 0 { try!(write!(f, " rxt"))}
      if self.txt() != 0 { try!(write!(f, " txt"))}
      if self.gtv() != 0 { try!(write!(f, " gtv"))}
      if self.initd() != 0 { try!(write!(f, " initd"))}
      if self.bwt() != 0 { try!(write!(f, " bwt"))}
      if self.cwt() != 0 { try!(write!(f, " cwt"))}
      if self.wt() != 0 { try!(write!(f, " wt"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART 7816 Wait Parameter Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Wp7816t0(pub u8);
impl Wp7816t0 {
#[doc="Wait Time Integer (C7816[TTYPE] = 0)"]
   #[inline] pub fn wi(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Wait Time Integer (C7816[TTYPE] = 0)"]
   #[inline] pub fn set_wi<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Wp7816t0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Wp7816t0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.wi() != 0 { try!(write!(f, " wi=0x{:x}", self.wi()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART 7816 Wait Parameter Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Wp7816t1(pub u8);
impl Wp7816t1 {
#[doc="Block Wait Time Integer(C7816[TTYPE] = 1)"]
   #[inline] pub fn bwi(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
   }
#[doc="Block Wait Time Integer(C7816[TTYPE] = 1)"]
   #[inline] pub fn set_bwi<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xf << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Character Wait Time Integer (C7816[TTYPE] = 1)"]
   #[inline] pub fn cwi(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
   }
#[doc="Character Wait Time Integer (C7816[TTYPE] = 1)"]
   #[inline] pub fn set_cwi<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xf << 4);
      self.0 |= value << 4;
      self
   }

}
impl ::core::fmt::Display for Wp7816t1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Wp7816t1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.bwi() != 0 { try!(write!(f, " bwi=0x{:x}", self.bwi()))}
      if self.cwi() != 0 { try!(write!(f, " cwi=0x{:x}", self.cwi()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART 7816 Wait N Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Wn7816(pub u8);
impl Wn7816 {
#[doc="Guard Band N"]
   #[inline] pub fn gtn(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Guard Band N"]
   #[inline] pub fn set_gtn<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Wn7816 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Wn7816 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.gtn() != 0 { try!(write!(f, " gtn=0x{:x}", self.gtn()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART 7816 Wait FD Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Wf7816(pub u8);
impl Wf7816 {
#[doc="FD Multiplier"]
   #[inline] pub fn gtfd(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="FD Multiplier"]
   #[inline] pub fn set_gtfd<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Wf7816 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Wf7816 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.gtfd() != 0 { try!(write!(f, " gtfd=0x{:x}", self.gtfd()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART 7816 Error Threshold Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Et7816(pub u8);
impl Et7816 {
#[doc="Receive NACK Threshold"]
   #[inline] pub fn rxthreshold(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
   }
#[doc="Receive NACK Threshold"]
   #[inline] pub fn set_rxthreshold<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xf << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Transmit NACK Threshold"]
   #[inline] pub fn txthreshold(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
   }
#[doc="Transmit NACK Threshold"]
   #[inline] pub fn set_txthreshold<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xf << 4);
      self.0 |= value << 4;
      self
   }

}
impl ::core::fmt::Display for Et7816 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Et7816 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rxthreshold() != 0 { try!(write!(f, " rxthreshold=0x{:x}", self.rxthreshold()))}
      if self.txthreshold() != 0 { try!(write!(f, " txthreshold=0x{:x}", self.txthreshold()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART 7816 Transmit Length Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tl7816(pub u8);
impl Tl7816 {
#[doc="Transmit Length"]
   #[inline] pub fn tlen(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Transmit Length"]
   #[inline] pub fn set_tlen<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Tl7816 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tl7816 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tlen() != 0 { try!(write!(f, " tlen=0x{:x}", self.tlen()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
