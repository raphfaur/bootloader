MEMORY {
    BOOT : ORIGIN = 0x00 , LENGTH = 512
    DATA : ORIGIN = 512, LENGTH = 1024
}

ENTRY(_start)

SECTIONS {
    .text : {
        FILL(0x00);
        *(*.text*);
        . = LENGTH(BOOT) - ORIGIN(BOOT) - 2;
        SHORT(0xaa55)
    } > BOOT

    .data : {
        *(*.data*);
    } > DATA

    .shstrtab : {
        *(.shstrtab)
    }

    /DISCARD/ : {
        *(.*)
    }
}