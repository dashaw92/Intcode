use std::io::{Read, BufRead};

use crate::prelude::*;

fn to_rom_idx(mode: Mode, raw_idx: i32, rom: &Rom) -> i32 {
    match mode {
        Mode::Imm => rom[raw_idx as usize],
        Mode::Pos => raw_idx,
    }
}

pub struct Add;
impl Opcode for Add {
    fn len(&self) -> usize { 4 }
    fn apply(&self, state: &mut State, rom: &mut Rom) {
        let modes: Modes  = rom[state.ip].into();
        let op_a = rom[to_rom_idx(modes.0[0], rom[state.ip + 1 as usize], &rom) as usize];
        let op_b = rom[to_rom_idx(modes.0[1], rom[state.ip + 2 as usize], &rom) as usize];
        let dst = rom[to_rom_idx(modes.0[2], rom[state.ip + 3 as usize], &rom) as usize];

        rom[dst as usize] = op_a + op_b;
    }
}

pub struct Mul;
impl Opcode for Mul {
    fn len(&self) -> usize { 4 }
    fn apply(&self, state: &mut State, rom: &mut Rom) {
        let modes: Modes  = rom[state.ip].into();
        dbg!(&modes);
        let op_a = rom[to_rom_idx(modes.first(), rom[state.ip + 1 as usize], &rom) as usize];
        let op_b = rom[to_rom_idx(modes.second(), rom[state.ip + 2 as usize], &rom) as usize];
        let dst = rom[to_rom_idx(modes.third(), rom[state.ip + 3 as usize], &rom) as usize];

        rom[dst as usize] = op_a * op_b;
    }
}

pub struct In;
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

pub struct Out;
impl Opcode for Out {
    fn len(&self) -> usize { 2 }
    fn apply(&self, state: &mut State, rom: &mut Rom) {
        let modes: Modes  = rom[state.ip].into();
        let out = rom[to_rom_idx(modes.first(), rom[state.ip + 1 as usize], &rom) as usize];
        print!("{}", out);
    }
}

pub trait Opcode {
    fn len(&self) -> usize;
    fn apply(&self, state: &mut State, rom: &mut Rom);
}

#[derive(Debug)]
pub struct Modes([Mode; 3]);

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

        for (i, pow) in [(0, 100), (1, 1000), (2, 10000)] {
            if x < pow {
                modes[i] = Mode::Pos;
            } else {
                modes[i] = ((x / pow ) % (pow / 10)).into();
            }
        }

        Self(modes)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Mode {
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
