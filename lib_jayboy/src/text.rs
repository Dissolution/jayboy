use anyhow::anyhow;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

#[derive(Eq, PartialEq)]
pub struct GBText<'a> {
    bytes: &'a [u8],
}

impl<'a> GBText<'a> {
    pub fn try_from_ascii<'b: 'a>(value: &'b [u8]) -> anyhow::Result<Self> {
        // todo: verify this is true
        let bad = value.iter().any(|v| *v > 127);
        if bad {
            Err(anyhow!("Contains invalid ASCII characters"))
        } else {
            let text = GBText { bytes: value };
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
