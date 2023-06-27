use crate::CPU;

pub struct OpCode {
    pub value: u8,
    pub name: &'static str,
    pub length: u8,
    pub time: u8,
}
impl OpCode {
    pub fn new(value: u8, name: &'static str, length: u8, time: u8) -> Self {
        Self {
            value,
            name,
            length,
            time,
        }
    }

    pub fn execute(&self, _cpu: &mut CPU) {
        unimplemented!()
    }
}
