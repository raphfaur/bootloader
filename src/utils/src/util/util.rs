pub fn _init() {
    clear();
    let a = "Welcome !";
    for i in a.bytes() {
        printc(i as i8);
    }
}

pub fn switch_protected(gdt : Gdtr) {
    // Deactivate interrupts
    unsafe {
        asm!(
        "cli"
        )
    }
    //Write gdt to memory
    gdt.write();
    //Load gdtr
    gdt.load();
    // Set protected bit
    unsafe {
        asm!(
        "mov eax, cr0",
        "or al, 1",
        "mov cr0, eax"
        )
    }
    let os_ptr = 0xff as *mut fn();
    unsafe { write_volatile(os_ptr, kernel)}

    //Far jump
    unsafe {
        asm!(
        "jmp cs:0xff"
        )
    }


}