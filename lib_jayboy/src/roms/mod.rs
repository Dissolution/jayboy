mod cart;
mod cart_type;
mod publisher;

use crate::logger::*;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

pub use cart::*;
pub use publisher::*;

pub struct Roms;
impl Roms {
    pub fn load_rom_bytes<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<u8>> {
        Logger::debug(&format!("Loading {}...", path.as_ref().display()));
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();

        // Read the entire ROM
        reader.read_to_end(&mut buffer)?;
        Logger::debug(&format!("Loaded {} bytes", buffer.len()));
        Ok(buffer)
    }

    pub fn load_cartridge<P: AsRef<Path>>(path: P) -> anyhow::Result<Cartridge> {
        let bytes = Roms::load_rom_bytes(path)?;
        let cart = Cartridge::from_bytes(bytes);
        Ok(cart)
    }
}
