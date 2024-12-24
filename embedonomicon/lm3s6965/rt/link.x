/* Memory layout of the LM3S6965 microcontroller */
/* 1K = 1 KiBi = 1024 bytes */ 

MEMORY
{
  FLASH : ORIGIN = 0x00000000 , LENGTH = 256K
  RAM : ORIGIN = 0x20000000 , LENGTH = 64K
}

/* The entry point is the reset handler */
ENTRY(ResetHandler);
EXTERN(RESET_VECTOR);

SECTIONS
{
  .vector_table ORIGIN(FLASH) :
  {
    /* 1st entry: 
    Initial Stack Pointer value.
    By allocating the stack from the top of the address space 
    and growing it downwards, the heap (dynamic memory allocation)
    could grow upwards without conflicting with the stack.
    */
    LONG(ORIGIN(RAM) + LENGTH(RAM));

    /* 2nd entry: 
    Reset vector initialized poiting to ResetHandler() */
    KEEP(*(.vector_table.reset_vector));

    /* 3rd .. 16th (14) entries: */
    KEEP(*(.vector_table.exceptions));
  } > FLASH

  /* The linker will place .text after the previous output 
  section which is .vector_table */

  /* Code: located in ROM */
  .text :
  {
    *(.text .text.*);
  } > FLASH

  /* Read-Only Data:
  Constant data (e.g., string literals, const variables).
  */
  .rodata :
  {
    *(.rodata .rodata.*);
  } > FLASH

  /* Block Started by Symbol
  Uninitialized global/static variables (static mut, let mut)
  */
  .bss :
  {
    /* _sbss is the start address of bss */
    _sbss = .;
    *(.bss .bss.*);
    _ebss = .;
    /* _ebss is the end address of bss */
  } > RAM

  /* Initialized global/static variables 
  The AT() address is where the initialization values for static
  variables are stored, in LMA in ROM. Later these values are used tp set
  the static variables in RAM.
  */
  .data : AT(ADDR(.rodata) + SIZEOF(.rodata))
  {
    _sdata = .;
    *(.data .data.*);
    _edata = .;
  } > RAM

  /* associate a symbol to the LMA to .data */
  _sidata = LOADADDR(.data);

  /* These sections are for stack trace when an exception occurs,  
  but stack unwinding on panics is not configured.
  */
  /DISCARD/ :
  {
    *(.ARM.exidx .ARM.exidx.*);
  }
}

/* in case these functions are not defined a default 
 handler is assigned */
PROVIDE(NMI = DefaultExceptionHandler);
PROVIDE(HardFault = DefaultExceptionHandler);
PROVIDE(MemManage = DefaultExceptionHandler);
PROVIDE(BusFault = DefaultExceptionHandler);
PROVIDE(UsageFault = DefaultExceptionHandler);
PROVIDE(SVCall = DefaultExceptionHandler);
PROVIDE(PendSV = DefaultExceptionHandler);
PROVIDE(SysTick = DefaultExceptionHandler);

