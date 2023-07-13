use std::fmt::{Debug, Display, Formatter, LowerHex, Result as FmtResult, UpperHex};

/// A thin wrapper around `u8` for better debugging support
#[derive(Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct GByte(u8);
impl Debug for GByte {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        UpperHex::fmt(self, f)
    }
}
impl Display for GByte {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        UpperHex::fmt(self, f)
    }
}
impl LowerHex for GByte {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "0x{:0<2x}", self.0)
    }
}
impl UpperHex for GByte {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "0x{:0<2X}", self.0)
    }
}
impl From<u8> for GByte {
    fn from(value: u8) -> Self {
        GByte(value)
    }
}
impl From<GByte> for u8 {
    fn from(value: GByte) -> Self {
        value.0
    }
}
