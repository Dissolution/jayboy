use anyhow::{anyhow, Result};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// **C**olor **G**ame **B**oy **Flag**  
/// This bit specifies the Color Game Boy compatibility mode this cart supports
#[derive(Default, Debug, Eq, PartialEq)]
pub enum CGBFlag {
    /// Pre-GBC Game
    #[default]
    None,
    /// Compatible with Game Boy Color (aware)
    Compat,
    /// Only works on Game Boy Color
    /// According to `Pan Docs`, this is exactly the same as `Compat`
    GCBOnly,
    /// A special, non-CGB-mode
    /// According to `Pan Docs`, little is known about this
    PGBMode,
}
impl CGBFlag {
    /// Does the cart header `byte` indicate a specific `CGBFlag`?
    pub fn not_none(byte: u8) -> bool {
        byte == 0x80 || byte == 0xC0 || byte == 0x88 || byte == 0x84
    }
}

impl Display for CGBFlag {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            CGBFlag::None => FmtResult::Ok(()),
            CGBFlag::Compat | CGBFlag::GCBOnly => {
                write!(f, "CGB-Compat")
            }
            CGBFlag::PGBMode => {
                write!(f, "!PGB Mode!")
            }
        }
    }
}
impl TryFrom<u8> for CGBFlag {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            //
            0x80 => Ok(CGBFlag::Compat),
            // The hardware ignores bit 6, so this is functionally the same as `Compat`
            0xC0 => Ok(CGBFlag::GCBOnly),
            // values with bit 7 and either bit 2 or 3 set will switch the Game Boy into a special non-CGB mode
            0x88 | 0x84 => Ok(CGBFlag::PGBMode),
            _ => Err(anyhow!("Invalid CGBFlag: 0x{:0<2X}", value)),
        }
    }
}
