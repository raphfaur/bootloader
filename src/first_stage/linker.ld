MEMORY {
    BOOT : ORIGIN = 0x7e00 , LENGTH = 25600
}

ENTRY(_start)

SECTIONS {
    .text : {
        . = ORIGIN(BOOT);
        FILL(0x00);
        *(*.text*);
        *(*.rodata*);
        *(*.data*);
        . = LENGTH(BOOT) + ORIGIN(BOOT);
    } > BOOT

    .shstrtab : {
        *(.shstrtab)
    }

    /DISCARD/ : {
            *(.*)
        }


}