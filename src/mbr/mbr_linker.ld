MEMORY {
    BOOT : ORIGIN = 0x00 , LENGTH = 512
}

ENTRY(_start)

SECTIONS {
    .text : {
        FILL(0x00);
        *(*.text*);
        . = LENGTH(BOOT) - ORIGIN(BOOT) - 2;
        SHORT(0xaa55);
    } > BOOT

    .shstrtab : {
        *(.shstrtab)
    }

    /DISCARD/ : {
        *(.*)
    }
}