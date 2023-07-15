.global _start
.code16


_start:
    xor ax, ax
    mov ax, 0x00
    mov ds, ax
    mov es, ax
    mov ax, ax
    mov ss, ax
    mov fs, ax
    mov gs, ax

    cld

    # enable A20-Line via IO-Port 92, might not work on all motherboards
        in al, 0x92
        test al, 2
        or al, 2
        and al, 0xFE
        out 0x92, al

    mov sp, 0x7C00
    call main

spin:
    jmp spin