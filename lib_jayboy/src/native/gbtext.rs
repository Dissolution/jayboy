use anyhow::{anyhow, Result};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

#[derive(Eq, PartialEq)]
pub struct GBText<'a> {
    bytes: &'a [u8],
}

impl<'a> GBText<'a> {
    pub fn from_ascii<'b: 'a>(ascii_bytes: &'b [u8]) -> Result<Self> {
        // todo: verify this is true

        // find all bad characters
        let bad: Vec<char> = ascii_bytes
            .iter()
            .filter(|i| i.is_ascii())
            .map(|i| char::from(*i))
            .collect();
        if !bad.is_empty() {
            Err(anyhow!("Contains invalid ASCII characters: {:?}", bad))
        } else {
            let text = GBText { bytes: ascii_bytes };
            Ok(text)
        }
    }

    pub fn from_uppercase_ascii<'b: 'a>(ascii_bytes: &'b [u8]) -> Result<Self> {
        // todo: verify this is true

        // find all bad characters
        let bad: Vec<char> = ascii_bytes
            .iter()
            .filter(|i| i.is_ascii_uppercase())
            .map(|i| char::from(*i))
            .collect();
        if !bad.is_empty() {
            Err(anyhow!("Contains invalid ASCII characters: {:?}", bad))
        } else {
            let text = GBText { bytes: ascii_bytes };
            Ok(text)
        }
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }
}

impl Display for GBText<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        for byte in self.bytes.iter() {
            write!(f, "{}", char::from(*byte))?;
        }
        Ok(())
    }
}

impl Debug for GBText<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(self, f)
    }
}
