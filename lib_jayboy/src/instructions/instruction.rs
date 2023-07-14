use crate::errors::JayBoyError;
use crate::native::*;
use crate::*;
use anyhow::anyhow;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

#[derive(Debug, Eq, PartialEq)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Instruction {
    NOP,
    JPa16(gb_u16),
    LD(Register, Register),
    LDA(gb_u16),
    /// Reset the master enable (IME) flag and prohibit maskable interrupts.
    DI,
    /// Calls a method? at the address
    CALLa16(gb_u16),
    ANDd8(gb_u8),
}
#[allow(unreachable_patterns)]
impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Instruction::NOP => write!(f, "NOP"),
            Instruction::JPa16(v) => write!(f, "JP a16: {}", v),
            Instruction::LD(d, s) => write!(f, "LD: {:?} {:?}", d, s),
            Instruction::DI => write!(f, "DI"),
            _ => write!(f, "???"),
        }
    }
}

impl Instruction {
    pub fn is_prefix(byte: u8) -> bool {
        byte == 0xCB
    }

    pub fn byte_size(&self) -> usize {
        match self {
            _ => 1,
        }
    }

    pub fn cycle_count(&self) -> usize {
        match self {
            _ => 1,
        }
    }

    pub fn cpu_flags(&self) -> CpuFlags {
        match self {
            _ => CpuFlags::NONE,
        }
    }
}

fn read_gb_u8(bytes: &[u8], index: &mut usize) -> Result<gb_u8, JayBoyError> {
    let i = *index;
    if i + 1 > bytes.len() {
        Err(JayBoyError::Misc(anyhow!("Not enough bytes")))
    } else {
        *index = i + 1;
        Ok(gb_u8::from(bytes[i]))
    }
}

fn read_gb_u16(bytes: &[u8], index: &mut usize) -> Result<gb_u16, JayBoyError> {
    let i = *index;
    if i + 2 > bytes.len() {
        Err(JayBoyError::Misc(anyhow!("Not enough bytes")))
    } else {
        *index = i + 2;
        Ok(gb_u16::from_le_bytes(bytes[i], bytes[i + 1]))
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct InstructionPosition(pub gb_u16, pub Instruction);
impl Display for InstructionPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:X}: {}", self.0, self.1)
    }
}

/// [OpCodes](https://meganesu.github.io/generate-gb-opcodes/)
pub struct InstructionReader;
impl InstructionReader {
    pub fn try_parse(bytes: &[u8]) -> Result<Vec<InstructionPosition>, JayBoyError> {
        assert!(bytes.len() <= u16::MAX as usize);

        let mut instruction_positions = Vec::new();
        // the index in `bytes`
        let mut index = 0;

        while index < bytes.len() {
            let offset = index;
            let byte = bytes[index];
            index += 1;
            let instruction = match byte {
                0x00 => Instruction::NOP,
                0x7F => Instruction::LD(Register::A, Register::A),
                0xC3 => {
                    let address = read_gb_u16(bytes, &mut index)?;
                    Instruction::JPa16(address)
                }
                0xCD => {
                    let address = read_gb_u16(bytes, &mut index)?;
                    Instruction::CALLa16(address)
                }
                0xE6 => {
                    let byte = read_gb_u8(bytes, &mut index)?;
                    Instruction::ANDd8(byte)
                }
                0xF3 => Instruction::DI,
                0xF0 => {
                    let address = read_gb_u8(bytes, &mut index)?;
                    let address = gb_u16::from_le_bytes(address.into(), 0xFF);
                    Instruction::LDA(address)
                }
                _ => {
                    println!("Unknown Byte: {}", gb_u8::from(byte));
                    return Err(JayBoyError::Misc(anyhow!(
                        "Unknown Instruction Byte: {}",
                        gb_u8::from(byte)
                    )));
                }
            };

            println!("{}: Instruction: {:?}", index, &instruction);

            instruction_positions.push(InstructionPosition((offset as u16).into(), instruction));
        }

        Ok(instruction_positions)
    }
}
