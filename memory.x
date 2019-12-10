MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  /* TODO Adjust these memory regions to match your device memory layout */
  /* These values correspond to the LM3S6965, one of the few devices QEMU can emulate */
/*  FLASH : ORIGIN = 0x00000000, LENGTH = 256K */
/*  RAM : ORIGIN = 0x20000000, LENGTH = 64K */

/*

SAM3X layout

0x0008 0000 - 0x000B FFFF   256 KiB flash bank 0
0x000C 0000 - 0x000F FFFF   256 KiB flash bank 1
                            Both banks above provide 512 KiB of contiguous flash memory
0x2000 0000 - 0x2000 FFFF   64 KiB SRAM0
0x2007 0000 - 0x2007 FFFF   64 KiB mirrored SRAM0, so that it's consecutive with SRAM1
0x2008 0000 - 0x2008 7FFF   32 KiB SRAM1
0x2010 0000 - 0x2010 107F   4224 bytes of NAND flash controller buffer
*/

  /*

  DUE LAYOUT

  FLASH : ORIGIN = 0x00080000, LENGTH = 512K
  RAM : ORIGIN = 0x20000000, LENGTH = 64K
  */

  /*
  STM32F407 LAYOUT
  https://github.com/timbod7/rust-stm32f4-examples/blob/master/blinky/memory.x
  */
  FLASH : ORIGIN = 0x08000000, LENGTH = 256K
  RAM : ORIGIN = 0x20000000, LENGTH = 40K
}

/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
/* You may want to use this variable to locate the call stack and static
   variables in different memory regions. Below is shown the default value */
/* _stack_start = ORIGIN(RAM) + LENGTH(RAM); */

/* You can use this symbol to customize the location of the .text section */
/* If omitted the .text section will be placed right after the .vector_table
   section */
/* This is required only on microcontrollers that store some configuration right
   after the vector table */
/* _stext = ORIGIN(FLASH) + 0x400; */

/* Example of putting non-initialized variables into custom RAM locations. */
/* This assumes you have defined a region RAM2 above, and in the Rust
   sources added the attribute `#[link_section = ".ram2bss"]` to the data
   you want to place there. */
/* Note that the section will not be zero-initialized by the runtime! */
/* SECTIONS {
     .ram2bss (NOLOAD) : ALIGN(4) {
       *(.ram2bss);
       . = ALIGN(4);
     } > RAM2
   } INSERT AFTER .bss;
*/