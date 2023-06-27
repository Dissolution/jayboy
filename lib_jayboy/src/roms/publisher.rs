use anyhow::anyhow;

#[derive(Debug, Eq, PartialEq)]
pub struct Publisher {
    pub old_code: u8,
    pub new_code: [u8; 2],
    pub name: &'static str,
}

impl Publisher {
    pub const NONE: Publisher = Publisher {
        old_code: 0x00,
        new_code: [b'0', b'0'],
        name: "None",
    };

    const fn create_old(old_code: u8, name: &'static str) -> Self {
        Publisher {
            old_code,
            new_code: [0x00, 0x00],
            name,
        }
    }
    const fn create_new(new_code_left: u8, new_code_right: u8, name: &'static str) -> Self {
        Publisher {
            old_code: 0x33,
            new_code: [new_code_left, new_code_right],
            name,
        }
    }
}

impl Default for Publisher {
    fn default() -> Self {
        Publisher::NONE
    }
}

impl TryFrom<u8> for Publisher {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value == 0x00 {
            return Ok(Publisher::NONE);
        }
        let name = match value {
            //0x00 => "None",
            0x01 => "Nintendo",
            0x08 => "Capcom",
            0x09 => "Hot-B",
            0x0A => "Jaleco",
            0x0B => "Coconuts Japan",
            0x0C => "Elite Systems",
            0x13 => "EA (Electronic Arts)",
            0x18 => "Hudsonsoft",
            0x19 => "ITC Entertainment",
            0x1A => "Yanoman",
            0x1D => "Japan Clary",
            0x1F => "Virgin Interactive",
            0x24 => "PCM Complete",
            0x25 => "San-X",
            0x28 => "Kotobuki Systems",
            0x29 => "Seta",
            0x30 => "Infogrames",
            0x31 => "Nintendo",
            0x32 => "Bandai",
            0x33 => {
                return Err(anyhow!("This is a New Licensee"));
            }
            0x34 => "Konami",
            0x35 => "HectorSoft",
            0x38 => "Capcom",
            0x39 => "Banpresto",
            0x3C => ".Entertainment i",
            0x3E => "Gremlin",
            0x41 => "Ubisoft",
            0x42 => "Atlus",
            0x44 => "Malibu",
            0x46 => "Angel",
            0x47 => "Spectrum Holoby",
            0x49 => "Irem",
            0x4A => "Virgin Interactive",
            0x4D => "Malibu",
            0x4F => "U.S. Gold",
            0x50 => "Absolute",
            0x51 => "Acclaim",
            0x52 => "Activision",
            0x53 => "American Sammy",
            0x54 => "GameTek",
            0x55 => "Park Place",
            0x56 => "LJN",
            0x57 => "Matchbox",
            0x59 => "Milton Bradley",
            0x5A => "Mindscape",
            0x5B => "Romstar",
            0x5C => "Naxat Soft",
            0x5D => "Tradewest",
            0x60 => "Titus",
            0x61 => "Virgin Interactive",
            0x67 => "Ocean Interactive",
            0x69 => "EA (Electronic Arts)",
            0x6E => "Elite Systems",
            0x6F => "Electro Brain",
            0x70 => "Infogrames",
            0x71 => "Interplay",
            0x72 => "Broderbund",
            0x73 => "Sculptered Soft",
            0x75 => "The Sales Curve",
            0x78 => "t.hq",
            0x79 => "Accolade",
            0x7A => "Triffix Entertainment",
            0x7C => "Microprose",
            0x7F => "Kemco",
            0x80 => "Misawa Entertainment",
            0x83 => "Lozc",
            0x86 => "Tokuma Shoten Intermedia",
            0x8B => "Bullet-Proof Software",
            0x8C => "Vic Tokai",
            0x8E => "Ape",
            0x8F => "I’Max",
            0x91 => "Chunsoft Co.",
            0x92 => "Video System",
            0x93 => "Tsubaraya Productions Co.",
            0x95 => "Varie Corporation",
            0x96 => "Yonezawa/S’Pal",
            0x97 => "Kaneko",
            0x99 => "Arc",
            0x9A => "Nihon Bussan",
            0x9B => "Tecmo",
            0x9C => "Imagineer",
            0x9D => "Banpresto",
            0x9F => "Nova",
            0xA1 => "Hori Electric",
            0xA2 => "Bandai",
            0xA4 => "Konami",
            0xA6 => "Kawada",
            0xA7 => "Takara",
            0xA9 => "Technos Japan",
            0xAA => "Broderbund",
            0xAC => "Toei Animation",
            0xAD => "Toho",
            0xAF => "Namco",
            0xB0 => "acclaim",
            0xB1 => "ASCII or Nexsoft",
            0xB2 => "Bandai",
            0xB4 => "Square Enix",
            0xB6 => "HAL Laboratory",
            0xB7 => "SNK",
            0xB9 => "Pony Canyon",
            0xBA => "Culture Brain",
            0xBB => "Sunsoft",
            0xBD => "Sony Imagesoft",
            0xBF => "Sammy",
            0xC0 => "Taito",
            0xC2 => "Kemco",
            0xC3 => "Squaresoft",
            0xC4 => "Tokuma Shoten Intermedia",
            0xC5 => "Data East",
            0xC6 => "Tonkinhouse",
            0xC8 => "Koei",
            0xC9 => "UFL",
            0xCA => "Ultra",
            0xCB => "Vap",
            0xCC => "Use Corporation",
            0xCD => "Meldac",
            0xCE => ".Pony Canyon or",
            0xCF => "Angel",
            0xD0 => "Taito",
            0xD1 => "Sofel",
            0xD2 => "Quest",
            0xD3 => "Sigma Enterprises",
            0xD4 => "ASK Kodansha Co.",
            0xD6 => "Naxat Soft",
            0xD7 => "Copya System",
            0xD9 => "Banpresto",
            0xDA => "Tomy",
            0xDB => "LJN",
            0xDD => "NCS",
            0xDE => "Human",
            0xDF => "Altron",
            0xE0 => "Jaleco",
            0xE1 => "Towa Chiki",
            0xE2 => "Yutaka",
            0xE3 => "Varie",
            0xE5 => "Epcoh",
            0xE7 => "Athena",
            0xE8 => "Asmik ACE Entertainment",
            0xE9 => "Natsume",
            0xEA => "King Records",
            0xEB => "Atlus",
            0xEC => "Epic/Sony Records",
            0xEE => "IGS",
            0xF0 => "A Wave",
            0xF3 => "Extreme Entertainment",
            0xFF => "LJN",
            _ => {
                return Err(anyhow!("Unknown old licensee"));
            }
        };
        Ok(Publisher::create_old(value, name))
    }
}

impl TryFrom<[u8; 2]> for Publisher {
    type Error = anyhow::Error;

    fn try_from(value: [u8; 2]) -> Result<Self, Self::Error> {
        if value == [0, 0] {
            // DEBUG THIS
            assert_eq!(1, 2);
            return Ok(Publisher::NONE);
        }
        // treat value as two ASCII chars
        let str = std::str::from_utf8(&value)?;
        if str.chars().count() != 2 {
            return Err(anyhow!("Invalid ASCII characters"));
        }
        if str == "00" {
            return Ok(Publisher::NONE);
        }
        let name = match str {
            //"00" => "None",
            "01" => "Nintendo R&D1",
            "08" => "Capcom",
            "13" => "Electronic Arts",
            "18" => "Hudson Soft",
            "19" => "b-ai",
            "20" => "kss",
            "22" => "pow",
            "24" => "PCM Complete",
            "25" => "san-x",
            "28" => "Kemco Japan",
            "29" => "seta",
            "30" => "Viacom",
            "31" => "Nintendo",
            "32" => "Bandai",
            "33" => "Ocean/Acclaim",
            "34" => "Konami",
            "35" => "Hector",
            "37" => "Taito",
            "38" => "Hudson",
            "39" => "Banpresto",
            "41" => "Ubi Soft",
            "42" => "Atlus",
            "44" => "Malibu",
            "46" => "angel",
            "47" => "Bullet-Proof",
            "49" => "irem",
            "50" => "Absolute",
            "51" => "Acclaim",
            "52" => "Activision",
            "53" => "American sammy",
            "54" => "Konami",
            "55" => "Hi tech entertainment",
            "56" => "LJN",
            "57" => "Matchbox",
            "58" => "Mattel",
            "59" => "Milton Bradley",
            "60" => "Titus",
            "61" => "Virgin",
            "64" => "LucasArts",
            "67" => "Ocean",
            "69" => "Electronic Arts",
            "70" => "Infogrames",
            "71" => "Interplay",
            "72" => "Broderbund",
            "73" => "sculptured",
            "75" => "sci",
            "78" => "THQ",
            "79" => "Accolade",
            "80" => "misawa",
            "83" => "lozc",
            "86" => "Tokuma Shoten Intermedia",
            "87" => "Tsukuda Original",
            "91" => "Chunsoft",
            "92" => "Video system",
            "93" => "Ocean/Acclaim",
            "95" => "Varie",
            "96" => "Yonezawa/s’pal",
            "97" => "Kaneko",
            "99" => "Pack in soft",
            "A4" => "Konami (Yu-Gi-Oh!)",
            _ => {
                return Err(anyhow!("Invalid New Licensee chars: {}", str));
            }
        };
        Ok(Publisher::create_new(value[0], value[1], name))
    }
}
