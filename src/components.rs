// Components for 32-bit microprocessor
pub const NUM_REGISTERS: usize = 16;

pub struct Register {
    value: u32,
}

impl Register {
    pub fn new() -> Register {
        Register { value: 0 }
    }

    pub fn read(&self) -> u32 {
        self.value
    }

    pub fn write(&mut self, value: u32) {
        self.value = value;
    }
}

pub struct RegisterFile {
    registers: Vec<Register>,
    write_enable: bool,
}

impl RegisterFile {
    pub fn new() -> RegisterFile {
        let mut registers = Vec::new();
        for _ in 0..NUM_REGISTERS {
            registers.push(Register::new());
        }
        RegisterFile {
            registers,
            write_enable: false,
        }
    }

    pub fn read(&self, address: usize) -> u32 {
        self.registers[address].read()
    }

    pub fn write(&mut self, address: usize, value: u32) {
        if self.write_enable {
            self.registers[address].write(value);
        }
    }

    pub fn set_write_enable(&mut self, write_enable: bool) {
        self.write_enable = write_enable;
    }
}

pub struct ALU;

impl ALU {
    pub fn new() -> ALU {
        ALU
    }

    pub fn execute(&self, op: u8, a: u32, b: u32) -> u32 {
        match op {
            0 => a + b,
            1 => a - b,
            2 => a & b,
            3 => a | b,
            4 => a ^ b,
            5 => a << b,
            6 => a >> b,
            7 => {
                if a < b {
                    1
                } else {
                    0
                }
            }
            _ => panic!("Invalid ALU operation"),
        }
    }
}

pub struct Memory {
    memory: Vec<u32>,
    pub size: usize,
    write_enable: bool,
}

impl Memory {
    pub fn new(size: usize) -> Memory {
        let mut memory = Vec::new();
        for _ in 0..size {
            memory.push(0);
        }
        Memory {
            memory,
            size,
            write_enable: false,
        }
    }

    pub fn read(&self, address: usize) -> u32 {
        self.memory[address]
    }

    pub fn write(&mut self, address: usize, value: u32) {
        if self.write_enable {
            self.memory[address] = value;
        }
    }

    pub fn set_write_enable(&mut self, write_enable: bool) {
        self.write_enable = write_enable;
    }
}

pub struct Flags {
    flags: Vec<bool>,
}

impl Flags {
    pub fn new() -> Flags {
        let mut flags = Vec::new();
        for _ in 0..NUM_REGISTERS {
            flags.push(false);
        }
        Flags { flags }
    }

    pub fn set_flag(&mut self, address: usize, flag: bool) {
        self.flags[address] = flag;
    }

    pub fn get_flag(&self, address: usize) -> bool {
        self.flags[address]
    }

    pub fn get_flags(&self) -> &[bool] {
        &self.flags
    }

}
