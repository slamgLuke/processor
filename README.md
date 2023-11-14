# Microprocessor

## Description

- 16 bit microprocessor
- 8 registers, including program counter
- supports conditional execution, basic data processing and memory manipulation

## Instruction Format

### Data Processing

DP Instructions consist of half-words in the following format:

- 2 bits for conditition code
- 1: 0
- 3 bits for opcode
- 3 bits for Rn
- 3 bits for Rd
- 1 bit for I
- 3 bits for Src2

#### OpCode List

- 000: a & b
- 001: a | b
- 010: a + b
- 011: a - b
- 100: a * b
- 101: a / b
- 110: a % b
- 111: a ^ b


### Memory

Memory Manipulation Instructions consist of half-words in the following format:

- 2 bits for condition code
- 1: 1
- 1 bit for write (0 read, 1 write)
- 1 bit for offset writeback
- 1 bit for operation (0 add, 1 sub)
- 3 bits for Rn
- 3 bits for Rd
- 1 bit for I
- 3 bits for Src2


## TODO

- Components
- Wires
- Interaction logic
- Event driven simulation
- Waveform visualizer


