use std::fs::read_to_string;

use intcode::rom::Rom;
use intcode::cpu::v1::V1Cpu;
use intcode::computer::Computer;

fn main() {
    /* demo rom
    let rom = Rom::new(&[
        1,9,10,3,2,3,11,0,99,30,40,50
    ]);
    */

    let mut rom = Rom::from_string(read_to_string("day2.in").unwrap());

    rom[1] = 12;
    rom[2] = 2;

    let mut computer = Computer::build(&V1Cpu, rom);

    computer.run();
    let rom = computer.rom();
    println!("{:?}", rom); 
}
