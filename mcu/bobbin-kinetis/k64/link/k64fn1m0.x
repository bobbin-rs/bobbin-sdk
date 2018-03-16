MEMORY
{
  VECTORS (rx) : ORIGIN = 0x00000000, LENGTH = 0x00000400
  FLASH_PROTECTION	(rx) : ORIGIN = 0x00000400, LENGTH = 0x00000010
  FLASH (rx) : ORIGIN = 0x00000410, LENGTH = 1024K - 0x00000410
  RAM (rwx) : ORIGIN = 0x1FFF0000, LENGTH = 64K + 192K
}

SECTIONS {
  .flash_protect :
  {
      KEEP(*(.kinetis_flash_config_field))
      . = ALIGN(4);
      LONG(0xffffffff)
      LONG(0xffffffff)
      LONG(0xffffffff)
      LONG(0xfffffffe)
  } > FLASH_PROTECTION
}


/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
/* NOTE Do NOT modify `_stack_start` unless you know what you are doing */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);
