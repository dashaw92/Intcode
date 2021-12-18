use crate::rom::Rom;
use crate::computer::State;

pub mod v1;

pub trait Cpu {
    fn exec(&self, state: &mut State, rom: &mut Rom) -> Execution;
}

pub enum Execution {
    Normal,
    Halted,
}
