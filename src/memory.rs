use crate::prelude::*;
use std::ops::{Range, RangeBounds, RangeInclusive};

#[derive(Debug, Eq, PartialEq)]
pub struct Memory {
    bytes: Box<[u8]>,
}

pub struct MemoryArea<'m> {
    memory_area: &'m [u8],
}
impl<'m> MemoryArea<'m> {
    pub fn new(slice: &'m [u8]) -> Self {
        Self { memory_area: slice }
    }
    pub fn len(&self) -> usize {
        self.memory_area.len()
    }

    pub fn get_bit(&self, bit_index: u16) -> bool {
        let byte_offset = bit_index % 8;
        let bit_flag = (1 << (bit_index / 8));
        ((self.memory_area[byte_offset as usize]) & bit_flag) != 0
    }
    // pub fn get_byte(&self, byte_index: u8) -> u8 {}
    // pub fn get_ushort(&self, ushort_index: u16) -> u16;
    //
    // pub fn set_bit(&mut self, index: u16, bit: bool);
    // pub fn set_u8(&mut self, index: u16, byte: u8);
    // pub fn set_u16(&mut self, index: u16, ushort: u16);
}

impl Memory {
    pub fn new() -> Self {
        Self {
            bytes: Box::new([0u8; 65_536]),
        }
    }

    pub fn set_boot_rom(&mut self, rom: &[u8]) {
        assert_eq!(rom.len(), 256, "Boot Rom must be exactly 256 bytes");
        self.bytes[0..256].copy_from_slice(rom)
    }

    pub fn get_boot_rom(&self) -> &[u8] {
        self.bytes[0..256].as_ref()
    }
}
