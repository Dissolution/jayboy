#![allow(dead_code, unused)]

use lib_jayboy::*;

fn main() {
    //display_dmg_rom().expect("WTF");

    let bin_path = "./files/DMG_ROM.bin";
    let mut bytes = Roms::load_rom_bytes(bin_path).unwrap();

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

fn display_dmg_rom() -> anyhow::Result<()> {
    let bin_path = "./files/DMG_ROM.bin";
    let mut bytes = Roms::load_rom_bytes(bin_path)?;

    // Show them!
    println!("Idx:  _u8  0x_");
    for (i, byte) in bytes.iter().enumerate() {
        println!("{i:>3}:  {byte:>3}  {byte:X}");
    }

    Ok(())
}
