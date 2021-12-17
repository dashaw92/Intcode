use intcode::rom::Rom;
use intcode::cpu::v1::V1Cpu;
use intcode::computer::Computer;

//use std::io::Cursor;
use std::io::{stdin, stdout};

fn main() {
    let rom = Rom::new(&[1,2,3]);

    //let mut cursor = Cursor::new(String::from("123\n"));
    let stdin = stdin();
    let mut in_handle = stdin.lock();
    let stdout = stdout();
    let mut out_handle = stdout.lock();

    let mut computer = Computer::build(&V1Cpu, rom, &mut in_handle, &mut out_handle);
    computer.read_line().unwrap();
    computer.print_buf();
}
