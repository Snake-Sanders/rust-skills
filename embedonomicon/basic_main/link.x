/* Memory layout of the RP2040 microcontroller */
/* 1K = 1 KiBi = 1024 bytes */ 

MEMORY
{
  FLASH : ORIGIN = 0x10000000 , LENGTH = 2048K
  RAM : ORIGIN = 0x20000000 , LENGTH = 264K
}

/* The entry point is the reset handler */
ENTRY(ResetHandler);
EXTERN(RESET_VECTOR);

SECTIONS
{
  .vector_table ORIGIN(FLASH) :
  {
    /* First entry: 
    Initial Stack Pointer value.
    By allocating the stack from the top of the address space 
    and growing it downwards, the heap (dynamic memory allocation)
    could grow upwards without conflicting with the stack.
    */
    LONG(ORIGIN(RAM) + LENGTH(RAM));

    /* Second entry: 
    Reset vector initialized poiting to ResetHandler() */
    KEEP(*(.vector_table.reset_vector));
  } > FLASH

  /* The linker will place .text after the previous output 
  section: .vector_table */
  .text :
  {
    *(.text .text.*);
  } > FLASH

  /* These sections are for stack trace when an exception occurs,  
  but stack unwinding on panics is not configured.
  */
  /DISCARD/ :
  {
    *(.ARM.exidx .ARM.exidx.*);
  }
}
