use crate::{rom::Rom, cpu::Execution};
use crate::cpu::Cpu;

use std::{io::{Read, Write}, ops::AddAssign};

pub struct Computer<'cpu> {
    cpu: &'cpu dyn Cpu,
    state: State,
    rom: Rom,
    stdin_buf: Vec<u8>,
}

#[derive(Debug)]
pub struct State {
    pub ip: usize,
    pub steps: usize,
}

impl<'cpu> Computer<'cpu> {
    pub fn build(cpu: &'cpu impl Cpu, rom: Rom) -> Self {
        Self {
            cpu,
            state: State::new(),
            rom,
            stdin_buf: Vec::with_capacity(32),
        }
    }

    pub fn read(&mut self, stdin: &mut impl Read) -> Result<(), Box<dyn std::error::Error>> {
        let mut buf = [0; 1];
        stdin.read_exact(&mut buf)?;
        self.stdin_buf.extend_from_slice(&buf);
        Ok(())
    }

    pub fn read_line(&mut self, stdin: &mut impl Read) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            self.read(stdin)?;
            match self.stdin_buf.last() {
                Some(&ch) if ch == '\n' as u8 => return Ok(()),
                _ => {},
            }
        }
    }

    pub fn print_buf(&mut self, stdout: &mut impl Write) {
        let mut new_buf = Vec::new();
        std::mem::swap(&mut self.stdin_buf, &mut new_buf);
        let out = String::from_utf8(new_buf).unwrap();
        self.print(out, stdout);
    }

    pub fn print(&mut self, out: impl AsRef<str>, stdout: &mut impl Write) {
        writeln!(stdout, "{}", out.as_ref());
    }

    pub fn run(&mut self) {
        while let Execution::Normal = self.cpu.exec(&mut self.state, &mut self.rom) {}
    }

    pub fn rom(&self) -> &Rom {
        &self.rom
    }

    pub fn reset(&mut self) {
        self.rom.reset();
        self.state.reset();
        self.stdin_buf.clear();
    }
}

impl State {
    fn new() -> Self {
        Self {
            ip: 0,
            steps: 0,
        }
    }

    pub fn inc(&mut self, len: usize) {
        self.ip += len;
        self.steps += 1;
    }

    fn reset(&mut self) {
        self.ip = 0;
        self.steps = 0;
    }
}
