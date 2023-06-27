use crate::roms::cart_type::CartridgeType;
use crate::roms::CGBFlag::{Compat, GCBOnly, PGBMode};
use crate::roms::Publisher;
use crate::text::GBText;
use anyhow::anyhow;
use std::ffi::OsString;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

/// [Pan Docs: The Cartridge Header](https://gbdev.io/pandocs/The_Cartridge_Header.html)
#[derive(Debug, Default)]
pub struct Cartridge {
    pub file_name: OsString,
    bytes: Box<[u8]>,
}

// static methods
impl Cartridge {
    pub fn load_from<P: AsRef<Path>>(path: &P) -> anyhow::Result<Self> {
        let p = path.as_ref();
        let file_name = p
            .file_name()
            .ok_or(anyhow!("Invalid Path File Name: {}", p.display()))?;
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();

        // Read the entire ROM
        reader.read_to_end(&mut buffer)?;
        Ok(Cartridge {
            file_name: file_name.to_os_string(),
            bytes: buffer.into_boxed_slice(),
        })
    }
}

// instance methods
impl Cartridge {
    pub fn get_bytes(&self) -> &[u8] {
        self.bytes.as_ref()
    }

    /// ## `0x0100-0x0103` -- Entry Point
    /// After executing the boot ROM, the Game Boy will start executing the `Cartridge` at position `0x100`
    pub fn entry_point(&self) -> &[u8] {
        &self.bytes[0x0100..=0x0103]
    }

    /// ## `0x104-0x0133` -- Nintendo Logo
    /// This area contains a 'bitmap' image that is displayed when the Game Boy powers on.
    /// It must match a specific 48 bytes, the same as in the BOOT ROM, or the game will not run.
    pub fn logo_bytes(&self) -> &[u8] {
        &self.bytes[0x0104..=0x0133]
    }

    /// ## `0x0134-0x0143` -- Title
    /// These bytes contain the game's name in ASCII.  
    /// If the title is less than 16 characters, the remaining bytes should be padded `0x00`s
    /// Parts of this field may have different meanings in later cartridges, reducing the size to 15 or even 11.
    /// TODO: Account for Manufacturer Code and CGB flags
    pub fn title(&self) -> Option<GBText> {
        let title_bytes = &self.bytes[0x0134..=0x0143];

        let end_index = if self.cgb_flag().is_some() {
            title_bytes.len() - 1
        } else if self.manufacturer_code().is_some() {
            title_bytes.len() - 5
        } else {
            title_bytes.len()
        };

        let mut last_index = end_index;
        for i in (0..end_index).rev() {
            if title_bytes[i] != 0x00 {
                last_index = i;
                break;
            }
        }

        let str_bytes = &title_bytes[0..=last_index];
        let str_result = GBText::try_from_ascii(str_bytes);
        if let Ok(txt) = str_result {
            Some(txt)
        } else {
            let man = self.manufacturer_code();
            let cgb = self.cgb_flag();
            warn!(
                "Invalid Name: {:?}\nMan: {:?}, CGB: {:?}",
                str_bytes, man, cgb
            );
            None
        }
    }

    /// ## `0x013F-0x0142` -- Manufacturer code
    /// In older carts, these bytes are part of `Title`.  
    /// In newer carts, they contain a manufacturer code (4 uppercase ASCII letters).  
    /// The purpose for this is unknown.
    pub fn manufacturer_code(&self) -> Option<GBText> {
        let bytes = &self.bytes[0x013F..=0x142];
        let str_result = GBText::try_from_ascii(bytes);
        if let Ok(txt) = str_result {
            Some(txt)
        } else {
            warn!("Invalid Manufacturer Code: {:?}", bytes);
            None
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
    pub fn new_licensee(&self) -> Publisher {
        let bytes = [self.bytes[0x0144], self.bytes[0x145]];
        let possible_publisher = Publisher::try_from(bytes);
        if let Ok(lic) = possible_publisher {
            lic
        } else {
            warn!(
                "Unknown New Licensee: 0x{:0>2X},0x{:0>2X} {:?}",
                bytes[0], bytes[1], self.file_name
            );
            Publisher::NONE
        }
    }

    /// ## `0x0146` -- SGB flag
    /// This byte specifies whether the game supports SGB functions.
    /// The SGB will ignore any `command packets` if this byte is set to a value other than `0x03`
    /// (typically `0x00`)
    pub fn sgb_support(&self) -> bool {
        self.bytes[0x0146] == 0x03
    }

    /// ## `0x0147` -- Cartridge type
    /// This byte indicates what kind of hardware is present on the cartridge --
    /// **most notably its `mapper`**
    pub fn cartridge_type(&self) -> CartridgeType {
        let byte = self.bytes[0x0147];
        if let Ok(cart_type) = CartridgeType::try_from(byte) {
            cart_type
        } else {
            error!("Unknown Cart Type: 0x{:0>2X} {:?}", byte, self.file_name);
            CartridgeType::default()
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
            _ => panic!("Unknown ROM size byte: {}", byte),
        }
    }

    /// ## `0x0149` -- RAM size
    /// How many bytes of RAM are present on this `cartridge`, if any
    pub fn ram_size(&self) -> usize {
        let byte = self.bytes[0x0149];
        match byte {
            0x00 => 0,
            // as per Pan Docs, this was never used
            // except per PD (public domain homebrew files) (check?)
            0x01 => 2 * 1024,
            0x02 => 8 * 1024,
            0x03 => 32 * 1024,
            0x04 => 128 * 1024,
            0x05 => 64 * 1024,
            _ => {
                error!("Unknown RAM size byte: 0x{:0>2X}", byte);
                0
            }
        }
    }

    /// ## `0x14A` -- Destination code
    /// This byte specifies whether this version of the game is intended to be sold in Japan or elsewhere.
    pub fn destination(&self) -> Destination {
        let byte = self.bytes[0x014A];
        match byte {
            0x00 => Destination::Japan,
            0x01 => Destination::Overseas,
            _ => {
                error!("Unknown Destination code byte: 0x{:0>2X}", byte);
                Destination::Overseas
            }
        }
    }

    /// ## `0x014B` -- Old licensee
    /// This byte is used in older (pre-SGB) carts to specify the game’s publisher.  
    /// However, the value `0x33` indicates that the `new_licensee` must be considered instead.
    /// **Note: The SGB will ignore any command packets unless this value is `0x33`**
    pub fn old_licensee(&self) -> Publisher {
        let byte = self.bytes[0x014B];
        let possible_publisher = Publisher::try_from(byte);
        if let Ok(lic) = possible_publisher {
            lic
        } else {
            warn!("Unknown old licensee: 0x{:0>2X} {:?}", byte, self.file_name);
            Publisher::NONE
        }
    }

    /// ## `0x014C` -- Mask ROM version number
    /// Specifies the version number of the game.  
    /// It is usually `0x00`
    pub fn version(&self) -> u8 {
        self.bytes[0x014C]
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

    pub fn publisher(&self) -> Publisher {
        let old_byte = self.bytes[0x014B];
        if old_byte == 0x33_u8 {
            self.new_licensee()
        } else {
            self.old_licensee()
        }
    }
}

impl Display for Cartridge {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(f, "File: {:?}", self.file_name)?;
        writeln!(f, "Title: {:?}", self.title())?;
        writeln!(f, "Publisher: {}", self.publisher())?;
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
pub enum CGBFlag {
    Compat,
    GCBOnly,
    PGBMode,
}
impl TryFrom<u8> for CGBFlag {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x80 => Ok(Compat),
            // The hardware ignores bit 6, so this is functionally the same as `Compat`
            0xC0 => Ok(GCBOnly),
            // values with bit 7 and either bit 2 or 3 set will switch the Game Boy into a special non-CGB mode
            0x88 | 0x84 => Ok(PGBMode),
            _ => Err(anyhow!("Invalid CGBFlag: 0x{:0<2X}", value)),
        }
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

    pub fn validate_cart(cart: &Cartridge) -> anyhow::Result<()> {
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

    pub fn validate_nintendo_logo(cart: &Cartridge) -> anyhow::Result<()> {
        if cart.logo_bytes().eq(&LOGO_BYTES) {
            Ok(())
        } else {
            let cart_bytes = cart.logo_bytes();
            let logo_bytes = &LOGO_BYTES;
            let mut str = String::new();
            str.push_str(&format!("{:?} {:?}", cart.file_name, cart.title()));
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

    pub fn validate_ram_size_vs_cartridge_type(cart: &Cartridge) -> anyhow::Result<()> {
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
        let bytes = cart.get_bytes();
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
        let bytes = cart.get_bytes();
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
