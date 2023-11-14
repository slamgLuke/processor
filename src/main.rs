// 32-bit microprocessor simulator
#[allow(dead_code)]
mod components;
#[allow(overflowing_literals)]
#[allow(dead_code)]
mod processor;
mod instruction;

use components::*;
use processor::*;

fn main() {
    let debug = false;
    let code_len = 4;
    let mut imem = Memory::new(code_len);
    imem.load_from_file("src/test2.asm");
    let mut cpu = Processor::from_imem(imem);

    for _ in 0..code_len {
        cpu.cycle(debug);
        // print all registers
        for i in 0..8 {
            print!("R{}: {:04x} ", i, cpu.rf.read(i));
        }
        println!();
    }

}
