MEMORY {
    BOOT : ORIGIN = 0x7c00 , LENGTH = 512
}

ENTRY(_start)

SECTIONS {
    .text : {
        . = ORIGIN(BOOT);
        FILL(0x00);
        *(*.text*);
        *(*.rodata*);
        *(*.data*);
        . = LENGTH(BOOT) + ORIGIN(BOOT) - 2;
        SHORT(0xaa55);
    } > BOOT

    .shstrtab : {
        *(.shstrtab)
    }

    /DISCARD/ : {
            *(.*)
        }


}