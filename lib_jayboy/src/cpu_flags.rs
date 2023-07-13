#[allow(unused_imports)] // This _is_ used
use bitflags::{bitflags, Flags};

bitflags! {
    /// CPU Flags
    /// _note: lower 4 bits are unused_
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CpuFlags: u8 {
        /// zero
        const Z = 0b10000000;
        /// subtraction
        const S = 0b01000000;
        /// half carry
        const H = 0b00100000;
        /// carry
        const C = 0b00010000;
    }
}
impl CpuFlags {
    pub const NONE: CpuFlags = CpuFlags::from_bits_truncate(0);
}
impl From<u8> for CpuFlags {
    fn from(value: u8) -> Self {
        CpuFlags::from_bits_truncate(value)
    }
}
impl From<CpuFlags> for u8 {
    fn from(value: CpuFlags) -> Self {
        value.bits()
    }
}
