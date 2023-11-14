// 32-bit microprocessor class
use crate::components::*;

pub struct Processor {
    pub rf: RegisterFile,
    pub alu: ALU,
    pub flags: Flags,
    pub imem: Memory,
    pub dmem: Memory,
    pub pc: u32,
}

impl Processor {
    pub fn new() -> Processor {
        Processor {
            rf: RegisterFile::new(),
            alu: ALU::new(),
            flags: Flags::new(),
            imem: Memory::new(256),
            dmem: Memory::new(256),
            pc: 0,
        }
    }

    pub fn from_imem(imem: Memory) -> Processor {
        Processor {
            rf: RegisterFile::new(),
            alu: ALU::new(),
            flags: Flags::new(),
            imem,
            dmem: Memory::new(256),
            pc: 0,
        }
    }

    fn decode(instr: u32) {
        todo!();
    }

    pub fn cycle(&mut self) {
        self.pc += 1;
        let instr = self.imem.read(self.pc as usize);
        // decode(instr);
    }
}
