use crate::rom::Rom;
use crate::cpu::Cpu;

use std::io::{Read, Write};

pub struct Computer<'cpu, 'i, 'o> {
    cpu: &'cpu dyn Cpu,
    state: State,
    rom: Rom,
    stdin: &'i mut dyn Read,
    stdin_buf: Vec<u8>,
    stdout: &'o mut dyn Write,
}

pub struct State {
    ip: usize,
    steps: usize,
}

impl<'cpu, 'i, 'o> Computer<'cpu, 'i, 'o> {
    pub fn build(cpu: &'cpu impl Cpu, rom: Rom, stdin: &'i mut impl Read, stdout: &'o mut impl Write) -> Self {
        Self {
            cpu,
            state: State::new(),
            rom,
            stdin,
            stdin_buf: Vec::with_capacity(32),
            stdout
        }
    }

    pub fn read(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut buf = [0; 1];
        self.stdin.read_exact(&mut buf)?;
        self.stdin_buf.extend_from_slice(&buf);
        Ok(())
    }

    pub fn read_line(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            self.read()?;
            match self.stdin_buf.last() {
                Some(&ch) if ch == '\n' as u8 => return Ok(()),
                _ => {},
            }
        }
    }

    pub fn print_buf(&mut self) {
        let mut new_buf = Vec::new();
        std::mem::swap(&mut self.stdin_buf, &mut new_buf);
        let out = String::from_utf8(new_buf).unwrap();
        self.print(out);
    }

    pub fn print(&mut self, out: impl AsRef<str>) {
        writeln!(self.stdout, "{}", out.as_ref());
    }
}

impl State {
    fn new() -> Self {
        Self {
            ip: 0,
            steps: 0,
        }
    }
}
