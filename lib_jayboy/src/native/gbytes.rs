use crate::formatting::FormatterBuilder;
use crate::GByte;
use std::fmt::{Debug, Display, Formatter, LowerHex, Result as FmtResult, UpperHex, Write};

/// A thin wrapper around `&[u8]` for better debugging support
#[derive(Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct GBytes<'a>(pub &'a [u8]);
impl GBytes<'_> {
    // pub fn as_instructions(&self) -> Vec<Instruction> {
    //     InstructionReader::as_instructions(self.0)
    // }
}
impl<'a> Debug for GBytes<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.append('[')
            .delimit(
                |f| f.write_char(',').unwrap(),
                self.0.iter(),
                |f, _, byte| {
                    f.debug(&GByte::from(*byte));
                },
            )
            .append(']');
        Ok(())
    }
}
impl<'a> Display for GBytes<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.append('[')
            .delimit(
                |f| f.write_char(',').unwrap(),
                self.0.iter(),
                |f, _, byte| {
                    f.display(&GByte::from(*byte));
                },
            )
            .append(']');
        Ok(())
    }
}
impl<'a> LowerHex for GBytes<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.append('[')
            .delimit(
                |f| f.write_char(',').unwrap(),
                self.0.iter(),
                |f, _, byte| {
                    f.lower_hex(&GByte::from(*byte));
                },
            )
            .append(']');
        Ok(())
    }
}
impl<'a> UpperHex for GBytes<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.append('[')
            .delimit(
                |f| f.write_char(',').unwrap(),
                self.0.iter(),
                |f, _, byte| {
                    f.upper_hex(&GByte::from(*byte));
                },
            )
            .append(']');
        Ok(())
    }
}
impl<'a> From<&'a [u8]> for GBytes<'a> {
    fn from(value: &'a [u8]) -> Self {
        Self(value)
    }
}
impl<'a> From<GBytes<'a>> for &'a [u8] {
    fn from(value: GBytes<'a>) -> Self {
        value.0
    }
}
