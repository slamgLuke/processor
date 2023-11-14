// 16-bit instruction format

#[derive(Debug)]
pub struct Instruction {
    pub cond: u8, // 2 bits
    pub mem: u8, // 1 bit
    pub opcode: u8, // 3 bits
    pub rn: u8, // 3 bits
    pub rd: u8, // 3 bits
    pub i: u8, // 1 bit
    pub src2: u8, // 3 bits
}
