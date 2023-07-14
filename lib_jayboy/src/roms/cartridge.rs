use crate::roms::Licensee;
use crate::roms::*;
use crate::{gb_str, gb_u16, gb_u8};
use anyhow::{anyhow, Result};
use std::ffi::OsStr;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

/// [Pan Docs: The Cartridge Header](https://gbdev.io/pandocs/The_Cartridge_Header.html)
#[derive(Debug, Default)]
pub struct Cartridge {
    pub name: Box<OsStr>,
    pub bytes: Box<[u8]>,
}

// static methods
impl Cartridge {
    /// Attempts to load a `Cartridge` from a local Path
    pub fn load_from<P: AsRef<Path>>(path: &P) -> Result<Self> {
        let path = path.as_ref();
        let file_name = path
            .file_name()
            .ok_or(anyhow!("Invalid Path File Name: {}", path.display()))?;
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();

        // Read the entire ROM
        reader.read_to_end(&mut buffer)?;
        Ok(Cartridge {
            name: file_name.into(),
            bytes: buffer.into_boxed_slice(),
        })
    }
}

// instance methods
impl Cartridge {
    pub fn header_bytes(&self) -> &[u8] {
        &self.bytes[0x0100..=0x014F]
    }

    pub fn non_header_bytes(&self) -> &[u8] {
        &self.bytes[0x0150..]
    }

    /// ## `0x0100-0x0103` -- Entry Point
    /// After executing the boot ROM, the Game Boy will start executing at position `0x0100`
    pub fn entry_point(&self) -> &[u8] {
        &self.bytes[0x0100..=0x0103]
    }

    /// ## `0x0104-0x0133` -- Nintendo Logo
    /// This area contains a 'bitmap' image that is displayed when the Game Boy powers on.
    /// It must match a specific 48 bytes, the same as in the BOOT ROM, or the game will not run.
    pub fn logo_bytes(&self) -> &[u8] {
        &self.bytes[0x0104..=0x0133]
    }

    /// ## `0x0134-0x0143` -- Title
    /// These bytes contain the game's name in ASCII.  
    /// If the title is less than 16 characters, the remaining bytes should be empty padding bytes (`0x00`)  
    /// Parts of this field may have different meanings in later cartridges, reducing the size to 15 or even 11 bytes.
    pub fn title(&self) -> Option<gb_str> {
        // calculate title length

        let title_length = if self.manufacturer_code().is_some() {
            // First, check for manufacturer, it limits our size the most
            11
        } else if CGBFlag::not_none(self.bytes[0x0143]) {
            // check last byte for CGBFlag
            15
        } else {
            // default to the full 16
            16
        };
        // Total range we might use
        let title_bytes = &self.bytes[0x0134..=0x0143];

        // scan from end to front until we find a non-empty byte
        let last_index = (0..title_length)
            .rposition(|i| title_bytes[i] != 0x00)
            .unwrap_or(16);

        let str_bytes = &title_bytes[0..=last_index];
        let str_result = gb_str::try_from_ascii(str_bytes);
        match str_result {
            Ok(text) => Some(text),
            Err(ex) => {
                let man = self.manufacturer_code();
                let cgb = self.cgb_flag();
                warn!(
                    "{}: Invalid Name Bytes '{:?}'\nMan: {:?}, CGB: {:?}",
                    ex, str_bytes, man, cgb
                );
                None
            }
        }
    }

    /// ## `0x013F-0x0142` -- Manufacturer code
    /// In older carts, these bytes are part of `Title`.  
    /// In newer carts, they contain a manufacturer code (4 uppercase ASCII letters).  
    /// The purpose for this is unknown.
    pub fn manufacturer_code(&self) -> Option<gb_str> {
        let bytes = &self.bytes[0x013F..=0x142];
        let str_result = gb_str::try_from_uppercase_ascii(bytes);
        match str_result {
            Ok(text) => Some(text),
            Err(ex) => {
                warn!("{}: Invalid Manufacturer Code '{:?}'", ex, bytes);
                None
            }
        }
    }

    /// ## `0x0143` -- CGB flag
    /// In older carts, this byte is part of `Title`
    /// In later carts (and CGB titles), this indicates Color mode
    pub fn cgb_flag(&self) -> Option<CGBFlag> {
        self.bytes[0x0143].try_into().ok()
    }

    /// ## `0x0144-0x0145` -- New licensee
    /// Indicates the game's publishers.  
    /// Only relevant if `old_licensee` is 0x33 (true for all games after the CGB was released),
    /// otherwise the `old_licensee` must be considered.  
    /// See `&self.licensee()` for a method that can return a New or Old Licensee
    pub fn new_licensee(&self) -> Option<Licensee> {
        let bytes = [self.bytes[0x0144], self.bytes[0x145]];
        let licensee = Licensee::try_from(bytes);
        if let Ok(licensee) = licensee {
            Some(licensee)
        } else {
            // warn!(
            //     "Unknown New Licensee: 0x{:0>2X},0x{:0>2X} {:?}",
            //     bytes[0], bytes[1], self.file_name
            // );
            None
        }
    }

    /// ## `0x0146` -- SGB flag
    /// This byte specifies whether the game supports SGB functions.
    /// The SGB will ignore any `command packets` unless this is `true`
    pub fn sgb_support(&self) -> bool {
        // any value other than 0x03, usually 0x00
        let byte = self.bytes[0x0146];
        match byte {
            0x03 => true,
            0x00 => false,
            _ => {
                info!("Non-typical SGB support byte: {}", gb_u8::from(byte));
                false
            }
        }
    }

    /// ## `0x0147` -- Cartridge type
    /// This byte indicates what kind of hardware is present on the cartridge --
    /// **most notably its `mapper`**
    pub fn cartridge_type(&self) -> CartridgeType {
        let byte = self.bytes[0x0147];
        if let Ok(cart_type) = CartridgeType::try_from(byte) {
            cart_type
        } else {
            error!("Unknown Cart Type: {} {:?}", gb_u8::from(byte), self.name);
            //CartridgeType::default()
            panic!("UNKNOWN CART TYPE")
        }
    }

    /// ## `0x0148` -- ROM size
    /// How many bytes of ROM are present on this `cartridge`
    pub fn rom_size(&self) -> usize {
        let byte = self.bytes[0x0148];
        // In most cases, the ROM size is given by `32 KiB × (1 << <value>)`:
        match byte {
            ..=0x08 => 32 * (1 << byte) * 1024, // KiB -> bytes
            // ! unofficial, per Pan Docs ! (all three below)
            0x52 => (1024 + 128) * 1024,
            0x53 => (1024 + 256) * 1024,
            0x54 => (1024 + 512) * 1024,
            _ => panic!("Unknown ROM size byte: {}", gb_u8::from(byte)),
        }
    }

    /// ## `0x0149` -- RAM size
    /// How many bytes of RAM are present on this `cartridge`, if any
    pub fn ram_size(&self) -> usize {
        let byte = self.bytes[0x0149];
        match byte {
            0x00 => 0,
            // as per Pan Docs, this value was never used
            // except per PD (public domain homebrew files)
            // TODO: VERIFY
            0x01 => 2 * 1024,
            0x02 => 8 * 1024,
            0x03 => 32 * 1024,
            0x04 => 128 * 1024,
            0x05 => 64 * 1024,
            _ => {
                error!("Unknown RAM size byte: {}", gb_u8::from(byte));
                0
            }
        }
    }

    /// ## `0x14A` -- Destination code
    /// This byte specifies whether this version of the game is intended to be sold in
    /// Japan or Overseas.  
    /// Note: `Overseas` cartridges were also sold in `Japan` ¯\_(ツ)_/¯
    pub fn destination(&self) -> Destination {
        let byte = self.bytes[0x014A];
        match byte {
            0x00 => Destination::Japan,
            0x01 => Destination::Overseas,
            _ => {
                error!("Unknown Destination code byte: {}", gb_u8::from(byte));
                Destination::Overseas
            }
        }
    }

    /// ## `0x014B` -- Old licensee
    /// This byte is used in older (pre-SGB) carts to specify the game’s publisher.  
    /// However, the value `0x33` indicates that the `new_licensee` must be considered instead.
    /// **Note: The SGB will ignore any command packets unless this value is `0x33`**  
    /// See `&self.licensee()` for a method that can return a New or Old Licensee
    pub fn old_licensee(&self) -> Option<Licensee> {
        let byte = self.bytes[0x014B];
        let licensee = Licensee::try_from(byte);
        if let Ok(licensee) = licensee {
            Some(licensee)
        } else {
            warn!(
                "Unknown old licensee: {} {:?}",
                gb_u8::from(byte),
                self.name
            );
            None
        }
    }

    /// ## `0x014C` -- Mask ROM version number
    /// Specifies the version number of the game.  
    /// It is usually `0x00`
    pub fn version(&self) -> u8 {
        match self.bytes[0x014C] {
            0x00 => 0x00,
            def => {
                info!("Non-0 Version!");
                def
            }
        }
    }

    /// ## `0x14D` -- Header checksum
    /// An 8-bit checksum computed from `0x0134-0x14C` (Title through Version)
    /// **The BOOT ROM verifies this value!**
    pub fn header_checksum(&self) -> u8 {
        self.bytes[0x014D]
    }

    /// ## `0x014E-0x014F` -- Global checksum
    /// A 16-bit (big-endian) checksum computed from all the bytes in the Cartridge ROM (except these two bytes)
    pub fn global_checksum(&self) -> u16 {
        u16::from_be_bytes([self.bytes[0x14E], self.bytes[0x14F]])
    }

    pub fn licensee(&self) -> Licensee {
        let old_byte = self.bytes[0x014B];
        if old_byte == 0x33_u8 {
            self.new_licensee().unwrap()
        } else {
            self.old_licensee().unwrap()
        }
    }
}

impl Display for Cartridge {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(f, "File: {:?}", self.name)?;
        writeln!(f, "Title: {:?}", self.title())?;
        writeln!(f, "Publisher: {}", self.licensee())?;
        writeln!(f, "Type: {}", self.cartridge_type())?;
        writeln!(
            f,
            "ROM: {} KiB,  RAM: {} KiB",
            self.rom_size() / 1024,
            self.ram_size() / 1024
        )?;
        writeln!(
            f,
            "Dest: {:?},  Version: {}",
            self.destination(),
            self.version()
        )?;
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Destination {
    Japan,
    Overseas,
}

//#[cfg(test)]
pub mod cart_tests {
    use super::*;
    use anyhow::*;

    const LOGO_BYTES: [u8; 48] = [
        0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00,
        0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD,
        0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB,
        0xB9, 0x33, 0x3E,
    ];

    pub fn validate_cart(cart: &Cartridge) -> Result<()> {
        // validate the logo
        validate_nintendo_logo(cart)?;
        validate_ram_size_vs_cartridge_type(cart)?;
        let checksum = generate_header_checksum(cart);
        if checksum != cart.header_checksum() {
            return Err(anyhow!("Invalid Header Checksum"));
        }
        // TODO: This fails ~20% of the time for anything other than the most standard of carts
        // is this worth fixing?
        //let checksum = generate_global_checksum(cart);
        //if checksum != cart.global_checksum() {
        //    return Err(anyhow!("Invalid Global Checksum"));
        //}
        // more?
        Ok(())
    }

    pub fn validate_nintendo_logo(cart: &Cartridge) -> Result<()> {
        if cart.logo_bytes().eq(&LOGO_BYTES) {
            Ok(())
        } else {
            let cart_bytes = cart.logo_bytes();
            let logo_bytes = &LOGO_BYTES;
            let mut str = String::new();
            str.push_str(&format!("{:?} {:?}", cart.name, cart.title()));
            str.push('\n');
            str.push_str("Invalid Nintendo Logo!\n");
            str.push_str("Cart: ");
            for byte in cart_bytes {
                str.push_str(&format!("{:0>2X} ", byte));
            }
            str.push_str("\nLogo: ");
            for byte in logo_bytes {
                str.push_str(&format!("{:0>2X} ", byte));
            }

            error!("{}", str);

            Err(anyhow!("Invalid Nintendo Logo"))
        }
    }

    pub fn validate_ram_size_vs_cartridge_type(cart: &Cartridge) -> Result<()> {
        // If the cartridge type does not include “RAM” in its name, this should be set to 0.
        // This includes MBC2, since its 512 × 4 bits of memory are built directly into the mapper.
        let has_ram = cart.cartridge_type().ram;
        let ram_size = cart.ram_size();
        if has_ram && ram_size == 0 {
            Err(anyhow!("Cartridge Type indicates RAM, but RAM size is 0"))
        } else if !has_ram && ram_size != 0 {
            Err(anyhow!(
                "Cartridge Type indicates no RAM, but RAM size is {}",
                ram_size
            ))
        } else {
            Ok(())
        }
    }

    /// The BOOT ROM computes the header checksum as follows:
    /// ```C
    /// uint8_t checksum = 0;
    /// for (uint16_t address = 0x0134; address <= 0x014C; address++) {
    ///     checksum = checksum - rom[address] - 1;
    /// }
    /// ```
    pub fn generate_header_checksum(cart: &Cartridge) -> u8 {
        let mut checksum: u8 = 0;
        let bytes = cart.bytes.as_ref();
        assert!(bytes.len() >= 0x014C);
        for address in 0x0134..=0x014C_u16 {
            let byte = bytes[address as usize];
            checksum = u8::wrapping_sub(checksum, byte);
            checksum = u8::wrapping_sub(checksum, 1);
        }
        checksum
    }

    /// These bytes contain a 16-bit (big-endian) checksum simply computed as the sum of all the bytes of the cartridge ROM (except these two checksum bytes).
    pub fn generate_global_checksum(cart: &Cartridge) -> u16 {
        let mut checksum: u16 = 0;
        let bytes = cart.bytes.as_ref();
        assert!(bytes.len() >= 0x14D);
        // TODO: Is this for the entire ROM (as in _every_ other byte) or just this range below?
        // for byte in bytes[0x0100..=0x14D].iter() {
        //     checksum = u16::wrapping_add(checksum, *byte as u16);
        // }

        for (i, byte) in bytes.iter().enumerate() {
            if i == 0x014E || i == 0x014F {
                continue;
            }
            checksum = u16::wrapping_add(checksum, *byte as u16);
        }

        checksum
    }
}
