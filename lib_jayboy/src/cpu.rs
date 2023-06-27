use crate::{Memory, Registers, Timer};

#[derive(Debug)]
pub struct CPU {
    pub registers: Registers,
    pub memory: Memory,
    pub timer: Timer,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: Registers::default(),
            memory: Memory::new(),
            timer: Timer::default(),
        }
    }
}
