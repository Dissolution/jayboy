use crate::cpu_flags::CpuFlags;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

#[derive(Copy, Clone)]
pub struct Instruction {
    pub byte: u8,
    pub byte_count: usize,
    pub clock_cycles: usize,
    pub flags: CpuFlags,
    pub mnemonic: &'static str,
    pub description: &'static str,
}
impl Instruction {
    pub const NONE: Instruction = Instruction {
        byte: 0x00,
        byte_count: 0,
        clock_cycles: 0,
        flags: CpuFlags::NONE,
        mnemonic: "NULL",
        description: "NULL",
    };

    /// `Pan Docs`:
    /// Because all Game Boy timings are divisible by 4,
    /// many people specify timings and clock frequency divided by 4,
    /// called “M-cycles”.
    pub fn m_cycles(&self) -> usize {
        self.clock_cycles / 4
    }
}
impl Default for Instruction {
    fn default() -> Self {
        Instruction::NONE
    }
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.mnemonic)
    }
}
impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{} - {} - {} - {:?}",
            self.mnemonic, self.byte_count, self.clock_cycles, self.flags
        )
    }
}
impl PartialEq for Instruction {
    fn eq(&self, other: &Self) -> bool {
        self.byte == other.byte
    }
}
impl Eq for Instruction {}
