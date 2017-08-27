#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "EDMA", peripherals: [], prototype: Some(Peripheral { derived_from: None, group_name: Some("EDMA"), name: "", address: 0, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [], clusters: [], registers: [Register { name: "CR", offset: 0, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Control Register"), fields: [Field { name: "EDBG", bit_offset: 1, bit_width: 1, access: Some(ReadWrite), description: Some("Enable Debug"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("When in debug mode, the DMA continues to operate.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("When in debug mode, the DMA stalls the start of a new channel. Executing channels are allowed to complete. Channel execution resumes when the system exits debug mode or the EDBG bit is cleared.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ERCA", bit_offset: 2, bit_width: 1, access: Some(ReadWrite), description: Some("Enable Round Robin Channel Arbitration"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Fixed priority arbitration is used for channel selection .") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Round robin arbitration is used for channel selection .") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "HOE", bit_offset: 4, bit_width: 1, access: Some(ReadWrite), description: Some("Halt On Error"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Normal operation") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Any error causes the HALT bit to set. Subsequently, all service requests are ignored until the HALT bit is cleared.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "HALT", bit_offset: 5, bit_width: 1, access: Some(ReadWrite), description: Some("Halt DMA Operations"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Normal operation") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Stall the start of any new channels. Executing channels are allowed to complete. Channel execution resumes when this bit is cleared.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CLM", bit_offset: 6, bit_width: 1, access: Some(ReadWrite), description: Some("Continuous Link Mode"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("A minor loop channel link made to itself goes through channel arbitration before being activated again.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("A minor loop channel link made to itself does not go through channel arbitration before being activated again. Upon minor loop completion, the channel activates again if that channel has a minor loop channel link enabled and the link channel is itself. This effectively applies the minor loop offsets and restarts the next minor loop.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "EMLM", bit_offset: 7, bit_width: 1, access: Some(ReadWrite), description: Some("Enable Minor Loop Mapping"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Disabled. TCDn.word2 is defined as a 32-bit NBYTES field.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Enabled. TCDn.word2 is redefined to include individual enable fields, an offset field, and the NBYTES field. The individual enable fields allow the minor loop offset to be applied to the source address, the destination address, or both. The NBYTES field is reduced when either offset is enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ECX", bit_offset: 16, bit_width: 1, access: Some(ReadWrite), description: Some("Error Cancel Transfer"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Normal operation") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Cancel the remaining data transfer in the same fashion as the CX bit. Stop the executing channel and force the minor loop to finish. The cancel takes effect after the last write of the current read/write sequence. The ECX bit clears itself after the cancel is honored. In addition to cancelling the transfer, ECX treats the cancel as an error condition, thus updating the Error Status register (DMAx_ES) and generating an optional error interrupt.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CX", bit_offset: 17, bit_width: 1, access: Some(ReadWrite), description: Some("Cancel Transfer"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Normal operation") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Cancel the remaining data transfer. Stop the executing channel and force the minor loop to finish. The cancel takes effect after the last write of the current read/write sequence. The CX bit clears itself after the cancel has been honored. This cancel retires the channel normally as if the minor loop was completed.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "ES", offset: 4, size: Some(32), access: Some(ReadOnly), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Error Status Register"), fields: [Field { name: "DBE", bit_offset: 0, bit_width: 1, access: Some(ReadOnly), description: Some("Destination Bus Error"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No destination bus error") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The last recorded error was a bus error on a destination write") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SBE", bit_offset: 1, bit_width: 1, access: Some(ReadOnly), description: Some("Source Bus Error"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No source bus error") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The last recorded error was a bus error on a source read") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SGE", bit_offset: 2, bit_width: 1, access: Some(ReadOnly), description: Some("Scatter/Gather Configuration Error"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No scatter/gather configuration error") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The last recorded error was a configuration error detected in the TCDn_DLASTSGA field. This field is checked at the beginning of a scatter/gather operation after major loop completion if TCDn_CSR[ESG] is enabled. TCDn_DLASTSGA is not on a 32 byte boundary.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "NCE", bit_offset: 3, bit_width: 1, access: Some(ReadOnly), description: Some("NBYTES/CITER Configuration Error"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No NBYTES/CITER configuration error") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The last recorded error was a configuration error detected in the TCDn_NBYTES or TCDn_CITER fields. TCDn_NBYTES is not a multiple of TCDn_ATTR[SSIZE] and TCDn_ATTR[DSIZE], or TCDn_CITER[CITER] is equal to zero, or TCDn_CITER[ELINK] is not equal to TCDn_BITER[ELINK]") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "DOE", bit_offset: 4, bit_width: 1, access: Some(ReadOnly), description: Some("Destination Offset Error"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No destination offset configuration error") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The last recorded error was a configuration error detected in the TCDn_DOFF field. TCDn_DOFF is inconsistent with TCDn_ATTR[DSIZE].") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "DAE", bit_offset: 5, bit_width: 1, access: Some(ReadOnly), description: Some("Destination Address Error"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No destination address configuration error") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The last recorded error was a configuration error detected in the TCDn_DADDR field. TCDn_DADDR is inconsistent with TCDn_ATTR[DSIZE].") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SOE", bit_offset: 6, bit_width: 1, access: Some(ReadOnly), description: Some("Source Offset Error"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No source offset configuration error") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The last recorded error was a configuration error detected in the TCDn_SOFF field. TCDn_SOFF is inconsistent with TCDn_ATTR[SSIZE].") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SAE", bit_offset: 7, bit_width: 1, access: Some(ReadOnly), description: Some("Source Address Error"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No source address configuration error.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The last recorded error was a configuration error detected in the TCDn_SADDR field. TCDn_SADDR is inconsistent with TCDn_ATTR[SSIZE].") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ERRCHN", bit_offset: 8, bit_width: 4, access: Some(ReadOnly), description: Some("Error Channel Number or Canceled Channel Number"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CPE", bit_offset: 14, bit_width: 1, access: Some(ReadOnly), description: Some("Channel Priority Error"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No channel priority error") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The last recorded error was a configuration error in the channel priorities . Channel priorities are not unique.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ECX", bit_offset: 16, bit_width: 1, access: Some(ReadOnly), description: Some("Transfer Canceled"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No canceled transfers") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The last recorded entry was a canceled transfer by the error cancel transfer input") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "VLD", bit_offset: 31, bit_width: 1, access: Some(ReadOnly), description: Some("Logical OR of all ERR status bits"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No ERR bits are set") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("At least one ERR bit is set indicating a valid error exists that has not been cleared") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "ERQ", offset: 12, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Enable Request Register"), fields: [Field { name: "ERQ", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("Enable DMA Request n"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The DMA request signal for the corresponding channel is disabled") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The DMA request signal for the corresponding channel is enabled") }], links: [], dim: Some(16), dim_increment: Some(1), dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "EEI", offset: 20, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Enable Error Interrupt Register"), fields: [Field { name: "EEI", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("Enable Error Interrupt No"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The error signal for corresponding channel does not generate an error interrupt") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The assertion of the error signal for corresponding channel generates an error interrupt request") }], links: [], dim: Some(16), dim_increment: Some(1), dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "CEEI", offset: 24, size: Some(8), access: Some(WriteOnly), reset_value: Some(0), reset_mask: Some(255), description: Some("Clear Enable Error Interrupt Register"), fields: [Field { name: "CEEI", bit_offset: 0, bit_width: 4, access: Some(WriteOnly), description: Some("Clear Enable Error Interrupt"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CAEE", bit_offset: 6, bit_width: 1, access: Some(WriteOnly), description: Some("Clear All Enable Error Interrupts"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Clear only the EEI bit specified in the CEEI field") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Clear all bits in EEI") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "NOP", bit_offset: 7, bit_width: 1, access: Some(WriteOnly), description: Some("No Op enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Normal operation") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("No operation, ignore the other bits in this register") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "SEEI", offset: 25, size: Some(8), access: Some(WriteOnly), reset_value: Some(0), reset_mask: Some(255), description: Some("Set Enable Error Interrupt Register"), fields: [Field { name: "SEEI", bit_offset: 0, bit_width: 4, access: Some(WriteOnly), description: Some("Set Enable Error Interrupt"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SAEE", bit_offset: 6, bit_width: 1, access: Some(WriteOnly), description: Some("Sets All Enable Error Interrupts"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Set only the EEI bit specified in the SEEI field.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Sets all bits in EEI") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "NOP", bit_offset: 7, bit_width: 1, access: Some(WriteOnly), description: Some("No Op enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Normal operation") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("No operation, ignore the other bits in this register") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "CERQ", offset: 26, size: Some(8), access: Some(WriteOnly), reset_value: Some(0), reset_mask: Some(255), description: Some("Clear Enable Request Register"), fields: [Field { name: "CERQ", bit_offset: 0, bit_width: 4, access: Some(WriteOnly), description: Some("Clear Enable Request"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CAER", bit_offset: 6, bit_width: 1, access: Some(WriteOnly), description: Some("Clear All Enable Requests"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Clear only the ERQ bit specified in the CERQ field") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Clear all bits in ERQ") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "NOP", bit_offset: 7, bit_width: 1, access: Some(WriteOnly), description: Some("No Op enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Normal operation") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("No operation, ignore the other bits in this register") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "SERQ", offset: 27, size: Some(8), access: Some(WriteOnly), reset_value: Some(0), reset_mask: Some(255), description: Some("Set Enable Request Register"), fields: [Field { name: "SERQ", bit_offset: 0, bit_width: 4, access: Some(WriteOnly), description: Some("Set enable request"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SAER", bit_offset: 6, bit_width: 1, access: Some(WriteOnly), description: Some("Set All Enable Requests"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Set only the ERQ bit specified in the SERQ field") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Set all bits in ERQ") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "NOP", bit_offset: 7, bit_width: 1, access: Some(WriteOnly), description: Some("No Op enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Normal operation") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("No operation, ignore the other bits in this register") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "CDNE", offset: 28, size: Some(8), access: Some(WriteOnly), reset_value: Some(0), reset_mask: Some(255), description: Some("Clear DONE Status Bit Register"), fields: [Field { name: "CDNE", bit_offset: 0, bit_width: 4, access: Some(WriteOnly), description: Some("Clear DONE Bit"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CADN", bit_offset: 6, bit_width: 1, access: Some(WriteOnly), description: Some("Clears All DONE Bits"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Clears only the TCDn_CSR[DONE] bit specified in the CDNE field") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Clears all bits in TCDn_CSR[DONE]") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "NOP", bit_offset: 7, bit_width: 1, access: Some(WriteOnly), description: Some("No Op enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Normal operation") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("No operation, ignore the other bits in this register") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "SSRT", offset: 29, size: Some(8), access: Some(WriteOnly), reset_value: Some(0), reset_mask: Some(255), description: Some("Set START Bit Register"), fields: [Field { name: "SSRT", bit_offset: 0, bit_width: 4, access: Some(WriteOnly), description: Some("Set START Bit"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SAST", bit_offset: 6, bit_width: 1, access: Some(WriteOnly), description: Some("Set All START Bits (activates all channels)"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Set only the TCDn_CSR[START] bit specified in the SSRT field") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Set all bits in TCDn_CSR[START]") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "NOP", bit_offset: 7, bit_width: 1, access: Some(WriteOnly), description: Some("No Op enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Normal operation") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("No operation, ignore the other bits in this register") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "CERR", offset: 30, size: Some(8), access: Some(WriteOnly), reset_value: Some(0), reset_mask: Some(255), description: Some("Clear Error Register"), fields: [Field { name: "CERR", bit_offset: 0, bit_width: 4, access: Some(WriteOnly), description: Some("Clear Error Indicator"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CAEI", bit_offset: 6, bit_width: 1, access: Some(WriteOnly), description: Some("Clear All Error Indicators"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Clear only the ERR bit specified in the CERR field") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Clear all bits in ERR") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "NOP", bit_offset: 7, bit_width: 1, access: Some(WriteOnly), description: Some("No Op enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Normal operation") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("No operation, ignore the other bits in this register") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "CINT", offset: 31, size: Some(8), access: Some(WriteOnly), reset_value: Some(0), reset_mask: Some(255), description: Some("Clear Interrupt Request Register"), fields: [Field { name: "CINT", bit_offset: 0, bit_width: 4, access: Some(WriteOnly), description: Some("Clear Interrupt Request"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CAIR", bit_offset: 6, bit_width: 1, access: Some(WriteOnly), description: Some("Clear All Interrupt Requests"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Clear only the INT bit specified in the CINT field") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Clear all bits in INT") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "NOP", bit_offset: 7, bit_width: 1, access: Some(WriteOnly), description: Some("No Op enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Normal operation") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("No operation, ignore the other bits in this register") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "INT", offset: 36, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Interrupt Request Register"), fields: [Field { name: "INT", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("Interrupt Request n"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The interrupt request for corresponding channel is cleared") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The interrupt request for corresponding channel is active") }], links: [], dim: Some(16), dim_increment: Some(1), dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "ERR", offset: 44, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Error Register"), fields: [Field { name: "ERR", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("Error In Channel n"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("An error in the corresponding channel has not occurred") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("An error in the corresponding channel has occurred") }], links: [], dim: Some(16), dim_increment: Some(1), dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "HRS", offset: 52, size: Some(32), access: Some(ReadOnly), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Hardware Request Status Register"), fields: [Field { name: "HRS", bit_offset: 0, bit_width: 1, access: Some(ReadOnly), description: Some("Hardware Request Status Channel n"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("A hardware service request for channel n is not present") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("A hardware service request for channel n is present") }], links: [], dim: Some(16), dim_increment: Some(1), dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "DCHPRI", offset: 256, size: Some(8), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(255), description: Some("Channel n Priority Register"), fields: [Field { name: "CHPRI", bit_offset: 0, bit_width: 4, access: Some(ReadWrite), description: Some("Channel n Arbitration Priority"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "DPA", bit_offset: 6, bit_width: 1, access: Some(ReadWrite), description: Some("Disable Preempt Ability"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Channel n can suspend a lower priority channel") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Channel n cannot suspend any channel, regardless of channel priority") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ECP", bit_offset: 7, bit_width: 1, access: Some(ReadWrite), description: Some("Enable Channel Preemption"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Channel n cannot be suspended by a higher priority channel\\\'s service request") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Channel n can be temporarily suspended by the service request of a higher priority channel") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: Some(16), dim_increment: Some(1), dim_index: Some("3,2,1,0,7,6,5,4,11,10,9,8,15,14,13,12") }, Register { name: "TCD_SADDR", offset: 4096, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(0), description: Some("TCD Source Address"), fields: [Field { name: "SADDR", bit_offset: 0, bit_width: 32, access: Some(ReadWrite), description: Some("Source Address"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: Some(16), dim_increment: Some(32), dim_index: Some("0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15") }, Register { name: "TCD_SOFF", offset: 4100, size: Some(16), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(0), description: Some("TCD Signed Source Address Offset"), fields: [Field { name: "SOFF", bit_offset: 0, bit_width: 16, access: Some(ReadWrite), description: Some("Source address signed offset"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: Some(16), dim_increment: Some(32), dim_index: Some("0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15") }, Register { name: "TCD_ATTR", offset: 4102, size: Some(16), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(0), description: Some("TCD Transfer Attributes"), fields: [Field { name: "DSIZE", bit_offset: 0, bit_width: 3, access: Some(ReadWrite), description: Some("Destination Data Transfer Size"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "DMOD", bit_offset: 3, bit_width: 5, access: Some(ReadWrite), description: Some("Destination Address Modulo"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SSIZE", bit_offset: 8, bit_width: 3, access: Some(ReadWrite), description: Some("Source data transfer size"), enumerated_values: [EnumeratedValue { value: "#000", name: Some("000"), description: Some("8-bit") }, EnumeratedValue { value: "#001", name: Some("001"), description: Some("16-bit") }, EnumeratedValue { value: "#010", name: Some("010"), description: Some("32-bit") }, EnumeratedValue { value: "#100", name: Some("100"), description: Some("16-byte") }, EnumeratedValue { value: "#101", name: Some("101"), description: Some("32-byte") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SMOD", bit_offset: 11, bit_width: 5, access: Some(ReadWrite), description: Some("Source Address Modulo."), enumerated_values: [EnumeratedValue { value: "#00000", name: Some("0"), description: Some("Source address modulo feature is disabled") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: Some(16), dim_increment: Some(32), dim_index: Some("0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15") }, Register { name: "TCD_NBYTES_MLNO", offset: 4104, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(0), description: Some("TCD Minor Byte Count (Minor Loop Disabled)"), fields: [Field { name: "NBYTES", bit_offset: 0, bit_width: 32, access: Some(ReadWrite), description: Some("Minor Byte Transfer Count"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: Some(16), dim_increment: Some(32), dim_index: Some("0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15") }, Register { name: "TCD_NBYTES_MLOFFNO", offset: 4104, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(0), description: Some("TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"), fields: [Field { name: "NBYTES", bit_offset: 0, bit_width: 30, access: Some(ReadWrite), description: Some("Minor Byte Transfer Count"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "DMLOE", bit_offset: 30, bit_width: 1, access: Some(ReadWrite), description: Some("Destination Minor Loop Offset enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The minor loop offset is not applied to the DADDR") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The minor loop offset is applied to the DADDR") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SMLOE", bit_offset: 31, bit_width: 1, access: Some(ReadWrite), description: Some("Source Minor Loop Offset Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The minor loop offset is not applied to the SADDR") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The minor loop offset is applied to the SADDR") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: Some(16), dim_increment: Some(32), dim_index: Some("0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15") }, Register { name: "TCD_NBYTES_MLOFFYES", offset: 4104, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(0), description: Some("TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"), fields: [Field { name: "NBYTES", bit_offset: 0, bit_width: 10, access: Some(ReadWrite), description: Some("Minor Byte Transfer Count"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MLOFF", bit_offset: 10, bit_width: 20, access: Some(ReadWrite), description: Some("If SMLOE or DMLOE is set, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "DMLOE", bit_offset: 30, bit_width: 1, access: Some(ReadWrite), description: Some("Destination Minor Loop Offset enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The minor loop offset is not applied to the DADDR") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The minor loop offset is applied to the DADDR") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SMLOE", bit_offset: 31, bit_width: 1, access: Some(ReadWrite), description: Some("Source Minor Loop Offset Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The minor loop offset is not applied to the SADDR") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The minor loop offset is applied to the SADDR") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: Some(16), dim_increment: Some(32), dim_index: Some("0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15") }, Register { name: "TCD_SLAST", offset: 4108, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(0), description: Some("TCD Last Source Address Adjustment"), fields: [Field { name: "SLAST", bit_offset: 0, bit_width: 32, access: Some(ReadWrite), description: Some("Last source Address Adjustment"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: Some(16), dim_increment: Some(32), dim_index: Some("0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15") }, Register { name: "TCD_DADDR", offset: 4112, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(0), description: Some("TCD Destination Address"), fields: [Field { name: "DADDR", bit_offset: 0, bit_width: 32, access: Some(ReadWrite), description: Some("Destination Address"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: Some(16), dim_increment: Some(32), dim_index: Some("0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15") }, Register { name: "TCD_DOFF", offset: 4116, size: Some(16), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(0), description: Some("TCD Signed Destination Address Offset"), fields: [Field { name: "DOFF", bit_offset: 0, bit_width: 16, access: Some(ReadWrite), description: Some("Destination Address Signed offset"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: Some(16), dim_increment: Some(32), dim_index: Some("0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15") }, Register { name: "TCD_CITER_ELINKNO", offset: 4118, size: Some(16), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(0), description: Some("TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"), fields: [Field { name: "CITER", bit_offset: 0, bit_width: 15, access: Some(ReadWrite), description: Some("Current Major Iteration Count"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ELINK", bit_offset: 15, bit_width: 1, access: Some(ReadWrite), description: Some("Enable channel-to-channel linking on minor-loop complete"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The channel-to-channel linking is disabled") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The channel-to-channel linking is enabled") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: Some(16), dim_increment: Some(32), dim_index: Some("0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15") }, Register { name: "TCD_CITER_ELINKYES", offset: 4118, size: Some(16), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(0), description: Some("TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"), fields: [Field { name: "CITER", bit_offset: 0, bit_width: 9, access: Some(ReadWrite), description: Some("Current Major Iteration Count"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "LINKCH", bit_offset: 9, bit_width: 4, access: Some(ReadWrite), description: Some("Link Channel Number"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ELINK", bit_offset: 15, bit_width: 1, access: Some(ReadWrite), description: Some("Enable channel-to-channel linking on minor-loop complete"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The channel-to-channel linking is disabled") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The channel-to-channel linking is enabled") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: Some(16), dim_increment: Some(32), dim_index: Some("0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15") }, Register { name: "TCD_DLASTSGA", offset: 4120, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(0), description: Some("TCD Last Destination Address Adjustment/Scatter Gather Address"), fields: [Field { name: "DLASTSGA", bit_offset: 0, bit_width: 32, access: Some(ReadWrite), description: Some("Destination last address adjustment or the memory address for the next transfer control descriptor to be loaded into this channel (scatter/gather)"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: Some(16), dim_increment: Some(32), dim_index: Some("0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15") }, Register { name: "TCD_CSR", offset: 4124, size: Some(16), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(193), description: Some("TCD Control and Status"), fields: [Field { name: "START", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("Channel Start"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The channel is not explicitly started") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The channel is explicitly started via a software initiated service request") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "INTMAJOR", bit_offset: 1, bit_width: 1, access: Some(ReadWrite), description: Some("Enable an interrupt when major iteration count completes"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The end-of-major loop interrupt is disabled") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The end-of-major loop interrupt is enabled") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "INTHALF", bit_offset: 2, bit_width: 1, access: Some(ReadWrite), description: Some("Enable an interrupt when major counter is half complete."), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The half-point interrupt is disabled") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The half-point interrupt is enabled") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "DREQ", bit_offset: 3, bit_width: 1, access: Some(ReadWrite), description: Some("Disable Request"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The channel\\\'s ERQ bit is not affected") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The channel\\\'s ERQ bit is cleared when the major loop is complete") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ESG", bit_offset: 4, bit_width: 1, access: Some(ReadWrite), description: Some("Enable Scatter/Gather Processing"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The current channel\\\'s TCD is normal format.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The current channel\\\'s TCD specifies a scatter gather format. The DLASTSGA field provides a memory pointer to the next TCD to be loaded into this channel after the major loop completes its execution.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MAJORELINK", bit_offset: 5, bit_width: 1, access: Some(ReadWrite), description: Some("Enable channel-to-channel linking on major loop complete"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The channel-to-channel linking is disabled") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The channel-to-channel linking is enabled") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ACTIVE", bit_offset: 6, bit_width: 1, access: Some(ReadWrite), description: Some("Channel Active"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "DONE", bit_offset: 7, bit_width: 1, access: Some(ReadWrite), description: Some("Channel Done"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MAJORLINKCH", bit_offset: 8, bit_width: 4, access: Some(ReadWrite), description: Some("Link Channel Number"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "BWC", bit_offset: 14, bit_width: 2, access: Some(ReadWrite), description: Some("Bandwidth Control"), enumerated_values: [EnumeratedValue { value: "#00", name: Some("00"), description: Some("No eDMA engine stalls") }, EnumeratedValue { value: "#10", name: Some("10"), description: Some("eDMA engine stalls for 4 cycles after each r/w") }, EnumeratedValue { value: "#11", name: Some("11"), description: Some("eDMA engine stalls for 8 cycles after each r/w") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: Some(16), dim_increment: Some(32), dim_index: Some("0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15") }, Register { name: "TCD_BITER_ELINKNO", offset: 4126, size: Some(16), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(0), description: Some("TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"), fields: [Field { name: "BITER", bit_offset: 0, bit_width: 15, access: Some(ReadWrite), description: Some("Starting Major Iteration Count"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ELINK", bit_offset: 15, bit_width: 1, access: Some(ReadWrite), description: Some("Enables channel-to-channel linking on minor loop complete"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The channel-to-channel linking is disabled") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The channel-to-channel linking is enabled") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: Some(16), dim_increment: Some(32), dim_index: Some("0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15") }, Register { name: "TCD_BITER_ELINKYES", offset: 4126, size: Some(16), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(0), description: Some("TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"), fields: [Field { name: "BITER", bit_offset: 0, bit_width: 9, access: Some(ReadWrite), description: Some("Starting Major Iteration Count"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "LINKCH", bit_offset: 9, bit_width: 4, access: Some(ReadWrite), description: Some("Link Channel Number"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ELINK", bit_offset: 15, bit_width: 1, access: Some(ReadWrite), description: Some("Enables channel-to-channel linking on minor loop complete"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The channel-to-channel linking is disabled") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The channel-to-channel linking is enabled") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: Some(16), dim_increment: Some(32), dim_index: Some("0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15") }], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }), modules: [], has_pins: false, has_channels: true, description: None }

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="EDMA Peripheral"]
pub struct EdmaPeriph(pub usize); 


impl EdmaPeriph {
#[doc="Get the *const pointer for the CR register."]
   #[inline] pub fn cr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x0) as *const u32
   }
#[doc="Get the *mut pointer for the CR register."]
   #[inline] pub fn cr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x0) as *mut u32
   }
#[doc="Read the CR register."]
   #[inline] pub fn cr(&self) -> Cr { 
      unsafe {
         Cr(::core::ptr::read_volatile((self.0 + 0x0) as *const u32))
      }
   }
#[doc="Write the CR register."]
   #[inline] pub fn set_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
      let value = f(Cr(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x0) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CR register."]
   #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
      let tmp = self.cr();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x0) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the ES register."]
   #[inline] pub fn es_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x4) as *const u32
   }
#[doc="Get the *mut pointer for the ES register."]
   #[inline] pub fn es_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x4) as *mut u32
   }
#[doc="Read the ES register."]
   #[inline] pub fn es(&self) -> Es { 
      unsafe {
         Es(::core::ptr::read_volatile((self.0 + 0x4) as *const u32))
      }
   }

#[doc="Get the *const pointer for the ERQ register."]
   #[inline] pub fn erq_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xc) as *const u32
   }
#[doc="Get the *mut pointer for the ERQ register."]
   #[inline] pub fn erq_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xc) as *mut u32
   }
#[doc="Read the ERQ register."]
   #[inline] pub fn erq(&self) -> Erq { 
      unsafe {
         Erq(::core::ptr::read_volatile((self.0 + 0xc) as *const u32))
      }
   }
#[doc="Write the ERQ register."]
   #[inline] pub fn set_erq<F: FnOnce(Erq) -> Erq>(&self, f: F) -> &Self {
      let value = f(Erq(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xc) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the ERQ register."]
   #[inline] pub fn with_erq<F: FnOnce(Erq) -> Erq>(&self, f: F) -> &Self {
      let tmp = self.erq();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xc) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the EEI register."]
   #[inline] pub fn eei_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x14) as *const u32
   }
#[doc="Get the *mut pointer for the EEI register."]
   #[inline] pub fn eei_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x14) as *mut u32
   }
#[doc="Read the EEI register."]
   #[inline] pub fn eei(&self) -> Eei { 
      unsafe {
         Eei(::core::ptr::read_volatile((self.0 + 0x14) as *const u32))
      }
   }
#[doc="Write the EEI register."]
   #[inline] pub fn set_eei<F: FnOnce(Eei) -> Eei>(&self, f: F) -> &Self {
      let value = f(Eei(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x14) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the EEI register."]
   #[inline] pub fn with_eei<F: FnOnce(Eei) -> Eei>(&self, f: F) -> &Self {
      let tmp = self.eei();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x14) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CEEI register."]
   #[inline] pub fn ceei_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x18) as *const u8
   }
#[doc="Get the *mut pointer for the CEEI register."]
   #[inline] pub fn ceei_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x18) as *mut u8
   }
#[doc="Write the CEEI register."]
   #[inline] pub fn set_ceei<F: FnOnce(Ceei) -> Ceei>(&self, f: F) -> &Self {
      let value = f(Ceei(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x18) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SEEI register."]
   #[inline] pub fn seei_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x19) as *const u8
   }
#[doc="Get the *mut pointer for the SEEI register."]
   #[inline] pub fn seei_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x19) as *mut u8
   }
#[doc="Write the SEEI register."]
   #[inline] pub fn set_seei<F: FnOnce(Seei) -> Seei>(&self, f: F) -> &Self {
      let value = f(Seei(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x19) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CERQ register."]
   #[inline] pub fn cerq_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x1a) as *const u8
   }
#[doc="Get the *mut pointer for the CERQ register."]
   #[inline] pub fn cerq_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x1a) as *mut u8
   }
#[doc="Write the CERQ register."]
   #[inline] pub fn set_cerq<F: FnOnce(Cerq) -> Cerq>(&self, f: F) -> &Self {
      let value = f(Cerq(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1a) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SERQ register."]
   #[inline] pub fn serq_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x1b) as *const u8
   }
#[doc="Get the *mut pointer for the SERQ register."]
   #[inline] pub fn serq_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x1b) as *mut u8
   }
#[doc="Write the SERQ register."]
   #[inline] pub fn set_serq<F: FnOnce(Serq) -> Serq>(&self, f: F) -> &Self {
      let value = f(Serq(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1b) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CDNE register."]
   #[inline] pub fn cdne_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x1c) as *const u8
   }
#[doc="Get the *mut pointer for the CDNE register."]
   #[inline] pub fn cdne_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x1c) as *mut u8
   }
#[doc="Write the CDNE register."]
   #[inline] pub fn set_cdne<F: FnOnce(Cdne) -> Cdne>(&self, f: F) -> &Self {
      let value = f(Cdne(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1c) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SSRT register."]
   #[inline] pub fn ssrt_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x1d) as *const u8
   }
#[doc="Get the *mut pointer for the SSRT register."]
   #[inline] pub fn ssrt_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x1d) as *mut u8
   }
#[doc="Write the SSRT register."]
   #[inline] pub fn set_ssrt<F: FnOnce(Ssrt) -> Ssrt>(&self, f: F) -> &Self {
      let value = f(Ssrt(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1d) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CERR register."]
   #[inline] pub fn cerr_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x1e) as *const u8
   }
#[doc="Get the *mut pointer for the CERR register."]
   #[inline] pub fn cerr_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x1e) as *mut u8
   }
#[doc="Write the CERR register."]
   #[inline] pub fn set_cerr<F: FnOnce(Cerr) -> Cerr>(&self, f: F) -> &Self {
      let value = f(Cerr(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1e) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CINT register."]
   #[inline] pub fn cint_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x1f) as *const u8
   }
#[doc="Get the *mut pointer for the CINT register."]
   #[inline] pub fn cint_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x1f) as *mut u8
   }
#[doc="Write the CINT register."]
   #[inline] pub fn set_cint<F: FnOnce(Cint) -> Cint>(&self, f: F) -> &Self {
      let value = f(Cint(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1f) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the INT register."]
   #[inline] pub fn int_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x24) as *const u32
   }
#[doc="Get the *mut pointer for the INT register."]
   #[inline] pub fn int_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x24) as *mut u32
   }
#[doc="Read the INT register."]
   #[inline] pub fn int(&self) -> Int { 
      unsafe {
         Int(::core::ptr::read_volatile((self.0 + 0x24) as *const u32))
      }
   }
#[doc="Write the INT register."]
   #[inline] pub fn set_int<F: FnOnce(Int) -> Int>(&self, f: F) -> &Self {
      let value = f(Int(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x24) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the INT register."]
   #[inline] pub fn with_int<F: FnOnce(Int) -> Int>(&self, f: F) -> &Self {
      let tmp = self.int();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x24) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the ERR register."]
   #[inline] pub fn err_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x2c) as *const u32
   }
#[doc="Get the *mut pointer for the ERR register."]
   #[inline] pub fn err_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x2c) as *mut u32
   }
#[doc="Read the ERR register."]
   #[inline] pub fn err(&self) -> Err { 
      unsafe {
         Err(::core::ptr::read_volatile((self.0 + 0x2c) as *const u32))
      }
   }
#[doc="Write the ERR register."]
   #[inline] pub fn set_err<F: FnOnce(Err) -> Err>(&self, f: F) -> &Self {
      let value = f(Err(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x2c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the ERR register."]
   #[inline] pub fn with_err<F: FnOnce(Err) -> Err>(&self, f: F) -> &Self {
      let tmp = self.err();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x2c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the HRS register."]
   #[inline] pub fn hrs_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x34) as *const u32
   }
#[doc="Get the *mut pointer for the HRS register."]
   #[inline] pub fn hrs_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x34) as *mut u32
   }
#[doc="Read the HRS register."]
   #[inline] pub fn hrs(&self) -> Hrs { 
      unsafe {
         Hrs(::core::ptr::read_volatile((self.0 + 0x34) as *const u32))
      }
   }

#[doc="Get the *const pointer for the DCHPRI register."]
   #[inline] pub fn dchpri_ptr<I: Into<bits::R16>>(&self, index: I) -> *const u8 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x100 + (index)) as *const u8
   }
#[doc="Get the *mut pointer for the DCHPRI register."]
   #[inline] pub fn dchpri_mut<I: Into<bits::R16>>(&self, index: I) -> *mut u8 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x100 + (index)) as *mut u8
   }
#[doc="Read the DCHPRI register."]
   #[inline] pub fn dchpri<I: Into<bits::R16>>(&self, index: I) -> Dchpri { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         Dchpri(::core::ptr::read_volatile((self.0 + 0x100 + (index)) as *const u8))
      }
   }
#[doc="Write the DCHPRI register."]
   #[inline] pub fn set_dchpri<I: Into<bits::R16>, F: FnOnce(Dchpri) -> Dchpri>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let value = f(Dchpri(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x100 + (index)) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the DCHPRI register."]
   #[inline] pub fn with_dchpri<I: Into<bits::R16> + Copy, F: FnOnce(Dchpri) -> Dchpri>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.dchpri(index);
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x100 + (index)) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the TCD_SADDR register."]
   #[inline] pub fn tcd_saddr_ptr<I: Into<bits::R16>>(&self, index: I) -> *const u32 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x1000 + (index * 32)) as *const u32
   }
#[doc="Get the *mut pointer for the TCD_SADDR register."]
   #[inline] pub fn tcd_saddr_mut<I: Into<bits::R16>>(&self, index: I) -> *mut u32 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x1000 + (index * 32)) as *mut u32
   }
#[doc="Read the TCD_SADDR register."]
   #[inline] pub fn tcd_saddr<I: Into<bits::R16>>(&self, index: I) -> TcdSaddr { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         TcdSaddr(::core::ptr::read_volatile((self.0 + 0x1000 + (index * 32)) as *const u32))
      }
   }
#[doc="Write the TCD_SADDR register."]
   #[inline] pub fn set_tcd_saddr<I: Into<bits::R16>, F: FnOnce(TcdSaddr) -> TcdSaddr>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let value = f(TcdSaddr(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1000 + (index * 32)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the TCD_SADDR register."]
   #[inline] pub fn with_tcd_saddr<I: Into<bits::R16> + Copy, F: FnOnce(TcdSaddr) -> TcdSaddr>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.tcd_saddr(index);
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1000 + (index * 32)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the TCD_SOFF register."]
   #[inline] pub fn tcd_soff_ptr<I: Into<bits::R16>>(&self, index: I) -> *const u16 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x1004 + (index * 32)) as *const u16
   }
#[doc="Get the *mut pointer for the TCD_SOFF register."]
   #[inline] pub fn tcd_soff_mut<I: Into<bits::R16>>(&self, index: I) -> *mut u16 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x1004 + (index * 32)) as *mut u16
   }
#[doc="Read the TCD_SOFF register."]
   #[inline] pub fn tcd_soff<I: Into<bits::R16>>(&self, index: I) -> TcdSoff { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         TcdSoff(::core::ptr::read_volatile((self.0 + 0x1004 + (index * 32)) as *const u16))
      }
   }
#[doc="Write the TCD_SOFF register."]
   #[inline] pub fn set_tcd_soff<I: Into<bits::R16>, F: FnOnce(TcdSoff) -> TcdSoff>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let value = f(TcdSoff(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1004 + (index * 32)) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the TCD_SOFF register."]
   #[inline] pub fn with_tcd_soff<I: Into<bits::R16> + Copy, F: FnOnce(TcdSoff) -> TcdSoff>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.tcd_soff(index);
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1004 + (index * 32)) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the TCD_ATTR register."]
   #[inline] pub fn tcd_attr_ptr<I: Into<bits::R16>>(&self, index: I) -> *const u16 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x1006 + (index * 32)) as *const u16
   }
#[doc="Get the *mut pointer for the TCD_ATTR register."]
   #[inline] pub fn tcd_attr_mut<I: Into<bits::R16>>(&self, index: I) -> *mut u16 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x1006 + (index * 32)) as *mut u16
   }
#[doc="Read the TCD_ATTR register."]
   #[inline] pub fn tcd_attr<I: Into<bits::R16>>(&self, index: I) -> TcdAttr { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         TcdAttr(::core::ptr::read_volatile((self.0 + 0x1006 + (index * 32)) as *const u16))
      }
   }
#[doc="Write the TCD_ATTR register."]
   #[inline] pub fn set_tcd_attr<I: Into<bits::R16>, F: FnOnce(TcdAttr) -> TcdAttr>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let value = f(TcdAttr(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1006 + (index * 32)) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the TCD_ATTR register."]
   #[inline] pub fn with_tcd_attr<I: Into<bits::R16> + Copy, F: FnOnce(TcdAttr) -> TcdAttr>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.tcd_attr(index);
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1006 + (index * 32)) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the TCD_NBYTES_MLNO register."]
   #[inline] pub fn tcd_nbytes_mlno_ptr<I: Into<bits::R16>>(&self, index: I) -> *const u32 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x1008 + (index * 32)) as *const u32
   }
#[doc="Get the *mut pointer for the TCD_NBYTES_MLNO register."]
   #[inline] pub fn tcd_nbytes_mlno_mut<I: Into<bits::R16>>(&self, index: I) -> *mut u32 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x1008 + (index * 32)) as *mut u32
   }
#[doc="Read the TCD_NBYTES_MLNO register."]
   #[inline] pub fn tcd_nbytes_mlno<I: Into<bits::R16>>(&self, index: I) -> TcdNbytesMlno { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         TcdNbytesMlno(::core::ptr::read_volatile((self.0 + 0x1008 + (index * 32)) as *const u32))
      }
   }
#[doc="Write the TCD_NBYTES_MLNO register."]
   #[inline] pub fn set_tcd_nbytes_mlno<I: Into<bits::R16>, F: FnOnce(TcdNbytesMlno) -> TcdNbytesMlno>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let value = f(TcdNbytesMlno(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1008 + (index * 32)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the TCD_NBYTES_MLNO register."]
   #[inline] pub fn with_tcd_nbytes_mlno<I: Into<bits::R16> + Copy, F: FnOnce(TcdNbytesMlno) -> TcdNbytesMlno>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.tcd_nbytes_mlno(index);
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1008 + (index * 32)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the TCD_NBYTES_MLOFFNO register."]
   #[inline] pub fn tcd_nbytes_mloffno_ptr<I: Into<bits::R16>>(&self, index: I) -> *const u32 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x1008 + (index * 32)) as *const u32
   }
#[doc="Get the *mut pointer for the TCD_NBYTES_MLOFFNO register."]
   #[inline] pub fn tcd_nbytes_mloffno_mut<I: Into<bits::R16>>(&self, index: I) -> *mut u32 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x1008 + (index * 32)) as *mut u32
   }
#[doc="Read the TCD_NBYTES_MLOFFNO register."]
   #[inline] pub fn tcd_nbytes_mloffno<I: Into<bits::R16>>(&self, index: I) -> TcdNbytesMloffno { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         TcdNbytesMloffno(::core::ptr::read_volatile((self.0 + 0x1008 + (index * 32)) as *const u32))
      }
   }
#[doc="Write the TCD_NBYTES_MLOFFNO register."]
   #[inline] pub fn set_tcd_nbytes_mloffno<I: Into<bits::R16>, F: FnOnce(TcdNbytesMloffno) -> TcdNbytesMloffno>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let value = f(TcdNbytesMloffno(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1008 + (index * 32)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the TCD_NBYTES_MLOFFNO register."]
   #[inline] pub fn with_tcd_nbytes_mloffno<I: Into<bits::R16> + Copy, F: FnOnce(TcdNbytesMloffno) -> TcdNbytesMloffno>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.tcd_nbytes_mloffno(index);
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1008 + (index * 32)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the TCD_NBYTES_MLOFFYES register."]
   #[inline] pub fn tcd_nbytes_mloffyes_ptr<I: Into<bits::R16>>(&self, index: I) -> *const u32 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x1008 + (index * 32)) as *const u32
   }
#[doc="Get the *mut pointer for the TCD_NBYTES_MLOFFYES register."]
   #[inline] pub fn tcd_nbytes_mloffyes_mut<I: Into<bits::R16>>(&self, index: I) -> *mut u32 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x1008 + (index * 32)) as *mut u32
   }
#[doc="Read the TCD_NBYTES_MLOFFYES register."]
   #[inline] pub fn tcd_nbytes_mloffyes<I: Into<bits::R16>>(&self, index: I) -> TcdNbytesMloffyes { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         TcdNbytesMloffyes(::core::ptr::read_volatile((self.0 + 0x1008 + (index * 32)) as *const u32))
      }
   }
#[doc="Write the TCD_NBYTES_MLOFFYES register."]
   #[inline] pub fn set_tcd_nbytes_mloffyes<I: Into<bits::R16>, F: FnOnce(TcdNbytesMloffyes) -> TcdNbytesMloffyes>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let value = f(TcdNbytesMloffyes(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1008 + (index * 32)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the TCD_NBYTES_MLOFFYES register."]
   #[inline] pub fn with_tcd_nbytes_mloffyes<I: Into<bits::R16> + Copy, F: FnOnce(TcdNbytesMloffyes) -> TcdNbytesMloffyes>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.tcd_nbytes_mloffyes(index);
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1008 + (index * 32)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the TCD_SLAST register."]
   #[inline] pub fn tcd_slast_ptr<I: Into<bits::R16>>(&self, index: I) -> *const u32 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x100c + (index * 32)) as *const u32
   }
#[doc="Get the *mut pointer for the TCD_SLAST register."]
   #[inline] pub fn tcd_slast_mut<I: Into<bits::R16>>(&self, index: I) -> *mut u32 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x100c + (index * 32)) as *mut u32
   }
#[doc="Read the TCD_SLAST register."]
   #[inline] pub fn tcd_slast<I: Into<bits::R16>>(&self, index: I) -> TcdSlast { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         TcdSlast(::core::ptr::read_volatile((self.0 + 0x100c + (index * 32)) as *const u32))
      }
   }
#[doc="Write the TCD_SLAST register."]
   #[inline] pub fn set_tcd_slast<I: Into<bits::R16>, F: FnOnce(TcdSlast) -> TcdSlast>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let value = f(TcdSlast(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x100c + (index * 32)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the TCD_SLAST register."]
   #[inline] pub fn with_tcd_slast<I: Into<bits::R16> + Copy, F: FnOnce(TcdSlast) -> TcdSlast>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.tcd_slast(index);
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x100c + (index * 32)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the TCD_DADDR register."]
   #[inline] pub fn tcd_daddr_ptr<I: Into<bits::R16>>(&self, index: I) -> *const u32 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x1010 + (index * 32)) as *const u32
   }
#[doc="Get the *mut pointer for the TCD_DADDR register."]
   #[inline] pub fn tcd_daddr_mut<I: Into<bits::R16>>(&self, index: I) -> *mut u32 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x1010 + (index * 32)) as *mut u32
   }
#[doc="Read the TCD_DADDR register."]
   #[inline] pub fn tcd_daddr<I: Into<bits::R16>>(&self, index: I) -> TcdDaddr { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         TcdDaddr(::core::ptr::read_volatile((self.0 + 0x1010 + (index * 32)) as *const u32))
      }
   }
#[doc="Write the TCD_DADDR register."]
   #[inline] pub fn set_tcd_daddr<I: Into<bits::R16>, F: FnOnce(TcdDaddr) -> TcdDaddr>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let value = f(TcdDaddr(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1010 + (index * 32)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the TCD_DADDR register."]
   #[inline] pub fn with_tcd_daddr<I: Into<bits::R16> + Copy, F: FnOnce(TcdDaddr) -> TcdDaddr>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.tcd_daddr(index);
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1010 + (index * 32)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the TCD_DOFF register."]
   #[inline] pub fn tcd_doff_ptr<I: Into<bits::R16>>(&self, index: I) -> *const u16 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x1014 + (index * 32)) as *const u16
   }
#[doc="Get the *mut pointer for the TCD_DOFF register."]
   #[inline] pub fn tcd_doff_mut<I: Into<bits::R16>>(&self, index: I) -> *mut u16 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x1014 + (index * 32)) as *mut u16
   }
#[doc="Read the TCD_DOFF register."]
   #[inline] pub fn tcd_doff<I: Into<bits::R16>>(&self, index: I) -> TcdDoff { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         TcdDoff(::core::ptr::read_volatile((self.0 + 0x1014 + (index * 32)) as *const u16))
      }
   }
#[doc="Write the TCD_DOFF register."]
   #[inline] pub fn set_tcd_doff<I: Into<bits::R16>, F: FnOnce(TcdDoff) -> TcdDoff>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let value = f(TcdDoff(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1014 + (index * 32)) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the TCD_DOFF register."]
   #[inline] pub fn with_tcd_doff<I: Into<bits::R16> + Copy, F: FnOnce(TcdDoff) -> TcdDoff>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.tcd_doff(index);
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1014 + (index * 32)) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the TCD_CITER_ELINKNO register."]
   #[inline] pub fn tcd_citer_elinkno_ptr<I: Into<bits::R16>>(&self, index: I) -> *const u16 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x1016 + (index * 32)) as *const u16
   }
#[doc="Get the *mut pointer for the TCD_CITER_ELINKNO register."]
   #[inline] pub fn tcd_citer_elinkno_mut<I: Into<bits::R16>>(&self, index: I) -> *mut u16 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x1016 + (index * 32)) as *mut u16
   }
#[doc="Read the TCD_CITER_ELINKNO register."]
   #[inline] pub fn tcd_citer_elinkno<I: Into<bits::R16>>(&self, index: I) -> TcdCiterElinkno { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         TcdCiterElinkno(::core::ptr::read_volatile((self.0 + 0x1016 + (index * 32)) as *const u16))
      }
   }
#[doc="Write the TCD_CITER_ELINKNO register."]
   #[inline] pub fn set_tcd_citer_elinkno<I: Into<bits::R16>, F: FnOnce(TcdCiterElinkno) -> TcdCiterElinkno>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let value = f(TcdCiterElinkno(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1016 + (index * 32)) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the TCD_CITER_ELINKNO register."]
   #[inline] pub fn with_tcd_citer_elinkno<I: Into<bits::R16> + Copy, F: FnOnce(TcdCiterElinkno) -> TcdCiterElinkno>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.tcd_citer_elinkno(index);
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1016 + (index * 32)) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the TCD_CITER_ELINKYES register."]
   #[inline] pub fn tcd_citer_elinkyes_ptr<I: Into<bits::R16>>(&self, index: I) -> *const u16 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x1016 + (index * 32)) as *const u16
   }
#[doc="Get the *mut pointer for the TCD_CITER_ELINKYES register."]
   #[inline] pub fn tcd_citer_elinkyes_mut<I: Into<bits::R16>>(&self, index: I) -> *mut u16 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x1016 + (index * 32)) as *mut u16
   }
#[doc="Read the TCD_CITER_ELINKYES register."]
   #[inline] pub fn tcd_citer_elinkyes<I: Into<bits::R16>>(&self, index: I) -> TcdCiterElinkyes { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         TcdCiterElinkyes(::core::ptr::read_volatile((self.0 + 0x1016 + (index * 32)) as *const u16))
      }
   }
#[doc="Write the TCD_CITER_ELINKYES register."]
   #[inline] pub fn set_tcd_citer_elinkyes<I: Into<bits::R16>, F: FnOnce(TcdCiterElinkyes) -> TcdCiterElinkyes>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let value = f(TcdCiterElinkyes(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1016 + (index * 32)) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the TCD_CITER_ELINKYES register."]
   #[inline] pub fn with_tcd_citer_elinkyes<I: Into<bits::R16> + Copy, F: FnOnce(TcdCiterElinkyes) -> TcdCiterElinkyes>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.tcd_citer_elinkyes(index);
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1016 + (index * 32)) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the TCD_DLASTSGA register."]
   #[inline] pub fn tcd_dlastsga_ptr<I: Into<bits::R16>>(&self, index: I) -> *const u32 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x1018 + (index * 32)) as *const u32
   }
#[doc="Get the *mut pointer for the TCD_DLASTSGA register."]
   #[inline] pub fn tcd_dlastsga_mut<I: Into<bits::R16>>(&self, index: I) -> *mut u32 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x1018 + (index * 32)) as *mut u32
   }
#[doc="Read the TCD_DLASTSGA register."]
   #[inline] pub fn tcd_dlastsga<I: Into<bits::R16>>(&self, index: I) -> TcdDlastsga { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         TcdDlastsga(::core::ptr::read_volatile((self.0 + 0x1018 + (index * 32)) as *const u32))
      }
   }
#[doc="Write the TCD_DLASTSGA register."]
   #[inline] pub fn set_tcd_dlastsga<I: Into<bits::R16>, F: FnOnce(TcdDlastsga) -> TcdDlastsga>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let value = f(TcdDlastsga(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1018 + (index * 32)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the TCD_DLASTSGA register."]
   #[inline] pub fn with_tcd_dlastsga<I: Into<bits::R16> + Copy, F: FnOnce(TcdDlastsga) -> TcdDlastsga>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.tcd_dlastsga(index);
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1018 + (index * 32)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the TCD_CSR register."]
   #[inline] pub fn tcd_csr_ptr<I: Into<bits::R16>>(&self, index: I) -> *const u16 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x101c + (index * 32)) as *const u16
   }
#[doc="Get the *mut pointer for the TCD_CSR register."]
   #[inline] pub fn tcd_csr_mut<I: Into<bits::R16>>(&self, index: I) -> *mut u16 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x101c + (index * 32)) as *mut u16
   }
#[doc="Read the TCD_CSR register."]
   #[inline] pub fn tcd_csr<I: Into<bits::R16>>(&self, index: I) -> TcdCsr { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         TcdCsr(::core::ptr::read_volatile((self.0 + 0x101c + (index * 32)) as *const u16))
      }
   }
#[doc="Write the TCD_CSR register."]
   #[inline] pub fn set_tcd_csr<I: Into<bits::R16>, F: FnOnce(TcdCsr) -> TcdCsr>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let value = f(TcdCsr(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x101c + (index * 32)) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the TCD_CSR register."]
   #[inline] pub fn with_tcd_csr<I: Into<bits::R16> + Copy, F: FnOnce(TcdCsr) -> TcdCsr>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.tcd_csr(index);
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x101c + (index * 32)) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the TCD_BITER_ELINKNO register."]
   #[inline] pub fn tcd_biter_elinkno_ptr<I: Into<bits::R16>>(&self, index: I) -> *const u16 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x101e + (index * 32)) as *const u16
   }
#[doc="Get the *mut pointer for the TCD_BITER_ELINKNO register."]
   #[inline] pub fn tcd_biter_elinkno_mut<I: Into<bits::R16>>(&self, index: I) -> *mut u16 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x101e + (index * 32)) as *mut u16
   }
#[doc="Read the TCD_BITER_ELINKNO register."]
   #[inline] pub fn tcd_biter_elinkno<I: Into<bits::R16>>(&self, index: I) -> TcdBiterElinkno { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         TcdBiterElinkno(::core::ptr::read_volatile((self.0 + 0x101e + (index * 32)) as *const u16))
      }
   }
#[doc="Write the TCD_BITER_ELINKNO register."]
   #[inline] pub fn set_tcd_biter_elinkno<I: Into<bits::R16>, F: FnOnce(TcdBiterElinkno) -> TcdBiterElinkno>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let value = f(TcdBiterElinkno(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x101e + (index * 32)) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the TCD_BITER_ELINKNO register."]
   #[inline] pub fn with_tcd_biter_elinkno<I: Into<bits::R16> + Copy, F: FnOnce(TcdBiterElinkno) -> TcdBiterElinkno>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.tcd_biter_elinkno(index);
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x101e + (index * 32)) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the TCD_BITER_ELINKYES register."]
   #[inline] pub fn tcd_biter_elinkyes_ptr<I: Into<bits::R16>>(&self, index: I) -> *const u16 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x101e + (index * 32)) as *const u16
   }
#[doc="Get the *mut pointer for the TCD_BITER_ELINKYES register."]
   #[inline] pub fn tcd_biter_elinkyes_mut<I: Into<bits::R16>>(&self, index: I) -> *mut u16 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x101e + (index * 32)) as *mut u16
   }
#[doc="Read the TCD_BITER_ELINKYES register."]
   #[inline] pub fn tcd_biter_elinkyes<I: Into<bits::R16>>(&self, index: I) -> TcdBiterElinkyes { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         TcdBiterElinkyes(::core::ptr::read_volatile((self.0 + 0x101e + (index * 32)) as *const u16))
      }
   }
#[doc="Write the TCD_BITER_ELINKYES register."]
   #[inline] pub fn set_tcd_biter_elinkyes<I: Into<bits::R16>, F: FnOnce(TcdBiterElinkyes) -> TcdBiterElinkyes>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let value = f(TcdBiterElinkyes(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x101e + (index * 32)) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the TCD_BITER_ELINKYES register."]
   #[inline] pub fn with_tcd_biter_elinkyes<I: Into<bits::R16> + Copy, F: FnOnce(TcdBiterElinkyes) -> TcdBiterElinkyes>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.tcd_biter_elinkyes(index);
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x101e + (index * 32)) as *mut u16, value.0);
      }
      self
   }

}

#[doc="Control Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
#[doc="Enable Debug"]
   #[inline] pub fn edbg(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Enable Debug"]
   #[inline] pub fn set_edbg<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Enable Round Robin Channel Arbitration"]
   #[inline] pub fn erca(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Enable Round Robin Channel Arbitration"]
   #[inline] pub fn set_erca<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Halt On Error"]
   #[inline] pub fn hoe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Halt On Error"]
   #[inline] pub fn set_hoe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Halt DMA Operations"]
   #[inline] pub fn halt(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Halt DMA Operations"]
   #[inline] pub fn set_halt<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Continuous Link Mode"]
   #[inline] pub fn clm(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Continuous Link Mode"]
   #[inline] pub fn set_clm<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Enable Minor Loop Mapping"]
   #[inline] pub fn emlm(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Enable Minor Loop Mapping"]
   #[inline] pub fn set_emlm<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="Error Cancel Transfer"]
   #[inline] pub fn ecx(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
   }
#[doc="Error Cancel Transfer"]
   #[inline] pub fn set_ecx<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Cancel Transfer"]
   #[inline] pub fn cx(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
   }
#[doc="Cancel Transfer"]
   #[inline] pub fn set_cx<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 17);
      self.0 |= value << 17;
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
      if self.edbg() != 0 { try!(write!(f, " edbg"))}
      if self.erca() != 0 { try!(write!(f, " erca"))}
      if self.hoe() != 0 { try!(write!(f, " hoe"))}
      if self.halt() != 0 { try!(write!(f, " halt"))}
      if self.clm() != 0 { try!(write!(f, " clm"))}
      if self.emlm() != 0 { try!(write!(f, " emlm"))}
      if self.ecx() != 0 { try!(write!(f, " ecx"))}
      if self.cx() != 0 { try!(write!(f, " cx"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Error Status Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Es(pub u32);
impl Es {
#[doc="Destination Bus Error"]
   #[inline] pub fn dbe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Destination Bus Error"]
   #[inline] pub fn set_dbe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Source Bus Error"]
   #[inline] pub fn sbe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Source Bus Error"]
   #[inline] pub fn set_sbe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Scatter/Gather Configuration Error"]
   #[inline] pub fn sge(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Scatter/Gather Configuration Error"]
   #[inline] pub fn set_sge<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="NBYTES/CITER Configuration Error"]
   #[inline] pub fn nce(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="NBYTES/CITER Configuration Error"]
   #[inline] pub fn set_nce<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Destination Offset Error"]
   #[inline] pub fn doe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Destination Offset Error"]
   #[inline] pub fn set_doe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Destination Address Error"]
   #[inline] pub fn dae(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Destination Address Error"]
   #[inline] pub fn set_dae<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Source Offset Error"]
   #[inline] pub fn soe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Source Offset Error"]
   #[inline] pub fn set_soe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Source Address Error"]
   #[inline] pub fn sae(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Source Address Error"]
   #[inline] pub fn set_sae<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="Error Channel Number or Canceled Channel Number"]
   #[inline] pub fn errchn(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
   }
#[doc="Error Channel Number or Canceled Channel Number"]
   #[inline] pub fn set_errchn<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Channel Priority Error"]
   #[inline] pub fn cpe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
   }
#[doc="Channel Priority Error"]
   #[inline] pub fn set_cpe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 14);
      self.0 |= value << 14;
      self
   }

#[doc="Transfer Canceled"]
   #[inline] pub fn ecx(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
   }
#[doc="Transfer Canceled"]
   #[inline] pub fn set_ecx<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Logical OR of all ERR status bits"]
   #[inline] pub fn vld(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Logical OR of all ERR status bits"]
   #[inline] pub fn set_vld<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 31);
      self.0 |= value << 31;
      self
   }

}
impl ::core::fmt::Display for Es {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Es {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dbe() != 0 { try!(write!(f, " dbe"))}
      if self.sbe() != 0 { try!(write!(f, " sbe"))}
      if self.sge() != 0 { try!(write!(f, " sge"))}
      if self.nce() != 0 { try!(write!(f, " nce"))}
      if self.doe() != 0 { try!(write!(f, " doe"))}
      if self.dae() != 0 { try!(write!(f, " dae"))}
      if self.soe() != 0 { try!(write!(f, " soe"))}
      if self.sae() != 0 { try!(write!(f, " sae"))}
      if self.errchn() != 0 { try!(write!(f, " errchn=0x{:x}", self.errchn()))}
      if self.cpe() != 0 { try!(write!(f, " cpe"))}
      if self.ecx() != 0 { try!(write!(f, " ecx"))}
      if self.vld() != 0 { try!(write!(f, " vld"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Enable Request Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Erq(pub u32);
impl Erq {
#[doc="Enable DMA Request n"]
   #[inline] pub fn erq<I: Into<bits::R16>>(&self, index: I) -> bits::U1 {
      let index: bits::R16 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="Enable DMA Request n"]
   #[inline] pub fn set_erq<I: Into<bits::R16>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Erq {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Erq {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.erq(0) != 0 { try!(write!(f, " erq[0]"))}
      if self.erq(1) != 0 { try!(write!(f, " erq[1]"))}
      if self.erq(2) != 0 { try!(write!(f, " erq[2]"))}
      if self.erq(3) != 0 { try!(write!(f, " erq[3]"))}
      if self.erq(4) != 0 { try!(write!(f, " erq[4]"))}
      if self.erq(5) != 0 { try!(write!(f, " erq[5]"))}
      if self.erq(6) != 0 { try!(write!(f, " erq[6]"))}
      if self.erq(7) != 0 { try!(write!(f, " erq[7]"))}
      if self.erq(8) != 0 { try!(write!(f, " erq[8]"))}
      if self.erq(9) != 0 { try!(write!(f, " erq[9]"))}
      if self.erq(10) != 0 { try!(write!(f, " erq[10]"))}
      if self.erq(11) != 0 { try!(write!(f, " erq[11]"))}
      if self.erq(12) != 0 { try!(write!(f, " erq[12]"))}
      if self.erq(13) != 0 { try!(write!(f, " erq[13]"))}
      if self.erq(14) != 0 { try!(write!(f, " erq[14]"))}
      if self.erq(15) != 0 { try!(write!(f, " erq[15]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Enable Error Interrupt Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Eei(pub u32);
impl Eei {
#[doc="Enable Error Interrupt No"]
   #[inline] pub fn eei<I: Into<bits::R16>>(&self, index: I) -> bits::U1 {
      let index: bits::R16 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="Enable Error Interrupt No"]
   #[inline] pub fn set_eei<I: Into<bits::R16>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Eei {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Eei {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.eei(0) != 0 { try!(write!(f, " eei[0]"))}
      if self.eei(1) != 0 { try!(write!(f, " eei[1]"))}
      if self.eei(2) != 0 { try!(write!(f, " eei[2]"))}
      if self.eei(3) != 0 { try!(write!(f, " eei[3]"))}
      if self.eei(4) != 0 { try!(write!(f, " eei[4]"))}
      if self.eei(5) != 0 { try!(write!(f, " eei[5]"))}
      if self.eei(6) != 0 { try!(write!(f, " eei[6]"))}
      if self.eei(7) != 0 { try!(write!(f, " eei[7]"))}
      if self.eei(8) != 0 { try!(write!(f, " eei[8]"))}
      if self.eei(9) != 0 { try!(write!(f, " eei[9]"))}
      if self.eei(10) != 0 { try!(write!(f, " eei[10]"))}
      if self.eei(11) != 0 { try!(write!(f, " eei[11]"))}
      if self.eei(12) != 0 { try!(write!(f, " eei[12]"))}
      if self.eei(13) != 0 { try!(write!(f, " eei[13]"))}
      if self.eei(14) != 0 { try!(write!(f, " eei[14]"))}
      if self.eei(15) != 0 { try!(write!(f, " eei[15]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Clear Enable Error Interrupt Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ceei(pub u8);
impl Ceei {
#[doc="Clear Enable Error Interrupt"]
   #[inline] pub fn ceei(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
   }
#[doc="Clear Enable Error Interrupt"]
   #[inline] pub fn set_ceei<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xf << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Clear All Enable Error Interrupts"]
   #[inline] pub fn caee(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Clear All Enable Error Interrupts"]
   #[inline] pub fn set_caee<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="No Op enable"]
   #[inline] pub fn nop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="No Op enable"]
   #[inline] pub fn set_nop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Ceei {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ceei {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ceei() != 0 { try!(write!(f, " ceei=0x{:x}", self.ceei()))}
      if self.caee() != 0 { try!(write!(f, " caee"))}
      if self.nop() != 0 { try!(write!(f, " nop"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Set Enable Error Interrupt Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Seei(pub u8);
impl Seei {
#[doc="Set Enable Error Interrupt"]
   #[inline] pub fn seei(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
   }
#[doc="Set Enable Error Interrupt"]
   #[inline] pub fn set_seei<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xf << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Sets All Enable Error Interrupts"]
   #[inline] pub fn saee(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Sets All Enable Error Interrupts"]
   #[inline] pub fn set_saee<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="No Op enable"]
   #[inline] pub fn nop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="No Op enable"]
   #[inline] pub fn set_nop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Seei {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Seei {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.seei() != 0 { try!(write!(f, " seei=0x{:x}", self.seei()))}
      if self.saee() != 0 { try!(write!(f, " saee"))}
      if self.nop() != 0 { try!(write!(f, " nop"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Clear Enable Request Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cerq(pub u8);
impl Cerq {
#[doc="Clear Enable Request"]
   #[inline] pub fn cerq(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
   }
#[doc="Clear Enable Request"]
   #[inline] pub fn set_cerq<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xf << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Clear All Enable Requests"]
   #[inline] pub fn caer(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Clear All Enable Requests"]
   #[inline] pub fn set_caer<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="No Op enable"]
   #[inline] pub fn nop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="No Op enable"]
   #[inline] pub fn set_nop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Cerq {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cerq {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cerq() != 0 { try!(write!(f, " cerq=0x{:x}", self.cerq()))}
      if self.caer() != 0 { try!(write!(f, " caer"))}
      if self.nop() != 0 { try!(write!(f, " nop"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Set Enable Request Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Serq(pub u8);
impl Serq {
#[doc="Set enable request"]
   #[inline] pub fn serq(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
   }
#[doc="Set enable request"]
   #[inline] pub fn set_serq<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xf << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Set All Enable Requests"]
   #[inline] pub fn saer(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Set All Enable Requests"]
   #[inline] pub fn set_saer<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="No Op enable"]
   #[inline] pub fn nop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="No Op enable"]
   #[inline] pub fn set_nop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Serq {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Serq {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.serq() != 0 { try!(write!(f, " serq=0x{:x}", self.serq()))}
      if self.saer() != 0 { try!(write!(f, " saer"))}
      if self.nop() != 0 { try!(write!(f, " nop"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Clear DONE Status Bit Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cdne(pub u8);
impl Cdne {
#[doc="Clear DONE Bit"]
   #[inline] pub fn cdne(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
   }
#[doc="Clear DONE Bit"]
   #[inline] pub fn set_cdne<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xf << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Clears All DONE Bits"]
   #[inline] pub fn cadn(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Clears All DONE Bits"]
   #[inline] pub fn set_cadn<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="No Op enable"]
   #[inline] pub fn nop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="No Op enable"]
   #[inline] pub fn set_nop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Cdne {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cdne {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cdne() != 0 { try!(write!(f, " cdne=0x{:x}", self.cdne()))}
      if self.cadn() != 0 { try!(write!(f, " cadn"))}
      if self.nop() != 0 { try!(write!(f, " nop"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Set START Bit Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ssrt(pub u8);
impl Ssrt {
#[doc="Set START Bit"]
   #[inline] pub fn ssrt(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
   }
#[doc="Set START Bit"]
   #[inline] pub fn set_ssrt<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xf << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Set All START Bits (activates all channels)"]
   #[inline] pub fn sast(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Set All START Bits (activates all channels)"]
   #[inline] pub fn set_sast<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="No Op enable"]
   #[inline] pub fn nop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="No Op enable"]
   #[inline] pub fn set_nop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Ssrt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ssrt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ssrt() != 0 { try!(write!(f, " ssrt=0x{:x}", self.ssrt()))}
      if self.sast() != 0 { try!(write!(f, " sast"))}
      if self.nop() != 0 { try!(write!(f, " nop"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Clear Error Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cerr(pub u8);
impl Cerr {
#[doc="Clear Error Indicator"]
   #[inline] pub fn cerr(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
   }
#[doc="Clear Error Indicator"]
   #[inline] pub fn set_cerr<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xf << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Clear All Error Indicators"]
   #[inline] pub fn caei(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Clear All Error Indicators"]
   #[inline] pub fn set_caei<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="No Op enable"]
   #[inline] pub fn nop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="No Op enable"]
   #[inline] pub fn set_nop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Cerr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cerr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cerr() != 0 { try!(write!(f, " cerr=0x{:x}", self.cerr()))}
      if self.caei() != 0 { try!(write!(f, " caei"))}
      if self.nop() != 0 { try!(write!(f, " nop"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Clear Interrupt Request Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cint(pub u8);
impl Cint {
#[doc="Clear Interrupt Request"]
   #[inline] pub fn cint(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
   }
#[doc="Clear Interrupt Request"]
   #[inline] pub fn set_cint<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xf << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Clear All Interrupt Requests"]
   #[inline] pub fn cair(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Clear All Interrupt Requests"]
   #[inline] pub fn set_cair<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="No Op enable"]
   #[inline] pub fn nop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="No Op enable"]
   #[inline] pub fn set_nop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Cint {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cint {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cint() != 0 { try!(write!(f, " cint=0x{:x}", self.cint()))}
      if self.cair() != 0 { try!(write!(f, " cair"))}
      if self.nop() != 0 { try!(write!(f, " nop"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Interrupt Request Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Int(pub u32);
impl Int {
#[doc="Interrupt Request n"]
   #[inline] pub fn int<I: Into<bits::R16>>(&self, index: I) -> bits::U1 {
      let index: bits::R16 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="Interrupt Request n"]
   #[inline] pub fn set_int<I: Into<bits::R16>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Int {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Int {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.int(0) != 0 { try!(write!(f, " int[0]"))}
      if self.int(1) != 0 { try!(write!(f, " int[1]"))}
      if self.int(2) != 0 { try!(write!(f, " int[2]"))}
      if self.int(3) != 0 { try!(write!(f, " int[3]"))}
      if self.int(4) != 0 { try!(write!(f, " int[4]"))}
      if self.int(5) != 0 { try!(write!(f, " int[5]"))}
      if self.int(6) != 0 { try!(write!(f, " int[6]"))}
      if self.int(7) != 0 { try!(write!(f, " int[7]"))}
      if self.int(8) != 0 { try!(write!(f, " int[8]"))}
      if self.int(9) != 0 { try!(write!(f, " int[9]"))}
      if self.int(10) != 0 { try!(write!(f, " int[10]"))}
      if self.int(11) != 0 { try!(write!(f, " int[11]"))}
      if self.int(12) != 0 { try!(write!(f, " int[12]"))}
      if self.int(13) != 0 { try!(write!(f, " int[13]"))}
      if self.int(14) != 0 { try!(write!(f, " int[14]"))}
      if self.int(15) != 0 { try!(write!(f, " int[15]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Error Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Err(pub u32);
impl Err {
#[doc="Error In Channel n"]
   #[inline] pub fn err<I: Into<bits::R16>>(&self, index: I) -> bits::U1 {
      let index: bits::R16 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="Error In Channel n"]
   #[inline] pub fn set_err<I: Into<bits::R16>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Err {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Err {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.err(0) != 0 { try!(write!(f, " err[0]"))}
      if self.err(1) != 0 { try!(write!(f, " err[1]"))}
      if self.err(2) != 0 { try!(write!(f, " err[2]"))}
      if self.err(3) != 0 { try!(write!(f, " err[3]"))}
      if self.err(4) != 0 { try!(write!(f, " err[4]"))}
      if self.err(5) != 0 { try!(write!(f, " err[5]"))}
      if self.err(6) != 0 { try!(write!(f, " err[6]"))}
      if self.err(7) != 0 { try!(write!(f, " err[7]"))}
      if self.err(8) != 0 { try!(write!(f, " err[8]"))}
      if self.err(9) != 0 { try!(write!(f, " err[9]"))}
      if self.err(10) != 0 { try!(write!(f, " err[10]"))}
      if self.err(11) != 0 { try!(write!(f, " err[11]"))}
      if self.err(12) != 0 { try!(write!(f, " err[12]"))}
      if self.err(13) != 0 { try!(write!(f, " err[13]"))}
      if self.err(14) != 0 { try!(write!(f, " err[14]"))}
      if self.err(15) != 0 { try!(write!(f, " err[15]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Hardware Request Status Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Hrs(pub u32);
impl Hrs {
#[doc="Hardware Request Status Channel n"]
   #[inline] pub fn hrs<I: Into<bits::R16>>(&self, index: I) -> bits::U1 {
      let index: bits::R16 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="Hardware Request Status Channel n"]
   #[inline] pub fn set_hrs<I: Into<bits::R16>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Hrs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Hrs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.hrs(0) != 0 { try!(write!(f, " hrs[0]"))}
      if self.hrs(1) != 0 { try!(write!(f, " hrs[1]"))}
      if self.hrs(2) != 0 { try!(write!(f, " hrs[2]"))}
      if self.hrs(3) != 0 { try!(write!(f, " hrs[3]"))}
      if self.hrs(4) != 0 { try!(write!(f, " hrs[4]"))}
      if self.hrs(5) != 0 { try!(write!(f, " hrs[5]"))}
      if self.hrs(6) != 0 { try!(write!(f, " hrs[6]"))}
      if self.hrs(7) != 0 { try!(write!(f, " hrs[7]"))}
      if self.hrs(8) != 0 { try!(write!(f, " hrs[8]"))}
      if self.hrs(9) != 0 { try!(write!(f, " hrs[9]"))}
      if self.hrs(10) != 0 { try!(write!(f, " hrs[10]"))}
      if self.hrs(11) != 0 { try!(write!(f, " hrs[11]"))}
      if self.hrs(12) != 0 { try!(write!(f, " hrs[12]"))}
      if self.hrs(13) != 0 { try!(write!(f, " hrs[13]"))}
      if self.hrs(14) != 0 { try!(write!(f, " hrs[14]"))}
      if self.hrs(15) != 0 { try!(write!(f, " hrs[15]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Channel n Priority Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dchpri(pub u8);
impl Dchpri {
#[doc="Channel n Arbitration Priority"]
   #[inline] pub fn chpri(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
   }
#[doc="Channel n Arbitration Priority"]
   #[inline] pub fn set_chpri<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0xf << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Disable Preempt Ability"]
   #[inline] pub fn dpa(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Disable Preempt Ability"]
   #[inline] pub fn set_dpa<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Enable Channel Preemption"]
   #[inline] pub fn ecp(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Enable Channel Preemption"]
   #[inline] pub fn set_ecp<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Dchpri {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dchpri {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.chpri() != 0 { try!(write!(f, " chpri=0x{:x}", self.chpri()))}
      if self.dpa() != 0 { try!(write!(f, " dpa"))}
      if self.ecp() != 0 { try!(write!(f, " ecp"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="TCD Source Address"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TcdSaddr(pub u32);
impl TcdSaddr {
#[doc="Source Address"]
   #[inline] pub fn saddr(&self) -> bits::U32 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
   }
#[doc="Source Address"]
   #[inline] pub fn set_saddr<V: Into<bits::U32>>(mut self, value: V) -> Self {
      let value: bits::U32 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffffffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for TcdSaddr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for TcdSaddr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="TCD Signed Source Address Offset"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TcdSoff(pub u16);
impl TcdSoff {
#[doc="Source address signed offset"]
   #[inline] pub fn soff(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Source address signed offset"]
   #[inline] pub fn set_soff<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for TcdSoff {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for TcdSoff {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.soff() != 0 { try!(write!(f, " soff=0x{:x}", self.soff()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="TCD Transfer Attributes"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TcdAttr(pub u16);
impl TcdAttr {
#[doc="Destination Data Transfer Size"]
   #[inline] pub fn dsize(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
   }
#[doc="Destination Data Transfer Size"]
   #[inline] pub fn set_dsize<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x7 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Destination Address Modulo"]
   #[inline] pub fn dmod(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1f) as u8) } // [7:3]
   }
#[doc="Destination Address Modulo"]
   #[inline] pub fn set_dmod<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1f << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Source data transfer size"]
   #[inline] pub fn ssize(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
   }
#[doc="Source data transfer size"]
   #[inline] pub fn set_ssize<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x7 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Source Address Modulo."]
   #[inline] pub fn smod(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1f) as u8) } // [15:11]
   }
#[doc="Source Address Modulo."]
   #[inline] pub fn set_smod<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1f << 11);
      self.0 |= value << 11;
      self
   }

}
impl ::core::fmt::Display for TcdAttr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for TcdAttr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dsize() != 0 { try!(write!(f, " dsize=0x{:x}", self.dsize()))}
      if self.dmod() != 0 { try!(write!(f, " dmod=0x{:x}", self.dmod()))}
      if self.ssize() != 0 { try!(write!(f, " ssize=0x{:x}", self.ssize()))}
      if self.smod() != 0 { try!(write!(f, " smod=0x{:x}", self.smod()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="TCD Minor Byte Count (Minor Loop Disabled)"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TcdNbytesMlno(pub u32);
impl TcdNbytesMlno {
#[doc="Minor Byte Transfer Count"]
   #[inline] pub fn nbytes(&self) -> bits::U32 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
   }
#[doc="Minor Byte Transfer Count"]
   #[inline] pub fn set_nbytes<V: Into<bits::U32>>(mut self, value: V) -> Self {
      let value: bits::U32 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffffffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for TcdNbytesMlno {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for TcdNbytesMlno {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TcdNbytesMloffno(pub u32);
impl TcdNbytesMloffno {
#[doc="Minor Byte Transfer Count"]
   #[inline] pub fn nbytes(&self) -> bits::U30 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3fffffff) as u32) } // [29:0]
   }
#[doc="Minor Byte Transfer Count"]
   #[inline] pub fn set_nbytes<V: Into<bits::U30>>(mut self, value: V) -> Self {
      let value: bits::U30 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3fffffff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Destination Minor Loop Offset enable"]
   #[inline] pub fn dmloe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Destination Minor Loop Offset enable"]
   #[inline] pub fn set_dmloe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Source Minor Loop Offset Enable"]
   #[inline] pub fn smloe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Source Minor Loop Offset Enable"]
   #[inline] pub fn set_smloe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 31);
      self.0 |= value << 31;
      self
   }

}
impl ::core::fmt::Display for TcdNbytesMloffno {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for TcdNbytesMloffno {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.nbytes() != 0 { try!(write!(f, " nbytes=0x{:x}", self.nbytes()))}
      if self.dmloe() != 0 { try!(write!(f, " dmloe"))}
      if self.smloe() != 0 { try!(write!(f, " smloe"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TcdNbytesMloffyes(pub u32);
impl TcdNbytesMloffyes {
#[doc="Minor Byte Transfer Count"]
   #[inline] pub fn nbytes(&self) -> bits::U10 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
   }
#[doc="Minor Byte Transfer Count"]
   #[inline] pub fn set_nbytes<V: Into<bits::U10>>(mut self, value: V) -> Self {
      let value: bits::U10 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3ff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="If SMLOE or DMLOE is set, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
   #[inline] pub fn mloff(&self) -> bits::U20 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0xfffff) as u32) } // [29:10]
   }
#[doc="If SMLOE or DMLOE is set, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
   #[inline] pub fn set_mloff<V: Into<bits::U20>>(mut self, value: V) -> Self {
      let value: bits::U20 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xfffff << 10);
      self.0 |= value << 10;
      self
   }

#[doc="Destination Minor Loop Offset enable"]
   #[inline] pub fn dmloe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Destination Minor Loop Offset enable"]
   #[inline] pub fn set_dmloe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Source Minor Loop Offset Enable"]
   #[inline] pub fn smloe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Source Minor Loop Offset Enable"]
   #[inline] pub fn set_smloe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 31);
      self.0 |= value << 31;
      self
   }

}
impl ::core::fmt::Display for TcdNbytesMloffyes {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for TcdNbytesMloffyes {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.nbytes() != 0 { try!(write!(f, " nbytes=0x{:x}", self.nbytes()))}
      if self.mloff() != 0 { try!(write!(f, " mloff=0x{:x}", self.mloff()))}
      if self.dmloe() != 0 { try!(write!(f, " dmloe"))}
      if self.smloe() != 0 { try!(write!(f, " smloe"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="TCD Last Source Address Adjustment"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TcdSlast(pub u32);
impl TcdSlast {
#[doc="Last source Address Adjustment"]
   #[inline] pub fn slast(&self) -> bits::U32 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
   }
#[doc="Last source Address Adjustment"]
   #[inline] pub fn set_slast<V: Into<bits::U32>>(mut self, value: V) -> Self {
      let value: bits::U32 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffffffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for TcdSlast {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for TcdSlast {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="TCD Destination Address"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TcdDaddr(pub u32);
impl TcdDaddr {
#[doc="Destination Address"]
   #[inline] pub fn daddr(&self) -> bits::U32 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
   }
#[doc="Destination Address"]
   #[inline] pub fn set_daddr<V: Into<bits::U32>>(mut self, value: V) -> Self {
      let value: bits::U32 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffffffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for TcdDaddr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for TcdDaddr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="TCD Signed Destination Address Offset"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TcdDoff(pub u16);
impl TcdDoff {
#[doc="Destination Address Signed offset"]
   #[inline] pub fn doff(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Destination Address Signed offset"]
   #[inline] pub fn set_doff<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for TcdDoff {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for TcdDoff {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.doff() != 0 { try!(write!(f, " doff=0x{:x}", self.doff()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TcdCiterElinkno(pub u16);
impl TcdCiterElinkno {
#[doc="Current Major Iteration Count"]
   #[inline] pub fn citer(&self) -> bits::U15 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7fff) as u16) } // [14:0]
   }
#[doc="Current Major Iteration Count"]
   #[inline] pub fn set_citer<V: Into<bits::U15>>(mut self, value: V) -> Self {
      let value: bits::U15 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x7fff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Enable channel-to-channel linking on minor-loop complete"]
   #[inline] pub fn elink(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
   }
#[doc="Enable channel-to-channel linking on minor-loop complete"]
   #[inline] pub fn set_elink<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 15);
      self.0 |= value << 15;
      self
   }

}
impl ::core::fmt::Display for TcdCiterElinkno {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for TcdCiterElinkno {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.citer() != 0 { try!(write!(f, " citer=0x{:x}", self.citer()))}
      if self.elink() != 0 { try!(write!(f, " elink"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TcdCiterElinkyes(pub u16);
impl TcdCiterElinkyes {
#[doc="Current Major Iteration Count"]
   #[inline] pub fn citer(&self) -> bits::U9 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
   }
#[doc="Current Major Iteration Count"]
   #[inline] pub fn set_citer<V: Into<bits::U9>>(mut self, value: V) -> Self {
      let value: bits::U9 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1ff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Link Channel Number"]
   #[inline] pub fn linkch(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0xf) as u8) } // [12:9]
   }
#[doc="Link Channel Number"]
   #[inline] pub fn set_linkch<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0xf << 9);
      self.0 |= value << 9;
      self
   }

#[doc="Enable channel-to-channel linking on minor-loop complete"]
   #[inline] pub fn elink(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
   }
#[doc="Enable channel-to-channel linking on minor-loop complete"]
   #[inline] pub fn set_elink<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 15);
      self.0 |= value << 15;
      self
   }

}
impl ::core::fmt::Display for TcdCiterElinkyes {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for TcdCiterElinkyes {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.citer() != 0 { try!(write!(f, " citer=0x{:x}", self.citer()))}
      if self.linkch() != 0 { try!(write!(f, " linkch=0x{:x}", self.linkch()))}
      if self.elink() != 0 { try!(write!(f, " elink"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="TCD Last Destination Address Adjustment/Scatter Gather Address"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TcdDlastsga(pub u32);
impl TcdDlastsga {
#[doc="Destination last address adjustment or the memory address for the next transfer control descriptor to be loaded into this channel (scatter/gather)"]
   #[inline] pub fn dlastsga(&self) -> bits::U32 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
   }
#[doc="Destination last address adjustment or the memory address for the next transfer control descriptor to be loaded into this channel (scatter/gather)"]
   #[inline] pub fn set_dlastsga<V: Into<bits::U32>>(mut self, value: V) -> Self {
      let value: bits::U32 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffffffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for TcdDlastsga {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for TcdDlastsga {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="TCD Control and Status"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TcdCsr(pub u16);
impl TcdCsr {
#[doc="Channel Start"]
   #[inline] pub fn start(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Channel Start"]
   #[inline] pub fn set_start<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Enable an interrupt when major iteration count completes"]
   #[inline] pub fn intmajor(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Enable an interrupt when major iteration count completes"]
   #[inline] pub fn set_intmajor<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Enable an interrupt when major counter is half complete."]
   #[inline] pub fn inthalf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Enable an interrupt when major counter is half complete."]
   #[inline] pub fn set_inthalf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Disable Request"]
   #[inline] pub fn dreq(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Disable Request"]
   #[inline] pub fn set_dreq<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Enable Scatter/Gather Processing"]
   #[inline] pub fn esg(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Enable Scatter/Gather Processing"]
   #[inline] pub fn set_esg<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Enable channel-to-channel linking on major loop complete"]
   #[inline] pub fn majorelink(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Enable channel-to-channel linking on major loop complete"]
   #[inline] pub fn set_majorelink<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Channel Active"]
   #[inline] pub fn active(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Channel Active"]
   #[inline] pub fn set_active<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Channel Done"]
   #[inline] pub fn done(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Channel Done"]
   #[inline] pub fn set_done<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="Link Channel Number"]
   #[inline] pub fn majorlinkch(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
   }
#[doc="Link Channel Number"]
   #[inline] pub fn set_majorlinkch<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0xf << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Bandwidth Control"]
   #[inline] pub fn bwc(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x3) as u8) } // [15:14]
   }
#[doc="Bandwidth Control"]
   #[inline] pub fn set_bwc<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x3 << 14);
      self.0 |= value << 14;
      self
   }

}
impl ::core::fmt::Display for TcdCsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for TcdCsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.start() != 0 { try!(write!(f, " start"))}
      if self.intmajor() != 0 { try!(write!(f, " intmajor"))}
      if self.inthalf() != 0 { try!(write!(f, " inthalf"))}
      if self.dreq() != 0 { try!(write!(f, " dreq"))}
      if self.esg() != 0 { try!(write!(f, " esg"))}
      if self.majorelink() != 0 { try!(write!(f, " majorelink"))}
      if self.active() != 0 { try!(write!(f, " active"))}
      if self.done() != 0 { try!(write!(f, " done"))}
      if self.majorlinkch() != 0 { try!(write!(f, " majorlinkch=0x{:x}", self.majorlinkch()))}
      if self.bwc() != 0 { try!(write!(f, " bwc=0x{:x}", self.bwc()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TcdBiterElinkno(pub u16);
impl TcdBiterElinkno {
#[doc="Starting Major Iteration Count"]
   #[inline] pub fn biter(&self) -> bits::U15 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7fff) as u16) } // [14:0]
   }
#[doc="Starting Major Iteration Count"]
   #[inline] pub fn set_biter<V: Into<bits::U15>>(mut self, value: V) -> Self {
      let value: bits::U15 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x7fff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Enables channel-to-channel linking on minor loop complete"]
   #[inline] pub fn elink(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
   }
#[doc="Enables channel-to-channel linking on minor loop complete"]
   #[inline] pub fn set_elink<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 15);
      self.0 |= value << 15;
      self
   }

}
impl ::core::fmt::Display for TcdBiterElinkno {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for TcdBiterElinkno {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.biter() != 0 { try!(write!(f, " biter=0x{:x}", self.biter()))}
      if self.elink() != 0 { try!(write!(f, " elink"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TcdBiterElinkyes(pub u16);
impl TcdBiterElinkyes {
#[doc="Starting Major Iteration Count"]
   #[inline] pub fn biter(&self) -> bits::U9 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
   }
#[doc="Starting Major Iteration Count"]
   #[inline] pub fn set_biter<V: Into<bits::U9>>(mut self, value: V) -> Self {
      let value: bits::U9 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1ff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Link Channel Number"]
   #[inline] pub fn linkch(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0xf) as u8) } // [12:9]
   }
#[doc="Link Channel Number"]
   #[inline] pub fn set_linkch<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0xf << 9);
      self.0 |= value << 9;
      self
   }

#[doc="Enables channel-to-channel linking on minor loop complete"]
   #[inline] pub fn elink(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
   }
#[doc="Enables channel-to-channel linking on minor loop complete"]
   #[inline] pub fn set_elink<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 15);
      self.0 |= value << 15;
      self
   }

}
impl ::core::fmt::Display for TcdBiterElinkyes {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for TcdBiterElinkyes {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.biter() != 0 { try!(write!(f, " biter=0x{:x}", self.biter()))}
      if self.linkch() != 0 { try!(write!(f, " linkch=0x{:x}", self.linkch()))}
      if self.elink() != 0 { try!(write!(f, " elink"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
pub struct EdmaCh { pub periph: EdmaPeriph, pub index: usize }
