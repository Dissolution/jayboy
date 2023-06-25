#![allow(dead_code, unused)]

pub use crate::prelude::*;
use std::fs::File;
use std::io::{BufReader, Read};

mod cpu;
mod logger;
mod memory;
mod opcodes;
mod prelude;
mod registers;
mod roms;
mod timer;
mod utils;

fn main() {
    //display_dmg_rom().expect("WTF");

    let bin_path = "./roms/DMG_ROM.bin";
    let mut bytes = Roms::load_rom(bin_path).unwrap();

    // Create our processor
    let mut cpu = CPU::new();
    // HACK
    // load the bytes as our memory
    cpu.memory.set_boot_rom(&bytes);

    // Run it
    let mut opcode: u8;
    loop {
        //opcode = bytes[cpu.registers.PC];
    }
}

fn display_dmg_rom() -> std::io::Result<()> {
    let bin_path = "./roms/DMG_ROM.bin";
    let mut bytes = Roms::load_rom(bin_path)?;

    // Show them!
    println!("Idx:  _u8  0x_");
    for (i, byte) in bytes.iter().enumerate() {
        println!("{i:>3}:  {byte:>3}  {byte:X}");
    }

    Ok(())
}
