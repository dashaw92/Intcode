use crate::cpu::Cpu;
use crate::computer::State;
use crate::rom::Rom;

pub struct V1Cpu;

impl Cpu for V1Cpu {
    fn exec(&mut self, state: &mut State, rom: &mut Rom) {

    }
}
