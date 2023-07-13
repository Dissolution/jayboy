use crate::{Instruction, Instructions};

pub struct InstructionReader {}
impl InstructionReader {
    pub fn as_instructions(bytes: &[u8]) -> Vec<Instruction> {
        let mut instructions = Vec::new();
        let mut index = 0;
        while index < bytes.len() {
            let byte = bytes[index];
            let instruction = if Instructions::is_prefix(byte) {
                Instructions::get_standard(byte)
            } else {
                Instructions::get_prefixed(byte)
            }
            .unwrap();
            // consume extra bytes

            instructions.push(instruction);
            index += 1;
        }
        instructions
    }
}
