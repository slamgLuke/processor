// Components for 16-bit microprocessor
use std::io::Read;
pub const NUM_REGISTERS: usize = 8;

pub struct Register {
    value: u16,
}

impl Register {
    pub fn new() -> Register {
        Register { value: 0 }
    }

    pub fn read(&self) -> u16 {
        self.value
    }

    pub fn write(&mut self, value: u16) {
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

    pub fn read(&self, address: usize) -> u16 {
        self.registers[address].read()
    }

    pub fn write(&mut self, address: usize, value: u16) {
        if self.write_enable {
            self.registers[address].write(value);
        }
    }

    pub fn set_write_enable(&mut self, write_enable: bool) {
        self.write_enable = write_enable;
    }
}

pub struct ALU {
    flags: Flags,
}

impl ALU {
    pub fn new() -> ALU {
        ALU { flags: Flags::new() }
    }

    pub fn execute(&mut self, op: u8, a: u16, b: u16) -> u16 {
        // execute and set flags
        let alu_result = match op {
            0 => a & b,
            1 => a | b,
            2 => a.wrapping_add(b),
            3 => a.wrapping_sub(b),
            4 => a.wrapping_mul(b),
            5 => a.wrapping_div(b),
            6 => a.wrapping_rem(b),
            7 => a ^ b,
            _ => panic!("Invalid opcode"),
        };
        self.flags.n = (alu_result >> 15) == 1;
        self.flags.z = alu_result == 0;
        alu_result
    }

    pub fn get_flags(&self) -> &Flags {
        &self.flags
    }
}

fn read_file(filename: &str) -> Vec<u16> {
    let mut data = Vec::new();
    let file = std::fs::File::open(filename);
    match file {
        Ok(file) => {
            // read file into buffer
            // parse buffer into u16 data
            // return data
            let mut reader = std::io::BufReader::new(file);
            let mut buffer = String::new();
            reader.read_to_string(&mut buffer).unwrap();
            let lines = buffer.lines();
            let mut line_counter = 0;
            for line in lines {
                let mut trimmed_line = line.trim();
                if trimmed_line.starts_with("//") {
                    continue;
                }
                if trimmed_line.contains("//") {
                    trimmed_line = trimmed_line.split("//").next().unwrap().trim();
                }
                if trimmed_line.len() == 0 {
                    continue;
                }
                // delete any whitespaces or underscores in between binary digits
                let clean_line = trimmed_line.replace(" ", "").replace("_", "");

                let numeric = u16::from_str_radix(&clean_line, 2);
                match numeric {
                    Ok(numeric) => data.push(numeric),
                    Err(error) => println!(
                        "Error '{0}' found when parsing {1}:{2}:'{3}'\nSkipping line {2}",
                        error, filename, line_counter, line
                    ),
                }
                line_counter += 1;
            }
            data
        }
        Err(error) => {
            println!("Error reading file: {}\nReturning null data", error);
            data
        }
    }
}

pub struct Memory {
    memory: Vec<u16>,
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

    pub fn load_from_file(&mut self, filename: &str) {
        let data = read_file(filename);
        for (i, value) in data.iter().enumerate() {
            self.memory[i] = *value;
        }
    }

    pub fn read(&self, address: usize) -> u16 {
        self.memory[address]
    }

    pub fn write(&mut self, address: usize, value: u16) {
        if self.write_enable {
            self.memory[address] = value;
        }
    }

    pub fn set_write_enable(&mut self, write_enable: bool) {
        self.write_enable = write_enable;
    }
}

pub struct Flags {
    pub n: bool,
    pub z: bool,
}

impl Flags {
    pub fn new() -> Flags {
        Flags {
            n: false,
            z: false,
        }
    }

    pub fn condex(&self, cond: u8) -> bool {
        match cond {
            0b00 => true,
            0b01 => self.z,
            0b10 => self.n,
            0b11 => !self.z && !self.n,
            _ => panic!("Invalid condition code"),
        }
    }
}

