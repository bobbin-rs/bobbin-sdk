MEMORY
{
  FLASH             (rx) : ORIGIN = 0x08000000, LENGTH = 256K
  RAM              (xrw) : ORIGIN = 0x20000000, LENGTH = 192K
  SRAM2a           (xrw) : ORIGIN = 0x20030000, LENGTH = 32K
  SRAM2b           (xrw) : ORIGIN = 0x20038000, LENGTH = 32K
}

/* Place stack at the end of SRAM1 */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);
