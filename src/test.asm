// first assembly test code
// 1 half-word per instruction
// COND(2) | MEM?(1) | OP(3) | Rn(3) | Rd(3) | I?(1) | Src2(3)
// COND(2) | MEM?(1) | W?(1) | WB?(1) | OP(1) | Rn(3) | Rd(3) | I?(1) | Src2(3)

// SUB R0, R7, R7       ; R0 = 0
00_0_011 111 000 0 111
// ADD R1, R0, #1       ; R1 = 1
00_0_010 000 001 1 001
// OR R2, R1, #0b110    ; R2 = 7
00_0_001 001 010 1 110
// AND R3, R2, #0b011   ; R3 = 3
00_0_000 010 011 1 011

// bye!
 
