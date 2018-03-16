MEMORY
{
  FLASH             (rx) : ORIGIN = 0x00000000, LENGTH = 0x00040000
  RAM              (xrw) : ORIGIN = 0x20000000, LENGTH = 0x00008000
}

/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
/* NOTE Do NOT modify `_stack_start` unless you know what you are doing */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);