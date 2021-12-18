use intcode::rom::Rom;
use intcode::cpu::v1::V1Cpu;
use intcode::computer::Computer;

use std::io::{stdin, BufRead};

fn main() {
    let stdin = stdin();
    let mut lock = stdin.lock();

    let mut buf = String::new();
    lock.read_line(&mut buf).expect("Unable to read from STDIN!");
    
    let rom = Rom::from_string(buf);

    let mut computer = Computer::build(&V1Cpu, rom);

    computer.run();
    let rom = computer.rom();
    println!("{:?}", rom); 
}
