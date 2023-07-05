use std::{env, io};
use std::io::{stdout, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};


fn main(){

    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let mbr_path: PathBuf = root.join("mbr");
    let first_stage_path =  root.join("first_stage");
    let build_path = root.join("builds");

    // Build MDR
    println!("Building MBR...");
    let output = Command::new("cargo")
        .current_dir(&mbr_path)
        .arg("build")
        .arg("--target")
        .arg("x86_16bits_mbr.json")
        .arg("-Zbuild-std=core,alloc")
        .arg("--release")
        .output()
        .expect("Failed to build MBR");
    if !output.status.success() {
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
    } else {
        println!("MDR was successfully built")
    }

    // Cleaning MBR
    println!("Cleaning MBR...");
    let output = Command::new("x86_64-elf-objcopy")
        .current_dir(&mbr_path)
        .arg("-O")
        .arg("binary")
        .arg("target/x86_16bits_mbr/release/mbr")
        .arg(env::current_dir().unwrap().join("builds").join("mbr.bin"))
        .output()
        .expect("Failed to clean MBR");
    if !output.status.success() {
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
    } else {
        println!("MDR was successfully cleaned")
    }

    // Build first_stage
    println!("Building first stage...");
    let output = Command::new("cargo")
        .current_dir(&first_stage_path)
        .arg("build")
        .arg("--target")
        .arg("x86_16bits.json")
        .arg("-Zbuild-std=core,alloc")
        .arg("--release")
        .output()
        .expect("Failed to build MBR");
    if !output.status.success() {
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
    } else {
        println!("First stage was successfully built")
    }

    // Cleaning MBR
    println!("Cleaning MBR...");
    let output = Command::new("x86_64-elf-objcopy")
        .current_dir(&first_stage_path)
        .arg("-O")
        .arg("binary")
        .arg("target/x86_16bits/release/first_stage")
        .arg(env::current_dir().unwrap().join("builds").join("first_stage.bin"))
        .output()
        .expect("Failed to clean MBR");
    if !output.status.success() {
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
    } else {
        println!("MBR was successfully cleaned")
    }

    // Merge files
    let output = Command::new("dd")
        .current_dir(&build_path)
        .arg("if=first_stage.bin")
        .arg("of=mbr.bin")
        .arg("seek=1")
        .output()
        .expect("Could not run dd");

    if !output.status.success() {
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
    } else {
        println!("Added first_stage at end of MBR")
    }

    let output = Command::new("mv")
        .current_dir(&build_path)
        .arg("mbr.bin")
        .arg("boot")
        .output()
        .expect("Failed to rename");

    println!("Done, booting");

    let output = Command::new("qemu-system-i386")
        .current_dir(&build_path)
        .arg("-monitor")
        .arg("stdio")
        .arg("boot")
        .spawn()
        .expect("Falied to launch Qemu");








}