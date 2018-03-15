MEMORY
{
  FLASH             (rx) : ORIGIN = 0x08000000, LENGTH = 2048K
  RAM              (xrw) : ORIGIN = 0x20000000, LENGTH = 512K
  DTCM             (xrw) : ORIGIN = 0x20000000, LENGTH = 128K 
  SRAM1            (xrw) : ORIGIN = 0x20020000, LENGTH = 368K
  SRAM2            (xrw) : ORIGIN = 0x2007C000, LENGTH = 16K
}

/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
/* NOTE Do NOT modify `_stack_start` unless you know what you are doing */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);