use crate::cpu::{Cpu, Execution};
use crate::computer::State;
use crate::rom::Rom;

pub struct V1Cpu;

impl Cpu for V1Cpu {
    fn exec(&self, state: &mut State, rom: &mut Rom) -> Execution {
        let current = match rom[state.ip] {
            1 => Op::Add,
            2 => Op::Mul,
            _ => return Execution::Halted,
        };

        let src_a = rom[state.ip + 1];
        let src_b = rom[state.ip + 2];
        let dst = rom[state.ip + 3];
        let (op_a, op_b) = (rom[src_a as usize], rom[src_b as usize]);
        #[cfg(debug_assertions)]
        dbg!(state.ip, &rom, src_a, src_b, dst, op_a, op_b);
        rom[dst as usize] = current.apply(op_a, op_b);

        state.inc(current.len());
        Execution::Normal
    }
}

enum Op {
    Add,
    Mul,
    Halt,
}

impl Op {
    fn apply(&self, op_a: i32, op_b: i32) -> i32 {
        match self {
            Op::Halt => unreachable!(),
            Op::Add => op_a + op_b,
            Op::Mul => op_a * op_b,
        }
    }

    fn len(&self) -> usize {
        match self {
            Op::Halt => 1,
            Op::Mul | Op::Add => 4,
        }
    }
}
