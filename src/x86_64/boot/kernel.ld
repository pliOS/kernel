ENTRY(start)
OUTPUT_FORMAT(elf32-i386)

KERNEL_PHYSICAL_BASE = 0x00100000;
KERNEL_OFFSET        = 0xFFFF800000000000;

KERNEL_VIRTUAL_BASE  = KERNEL_PHYSICAL_BASE + KERNEL_OFFSET;

SECTIONS {
    . = KERNEL_PHYSICAL_BASE;

    .early_text :
    {
        KEEP(*(.multiboot_header))
        *(.early_text)
    }

   .early_data ALIGN(0x1000): {
        *(.early_rodata)
        *(.early_data)
   }

    . += KERNEL_OFFSET;

    .text ALIGN(0x1000) : AT(ADDR(.text) - KERNEL_OFFSET) {
        *(.text*)
    }

    .rodata ALIGN(0x1000) : AT(ADDR(.rodata) - KERNEL_OFFSET) {
        *(.rodata*)
    }

    .data ALIGN(0x1000) : AT(ADDR(.data) - KERNEL_OFFSET) {
        *(.data*)
    }

    .bss ALIGN(0x1000) : AT(ADDR(.bss) - KERNEL_OFFSET) {
        *(.bss*)
    }

    /DISCARD/ : {
        *(.comment*)
        *(.debug*)
        *(.eh_frame*)
        *(.gcc_except_table*)
        *(.note*)
        *(.rel.eh_frame*)
    }
}
