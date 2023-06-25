mod cart;

use crate::logger::*;
use crate::prelude::*;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

pub use cart::*;

pub struct Roms;
impl Roms {
    pub fn load_rom<P: AsRef<Path>>(path: P) -> core::result::Result<Vec<u8>, std::io::Error> {
        Logger::debug(&format!("Loading {}...", path.as_ref().display()));
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();

        // Read the entire ROM
        reader.read_to_end(&mut buffer)?;
        Logger::debug(&format!("Loaded {} bytes", buffer.len()));
        Ok(buffer)
    }
}
