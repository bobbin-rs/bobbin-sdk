MEMORY
{
  FLASH             (rx) : ORIGIN = 0x08000000, LENGTH = 512K
  RAM              (xrw) : ORIGIN = 0x20000000, LENGTH = 256K
  DTCM             (xrw) : ORIGIN = 0x20000000, LENGTH = 64K 
  SRAM1            (xrw) : ORIGIN = 0x20010000, LENGTH = 176K
  SRAM2            (xrw) : ORIGIN = 0x2003C000, LENGTH = 16K  
}

/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
/* NOTE Do NOT modify `_stack_start` unless you know what you are doing */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);