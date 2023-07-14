#![allow(non_camel_case_types)] // I want to!

use std::fmt::{Binary, Debug, Display, Formatter, LowerHex, Result as FmtResult, UpperHex};

#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct gb_u16(u16);
impl gb_u16 {
    pub fn from_le_bytes(low: u8, high: u8) -> Self {
        gb_u16(u16::from_le_bytes([low, high]))
    }
    pub fn to_le_bytes(&self) -> [u8;2] {
        u16::to_le_bytes(self.0)
    }
    
    pub fn low_byte(&self) -> u8 {
        u16::to_le_bytes(self.0)[0]
    }
    pub fn high_byte(&self) -> u8 {
        u16::to_le_bytes(self.0)[1]
    }
}

impl From<u16> for gb_u16 {
    fn from(value: u16) -> Self {
        gb_u16(value)
    }
}
impl From<gb_u16> for u16 {
    fn from(value: gb_u16) -> Self {
        value.0
    }
}

impl Debug for gb_u16 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "0x{:0>4X} - {}", self.0, self.0)
    }
}
impl Display for gb_u16 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        UpperHex::fmt(self, f)
    }
}
impl LowerHex for gb_u16 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "0x{:0>4x}", self.0)
    }
}
impl UpperHex for gb_u16 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "0x{:0>4X}", self.0)
    }
}
impl Binary for gb_u16 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "0b{:0>16b}", self.0)
    }
}
