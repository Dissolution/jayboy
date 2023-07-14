#![allow(non_camel_case_types)] // I want to!

use crate::errors::JayBoyError;
use anyhow::anyhow;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult, Write};

/// [Code Page 437](https://en.wikipedia.org/wiki/Code_page_437)
#[derive(Debug, Eq, PartialEq)]
pub struct gb_str {
    bytes: Box<[u8]>,
}
impl gb_str {
    pub fn empty() -> Self {
        gb_str {
            bytes: Box::new([]),
        }
    }

    pub fn try_from_ascii(bytes: &[u8]) -> Result<Self, JayBoyError> {
        // verify every byte is valid ASCII
        if bytes.iter().any(|b| !b.is_ascii()) {
            Err(JayBoyError::Misc(anyhow!("Invalid ASCII bytes")))
        } else {
            Ok(bytes.into())
        }
    }
    pub fn try_from_uppercase_ascii(bytes: &[u8]) -> Result<Self, JayBoyError> {
        // verify every byte is valid uppercase ASCII
        if bytes.iter().any(|b| !b.is_ascii_uppercase()) {
            Err(JayBoyError::Misc(anyhow!("Invalid uppercase ASCII bytes")))
        } else {
            Ok(bytes.into())
        }
    }
}

impl From<&[u8]> for gb_str {
    fn from(value: &[u8]) -> Self {
        Self {
            bytes: value.into(),
        }
    }
}

impl Display for gb_str {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // process each byte
        for byte in self.bytes.iter() {
            f.write_char(get_cp437_char(*byte))?;
        }
        Ok(())
    }
}

fn get_cp437_char(byte: u8) -> char {
    CODE_PAGE_437[byte as usize]
}

const CODE_PAGE_437: [char; 256] = [
    '\0', '☺', '☻', '♥', '♦', '♣', '♠', '•', '◘', '○', '◙', '♂', '♀', '♪', '♫', '☼', //
    '►', '◄', '↕', '‼', '¶', '§', '▬', '↨', '↑', '↓', '→', '←', '∟', '↔', '▲', '▼', //
    ' ', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', //
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ':', ';', '<', '=', '>', '?', //
    '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', //
    'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '[', '\\', ']', '^', '_', //
    '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', //
    'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '{', '|', '}', '~', '⌂', //
    'Ç', 'ü', 'é', 'â', 'ä', 'à', 'å', 'ç', 'ê', 'ë', 'è', 'ï', 'î', 'ì', 'Ä', 'Å', //
    'É', 'æ', 'Æ', 'ô', 'ö', 'ò', 'û', 'ù', 'ÿ', 'Ö', 'Ü', '¢', '£', '¥', '₧', 'ƒ', //
    'á', 'í', 'ó', 'ú', 'ñ', 'Ñ', 'ª', 'º', '¿', '⌐', '¬', '½', '¼', '¡', '«', '»', //
    '░', '▒', '▓', '│', '┤', '╡', '╢', '╖', '╕', '╣', '║', '╗', '╝', '╜', '╛', '┐', //
    '└', '┴', '┬', '├', '─', '┼', '╞', '╟', '╚', '╔', '╩', '╦', '╠', '═', '╬', '╧', //
    '╨', '╤', '╥', '╙', '╘', '╒', '╓', '╫', '╪', '┘', '┌', '█', '▄', '▌', '▐', '▀', //
    'α', 'ß', 'Γ', 'π', 'Σ', 'σ', 'µ', 'τ', 'Φ', 'Θ', 'Ω', 'δ', '∞', 'φ', 'ε', '∩', //
    '≡', '±', '≥', '≤', '⌠', '⌡', '÷', '≈', '°', '∙', '·', '√', 'ⁿ', '²', '■', ' ',
];
