#![allow(dead_code, unused)]

mod rom_scanner;

extern crate core;
#[macro_use]
extern crate log;
extern crate simplelog;
use simplelog::*;

use crate::rom_scanner::RomScanner;
use anyhow::anyhow;
use lib_jayboy::*;
use rand::prelude::*;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    TermLogger::init(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Always,
    )
    .unwrap();

    //display_dmg_rom().expect("WTF");

    // let bin_path = "./files/DMG_ROM.bin";
    // let mut bytes = Roms::load_rom_bytes(bin_path).unwrap();
    //
    // // Create our processor
    // let mut cpu = CPU::new();
    // // HACK
    // // load the bytes as our memory
    // cpu.memory.set_boot_rom(&bytes);
    //
    // // Run it
    // let mut opcode: u8;
    // loop {
    //     //opcode = bytes[cpu.registers.PC];
    // }

    //let file = r"c:\gb_roms\Tetris (Japan) (En).gb";

    let scan_result = RomScanner::scan_roms();

    //let mut rand = ThreadRng::default();
    let mut files = get_rom_files().unwrap();
    let offset = 0; // updated as they pass
                    //files.shuffle(&mut rand);
    for (index, file) in files.iter().enumerate().skip(offset) {
        let load_cart = Cartridge::load_from(file);
        if let Ok(cart) = load_cart {
            let validate_result = crate::cart_tests::validate_cart(&cart);
            if validate_result.is_ok() {
                let cdisp = format!("{}", &cart);

                info!("#{}\n{}Validated: {:?}\n", index, cdisp, validate_result);
            } else {
                let cdisp = format!("{}", &cart);
                error!("#{}\n{}Validated: {:?}\n", index, cdisp, validate_result);
                sleep(Duration::from_secs(10));
            }

            //sleep(Duration::from_millis(100));
        } else {
            error!("Invalid Cart: {}", file.display());
            trace!("Offset should be: {}", index - 1);
            sleep(Duration::from_secs(10));
        }
    }

    println!("FIN");
}

fn display_dmg_rom() -> anyhow::Result<()> {
    let cart = Cartridge::load_from(&"./files/DMG_ROM.bin".to_string())?;
    let bytes = cart.bytes;

    // Show them!
    println!("Idx:  _u8  0x_");
    for (i, byte) in bytes.iter().enumerate() {
        println!("{i:>3}:  {byte:>3}  {byte:X}");
    }

    Ok(())
}

// Change this to point at your own roms!
// TODO: include gpl test roms
pub const ROMS_PATH: &str = r"c:\gb_roms\";

/// Returns a list of all the `.gb` files from `ROMS_PATH`
pub fn get_rom_files() -> anyhow::Result<Vec<PathBuf>> {
    let file_paths = fs::read_dir(ROMS_PATH)?
        .filter_map(|r| r.ok())
        .map(|de| de.path())
        .filter(|p| {
            if let Some(ext) = p.extension() {
                ext == "gb" // .gb is the extension for Game Boy rom files
            } else {
                false
            }
        })
        .collect::<Vec<PathBuf>>();
    Ok(file_paths)
}

pub fn load_rom_memory<P: AsRef<Path>>(path: P) -> anyhow::Result<VecMemory> {
    let path = path.as_ref();
    let file_name = path
        .file_name()
        .ok_or(anyhow!("Invalid Path File Name: {}", path.display()))?;
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();

    // Read the entire ROM
    reader.read_to_end(&mut buffer)?;
    Ok(VecMemory::new(buffer))
}
