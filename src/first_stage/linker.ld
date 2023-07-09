MEMORY {
    BOOT : ORIGIN = 0x00 , LENGTH = 32768
}

ENTRY(_start)

SECTIONS {
    .text : {
        FILL(0x00);
        *(*.text*);
        . = LENGTH(BOOT) - ORIGIN(BOOT);
    } > BOOT

    .shstrtab : {
        *(.shstrtab)
    }

    /DISCARD/ : {
        *(.*)
    }
}