pub struct Instruction(pub u16);

#[derive(Debug)]
pub enum Opcode {
    JP,
    CALL,
    SEN,
    SNEN,
    SEY,
    LDN,
    ADDN,
    LDY,
    OR,
    AND,
    XOR,
    ADDY,
    SUB,
    SHR,
    SUBN,
    SHL,
    SNEY,
    LDI,
    JPV,
    RND,
    DRW,
    SKIP,
    SKNP,
    SET,
    LDKEY,
    LDDT,
    LDST,
    ADDI,
    LDF,
    BCD,
    MEMI,
    MEMX,
    SYS,
    CLS,
    RET
}

impl Instruction {
    pub fn decode_instruction(instruction: &Instruction) -> Opcode {
        use Opcode::*;
        let Instruction(inst) = *instruction;
        match inst >> 12 {
            0x0 => match inst & 0xFFF {
                0x0E0 => CLS,
                0x0EE => RET,
                _     => SYS
            },
            0x1 => JP,
            0x2 => CALL,
            0x3 => SEN,
            0x4 => SNEN,
            0x5 => SEY,
            0x6 => LDN,
            0x7 => ADDN,
            0x8 => match inst & 0xF {
                0x0 => LDY,
                0x1 => OR,
                0x2 => AND,
                0x3 => XOR,
                0x4 => ADDY,
                0x5 => SUB,
                0x6 => SHR,
                0x7 => SUBN,
                0xE => SHL,
                _   => panic!("Unknown opcode: {}", inst)
            },
            0x9 => SNEY,
            0xA => LDI,
            0xB => JPV,
            0xC => RND,
            0xD => DRW,
            0xE => match inst & 0xFF {
                0x9E => SKIP,
                0xA1 => SKNP,
                _    => panic!("Unknown opcode: {}", inst)
            },
            0xF => match inst & 0xFF {
                0x07 => SET,
                0x0A => LDKEY,
                0x15 => LDDT,
                0x18 => LDST,
                0x1E => ADDI,
                0x29 => LDF,
                0x33 => BCD,
                0x55 => MEMI,
                0x65 => MEMX,
                _    => panic!("Unknown opcode: {}", inst)
            },
            _   => panic!("Unknown opcode: {}", inst)
        }
    }
}
