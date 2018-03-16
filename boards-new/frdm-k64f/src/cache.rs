pub fn init() {
    // Disable Watchdog
//     // Write 0xC520 followed by 0xD928 within 20 bus clock cycles to a specific unlock register (WDOG_UNLOCK).
//     unsafe { 
//         asm!("

//         /* unlock */

//         ldr r1, =0x4005200e
//         ldr r0, =0xc520
//         strh r0, [r1]
//         ldr r0, =0xd928
//         strh r0, [r1]

//         /* disable */

//         ldr r1, =0x40052000
//         /* ldr r0, =0x01d2 */
//         ldr r0, =0x00d2 
//         strh r0, [r1]
//         ");
//     }
// }
}