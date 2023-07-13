#![allow(unused_imports)]

use crate::{GByte, GBytes};
use anyhow::{anyhow, Result};
use std::ops::{Bound, RangeBounds};

pub trait ReadOnlyMemory {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn get_byte(&self, index: u16) -> Result<u8>;
    fn get_bytes<R: RangeBounds<u16>>(&self, range: R) -> Result<&[u8]>;
}

pub trait ReadWriteMemory: ReadOnlyMemory {
    fn set_byte(&mut self, index: u16, byte: u8) -> Result<()>;
    fn set_bytes<R: RangeBounds<u16>>(&mut self, range: R, bytes: &[u8]) -> Result<()>;
}

#[derive(Debug, Eq, PartialEq, Default)]
pub struct VecMemory(Vec<u8>);
impl VecMemory {
    pub fn new(vec: Vec<u8>) -> Self {
        VecMemory(vec)
    }
}
impl ReadOnlyMemory for VecMemory {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn get_byte(&self, index: u16) -> Result<u8> {
        let i = index as usize;
        if let Some(byte) = self.0.get(i) {
            Ok(*byte)
        } else {
            Err(anyhow!("Invalid Index: {:X}", index))
        }
    }

    fn get_bytes<R: RangeBounds<u16>>(&self, range: R) -> Result<&[u8]> {
        let len = self.0.len();

        let inclusive_start = match range.start_bound() {
            Bound::Included(v) => *v as usize,
            Bound::Excluded(v) => (*v as usize) + 1,
            Bound::Unbounded => 0,
        };
        if inclusive_start >= len {
            return Err(anyhow!("Invalid Range Start"));
        }
        let exclusive_end = match range.end_bound() {
            Bound::Included(v) => (*v as usize) + 1,
            Bound::Excluded(v) => *v as usize,
            Bound::Unbounded => len,
        };
        if exclusive_end > len {
            return Err(anyhow!("Invalid Range End"));
        }
        Ok(&self.0[inclusive_start..exclusive_end])
    }
}
impl ReadWriteMemory for VecMemory {
    fn set_byte(&mut self, index: u16, byte: u8) -> Result<()> {
        let i = index as usize;
        if i >= self.0.len() {
            return Err(anyhow!("Invalid Index: {}", index));
        }
        self.0[i] = byte;
        Ok(())
    }

    fn set_bytes<R: RangeBounds<u16>>(&mut self, range: R, bytes: &[u8]) -> Result<()> {
        let len = self.0.len();

        let inclusive_start = match range.start_bound() {
            Bound::Included(v) => *v as usize,
            Bound::Excluded(v) => (*v as usize) + 1,
            Bound::Unbounded => 0,
        };
        if inclusive_start >= len {
            return Err(anyhow!("Invalid Range Start"));
        }
        let exclusive_end = match range.end_bound() {
            Bound::Included(v) => (*v as usize) + 1,
            Bound::Excluded(v) => *v as usize,
            Bound::Unbounded => len,
        };
        if exclusive_end > len {
            return Err(anyhow!("Invalid Range End"));
        }
        if exclusive_end - inclusive_start != bytes.len() {
            return Err(anyhow!("Invalid Range Length"));
        }
        self.0[inclusive_start..exclusive_end].copy_from_slice(bytes);
        Ok(())
    }
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
    pub fn is_empty(&self) -> bool {
        self.memory_area.is_empty()
    }

    pub fn get_bit(&self, bit_index: u16) -> bool {
        let byte_offset = bit_index % 8;
        let bit_flag = 1 << (bit_index / 8);
        ((self.memory_area[byte_offset as usize]) & bit_flag) != 0
    }
    // pub fn get_byte(&self, byte_index: u8) -> u8 {}
    // pub fn get_ushort(&self, ushort_index: u16) -> u16;
    //
    // pub fn set_bit(&mut self, index: u16, bit: bool);
    // pub fn set_u8(&mut self, index: u16, byte: u8);
    // pub fn set_u16(&mut self, index: u16, ushort: u16);
}
//
// impl Memory {
//     pub fn new() -> Self {
//         Self {
//             bytes: Box::new([0u8; 65_536]),
//         }
//     }
//
//     pub fn set_boot_rom(&mut self, rom: &[u8]) {
//         assert_eq!(rom.len(), 256, "Boot Rom must be exactly 256 bytes");
//         self.bytes[0..256].copy_from_slice(rom)
//     }
//
//     pub fn get_boot_rom(&self) -> &[u8] {
//         self.bytes[0..256].as_ref()
//     }
// }
