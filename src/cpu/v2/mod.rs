use crate::cpu::{Cpu, Execution};
use crate::computer::State;
use crate::rom::Rom;

mod opcode;
use opcode::*;

pub struct V2Cpu;

impl Cpu for V2Cpu {
    fn exec(&self, state: &mut State, rom: &mut Rom) -> Execution {
        let op = rom[state.ip];
        let decoded = op % 100;
        let op: &dyn Opcode = match decoded {
            1 => &Add as &dyn Opcode,
            2 => &Mul as &dyn Opcode,
            3 => &In as &dyn Opcode,
            4 => &Out as &dyn Opcode,
            _ => return Execution::Halted,
        };

        op.apply(state, rom);
        state.inc(op.len());
        Execution::Normal
    }
}

#[cfg(test)]
mod v2test {
    use super::V2Cpu;
    use crate::prelude::*;
    use std::fs::read_to_string;

    #[test]
    fn day2() {
        let rom = Rom::from_string("3,0,4,0,99");
        let mut computer = Computer::build(&V2Cpu, rom);

        computer.run();
        let rom = computer.rom();
        assert_eq!(1, rom[0]);
    }
}
