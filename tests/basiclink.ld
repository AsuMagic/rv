OUTPUT_FORMAT("elf32-littleriscv", "elf32-littleriscv", "elf32-littleriscv")
ENTRY(_start)

MEMORY
{
  DATA (rwx) : ORIGIN = 0x0, LENGTH = 0x80
  INST (rx) : ORIGIN = 0x80, LENGTH = 0x80
}

SECTIONS
{
  .data :
  {
    *(.data)
  }> DATA

  .text :
  {
    *(.text)
  }> INST
}
