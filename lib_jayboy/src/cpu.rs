use crate::{Registers, Timer, VecMemory};

#[derive(Default, Debug)]
pub struct CPU {
    pub registers: Registers,
    pub memory: VecMemory,
    pub timer: Timer,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: Registers::default(),
            memory: VecMemory::default(),
            timer: Timer::default(),
        }
    }
}
