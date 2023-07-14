#![allow(non_camel_case_types)] // I want to!

use std::fmt::{Binary, Debug, Display, Formatter, LowerHex, Result as FmtResult, UpperHex};

#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct gb_i8(i8);
impl gb_i8 {}

impl From<i8> for gb_i8 {
    fn from(value: i8) -> Self {
        gb_i8(value)
    }
}
impl From<gb_i8> for i8 {
    fn from(value: gb_i8) -> Self {
        value.0
    }
}

impl Debug for gb_i8 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "0x{:0>2X} - {}", self.0, self.0)
    }
}
impl Display for gb_i8 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        UpperHex::fmt(self, f)
    }
}
impl LowerHex for gb_i8 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "0x{:0>2x}", self.0)
    }
}
impl UpperHex for gb_i8 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "0x{:0>2X}", self.0)
    }
}
impl Binary for gb_i8 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "0b{:0>8b}", self.0)
    }
}
