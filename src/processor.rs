// 32-bit microprocessor class
use crate::components::*;
use crate::instruction::*;

pub struct Processor {
    pub rf: RegisterFile,
    pub alu: ALU,
    pub imem: Memory,
    pub dmem: Memory,
}

impl Processor {
    pub fn new() -> Processor {
        Processor {
            rf: RegisterFile::new(),
            alu: ALU::new(),
            imem: Memory::new(256),
            dmem: Memory::new(256),
        }
    }

    pub fn from_imem(imem: Memory) -> Processor {
        Processor {
            rf: RegisterFile::new(),
            alu: ALU::new(),
            imem,
            dmem: Memory::new(256),
        }
    }

    fn decode(&self, instr: u16) -> Option<Instruction> {
        // 16 bit format decode
        // see README.md for details
        let cond = instr >> 14;
        if !self.alu.get_flags().condex(cond as u8) {
            None
        } else {
            let mem = (instr >> 14) & 0b1;
            let opcode = (instr >> 10) & 0b111;
            let rn = (instr >> 7) & 0b111;
            let rd = (instr >> 4) & 0b111;
            let i = (instr >> 3) & 0b1;
            let src2 = instr & 0b111;

            let instr = Instruction {
                cond: cond as u8,
                mem: mem as u8,
                opcode: opcode as u8,
                rn: rn as u8,
                rd: rd as u8,
                i: i as u8,
                src2: src2 as u8,
            };

            Some(instr)
        }
    }

    fn process(&mut self, instr: Instruction) {
        // determine if data processing or memory manipulation
        match instr.mem {
            0 => self.data_processing(instr),
            1 => self.memory_manip(instr),
            _ => panic!("Invalid instruction"),
        }
    }

    fn data_processing(&mut self, instr: Instruction) {
        let src_a = self.rf.read(instr.rn as usize);
        let src_b = if instr.i == 0 {
            self.rf.read(instr.src2 as usize)
        } else {
            instr.src2 as u16
        };
        let alu_result = self.alu.execute(instr.opcode, src_a, src_b);
        self.rf.set_write_enable(true);
        self.rf.write(instr.rd as usize, alu_result);
        self.rf.set_write_enable(false);
    }

    fn memory_manip(&mut self, instr: Instruction) {
        let src_a = self.rf.read(instr.rn as usize);
        let src_b = if instr.i == 0 {
            self.rf.read(instr.src2 as usize)
        } else {
            instr.src2 as u16
        };
        let write = (instr.opcode >> 2) & 0b1; // LDR = 0, STR = 1
        let write_back = (instr.opcode >> 1) & 0b1;
        let operation = instr.opcode & 0b1; // add = 0, sub = 1

        let alu_result = self.alu.execute(0b10 | operation, src_a, src_b);
        if write_back == 1 {
            self.rf.set_write_enable(true);
            self.rf.write(instr.rn as usize, alu_result);
            self.rf.set_write_enable(false);
        }
        match write {
            0 => {
                let value = self.dmem.read(alu_result as usize);
                self.rf.set_write_enable(true);
                self.rf.write(instr.rd as usize, value);
                self.rf.set_write_enable(false);
            }
            1 => {
                let value = self.rf.read(instr.rd as usize);
                self.dmem.write(alu_result as usize, value);
            }
            _ => panic!("Invalid memory manipulation instruction"),
        }
    }

    pub fn cycle(&mut self, debug: bool) {
        let pc = self.rf.read(7);
        self.rf.set_write_enable(true);
        self.rf.write(7, pc + 1);
        self.rf.set_write_enable(false);
        let numeric_instr = self.imem.read(pc as usize);
        let instr: Option<Instruction> = self.decode(numeric_instr);
        if let Some(instr) = instr {
            if debug {
                println!("{:#?}", instr);
            }
            self.process(instr);
        }
    }
}
