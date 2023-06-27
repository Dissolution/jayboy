#[derive(Debug, Default, Eq, PartialEq)]
pub struct Registers {
    /// Accumulator
    pub a: u8,
    /// Flags
    pub f: CpuFlags,
    /// B / BC
    pub b: u8,
    /// C / BC
    pub c: u8,
    /// D / DE
    pub d: u8,
    /// E / DE
    pub e: u8,
    /// H / High / HL
    pub h: u8,
    /// L / Low / HL
    pub l: u8,

    /// Stack Pointer
    pub sp: u16,

    /// Program Counter
    /// This tells the CPU where to read the next instruction from Memory
    pub pc: u16,
}

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

impl Registers {
    /// New, empty `Registers`
    pub fn pre_boot() -> Self {
        Registers {
            a: 0x00,
            f: CpuFlags::empty(),
            b: 0x00,
            c: 0x00,
            d: 0x00,
            e: 0x00,
            h: 0x00,
            l: 0x00,
            pc: 0x0000,
            sp: 0x0000,
        }
    }
    /// New `Registers` setup in the state they would be after the Boot Rom has executed
    pub fn post_boot() -> Self {
        Registers {
            a: 0x01,
            f: 0xB0.into(), // 0x10110000 = Z|H|C
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            pc: 0x0100,
            sp: 0xFFFE,
        }
    }

    /// The `AF` registers together represent a `u16` value
    pub fn get_af(&self) -> u16 {
        // The lower 4 bits of `F` must always be zero
        ((self.a as u16) << 8) | (self.f.bits() as u16)
    }
    /// The `AF` registers together represent a `u16` value
    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        // The lower 4 bits of `F` must always be zero
        self.f = ((value & 0x00F0) as u8).into();
    }

    pub fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }
    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    }

    pub fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }
    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0x00FF) as u8;
    }

    pub fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }
    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0x00FF) as u8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_register_conv() {
        let mut reg = Registers::pre_boot();
        reg.a = 0xDE;
        reg.f = 0xAD.into();
        reg.b = 0xBE;
        reg.c = 0xEF;
        reg.d = 0xCA;
        reg.e = 0xFE;
        reg.h = 0xBA;
        reg.l = 0xBE;

        // lower 4 bits of f must always be 0
        assert_eq!(reg.f, 0xA0.into());
        assert_eq!(reg.get_af(), 0xDEA0);
        assert_eq!(reg.get_bc(), 0xBEEF);
        assert_eq!(reg.get_de(), 0xCAFE);
        assert_eq!(reg.get_hl(), 0xBABE);

        reg.set_af(0x1111);
        reg.set_bc(0x1111);
        reg.set_de(0x1111);
        reg.set_hl(0x1111);
        assert_eq!(reg.a, 0x11);
        assert_eq!(reg.b, 0x11);
        assert_eq!(reg.c, 0x11);
        assert_eq!(reg.d, 0x11);
        assert_eq!(reg.e, 0x11);
        assert_eq!(reg.h, 0x11);
        assert_eq!(reg.l, 0x11);
        // lower 4 bits of f must always be 0
        assert_eq!(reg.f, 0x10.into());
        assert_eq!(reg.get_af(), 0x1110);
        assert_eq!(reg.get_bc(), 0x1111);
        assert_eq!(reg.get_de(), 0x1111);
        assert_eq!(reg.get_hl(), 0x1111);
    }
}
