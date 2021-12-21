use std::io::{Read, BufRead};

use crate::prelude::*;

pub const OPCODES: [&dyn Opcode; 5] = [
    &NoOp as &dyn Opcode, //acts as padding
    &Add as &dyn Opcode,
    &Mul as &dyn Opcode,
    &In as &dyn Opcode,
    &Out as &dyn Opcode,
];

fn to_rom_idx(mode: Mode, raw_idx: i32, rom: &Rom) -> i32 {
    match mode {
        Mode::Pos => rom[raw_idx as usize],
        Mode::Imm => raw_idx,
    }
}

fn common_mul_add(state: &mut State, rom: &mut Rom) -> (i32, i32, i32) {
    let modes: Modes  = rom[state.ip].into();
    let op_a = to_rom_idx(modes.first(), rom[state.ip + 1 as usize], &rom);
    let op_b = to_rom_idx(modes.second(), rom[state.ip + 2 as usize], &rom);
    
    //"Parameters that an instruction writes to will never be in immediate mode."
    let dst = rom[state.ip + 3 as usize];

    dbg!(&modes, op_a, op_b, dst);
    (op_a, op_b, dst)
}

struct NoOp;
impl Opcode for NoOp {
    fn len(&self) -> usize { 0 }
    fn apply(&self, state: &mut State, rom: &mut Rom) {}
}

struct Add;
impl Opcode for Add {
    fn len(&self) -> usize { 4 }
    fn apply(&self, state: &mut State, rom: &mut Rom) {
        let (op_a, op_b, dst) = common_mul_add(state, rom);
        rom[dst as usize] = op_a + op_b;
    }
}

struct Mul;
impl Opcode for Mul {
    fn len(&self) -> usize { 4 }
    fn apply(&self, state: &mut State, rom: &mut Rom) {
        let (op_a, op_b, dst) = common_mul_add(state, rom);
        rom[dst as usize] = op_a * op_b;
    }
}

struct In;
impl Opcode for In {
    fn len(&self) -> usize { 2 }
    fn apply(&self, state: &mut State, rom: &mut Rom) {
        let modes: Modes  = rom[state.ip].into();
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf);

        let int = buf.trim().parse().unwrap();
        let idx = to_rom_idx(modes.first(), rom[state.ip + 1 as usize], &rom) as usize;
        rom[idx] = int;
    }
}

struct Out;
impl Opcode for Out {
    fn len(&self) -> usize { 2 }
    fn apply(&self, state: &mut State, rom: &mut Rom) {
        let modes: Modes  = rom[state.ip].into();
        let out = to_rom_idx(modes.first(), rom[state.ip + 1 as usize], &rom);
        print!("{}", out);
    }
}

pub trait Opcode {
    fn len(&self) -> usize;
    fn apply(&self, state: &mut State, rom: &mut Rom);
}

#[derive(Debug)]
struct Modes([Mode; 3]);

impl Modes {
    pub fn first(&self) -> Mode {
        self.0[0]
    }

    pub fn second(&self) -> Mode {
        self.0[1]
    }

    pub fn third(&self) -> Mode {
        self.0[2]
    }
}

impl From<i32> for Modes {
    fn from(x: i32) -> Self {
        let mut modes = [Mode::Pos; 3];

        let x = (x / 100).to_string();
        let mut bits = x.chars().rev();
        
        modes[0] = (bits.next().and_then(|c| c.to_digit(10)).unwrap_or(0) as i32).into();
        modes[1] = (bits.next().and_then(|c| c.to_digit(10)).unwrap_or(0) as i32).into();
        modes[2] = (bits.next().and_then(|c| c.to_digit(10)).unwrap_or(0) as i32).into();

        Self(modes)
    }
}

#[derive(Copy, Clone, Debug)]
enum Mode {
    Pos,
    Imm,
}

impl From<i32> for Mode {
    fn from(x: i32) -> Self {
        match x {
            0 => Mode::Pos,
            _ => Mode::Imm,
        }
    }
}
