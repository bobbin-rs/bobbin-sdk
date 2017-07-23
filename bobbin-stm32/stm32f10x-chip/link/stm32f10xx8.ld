MEMORY
{
  FLASH             (rx) : ORIGIN = 0x08000000, LENGTH = 64K
  RAM              (xrw) : ORIGIN = 0x20000000, LENGTH = 20K
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
  } > FLASH

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
    _ervector = .;
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
