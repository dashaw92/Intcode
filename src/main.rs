use intcode::rom::Rom;
use intcode::cpu::v2::V2Cpu;
use intcode::computer::Computer;

use std::io::stdin;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("Unable to read from STDIN!");
    
    let rom = Rom::from_string(buf);

    let mut computer = Computer::build(&V2Cpu, rom);

    computer.run();
    let rom = computer.rom();
    println!("\nExecution Halted\nRom = {:?}", rom); 
}
