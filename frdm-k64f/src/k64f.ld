MEMORY
{
  VECTORS (rx) : ORIGIN = 0x00000000, LENGTH = 0x00000400
  FLASH_PROTECTION	(rx) : ORIGIN = 0x00000400, LENGTH = 0x00000010
  FLASH (rx) : ORIGIN = 0x00000410, LENGTH = 1024K - 0x00000410
  RAM (rwx) : ORIGIN = 0x1FFFF000, LENGTH = 64K
}

ENTRY(_reset)

SECTIONS
{
  .vector :
  {
    _svector = .;
    LONG(ORIGIN(RAM) + LENGTH(RAM));
    KEEP(*(.vector.exceptions));
    KEEP(*(.vector.interrupts));    
    _evector = ALIGN(4);    
  } > VECTORS
  
  .flash_protect :
  {
      KEEP(*(.kinetis_flash_config_field))
      . = ALIGN(4);
      LONG(0xffffffff)
      LONG(0xffffffff)
      LONG(0xffffffff)
      LONG(0xfffffffe)
  } > FLASH_PROTECTION

  .text : ALIGN(4)
  {
    /* Entry point: reset handler */
    _reset = .;
    KEEP(*(.text._reset));
    KEEP(*(.text._default_exception_handler));
    KEEP(*(.text._default_exception_handler_impl));
    *(.text.*);
    *(.rodata.*);
  } > FLASH

  .bss : ALIGN(4)
  {
    _srvector = .;
    . = . + 4;
    KEEP(*(.bss.r_exceptions));
    KEEP(*(.bss.r_interrupts));    
    _ervector = ALIGN(4);    
    _sbss = .;
    *(.bss.*);
    _ebss = ALIGN(4);
  } > RAM

  .data : ALIGN(4)
  {
    _sdata = .;
    *(.data.*);
    _edata = ALIGN(4);
  } > RAM AT > FLASH

  _sidata = LOADADDR(.data);
  _sivector = LOADADDR(.vector);

  /DISCARD/ :
  {
    *(.ARM.exidx.*)
  }
}

/* HACK to make these symbols overrideable by _dependencies_ (they were already
   overridable by the top crate), we declare them as undefined (EXTERN) here. */
EXTERN(_default_exception_handler);

/* Exceptions */
PROVIDE(_reset = _reset);
PROVIDE(_nmi = _default_exception_handler);
PROVIDE(_hard_fault = _default_exception_handler);
PROVIDE(_memmanage_fault = _default_exception_handler);
PROVIDE(_bus_fault = _default_exception_handler);
PROVIDE(_usage_fault = _default_exception_handler);
PROVIDE(_svcall = _default_exception_handler);
PROVIDE(_debug = _default_exception_handler);
PROVIDE(_pendsv = _default_exception_handler);
PROVIDE(_systick = _default_exception_handler);
