use crate::rom::Rom;
use crate::computer::State;

pub mod v1;

pub trait Cpu {
    fn exec(&mut self, state: &mut State, rom: &mut Rom);
}
