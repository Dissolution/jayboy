use anyhow::anyhow;
use byteorder::{ByteOrder, LittleEndian};
use num_enum::{IntoPrimitive, TryFromPrimitive};

/// [Pan Docs: The Cartridge Header](https://gbdev.io/pandocs/The_Cartridge_Header.html)
#[derive(Debug, Default)]
pub struct Cartridge {
    bytes: Box<[u8]>,
}

impl Cartridge {
    pub fn entry_point(&self) -> &[u8] {
        &self.bytes[0x0100..=0x0103]
    }
    pub fn nintendo_logo(&self) -> &[u8] {
        &self.bytes[0x0104..=0x0133]
    }

    pub fn title(&self) -> String {
        // todo: handle manufacturer code and CGB flag!
        let title: String;

        let title_bytes = self.bytes[0x0134..=0x0143]
            .iter()
            .copied()
            .take_while(|b| *b != 0_u8)
            .collect();
        let title: String = String::from_utf8(title_bytes).unwrap();
        title
    }

    pub fn new_licensee(&self) -> NewLicensee {
        let bytes = &self.bytes[0x0144..=0x145];
        let new_licensee_code = LittleEndian::read_u16(bytes);
        NewLicensee::try_from(new_licensee_code).expect("Unknown Licensee!")
    }

    /// ## `0x0146` -- SGB flag
    /// This byte specifies whether the game supports SGB functions.
    /// The SGB will ignore any `command packets` if this byte is set to a value other than `0x03`
    /// (typically `0x00`)
    pub fn sgb(&self) -> u8 {
        self.bytes[0x0146]
    }

    /// ## `0x0147` -- Cartridge type
    /// This byte indicates what kind of hardware is present on the cartridge --
    /// **most notably its `mapper`**
    pub fn cartridge_type(&self) -> CartridgeType {
        let byte = self.bytes[0x0147];
        CartridgeType::try_from(byte).expect("Unknown cartridge type!")
    }

    /// ## `0x0148` -- ROM size
    /// How many bytes of ROM are present on this `cartridge`
    pub fn rom_size(&self) -> usize {
        let byte = self.bytes[0x0148];
        /// In most cases, the ROM size is given by `32 KiB × (1 << <value>)`:
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
            // except per PD (public domain homebrew roms) (check?)
            0x01 => 2 * 1024,
            0x02 => 8 * 1024,
            0x03 => 32 * 1024,
            0x04 => 128 * 1024,
            0x05 => 64 * 1024,
            _ => panic!("Unknown RAM size byte: {}", byte),
        }
    }

    /// ## `0x14A` -- Destination code
    /// This byte specifies whether this version of the game is intended to be sold in Japan or elsewhere.
    pub fn destination(&self) -> Destination {
        let byte = self.bytes[0x014A];
        match byte {
            0x00 => Destination::Japan,
            0x01 => Destination::Overseas,
            _ => panic!("Unknown Destination code byte: {}", byte),
        }
    }
}

pub enum Destination {
    Japan,
    Overseas,
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct CartridgeType {
    pub mbc: u8,
    pub ram: bool,
    pub battery: bool,
    pub rom: bool,
    pub mmm01: bool,
    pub timer: bool,
    pub rumble: bool,
    pub sensor: bool,
    pub camera: bool,
    pub bandai_tama5: bool,
    pub huc: u8,
}
impl CartridgeType {}

impl TryFrom<u8> for CartridgeType {
    type Error = anyhow::Error;

    // TODO: Pan Docs says I need to watch for Pocket Monsters: Crystal Version
    // to know that MBC3 might be MCB30
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            // ROM ONLY
            0x00 => Ok(CartridgeType {
                rom: true,
                ..Default::default()
            }),
            0x01 => Ok(CartridgeType {
                mbc: 1,
                ..Default::default()
            }),
            0x02 => Ok(CartridgeType {
                mbc: 1,
                ram: true,
                ..Default::default()
            }),
            0x03 => Ok(CartridgeType {
                mbc: 1,
                ram: true,
                battery: true,
                ..Default::default()
            }),
            // 0x04 => DNE
            0x05 => Ok(CartridgeType {
                mbc: 2,
                ..Default::default()
            }),
            0x06 => Ok(CartridgeType {
                mbc: 2,
                battery: true,
                ..Default::default()
            }),
            // 0x07 => DNE
            // This cart type should not exist (no licensed cart does)
            0x08 => Ok(CartridgeType {
                rom: true,
                ram: true,
                ..Default::default()
            }),
            // This cart type should not exist (no licensed cart does)
            0x09 => Ok(CartridgeType {
                rom: true,
                ram: true,
                battery: true,
                ..Default::default()
            }),
            // 0x0A => DNE
            0x0B => Ok(CartridgeType {
                mmm01: true,
                ..Default::default()
            }),
            0x0C => Ok(CartridgeType {
                mmm01: true,
                ram: true,
                ..Default::default()
            }),
            0x0D => Ok(CartridgeType {
                mmm01: true,
                ram: true,
                battery: true,
                ..Default::default()
            }),
            // 0x0E => DNE
            0x0F => Ok(CartridgeType {
                mbc: 3,
                timer: true,
                battery: true,
                ..Default::default()
            }),
            0x10 => Ok(CartridgeType {
                mbc: 3,
                timer: true,
                ram: true,
                battery: true,
                ..Default::default()
            }),
            0x11 => Ok(CartridgeType {
                mbc: 3,
                ..Default::default()
            }),
            0x12 => Ok(CartridgeType {
                mbc: 3,
                ram: true,
                ..Default::default()
            }),
            0x13 => Ok(CartridgeType {
                mbc: 3,
                ram: true,
                battery: true,
                ..Default::default()
            }),
            // 0x14 -> 0x18 => DNE
            0x19 => Ok(CartridgeType {
                mbc: 5,
                ..Default::default()
            }),
            0x1A => Ok(CartridgeType {
                mbc: 5,
                ram: true,
                ..Default::default()
            }),
            0x1B => Ok(CartridgeType {
                mbc: 5,
                ram: true,
                battery: true,
                ..Default::default()
            }),
            0x1C => Ok(CartridgeType {
                mbc: 5,
                rumble: true,
                ..Default::default()
            }),
            0x1D => Ok(CartridgeType {
                mbc: 5,
                rumble: true,
                ram: true,
                ..Default::default()
            }),
            0x1E => Ok(CartridgeType {
                mbc: 5,
                rumble: true,
                ram: true,
                battery: true,
                ..Default::default()
            }),
            // 0x1F => DNE
            0x20 => Ok(CartridgeType {
                mbc: 6,
                ..Default::default()
            }),
            // 0x21 => DNE
            0x22 => Ok(CartridgeType {
                mbc: 7,
                sensor: true,
                rumble: true,
                ram: true,
                battery: true,
                ..Default::default()
            }),
            // 0x23 -> 0xFB => DNE
            0xFC => Ok(CartridgeType {
                camera: true,
                ..Default::default()
            }),
            0xFD => Ok(CartridgeType {
                bandai_tama5: true,
                ..Default::default()
            }),
            0xFE => Ok(CartridgeType {
                huc: 3,
                ..Default::default()
            }),
            0xFF => Ok(CartridgeType {
                huc: 1,
                ram: true,
                battery: true,
                ..Default::default()
            }),
            _ => Err(anyhow!("Unknown cartridge type: {}", value)),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
//noinspection ALL
pub enum NewLicensee {
    None = 0x00,
    NintendoRND1 = 0x01,
    Capcom = 0x08,
    ElectronicArts = 0x13,
    HudsonSoft = 0x18,
    b_ai = 0x19,
    kss = 0x20,
    pow = 0x22,
    PCMComplete = 0x24,
    san_x = 0x25,
    KemcoJapan = 0x28,
    seta = 0x29,
    Viacom = 0x30,
    Nintendo = 0x31,
    Bandai = 0x32,
    Ocean_Acclaim = 0x33,
    Konami = 0x34,
    Hector = 0x35,
    Taito = 0x37,
    Hudson = 0x38,
    Banpresto = 0x39,
    UbiSoft = 0x41,
    Atlus = 0x42,
    Malibu = 0x44,
    angel = 0x46,
    Bullet_Proof = 0x47,
    irem = 0x49,
    Absolute = 0x50,
    Acclaim = 0x51,
    Activision = 0x52,
    Americansammy = 0x53,
    Konami_2 = 0x54,
    Hitechentertainment = 0x55,
    LJN = 0x56,
    Matchbox = 0x57,
    Mattel = 0x58,
    MiltonBradley = 0x59,
    Titus = 0x60,
    Virgin = 0x61,
    LucasArts = 0x64,
    Ocean = 0x67,
    ElectronicArts_2 = 0x69,
    Infogrames = 0x70,
    Interplay = 0x71,
    Broderbund = 0x72,
    sculptured = 0x73,
    sci = 0x75,
    THQ = 0x78,
    Accolade = 0x79,
    misawa = 0x80,
    lozc = 0x83,
    TokumaShotenIntermedia = 0x86,
    TsukudaOriginal = 0x87,
    Chunsoft = 0x91,
    Videosystem = 0x92,
    Ocean_Acclaim_2 = 0x93,
    Varie = 0x95,
    Yonezawas_pal = 0x96,
    Kaneko = 0x97,
    Packinsoft = 0x99,
    Konami_Yu_Gi_Oh = 0xA4,
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::*;

    pub const NINTENDO_LOGO_BYTES: [u8; 48] = [
        0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00,
        0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD,
        0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB,
        0xB9, 0x33, 0x3E,
    ];

    pub fn validate_nintendo_logo(bytes: &[u8]) -> anyhow::Result<()> {
        if bytes.eq(&NINTENDO_LOGO_BYTES) {
            Ok(())
        } else {
            Err(anyhow!("Invalid Nintendo Logo"))
        }
    }

    pub fn validate_ram_size_vs_cartridge_type(cart: &Cartridge) -> anyhow::Result<()> {
        unimplemented!()
    }

    #[test]
    pub fn test_nintendo_logo() {
        todo!()
    }
}
