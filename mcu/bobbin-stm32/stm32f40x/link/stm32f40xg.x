MEMORY
{
  FLASH             (rx) : ORIGIN = 0x08000000, LENGTH = 1024K
  RAM              (xrw) : ORIGIN = 0x20000000, LENGTH = 128K
  CCRAM            (xrw) : ORIGIN = 0x10000000, LENGTH = 64K  
}

/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
/* NOTE Do NOT modify `_stack_start` unless you know what you are doing */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);